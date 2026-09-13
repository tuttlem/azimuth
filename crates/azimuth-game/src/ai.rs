use crate::{
    aiming::AimingState,
    battlefield::BattlefieldTerrain,
    match_setup::AiDifficulty,
    projectile::{Wind, azimuth_from_horizontal_direction},
    tank::{FIRING_ORIGIN_FORWARD_OFFSET, FIRING_ORIGIN_HEIGHT, PlayerId, Tank},
    weapon::{PlayerWeaponLoadout, WeaponId, weapon_price},
};

const GROUPING_RADIUS: f32 = 20.0;
const STRONG_WIND: f32 = 1.15;
const AI_GRAVITY_ESTIMATE: f32 = 8.0;

/// A controller proposal only. It cannot launch, spend ammunition, or resolve a shot by itself.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AiFiringDecision {
    pub target: PlayerId,
    pub aim: AimingState,
    pub weapon: WeaponId,
}

/// Terrain-aware Hard mode takes a visibly higher arc when samples along the direct line reveal a
/// ridge. It uses only the same current terrain humans can see; it does not predict impacts.
pub fn decide_firing_with_terrain(
    seed: &mut u64,
    actor: PlayerId,
    difficulty: AiDifficulty,
    tanks: &[Tank],
    loadout: PlayerWeaponLoadout,
    wind: Wind,
    terrain: &BattlefieldTerrain,
) -> Option<AiFiringDecision> {
    let mut decision = decide_firing(seed, actor, difficulty, tanks, loadout, wind)?;
    if difficulty != AiDifficulty::Hard {
        return Some(decision);
    }
    let shooter = tanks
        .iter()
        .find(|tank| tank.owner == actor && !tank.is_eliminated())?;
    let target = tanks
        .iter()
        .find(|tank| tank.owner == decision.target && !tank.is_eliminated())?;
    if direct_line_is_blocked(shooter, target, terrain) {
        decision.aim.elevation_degrees = (decision.aim.elevation_degrees + 22.0).min(85.0);
        decision.aim.launch_speed = (decision.aim.launch_speed + 3.0).min(30.0);
    }
    Some(decision)
}

fn direct_line_is_blocked(shooter: &Tank, target: &Tank, terrain: &BattlefieldTerrain) -> bool {
    (1..8).any(|step| {
        let fraction = step as f32 / 8.0;
        let x =
            shooter.pose.position.x + (target.pose.position.x - shooter.pose.position.x) * fraction;
        let z =
            shooter.pose.position.z + (target.pose.position.z - shooter.pose.position.z) * fraction;
        let line_height =
            shooter.pose.position.y + (target.pose.position.y - shooter.pose.position.y) * fraction;
        terrain
            .height_if_within_bounds(x, z)
            .is_some_and(|height| height > line_height + 3.0)
    })
}

/// Uses only public, current state. Fixed roles make AI fallibility understandable and tests
/// repeatable without an expensive, omniscient trajectory search.
pub fn decide_firing(
    seed: &mut u64,
    actor: PlayerId,
    difficulty: AiDifficulty,
    tanks: &[Tank],
    loadout: PlayerWeaponLoadout,
    wind: Wind,
) -> Option<AiFiringDecision> {
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
    let grouped_targets = tanks
        .iter()
        .filter(|other| !other.is_eliminated() && other.owner != actor)
        .filter(|other| horizontal_distance_squared(target, other) <= GROUPING_RADIUS.powi(2))
        .count();
    let (azimuth_error, elevation_error, power_error) = match difficulty {
        AiDifficulty::Easy => (12.0, 8.0, 2.5),
        AiDifficulty::Normal => (7.0, 4.0, 1.5),
        // Hard should feel threatening rather than merely a slightly tidier Normal opponent.
        // It still receives the same public state and bounded seeded error, so it can miss.
        // Hard aims from the improved ballistic estimate, then deliberately spreads shells
        // through the dangerous area instead of repeating one sterile pixel-perfect solution.
        AiDifficulty::Hard => (1.5, 0.75, 0.35),
    };
    let elevation = 47.0
        + (target.pose.position.y - shooter.pose.position.y) * 0.35
        + signed_error(seed, elevation_error);
    let rough_power = 8.0 + distance * 0.18;
    let power = if difficulty == AiDifficulty::Hard {
        let elevation_radians = elevation.to_radians();
        let muzzle_distance =
            (distance - FIRING_ORIGIN_FORWARD_OFFSET * elevation_radians.cos()).max(0.1);
        let muzzle_height_difference = target.pose.position.y
            - shooter.pose.position.y
            - FIRING_ORIGIN_HEIGHT
            - FIRING_ORIGIN_FORWARD_OFFSET * elevation_radians.sin();
        ballistic_power(muzzle_distance, muzzle_height_difference, elevation).unwrap_or(rough_power)
    } else {
        rough_power
    } + signed_error(seed, power_error);
    Some(AiFiringDecision {
        target: target.owner,
        weapon: select_weapon(difficulty, loadout, distance, grouped_targets, wind),
        aim: AimingState::new(
            compensated_azimuth(difficulty, dx, dz, elevation, power, wind)?
                + signed_error(seed, azimuth_error),
            elevation,
            power,
        ),
    })
}

