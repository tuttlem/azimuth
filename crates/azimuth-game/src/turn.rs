use crate::{
    aiming::{AimAdjustment, AimingState},
    tank::PlayerId,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TurnPhase {
    Ready,
    Resolving,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TurnState {
    pub current_player: PlayerId,
    pub phase: TurnPhase,
    player_one_aim: AimingState,
    player_two_aim: AimingState,
}

impl TurnState {
    pub fn new(player_one_aim: AimingState, player_two_aim: AimingState) -> Self {
        Self {
            current_player: PlayerId::One,
            phase: TurnPhase::Ready,
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
        if self.phase != TurnPhase::Ready {
            return false;
        }

        match self.current_player {
            PlayerId::One => self.player_one_aim.apply(adjustment, coarse),
            PlayerId::Two => self.player_two_aim.apply(adjustment, coarse),
        }
        true
    }

    pub fn begin_fire(&mut self) -> Option<(PlayerId, AimingState)> {
        if self.phase != TurnPhase::Ready {
            return None;
        }

        self.phase = TurnPhase::Resolving;
        Some((self.current_player, self.current_aim()))
    }

    pub fn complete_resolution(&mut self) -> bool {
        if self.phase != TurnPhase::Resolving {
            return false;
        }

        self.current_player = self.current_player.other();
        self.phase = TurnPhase::Ready;
        true
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
    fn player_one_starts_ready_with_their_own_aim() {
        let state = state();

        assert_eq!(state.current_player, PlayerId::One);
        assert_eq!(state.phase, TurnPhase::Ready);
        assert_eq!(state.current_aim(), state.aim_for(PlayerId::One));
    }

    #[test]
    fn firing_resolves_before_switching_players() {
        let mut state = state();
        let fired = state.begin_fire();

        assert_eq!(fired, Some((PlayerId::One, state.aim_for(PlayerId::One))));
        assert_eq!(state.current_player, PlayerId::One);
        assert_eq!(state.phase, TurnPhase::Resolving);
        assert!(state.begin_fire().is_none());

        assert!(state.complete_resolution());
        assert_eq!(state.current_player, PlayerId::Two);
        assert_eq!(state.phase, TurnPhase::Ready);
    }

    #[test]
    fn each_player_retains_independent_aim_across_a_round_trip() {
        let mut state = state();
        let player_one = state.current_aim();
        state.apply_current_aim(AimAdjustment::AzimuthIncrease, false);
        let changed_player_one = state.current_aim();
        assert_ne!(player_one, changed_player_one);
        state.begin_fire();
        state.complete_resolution();

        let player_two = state.current_aim();
        state.apply_current_aim(AimAdjustment::PowerDecrease, false);
        let changed_player_two = state.current_aim();
        assert_ne!(player_two, changed_player_two);
        assert_eq!(state.aim_for(PlayerId::One), changed_player_one);
        state.begin_fire();
        state.complete_resolution();

        assert_eq!(state.current_player, PlayerId::One);
        assert_eq!(state.current_aim(), changed_player_one);
        assert_eq!(state.aim_for(PlayerId::Two), changed_player_two);
    }

    #[test]
    fn resolving_rejects_aim_and_repeated_fire() {
        let mut state = state();
        let before = state.current_aim();
        state.begin_fire();

        assert!(!state.apply_current_aim(AimAdjustment::ElevationIncrease, false));
        assert!(state.begin_fire().is_none());
        assert_eq!(state.current_aim(), before);
    }

    #[test]
    fn twenty_completions_alternate_deterministically() {
        let mut state = state();
        for index in 0..20 {
            let expected = if index % 2 == 0 {
                PlayerId::One
            } else {
                PlayerId::Two
            };
            assert_eq!(state.current_player, expected);
            assert!(state.begin_fire().is_some());
            assert!(state.complete_resolution());
        }
        assert_eq!(state.current_player, PlayerId::One);
    }

    #[test]
    fn identical_action_sequences_have_identical_turn_progression() {
        fn play_trace() -> Vec<(PlayerId, TurnPhase, AimingState)> {
            let mut state = state();
            let mut trace = Vec::new();
            for adjustment in [
                AimAdjustment::AzimuthIncrease,
                AimAdjustment::PowerDecrease,
                AimAdjustment::ElevationIncrease,
                AimAdjustment::AzimuthDecrease,
            ] {
                state.apply_current_aim(adjustment, false);
                state.begin_fire();
                trace.push((state.current_player, state.phase, state.current_aim()));
                state.complete_resolution();
                trace.push((state.current_player, state.phase, state.current_aim()));
            }
            trace
        }

        assert_eq!(play_trace(), play_trace());
    }
}
