use crate::world::{WorldPosition, WorldVector};

pub const FIXED_STEP_SECONDS: f32 = 1.0 / 120.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShotParameters {
    pub launch_position: WorldPosition,
    pub azimuth_degrees: f32,
    pub elevation_degrees: f32,
    pub launch_speed: f32,
}

impl ShotParameters {
    pub fn new(
        launch_position: WorldPosition,
        azimuth_degrees: f32,
        elevation_degrees: f32,
        launch_speed: f32,
    ) -> Result<Self, ProjectileParameterError> {
        if !launch_position.is_finite() {
            return Err(ProjectileParameterError::LaunchPosition);
        }
        if !azimuth_degrees.is_finite() {
            return Err(ProjectileParameterError::Azimuth);
        }
        if !elevation_degrees.is_finite() || !(0.0..=90.0).contains(&elevation_degrees) {
            return Err(ProjectileParameterError::Elevation);
        }
        if !launch_speed.is_finite() || launch_speed <= 0.0 {
            return Err(ProjectileParameterError::LaunchSpeed);
        }

        Ok(Self {
            launch_position,
            azimuth_degrees: azimuth_degrees.rem_euclid(360.0),
            elevation_degrees,
            launch_speed,
        })
    }

    pub fn launch_direction(self) -> WorldVector {
        let azimuth = self.azimuth_degrees.to_radians();
        let elevation = self.elevation_degrees.to_radians();
        let horizontal_scale = elevation.cos();

        WorldVector {
            x: azimuth.sin() * horizontal_scale,
            y: elevation.sin(),
            z: -azimuth.cos() * horizontal_scale,
        }
    }

