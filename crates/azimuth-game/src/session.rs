use crate::{
    match_setup::PlayerConfiguration,
    tank::PlayerId,
    turn::MatchState,
    weapon::{PlayerWeaponLoadout, WeaponId, weapon_price},
};
use bevy::prelude::Resource;

pub const CASH_PER_DAMAGE: u32 = 10;
pub const ELIMINATION_BONUS: u32 = 250;
pub const ROUND_WIN_BONUS: u32 = 500;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionPhase {
    Setup,
    Playing,
    Celebrating,
    Accounting,
    Shopping,
    Transition,
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
    pub damage: u32,
    pub eliminations: u32,
    pub winner: u32,
}
impl RoundEarnings {
    pub fn total(self) -> u32 {
        self.damage + self.eliminations + self.winner
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Resource)]
pub struct GameSession {
    pub players: Vec<SessionPlayer>,
    pub round_number: u32,
    pub phase: SessionPhase,
    pub earnings: Vec<(PlayerId, RoundEarnings)>,
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
    pub fn credit_damage(
        &mut self,
        owner: PlayerId,
        target: PlayerId,
        applied: u8,
        eliminated: bool,
    ) {
        if owner == target {
            return;
        }
        let damage = applied as u32 * CASH_PER_DAMAGE;
        let elimination = if eliminated { ELIMINATION_BONUS } else { 0 };
        let earned = self.earnings_mut(owner);
        earned.damage += damage;
        earned.eliminations += elimination;
        self.player_mut(owner).cash += damage + elimination;
    }
    pub fn finalise(&mut self, result: MatchState) {
        if let MatchState::Winner(winner) = result {
            if self.phase == SessionPhase::Celebrating {
                return;
            }
            self.earnings_mut(winner).winner += ROUND_WIN_BONUS;
            let player = self.player_mut(winner);
            player.cash += ROUND_WIN_BONUS;
            player.wins += 1;
        }
        self.phase = SessionPhase::Celebrating;
    }
    pub fn purchase(&mut self, player: PlayerId, weapon: WeaponId) -> bool {
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
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::match_setup::MatchConfiguration;
    #[test]
    fn actual_opponent_damage_only_is_paid() {
        let mut s = GameSession::new(MatchConfiguration::default().players);
        s.credit_damage(PlayerId::One, PlayerId::Two, 12, true);
        s.credit_damage(PlayerId::One, PlayerId::One, 99, true);
        assert_eq!(s.player(PlayerId::One).cash, 370);
    }
    #[test]
    fn winner_bonus_is_once() {
        let mut s = GameSession::new(MatchConfiguration::default().players);
        s.finalise(MatchState::Winner(PlayerId::One));
        s.finalise(MatchState::Winner(PlayerId::One));
        assert_eq!(s.player(PlayerId::One).cash, 500);
    }
}
