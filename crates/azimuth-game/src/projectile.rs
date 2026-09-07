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

    pub fn direction_for_angles(
        azimuth_degrees: f32,
        elevation_degrees: f32,
    ) -> Result<WorldVector, ProjectileParameterError> {
        Self::new(
            WorldPosition {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            azimuth_degrees,
            elevation_degrees,
            1.0,
        )
        .map(Self::launch_direction)
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

    pub fn downward_acceleration(self) -> f32 {
        self.downward_acceleration
    }

    fn acceleration(self) -> WorldVector {
        WorldVector {
            y: -self.downward_acceleration,
            ..WorldVector::ZERO
        }
    }
}

/// A constant horizontal acceleration applied to every projectile in a match.
///
/// This is deliberately a gameplay approximation: the vector says where wind pushes a
/// projectile *toward*, rather than modelling air resistance or a wind velocity. Keeping Y at
/// zero leaves vertical ballistics owned solely by [`Gravity`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Wind {
    horizontal_acceleration: WorldVector,
}

/// A projectile-specific multiplier for the match's horizontal wind acceleration.
///
/// This is a deliberate gameplay value, not mass or an aerodynamic coefficient. Keeping it on
/// the projectile snapshots wind behaviour when a shot is committed, while gravity stays shared.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindResponse(f32);

impl WindResponse {
    pub const NORMAL: Self = Self(1.0);

    pub fn new(factor: f32) -> Result<Self, ProjectileParameterError> {
        if !factor.is_finite() || factor < 0.0 {
            return Err(ProjectileParameterError::WindResponse);
        }
        Ok(Self(factor))
    }

    pub fn factor(self) -> f32 {
        self.0
    }
}

impl Wind {
    pub fn new(horizontal_acceleration: WorldVector) -> Result<Self, ProjectileParameterError> {
        if !horizontal_acceleration.x.is_finite()
            || !horizontal_acceleration.y.is_finite()
            || !horizontal_acceleration.z.is_finite()
            || horizontal_acceleration.y != 0.0
        {
            return Err(ProjectileParameterError::Wind);
        }

        Ok(Self {
            horizontal_acceleration,
        })
    }

    pub fn horizontal_acceleration(self) -> WorldVector {
        self.horizontal_acceleration
    }

