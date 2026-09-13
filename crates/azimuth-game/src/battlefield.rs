use crate::environment::{PaletteProfile, TerrainProfile};
use crate::world::{WorldPosition, WorldVector};

/// One large default battlefield. At maximum 45-degree power, ordinary shells can still cross
/// most of this width; a larger map would make normal artillery engagement needlessly rare.
pub const HALF_EXTENT: f32 = 60.0;
/// A coarse, render-only skirt reaches far enough to conceal the playable square's edge without
/// changing the 120-unit gameplay world or multiplying the mutable terrain mesh.
pub const HORIZON_HALF_EXTENT: f32 = 180.0;
/// A gentle render-only drop across the exterior skirt suggests a broad curved horizon without
/// changing the square, authoritative battlefield beneath it.
const HORIZON_CURVATURE_DROP: f32 = 3.5;
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

/// Immutable exterior scenery data. It deliberately offers mesh data only: gameplay must continue
/// to query, collide with, and deform `BattlefieldTerrain` inside `HALF_EXTENT`.
#[derive(Clone, Debug, PartialEq)]
pub struct VisualHorizon {
    positions: Vec<[f32; 3]>,
    colours: Vec<[f32; 4]>,
    indices: Vec<u32>,
}

impl VisualHorizon {
    // Match every mutable terrain-edge vertex. A coarser perimeter interpolates differently
    // between samples and can read as a slight tear even when its corner positions coincide.
    const SEGMENTS_PER_EDGE: usize = TERRAIN_CELLS_PER_SIDE;
    // Extra rings make the exterior height/colour transition gradual without expanding the
    // authoritative terrain surface.
    const RADIAL_STEPS: usize = 8;

    pub fn from_terrain(
        terrain: &BattlefieldTerrain,
        seed: BattlefieldSeed,
        palette: PaletteProfile,
    ) -> Self {
        let inner = square_perimeter(HALF_EXTENT, Self::SEGMENTS_PER_EDGE);
        let outer = square_perimeter(HORIZON_HALF_EXTENT, Self::SEGMENTS_PER_EDGE);
        let ring_len = inner.len();
        let mut positions = Vec::with_capacity(ring_len * (Self::RADIAL_STEPS + 1));
        let mut colours = Vec::with_capacity(positions.capacity());
        for ring in 0..=Self::RADIAL_STEPS {
            let fraction = ring as f32 / Self::RADIAL_STEPS as f32;
            // Ease the palette transition as well as the geometry.  The first ring must be an
            // exact visual continuation of the mutable combat terrain; farther rings fade into
            // deterministic generated scenery without becoming part of that terrain.
            let colour_fraction = smoothstep(fraction);
            for ((inner_x, inner_z), (outer_x, outer_z)) in inner.iter().zip(&outer) {
                let x = inner_x + (outer_x - inner_x) * fraction;
                let z = inner_z + (outer_z - inner_z) * fraction;
                let inner_height = terrain.height(*inner_x, *inner_z);
                let outer_height =
                    generated_height(*outer_x, *outer_z, seed, TerrainProfile::Standard);
                // Keep the inner ring exactly welded to the mutable terrain, then let the
                // exterior fall away very gently so the surrounding plane reads as a distant
                // planetary surface rather than a perfectly flat tabletop.
                let height = inner_height + (outer_height - inner_height) * fraction
                    - HORIZON_CURVATURE_DROP * smoothstep(fraction);
                positions.push([x, height, z]);
                colours.push(blend_colour(
                    surface_colour(inner_height, *inner_x, *inner_z, palette),
                    surface_colour(outer_height, *outer_x, *outer_z, palette),
                    colour_fraction,
                ));
            }
        }
        let mut indices = Vec::with_capacity(ring_len * Self::RADIAL_STEPS * 6);
        for ring in 0..Self::RADIAL_STEPS {
            let base = ring * ring_len;
            let outer_base = base + ring_len;
            for index in 0..ring_len {
                let next = (index + 1) % ring_len;
                indices.extend_from_slice(&[
                    (base + index) as u32,
                    (base + next) as u32,
                    (outer_base + index) as u32,
                    (outer_base + index) as u32,
                    (base + next) as u32,
                    (outer_base + next) as u32,
                ]);
            }
        }
        Self {
            positions,
            colours,
            indices,
        }
    }

