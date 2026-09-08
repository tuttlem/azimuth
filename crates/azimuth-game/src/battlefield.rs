use crate::world::WorldPosition;

/// One large default battlefield. At maximum 45-degree power, ordinary shells can still cross
/// most of this width; a larger map would make normal artillery engagement needlessly rare.
pub const HALF_EXTENT: f32 = 60.0;
/// Physical scale and sample density are deliberately separate. This keeps existing craters and
/// one-unit positioning readable without multiplying mesh density with the map's area.
pub const TERRAIN_CELLS_PER_SIDE: usize = 64;
pub const WATER_TABLE: f32 = 0.0;
const VERTICES_PER_SIDE: usize = TERRAIN_CELLS_PER_SIDE + 1;
const TERRAIN_CELL_SIZE: f32 = HALF_EXTENT * 2.0 / TERRAIN_CELLS_PER_SIDE as f32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BattlefieldSeed(pub u64);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BuildingPlacement {
    pub position: WorldPosition,
    pub width: f32,
    pub depth: f32,
    pub height: f32,
    pub yaw_radians: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Crater {
    radius: f32,
    depth: f32,
}

impl Crater {
    #[cfg(test)]
    pub fn default_development() -> Self {
        Self::new(4.0, 1.8).expect("development crater must be valid")
    }

    pub fn new(radius: f32, depth: f32) -> Result<Self, CraterError> {
        if !radius.is_finite() || radius <= 0.0 {
            return Err(CraterError::Radius);
        }
        if !depth.is_finite() || depth <= 0.0 {
            return Err(CraterError::Depth);
        }
        Ok(Self { radius, depth })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CraterError {
    Radius,
    Depth,
}

/// The single authoritative battlefield surface. Its current vertex heights drive both triangle
/// interpolation for gameplay and positions for the rendered mesh.
#[derive(Clone, Debug, PartialEq)]
pub struct BattlefieldTerrain {
    heights: Vec<f32>,
}

impl Default for BattlefieldTerrain {
    fn default() -> Self {
        Self::initial()
    }
}

impl BattlefieldTerrain {
    pub fn initial() -> Self {
        // Retained as a compact deterministic fixture for older focused physics tests. Running
        // matches use `generated`, which is the map-selection path introduced by this feature.
        let mut heights = Vec::with_capacity(VERTICES_PER_SIDE * VERTICES_PER_SIDE);
        for z_index in 0..VERTICES_PER_SIDE {
            for x_index in 0..VERTICES_PER_SIDE {
                let (x, z) = vertex_position(x_index, z_index);
                heights.push(authored_fixture_height(x, z));
            }
        }
        Self { heights }
    }

    /// Macro shapes create the artillery decisions; local variation merely stops their slopes
    /// feeling synthetic. Keeping both here makes the rendered mesh and gameplay query one world.
    pub fn generated(seed: BattlefieldSeed) -> Self {
        let mut heights = Vec::with_capacity(VERTICES_PER_SIDE * VERTICES_PER_SIDE);
        for z_index in 0..VERTICES_PER_SIDE {
            for x_index in 0..VERTICES_PER_SIDE {
                let (x, z) = vertex_position(x_index, z_index);
                heights.push(generated_height(x, z, seed));
            }
        }
        Self { heights }
    }

    pub fn height(&self, x: f32, z: f32) -> f32 {
        assert!(
            is_within_bounds(x, z),
            "terrain height queries must remain within the battlefield"
        );
        let (x_index, x_fraction) = terrain_cell_coordinate(x);
        let (z_index, z_fraction) = terrain_cell_coordinate(z);
        let lower_left = self.vertex_height(x_index, z_index);
        let lower_right = self.vertex_height(x_index + 1, z_index);
        let upper_left = self.vertex_height(x_index, z_index + 1);
        let upper_right = self.vertex_height(x_index + 1, z_index + 1);
        if x_fraction + z_fraction <= 1.0 {
            lower_left * (1.0 - x_fraction - z_fraction)
                + lower_right * x_fraction
                + upper_left * z_fraction
        } else {
            lower_right * (1.0 - z_fraction)
                + upper_left * (1.0 - x_fraction)
                + upper_right * (x_fraction + z_fraction - 1.0)
        }
    }

    pub fn height_if_within_bounds(&self, x: f32, z: f32) -> Option<f32> {
        is_within_bounds(x, z).then(|| self.height(x, z))
    }

    /// Returns the absolute current-surface elevation change between two in-bounds horizontal
    /// positions. Movement uses this rather than a tank's stored Y coordinate because a later
    /// crater may have changed terrain below a stationary tank that intentionally has not settled.
    pub fn elevation_change_if_within_bounds(
        &self,
        start_x: f32,
        start_z: f32,
        end_x: f32,
        end_z: f32,
    ) -> Option<f32> {
        Some(
            (self.height_if_within_bounds(end_x, end_z)?
                - self.height_if_within_bounds(start_x, start_z)?)
            .abs(),
        )
    }

    /// Lowers current vertices by a smooth bowl. The squared remaining fraction reaches zero at
    /// the radius, so adjacent unaffected terrain joins without a hard deformation edge.
    pub fn apply_crater(&mut self, centre: WorldPosition, crater: Crater) {
        assert!(
            centre.is_finite() && is_within_bounds(centre.x, centre.z),
            "crater centre must be an in-bounds impact"
        );
        for z_index in 0..VERTICES_PER_SIDE {
            for x_index in 0..VERTICES_PER_SIDE {
                let (x, z) = vertex_position(x_index, z_index);
                let dx = x - centre.x;
                let dz = z - centre.z;
                let fraction = 1.0 - (dx * dx + dz * dz) / (crater.radius * crater.radius);
                if fraction > 0.0 {
                    self.heights[vertex_index(x_index, z_index)] -=
                        crater.depth * fraction * fraction;
                }
            }
        }
    }

    pub fn mesh_positions(&self) -> Vec<[f32; 3]> {
        let mut positions = Vec::with_capacity(self.heights.len());
        for z_index in 0..VERTICES_PER_SIDE {
            for x_index in 0..VERTICES_PER_SIDE {
                let (x, z) = vertex_position(x_index, z_index);
                positions.push([x, self.vertex_height(x_index, z_index), z]);
            }
        }
        positions
    }

    /// Presentation is derived from current terrain height so a crater cannot reveal stale or
    /// uninitialised colour. Water itself is a separate flat presentation plane.
    pub fn mesh_colours(&self) -> Vec<[f32; 4]> {
        self.heights.iter().copied().map(elevation_colour).collect()
    }

    #[cfg(test)]
    pub fn sampled_elevation_span(&self) -> f32 {
        let minimum = self.heights.iter().copied().fold(f32::INFINITY, f32::min);
        let maximum = self
            .heights
            .iter()
            .copied()
            .fold(f32::NEG_INFINITY, f32::max);
        maximum - minimum
    }

    #[cfg(test)]
    pub fn all_heights_finite(&self) -> bool {
        self.heights.iter().all(|height| height.is_finite())
    }
    fn vertex_height(&self, x_index: usize, z_index: usize) -> f32 {
        self.heights[vertex_index(x_index, z_index)]
    }
}

pub fn is_within_bounds(x: f32, z: f32) -> bool {
    (-HALF_EXTENT..=HALF_EXTENT).contains(&x) && (-HALF_EXTENT..=HALF_EXTENT).contains(&z)
}

pub fn elevation_colour(height: f32) -> [f32; 4] {
    // Stops are intentionally world-height based: falling into a crater can expose lower grass,
    // which reads naturally without pretending to simulate soil layers.
    const LOWLAND: [f32; 3] = [0.06, 0.28, 0.10];
    const GRASS: [f32; 3] = [0.16, 0.52, 0.16];
    const EARTH: [f32; 3] = [0.40, 0.30, 0.17];
    const ROCK: [f32; 3] = [0.43, 0.43, 0.40];
    const SNOW: [f32; 3] = [0.94, 0.95, 0.97];
    let colour = if height <= WATER_TABLE {
        LOWLAND
    } else if height < 4.0 {
        blend(LOWLAND, GRASS, (height - WATER_TABLE) / 4.0)
    } else if height < 11.0 {
        blend(GRASS, EARTH, (height - 4.0) / 7.0)
    } else if height < 18.0 {
        blend(EARTH, ROCK, (height - 11.0) / 7.0)
    } else if height < 25.0 {
        blend(ROCK, SNOW, (height - 18.0) / 7.0)
    } else {
        SNOW
    };
    [colour[0], colour[1], colour[2], 1.0]
}

fn blend(first: [f32; 3], second: [f32; 3], fraction: f32) -> [f32; 3] {
    let fraction = fraction.clamp(0.0, 1.0);
    [
        first[0] + (second[0] - first[0]) * fraction,
        first[1] + (second[1] - first[1]) * fraction,
        first[2] + (second[2] - first[2]) * fraction,
    ]
}

pub fn is_dry_and_gentle(terrain: &BattlefieldTerrain, x: f32, z: f32) -> bool {
    const STEP: f32 = 1.0;
    const MAX_CHANGE: f32 = 0.75;
    let Some(height) = terrain.height_if_within_bounds(x, z) else {
        return false;
    };
    height > WATER_TABLE
        && [(STEP, 0.0), (-STEP, 0.0), (0.0, STEP), (0.0, -STEP)]
            .into_iter()
            .all(|(dx, dz)| {
                terrain
                    .height_if_within_bounds(x + dx, z + dz)
                    .is_some_and(|neighbour| (neighbour - height).abs() <= MAX_CHANGE)
            })
}

/// Builds are intentionally records rather than world objects. The renderer may turn them into
/// cuboids, but gameplay never receives them as collision, cover, or damage data.
pub fn generate_buildings(
    terrain: &BattlefieldTerrain,
    starts: &[(f32, f32)],
    mut seed: u64,
) -> Vec<BuildingPlacement> {
    let mut buildings = Vec::new();
    for _ in 0..24 {
        let x = random_range(&mut seed, -50.0, 50.0);
        let z = random_range(&mut seed, -50.0, 50.0);
        if !is_dry_and_gentle(terrain, x, z)
            || starts.iter().any(|(start_x, start_z)| {
                let dx = x - start_x;
                let dz = z - start_z;
                dx * dx + dz * dz < 8.0 * 8.0
            })
        {
            continue;
        }
        let base = WorldPosition {
            x,
            y: terrain.height(x, z),
            z,
        };
        buildings.push(BuildingPlacement {
            position: base,
            width: random_range(&mut seed, 1.5, 3.5),
            depth: random_range(&mut seed, 1.5, 3.5),
            height: random_range(&mut seed, 1.5, 5.0),
            yaw_radians: random_range(&mut seed, 0.0, std::f32::consts::TAU),
        });
        if buildings.len() >= 8 {
            break;
        }
    }
    buildings
}

pub fn terrain_mesh_indices() -> Vec<u32> {
    let mut indices = Vec::with_capacity(TERRAIN_CELLS_PER_SIDE * TERRAIN_CELLS_PER_SIDE * 6);
    for z_index in 0..TERRAIN_CELLS_PER_SIDE {
        for x_index in 0..TERRAIN_CELLS_PER_SIDE {
            let lower_left = vertex_index(x_index, z_index) as u32;
            let lower_right = lower_left + 1;
            let upper_left = lower_left + VERTICES_PER_SIDE as u32;
            let upper_right = upper_left + 1;
            indices.extend_from_slice(&[
                lower_left,
                upper_left,
                lower_right,
                lower_right,
                upper_left,
                upper_right,
            ]);
        }
    }
    indices
}

fn generated_height(x: f32, z: f32, seed: BattlefieldSeed) -> f32 {
    let mut random = seed.0;
    let mountain_x = random_range(&mut random, -42.0, -24.0);
    let mountain_z = random_range(&mut random, -30.0, 30.0);
    let ridge_x = random_range(&mut random, -10.0, 25.0);
    let ridge_z = random_range(&mut random, -35.0, 35.0);
    let ridge_angle = random_range(&mut random, -0.8, 0.8);
    let bowl_x = random_range(&mut random, 18.0, 40.0);
    let bowl_z = random_range(&mut random, -28.0, 28.0);
    let mountain = smooth_bump(x - mountain_x, z - mountain_z, 22.0, 20.0) * 21.0;
    let rotated_x = (x - ridge_x) * ridge_angle.cos() + (z - ridge_z) * ridge_angle.sin();
    let rotated_z = -(x - ridge_x) * ridge_angle.sin() + (z - ridge_z) * ridge_angle.cos();
    let ridge = smooth_bump(rotated_x, rotated_z, 42.0, 7.0) * 7.0;
    let bowl = smooth_bump(x - bowl_x, z - bowl_z, 25.0, 22.0) * -12.0;
    let local = (x * random_range(&mut random, 0.08, 0.13)).sin()
        * (z * random_range(&mut random, 0.07, 0.12)).cos()
        * 1.2
        + ((x + z) * random_range(&mut random, 0.04, 0.08)).sin() * 0.8;
    6.0 + mountain + ridge + bowl + local
}

fn authored_fixture_height(x: f32, z: f32) -> f32 {
    1.8 * (x * 0.16).sin() * (z * 0.13).cos() + z * 0.08
}

fn smooth_bump(x: f32, z: f32, radius_x: f32, radius_z: f32) -> f32 {
    let distance = x * x / (radius_x * radius_x) + z * z / (radius_z * radius_z);
    (1.0 - distance).max(0.0).powi(2)
}

fn random_range(seed: &mut u64, minimum: f32, maximum: f32) -> f32 {
    *seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
    minimum + (maximum - minimum) * ((*seed >> 40) as f32 / (1_u32 << 24) as f32)
}
fn vertex_index(x_index: usize, z_index: usize) -> usize {
    z_index * VERTICES_PER_SIDE + x_index
}
fn vertex_position(x_index: usize, z_index: usize) -> (f32, f32) {
    (
        -HALF_EXTENT + x_index as f32 * TERRAIN_CELL_SIZE,
        -HALF_EXTENT + z_index as f32 * TERRAIN_CELL_SIZE,
    )
}
fn terrain_cell_coordinate(value: f32) -> (usize, f32) {
    let coordinate =
        ((value + HALF_EXTENT) / TERRAIN_CELL_SIZE).clamp(0.0, TERRAIN_CELLS_PER_SIDE as f32);
    let index = coordinate.floor() as usize;
    if index == TERRAIN_CELLS_PER_SIDE {
        (TERRAIN_CELLS_PER_SIDE - 1, 1.0)
    } else {
        (index, coordinate - index as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f32 = 0.000_1;
    fn close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < EPSILON,
            "expected {expected}, got {actual}"
        );
    }
    fn centre() -> WorldPosition {
        WorldPosition {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[test]
    fn crater_lowers_centre_but_not_its_exterior() {
        let mut terrain = BattlefieldTerrain::initial();
        let before_centre = terrain.height(0.0, 0.0);
        let before_outside = terrain.height(10.0, 0.0);
        terrain.apply_crater(centre(), Crater::default_development());
        assert!(terrain.height(0.0, 0.0) < before_centre);
        close(terrain.height(10.0, 0.0), before_outside);
    }
    #[test]
    fn crater_radius_and_depth_change_its_effect() {
        let mut shallow = BattlefieldTerrain::initial();
        let mut deep = shallow.clone();
        let mut narrow = shallow.clone();
        shallow.apply_crater(centre(), Crater::new(4.0, 1.0).unwrap());
        deep.apply_crater(centre(), Crater::new(4.0, 2.0).unwrap());
        narrow.apply_crater(centre(), Crater::new(2.0, 1.0).unwrap());
        assert!(deep.height(0.0, 0.0) < shallow.height(0.0, 0.0));
        assert!(shallow.height(3.0, 0.0) < narrow.height(3.0, 0.0));
    }
    #[test]
    fn mesh_and_query_share_current_surface() {
        let mut terrain = BattlefieldTerrain::initial();
        terrain.apply_crater(centre(), Crater::default_development());
        for position in terrain.mesh_positions() {
            close(terrain.height(position[0], position[2]), position[1]);
        }
    }
    #[test]
    fn overlapping_and_edge_craters_are_deterministic_and_finite() {
        let impacts = [
            centre(),
            WorldPosition {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            WorldPosition {
                x: HALF_EXTENT,
                y: 0.0,
                z: HALF_EXTENT,
            },
        ];
        let mut first = BattlefieldTerrain::initial();
        let mut second = first.clone();
        for _ in 0..100 {
            for impact in impacts {
                first.apply_crater(impact, Crater::default_development());
                second.apply_crater(impact, Crater::default_development());
            }
        }
        assert_eq!(first, second);
        assert!(first.all_heights_finite());
        assert!(first.height(0.0, 0.0) < BattlefieldTerrain::initial().height(0.0, 0.0));
    }
    #[test]
    fn invalid_craters_are_rejected() {
        assert_eq!(Crater::new(0.0, 1.0), Err(CraterError::Radius));
        assert_eq!(Crater::new(1.0, f32::NAN), Err(CraterError::Depth));
    }

    #[test]
    fn elevation_change_uses_current_deformed_surface_and_rejects_bounds() {
        let mut terrain = BattlefieldTerrain::initial();
        let before = terrain
            .elevation_change_if_within_bounds(0.0, 0.0, 1.0, 0.0)
            .unwrap();
        terrain.apply_crater(centre(), Crater::default_development());
        let after = terrain
            .elevation_change_if_within_bounds(0.0, 0.0, 1.0, 0.0)
            .unwrap();

        assert_ne!(before, after);
        assert!(
            terrain
                .elevation_change_if_within_bounds(HALF_EXTENT, 0.0, HALF_EXTENT + 1.0, 0.0)
                .is_none()
        );
    }

    #[test]
    fn narrow_deep_crater_produces_a_repeatable_impassable_step() {
        let start = WorldPosition {
            x: -11.25,
            y: 0.0,
            z: -8.0,
        };
        let mut first = BattlefieldTerrain::initial();
        let mut second = BattlefieldTerrain::initial();
        let crater = Crater::new(4.0, 8.0).unwrap();
        first.apply_crater(start, crater);
        second.apply_crater(start, crater);

        let first_change = first
            .elevation_change_if_within_bounds(start.x, start.z, start.x + 1.0, start.z)
            .unwrap();
        let second_change = second
            .elevation_change_if_within_bounds(start.x, start.z, start.x + 1.0, start.z)
            .unwrap();
        assert_eq!(first_change, second_change);
        assert!(first_change > crate::tank::MAX_MOVEMENT_ELEVATION_CHANGE);
    }

    #[test]
    fn generated_battlefields_are_reproducible_varied_and_large_scale() {
        let first = BattlefieldTerrain::generated(BattlefieldSeed(42));
        let repeated = BattlefieldTerrain::generated(BattlefieldSeed(42));
        let different = BattlefieldTerrain::generated(BattlefieldSeed(43));
        assert_eq!(first, repeated);
        assert_ne!(first, different);
        assert_eq!(first.mesh_positions().len(), 65 * 65);
        assert!(first.sampled_elevation_span() >= 24.0);
        for (x, z) in [(-HALF_EXTENT, -HALF_EXTENT), (HALF_EXTENT, HALF_EXTENT)] {
            assert!(first.height(x, z).is_finite());
        }
    }

    #[test]
    fn elevation_colours_and_dressing_are_bounded_and_deterministic() {
        let terrain = BattlefieldTerrain::generated(BattlefieldSeed(9));
        let low = elevation_colour(1.0);
        let high = elevation_colour(30.0);
        assert!(
            low.iter()
                .chain(high.iter())
                .all(|value| value.is_finite() && *value >= 0.0 && *value <= 1.0)
        );
        assert!(high[0] > low[0]);
        let starts = [(-40.0, -40.0), (40.0, 40.0)];
        let first = generate_buildings(&terrain, &starts, 99);
        assert_eq!(first, generate_buildings(&terrain, &starts, 99));
        assert!(first.iter().all(|building| {
            is_within_bounds(building.position.x, building.position.z)
                && building.position.y > WATER_TABLE
                && is_dry_and_gentle(&terrain, building.position.x, building.position.z)
        }));
    }
}