    pub fn strength(self) -> f32 {
        (self.horizontal_acceleration.x * self.horizontal_acceleration.x
            + self.horizontal_acceleration.z * self.horizontal_acceleration.z)
            .sqrt()
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
    wind_response: WindResponse,
    elapsed_steps: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TerrainImpact {
    pub position: WorldPosition,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProjectileAdvance {
    Active,
    TerrainImpact(TerrainImpact),
    OutOfBounds,
}

impl Projectile {
    #[cfg(test)]
    pub fn launch(parameters: ShotParameters) -> Self {
        Self::launch_with_wind_response(parameters, WindResponse::NORMAL)
    }

    pub fn launch_with_wind_response(
        parameters: ShotParameters,
        wind_response: WindResponse,
    ) -> Self {
        Self {
            position: parameters.launch_position,
            velocity: parameters.launch_velocity(),
            wind_response,
            elapsed_steps: 0,
        }
    }

    #[cfg(test)]
    pub fn wind_response(self) -> WindResponse {
        self.wind_response
    }

    pub fn elapsed_seconds(self) -> f32 {
        self.elapsed_steps as f32 * FIXED_STEP_SECONDS
    }

    /// Advances one fixed step and resolves an in-bounds terrain crossing along the travelled
    /// segment. Twenty-four bisection iterations make the contact precise enough for the compact
    /// battlefield while keeping work and results fixed for deterministic simulation.
    pub fn advance_with_terrain<F>(
        &mut self,
        gravity: Gravity,
        wind: Wind,
        limits: SimulationLimits,
        terrain_height: F,
    ) -> ProjectileAdvance
    where
        F: Fn(f32, f32) -> Option<f32>,
    {
        let previous_position = self.position;
        let acceleration = gravity.acceleration().added(
            wind.horizontal_acceleration()
                .scaled(self.wind_response.factor()),
        );
        let step = FIXED_STEP_SECONDS;
        let displacement = self
            .velocity
            .scaled(step)
            .added(acceleration.scaled(0.5 * step * step));
        let candidate_position = self.position.translated(displacement);
        let candidate_velocity = self.velocity.added(acceleration.scaled(step));

        self.velocity = candidate_velocity;
        self.elapsed_steps += 1;

        if let (Some(previous_height), Some(candidate_height)) = (
            terrain_height(previous_position.x, previous_position.z),
            terrain_height(candidate_position.x, candidate_position.z),
        ) && previous_position.y > previous_height
            && candidate_position.y <= candidate_height
        {
            let impact_position =
                refine_terrain_impact(previous_position, candidate_position, terrain_height);
            self.position = impact_position;
            return ProjectileAdvance::TerrainImpact(TerrainImpact {
                position: impact_position,
            });
        }

        self.position = candidate_position;
        if limits.contains(*self) {
            ProjectileAdvance::Active
        } else {
            ProjectileAdvance::OutOfBounds
        }
    }
}

fn refine_terrain_impact<F>(
    mut above: WorldPosition,
    mut below: WorldPosition,
    terrain_height: F,
) -> WorldPosition
where
    F: Fn(f32, f32) -> Option<f32>,
{
    for _ in 0..24 {
        let midpoint = interpolate_position(above, below, 0.5);
        let height = terrain_height(midpoint.x, midpoint.z)
            .expect("terrain crossing refinement must remain within terrain bounds");

        if midpoint.y > height {
            above = midpoint;
        } else {
            below = midpoint;
        }
    }

    let height = terrain_height(below.x, below.z)
        .expect("terrain crossing refinement must remain within terrain bounds");
    WorldPosition { y: height, ..below }
}

fn interpolate_position(start: WorldPosition, end: WorldPosition, fraction: f32) -> WorldPosition {
    WorldPosition {
        x: start.x + (end.x - start.x) * fraction,
        y: start.y + (end.y - start.y) * fraction,
        z: start.z + (end.z - start.z) * fraction,
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
    Wind,
    WindResponse,
    HorizontalDirection,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 0.000_1;

    #[test]
    fn gravity_exposes_its_validated_downward_acceleration() {
        assert_eq!(Gravity::new(8.0).unwrap().downward_acceleration(), 8.0);
    }

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < EPSILON,
            "expected {expected}, got {actual}"
        );
    }

    fn no_terrain(_: f32, _: f32) -> Option<f32> {
        None
    }

    fn calm_wind() -> Wind {
        Wind::new(WorldVector::ZERO).unwrap()
    }

    fn advance_without_terrain(
        projectile: &mut Projectile,
        gravity: Gravity,
        limits: SimulationLimits,
    ) -> bool {
        matches!(
            projectile.advance_with_terrain(gravity, calm_wind(), limits, no_terrain),
            ProjectileAdvance::Active
        )
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
    fn wind_is_finite_horizontal_and_exposes_its_strength() {
        let wind = Wind::new(WorldVector {
            x: 3.0,
            y: 0.0,
            z: 4.0,
        })
        .unwrap();
        assert_eq!(wind.horizontal_acceleration().y, 0.0);
        assert_close(wind.strength(), 5.0);
        assert_eq!(calm_wind().strength(), 0.0);
        assert_eq!(
            Wind::new(WorldVector {
                x: 1.0,
                y: 0.1,
                z: 0.0
            }),
            Err(ProjectileParameterError::Wind)
        );
        assert_eq!(
            Wind::new(WorldVector {
                x: f32::NAN,
                y: 0.0,
                z: 0.0
            }),
            Err(ProjectileParameterError::Wind)
        );
    }

    #[test]
    fn reusable_direction_uses_the_same_angle_convention_as_shot_parameters() {
        let parameters = ShotParameters::new(
            WorldPosition {
                x: 2.0,
                y: 3.0,
                z: 4.0,
            },
            450.0,
            30.0,
            12.0,
        )
        .unwrap();

        assert_eq!(
            ShotParameters::direction_for_angles(450.0, 30.0).unwrap(),
            parameters.launch_direction()
        );
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

        assert!(advance_without_terrain(
            &mut projectile,
            gravity,
            SimulationLimits::DEVELOPMENT
        ));
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
            assert!(advance_without_terrain(
                &mut projectile,
                gravity,
                SimulationLimits::DEVELOPMENT
            ));
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
    fn zero_wind_preserves_the_gravity_only_trajectory() {
        let gravity = Gravity::new(8.0).unwrap();
        let mut calm = launch(0.0, 45.0, 10.0);
        let mut explicit_zero = calm;

        for _ in 0..120 {
            assert!(advance_without_terrain(
                &mut calm,
                gravity,
                SimulationLimits::DEVELOPMENT
            ));
            assert!(matches!(
                explicit_zero.advance_with_terrain(
                    gravity,
                    Wind::new(WorldVector::ZERO).unwrap(),
                    SimulationLimits::DEVELOPMENT,
                    no_terrain,
                ),
                ProjectileAdvance::Active
            ));
        }

        assert_eq!(calm, explicit_zero);
    }

    #[test]
    fn wind_accumulates_horizontal_displacement_without_vertical_acceleration() {
        let gravity = Gravity::new(8.0).unwrap();
        let wind = Wind::new(WorldVector {
            x: 1.5,
            y: 0.0,
            z: 0.0,
        })
        .unwrap();
        let mut projectile = launch(0.0, 45.0, 10.0);
        let initial = projectile;

        for _ in 0..120 {
            assert!(matches!(
                projectile.advance_with_terrain(
                    gravity,
                    wind,
                    SimulationLimits::DEVELOPMENT,
                    no_terrain
                ),
                ProjectileAdvance::Active
            ));
        }

        assert_close(projectile.position.x, initial.position.x + 0.75);
        assert_close(projectile.velocity.x, initial.velocity.x + 1.5);
        assert_close(
            projectile.position.y,
            initial.position.y + initial.velocity.y - 4.0,
        );
        assert_close(projectile.velocity.y, initial.velocity.y - 8.0);
    }

    #[test]
    fn reverse_stronger_and_longer_wind_exposure_change_horizontal_results_predictably() {
        let gravity = Gravity::new(0.0).unwrap();
        let toward_x = Wind::new(WorldVector {
            x: 1.5,
            y: 0.0,
            z: 0.0,
        })
        .unwrap();
        let away_x = Wind::new(WorldVector {
            x: -1.5,
            y: 0.0,
            z: 0.0,
        })
        .unwrap();
        let stronger = Wind::new(WorldVector {
            x: 3.0,
            y: 0.0,
            z: 0.0,
        })
        .unwrap();
        let mut positive = launch(0.0, 60.0, 10.0);
        let mut negative = positive;
        let mut strong = positive;
        let mut short = positive;

        for _ in 0..240 {
            for (projectile, wind) in [
                (&mut positive, toward_x),
                (&mut negative, away_x),
                (&mut strong, stronger),
            ] {
                assert!(matches!(
                    projectile.advance_with_terrain(gravity, wind, generous_limits(), no_terrain),
                    ProjectileAdvance::Active
                ));
            }
            if short.elapsed_steps == 119 {
                break;
            }
            assert!(matches!(
                short.advance_with_terrain(gravity, toward_x, generous_limits(), no_terrain),
                ProjectileAdvance::Active
            ));
        }

        assert!(positive.position.x > 0.0 && negative.position.x < 0.0);
        assert!(strong.position.x > positive.position.x);
        assert!(positive.position.x > short.position.x);
    }

    #[test]
    fn crosswind_and_parallel_wind_change_the_expected_horizontal_axis() {
        let gravity = Gravity::new(0.0).unwrap();
        let crosswind = Wind::new(WorldVector {
            x: 1.5,
            y: 0.0,
            z: 0.0,
        })
        .unwrap();
        let downrange = Wind::new(WorldVector {
            x: 0.0,
            y: 0.0,
            z: -1.5,
        })
        .unwrap();
        let mut crosswind_shot = launch(0.0, 45.0, 10.0);
        let mut downrange_shot = crosswind_shot;
        for _ in 0..120 {
            assert!(matches!(
                crosswind_shot.advance_with_terrain(
                    gravity,
                    crosswind,
                    generous_limits(),
                    no_terrain
                ),
                ProjectileAdvance::Active
            ));
            assert!(matches!(
                downrange_shot.advance_with_terrain(
                    gravity,
                    downrange,
                    generous_limits(),
                    no_terrain
                ),
                ProjectileAdvance::Active
            ));
        }
        assert!(crosswind_shot.position.x > 0.0);
        assert_close(crosswind_shot.position.z, downrange_shot.position.z + 0.75);
        assert!(downrange_shot.position.z < crosswind_shot.position.z);
    }

    #[test]
    fn repeated_inputs_produce_identical_trajectories() {
        let gravity = Gravity::new(8.0).unwrap();
        let mut first = launch(30.0, 55.0, 18.0);
        let mut second = first;

        for _ in 0..300 {
            assert_eq!(
                advance_without_terrain(&mut first, gravity, SimulationLimits::DEVELOPMENT),
                advance_without_terrain(&mut second, gravity, SimulationLimits::DEVELOPMENT)
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
            if !advance_without_terrain(&mut projectile, gravity, SimulationLimits::DEVELOPMENT) {
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
            assert!(advance_without_terrain(
                &mut projectile,
                Gravity::new(0.0).unwrap(),
                SimulationLimits::DEVELOPMENT
            ));
        }

        assert_eq!(projectile.velocity, initial_velocity);
    }

    #[test]
    fn stronger_gravity_produces_a_lower_equal_duration_trajectory() {
        let mut weak = launch(0.0, 45.0, 18.0);
        let mut strong = weak;

        for _ in 0..120 {
            assert!(advance_without_terrain(
                &mut weak,
                Gravity::new(4.0).unwrap(),
                SimulationLimits::DEVELOPMENT
            ));
            assert!(advance_without_terrain(
                &mut strong,
                Gravity::new(12.0).unwrap(),
                SimulationLimits::DEVELOPMENT
            ));
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
            wind_response: WindResponse::NORMAL,
            elapsed_steps: 0,
        };
        assert!(advance_without_terrain(
            &mut below_terrain,
            Gravity::new(0.0).unwrap(),
            SimulationLimits::DEVELOPMENT
        ));

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
            wind_response: WindResponse::NORMAL,
            elapsed_steps: 0,
        };
        assert!(!advance_without_terrain(
            &mut outside_horizontal,
            Gravity::new(0.0).unwrap(),
            SimulationLimits::DEVELOPMENT
        ));
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

    fn flat_terrain(_: f32, _: f32) -> Option<f32> {
        Some(0.0)
    }

    fn sloped_terrain(x: f32, z: f32) -> Option<f32> {
        Some(0.25 * x - 0.1 * z)
    }

    fn generous_limits() -> SimulationLimits {
        SimulationLimits {
            horizontal_extent: 10_000.0,
            minimum_y: -10_000.0,
            maximum_y: 10_000.0,
            maximum_flight_seconds: 20.0,
        }
    }

    #[test]
    fn terrain_advance_keeps_a_projectile_above_terrain_active() {
        let mut projectile = launch(0.0, 0.0, 12.0);

        assert_eq!(
            projectile.advance_with_terrain(
                Gravity::new(0.0).unwrap(),
                calm_wind(),
                SimulationLimits::DEVELOPMENT,
                flat_terrain,
            ),
            ProjectileAdvance::Active
        );
    }

    #[test]
    fn terrain_advance_resolves_a_flat_ground_crossing() {
        let mut projectile = Projectile {
            position: WorldPosition {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            velocity: WorldVector {
                y: -240.0,
                ..WorldVector::ZERO
            },
            wind_response: WindResponse::NORMAL,
            elapsed_steps: 0,
        };

        let outcome = projectile.advance_with_terrain(
            Gravity::new(0.0).unwrap(),
            calm_wind(),
            generous_limits(),
            flat_terrain,
        );

        assert_eq!(
            outcome,
            ProjectileAdvance::TerrainImpact(TerrainImpact {
                position: projectile.position,
            })
        );
        assert_close(projectile.position.y, 0.0);
        assert_close(projectile.velocity.y, -240.0);
    }

    #[test]
    fn terrain_advance_treats_endpoint_contact_as_one_impact() {
        let mut projectile = Projectile {
            position: WorldPosition {
                x: 0.0,
                y: 2.0,
                z: 0.0,
            },
            velocity: WorldVector {
                y: -240.0,
                ..WorldVector::ZERO
            },
            wind_response: WindResponse::NORMAL,
            elapsed_steps: 0,
        };

        assert!(matches!(
            projectile.advance_with_terrain(
                Gravity::new(0.0).unwrap(),
                calm_wind(),
                generous_limits(),
                flat_terrain,
            ),
            ProjectileAdvance::TerrainImpact(_)
        ));
        assert_close(projectile.position.y, 0.0);
    }

    #[test]
    fn terrain_advance_prevents_high_speed_tunnelling() {
        let mut projectile = Projectile {
            position: WorldPosition {
                x: 0.0,
                y: 5.0,
                z: 0.0,
            },
            velocity: WorldVector {
                y: -12_000.0,
                ..WorldVector::ZERO
            },
            wind_response: WindResponse::NORMAL,
            elapsed_steps: 0,
        };

        let outcome = projectile.advance_with_terrain(
            Gravity::new(0.0).unwrap(),
            calm_wind(),
            generous_limits(),
            flat_terrain,
        );

        assert!(matches!(outcome, ProjectileAdvance::TerrainImpact(_)));
        assert!(projectile.position.y.abs() < 0.01);
    }

    #[test]
    fn terrain_advance_resolves_contact_on_a_slope() {
        let mut projectile = Projectile {
            position: WorldPosition {
                x: 0.0,
                y: 2.0,
                z: 0.0,
            },
            velocity: WorldVector {
                x: 120.0,
                y: -300.0,
                z: 60.0,
            },
            wind_response: WindResponse::NORMAL,
            elapsed_steps: 0,
        };

        let outcome = projectile.advance_with_terrain(
            Gravity::new(0.0).unwrap(),
            calm_wind(),
            generous_limits(),
            sloped_terrain,
        );

        assert!(matches!(outcome, ProjectileAdvance::TerrainImpact(_)));
        assert!(
            (projectile.position.y
                - sloped_terrain(projectile.position.x, projectile.position.z).unwrap())
            .abs()
                < 0.01
        );
    }

    #[test]
    fn terrain_advance_is_repeatable_under_different_valid_gravity() {
        for gravity in [0.0, 8.0, 16.0] {
            let mut first = Projectile {
                position: WorldPosition {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                velocity: WorldVector {
                    x: 24.0,
                    y: -240.0,
                    ..WorldVector::ZERO
                },
                wind_response: WindResponse::NORMAL,
                elapsed_steps: 0,
            };
            let mut second = first;
            let gravity = Gravity::new(gravity).unwrap();

            assert_eq!(
                first.advance_with_terrain(gravity, calm_wind(), generous_limits(), sloped_terrain),
                second.advance_with_terrain(
                    gravity,
                    calm_wind(),
                    generous_limits(),
                    sloped_terrain
                )
            );
            assert_eq!(first, second);
        }
    }

    #[test]
    fn terrain_advance_keeps_non_impact_termination_distinct() {
        let mut projectile = Projectile {
            position: WorldPosition {
                x: 60.0,
                y: 5.0,
                z: 0.0,
            },
            velocity: WorldVector {
                x: 1.0,
                ..WorldVector::ZERO
            },
            wind_response: WindResponse::NORMAL,
            elapsed_steps: 0,
        };

        assert_eq!(
            projectile.advance_with_terrain(
                Gravity::new(0.0).unwrap(),
                calm_wind(),
                SimulationLimits::DEVELOPMENT,
                flat_terrain,
            ),
            ProjectileAdvance::OutOfBounds
        );
    }

    #[test]
    fn fixed_development_impact_shot_hits_the_non_flat_battlefield() {
        let terrain = crate::battlefield::BattlefieldTerrain::initial();
        let tank = crate::tank::initial_tanks(&terrain)[0];
        let azimuth = azimuth_from_horizontal_direction(
            tank.pose.turret_forward.x,
            tank.pose.turret_forward.z,
        )
        .unwrap();
        let mut projectile = Projectile::launch(
            ShotParameters::new(tank.firing_origin(), azimuth, 45.0, 14.0).unwrap(),
        );

        let outcome = (0..2_400)
            .find_map(|_| {
                match projectile.advance_with_terrain(
                    Gravity::new(8.0).unwrap(),
                    calm_wind(),
                    SimulationLimits::DEVELOPMENT,
                    |x, z| terrain.height_if_within_bounds(x, z),
                ) {
                    ProjectileAdvance::Active => None,
                    outcome => Some(outcome),
                }
            })
            .expect("fixed development impact shot must terminate");

        let ProjectileAdvance::TerrainImpact(impact) = outcome else {
            panic!("fixed development impact shot must hit terrain, got {outcome:?}");
        };
        assert!(
            (impact.position.y - terrain.height(impact.position.x, impact.position.z)).abs() < 0.01
        );
    }

    #[test]
    fn wind_altered_segment_resolves_one_impact_on_the_current_terrain_surface() {
        let gravity = Gravity::new(8.0).unwrap();
        let wind = Wind::new(WorldVector {
            x: 1.5,
            y: 0.0,
            z: 0.0,
        })
        .unwrap();
        let mut calm = launch(0.0, 45.0, 10.0);
        let mut windy = calm;
        let impact = |projectile: &mut Projectile, wind| {
            (0..2_400)
                .find_map(|_| {
                    match projectile.advance_with_terrain(
                        gravity,
                        wind,
                        generous_limits(),
                        flat_terrain,
                    ) {
                        ProjectileAdvance::Active => None,
                        ProjectileAdvance::TerrainImpact(impact) => Some(impact),
                        ProjectileAdvance::OutOfBounds => {
                            panic!("generous limits must retain the shot")
                        }
                    }
                })
                .expect("the downward arc must reach flat terrain")
        };
        let calm_impact = impact(&mut calm, calm_wind());
        let windy_impact = impact(&mut windy, wind);

        assert_close(windy_impact.position.y, 0.0);
        assert!(windy_impact.position.x > calm_impact.position.x);
        assert_close(windy_impact.position.z, calm_impact.position.z);
    }

    #[test]
    fn crater_removed_space_does_not_collide_with_the_old_surface() {
        let mut terrain = crate::battlefield::BattlefieldTerrain::initial();
        let old_height = terrain.height(0.0, 0.0);
        terrain.apply_crater(
            WorldPosition {
                x: 0.0,
                y: old_height,
                z: 0.0,
            },
            crate::battlefield::Crater::default_development(),
        );
        let mut projectile = Projectile {
            position: WorldPosition {
                x: 0.0,
                y: old_height + 0.1,
                z: 0.0,
            },
            velocity: WorldVector {
                y: -1.0,
                ..WorldVector::ZERO
            },
            wind_response: WindResponse::NORMAL,
            elapsed_steps: 0,
        };
        assert_eq!(
            projectile.advance_with_terrain(
                Gravity::new(0.0).unwrap(),
                calm_wind(),
                generous_limits(),
                |x, z| terrain.height_if_within_bounds(x, z)
            ),
            ProjectileAdvance::Active
        );
    }

    #[test]
    fn projectile_impacts_the_lower_crater_surface() {
        let mut terrain = crate::battlefield::BattlefieldTerrain::initial();
        let old_height = terrain.height(0.0, 0.0);
        terrain.apply_crater(
            WorldPosition {
                x: 0.0,
                y: old_height,
                z: 0.0,
            },
            crate::battlefield::Crater::default_development(),
        );
        let new_height = terrain.height(0.0, 0.0);
        let mut projectile = Projectile {
            position: WorldPosition {
                x: 0.0,
                y: new_height + 1.0,
                z: 0.0,
            },
            velocity: WorldVector {
                y: -240.0,
                ..WorldVector::ZERO
            },
            wind_response: WindResponse::NORMAL,
            elapsed_steps: 0,
        };
        assert!(matches!(
            projectile.advance_with_terrain(
                Gravity::new(0.0).unwrap(),
                calm_wind(),
                generous_limits(),
                |x, z| terrain.height_if_within_bounds(x, z)
            ),
            ProjectileAdvance::TerrainImpact(_)
        ));
        assert_close(projectile.position.y, new_height);
    }

    #[test]
    fn wind_response_scales_every_horizontal_direction_without_changing_gravity() {
        let gravity = Gravity::new(8.0).unwrap();
        let response = WindResponse::new(0.4).unwrap();
        let winds = [
            WorldVector {
                x: 1.5,
                y: 0.0,
                z: 0.0,
            },
            WorldVector {
                x: -1.5,
                y: 0.0,
                z: 0.0,
            },
            WorldVector {
                x: 0.0,
                y: 0.0,
                z: -1.5,
            },
            WorldVector {
                x: 0.0,
                y: 0.0,
                z: 1.5,
            },
        ];

        for vector in winds {
            let wind = Wind::new(vector).unwrap();
            let mut normal = launch(0.0, 45.0, 10.0);
            let mut calm = normal;
            let mut reduced = Projectile::launch_with_wind_response(
                ShotParameters::new(normal.position, 0.0, 45.0, 10.0).unwrap(),
                response,
            );
            for _ in 0..120 {
                assert!(matches!(
                    normal.advance_with_terrain(gravity, wind, generous_limits(), no_terrain),
                    ProjectileAdvance::Active
                ));
                assert!(matches!(
                    calm.advance_with_terrain(gravity, calm_wind(), generous_limits(), no_terrain),
                    ProjectileAdvance::Active
                ));
                assert!(matches!(
                    reduced.advance_with_terrain(gravity, wind, generous_limits(), no_terrain),
                    ProjectileAdvance::Active
                ));
            }
            assert_close(
                reduced.position.x - calm.position.x,
                (normal.position.x - calm.position.x) * 0.4,
            );
            assert_close(
                reduced.position.z - calm.position.z,
                (normal.position.z - calm.position.z) * 0.4,
            );
            assert_close(reduced.position.y, normal.position.y);
            assert_close(reduced.velocity.y, normal.velocity.y);
        }
    }

    #[test]
    fn wind_response_is_validated_and_zero_wind_is_invariant() {
        assert_eq!(
            WindResponse::new(-0.1),
            Err(ProjectileParameterError::WindResponse)
        );
        assert_eq!(
            WindResponse::new(f32::NAN),
            Err(ProjectileParameterError::WindResponse)
        );
        let mut normal = launch(0.0, 45.0, 10.0);
        let mut reduced = Projectile::launch_with_wind_response(
            ShotParameters::new(normal.position, 0.0, 45.0, 10.0).unwrap(),
            WindResponse::new(0.4).unwrap(),
        );
        for _ in 0..120 {
            normal.advance_with_terrain(
                Gravity::new(8.0).unwrap(),
                calm_wind(),
                generous_limits(),
                no_terrain,
            );
            reduced.advance_with_terrain(
                Gravity::new(8.0).unwrap(),
                calm_wind(),
                generous_limits(),
                no_terrain,
            );
        }
        assert_eq!(normal.position, reduced.position);
        assert_eq!(normal.velocity, reduced.velocity);
    }
}