/// Hard estimates ordinary ballistic drift from the same public wind and gravity values shown to
/// the match. Terrain and the remaining bounded input error still make it fallible.
fn compensated_azimuth(
    difficulty: AiDifficulty,
    dx: f32,
    dz: f32,
    elevation_degrees: f32,
    power: f32,
    wind: Wind,
) -> Option<f32> {
    if difficulty != AiDifficulty::Hard {
        return azimuth_from_horizontal_direction(dx, dz).ok();
    }
    let time = 2.0 * power * elevation_degrees.to_radians().sin() / AI_GRAVITY_ESTIMATE;
    let drift = wind.horizontal_acceleration();
    azimuth_from_horizontal_direction(
        dx - 0.5 * drift.x * time * time,
        dz - 0.5 * drift.z * time * time,
    )
    .ok()
}

/// Solve the ordinary no-drag ballistic equation for the selected angle. Hard uses this as its
/// starting power estimate instead of the deliberately rough beginner-AI range heuristic.
fn ballistic_power(
    horizontal_distance: f32,
    height_difference: f32,
    elevation_degrees: f32,
) -> Option<f32> {
    let angle = elevation_degrees.to_radians();
    let denominator =
        2.0 * angle.cos().powi(2) * (horizontal_distance * angle.tan() - height_difference);
    (denominator > 0.0)
        .then(|| (AI_GRAVITY_ESTIMATE * horizontal_distance.powi(2) / denominator).sqrt())
}

fn select_weapon(
    difficulty: AiDifficulty,
    loadout: PlayerWeaponLoadout,
    distance: f32,
    grouped_targets: usize,
    wind: Wind,
) -> WeaponId {
    let available = |weapon| loadout.availability(weapon).is_available();
    let tactical = difficulty != AiDifficulty::Easy;
    if tactical && grouped_targets >= 2 {
        if available(WeaponId::ClusterBomb) {
            return WeaponId::ClusterBomb;
        }
        if available(WeaponId::Mirv) {
            return WeaponId::Mirv;
        }
    }
    if tactical && wind.strength() >= STRONG_WIND && available(WeaponId::HeavyShell) {
        return WeaponId::HeavyShell;
    }
    if distance <= 34.0 && available(WeaponId::HighExplosive) {
        return WeaponId::HighExplosive;
    }
    if tactical && distance <= 24.0 && available(WeaponId::Roller) {
        return WeaponId::Roller;
    }
    WeaponId::BasicShell
}

/// Returns only bounded, affordable catalogue candidates. The caller must apply them through the
/// session transaction, which retains authority over cash and ammunition.
pub fn planned_shop_purchases(
    difficulty: AiDifficulty,
    cash: u32,
    loadout: PlayerWeaponLoadout,
) -> Vec<WeaponId> {
    let (limit, priorities): (usize, &[WeaponId]) = match difficulty {
        AiDifficulty::Easy => (1, &[WeaponId::HighExplosive, WeaponId::Roller]),
        AiDifficulty::Normal => (
            2,
            &[
                WeaponId::HighExplosive,
                WeaponId::HeavyShell,
                WeaponId::Mirv,
                WeaponId::Roller,
            ],
        ),
        AiDifficulty::Hard => (
            3,
            &[
                WeaponId::HeavyShell,
                WeaponId::ClusterBomb,
                WeaponId::HighExplosive,
                WeaponId::Mirv,
                WeaponId::Roller,
            ],
        ),
    };
    let mut remaining_cash = cash;
    let mut planned = Vec::new();
    for weapon in priorities {
        if planned.len() == limit || loadout.availability(*weapon).is_available() {
            continue;
        }
        let price = weapon_price(*weapon).expect("AI priorities are purchasable limited weapons");
        if remaining_cash >= price {
            remaining_cash -= price;
            planned.push(*weapon);
        }
    }
    planned
}

fn horizontal_distance_squared(left: &Tank, right: &Tank) -> f32 {
    let dx = left.pose.position.x - right.pose.position.x;
    let dz = left.pose.position.z - right.pose.position.z;
    dx * dx + dz * dz
}

