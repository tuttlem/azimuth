use crate::{
    aiming::{AimAdjustment, AimingState},
    tank::{MOVEMENT_ALLOWANCE, PlayerId},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TurnPhase {
    Choosing,
    Moving { remaining_steps: u8 },
    ResolvingFire,
    Finished,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MatchState {
    InProgress,
    Winner(PlayerId),
    Draw,
}

/// Slot order is the initial turn order. All per-player values are keyed by stable identity.
#[derive(Clone, Debug, PartialEq)]
pub struct TurnState {
    pub current_player: PlayerId,
    pub phase: TurnPhase,
    pub match_state: MatchState,
    players: Vec<PlayerId>,
    aims: Vec<(PlayerId, AimingState)>,
}
impl TurnState {
    pub fn new(aims: Vec<(PlayerId, AimingState)>) -> Self {
        assert!(
            (2..=8).contains(&aims.len()),
            "turn state needs two through eight players"
        );
        let players = aims.iter().map(|(id, _)| *id).collect::<Vec<_>>();
        assert!(
            players.windows(2).all(|pair| pair[0] != pair[1]),
            "player identities must be unique"
        );
        Self {
            current_player: players[0],
            phase: TurnPhase::Choosing,
            match_state: MatchState::InProgress,
            players,
            aims,
        }
    }
    pub fn aim_for(&self, player: PlayerId) -> AimingState {
        self.aims
            .iter()
            .find(|(id, _)| *id == player)
            .map(|(_, aim)| *aim)
            .expect("participant has aim")
    }
    pub fn current_aim(&self) -> AimingState {
        self.aim_for(self.current_player)
    }
    pub fn apply_current_aim(&mut self, adjustment: AimAdjustment, coarse: bool) -> bool {
        if self.match_state != MatchState::InProgress || self.phase != TurnPhase::Choosing {
            return false;
        }
        self.aims
            .iter_mut()
            .find(|(id, _)| *id == self.current_player)
            .expect("active aim")
            .1
            .apply(adjustment, coarse);
        true
    }
    pub fn begin_movement(&mut self) -> bool {
        if self.match_state != MatchState::InProgress || self.phase != TurnPhase::Choosing {
            return false;
        }
        self.phase = TurnPhase::Moving {
            remaining_steps: MOVEMENT_ALLOWANCE,
        };
        true
    }
    pub fn remaining_movement(&self) -> Option<u8> {
        match self.phase {
            TurnPhase::Moving { remaining_steps } => Some(remaining_steps),
            _ => None,
        }
    }
    pub fn accept_movement_step(&mut self, survivors: impl AsRef<[bool]>) -> bool {
        let TurnPhase::Moving { remaining_steps } = self.phase else {
            return false;
        };
        if remaining_steps == 0 {
            return false;
        }
        if remaining_steps == 1 {
            self.advance(survivors.as_ref());
        } else {
            self.phase = TurnPhase::Moving {
                remaining_steps: remaining_steps - 1,
            };
        }
        true
    }
    pub fn finish_movement(&mut self, survivors: impl AsRef<[bool]>) -> bool {
        if self.match_state != MatchState::InProgress || self.remaining_movement().is_none() {
            return false;
        }
        self.advance(survivors.as_ref());
        true
    }
    pub fn begin_fire(&mut self) -> Option<(PlayerId, AimingState)> {
        if self.match_state != MatchState::InProgress || self.phase != TurnPhase::Choosing {
            return None;
        }
        self.phase = TurnPhase::ResolvingFire;
        Some((self.current_player, self.current_aim()))
    }
    pub fn complete_fire_resolution(&mut self, survivors: impl AsRef<[bool]>) -> bool {
        let survivors = survivors.as_ref();
        if self.phase != TurnPhase::ResolvingFire || survivors.len() != self.players.len() {
            return false;
        }
        let alive: Vec<_> = survivors
            .iter()
            .enumerate()
            .filter_map(|(i, alive)| alive.then_some(self.players[i]))
            .collect();
        self.match_state = match alive.as_slice() {
            [] => MatchState::Draw,
            [winner] => MatchState::Winner(*winner),
            _ => MatchState::InProgress,
        };
        if self.match_state == MatchState::InProgress {
            self.advance(survivors);
        } else {
            self.phase = TurnPhase::Finished;
        }
        true
    }
    fn advance(&mut self, survivors: &[bool]) {
        let current = self
            .players
            .iter()
            .position(|id| *id == self.current_player)
            .expect("active player exists");
        for offset in 1..=self.players.len() {
            let index = (current + offset) % self.players.len();
            if survivors.get(index).copied().unwrap_or(false) {
                self.current_player = self.players[index];
                self.phase = TurnPhase::Choosing;
                return;
            }
        }
        self.phase = TurnPhase::Finished;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn state(count: u8) -> TurnState {
        TurnState::new(
            (1..=count)
                .map(|id| (PlayerId(id), AimingState::new(id as f32, 45.0, 18.0)))
                .collect(),
        )
    }
    #[test]
    fn rotates_and_wraps_for_eight() {
        let mut s = state(8);
        for _ in 0..8 {
            s.begin_movement();
            s.finish_movement([true; 8]);
        }
        assert_eq!(s.current_player, PlayerId(1));
    }
    #[test]
    fn skips_eliminated_players() {
        let mut s = state(4);
        s.begin_fire();
        s.complete_fire_resolution([true, false, false, true]);
        assert_eq!(s.current_player, PlayerId(4));
    }
    #[test]
    fn movement_handoff_skips_eliminated_players() {
        let mut s = state(4);
        s.begin_movement();
        assert!(s.finish_movement([true, false, true, true]));
        assert_eq!(s.current_player, PlayerId(3));

        s.begin_movement();
        assert!(s.finish_movement([true, false, true, true]));
        assert_eq!(s.current_player, PlayerId(4));
    }
    #[test]
    fn winner_and_draw_are_generic() {
        let mut s = state(3);
        s.begin_fire();
        s.complete_fire_resolution([false, false, true]);
        assert_eq!(s.match_state, MatchState::Winner(PlayerId(3)));
    }
}
