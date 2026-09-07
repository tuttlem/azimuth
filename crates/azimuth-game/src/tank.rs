use crate::battlefield::{BattlefieldTerrain, is_within_bounds};
use crate::projectile::{FIXED_STEP_SECONDS, Gravity};
use crate::world::{WorldPosition, WorldVector};

const FIRING_ORIGIN_FORWARD_OFFSET: f32 = 2.1;
const FIRING_ORIGIN_HEIGHT: f32 = 1.0;
pub const MOVEMENT_STEP_DISTANCE: f32 = 1.0;
pub const MOVEMENT_ALLOWANCE: u8 = 6;
pub const MAX_MOVEMENT_ELEVATION_CHANGE: f32 = 0.75;
pub const MAX_HEALTH: u8 = 100;
pub const SUPPORT_TOLERANCE: f32 = 0.05;

/// A tank is either resting on the current authoritative terrain or descends vertically until it
/// reaches it. This intentionally models tactical support rather than vehicle physics.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TankSupport {
    Supported,
    Falling { downward_velocity: f32 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TankFiringRepresentation {
    pub direction: WorldVector,
    pub turret_forward: HorizontalDirection,
    pub muzzle_position: WorldPosition,
}

/// Stable gameplay identity. The numeric value is allocated by Match Setup and never comes from
/// a display name, colour, current turn, or inventory position.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlayerId(pub u8);

impl PlayerId {
    #[allow(non_upper_case_globals)]
    pub const One: Self = Self(1);
    #[allow(non_upper_case_globals)]
    pub const Two: Self = Self(2);

    #[cfg(test)]
    pub fn other(self) -> Self {
        if self == Self::One {
            Self::Two
        } else {
            Self::One
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MovementDirection {
    NegativeZ,
    NegativeX,
    PositiveZ,
    PositiveX,
}

impl MovementDirection {
    pub fn horizontal_offset(self) -> (f32, f32) {
        match self {
            Self::NegativeZ => (0.0, -MOVEMENT_STEP_DISTANCE),
            Self::NegativeX => (-MOVEMENT_STEP_DISTANCE, 0.0),
            Self::PositiveZ => (0.0, MOVEMENT_STEP_DISTANCE),
            Self::PositiveX => (MOVEMENT_STEP_DISTANCE, 0.0),
        }
    }

    pub fn horizontal_direction(self) -> HorizontalDirection {
        let (x, z) = self.horizontal_offset();
        HorizontalDirection::new(x, z)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MovementRejection {
    Bounds,
    Slope,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HorizontalPosition {
    pub x: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HorizontalDirection {
    pub x: f32,
    pub z: f32,
}

impl HorizontalDirection {
    pub fn new(x: f32, z: f32) -> Self {
        let length = (x * x + z * z).sqrt();
        assert!(length > 0.0, "tank directions must not be zero");

        Self {
            x: x / length,
            z: z / length,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TankPose {
    pub position: WorldPosition,
    pub body_forward: HorizontalDirection,
    pub turret_forward: HorizontalDirection,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tank {
    pub owner: PlayerId,
    pub pose: TankPose,
    pub health: u8,
    pub support: TankSupport,
}

impl Tank {
    fn on_terrain(
        terrain: &BattlefieldTerrain,
        owner: PlayerId,
        horizontal_position: HorizontalPosition,
        body_forward: HorizontalDirection,
        turret_forward: HorizontalDirection,
    ) -> Self {
        assert!(
            is_within_bounds(horizontal_position.x, horizontal_position.z),
            "initial tank positions must remain within the battlefield"
        );

        Self {
            owner,
            pose: TankPose {
                position: WorldPosition {
                    x: horizontal_position.x,
                    y: terrain.height(horizontal_position.x, horizontal_position.z),
                    z: horizontal_position.z,
                },
                body_forward,
                turret_forward,
            },
            health: MAX_HEALTH,
            support: TankSupport::Supported,
        }
    }

    pub fn is_eliminated(self) -> bool {
        self.health == 0
    }

    pub fn apply_damage(&mut self, damage: u8) {
        self.health = self.health.saturating_sub(damage);
    }

    pub fn is_settling(self) -> bool {
        matches!(self.support, TankSupport::Falling { .. })
    }

    /// Re-evaluates the base-point support after terrain changes. The tank's X/Z and directions
    /// are deliberately preserved: craters alter the terrain, and gravity supplies only vertical
    /// response in this first model.
    pub fn reconcile_support(&mut self, terrain: &BattlefieldTerrain) {
        if self.is_eliminated() {
            return;
        }

        let ground = terrain.height(self.pose.position.x, self.pose.position.z);
        if self.pose.position.y - ground <= SUPPORT_TOLERANCE {
            self.pose.position.y = ground;
            self.support = TankSupport::Supported;
        } else if !self.is_settling() {
            self.support = TankSupport::Falling {
                downward_velocity: 0.0,
            };
        }
    }

    /// Advances one authoritative fixed settling step. Querying the current surface on every
    /// step keeps contact correct even if later terrain changes arrive before the tank lands.
    pub fn advance_settling(&mut self, terrain: &BattlefieldTerrain, gravity: Gravity) {
        if self.is_eliminated() {
            return;
        }
        let TankSupport::Falling { downward_velocity } = self.support else {
            return;
        };

        let acceleration = gravity.downward_acceleration();
        let candidate_y = self.pose.position.y
            - downward_velocity * FIXED_STEP_SECONDS
            - 0.5 * acceleration * FIXED_STEP_SECONDS * FIXED_STEP_SECONDS;
        let ground = terrain.height(self.pose.position.x, self.pose.position.z);
        if candidate_y <= ground {
            self.pose.position.y = ground;
            self.support = TankSupport::Supported;
        } else {
            self.pose.position.y = candidate_y;
            self.support = TankSupport::Falling {
                downward_velocity: downward_velocity + acceleration * FIXED_STEP_SECONDS,
            };
        }
    }

    #[cfg(test)]
    pub fn firing_origin(self) -> WorldPosition {
        WorldPosition {
            x: self.pose.position.x + self.pose.turret_forward.x * FIRING_ORIGIN_FORWARD_OFFSET,
            y: self.pose.position.y + FIRING_ORIGIN_HEIGHT,
            z: self.pose.position.z + self.pose.turret_forward.z * FIRING_ORIGIN_FORWARD_OFFSET,
        }
    }

    pub fn firing_representation(self, direction: WorldVector) -> TankFiringRepresentation {
        let turret_forward = HorizontalDirection::new(direction.x, direction.z);
        let barrel_pivot = WorldPosition {
            x: self.pose.position.x,
            y: self.pose.position.y + FIRING_ORIGIN_HEIGHT,
            z: self.pose.position.z,
        };

        TankFiringRepresentation {
            direction,
            turret_forward,
            muzzle_position: barrel_pivot
                .translated(direction.scaled(FIRING_ORIGIN_FORWARD_OFFSET)),
        }
    }

    /// Produces a new tank pose only when the adjacent current terrain position is in bounds and
    /// passable. The caller owns the action allowance, so rejected terrain requests never mutate
    /// this tank and an accepted result can be committed before one allowance is consumed.
    pub fn step_on_terrain(
        self,
        terrain: &BattlefieldTerrain,
        direction: MovementDirection,
    ) -> Result<Self, MovementRejection> {
        let (offset_x, offset_z) = direction.horizontal_offset();
        let destination_x = self.pose.position.x + offset_x;
        let destination_z = self.pose.position.z + offset_z;
        let Some(elevation_change) = terrain.elevation_change_if_within_bounds(
            self.pose.position.x,
            self.pose.position.z,
            destination_x,
            destination_z,
        ) else {
            return Err(MovementRejection::Bounds);
        };
        if elevation_change > MAX_MOVEMENT_ELEVATION_CHANGE {
            return Err(MovementRejection::Slope);
        }

        let mut moved = self;
        moved.pose.position = WorldPosition {
            x: destination_x,
            y: terrain.height(destination_x, destination_z),
            z: destination_z,
        };
        moved.pose.body_forward = direction.horizontal_direction();
        moved.support = TankSupport::Supported;
        Ok(moved)
    }
}

pub fn initial_tanks_for_players(terrain: &BattlefieldTerrain, players: &[PlayerId]) -> Vec<Tank> {
    const POSITIONS: [(f32, f32); 8] = [
        (-12.0, -8.0),
        (12.0, 8.0),
        (-12.0, 8.0),
        (12.0, -8.0),
        (-18.0, 0.0),
        (18.0, 0.0),
        (0.0, -12.0),
        (0.0, 12.0),
    ];
    assert!(
        (2..=8).contains(&players.len()),
        "matches support two through eight tanks"
    );
    players
        .iter()
        .copied()
        .zip(POSITIONS)
        .map(|(owner, (x, z))| {
            let direction = HorizontalDirection::new(-x, -z);
            Tank::on_terrain(
                terrain,
                owner,
                HorizontalPosition { x, z },
                direction,
                direction,
            )
        })
        .collect()
}

/// Compatibility helper for existing two-player-focused domain tests.
#[cfg(test)]
pub fn initial_tanks(terrain: &BattlefieldTerrain) -> [Tank; 2] {
    initial_tanks_for_players(terrain, &[PlayerId::One, PlayerId::Two])
        .try_into()
        .expect("two requested player IDs must produce two tanks")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crater_beneath(tank: Tank) -> BattlefieldTerrain {
        let mut terrain = BattlefieldTerrain::initial();
        terrain.apply_crater(
            WorldPosition {
                x: tank.pose.position.x,
                y: 0.0,
                z: tank.pose.position.z,
            },
            crate::battlefield::Crater::default_development(),
        );
        terrain
    }

    #[test]
    fn initial_tanks_have_distinct_in_bounds_terrain_resolved_spawns() {
        let terrain = BattlefieldTerrain::initial();
        let [first, second] = initial_tanks(&terrain);

        assert_ne!(first.owner, second.owner);
        assert_ne!(first.pose.position, second.pose.position);

        for tank in [first, second] {
            assert!(is_within_bounds(tank.pose.position.x, tank.pose.position.z));
            assert_eq!(
                tank.pose.position.y,
                terrain.height(tank.pose.position.x, tank.pose.position.z)
            );
        }
    }

    #[test]
    fn the_two_fixed_players_alternate_directly() {
        assert_eq!(PlayerId::One.other(), PlayerId::Two);
        assert_eq!(PlayerId::Two.other(), PlayerId::One);
    }

    #[test]
    fn firing_origin_is_above_and_ahead_of_the_turret() {
        let terrain = BattlefieldTerrain::initial();
        for tank in initial_tanks(&terrain) {
            let azimuth = crate::projectile::azimuth_from_horizontal_direction(
                tank.pose.turret_forward.x,
                tank.pose.turret_forward.z,
            )
            .unwrap();
            let origin = tank
                .firing_representation(
                    crate::projectile::ShotParameters::new(
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
                    .launch_direction(),
                )
                .muzzle_position;
            let offset_x = origin.x - tank.pose.position.x;
            let offset_z = origin.z - tank.pose.position.z;
            let forward_distance =
                offset_x * tank.pose.turret_forward.x + offset_z * tank.pose.turret_forward.z;

            assert!(origin.y > tank.pose.position.y);
            assert!(forward_distance > 0.0);
        }
    }

    #[test]
    fn direction_is_normalized_for_consistent_offsets() {
        let direction = HorizontalDirection::new(3.0, 4.0);

        assert!((direction.x * direction.x + direction.z * direction.z - 1.0).abs() < 0.000_001);
    }

    #[test]
    fn support_reconciliation_only_reacts_to_terrain_beneath_the_base() {
        let terrain = BattlefieldTerrain::initial();
        let mut tank = initial_tanks(&terrain)[0];
        let before_pose = tank.pose;
        let mut remote_terrain = terrain.clone();
        remote_terrain.apply_crater(
            WorldPosition {
                x: 12.0,
                y: 0.0,
                z: 8.0,
            },
            crate::battlefield::Crater::default_development(),
        );
        tank.reconcile_support(&remote_terrain);
        assert_eq!(tank.pose, before_pose);
        assert!(!tank.is_settling());

        let changed_terrain = crater_beneath(tank);
        tank.reconcile_support(&changed_terrain);
        assert!(tank.is_settling());
        assert_eq!(tank.pose.position.x, before_pose.position.x);
        assert_eq!(tank.pose.position.z, before_pose.position.z);
        assert_eq!(tank.pose.body_forward, before_pose.body_forward);
        assert_eq!(tank.pose.turret_forward, before_pose.turret_forward);
    }

    #[test]
    fn support_tolerance_and_terrain_rise_snap_a_tank_to_current_height() {
        let terrain = BattlefieldTerrain::initial();
        let mut tank = initial_tanks(&terrain)[0];
        let height = terrain.height(tank.pose.position.x, tank.pose.position.z);

        tank.pose.position.y = height + SUPPORT_TOLERANCE;
        tank.reconcile_support(&terrain);
        assert_eq!(tank.pose.position.y, height);
        assert!(!tank.is_settling());

        tank.pose.position.y = height - 1.0;
        tank.reconcile_support(&terrain);
        assert_eq!(tank.pose.position.y, height);
        assert!(!tank.is_settling());
    }

    #[test]
    fn settling_uses_gravity_and_clamps_to_the_current_terrain() {
        let terrain = BattlefieldTerrain::initial();
        let mut tank = initial_tanks(&terrain)[0];
        let changed_terrain = crater_beneath(tank);
        let starting_height = tank.pose.position.y;
        tank.reconcile_support(&changed_terrain);

        tank.advance_settling(
            &changed_terrain,
            crate::projectile::Gravity::new(8.0).unwrap(),
        );
        assert_eq!(
            tank.pose.position.y,
            starting_height
                - 0.5
                    * 8.0
                    * crate::projectile::FIXED_STEP_SECONDS
                    * crate::projectile::FIXED_STEP_SECONDS
        );
        assert!(tank.is_settling());

        for _ in 0..200 {
            tank.advance_settling(
                &changed_terrain,
                crate::projectile::Gravity::new(8.0).unwrap(),
            );
        }
        assert!(!tank.is_settling());
        assert_eq!(
            tank.pose.position.y,
            changed_terrain.height(tank.pose.position.x, tank.pose.position.z)
        );
    }

    #[test]
    fn settling_is_deterministic_and_zero_gravity_remains_unresolved() {
        let terrain = BattlefieldTerrain::initial();
        let mut first = initial_tanks(&terrain)[0];
        let mut second = first;
        let changed_terrain = crater_beneath(first);
        first.reconcile_support(&changed_terrain);
        second.reconcile_support(&changed_terrain);
        for _ in 0..12 {
            first.advance_settling(
                &changed_terrain,
                crate::projectile::Gravity::new(8.0).unwrap(),
            );
            second.advance_settling(
                &changed_terrain,
                crate::projectile::Gravity::new(8.0).unwrap(),
            );
        }
        assert_eq!(first, second);

        let mut zero_gravity = initial_tanks(&terrain)[0];
        zero_gravity.reconcile_support(&changed_terrain);
        let before = zero_gravity;
        zero_gravity.advance_settling(
            &changed_terrain,
            crate::projectile::Gravity::new(0.0).unwrap(),
        );
        assert_eq!(zero_gravity, before);
        assert!(zero_gravity.is_settling());
    }

    #[test]
    fn a_settled_tank_starts_later_movement_from_its_final_pose() {
        let terrain = BattlefieldTerrain::initial();
        let mut tank = initial_tanks(&terrain)[0];
        let changed_terrain = crater_beneath(tank);
        tank.reconcile_support(&changed_terrain);
        for _ in 0..200 {
            tank.advance_settling(
                &changed_terrain,
                crate::projectile::Gravity::new(8.0).unwrap(),
            );
        }
        let settled = tank.pose.position;

        let moved = tank
            .step_on_terrain(&changed_terrain, MovementDirection::PositiveX)
            .unwrap();
        assert_eq!(moved.pose.position.x, settled.x + MOVEMENT_STEP_DISTANCE);
        assert_eq!(moved.pose.position.z, settled.z);
        assert_eq!(
            moved.pose.position.y,
            changed_terrain.height(moved.pose.position.x, moved.pose.position.z)
        );
    }

    #[test]
    fn eliminated_tanks_do_not_enter_or_advance_settling() {
        let terrain = BattlefieldTerrain::initial();
        let mut tank = initial_tanks(&terrain)[0];
        let changed_terrain = crater_beneath(tank);
        tank.apply_damage(MAX_HEALTH);
        tank.reconcile_support(&changed_terrain);
        tank.advance_settling(
            &changed_terrain,
            crate::projectile::Gravity::new(8.0).unwrap(),
        );

        assert!(tank.is_eliminated());
        assert!(!tank.is_settling());
        assert_eq!(
            tank.pose.position.y,
            terrain.height(tank.pose.position.x, tank.pose.position.z)
        );
    }

    #[test]
    fn level_firing_representation_is_above_and_ahead_of_the_turret() {
        let terrain = BattlefieldTerrain::initial();
        let tank = initial_tanks(&terrain)[0];
        let azimuth = crate::projectile::azimuth_from_horizontal_direction(
            tank.pose.turret_forward.x,
            tank.pose.turret_forward.z,
        )
        .unwrap();
        let representation = tank.firing_representation(
            crate::projectile::ShotParameters::new(
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
            .launch_direction(),
        );

        assert_eq!(
            representation.muzzle_position,
            WorldPosition {
                x: tank.pose.position.x + tank.pose.turret_forward.x * FIRING_ORIGIN_FORWARD_OFFSET,
                y: tank.pose.position.y + FIRING_ORIGIN_HEIGHT,
                z: tank.pose.position.z + tank.pose.turret_forward.z * FIRING_ORIGIN_FORWARD_OFFSET,
            }
        );
        assert!((representation.turret_forward.x - tank.pose.turret_forward.x).abs() < 0.000_1);
        assert!((representation.turret_forward.z - tank.pose.turret_forward.z).abs() < 0.000_1);
    }

    #[test]
    fn elevated_firing_representation_moves_muzzle_with_barrel() {
        let terrain = BattlefieldTerrain::initial();
        let tank = initial_tanks(&terrain)[0];
        let representation = tank.firing_representation(
            crate::projectile::ShotParameters::new(
                WorldPosition {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                90.0,
                30.0,
                1.0,
            )
            .unwrap()
            .launch_direction(),
        );

        assert!((representation.direction.x - 30_f32.to_radians().cos()).abs() < 0.000_1);
        assert!(
            (representation.muzzle_position.x
                - (tank.pose.position.x
                    + FIRING_ORIGIN_FORWARD_OFFSET * 30_f32.to_radians().cos()))
            .abs()
                < 0.000_1
        );
        assert!(
            (representation.muzzle_position.y
                - (tank.pose.position.y
                    + FIRING_ORIGIN_HEIGHT
                    + FIRING_ORIGIN_FORWARD_OFFSET * 30_f32.to_radians().sin()))
            .abs()
                < 0.000_1
        );
    }

    #[test]
    fn movement_directions_are_one_unit_cardinal_steps() {
        assert_eq!(
            MovementDirection::NegativeZ.horizontal_offset(),
            (0.0, -1.0)
        );
        assert_eq!(
            MovementDirection::NegativeX.horizontal_offset(),
            (-1.0, 0.0)
        );
        assert_eq!(MovementDirection::PositiveZ.horizontal_offset(), (0.0, 1.0));
        assert_eq!(MovementDirection::PositiveX.horizontal_offset(), (1.0, 0.0));
        assert_eq!(MOVEMENT_ALLOWANCE, 6);
        assert!(MAX_MOVEMENT_ELEVATION_CHANGE.is_finite());
    }

    #[test]
    fn accepted_step_follows_current_terrain_and_changes_only_body_direction() {
        let terrain = BattlefieldTerrain::initial();
        let tank = initial_tanks(&terrain)[0];
        let moved = tank
            .step_on_terrain(&terrain, MovementDirection::PositiveX)
            .unwrap();

        assert_eq!(moved.pose.position.x, tank.pose.position.x + 1.0);
        assert_eq!(moved.pose.position.z, tank.pose.position.z);
        assert_eq!(
            moved.pose.position.y,
            terrain.height(moved.pose.position.x, moved.pose.position.z)
        );
        assert_eq!(moved.pose.turret_forward, tank.pose.turret_forward);
        assert_eq!(
            moved.pose.body_forward,
            MovementDirection::PositiveX.horizontal_direction()
        );
    }

    #[test]
    fn movement_rejections_preserve_the_original_tank() {
        let terrain = BattlefieldTerrain::initial();
        let mut tank = initial_tanks(&terrain)[0];
        tank.pose.position.x = 20.0;
        tank.pose.position.y = terrain.height(20.0, tank.pose.position.z);
        let before = tank;

        assert_eq!(
            tank.step_on_terrain(&terrain, MovementDirection::PositiveX),
            Err(MovementRejection::Bounds)
        );
        assert_eq!(tank, before);
    }

    #[test]
    fn deformed_terrain_is_grounded_when_passable_and_rejected_when_too_steep() {
        let mut terrain = BattlefieldTerrain::initial();
        let tank = initial_tanks(&terrain)[0];
        let impact = WorldPosition {
            x: tank.pose.position.x + 1.0,
            y: 0.0,
            z: tank.pose.position.z,
        };
        terrain.apply_crater(impact, crate::battlefield::Crater::default_development());
        let moved = tank
            .step_on_terrain(&terrain, MovementDirection::PositiveX)
            .unwrap();
        assert_eq!(
            moved.pose.position.y,
            terrain.height(moved.pose.position.x, moved.pose.position.z)
        );

        let mut steep_terrain = BattlefieldTerrain::initial();
        steep_terrain.apply_crater(
            WorldPosition {
                x: tank.pose.position.x,
                y: 0.0,
                z: tank.pose.position.z,
            },
            crate::battlefield::Crater::new(1.5, 4.0).unwrap(),
        );
        let before = tank;
        assert_eq!(
            tank.step_on_terrain(&steep_terrain, MovementDirection::PositiveX),
            Err(MovementRejection::Slope)
        );
        assert_eq!(tank, before);
    }
}