fn signed_error(seed: &mut u64, magnitude: f32) -> f32 {
    *seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
    (((*seed >> 40) as f32) / ((1_u32 << 24) as f32) * 2.0 - 1.0) * magnitude
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        battlefield::BattlefieldTerrain, tank::initial_tanks_for_players, world::WorldVector,
    };

    fn calm() -> Wind {
        Wind::new(WorldVector::ZERO).unwrap()
    }
    fn windy() -> Wind {
        Wind::new(WorldVector {
            x: 1.5,
            y: 0.0,
            z: 0.0,
        })
        .unwrap()
    }
    fn tanks() -> Vec<Tank> {
        initial_tanks_for_players(
            &BattlefieldTerrain::initial(),
            &[PlayerId(1), PlayerId(2), PlayerId(3)],
        )
    }
    fn with_round(mut loadout: PlayerWeaponLoadout, weapon: WeaponId) -> PlayerWeaponLoadout {
        assert!(loadout.add_round(weapon));
        loadout
    }

    #[test]
    fn decisions_are_seeded_legal_and_fall_back_when_depleted() {
        let tanks = tanks();
        let mut first = 44;
        let mut second = 44;
        let decision = decide_firing(
            &mut first,
            PlayerId(1),
            AiDifficulty::Normal,
            &tanks,
            PlayerWeaponLoadout::default(),
            calm(),
        )
        .unwrap();
        let repeated = decide_firing(
            &mut second,
            PlayerId(1),
            AiDifficulty::Normal,
            &tanks,
            PlayerWeaponLoadout::default(),
            calm(),
        )
        .unwrap();
        assert_eq!(decision, repeated);
        assert_eq!(decision.weapon, WeaponId::BasicShell);
        assert_eq!(decision.target, PlayerId(3));
    }

    #[test]
    fn normal_ai_uses_group_and_wind_roles_from_owned_ammunition() {
        let mut grouped = tanks();
        grouped[1].pose.position.x = 25.0;
        grouped[2].pose.position.x = 28.0;
        let cluster = with_round(PlayerWeaponLoadout::default(), WeaponId::ClusterBomb);
        assert_eq!(
            decide_firing(
                &mut 1,
                PlayerId(1),
                AiDifficulty::Normal,
                &grouped,
                cluster,
                calm()
            )
            .unwrap()
            .weapon,
            WeaponId::ClusterBomb
        );
        let heavy = with_round(PlayerWeaponLoadout::default(), WeaponId::HeavyShell);
        assert_eq!(
            decide_firing(
                &mut 1,
                PlayerId(1),
                AiDifficulty::Normal,
                &tanks(),
                heavy,
                windy()
            )
            .unwrap()
            .weapon,
            WeaponId::HeavyShell
        );
    }

    #[test]
    fn hard_aim_error_is_more_consistent_than_easy() {
        let tanks = tanks();
        let mut easy_seed = 7;
        let mut hard_seed = 7;
        let easy = decide_firing(
            &mut easy_seed,
            PlayerId(1),
            AiDifficulty::Easy,
            &tanks,
            PlayerWeaponLoadout::default(),
            calm(),
        )
        .unwrap();
        let hard = decide_firing(
            &mut hard_seed,
            PlayerId(1),
            AiDifficulty::Hard,
            &tanks,
            PlayerWeaponLoadout::default(),
            calm(),
        )
        .unwrap();
        let target = tanks.iter().find(|tank| tank.owner == easy.target).unwrap();
        let shooter = tanks.iter().find(|tank| tank.owner == PlayerId(1)).unwrap();
        let base_azimuth = azimuth_from_horizontal_direction(
            target.pose.position.x - shooter.pose.position.x,
            target.pose.position.z - shooter.pose.position.z,
        )
        .unwrap();
        assert!(
            (hard.aim.azimuth_degrees - base_azimuth).abs()
                < (easy.aim.azimuth_degrees - base_azimuth).abs()
        );
    }

    #[test]
    fn shop_plan_is_bounded_affordable_and_excludes_basic_and_nuke() {
        assert!(
            planned_shop_purchases(AiDifficulty::Hard, 0, PlayerWeaponLoadout::default())
                .is_empty()
        );
        let plan =
            planned_shop_purchases(AiDifficulty::Hard, 2_000, PlayerWeaponLoadout::default());
        assert_eq!(plan.len(), 3);
        assert!(
            plan.iter()
                .all(|weapon| !matches!(weapon, WeaponId::BasicShell | WeaponId::Nuke))
        );
        assert_eq!(
            planned_shop_purchases(AiDifficulty::Easy, 300, PlayerWeaponLoadout::default()),
            vec![WeaponId::HighExplosive]
        );
    }
}
