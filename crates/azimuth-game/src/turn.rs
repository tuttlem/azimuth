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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TurnState {
    pub current_player: PlayerId,
    pub phase: TurnPhase,
    pub match_state: MatchState,
    player_one_aim: AimingState,
    player_two_aim: AimingState,
}

impl TurnState {
    pub fn new(player_one_aim: AimingState, player_two_aim: AimingState) -> Self {
        Self {
            current_player: PlayerId::One,
            phase: TurnPhase::Choosing,
            match_state: MatchState::InProgress,
            player_one_aim,
            player_two_aim,
        }
    }

    pub fn aim_for(self, player: PlayerId) -> AimingState {
        match player {
            PlayerId::One => self.player_one_aim,
            PlayerId::Two => self.player_two_aim,
        }
    }

    pub fn current_aim(self) -> AimingState {
        self.aim_for(self.current_player)
    }

    pub fn apply_current_aim(&mut self, adjustment: AimAdjustment, coarse: bool) -> bool {
        if self.match_state != MatchState::InProgress || self.phase != TurnPhase::Choosing {
            return false;
        }
        match self.current_player {
            PlayerId::One => self.player_one_aim.apply(adjustment, coarse),
            PlayerId::Two => self.player_two_aim.apply(adjustment, coarse),
        }
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

    pub fn remaining_movement(self) -> Option<u8> {
        match self.phase {
            TurnPhase::Moving { remaining_steps } => Some(remaining_steps),
            TurnPhase::Choosing | TurnPhase::ResolvingFire | TurnPhase::Finished => None,
        }
    }

    /// The caller must first commit a terrain-validated tank pose. This order makes an invalid
    /// movement request incapable of spending allowance or advancing a turn.
    pub fn accept_movement_step(&mut self) -> bool {
        let TurnPhase::Moving { remaining_steps } = self.phase else {
            return false;
        };
        if remaining_steps == 0 {
            return false;
        }
        if remaining_steps == 1 {
            self.advance_to_next_choosing();
        } else {
            self.phase = TurnPhase::Moving {
                remaining_steps: remaining_steps - 1,
            };
        }
        true
    }

    pub fn finish_movement(&mut self) -> bool {
        if self.match_state != MatchState::InProgress || self.remaining_movement().is_none() {
            return false;
        }
        self.advance_to_next_choosing();
        true
    }

    pub fn begin_fire(&mut self) -> Option<(PlayerId, AimingState)> {
        if self.match_state != MatchState::InProgress || self.phase != TurnPhase::Choosing {
            return None;
        }
        self.phase = TurnPhase::ResolvingFire;
        Some((self.current_player, self.current_aim()))
    }

    /// Completes a firing turn only after every authoritative consequence, including terrain
    /// response, has settled. Presentation systems never call or gate this transition.
    pub fn complete_fire_resolution(&mut self, survivors: [bool; 2]) -> bool {
        if self.phase != TurnPhase::ResolvingFire {
            return false;
        }
        self.match_state = match survivors {
            [true, true] => MatchState::InProgress,
            [true, false] => MatchState::Winner(PlayerId::One),
            [false, true] => MatchState::Winner(PlayerId::Two),
            [false, false] => MatchState::Draw,
        };
        if self.match_state == MatchState::InProgress {
            self.advance_to_next_choosing();
        } else {
            self.phase = TurnPhase::Finished;
        }
        true
    }

    fn advance_to_next_choosing(&mut self) {
        self.current_player = self.current_player.other();
        self.phase = TurnPhase::Choosing;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state() -> TurnState {
        TurnState::new(
            AimingState::new(10.0, 20.0, 12.0),
            AimingState::new(210.0, 60.0, 24.0),
        )
    }

    #[test]
    fn player_one_starts_choosing_with_their_own_aim() {
        let state = state();
        assert_eq!(state.current_player, PlayerId::One);
        assert_eq!(state.phase, TurnPhase::Choosing);
        assert_eq!(state.current_aim(), state.aim_for(PlayerId::One));
    }

    #[test]
    fn choosing_exactly_one_primary_action_locks_the_other() {
        let mut state = state();
        assert!(state.begin_movement());
        assert_eq!(state.remaining_movement(), Some(MOVEMENT_ALLOWANCE));
        assert!(state.begin_fire().is_none());
        assert!(!state.apply_current_aim(AimAdjustment::ElevationIncrease, false));
        assert!(!state.begin_movement());
        assert!(state.finish_movement());
        assert_eq!(state.current_player, PlayerId::Two);
        assert!(state.begin_fire().is_some());
        assert!(!state.begin_movement());
    }

    #[test]
    fn firing_resolves_before_switching_players() {
        let mut state = state();
        let fired = state.begin_fire();
        assert_eq!(fired, Some((PlayerId::One, state.aim_for(PlayerId::One))));
        assert_eq!(state.phase, TurnPhase::ResolvingFire);
        assert!(state.complete_fire_resolution([true, true]));
        assert_eq!(state.current_player, PlayerId::Two);
        assert_eq!(state.phase, TurnPhase::Choosing);
    }

    #[test]
    fn movement_allowance_is_consumed_only_by_accepted_steps() {
        let mut state = state();
        assert!(state.begin_movement());
        for expected in (1..MOVEMENT_ALLOWANCE).rev() {
            assert!(state.accept_movement_step());
            assert_eq!(state.remaining_movement(), Some(expected));
            assert_eq!(state.current_player, PlayerId::One);
        }
        assert!(state.accept_movement_step());
        assert_eq!(state.current_player, PlayerId::Two);
        assert_eq!(state.phase, TurnPhase::Choosing);
        assert!(!state.accept_movement_step());
    }

    #[test]
    fn ending_movement_early_forfeits_allowance_once() {
        let mut state = state();
        state.begin_movement();
        state.accept_movement_step();
        assert_eq!(state.remaining_movement(), Some(MOVEMENT_ALLOWANCE - 1));
        assert!(state.finish_movement());
        assert_eq!(state.current_player, PlayerId::Two);
        assert!(!state.finish_movement());
    }

    #[test]
    fn each_player_retains_independent_aim_across_movement_and_fire() {
        let mut state = state();
        state.apply_current_aim(AimAdjustment::AzimuthIncrease, false);
        let changed_player_one = state.current_aim();
        state.begin_movement();
        state.finish_movement();
        let player_two = state.current_aim();
        state.apply_current_aim(AimAdjustment::PowerDecrease, false);
        state.begin_fire();
        state.complete_fire_resolution([true, true]);

        assert_eq!(state.current_player, PlayerId::One);
        assert_eq!(state.current_aim(), changed_player_one);
        assert_eq!(
            state.aim_for(PlayerId::Two).azimuth_degrees,
            player_two.azimuth_degrees
        );
    }

    #[test]
    fn resolving_rejects_every_other_action() {
        let mut state = state();
        let before = state.current_aim();
        state.begin_fire();
        assert!(!state.apply_current_aim(AimAdjustment::ElevationIncrease, false));
        assert!(state.begin_fire().is_none());
        assert!(!state.begin_movement());
        assert!(!state.finish_movement());
        assert!(!state.accept_movement_step());
        assert_eq!(state.current_aim(), before);
    }

    #[test]
    fn identical_action_sequences_have_identical_turn_progression() {
        fn play_trace() -> Vec<(PlayerId, TurnPhase, AimingState)> {
            let mut state = state();
            let mut trace = Vec::new();
            for movement in [true, false, true, false] {
                if movement {
                    state.begin_movement();
                    state.accept_movement_step();
                    state.finish_movement();
                } else {
                    state.apply_current_aim(AimAdjustment::AzimuthIncrease, false);
                    state.begin_fire();
                    trace.push((state.current_player, state.phase, state.current_aim()));
                    state.complete_fire_resolution([true, true]);
                }
                trace.push((state.current_player, state.phase, state.current_aim()));
            }
            trace
        }
        assert_eq!(play_trace(), play_trace());
    }

    #[test]
    fn lethal_resolution_finishes_match_and_rejects_actions() {
        let mut state = state();
        state.begin_fire();
        assert!(state.complete_fire_resolution([true, false]));
        assert_eq!(state.match_state, MatchState::Winner(PlayerId::One));
        assert_eq!(state.phase, TurnPhase::Finished);
        assert!(!state.begin_movement());
        assert!(state.begin_fire().is_none());
        assert!(!state.apply_current_aim(AimAdjustment::PowerIncrease, false));
    }

    #[test]
    fn simultaneous_elimination_is_a_draw() {
        let mut state = state();
        state.begin_fire();
        assert!(state.complete_fire_resolution([false, false]));
        assert_eq!(state.match_state, MatchState::Draw);
        assert_eq!(state.phase, TurnPhase::Finished);
    }
}
