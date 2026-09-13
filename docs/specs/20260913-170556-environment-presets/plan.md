# Implementation Plan: Environment Presets

**Branch**: `20260913-170556-environment-presets` | **Date**: 2026-09-13 | **Spec**: [spec.md](./spec.md)

## Summary

Create one deterministic environment definition selected in setup and retained by a match. It supplies gravity, wind policy, terrain profile, and presentation palette for Earth, Moon, Storm, Crusher, Turnwind, and Bowl. Existing weapons, turns, collision, and camera remain authoritative and unchanged.

## Technical Context

**Language**: Rust 2024; **Runtime**: existing Bevy desktop application; **Storage**: in-memory setup/session resources; **Testing**: Cargo unit tests, format, Clippy, build; **Constraints**: deterministic 2–8 player worlds, no new dependencies, visual presentation cannot change combat state.

## Constitution Check

| Gate | Response | Status |
|---|---|---|
| Fun/simple systems | Presets compose existing gravity, wind, terrain, and presentation rather than adding drag or aiming aids. | Pass |
| Deterministic/testable | Preset and Turnwind use labelled deterministic seeds and pure world generation. | Pass |
| Presentation boundary | Palette, stars, clouds, and horizon read selected state but do not own collision or combat bounds. | Pass |
| Scope | Six curated worlds only; advanced atmosphere and additional worlds remain roadmap work. | Pass |

## Project Structure

```text
crates/azimuth-game/src/
├── environment.rs      # Preset definition, gravity/wind/terrain/presentation policies
├── match_setup.rs      # Setup selection and retained match configuration
├── battlefield.rs      # Terrain profiles, Bowl geometry, palette-aware mesh/horizon data
└── main.rs             # World creation, Turnwind lifecycle, setup/HUD and sky/cloud/star presentation
```

**Structure Decision**: Introduce one small engine-independent environment module rather than scattering preset conditionals. Existing battlefield and main presentation boundaries consume its definition.
