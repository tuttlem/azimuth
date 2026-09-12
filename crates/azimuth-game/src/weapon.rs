use crate::{
    battlefield::{Crater, Mound},
    projectile::{Projectile, WindResponse},
    tank::PlayerId,
};

pub const HIGH_EXPLOSIVE_STARTING_ROUNDS: u8 = 2;
pub const HEAVY_SHELL_STARTING_ROUNDS: u8 = 2;
pub const MIRV_STARTING_ROUNDS: u8 = 2;
pub const CLUSTER_BOMB_STARTING_ROUNDS: u8 = 2;
pub const ROLLER_STARTING_ROUNDS: u8 = 2;
pub const BUNKER_BUSTER_STARTING_ROUNDS: u8 = 2;
pub const DIRT_BOMB_STARTING_ROUNDS: u8 = 2;
pub const CURVE_BALL_STARTING_ROUNDS: u8 = 2;
pub const BOUNCER_STARTING_ROUNDS: u8 = 2;
pub const NUKE_STARTING_ROUNDS: u8 = 1;
pub const MIRV_CHILD_COUNT: usize = 5;
pub const CLUSTER_BOMB_CHILD_COUNT: usize = 10;
/// Cluster Bomb is the largest concrete barrage currently in the game.
pub const MAX_SHOT_CHILDREN: usize = CLUSTER_BOMB_CHILD_COUNT;

/// Stable gameplay identity. Display names and inventory storage order are never identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WeaponId {
    BasicShell,
    HighExplosive,
    HeavyShell,
    Mirv,
    ClusterBomb,
    Roller,
    BunkerBuster,
    DirtBomb,
    CurveBall,
    Bouncer,
    Nuke,
    #[cfg(test)]
    TestConventional,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WeaponPresentation {
    pub compact_name: &'static str,
    pub glyph: &'static str,
}

/// Presentation is intentionally a small, engine-independent lookup shared by the shop and HUD.
pub const fn weapon_presentation(id: WeaponId) -> WeaponPresentation {
    match id {
        WeaponId::BasicShell => WeaponPresentation {
            compact_name: "BASIC",
            glyph: "●",
        },
        WeaponId::HighExplosive => WeaponPresentation {
            compact_name: "HE",
            glyph: "✹",
        },
        WeaponId::HeavyShell => WeaponPresentation {
            compact_name: "HEAVY",
            glyph: "⬤",
        },
        WeaponId::Mirv => WeaponPresentation {
            compact_name: "MIRV",
            glyph: "●↘",
        },
        WeaponId::ClusterBomb => WeaponPresentation {
            compact_name: "CLSTR",
            glyph: "●··",
        },
        WeaponId::Roller => WeaponPresentation {
            compact_name: "ROLL",
            glyph: "◉",
        },
        WeaponId::BunkerBuster => WeaponPresentation {
            compact_name: "BUNK",
            glyph: "│▼",
        },
        WeaponId::DirtBomb => WeaponPresentation {
            compact_name: "DIRT",
            glyph: "●⌂",
        },
        WeaponId::CurveBall => WeaponPresentation {
            compact_name: "CURVE",
            glyph: "⌒●",
        },
        WeaponId::Bouncer => WeaponPresentation {
            compact_name: "BOUNCE",
            glyph: "╱●╲",
        },
        WeaponId::Nuke => WeaponPresentation {
            compact_name: "NUKE",
            glyph: "☢",
        },
        #[cfg(test)]
        WeaponId::TestConventional => WeaponPresentation {
            compact_name: "TEST",
            glyph: "?",
        },
    }
}

pub const ACTIVE_WEAPONS: [WeaponId; 11] = [
    WeaponId::BasicShell,
    WeaponId::HighExplosive,
    WeaponId::HeavyShell,
    WeaponId::Mirv,
    WeaponId::ClusterBomb,
    WeaponId::Roller,
    WeaponId::BunkerBuster,
    WeaponId::DirtBomb,
    WeaponId::CurveBall,
    WeaponId::Bouncer,
    WeaponId::Nuke,
];

