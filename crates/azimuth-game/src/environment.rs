#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerrainProfile {
    Standard,
    Bowl,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkyProfile {
    Clear,
    MoonStars,
    Storm,
    CrusherHaze,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaletteProfile {
    Earth,
    Moon,
    Storm,
    Crusher,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindPolicy {
    StableMatch,
    RerollEachTurn,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EnvironmentPreset {
    #[default]
    Earth,
    Moon,
    Storm,
    Crusher,
    Turnwind,
    Bowl,
}
impl EnvironmentPreset {
    pub const ALL: [Self; 6] = [
        Self::Earth,
        Self::Moon,
        Self::Storm,
        Self::Crusher,
        Self::Turnwind,
        Self::Bowl,
    ];
    pub const fn label(self) -> &'static str {
        match self {
            Self::Earth => "Earth",
            Self::Moon => "Moon",
            Self::Storm => "Storm",
            Self::Crusher => "Crusher",
            Self::Turnwind => "Turnwind",
            Self::Bowl => "Bowl",
        }
    }
    pub const fn summary(self) -> &'static str {
        match self {
            Self::Earth => "Normal gravity, gentle stable wind.",
            Self::Moon => "Low gravity, calm air, starry dust world.",
            Self::Storm => "Normal gravity, strong stable wind, dark skies.",
            Self::Crusher => "High gravity, short arcs, warm badlands.",
            Self::Turnwind => "Normal gravity; wind rerolls before each turn.",
            Self::Bowl => "Normal physics around a broad central depression.",
        }
    }
    pub const fn next(self) -> Self {
        match self {
            Self::Earth => Self::Moon,
            Self::Moon => Self::Storm,
            Self::Storm => Self::Crusher,
            Self::Crusher => Self::Turnwind,
            Self::Turnwind => Self::Bowl,
            Self::Bowl => Self::Earth,
        }
    }
    pub const fn gravity(self) -> f32 {
        match self {
            Self::Moon => 2.0,
            Self::Crusher => 15.0,
            _ => 8.0,
        }
    }
    pub const fn wind_range(self) -> (f32, f32) {
        match self {
            Self::Moon => (0.0, 0.0),
            Self::Storm => (2.2, 3.8),
            _ => (0.75, 1.75),
        }
    }
    pub const fn wind_policy(self) -> WindPolicy {
        match self {
            Self::Turnwind => WindPolicy::RerollEachTurn,
            _ => WindPolicy::StableMatch,
        }
    }
    pub const fn terrain_profile(self) -> TerrainProfile {
        match self {
            Self::Bowl => TerrainProfile::Bowl,
            _ => TerrainProfile::Standard,
        }
    }
    pub const fn palette(self) -> PaletteProfile {
        match self {
            Self::Moon => PaletteProfile::Moon,
            Self::Storm => PaletteProfile::Storm,
            Self::Crusher => PaletteProfile::Crusher,
            _ => PaletteProfile::Earth,
        }
    }
    pub const fn sky(self) -> SkyProfile {
        match self {
            Self::Moon => SkyProfile::MoonStars,
            Self::Storm => SkyProfile::Storm,
            Self::Crusher => SkyProfile::CrusherHaze,
            _ => SkyProfile::Clear,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn presets_cycle_and_define_distinct_physics() {
        assert_eq!(EnvironmentPreset::ALL.len(), 6);
        assert_eq!(EnvironmentPreset::Bowl.next(), EnvironmentPreset::Earth);
        assert!(EnvironmentPreset::Moon.gravity() < EnvironmentPreset::Earth.gravity());
        assert!(EnvironmentPreset::Crusher.gravity() > EnvironmentPreset::Earth.gravity());
        assert_eq!(
            EnvironmentPreset::Turnwind.wind_policy(),
            WindPolicy::RerollEachTurn
        );
        assert_eq!(
            EnvironmentPreset::Bowl.terrain_profile(),
            TerrainProfile::Bowl
        );
        assert_eq!(EnvironmentPreset::Moon.sky(), SkyProfile::MoonStars);
    }
}
