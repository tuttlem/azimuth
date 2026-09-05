use crate::{projectile::ShotParameters, world::WorldPosition};

pub const MIN_ELEVATION_DEGREES: f32 = 5.0;
pub const MAX_ELEVATION_DEGREES: f32 = 85.0;
pub const MIN_LAUNCH_SPEED: f32 = 8.0;
pub const MAX_LAUNCH_SPEED: f32 = 30.0;
pub const FINE_ANGLE_DEGREES: f32 = 1.0;
pub const COARSE_ANGLE_DEGREES: f32 = 5.0;
pub const FINE_LAUNCH_SPEED: f32 = 0.5;
pub const COARSE_LAUNCH_SPEED: f32 = 2.5;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AimingState {
    pub azimuth_degrees: f32,
    pub elevation_degrees: f32,
    pub launch_speed: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AimAdjustment {
    AzimuthDecrease,
    AzimuthIncrease,
    ElevationIncrease,
    ElevationDecrease,
    PowerIncrease,
    PowerDecrease,
}

impl AimingState {
    pub fn new(azimuth_degrees: f32, elevation_degrees: f32, launch_speed: f32) -> Self {
        assert!(azimuth_degrees.is_finite(), "aim azimuth must be finite");
        assert!(
            elevation_degrees.is_finite(),
            "aim elevation must be finite"
        );
        assert!(launch_speed.is_finite(), "aim launch speed must be finite");

        Self {
            azimuth_degrees: azimuth_degrees.rem_euclid(360.0),
            elevation_degrees: elevation_degrees
                .clamp(MIN_ELEVATION_DEGREES, MAX_ELEVATION_DEGREES),
            launch_speed: launch_speed.clamp(MIN_LAUNCH_SPEED, MAX_LAUNCH_SPEED),
        }
    }

    pub fn apply(&mut self, adjustment: AimAdjustment, coarse: bool) {
        let angle_step = if coarse {
            COARSE_ANGLE_DEGREES
        } else {
            FINE_ANGLE_DEGREES
        };
        let speed_step = if coarse {
            COARSE_LAUNCH_SPEED
        } else {
            FINE_LAUNCH_SPEED
        };

        match adjustment {
            AimAdjustment::AzimuthDecrease => self.azimuth_degrees -= angle_step,
            AimAdjustment::AzimuthIncrease => self.azimuth_degrees += angle_step,
            AimAdjustment::ElevationIncrease => self.elevation_degrees += angle_step,
            AimAdjustment::ElevationDecrease => self.elevation_degrees -= angle_step,
            AimAdjustment::PowerIncrease => self.launch_speed += speed_step,
            AimAdjustment::PowerDecrease => self.launch_speed -= speed_step,
        }

        self.azimuth_degrees = self.azimuth_degrees.rem_euclid(360.0);
        self.elevation_degrees = self
            .elevation_degrees
            .clamp(MIN_ELEVATION_DEGREES, MAX_ELEVATION_DEGREES);
        self.launch_speed = self.launch_speed.clamp(MIN_LAUNCH_SPEED, MAX_LAUNCH_SPEED);
    }

    pub fn shot_parameters(self, muzzle_position: WorldPosition) -> ShotParameters {
        ShotParameters::new(
            muzzle_position,
            self.azimuth_degrees,
            self.elevation_degrees,
            self.launch_speed,
        )
        .expect("validated aiming state must create valid shot parameters")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn azimuth_wraps_and_bounds_clamp() {
        let mut state = AimingState::new(-1.0, -10.0, 99.0);
        assert_eq!(state.azimuth_degrees, 359.0);
        assert_eq!(state.elevation_degrees, MIN_ELEVATION_DEGREES);
        assert_eq!(state.launch_speed, MAX_LAUNCH_SPEED);

        state.apply(AimAdjustment::AzimuthIncrease, false);
        state.apply(AimAdjustment::ElevationDecrease, true);
        state.apply(AimAdjustment::PowerIncrease, true);
        assert_eq!(state.azimuth_degrees, 0.0);
        assert_eq!(state.elevation_degrees, MIN_ELEVATION_DEGREES);
        assert_eq!(state.launch_speed, MAX_LAUNCH_SPEED);
    }

    #[test]
    fn fine_and_coarse_adjustments_have_documented_sizes() {
        let mut state = AimingState::new(100.0, 45.0, 18.0);
        state.apply(AimAdjustment::AzimuthIncrease, false);
        state.apply(AimAdjustment::ElevationIncrease, true);
        state.apply(AimAdjustment::PowerDecrease, false);
        assert_eq!(state.azimuth_degrees, 101.0);
        assert_eq!(state.elevation_degrees, 50.0);
        assert_eq!(state.launch_speed, 17.5);
    }

    #[test]
    fn constructing_shot_parameters_does_not_change_aim() {
        let state = AimingState::new(120.0, 40.0, 16.0);
        let parameters = state.shot_parameters(WorldPosition {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        assert_eq!(parameters.azimuth_degrees, state.azimuth_degrees);
        assert_eq!(parameters.elevation_degrees, state.elevation_degrees);
        assert_eq!(parameters.launch_speed, state.launch_speed);
    }
}