pub const SHOP_WEAPONS: [WeaponId; 10] = [
    WeaponId::HighExplosive,
    WeaponId::HeavyShell,
    WeaponId::Mirv,
    WeaponId::ClusterBomb,
    WeaponId::Roller,
    WeaponId::BunkerBuster,
    WeaponId::DirtBomb,
    WeaponId::CurveBall,
    WeaponId::Bouncer,
    WeaponId::Nuke,
];

/// Prices deliberately live beside the weapon catalogue: the shop is ammunition-only, and the
/// unlimited Basic Shell must never become a finite purchase.
pub const fn weapon_price(id: WeaponId) -> Option<u32> {
    match id {
        WeaponId::BasicShell => None,
        WeaponId::HighExplosive => Some(300),
        WeaponId::HeavyShell => Some(250),
        WeaponId::Mirv => Some(600),
        WeaponId::ClusterBomb => Some(550),
        WeaponId::Roller => Some(400),
        WeaponId::BunkerBuster => Some(700),
        WeaponId::DirtBomb => Some(350),
        WeaponId::CurveBall => Some(400),
        WeaponId::Bouncer => Some(350),
        WeaponId::Nuke => Some(1500),
        #[cfg(test)]
        WeaponId::TestConventional => None,
    }
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
    pub mound: Option<Mound>,
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
                mound: None,
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
                mound: None,
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
                mound: None,
                explosion_visual_scale: 1.0,
            },
        ),
        WeaponId::Mirv => WeaponDefinition::new(
            WeaponId::Mirv,
            "MIRV",
            AmmunitionRule::Limited(MIRV_STARTING_ROUNDS),
            STANDARD_BALLISTIC_PROJECTILE,
            ImpactProfile {
                damage_radius: 3.5,
                maximum_damage: 22,
                crater: Crater::new(2.4, 0.9).expect("MIRV child crater must be valid"),
                mound: None,
                explosion_visual_scale: 0.65,
            },
        ),
        WeaponId::ClusterBomb => WeaponDefinition::new(
            WeaponId::ClusterBomb,
            "CLUSTER BOMB",
            AmmunitionRule::Limited(CLUSTER_BOMB_STARTING_ROUNDS),
            STANDARD_BALLISTIC_PROJECTILE,
            ImpactProfile {
                damage_radius: 2.6,
                maximum_damage: 16,
                crater: Crater::new(1.8, 0.65).expect("cluster crater"),
                mound: None,
                explosion_visual_scale: 0.45,
            },
        ),
        WeaponId::Roller => WeaponDefinition::new(
            WeaponId::Roller,
            "ROLLER",
            AmmunitionRule::Limited(ROLLER_STARTING_ROUNDS),
            STANDARD_BALLISTIC_PROJECTILE,
            ImpactProfile {
                damage_radius: 5.5,
                maximum_damage: 38,
                crater: Crater::new(3.8, 1.55).expect("roller crater"),
                mound: None,
                explosion_visual_scale: 0.9,
            },
        ),
        WeaponId::BunkerBuster => WeaponDefinition::new(
            WeaponId::BunkerBuster,
            "BUNKER BUSTER",
            AmmunitionRule::Limited(BUNKER_BUSTER_STARTING_ROUNDS),
            STANDARD_BALLISTIC_PROJECTILE,
            ImpactProfile {
                // This weapon tunnels before resolving. Its intentionally zero damage profile
                // makes the resulting deep excavation positional terrain play, never a blast.
                damage_radius: 0.0,
                maximum_damage: 0,
                crater: Crater::new(8.0, 12.0).expect("bunker crater"),
                mound: None,
                explosion_visual_scale: 1.15,
            },
        ),
        WeaponId::DirtBomb => WeaponDefinition::new(
            WeaponId::DirtBomb,
            "DIRT BOMB",
            AmmunitionRule::Limited(DIRT_BOMB_STARTING_ROUNDS),
            STANDARD_BALLISTIC_PROJECTILE,
            ImpactProfile {
                damage_radius: 0.0,
                maximum_damage: 0,
                crater: Crater::new(1.0, 0.01).unwrap(),
                mound: Some(Mound::new(8.0, 5.0).unwrap()),
                explosion_visual_scale: 0.45,
            },
        ),
        WeaponId::CurveBall => WeaponDefinition::new(
            WeaponId::CurveBall,
            "CURVE BALL",
            AmmunitionRule::Limited(CURVE_BALL_STARTING_ROUNDS),
            STANDARD_BALLISTIC_PROJECTILE,
            ImpactProfile {
                damage_radius: 5.5,
                maximum_damage: 38,
                crater: Crater::new(3.8, 1.55).unwrap(),
                mound: None,
                explosion_visual_scale: 0.9,
            },
        ),
        WeaponId::Bouncer => WeaponDefinition::new(
            WeaponId::Bouncer,
            "BOUNCER",
            AmmunitionRule::Limited(BOUNCER_STARTING_ROUNDS),
            STANDARD_BALLISTIC_PROJECTILE,
            ImpactProfile {
                damage_radius: 5.5,
                maximum_damage: 38,
                crater: Crater::new(3.8, 1.55).unwrap(),
                mound: None,
                explosion_visual_scale: 0.9,
            },
        ),
        WeaponId::Nuke => WeaponDefinition::new(
            WeaponId::Nuke,
            "NUKE",
            AmmunitionRule::Limited(NUKE_STARTING_ROUNDS),
            STANDARD_BALLISTIC_PROJECTILE,
            ImpactProfile {
                damage_radius: 28.0,
                maximum_damage: 100,
                crater: Crater::new(18.0, 9.0).unwrap(),
                mound: None,
                explosion_visual_scale: 4.5,
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
    mirv: WeaponAvailability,
    cluster_bomb: WeaponAvailability,
    roller: WeaponAvailability,
    bunker_buster: WeaponAvailability,
    dirt_bomb: WeaponAvailability,
    curve_ball: WeaponAvailability,
    bouncer: WeaponAvailability,
    nuke: WeaponAvailability,
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
            mirv: WeaponAvailability::from_rule(weapon_definition(WeaponId::Mirv).ammunition),
            cluster_bomb: WeaponAvailability::from_rule(
                weapon_definition(WeaponId::ClusterBomb).ammunition,
            ),
            roller: WeaponAvailability::from_rule(weapon_definition(WeaponId::Roller).ammunition),
            bunker_buster: WeaponAvailability::from_rule(
                weapon_definition(WeaponId::BunkerBuster).ammunition,
            ),
            dirt_bomb: WeaponAvailability::from_rule(
                weapon_definition(WeaponId::DirtBomb).ammunition,
            ),
            curve_ball: WeaponAvailability::from_rule(
                weapon_definition(WeaponId::CurveBall).ammunition,
            ),
            bouncer: WeaponAvailability::from_rule(weapon_definition(WeaponId::Bouncer).ammunition),
            nuke: WeaponAvailability::from_rule(weapon_definition(WeaponId::Nuke).ammunition),
        }
    }
}

