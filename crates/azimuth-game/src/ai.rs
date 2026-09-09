use crate::{
    aiming::AimingState,
    projectile::azimuth_from_horizontal_direction,
    tank::{PlayerId, Tank},
    weapon::WeaponId,
};

const PREFERRED_ELEVATION_DEGREES: f32 = 47.0;
const AZIMUTH_ERROR_DEGREES: f32 = 9.0;
const ELEVATION_ERROR_DEGREES: f32 = 6.0;
const POWER_ERROR: f32 = 1.8;

/// A controller proposal only. It cannot launch or resolve a shot by itself.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AiFiringDecision {
    pub target: PlayerId,
    pub aim: AimingState,
    pub weapon: WeaponId,
}

/// Deliberately simple and imperfect: target the nearest living opponent, ignore wind, and add
/// bounded seeded error. Terrain and ordinary ballistics remain free to make this look foolish.
pub fn decide_firing(seed: &mut u64, actor: PlayerId, tanks: &[Tank]) -> Option<AiFiringDecision> {
    let shooter = tanks
        .iter()
        .find(|tank| tank.owner == actor && !tank.is_eliminated())?;
    let target = tanks
        .iter()
        .filter(|tank| tank.owner != actor && !tank.is_eliminated())
        .min_by(|left, right| {
            horizontal_distance_squared(shooter, left)
                .total_cmp(&horizontal_distance_squared(shooter, right))
                .then_with(|| left.owner.cmp(&right.owner))
        })?;
    let dx = target.pose.position.x - shooter.pose.position.x;
    let dz = target.pose.position.z - shooter.pose.position.z;
    let distance = (dx * dx + dz * dz).sqrt();
    let azimuth = azimuth_from_horizontal_direction(dx, dz).ok()?;
    let elevation = PREFERRED_ELEVATION_DEGREES
        + (target.pose.position.y - shooter.pose.position.y) * 0.35
        + signed_error(seed, ELEVATION_ERROR_DEGREES);
    let power = 8.0 + distance * 0.18 + signed_error(seed, POWER_ERROR);
    Some(AiFiringDecision {
        target: target.owner,
        aim: AimingState::new(
            azimuth + signed_error(seed, AZIMUTH_ERROR_DEGREES),
            elevation,
            power,
        ),
        weapon: WeaponId::BasicShell,
    })
}

fn horizontal_distance_squared(left: &Tank, right: &Tank) -> f32 {
    let dx = left.pose.position.x - right.pose.position.x;
    let dz = left.pose.position.z - right.pose.position.z;
    dx * dx + dz * dz
}

fn signed_error(seed: &mut u64, magnitude: f32) -> f32 {
    *seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
    let fraction = ((*seed >> 40) as f32) / ((1_u32 << 24) as f32);
    (fraction * 2.0 - 1.0) * magnitude
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{battlefield::BattlefieldTerrain, tank::initial_tanks_for_players};

    #[test]
    fn decision_targets_a_living_non_self_tank_deterministically() {
        let terrain = BattlefieldTerrain::initial();
        let ids = [PlayerId(1), PlayerId(2), PlayerId(3)];
        let mut tanks = initial_tanks_for_players(&terrain, &ids);
        tanks[1].health = 0;
        let mut first_seed = 44;
        let mut second_seed = 44;
        let first = decide_firing(&mut first_seed, PlayerId(1), &tanks).unwrap();
        let second = decide_firing(&mut second_seed, PlayerId(1), &tanks).unwrap();
        assert_eq!(first, second);
        assert_eq!(first.target, PlayerId(3));
        assert_eq!(first.weapon, WeaponId::BasicShell);
        assert!((5.0..=85.0).contains(&first.aim.elevation_degrees));
        assert!((8.0..=30.0).contains(&first.aim.launch_speed));
    }

    #[test]
    fn farther_targets_receive_more_power_before_bounded_error() {
        let terrain = BattlefieldTerrain::initial();
        let ids = [PlayerId(1), PlayerId(2)];
        let mut tanks = initial_tanks_for_players(&terrain, &ids);
        tanks[0].pose.position.x = 0.0;
        tanks[0].pose.position.z = 0.0;
        tanks[1].pose.position.x = 10.0;
        tanks[1].pose.position.z = 0.0;
        let mut near_seed = 7;
        let near = decide_firing(&mut near_seed, PlayerId(1), &tanks).unwrap();
        tanks[1].pose.position.x = 80.0;
        let mut far_seed = 7;
        let far = decide_firing(&mut far_seed, PlayerId(1), &tanks).unwrap();
        assert!(far.aim.launch_speed > near.aim.launch_speed);
    }
}
