#![allow(dead_code)]

use crate::tank::PlayerId;

pub const MIN_PLAYERS: usize = 2;
pub const MAX_PLAYERS: usize = 8;
pub const MAX_DISPLAY_NAME_LEN: usize = 20;
pub const AI_NAMES: [&str; 16] = [
    "Boomstick Bob",
    "Crater Kate",
    "Deadeye Dave",
    "Windage Wendy",
    "Major Miss",
    "Captain Splash",
    "General Error",
    "Shellby",
    "Howitzer Hank",
    "Blastin' Beth",
    "Gustavo",
    "Tank Sinatra",
    "Mortar Marty",
    "Bunker Brenda",
    "Ricochet Rick",
    "Sarge Splat",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControllerType {
    Human,
    Ai,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerVisualIdentity {
    Red,
    Blue,
    Green,
    Orange,
    Purple,
    Cyan,
    Pink,
    Yellow,
}
impl PlayerVisualIdentity {
    pub const ALL: [Self; 8] = [
        Self::Red,
        Self::Blue,
        Self::Green,
        Self::Orange,
        Self::Purple,
        Self::Cyan,
        Self::Pink,
        Self::Yellow,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Red => "Red",
            Self::Blue => "Blue",
            Self::Green => "Green",
            Self::Orange => "Orange",
            Self::Purple => "Purple",
            Self::Cyan => "Cyan",
            Self::Pink => "Pink",
            Self::Yellow => "Yellow",
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlayerConfiguration {
    pub id: PlayerId,
    pub display_name: String,
    pub controller: ControllerType,
    pub visual: PlayerVisualIdentity,
    /// Retained while an AI name is shown, so switching back to Human restores the edit.
    human_name: String,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchConfiguration {
    pub players: Vec<PlayerConfiguration>,
    name_cursor: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MatchConfigurationError {
    PlayerCount,
    DuplicateIdentity,
    EmptyName,
    NameTooLong,
    AiUnavailable,
}

impl Default for MatchConfiguration {
    fn default() -> Self {
        Self::with_player_count(MIN_PLAYERS).expect("two players are valid")
    }
}
impl MatchConfiguration {
    pub fn with_player_count(count: usize) -> Result<Self, MatchConfigurationError> {
        if !(MIN_PLAYERS..=MAX_PLAYERS).contains(&count) {
            return Err(MatchConfigurationError::PlayerCount);
        }
        Ok(Self {
            players: (1..=count).map(Self::human_slot).collect(),
            name_cursor: 0,
        })
    }
    fn human_slot(number: usize) -> PlayerConfiguration {
        let human_name = format!("Player {number}");
        PlayerConfiguration {
            id: PlayerId(number as u8),
            display_name: human_name.clone(),
            controller: ControllerType::Human,
            visual: PlayerVisualIdentity::ALL[number - 1],
            human_name,
        }
    }
    pub fn set_player_count(&mut self, count: usize) -> Result<(), MatchConfigurationError> {
        if !(MIN_PLAYERS..=MAX_PLAYERS).contains(&count) {
            return Err(MatchConfigurationError::PlayerCount);
        }
        self.players.truncate(count);
        while self.players.len() < count {
            self.players.push(Self::human_slot(self.players.len() + 1));
        }
        Ok(())
    }
    pub fn set_name(&mut self, id: PlayerId, name: &str) -> Result<(), MatchConfigurationError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(MatchConfigurationError::EmptyName);
        }
        if name.chars().count() > MAX_DISPLAY_NAME_LEN {
            return Err(MatchConfigurationError::NameTooLong);
        }
        let player = self
            .players
            .iter_mut()
            .find(|player| player.id == id)
            .expect("configured player exists");
        player.display_name = name.into();
        player.human_name = name.into();
        Ok(())
    }
    pub fn append_human_name_character(&mut self, id: PlayerId, character: char) {
        let player = self
            .players
            .iter_mut()
            .find(|player| player.id == id)
            .expect("configured player exists");
        if player.controller == ControllerType::Human
            && player.display_name.chars().count() < MAX_DISPLAY_NAME_LEN
        {
            player.display_name.push(character);
            player.human_name.push(character);
        }
    }
    pub fn backspace_human_name(&mut self, id: PlayerId) {
        let player = self
            .players
            .iter_mut()
            .find(|player| player.id == id)
            .expect("configured player exists");
        if player.controller == ControllerType::Human {
            player.display_name.pop();
            player.human_name.pop();
        }
    }
    pub fn trim_human_names(&mut self) {
        for player in &mut self.players {
            if player.controller == ControllerType::Human {
                let trimmed = player.display_name.trim().to_owned();
                player.display_name = trimmed.clone();
                player.human_name = trimmed;
            }
        }
    }
    pub fn set_controller(&mut self, id: PlayerId, controller: ControllerType) {
        let index = self
            .players
            .iter()
            .position(|player| player.id == id)
            .expect("configured player exists");
        if controller == ControllerType::Ai {
            let name = self.next_ai_name(id);
            self.players[index].display_name = name;
        } else if self.players[index].controller == ControllerType::Ai {
            self.players[index].display_name = self.players[index].human_name.clone();
        }
        self.players[index].controller = controller;
    }
    pub fn reroll_ai_name(&mut self, id: PlayerId) {
        if self
            .players
            .iter()
            .any(|player| player.id == id && player.controller == ControllerType::Ai)
        {
            let name = self.next_ai_name(id);
            self.players
                .iter_mut()
                .find(|player| player.id == id)
                .unwrap()
                .display_name = name;
        }
    }
    fn next_ai_name(&mut self, id: PlayerId) -> String {
        let used: Vec<_> = self
            .players
            .iter()
            .filter(|player| player.id != id)
            .map(|player| player.display_name.as_str())
            .collect();
        for offset in 0..AI_NAMES.len() {
            let name = AI_NAMES[(self.name_cursor + offset) % AI_NAMES.len()];
            if !used.contains(&name) {
                self.name_cursor = (self.name_cursor + offset + 1) % AI_NAMES.len();
                return name.into();
            }
        }
        AI_NAMES[self.name_cursor % AI_NAMES.len()].into()
    }
    pub fn validate(&self) -> Result<(), MatchConfigurationError> {
        if !(MIN_PLAYERS..=MAX_PLAYERS).contains(&self.players.len()) {
            return Err(MatchConfigurationError::PlayerCount);
        }
        for player in &self.players {
            if player.display_name.trim().is_empty() {
                return Err(MatchConfigurationError::EmptyName);
            }
            if player.display_name.chars().count() > MAX_DISPLAY_NAME_LEN {
                return Err(MatchConfigurationError::NameTooLong);
            }
        }
        if self
            .players
            .iter()
            .enumerate()
            .any(|(i, player)| self.players[..i].iter().any(|other| other.id == player.id))
        {
            return Err(MatchConfigurationError::DuplicateIdentity);
        }
        if self
            .players
            .iter()
            .any(|player| player.controller == ControllerType::Ai)
        {
            return Err(MatchConfigurationError::AiUnavailable);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accepts_every_supported_count() {
        for count in MIN_PLAYERS..=MAX_PLAYERS {
            assert_eq!(
                MatchConfiguration::with_player_count(count)
                    .unwrap()
                    .players
                    .len(),
                count
            );
        }
    }
    #[test]
    fn rejects_invalid_counts_and_names() {
        assert!(MatchConfiguration::with_player_count(1).is_err());
        assert!(MatchConfiguration::with_player_count(9).is_err());
        let mut c = MatchConfiguration::default();
        assert_eq!(
            c.set_name(PlayerId::One, "  "),
            Err(MatchConfigurationError::EmptyName)
        );
    }
    #[test]
    fn ai_keeps_identity_colour_and_blocks_start() {
        let mut c = MatchConfiguration::default();
        let before = c.players[0].clone();
        c.set_controller(before.id, ControllerType::Ai);
        assert_eq!(c.players[0].id, before.id);
        assert_eq!(c.players[0].visual, before.visual);
        assert!(AI_NAMES.contains(&c.players[0].display_name.as_str()));
        assert_eq!(c.validate(), Err(MatchConfigurationError::AiUnavailable));
    }

    #[test]
    fn switching_back_to_human_restores_its_editable_name() {
        let mut configuration = MatchConfiguration::default();
        let id = configuration.players[0].id;
        configuration.set_name(id, "Ada").unwrap();
        configuration.set_controller(id, ControllerType::Ai);
        configuration.set_controller(id, ControllerType::Human);
        assert_eq!(configuration.players[0].display_name, "Ada");
    }

    #[test]
    fn finishing_keyboard_edits_trims_a_human_name() {
        let mut configuration = MatchConfiguration::default();
        let id = configuration.players[0].id;
        configuration.set_name(id, "Ada").unwrap();
        configuration.append_human_name_character(id, ' ');
        configuration.trim_human_names();
        assert_eq!(configuration.players[0].display_name, "Ada");
        assert!(configuration.validate().is_ok());
    }
}
