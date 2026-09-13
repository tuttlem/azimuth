# Research: Environment Presets

## Preset definition

**Decision**: A compact enum/definition owns label, gravity, wind policy, terrain profile, and presentation profile.

**Rationale**: A single shared source prevents setup, world generation, HUD, AI, and rendering from drifting into contradictory per-world rules.

**Alternatives considered**: Loose booleans and per-system conditionals were rejected as error-prone; data-driven external configuration is premature for six fixed worlds.

## Turnwind

**Decision**: Derive its wind from match seed plus round/player-turn ordinal exactly at handoff, and retain it for the complete turn/flight/resolution.

**Rationale**: It is surprising but readable, deterministic, and never changes a committed shot.

## Terrain and presentation

**Decision**: Bowl is a deterministic terrain height profile; Moon/Storm/Crusher palettes and sky treatments are render-only profiles. Moon replaces clouds with deterministic stars.

**Rationale**: Terrain remains one collision/render source while presentation remains non-authoritative.