impl PlayerWeaponLoadout {
    /// A new round keeps purchased ammunition but starts with the universal shell selected.
    pub fn reset_selection(&mut self) {
        self.selected = WeaponId::BasicShell;
    }

    pub fn add_round(&mut self, weapon: WeaponId) -> bool {
        let availability = match weapon {
            WeaponId::BasicShell => return false,
            WeaponId::HighExplosive => &mut self.high_explosive,
            WeaponId::HeavyShell => &mut self.heavy_shell,
            WeaponId::Mirv => &mut self.mirv,
            WeaponId::ClusterBomb => &mut self.cluster_bomb,
            WeaponId::Roller => &mut self.roller,
            WeaponId::BunkerBuster => &mut self.bunker_buster,
            WeaponId::DirtBomb => &mut self.dirt_bomb,
            WeaponId::CurveBall => &mut self.curve_ball,
            WeaponId::Bouncer => &mut self.bouncer,
            WeaponId::Nuke => &mut self.nuke,
            #[cfg(test)]
            WeaponId::TestConventional => return false,
        };
        if let WeaponAvailability::Remaining(rounds) = availability {
            *rounds = rounds.saturating_add(1);
            true
        } else {
            false
        }
    }
    pub fn selected(self) -> WeaponId {
        self.selected
    }

