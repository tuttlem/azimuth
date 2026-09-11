# Implementation Plan: Rounds, Economy and Weapon Shop — The Continuing Game Loop

**Branch**: 20260911-222730-rounds-economy-shop | **Date**: 2026-09-11 | **Spec**: [spec.md](spec.md)

## Summary

Turn the current artillery match into a disposable round inside an in-memory continuing session. A small authoritative session domain owns cash, finite loadouts, round number, wins, accounting, prices, validated purchases, and bounded deterministic AI shopping. Existing combat resources remain round-owned. Result, accounting, and sequential shop overlays use one phase resource, while the existing setup rebuild seam creates fresh rounds.

## Technical Context

**Language/Version**: Rust edition 2024  
**Primary Dependencies**: Bevy 0.18.1  
**Storage**: In-memory only  
**Testing**: cargo test --workspace, formatting, Clippy  
**Target Platform**: Local desktop graphical game  
**Project Type**: Cargo-workspace game application  
**Performance Goals**: Responsive turn-based play; bounded immediate AI shopping; no stale round entities  
**Constraints**: 2–8 Human/AI players, integer cash, deterministic gameplay streams, UI never owns accounting  
**Scale/Scope**: One focused session module, round-flow overlays, all existing weapons

## Constitution Check

- Fun and playable progress: PASS.
- Deterministic and testable simulation: PASS; applied-damage accounting and isolated streams are pure-testable.
- Presentation must not own game: PASS; UI projects authoritative state.
- Coherent boundaries: PASS; session values are not tank fields.
- Scope/dependencies: PASS; no dependency, persistence, general store, or rendering work.
- Roadmap discipline: PASS; only genuinely completed work will be checked.

## Project Structure

docs/specs/20260911-222730-rounds-economy-shop contains plan.md, research.md, data-model.md, quickstart.md, contracts/game-flow.md, and later tasks.md.

Source changes stay in crates/azimuth-game/src: main.rs integrates phases, overlays, and round rebuild; session.rs owns session/economy logic; combat.rs reports applied damage; weapon.rs exposes catalog and validated ammunition additions.

**Structure Decision**: Retain the existing crate and add only one focused domain module.

## Complexity Tracking

No constitution violations require justification.

