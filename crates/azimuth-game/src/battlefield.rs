pub const HALF_EXTENT: f32 = 20.0;

pub fn is_within_bounds(x: f32, z: f32) -> bool {
    (-HALF_EXTENT..=HALF_EXTENT).contains(&x) && (-HALF_EXTENT..=HALF_EXTENT).contains(&z)
}

pub fn terrain_height(x: f32, z: f32) -> f32 {
    // This is deliberately visual relief, not the beginning of Azimuth's terrain model.
    1.8 * (x * 0.16).sin() * (z * 0.13).cos() + z * 0.08
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
