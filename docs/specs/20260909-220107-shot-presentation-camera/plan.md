# Implementation Plan: Shot Presentation Camera

**Branch**: 20260909-220107-shot-presentation-camera | **Date**: 2026-09-09 | **Spec**: [spec.md](spec.md)

## Summary

Replace the one broad in-flight camera intent with a small presentation-only shot state. At the existing shared firing boundary, capture the firing player's configured controller type. Human shots use comfortable offset projectile follow that widens after observed apex; AI shots use broad tactical coverage. Both observe the same lifecycle and converge on a brief consequence-first impact view before next-player or result presentation. Projectile, impact, terrain, settling, and turn authority remain unchanged.

## Technical Context

**Language/Version**: Rust edition 2024, current stable toolchain

**Primary Dependencies**: Bevy 0.18.1

**Storage**: N/A; in-memory presentation state per running match

**Testing**: Rust module-local unit tests; cargo test workspace

**Target Platform**: Native desktop game runtime supported by the current application

**Project Type**: Single desktop-game application in a Cargo workspace

**Performance Goals**: Lightweight per-rendered-frame camera evaluation; no landing prediction and no extra fixed-step simulation work

**Constraints**: Preserve deterministic simulation, existing HUD/input, bounds, pitch and distance safety; no dependencies or generic cinematic framework; controller-specific behavior is presentation-only

**Scale/Scope**: One application crate, one battlefield camera, 2–8 Human/AI players, one active projectile, no external service or persistence

## Constitution Check

### Pre-design gate — PASS

| Principle | Plan response |
|---|---|
| Fun and simple emergent gameplay | Human framing teaches arc/wind/terrain; AI framing preserves awareness. No simulation changes. |
| Coherence and boundaries | Extend existing camera code with one small explicit state and pure helpers. No new crate or sequencing engine. |
| Deterministic testable simulation | Presentation reads existing state only; pure decisions get unit tests; existing authoritative fixtures remain unchanged. |
| Presentation must not own the game | Camera never writes or gates projectile, terrain, tank, weapon, or turn resolution. |
| Scope and roadmap discipline | Implement only controller-aware flight and shared impact coverage. Prediction, replay, slow motion, AI behavior, audio, and HUD redesign remain excluded. |
| Dependencies, comments, quality | Reuse existing facilities, centralise tuning, document intent, and run test/format/lint/manual review before roadmap completion. |

No violation needs a complexity justification.

## Project Structure

### Documentation

    docs/specs/20260909-220107-shot-presentation-camera/
    ├── plan.md
    ├── research.md
    ├── data-model.md
    ├── quickstart.md
    ├── contracts/shot-presentation-camera.md
    └── tasks.md                         # created later

### Source Code

    crates/azimuth-game/
    ├── src/main.rs                      # presentation state, launch capture, camera poses, tests
    ├── src/match_setup.rs               # existing controller source
    ├── src/projectile.rs                # existing observed flight source
    ├── src/tank.rs                      # existing aftermath source
    └── src/turn.rs                      # existing handoff/result source
    docs/roadmap.md                      # update only after acceptance evidence

**Structure Decision**: Keep this state and policy next to the existing BattlefieldCamera in the single application crate. There is no reusable boundary that earns a new crate or module.

## Post-design Constitution Check

**PASS.** The design uses a read-only presentation context, existing interpolation/bounds safeguards, and module-local tests. A brief presentation readiness gate can defer only a subsequent player input or autonomous AI launch while impact aftermath is visible; it never defers already-authoritative flight, impact, deformation, settling, damage, elimination, winner/draw, or turn resolution. No dependency, simulation mutation path, or generic sequencing abstraction is introduced.

## Complexity Tracking

No constitution violations or exceptional complexity are required.