    pub fn availability(self, weapon: WeaponId) -> WeaponAvailability {
        match weapon {
            WeaponId::BasicShell => self.basic_shell,
            WeaponId::HighExplosive => self.high_explosive,
            WeaponId::HeavyShell => self.heavy_shell,
            WeaponId::Mirv => self.mirv,
            WeaponId::ClusterBomb => self.cluster_bomb,
            WeaponId::Roller => self.roller,
            WeaponId::BunkerBuster => self.bunker_buster,
            WeaponId::DirtBomb => self.dirt_bomb,
            WeaponId::CurveBall => self.curve_ball,
            WeaponId::Bouncer => self.bouncer,
            WeaponId::Nuke => self.nuke,
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
            WeaponId::Mirv => &mut self.mirv,
            WeaponId::ClusterBomb => &mut self.cluster_bomb,
            WeaponId::Roller => &mut self.roller,
            WeaponId::BunkerBuster => &mut self.bunker_buster,
            WeaponId::DirtBomb => &mut self.dirt_bomb,
            WeaponId::CurveBall => &mut self.curve_ball,
            WeaponId::Bouncer => &mut self.bouncer,
            WeaponId::Nuke => &mut self.nuke,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlayerWeaponLoadouts(pub Vec<(PlayerId, PlayerWeaponLoadout)>);

impl Default for PlayerWeaponLoadouts {
    fn default() -> Self {
        Self::new(&[PlayerId::One, PlayerId::Two])
    }
}

impl PlayerWeaponLoadouts {
    pub fn new(players: &[PlayerId]) -> Self {
        Self(
            players
                .iter()
                .copied()
                .map(|player| (player, PlayerWeaponLoadout::default()))
                .collect(),
        )
    }

    pub fn for_player(&self, player: PlayerId) -> PlayerWeaponLoadout {
        self.0
            .iter()
            .find(|(owner, _)| *owner == player)
            .map(|(_, loadout)| *loadout)
            .expect("every participant must have a loadout")
    }

    pub fn for_player_mut(&mut self, player: PlayerId) -> &mut PlayerWeaponLoadout {
        self.0
            .iter_mut()
            .find(|(owner, _)| *owner == player)
            .map(|(_, loadout)| loadout)
            .expect("every participant must have a loadout")
    }
}

/// Bounded post-contact state earned by Roller and Bunker Buster. This is intentionally not a
/// general projectile state-machine: all other weapons remain ordinary ballistic projectiles.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ContactState {
    Airborne,
    Rolling {
        remaining_steps: u16,
    },
    Penetrating {
        remaining_steps: u8,
        direction: crate::world::WorldVector,
    },
    Bouncing {
        remaining_bounces: u8,
    },
}

/// A committed shot owns all of its active projectiles. The fixed array is sized by Cluster Bomb,
/// rather than being an open-ended projectile behaviour framework.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FiredShot {
    pub weapon: WeaponId,
    pub projectile: Projectile,
    pub impact: ImpactProfile,
    pub children: [Option<Projectile>; MAX_SHOT_CHILDREN],
    pub split: bool,
    pub contact: ContactState,
}

impl FiredShot {
    pub fn new(definition: WeaponDefinition, projectile: Projectile) -> Self {
        Self {
            weapon: definition.id,
            projectile,
            impact: definition.impact,
            children: [None; MAX_SHOT_CHILDREN],
            split: false,
            contact: ContactState::Airborne,
        }
    }

