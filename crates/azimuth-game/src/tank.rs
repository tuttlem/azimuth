use crate::battlefield::{is_within_bounds, terrain_height};
use crate::world::WorldPosition;

const FIRING_ORIGIN_FORWARD_OFFSET: f32 = 2.1;
const FIRING_ORIGIN_HEIGHT: f32 = 1.0;

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
                    y: terrain_height(horizontal_position.x, horizontal_position.z),
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
}

pub fn initial_tanks() -> [Tank; 2] {
    let player_one_direction = HorizontalDirection::new(3.0, 2.0);
    let player_two_direction = HorizontalDirection::new(-3.0, -2.0);

    [
        Tank::on_terrain(
            PlayerId::One,
            HorizontalPosition { x: -12.0, z: -8.0 },
            player_one_direction,
            player_one_direction,
        ),
        Tank::on_terrain(
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
        let [first, second] = initial_tanks();

        assert_ne!(first.owner, second.owner);
        assert_ne!(first.pose.position, second.pose.position);

        for tank in [first, second] {
            assert!(is_within_bounds(tank.pose.position.x, tank.pose.position.z));
            assert_eq!(
                tank.pose.position.y,
                terrain_height(tank.pose.position.x, tank.pose.position.z)
            );
        }
    }

    #[test]
    fn firing_origin_is_above_and_ahead_of_the_turret() {
        for tank in initial_tanks() {
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
}
