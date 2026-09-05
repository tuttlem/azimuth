pub const HALF_EXTENT: f32 = 20.0;
pub const TERRAIN_CELLS_PER_SIDE: usize = 20;

const TERRAIN_CELL_SIZE: f32 = HALF_EXTENT * 2.0 / TERRAIN_CELLS_PER_SIDE as f32;

pub fn is_within_bounds(x: f32, z: f32) -> bool {
    (-HALF_EXTENT..=HALF_EXTENT).contains(&x) && (-HALF_EXTENT..=HALF_EXTENT).contains(&z)
}

pub fn terrain_vertex_height(x: f32, z: f32) -> f32 {
    1.8 * (x * 0.16).sin() * (z * 0.13).cos() + z * 0.08
}

/// Returns the height of the same triangle surface rendered by `terrain_mesh_positions`.
///
/// The authored relief function samples grid vertices, but the visible battlefield consists of
/// flat triangles between them. Interpolating those triangles keeps tank grounding and projectile
/// collision on the surface players see rather than on a separate smooth approximation.
pub fn terrain_height(x: f32, z: f32) -> f32 {
    assert!(
        is_within_bounds(x, z),
        "terrain height queries must remain within the battlefield"
    );

    let (x_index, x_fraction) = terrain_cell_coordinate(x);
    let (z_index, z_fraction) = terrain_cell_coordinate(z);
    let lower_x = -HALF_EXTENT + x_index as f32 * TERRAIN_CELL_SIZE;
    let lower_z = -HALF_EXTENT + z_index as f32 * TERRAIN_CELL_SIZE;
    let lower_left = terrain_vertex_height(lower_x, lower_z);
    let lower_right = terrain_vertex_height(lower_x + TERRAIN_CELL_SIZE, lower_z);
    let upper_left = terrain_vertex_height(lower_x, lower_z + TERRAIN_CELL_SIZE);
    let upper_right =
        terrain_vertex_height(lower_x + TERRAIN_CELL_SIZE, lower_z + TERRAIN_CELL_SIZE);

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

pub fn terrain_height_if_within_bounds(x: f32, z: f32) -> Option<f32> {
    is_within_bounds(x, z).then(|| terrain_height(x, z))
}

pub fn terrain_mesh_positions() -> Vec<[f32; 3]> {
    let vertices_per_side = TERRAIN_CELLS_PER_SIDE + 1;
    let mut positions = Vec::with_capacity(vertices_per_side * vertices_per_side);

    for z_index in 0..=TERRAIN_CELLS_PER_SIDE {
        for x_index in 0..=TERRAIN_CELLS_PER_SIDE {
            let x = -HALF_EXTENT + x_index as f32 * TERRAIN_CELL_SIZE;
            let z = -HALF_EXTENT + z_index as f32 * TERRAIN_CELL_SIZE;
            positions.push([x, terrain_vertex_height(x, z), z]);
        }
    }

    positions
}

pub fn terrain_mesh_indices() -> Vec<u32> {
    let vertices_per_side = TERRAIN_CELLS_PER_SIDE + 1;
    let mut indices = Vec::with_capacity(TERRAIN_CELLS_PER_SIDE * TERRAIN_CELLS_PER_SIDE * 6);

    for z_index in 0..TERRAIN_CELLS_PER_SIDE {
        for x_index in 0..TERRAIN_CELLS_PER_SIDE {
            let lower_left = (z_index * vertices_per_side + x_index) as u32;
            let lower_right = lower_left + 1;
            let upper_left = lower_left + vertices_per_side as u32;
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

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < EPSILON,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn terrain_height_is_deterministic_and_non_flat() {
        assert_eq!(terrain_height(4.0, -7.0), terrain_height(4.0, -7.0));
        assert_ne!(terrain_height(0.0, 0.0), terrain_height(8.0, 8.0));
    }

    #[test]
    fn bounds_include_edges_and_reject_points_outside_them() {
        assert!(is_within_bounds(-HALF_EXTENT, HALF_EXTENT));
        assert!(!is_within_bounds(HALF_EXTENT + 0.1, 0.0));
    }

    #[test]
    fn local_height_matches_every_rendered_vertex() {
        for position in terrain_mesh_positions() {
            assert_close(terrain_height(position[0], position[2]), position[1]);
        }
    }

    #[test]
    fn local_height_interpolates_the_rendered_cell_triangles() {
        let lower_left = terrain_height(-20.0, -20.0);
        let lower_right = terrain_height(-18.0, -20.0);
        let upper_left = terrain_height(-20.0, -18.0);
        let upper_right = terrain_height(-18.0, -18.0);

        assert_close(
            terrain_height(-19.5, -19.5),
            lower_left * 0.5 + lower_right * 0.25 + upper_left * 0.25,
        );
        assert_close(
            terrain_height(-18.5, -18.5),
            lower_right * 0.25 + upper_left * 0.25 + upper_right * 0.5,
        );
    }
}