    pub fn positions(&self) -> &[[f32; 3]] {
        &self.positions
    }

    pub fn colours(&self) -> &[[f32; 4]] {
        &self.colours
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Crater {
    radius: f32,
    depth: f32,
}

/// A deliberately game-like radial terrain deposit. It mirrors crater deformation while adding
/// ground, which keeps terrain generation deterministic and easy to reason about.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mound {
    radius: f32,
    height: f32,
}

impl Mound {
    pub fn new(radius: f32, height: f32) -> Result<Self, CraterError> {
        if !radius.is_finite() || radius <= 0.0 {
            return Err(CraterError::Radius);
        }
        if !height.is_finite() || height <= 0.0 {
            return Err(CraterError::Depth);
        }
        Ok(Self { radius, height })
    }
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
    pub fn generated(seed: BattlefieldSeed, profile: TerrainProfile) -> Self {
        let mut heights = Vec::with_capacity(VERTICES_PER_SIDE * VERTICES_PER_SIDE);
        for z_index in 0..VERTICES_PER_SIDE {
            for x_index in 0..VERTICES_PER_SIDE {
                let (x, z) = vertex_position(x_index, z_index);
                heights.push(generated_height(x, z, seed, profile));
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

    /// Exact normal of the same current interpolation triangle used by `height`.
    pub fn surface_normal_if_within_bounds(&self, x: f32, z: f32) -> Option<WorldVector> {
        if !is_within_bounds(x, z) {
            return None;
        }
        let (xi, xf) = terrain_cell_coordinate(x);
        let (zi, zf) = terrain_cell_coordinate(z);
        let ll = self.vertex_height(xi, zi);
        let lr = self.vertex_height(xi + 1, zi);
        let ul = self.vertex_height(xi, zi + 1);
        let ur = self.vertex_height(xi + 1, zi + 1);
        let (dx, dz) = if xf + zf <= 1.0 {
            ((lr - ll) / TERRAIN_CELL_SIZE, (ul - ll) / TERRAIN_CELL_SIZE)
        } else {
            ((ur - ul) / TERRAIN_CELL_SIZE, (ur - lr) / TERRAIN_CELL_SIZE)
        };
        Some(
            WorldVector {
                x: -dx,
                y: 1.0,
                z: -dz,
            }
            .normalized(),
        )
    }

    /// Deterministic central-difference downhill direction on the current authoritative surface.
    /// `y` is zero because Roller only needs horizontal terrain guidance.
    pub fn downhill_direction_if_within_bounds(&self, x: f32, z: f32) -> Option<WorldVector> {
        const SAMPLE: f32 = 0.75;
        let west = self.height_if_within_bounds(x - SAMPLE, z)?;
        let east = self.height_if_within_bounds(x + SAMPLE, z)?;
        let north = self.height_if_within_bounds(x, z - SAMPLE)?;
        let south = self.height_if_within_bounds(x, z + SAMPLE)?;
        Some(WorldVector {
            x: west - east,
            y: 0.0,
            z: north - south,
        })
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

    pub fn apply_mound(&mut self, centre: WorldPosition, mound: Mound) {
        assert!(
            centre.is_finite() && is_within_bounds(centre.x, centre.z),
            "mound centre must be an in-bounds impact"
        );
        for z_index in 0..VERTICES_PER_SIDE {
            for x_index in 0..VERTICES_PER_SIDE {
                let (x, z) = vertex_position(x_index, z_index);
                let dx = x - centre.x;
                let dz = z - centre.z;
                let fraction = 1.0 - (dx * dx + dz * dz) / (mound.radius * mound.radius);
                if fraction > 0.0 {
                    self.heights[vertex_index(x_index, z_index)] +=
                        mound.height * fraction * fraction;
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
    pub fn mesh_colours(&self, palette: PaletteProfile) -> Vec<[f32; 4]> {
        self.heights
            .iter()
            .copied()
            .enumerate()
            .map(|(index, height)| {
                let (x, z) = vertex_position(index % VERTICES_PER_SIDE, index / VERTICES_PER_SIDE);
                surface_colour(height, x, z, palette)
            })
            .collect()
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

/// A tiny deterministic colour wobble gives the low-poly terrain surface character without
/// becoming terrain data: crater/mound physics still depend only on `heights`.
fn surface_colour(height: f32, x: f32, z: f32, palette: PaletteProfile) -> [f32; 4] {
    let mut colour = elevation_colour(height);
    let base = [colour[0], colour[1], colour[2]];
    let tint = match palette {
        PaletteProfile::Earth => base,
        PaletteProfile::Moon => {
            let shade = (height / 28.0).clamp(0.18, 0.72);
            [shade, shade, shade * 1.05]
        }
        PaletteProfile::Storm => [base[0] * 0.52, base[1] * 0.58, base[2] * 0.68],
        PaletteProfile::Crusher => {
            let heat = (height / 28.0).clamp(0.0, 1.0);
            [0.50 + heat * 0.34, 0.18 + heat * 0.36, 0.05 + heat * 0.08]
        }
    };
    colour[..3].copy_from_slice(&tint);
    let mottle = ((x * 0.37).sin() * (z * 0.29).cos()) * 0.035;
    for channel in &mut colour[..3] {
        *channel = (*channel + mottle).clamp(0.0, 1.0);
    }
    colour
}

fn blend(first: [f32; 3], second: [f32; 3], fraction: f32) -> [f32; 3] {
    let fraction = fraction.clamp(0.0, 1.0);
    [
        first[0] + (second[0] - first[0]) * fraction,
        first[1] + (second[1] - first[1]) * fraction,
        first[2] + (second[2] - first[2]) * fraction,
    ]
}

fn blend_colour(first: [f32; 4], second: [f32; 4], fraction: f32) -> [f32; 4] {
    let fraction = fraction.clamp(0.0, 1.0);
    [
        first[0] + (second[0] - first[0]) * fraction,
        first[1] + (second[1] - first[1]) * fraction,
        first[2] + (second[2] - first[2]) * fraction,
        first[3] + (second[3] - first[3]) * fraction,
    ]
}

fn smoothstep(fraction: f32) -> f32 {
    let fraction = fraction.clamp(0.0, 1.0);
    fraction * fraction * (3.0 - 2.0 * fraction)
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

fn square_perimeter(half_extent: f32, segments_per_edge: usize) -> Vec<(f32, f32)> {
    let step = half_extent * 2.0 / segments_per_edge as f32;
    let mut points = Vec::with_capacity(segments_per_edge * 4);
    for index in 0..segments_per_edge {
        points.push((-half_extent + index as f32 * step, -half_extent));
    }
    for index in 0..segments_per_edge {
        points.push((half_extent, -half_extent + index as f32 * step));
    }
    for index in 0..segments_per_edge {
        points.push((half_extent - index as f32 * step, half_extent));
    }
    for index in 0..segments_per_edge {
        points.push((-half_extent, half_extent - index as f32 * step));
    }
    points
}

fn generated_height(x: f32, z: f32, seed: BattlefieldSeed, profile: TerrainProfile) -> f32 {
    let mut random = seed.0;
    let mountain_x = random_range(&mut random, -42.0, -24.0);
    let mountain_z = random_range(&mut random, -30.0, 30.0);
    let second_mountain_x = random_range(&mut random, -8.0, 38.0);
    let second_mountain_z = random_range(&mut random, -42.0, 42.0);
    let third_mountain_x = random_range(&mut random, -38.0, 42.0);
    let third_mountain_z = random_range(&mut random, -45.0, 45.0);
    let ridge_x = random_range(&mut random, -10.0, 25.0);
    let ridge_z = random_range(&mut random, -35.0, 35.0);
    let ridge_angle = random_range(&mut random, -0.8, 0.8);
    let bowl_x = random_range(&mut random, 18.0, 40.0);
    let bowl_z = random_range(&mut random, -28.0, 28.0);
    // Small seed-driven features make the broad mountain/ridge/bowl composition feel less
    // authored while staying gentle enough for spawn selection and artillery readability.
    let knoll_one_x = random_range(&mut random, -48.0, 48.0);
    let knoll_one_z = random_range(&mut random, -48.0, 48.0);
    let knoll_two_x = random_range(&mut random, -48.0, 48.0);
    let knoll_two_z = random_range(&mut random, -48.0, 48.0);
    let mountain = smooth_bump(x - mountain_x, z - mountain_z, 22.0, 20.0) * 21.0
        + smooth_bump(x - second_mountain_x, z - second_mountain_z, 17.0, 15.0)
            * random_range(&mut random, 8.0, 15.0)
        // A smaller third peak makes some seeds feel properly mountainous without turning every
        // battlefield into an impassable wall.
        + smooth_bump(x - third_mountain_x, z - third_mountain_z, 12.0, 11.0)
            * random_range(&mut random, 0.0, 9.0);
    let rotated_x = (x - ridge_x) * ridge_angle.cos() + (z - ridge_z) * ridge_angle.sin();
    let rotated_z = -(x - ridge_x) * ridge_angle.sin() + (z - ridge_z) * ridge_angle.cos();
    let ridge = smooth_bump(rotated_x, rotated_z, 42.0, 7.0) * 7.0;
    let bowl = smooth_bump(x - bowl_x, z - bowl_z, 25.0, 22.0) * -12.0;
    let knolls = smooth_bump(x - knoll_one_x, z - knoll_one_z, 11.0, 9.0)
        * random_range(&mut random, 1.5, 3.5)
        + smooth_bump(x - knoll_two_x, z - knoll_two_z, 14.0, 12.0)
            * random_range(&mut random, -3.0, 2.5);
    let local = (x * random_range(&mut random, 0.08, 0.13)).sin()
        * (z * random_range(&mut random, 0.07, 0.12)).cos()
        * 1.2
        + ((x + z) * random_range(&mut random, 0.04, 0.08)).sin() * 0.8
        + (x * random_range(&mut random, 0.16, 0.24) + z * random_range(&mut random, 0.12, 0.20))
            .sin()
            * 0.35;
    let central_bowl = smooth_bump(x, z, 42.0, 42.0) * -13.0;
    6.0 + mountain
        + ridge
        + if profile == TerrainProfile::Bowl {
            central_bowl
        } else {
            bowl
        }
        + knolls
        + local
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
    fn visual_horizon_welds_to_the_authoritative_edge_without_expanding_it() {
        let terrain = BattlefieldTerrain::generated(BattlefieldSeed(73), TerrainProfile::Standard);
        let before = terrain.clone();
        let horizon =
            VisualHorizon::from_terrain(&terrain, BattlefieldSeed(73), PaletteProfile::Earth);
        assert_eq!(
            horizon,
            VisualHorizon::from_terrain(&terrain, BattlefieldSeed(73), PaletteProfile::Earth),
            "the render-only horizon palette must remain deterministic"
        );
        let positions = horizon.positions();
        let colours = horizon.colours();
        let ring_len = VisualHorizon::SEGMENTS_PER_EDGE * 4;
        let inner = square_perimeter(HALF_EXTENT, VisualHorizon::SEGMENTS_PER_EDGE);
        let outer = square_perimeter(HORIZON_HALF_EXTENT, VisualHorizon::SEGMENTS_PER_EDGE);

        assert_eq!(
            positions.len(),
            ring_len * (VisualHorizon::RADIAL_STEPS + 1)
        );
        assert_eq!(colours.len(), positions.len());
        assert_eq!(
            horizon.indices().len(),
            ring_len * VisualHorizon::RADIAL_STEPS * 6
        );
        for ((position, colour), (x, z)) in positions[..ring_len]
            .iter()
            .zip(&colours[..ring_len])
            .zip(&inner)
        {
            assert!(is_within_bounds(position[0], position[2]));
            close(position[1], terrain.height(position[0], position[2]));
            assert_eq!(
                *colour,
                surface_colour(position[1], *x, *z, PaletteProfile::Earth)
            );
        }
        for ring in 1..=VisualHorizon::RADIAL_STEPS {
            let fraction = ring as f32 / VisualHorizon::RADIAL_STEPS as f32;
            for (index, ((inner_x, inner_z), (outer_x, outer_z))) in
                inner.iter().zip(&outer).enumerate()
            {
                let inner_colour = surface_colour(
                    terrain.height(*inner_x, *inner_z),
                    *inner_x,
                    *inner_z,
                    PaletteProfile::Earth,
                );
                let outer_colour = surface_colour(
                    generated_height(
                        *outer_x,
                        *outer_z,
                        BattlefieldSeed(73),
                        TerrainProfile::Standard,
                    ),
                    *outer_x,
                    *outer_z,
                    PaletteProfile::Earth,
                );
                assert_eq!(
                    colours[ring * ring_len + index],
                    blend_colour(inner_colour, outer_colour, smoothstep(fraction))
                );
            }
        }
        assert!(positions[ring_len..].iter().all(|position| {
            position.iter().all(|value| value.is_finite())
                && (position[0].abs() >= HALF_EXTENT || position[2].abs() >= HALF_EXTENT)
        }));
        assert!(
            positions[ring_len..]
                .iter()
                .any(|position| position[0].abs() == HORIZON_HALF_EXTENT)
        );
        assert_eq!(terrain, before);
        assert!(!is_within_bounds(HALF_EXTENT + 0.1, 0.0));
        assert_eq!(
            terrain.height_if_within_bounds(HALF_EXTENT + 0.1, 0.0),
            None
        );
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
    fn narrow_deep_crater_produces_a_repeatable_terrain_change() {
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

        let first_change =
            (first.height(start.x, start.z) - first.height(start.x + 1.0, start.z)).abs();
        let second_change =
            (second.height(start.x, start.z) - second.height(start.x + 1.0, start.z)).abs();
        assert_eq!(first_change, second_change);
        assert!(first_change > 0.0);
    }

    #[test]
    fn generated_battlefields_are_reproducible_varied_and_large_scale() {
        let first = BattlefieldTerrain::generated(BattlefieldSeed(42), TerrainProfile::Standard);
        let repeated = BattlefieldTerrain::generated(BattlefieldSeed(42), TerrainProfile::Standard);
        let different =
            BattlefieldTerrain::generated(BattlefieldSeed(43), TerrainProfile::Standard);
        assert_eq!(first, repeated);
        assert_ne!(first, different);
        assert_eq!(first.mesh_positions().len(), 65 * 65);
        assert!(first.sampled_elevation_span() >= 24.0);
        for (x, z) in [(-HALF_EXTENT, -HALF_EXTENT), (HALF_EXTENT, HALF_EXTENT)] {
            assert!(first.height(x, z).is_finite());
        }
    }

    #[test]
    fn mounds_are_deterministic_raise_only_their_radius_and_have_surface_normals() {
        let mut first = BattlefieldTerrain::initial();
        let mut second = first.clone();
        let impact = WorldPosition {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let before = first.height(0.0, 0.0);
        let exterior = first.height(20.0, 20.0);
        let mound = Mound::new(8.0, 5.0).unwrap();
        first.apply_mound(impact, mound);
        second.apply_mound(impact, mound);
        assert_eq!(first, second);
        assert!(first.height(0.0, 0.0) > before);
        assert_eq!(first.height(20.0, 20.0), exterior);
        let normal = first.surface_normal_if_within_bounds(1.0, 0.5).unwrap();
        assert!(normal.y > 0.0 && normal.dot(normal) > 0.99);
        assert!(
            first
                .surface_normal_if_within_bounds(HALF_EXTENT + 1.0, 0.0)
                .is_none()
        );
    }

    #[test]
    fn elevation_colours_and_dressing_are_bounded_and_deterministic() {
        let terrain = BattlefieldTerrain::generated(BattlefieldSeed(9), TerrainProfile::Standard);
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

    #[test]
    fn surface_mottle_is_deterministic_and_does_not_change_height_queries() {
        let terrain = BattlefieldTerrain::generated(BattlefieldSeed(23), TerrainProfile::Standard);
        let before = terrain.height(3.0, -4.0);
        assert_eq!(
            surface_colour(8.0, 3.0, -4.0, PaletteProfile::Earth),
            surface_colour(8.0, 3.0, -4.0, PaletteProfile::Earth)
        );
        assert_ne!(
            surface_colour(8.0, 3.0, -4.0, PaletteProfile::Earth),
            surface_colour(8.0, 18.0, 11.0, PaletteProfile::Earth)
        );
        assert_eq!(terrain.height(3.0, -4.0), before);
    }
}