    pub fn launch_velocity(self) -> WorldVector {
        self.launch_direction().scaled(self.launch_speed)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Gravity {
    downward_acceleration: f32,
}

impl Gravity {
    pub fn new(downward_acceleration: f32) -> Result<Self, ProjectileParameterError> {
        if !downward_acceleration.is_finite() || downward_acceleration < 0.0 {
            return Err(ProjectileParameterError::Gravity);
        }

        Ok(Self {
            downward_acceleration,
        })
    }

    fn acceleration(self) -> WorldVector {
        WorldVector {
            y: -self.downward_acceleration,
            ..WorldVector::ZERO
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimulationLimits {
    horizontal_extent: f32,
    minimum_y: f32,
    maximum_y: f32,
    maximum_flight_seconds: f32,
}

impl SimulationLimits {
    pub const DEVELOPMENT: Self = Self {
        horizontal_extent: 60.0,
        minimum_y: -30.0,
        maximum_y: 100.0,
        maximum_flight_seconds: 20.0,
    };

    pub fn contains(self, projectile: Projectile) -> bool {
        projectile.position.x.abs() <= self.horizontal_extent
            && projectile.position.z.abs() <= self.horizontal_extent
            && projectile.position.y >= self.minimum_y
            && projectile.position.y <= self.maximum_y
            && projectile.elapsed_seconds() < self.maximum_flight_seconds
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Projectile {
    pub position: WorldPosition,
    pub velocity: WorldVector,
    elapsed_steps: u32,
}

impl Projectile {
    pub fn launch(parameters: ShotParameters) -> Self {
        Self {
            position: parameters.launch_position,
            velocity: parameters.launch_velocity(),
            elapsed_steps: 0,
        }
    }

    pub fn elapsed_seconds(self) -> f32 {
        self.elapsed_steps as f32 * FIXED_STEP_SECONDS
    }

    /// Advances one fixed, constant-acceleration step. Position uses the current velocity plus
    /// half the acceleration term so this simple model remains analytically understandable.
    pub fn advance(&mut self, gravity: Gravity, limits: SimulationLimits) -> bool {
        let acceleration = gravity.acceleration();
        let step = FIXED_STEP_SECONDS;
        let displacement = self
            .velocity
            .scaled(step)
            .added(acceleration.scaled(0.5 * step * step));

        self.position = self.position.translated(displacement);
        self.velocity = self.velocity.added(acceleration.scaled(step));
        self.elapsed_steps += 1;

        limits.contains(*self)
    }
}

pub fn azimuth_from_horizontal_direction(x: f32, z: f32) -> Result<f32, ProjectileParameterError> {
    if !x.is_finite() || !z.is_finite() || (x == 0.0 && z == 0.0) {
        return Err(ProjectileParameterError::HorizontalDirection);
    }

    Ok(x.atan2(-z).to_degrees().rem_euclid(360.0))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectileParameterError {
    LaunchPosition,
    Azimuth,
    Elevation,
    LaunchSpeed,
    Gravity,
    HorizontalDirection,
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

    fn launch(azimuth: f32, elevation: f32, speed: f32) -> Projectile {
        Projectile::launch(
            ShotParameters::new(
                WorldPosition {
                    x: 0.0,
                    y: 5.0,
                    z: 0.0,
                },
                azimuth,
                elevation,
                speed,
            )
            .unwrap(),
        )
    }

    #[test]
    fn cardinal_azimuths_use_the_documented_x_z_directions() {
        let expected = [
            (
                0.0,
                WorldVector {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
            ),
            (
                90.0,
                WorldVector {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
            ),
            (
                180.0,
                WorldVector {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
            ),
            (
                270.0,
                WorldVector {
                    x: -1.0,
                    y: 0.0,
                    z: 0.0,
                },
            ),
        ];

        for (azimuth, expected) in expected {
            let direction = ShotParameters::new(
                WorldPosition {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                azimuth,
                0.0,
                1.0,
            )
            .unwrap()
            .launch_direction();
            assert_close(direction.x, expected.x);
            assert_close(direction.y, expected.y);
            assert_close(direction.z, expected.z);
        }
    }

    #[test]
    fn elevation_controls_vertical_launch_direction() {
        let level = launch(0.0, 0.0, 1.0).velocity;
        let vertical = launch(0.0, 90.0, 1.0).velocity;

        assert_close(level.y, 0.0);
        assert_close(vertical.x, 0.0);
        assert_close(vertical.y, 1.0);
        assert_close(vertical.z, 0.0);
    }

    #[test]
    fn launch_speed_scales_the_unit_direction() {
        let projectile = launch(90.0, 0.0, 12.0);

        assert_close(projectile.velocity.x, 12.0);
        assert_close(projectile.velocity.y, 0.0);
        assert_close(projectile.velocity.z, 0.0);
    }

    #[test]
    fn horizontal_direction_maps_to_the_same_azimuth_convention() {
        assert_close(azimuth_from_horizontal_direction(0.0, -1.0).unwrap(), 0.0);
        assert_close(azimuth_from_horizontal_direction(1.0, 0.0).unwrap(), 90.0);
        assert_close(azimuth_from_horizontal_direction(0.0, 1.0).unwrap(), 180.0);
        assert_close(azimuth_from_horizontal_direction(-1.0, 0.0).unwrap(), 270.0);
    }

    #[test]
    fn gravity_changes_only_vertical_velocity() {
        let mut projectile = launch(45.0, 30.0, 18.0);
        let initial_velocity = projectile.velocity;
        let gravity = Gravity::new(8.0).unwrap();

        assert!(projectile.advance(gravity, SimulationLimits::DEVELOPMENT));
        assert_close(projectile.velocity.x, initial_velocity.x);
        assert_close(projectile.velocity.z, initial_velocity.z);
        assert_close(
            projectile.velocity.y,
            initial_velocity.y - 8.0 * FIXED_STEP_SECONDS,
        );
    }

    #[test]
    fn fixed_step_matches_constant_acceleration_after_one_second() {
        let mut projectile = launch(0.0, 45.0, 10.0);
        let initial_position = projectile.position;
        let initial_velocity = projectile.velocity;
        let gravity = Gravity::new(8.0).unwrap();

        for _ in 0..120 {
            assert!(projectile.advance(gravity, SimulationLimits::DEVELOPMENT));
        }

        assert_close(
            projectile.position.x,
            initial_position.x + initial_velocity.x,
        );
        assert_close(
            projectile.position.z,
            initial_position.z + initial_velocity.z,
        );
        assert_close(
            projectile.position.y,
            initial_position.y + initial_velocity.y - 0.5 * 8.0,
        );
        assert_close(projectile.velocity.y, initial_velocity.y - 8.0);
    }

    #[test]
    fn repeated_inputs_produce_identical_trajectories() {
        let gravity = Gravity::new(8.0).unwrap();
        let mut first = launch(30.0, 55.0, 18.0);
        let mut second = first;

        for _ in 0..300 {
            assert_eq!(
                first.advance(gravity, SimulationLimits::DEVELOPMENT),
                second.advance(gravity, SimulationLimits::DEVELOPMENT)
            );
            assert_eq!(first, second);
        }
    }

    #[test]
    fn upward_shot_reaches_an_apex_then_descends() {
        let gravity = Gravity::new(8.0).unwrap();
        let mut projectile = launch(0.0, 60.0, 18.0);
        let mut highest_y = projectile.position.y;
        let mut descended = false;

        for _ in 0..600 {
            if !projectile.advance(gravity, SimulationLimits::DEVELOPMENT) {
                break;
            }
            if projectile.position.y < highest_y {
                descended = true;
                break;
            }
            highest_y = highest_y.max(projectile.position.y);
        }

        assert!(highest_y > 5.0);
        assert!(descended);
    }

    #[test]
    fn zero_gravity_preserves_velocity() {
        let mut projectile = launch(10.0, 40.0, 18.0);
        let initial_velocity = projectile.velocity;

        for _ in 0..120 {
            assert!(projectile.advance(Gravity::new(0.0).unwrap(), SimulationLimits::DEVELOPMENT));
        }

        assert_eq!(projectile.velocity, initial_velocity);
    }

    #[test]
    fn stronger_gravity_produces_a_lower_equal_duration_trajectory() {
        let mut weak = launch(0.0, 45.0, 18.0);
        let mut strong = weak;

        for _ in 0..120 {
            assert!(weak.advance(Gravity::new(4.0).unwrap(), SimulationLimits::DEVELOPMENT));
            assert!(strong.advance(Gravity::new(12.0).unwrap(), SimulationLimits::DEVELOPMENT));
        }

        assert!(strong.position.y < weak.position.y);
        assert!(strong.velocity.y < weak.velocity.y);
    }

    #[test]
    fn limits_end_flight_but_terrain_has_no_effect() {
        let mut below_terrain = Projectile {
            position: WorldPosition {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            velocity: WorldVector::ZERO,
            elapsed_steps: 0,
        };
        assert!(below_terrain.advance(Gravity::new(0.0).unwrap(), SimulationLimits::DEVELOPMENT));

        let mut outside_horizontal = Projectile {
            position: WorldPosition {
                x: 60.0,
                y: 0.0,
                z: 0.0,
            },
            velocity: WorldVector {
                x: 1.0,
                ..WorldVector::ZERO
            },
            elapsed_steps: 0,
        };
        assert!(
            !outside_horizontal.advance(Gravity::new(0.0).unwrap(), SimulationLimits::DEVELOPMENT)
        );
    }

    #[test]
    fn invalid_parameters_are_rejected() {
        let position = WorldPosition {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        assert_eq!(
            ShotParameters::new(position, 0.0, -0.1, 1.0),
            Err(ProjectileParameterError::Elevation)
        );
        assert_eq!(
            ShotParameters::new(position, 0.0, 45.0, 0.0),
            Err(ProjectileParameterError::LaunchSpeed)
        );
        assert_eq!(Gravity::new(-1.0), Err(ProjectileParameterError::Gravity));
        assert_eq!(
            azimuth_from_horizontal_direction(0.0, 0.0),
            Err(ProjectileParameterError::HorizontalDirection)
        );
    }
}
