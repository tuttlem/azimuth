# Implementation Plan: First Audio Pass

**Branch**: `20260910-070129-first-audio-pass` | **Date**: 2026-09-10 | **Spec**: [spec.md](spec.md)

## Summary

Add a small presentation-only audio cue boundary to successful launch, active flight, resolved impact, elimination, and match wind. Reuse existing engine audio support and a tiny licensed asset set for spatial one-shots, restrained loops, and profile-led variation. Audio never owns or delays simulation.

## Technical Context

**Language/Version**: Rust edition 2024, current stable toolchain
**Primary Dependencies**: Bevy 0.18.1 with already-enabled native audio support
**Storage**: Repository audio assets/provenance and in-memory presentation state only
**Testing**: Existing module-local Rust tests; no device or waveform test requirement
**Target Platform**: Native desktop game runtime
**Project Type**: Single desktop-game application crate
**Performance Goals**: One active flight loop and one ambience loop; short-lived one-shots; no fixed-step audio work
**Constraints**: Small licensed asset set; basic positional coherence only; no music, mixer, engine, or acoustic simulation; playback failure cannot affect gameplay
**Scale/Scope**: One projectile, 2–8 players, three current weapons, one gameplay camera

## Constitution Check

### Pre-design gate — PASS

| Principle | Plan response |
|---|---|
| Fun over realism | Emphasise playful weight and contrast, not military acoustics. |
| Coherence | One small cue mapping beside existing presentation code; no new crate or middleware. |
| Determinism | Audio reads captured outcomes and uses no match RNG. |
| Presentation boundary | Playback cannot gate launch, flight, impact, damage, settling, or turns. |
| Scope discipline | Limit to fire, flight, impact, wind ambience, and a justified destruction accent. |
| Dependencies/assets | Reuse engine support and document a tiny licensed set. |

## Project Structure

```text
docs/specs/20260910-070129-first-audio-pass/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/audio-presentation.md
└── tasks.md

crates/azimuth-game/
├── src/main.rs       # cue mapping, playback lifecycle, asset loading, tests
└── assets/audio/     # curated effects and provenance
```

**Structure Decision**: Keep this one feature's presentation boundary in `main.rs`, beside the visual explosion, camera, and HUD systems. A separate module or crate is not earned.

## Post-design Constitution Check

**PASS.** The design observes existing authoritative facts, uses engine-native playback, preserves deterministic game state, and records nonessential sound-design ideas for later.

## Complexity Tracking

No constitution violations or exceptional complexity are required.
