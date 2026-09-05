use crate::battlefield::{BattlefieldTerrain, is_within_bounds};
use crate::world::{WorldPosition, WorldVector};

const FIRING_ORIGIN_FORWARD_OFFSET: f32 = 2.1;
const FIRING_ORIGIN_HEIGHT: f32 = 1.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TankFiringRepresentation {
    pub direction: WorldVector,
    pub turret_forward: HorizontalDirection,
    pub muzzle_position: WorldPosition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerId {
    One,
    Two,
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
        }
    }

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
}

pub fn initial_tanks(terrain: &BattlefieldTerrain) -> [Tank; 2] {
    let player_one_direction = HorizontalDirection::new(3.0, 2.0);
    let player_two_direction = HorizontalDirection::new(-3.0, -2.0);

    [
        Tank::on_terrain(
            terrain,
            PlayerId::One,
            HorizontalPosition { x: -12.0, z: -8.0 },
            player_one_direction,
            player_one_direction,
        ),
        Tank::on_terrain(
            terrain,
            PlayerId::Two,
            HorizontalPosition { x: 12.0, z: 8.0 },
            player_two_direction,
            player_two_direction,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn firing_origin_is_above_and_ahead_of_the_turret() {
        let terrain = BattlefieldTerrain::initial();
        for tank in initial_tanks(&terrain) {
            let origin = tank.firing_origin();
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
    fn existing_tanks_do_not_move_when_terrain_changes_after_startup() {
        let terrain = BattlefieldTerrain::initial();
        let tanks = initial_tanks(&terrain);
        let mut changed_terrain = terrain.clone();
        changed_terrain.apply_crater(
            WorldPosition {
                x: -12.0,
                y: 0.0,
                z: -8.0,
            },
            crate::battlefield::Crater::default_development(),
        );

        assert_eq!(tanks[0].pose.position.y, terrain.height(-12.0, -8.0));
        assert_ne!(
            tanks[0].pose.position.y,
            changed_terrain.height(-12.0, -8.0)
        );
    }

    #[test]
    fn level_firing_representation_matches_existing_firing_origin() {
        let terrain = BattlefieldTerrain::initial();
        let tank = initial_tanks(&terrain)[0];
        let azimuth = crate::projectile::azimuth_from_horizontal_direction(
            tank.pose.turret_forward.x,
            tank.pose.turret_forward.z,
        )
        .unwrap();
        let representation = tank.firing_representation(
            crate::projectile::ShotParameters::new(tank.firing_origin(), azimuth, 0.0, 1.0)
                .unwrap()
                .launch_direction(),
        );

        assert_eq!(representation.muzzle_position, tank.firing_origin());
        assert!((representation.turret_forward.x - tank.pose.turret_forward.x).abs() < 0.000_1);
        assert!((representation.turret_forward.z - tank.pose.turret_forward.z).abs() < 0.000_1);
    }

    #[test]
    fn elevated_firing_representation_moves_muzzle_with_barrel() {
        let terrain = BattlefieldTerrain::initial();
        let tank = initial_tanks(&terrain)[0];
        let representation = tank.firing_representation(
            crate::projectile::ShotParameters::new(tank.firing_origin(), 90.0, 30.0, 1.0)
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
}
