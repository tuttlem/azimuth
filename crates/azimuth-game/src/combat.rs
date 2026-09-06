use crate::{tank::Tank, weapon::ImpactProfile, world::WorldPosition};

/// First-duel splash damage is deliberately readable: every point strictly inside the radius
/// receives a whole linear-falloff amount, while the radius edge is safe.
pub fn damage_at(centre: WorldPosition, tank_position: WorldPosition, impact: ImpactProfile) -> u8 {
    let distance = centre.distance_to(tank_position);
    if distance >= impact.damage_radius {
        0
    } else {
        (impact.maximum_damage as f32 * (1.0 - distance / impact.damage_radius)).ceil() as u8
    }
}

/// Calculate every blast result before changing health so one tank cannot affect another tank's
/// eligibility for this same explosion.
pub fn resolve_explosion(
    tanks: &mut [Tank; 2],
    centre: WorldPosition,
    impact: ImpactProfile,
) -> [u8; 2] {
    let damages = [
        damage_at(centre, tanks[0].pose.position, impact),
        damage_at(centre, tanks[1].pose.position, impact),
    ];
    tanks[0].apply_damage(damages[0]);
    tanks[1].apply_damage(damages[1]);
    damages
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battlefield::BattlefieldTerrain;
    use crate::tank::{MAX_HEALTH, initial_tanks};
    use crate::weapon::{WeaponId, weapon_definition};

    #[test]
    fn damage_uses_three_dimensional_linear_falloff() {
        let centre = WorldPosition {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let impact = weapon_definition(WeaponId::BasicShell).impact;
        assert_eq!(damage_at(centre, centre, impact), 40);
        assert_eq!(
            damage_at(
                centre,
                WorldPosition {
                    x: 0.0,
                    y: 0.0,
                    z: impact.damage_radius
                },
                impact,
            ),
            0
        );
        assert_eq!(
            damage_at(
                centre,
                WorldPosition {
                    x: 0.0,
                    y: 3.0,
                    z: 0.0
                },
                impact,
            ),
            20
        );
        assert_eq!(
            damage_at(
                centre,
                WorldPosition {
                    x: 0.0,
                    y: 0.0,
                    z: 5.9
                },
                impact,
            ),
            1
        );
    }

    #[test]
    fn same_explosion_applies_damage_to_both_tanks() {
        let terrain = BattlefieldTerrain::initial();
        let mut tanks = initial_tanks(&terrain);
        tanks[0].pose.position = WorldPosition {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        tanks[1].pose.position = WorldPosition {
            x: 3.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(
            resolve_explosion(
                &mut tanks,
                WorldPosition {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0
                },
                weapon_definition(WeaponId::BasicShell).impact,
            ),
            [40, 20]
        );
        assert_eq!(tanks[0].health, MAX_HEALTH - 40);
        assert_eq!(tanks[1].health, MAX_HEALTH - 20);
    }
}
