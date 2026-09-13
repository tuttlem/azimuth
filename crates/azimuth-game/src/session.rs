use crate::{
    match_setup::{ControllerType, PlayerConfiguration},
    tank::PlayerId,
    weapon::{PlayerWeaponLoadout, SHOP_WEAPONS, WeaponId, weapon_price},
};
use bevy::prelude::Resource;

pub const CASH_PER_DAMAGE: u32 = 10;
pub const FIRST_PLACE_AWARD: u32 = 5_000;
pub const SECOND_PLACE_AWARD: u32 = 2_500;
pub const THIRD_PLACE_AWARD: u32 = 1_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionPhase {
    Setup,
    Playing,
    Celebrating,
    Accounting,
    Shopping,
    Transition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShopCategory {
    Weapons,
    Armour,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShopItem {
    pub category: ShopCategory,
    pub weapon: WeaponId,
    pub price: u32,
}

pub fn weapon_shop_items() -> [ShopItem; 10] {
    SHOP_WEAPONS.map(|weapon| ShopItem {
        category: ShopCategory::Weapons,
        weapon,
        price: weapon_price(weapon).expect("limited shop weapon has price"),
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionPlayer {
    pub configuration: PlayerConfiguration,
    pub cash: u32,
    pub loadout: PlayerWeaponLoadout,
    pub wins: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RoundEarnings {
    pub damage_income: u32,
    pub placement_income: u32,
}
impl RoundEarnings {
    pub fn total(self) -> u32 {
        self.damage_income + self.placement_income
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Resource)]
pub struct GameSession {
    pub players: Vec<SessionPlayer>,
    pub round_number: u32,
    pub phase: SessionPhase,
    pub earnings: Vec<(PlayerId, RoundEarnings)>,
    pub shop_index: usize,
}
impl GameSession {
    pub fn new(players: Vec<PlayerConfiguration>) -> Self {
        let earnings = players
            .iter()
            .map(|p| (p.id, RoundEarnings::default()))
            .collect();
        Self {
            players: players
                .into_iter()
                .map(|configuration| SessionPlayer {
                    configuration,
                    cash: 0,
                    loadout: PlayerWeaponLoadout::default(),
                    wins: 0,
                })
                .collect(),
            round_number: 1,
            phase: SessionPhase::Playing,
            earnings,
            shop_index: 0,
        }
    }
    pub fn player(&self, id: PlayerId) -> &SessionPlayer {
        self.players
            .iter()
            .find(|p| p.configuration.id == id)
            .expect("configured player")
    }
    pub fn player_mut(&mut self, id: PlayerId) -> &mut SessionPlayer {
        self.players
            .iter_mut()
            .find(|p| p.configuration.id == id)
            .expect("configured player")
    }
    fn earnings_mut(&mut self, id: PlayerId) -> &mut RoundEarnings {
        &mut self
            .earnings
            .iter_mut()
            .find(|(owner, _)| *owner == id)
            .expect("configured player")
            .1
    }
    pub fn credit_damage(&mut self, owner: PlayerId, target: PlayerId, actual_health_removed: u8) {
        if owner == target {
            return;
        }
        let damage = actual_health_removed as u32 * CASH_PER_DAMAGE;
        let earned = self.earnings_mut(owner);
        earned.damage_income += damage;
        self.player_mut(owner).cash += damage;
    }
    /// Placement is supplied by the authoritative turn resolver. An empty order represents a
    /// draw, which deliberately pays no invented first-place award.
    pub fn finalise(&mut self, placement_order: &[PlayerId]) {
        if self.phase == SessionPhase::Celebrating {
            return;
        }
        for (place, player) in placement_order.iter().copied().enumerate() {
            let award = match place {
                0 => FIRST_PLACE_AWARD,
                1 => SECOND_PLACE_AWARD,
                2 => THIRD_PLACE_AWARD,
                _ => 0,
            };
            self.earnings_mut(player).placement_income += award;
            self.player_mut(player).cash += award;
        }
        self.phase = SessionPhase::Celebrating;
    }
    pub fn purchase(&mut self, player: PlayerId, weapon: WeaponId) -> bool {
        if self.phase != SessionPhase::Shopping || self.active_shop_player() != Some(player) {
            return false;
        }
        let Some(price) = weapon_price(weapon) else {
            return false;
        };
        let purchaser = self.player_mut(player);
        if purchaser.cash < price {
            return false;
        }
        if purchaser.loadout.add_round(weapon) {
            purchaser.cash -= price;
            true
        } else {
            false
        }
    }
    pub fn begin_shop(&mut self) {
        self.phase = SessionPhase::Shopping;
        self.shop_index = 0;
    }
    pub fn active_shop_player(&self) -> Option<PlayerId> {
        self.players
            .get(self.shop_index)
            .map(|player| player.configuration.id)
    }
    pub fn active_shop_controller(&self) -> Option<ControllerType> {
        self.players
            .get(self.shop_index)
            .map(|player| player.configuration.controller)
    }
    pub fn complete_active_shopper(&mut self) -> bool {
        if self.phase != SessionPhase::Shopping || self.shop_index >= self.players.len() {
            return false;
        }
        self.shop_index += 1;
        if self.shop_index == self.players.len() {
            self.phase = SessionPhase::Transition;
        }
        true
    }

    /// Earnings explain one completed round; balances, wins, identities, and ammunition belong
    /// to the continuing session and deliberately remain untouched.
    pub fn clear_round_earnings(&mut self) {
        for (_, earnings) in &mut self.earnings {
            *earnings = RoundEarnings::default();
        }
        for player in &mut self.players {
            player.loadout.reset_selection();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{match_setup::MatchConfiguration, weapon::WeaponAvailability};
    #[test]
    fn actual_opponent_damage_only_is_paid() {
        let mut s = GameSession::new(MatchConfiguration::default().players);
        s.credit_damage(PlayerId::One, PlayerId::Two, 12);
        s.credit_damage(PlayerId::One, PlayerId::One, 99);
        assert_eq!(s.player(PlayerId::One).cash, 120);
    }
    #[test]
    fn placement_awards_are_exact_and_once() {
        let mut s = GameSession::new(MatchConfiguration::default().players);
        s.finalise(&[PlayerId::One, PlayerId::Two]);
        s.finalise(&[PlayerId::One, PlayerId::Two]);
        assert_eq!(s.player(PlayerId::One).cash, FIRST_PLACE_AWARD);
        assert_eq!(s.player(PlayerId::Two).cash, SECOND_PLACE_AWARD);
    }

    #[test]
    fn clearing_round_earnings_keeps_session_resources() {
        let mut s = GameSession::new(MatchConfiguration::default().players);
        assert!(
            s.player_mut(PlayerId::One)
                .loadout
                .add_round(WeaponId::HighExplosive)
        );
        assert!(
            s.player_mut(PlayerId::One)
                .loadout
                .select(WeaponId::HighExplosive)
        );
        s.credit_damage(PlayerId::One, PlayerId::Two, 12);
        s.finalise(&[PlayerId::One, PlayerId::Two]);
        let player = s.player(PlayerId::One).clone();
        s.clear_round_earnings();
        assert_eq!(s.player(PlayerId::One).configuration, player.configuration);
        assert_eq!(s.player(PlayerId::One).cash, player.cash);
        assert_eq!(s.player(PlayerId::One).wins, player.wins);
        assert_eq!(
            s.player(PlayerId::One)
                .loadout
                .availability(WeaponId::HighExplosive),
            player.loadout.availability(WeaponId::HighExplosive)
        );
        assert_eq!(
            s.player(PlayerId::One).loadout.selected(),
            WeaponId::BasicShell
        );
        assert!(
            s.earnings
                .iter()
                .all(|(_, earnings)| *earnings == RoundEarnings::default())
        );
    }

    #[test]
    fn shop_purchase_is_atomic_and_player_scoped() {
        let mut session = GameSession::new(MatchConfiguration::default().players);
        session.player_mut(PlayerId::One).cash = 600;
        session.begin_shop();
        let before = session
            .player(PlayerId::One)
            .loadout
            .availability(WeaponId::Mirv);
        assert!(session.purchase(PlayerId::One, WeaponId::Mirv));
        assert_eq!(session.player(PlayerId::One).cash, 0);
        assert_eq!(
            session
                .player(PlayerId::One)
                .loadout
                .availability(WeaponId::Mirv),
            WeaponAvailability::Remaining(1)
        );
        assert_eq!(before, WeaponAvailability::Remaining(0));
        assert!(!session.purchase(PlayerId::One, WeaponId::HighExplosive));
        assert!(!session.purchase(PlayerId::Two, WeaponId::HighExplosive));
        assert_eq!(
            session
                .player(PlayerId::Two)
                .loadout
                .availability(WeaponId::HighExplosive),
            WeaponAvailability::Remaining(0)
        );
    }

    #[test]
    fn shop_visits_every_configured_player_then_transitions() {
        let mut session = GameSession::new(MatchConfiguration::default().players);
        session.begin_shop();
        assert_eq!(session.active_shop_player(), Some(PlayerId::One));
        assert!(session.complete_active_shopper());
        assert_eq!(session.active_shop_player(), Some(PlayerId::Two));
        assert!(session.complete_active_shopper());
        assert_eq!(session.phase, SessionPhase::Transition);
        assert!(!session.complete_active_shopper());
    }

    #[test]
    fn shop_catalogue_has_every_limited_weapon_and_never_basic() {
        let items = weapon_shop_items();
        assert_eq!(items.len(), 10);
        assert!(
            items
                .iter()
                .all(|item| item.category == ShopCategory::Weapons && item.price > 0)
        );
        assert!(items.iter().all(|item| item.weapon != WeaponId::BasicShell));
        assert!(items.iter().any(|item| item.weapon == WeaponId::Nuke));
    }
}
