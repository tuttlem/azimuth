use crate::{
    battlefield::Crater,
    projectile::{Projectile, WindResponse},
    tank::PlayerId,
};

pub const HIGH_EXPLOSIVE_STARTING_ROUNDS: u8 = 2;
pub const HEAVY_SHELL_STARTING_ROUNDS: u8 = 2;

/// Stable gameplay identity. Display names and inventory storage order are never identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WeaponId {
    BasicShell,
    HighExplosive,
    HeavyShell,
    #[cfg(test)]
    TestConventional,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AmmunitionRule {
    Unlimited,
    Limited(u8),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProjectileProfile {
    pub wind_response: WindResponse,
}

/// Ordinary projectile differences are deliberately empty today: both first weapons use the
/// existing ballistic launch, gravity, and wind model. A real future conventional difference can
/// earn a field here without disturbing impact state.
pub const STANDARD_BALLISTIC_PROJECTILE: ProjectileProfile = ProjectileProfile {
    wind_response: WindResponse::NORMAL,
};

pub fn heavy_shell_projectile() -> ProjectileProfile {
    ProjectileProfile {
        wind_response: WindResponse::new(0.4).expect("heavy shell wind response must be valid"),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImpactProfile {
    pub damage_radius: f32,
    pub maximum_damage: u8,
    pub crater: Crater,
    pub explosion_visual_scale: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeaponDefinition {
    pub id: WeaponId,
    pub display_name: &'static str,
    pub ammunition: AmmunitionRule,
    pub projectile: ProjectileProfile,
    pub impact: ImpactProfile,
}

impl WeaponDefinition {
    pub fn new(
        id: WeaponId,
        display_name: &'static str,
        ammunition: AmmunitionRule,
        projectile: ProjectileProfile,
        impact: ImpactProfile,
    ) -> Self {
        Self {
            id,
            display_name,
            ammunition,
            projectile,
            impact,
        }
    }
}

/// The one discoverable source for normal Azimuth weapon values. Do not generalise behaviour
/// until a real weapon requires it; conventional explosives are definition data.
pub fn weapon_definition(id: WeaponId) -> WeaponDefinition {
    match id {
        WeaponId::BasicShell => WeaponDefinition::new(
            WeaponId::BasicShell,
            "BASIC SHELL",
            AmmunitionRule::Unlimited,
            STANDARD_BALLISTIC_PROJECTILE,
            ImpactProfile {
                damage_radius: 6.0,
                maximum_damage: 40,
                crater: Crater::new(4.0, 1.8).expect("basic shell crater must be valid"),
                explosion_visual_scale: 1.0,
            },
        ),
        WeaponId::HighExplosive => WeaponDefinition::new(
            WeaponId::HighExplosive,
            "HIGH EXPLOSIVE",
            AmmunitionRule::Limited(HIGH_EXPLOSIVE_STARTING_ROUNDS),
            STANDARD_BALLISTIC_PROJECTILE,
            ImpactProfile {
                damage_radius: 8.0,
                maximum_damage: 60,
                crater: Crater::new(6.0, 3.0).expect("high explosive crater must be valid"),
                explosion_visual_scale: 1.5,
            },
        ),
        WeaponId::HeavyShell => WeaponDefinition::new(
            WeaponId::HeavyShell,
            "HEAVY SHELL",
            AmmunitionRule::Limited(HEAVY_SHELL_STARTING_ROUNDS),
            heavy_shell_projectile(),
            ImpactProfile {
                damage_radius: 6.0,
                maximum_damage: 40,
                crater: Crater::new(4.0, 1.8).expect("heavy shell crater must be valid"),
                explosion_visual_scale: 1.0,
            },
        ),
        #[cfg(test)]
        WeaponId::TestConventional => test_conventional_definition(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeaponAvailability {
    Unlimited,
    Remaining(u8),
}

impl WeaponAvailability {
    fn from_rule(rule: AmmunitionRule) -> Self {
        match rule {
            AmmunitionRule::Unlimited => Self::Unlimited,
            AmmunitionRule::Limited(rounds) => Self::Remaining(rounds),
        }
    }

    pub fn is_available(self) -> bool {
        match self {
            Self::Unlimited => true,
            Self::Remaining(rounds) => rounds > 0,
        }
    }

    fn consume(&mut self) -> bool {
        match self {
            Self::Unlimited => true,
            Self::Remaining(rounds) if *rounds > 0 => {
                *rounds -= 1;
                true
            }
            Self::Remaining(_) => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlayerWeaponLoadout {
    selected: WeaponId,
    basic_shell: WeaponAvailability,
    high_explosive: WeaponAvailability,
    heavy_shell: WeaponAvailability,
}

impl Default for PlayerWeaponLoadout {
    fn default() -> Self {
        Self {
            selected: WeaponId::BasicShell,
            basic_shell: WeaponAvailability::from_rule(
                weapon_definition(WeaponId::BasicShell).ammunition,
            ),
            high_explosive: WeaponAvailability::from_rule(
                weapon_definition(WeaponId::HighExplosive).ammunition,
            ),
            heavy_shell: WeaponAvailability::from_rule(
                weapon_definition(WeaponId::HeavyShell).ammunition,
            ),
        }
    }
}

impl PlayerWeaponLoadout {
    pub fn selected(self) -> WeaponId {
        self.selected
    }

    pub fn availability(self, weapon: WeaponId) -> WeaponAvailability {
        match weapon {
            WeaponId::BasicShell => self.basic_shell,
            WeaponId::HighExplosive => self.high_explosive,
            WeaponId::HeavyShell => self.heavy_shell,
            #[cfg(test)]
            WeaponId::TestConventional => WeaponAvailability::Unlimited,
        }
    }

    pub fn select(&mut self, weapon: WeaponId) -> bool {
        if !self.availability(weapon).is_available() {
            return false;
        }
        self.selected = weapon;
        true
    }

    /// Captures the selected definition and consumes limited availability exactly once. The
    /// fallback happens only after a successful final-round commitment.
    pub fn commit_selected(&mut self) -> Option<WeaponDefinition> {
        let selected = self.selected;
        let availability = match selected {
            WeaponId::BasicShell => &mut self.basic_shell,
            WeaponId::HighExplosive => &mut self.high_explosive,
            WeaponId::HeavyShell => &mut self.heavy_shell,
            #[cfg(test)]
            WeaponId::TestConventional => return None,
        };
        if !availability.consume() {
            return None;
        }
        if !availability.is_available() {
            self.selected = WeaponId::BasicShell;
        }
        Some(weapon_definition(selected))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PlayerWeaponLoadouts(pub [PlayerWeaponLoadout; 2]);

impl PlayerWeaponLoadouts {
    pub fn for_player(&self, player: PlayerId) -> PlayerWeaponLoadout {
        self.0[player_index(player)]
    }

    pub fn for_player_mut(&mut self, player: PlayerId) -> &mut PlayerWeaponLoadout {
        &mut self.0[player_index(player)]
    }
}

fn player_index(player: PlayerId) -> usize {
    match player {
        PlayerId::One => 0,
        PlayerId::Two => 1,
    }
}

/// Immutable ordinary-shot authority. Flight advances only `projectile`; impact uses the copied
/// profile and never looks back at mutable selection or inventory.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FiredShot {
    pub weapon: WeaponId,
    pub projectile: Projectile,
    pub impact: ImpactProfile,
}

impl FiredShot {
    pub fn new(definition: WeaponDefinition, projectile: Projectile) -> Self {
        Self {
            weapon: definition.id,
            projectile,
            impact: definition.impact,
        }
    }
}

#[cfg(test)]
pub fn test_conventional_definition() -> WeaponDefinition {
    WeaponDefinition::new(
        WeaponId::TestConventional,
        "TEST CONVENTIONAL",
        AmmunitionRule::Limited(1),
        ProjectileProfile {
            wind_response: WindResponse::new(0.7)
                .expect("test conventional wind response must be valid"),
        },
        ImpactProfile {
            damage_radius: 5.0,
            maximum_damage: 25,
            crater: Crater::new(3.0, 1.0).expect("test crater must be valid"),
            explosion_visual_scale: 0.8,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::WorldPosition;

    #[test]
    fn catalogue_defines_distinct_baseline_and_high_explosive_profiles() {
        let basic = weapon_definition(WeaponId::BasicShell);
        let he = weapon_definition(WeaponId::HighExplosive);
        assert_ne!(basic.id, he.id);
        assert_eq!(basic.display_name, "BASIC SHELL");
        assert_eq!(basic.ammunition, AmmunitionRule::Unlimited);
        assert_eq!(basic.impact.damage_radius, 6.0);
        assert_eq!(basic.impact.maximum_damage, 40);
        assert!(he.impact.damage_radius > basic.impact.damage_radius);
        assert!(he.impact.maximum_damage > basic.impact.maximum_damage);
        assert_eq!(he.ammunition, AmmunitionRule::Limited(2));
    }

    #[test]
    fn player_loadouts_are_independent_and_final_he_falls_back_to_basic() {
        let mut loadouts = PlayerWeaponLoadouts::default();
        assert!(
            loadouts
                .for_player_mut(PlayerId::One)
                .select(WeaponId::HighExplosive)
        );
        assert_eq!(
            loadouts.for_player(PlayerId::Two).selected(),
            WeaponId::BasicShell
        );
        let player_one = loadouts.for_player_mut(PlayerId::One);
        assert_eq!(
            player_one.commit_selected().unwrap().id,
            WeaponId::HighExplosive
        );
        assert_eq!(
            player_one.commit_selected().unwrap().id,
            WeaponId::HighExplosive
        );
        assert_eq!(player_one.selected(), WeaponId::BasicShell);
        assert!(!player_one.select(WeaponId::HighExplosive));
        assert_eq!(
            loadouts
                .for_player(PlayerId::Two)
                .availability(WeaponId::HighExplosive),
            WeaponAvailability::Remaining(2)
        );
    }

    #[test]
    fn fired_shot_copies_the_profile_not_the_later_loadout() {
        let mut loadout = PlayerWeaponLoadout::default();
        loadout.select(WeaponId::HeavyShell);
        let definition = loadout.commit_selected().unwrap();
        let projectile = Projectile::launch_with_wind_response(
            crate::projectile::ShotParameters::new(
                WorldPosition {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                0.0,
                45.0,
                10.0,
            )
            .unwrap(),
            definition.projectile.wind_response,
        );
        let shot = FiredShot::new(definition, projectile);
        loadout.select(WeaponId::BasicShell);
        assert_eq!(shot.weapon, WeaponId::HeavyShell);
        assert_eq!(shot.impact.damage_radius, 6.0);
        assert_eq!(shot.projectile.wind_response().factor(), 0.4);
    }

    #[test]
    fn a_third_conventional_definition_uses_the_same_shot_boundary() {
        let definition = test_conventional_definition();
        let projectile = Projectile::launch(
            crate::projectile::ShotParameters::new(
                WorldPosition {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                0.0,
                45.0,
                10.0,
            )
            .unwrap(),
        );

        let shot = FiredShot::new(definition, projectile);
        assert_eq!(shot.weapon, WeaponId::TestConventional);
        assert_eq!(shot.impact, definition.impact);
        assert_eq!(definition.projectile.wind_response.factor(), 0.7);
    }

    #[test]
    fn heavy_shell_is_limited_wind_resistant_and_independent_per_player() {
        let basic = weapon_definition(WeaponId::BasicShell);
        let heavy = weapon_definition(WeaponId::HeavyShell);
        assert_eq!(heavy.display_name, "HEAVY SHELL");
        assert_eq!(heavy.ammunition, AmmunitionRule::Limited(2));
        assert_eq!(heavy.projectile.wind_response.factor(), 0.4);
        assert_eq!(basic.projectile.wind_response, WindResponse::NORMAL);
        assert_eq!(heavy.impact, basic.impact);

        let projectile = Projectile::launch_with_wind_response(
            crate::projectile::ShotParameters::new(
                WorldPosition {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                0.0,
                45.0,
                10.0,
            )
            .unwrap(),
            heavy.projectile.wind_response,
        );
        assert_eq!(projectile.wind_response(), heavy.projectile.wind_response);

        let mut loadouts = PlayerWeaponLoadouts::default();
        let player_one = loadouts.for_player_mut(PlayerId::One);
        assert_eq!(
            player_one.availability(WeaponId::HeavyShell),
            WeaponAvailability::Remaining(2)
        );
        assert!(player_one.select(WeaponId::HeavyShell));
        assert_eq!(
            player_one.commit_selected().unwrap().id,
            WeaponId::HeavyShell
        );
        assert_eq!(
            player_one.availability(WeaponId::HeavyShell),
            WeaponAvailability::Remaining(1)
        );
        assert_eq!(
            loadouts
                .for_player(PlayerId::Two)
                .availability(WeaponId::HeavyShell),
            WeaponAvailability::Remaining(2)
        );
    }
}
