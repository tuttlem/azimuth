use crate::world::WorldPosition;

pub const HALF_EXTENT: f32 = 20.0;
pub const TERRAIN_CELLS_PER_SIDE: usize = 20;
const VERTICES_PER_SIDE: usize = TERRAIN_CELLS_PER_SIDE + 1;
const TERRAIN_CELL_SIZE: f32 = HALF_EXTENT * 2.0 / TERRAIN_CELLS_PER_SIDE as f32;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Crater {
    radius: f32,
    depth: f32,
}

impl Crater {
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
        let mut heights = Vec::with_capacity(VERTICES_PER_SIDE * VERTICES_PER_SIDE);
        for z_index in 0..VERTICES_PER_SIDE {
            for x_index in 0..VERTICES_PER_SIDE {
                let (x, z) = vertex_position(x_index, z_index);
                heights.push(authored_height(x, z));
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

fn authored_height(x: f32, z: f32) -> f32 {
    1.8 * (x * 0.16).sin() * (z * 0.13).cos() + z * 0.08
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
            x: -12.0,
            y: 0.0,
            z: -8.0,
        };
        let mut first = BattlefieldTerrain::initial();
        let mut second = BattlefieldTerrain::initial();
        let crater = Crater::new(1.5, 4.0).unwrap();
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
}