    pub fn is_deployment_carrier(self) -> bool {
        matches!(self.weapon, WeaponId::Mirv | WeaponId::ClusterBomb) && !self.split
    }
    pub fn has_active_projectiles(self) -> bool {
        !self.split || self.children.iter().any(Option::is_some)
    }

    pub fn presentation_projectile(self) -> Option<Projectile> {
        if self.split {
            self.children.iter().find_map(|child| *child)
        } else {
            Some(self.projectile)
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
            mound: None,
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
    fn mirv_is_limited_and_each_child_profile_is_smaller_than_he() {
        let mirv = weapon_definition(WeaponId::Mirv);
        let he = weapon_definition(WeaponId::HighExplosive);
        assert_eq!(
            mirv.ammunition,
            AmmunitionRule::Limited(MIRV_STARTING_ROUNDS)
        );
        assert!(mirv.impact.maximum_damage < he.impact.maximum_damage);
        assert!(mirv.impact.damage_radius < he.impact.damage_radius);
        let mut loadout = PlayerWeaponLoadout::default();
        assert!(loadout.select(WeaponId::Mirv));
        assert_eq!(loadout.commit_selected().unwrap().id, WeaponId::Mirv);
        assert_eq!(
            loadout.availability(WeaponId::Mirv),
            WeaponAvailability::Remaining(1)
        );
    }

    #[test]
    fn arsenal_pack_inventory_is_limited_and_profiles_preserve_distinct_roles() {
        let cluster = weapon_definition(WeaponId::ClusterBomb);
        let roller = weapon_definition(WeaponId::Roller);
        let bunker = weapon_definition(WeaponId::BunkerBuster);
        assert_eq!(cluster.ammunition, AmmunitionRule::Limited(2));
        assert_eq!(roller.ammunition, AmmunitionRule::Limited(2));
        assert_eq!(bunker.ammunition, AmmunitionRule::Limited(2));
        assert!(
            cluster.impact.maximum_damage
                < weapon_definition(WeaponId::HighExplosive)
                    .impact
                    .maximum_damage
        );
        assert_eq!(bunker.impact.maximum_damage, 0);
        assert_eq!(bunker.impact.damage_radius, 0.0);
        assert!(weapon_price(WeaponId::BunkerBuster) > weapon_price(WeaponId::HighExplosive));
    }

    #[test]
    fn fired_shot_has_a_bounded_capacity_for_cluster_bomb() {
        let projectile = Projectile::launch(
            crate::projectile::ShotParameters::new(
                WorldPosition {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                0.0,
                45.0,
                10.0,
            )
            .unwrap(),
        );
        let shot = FiredShot::new(weapon_definition(WeaponId::ClusterBomb), projectile);
        assert_eq!(shot.children.len(), CLUSTER_BOMB_CHILD_COUNT);
        assert_eq!(shot.contact, ContactState::Airborne);
        assert!(shot.is_deployment_carrier());
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

    #[test]
    fn arsenal_pack_two_profiles_are_limited_and_distinct() {
        let dirt = weapon_definition(WeaponId::DirtBomb);
        let curve = weapon_definition(WeaponId::CurveBall);
        let bounce = weapon_definition(WeaponId::Bouncer);
        let nuke = weapon_definition(WeaponId::Nuke);
        assert_eq!(dirt.ammunition, AmmunitionRule::Limited(2));
        assert_eq!(nuke.ammunition, AmmunitionRule::Limited(1));
        assert!(dirt.impact.mound.is_some() && dirt.impact.maximum_damage == 0);
        assert_eq!(curve.ammunition, AmmunitionRule::Limited(2));
        assert_eq!(bounce.ammunition, AmmunitionRule::Limited(2));
        assert!(
            nuke.impact.damage_radius
                > weapon_definition(WeaponId::HighExplosive)
                    .impact
                    .damage_radius
        );
        let mut loadout = PlayerWeaponLoadout::default();
        assert!(loadout.select(WeaponId::Nuke));
        assert_eq!(loadout.commit_selected().unwrap().id, WeaponId::Nuke);
        assert!(!loadout.select(WeaponId::Nuke));
    }
}
