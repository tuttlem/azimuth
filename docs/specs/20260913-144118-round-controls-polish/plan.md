# Implementation Plan: Round, Controls and Battlefield Polish

**Branch**: `20260913-144118-round-controls-polish` | **Date**: 2026-09-13 | **Spec**: [spec.md](spec.md)

## Summary

Show all 2–8 accounting entries compactly, reduce nine ordinary shop prices tenfold while retaining the $15,000 Nuke, make movement unlimited across in-bounds terrain until Space or Enter ends it, use Tab for the selected setup controller, brighten the wind arrow, and blend the visual horizon into playable terrain. Existing session, turn, terrain, wind, and match configuration state remains authoritative.

## Technical Context

**Language/Version**: Rust 2024

**Primary Dependencies**: Bevy 0.18.1 and Cargo workspace

**Storage**: In-memory deterministic match/session resources; no persistence or external service

**Testing**: Module-local Rust tests, `cargo test --workspace`, `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and manual desktop validation

**Target Platform**: Existing native desktop builds supported by CI

**Project Type**: Single Rust desktop-game workspace with one game crate

**Performance Goals**: Responsive play; accounting rows spawned once and synchronised rather than rebuilt each frame

**Constraints**: 2–8 players at 1280×720 and wider views without accounting scroll; preserve combat bounds/terrain grounding; no dependency, new crate, external asset, or UI framework

**Scale/Scope**: Eight accounting rows, nine reduced-price limited weapons, retained Nuke price, two completion keys, Tab toggle, existing horizon/wind overlay

## Constitution Check

| Gate | Result | Design response |
|---|---|---|
| Fun/playability | Pass | Focused visible usability and gameplay polish. |
| Simple systems | Pass | Remove arbitrary budget/slope gate; keep fixed cardinal terrain-following steps/bounds. |
| Code coherence | Pass | Modify existing `turn`, `tank`, `weapon`, `battlefield`, and `main` paths only. |
| Deterministic/testable | Pass | Transitions, grounding, price lookup, and bounds remain pure/testable. |
| Presentation boundary | Pass | Accounting/wind/horizon read authority only; horizon cannot extend combat terrain. |
| Scope/roadmap | Pass | Excludes new weapons, broad economy, movement physics, key binding, terrain expansion, art systems. |
| Quality/dependencies | Pass | No dependency; add regressions, quality/manual checks, and verified roadmap updates. |

## Project Structure

### Documentation

```text
docs/specs/20260913-144118-round-controls-polish/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/game-ui.md
```

### Source Code

```text
crates/azimuth-game/src/
├── main.rs          # Bevy UI/input/rendering, accounting, wind, scene assembly
├── turn.rs          # Authoritative movement phase and handoff
├── tank.rs          # In-bounds terrain-following movement
├── weapon.rs        # Authoritative shop prices
├── battlefield.rs   # Authoritative terrain and render-only horizon
├── session.rs       # Players, earnings, cash, purchases
└── match_setup.rs   # Human/AI controller transition
docs/roadmap.md      # Verified completion updates only
```

**Structure Decision**: Keep work in the existing game crate. Domain changes stay in their focused modules; `main.rs` owns Bevy input/render projection. No new crate, UI layer, or asset pipeline is earned.

## Phase 0: Research Decisions

See [research.md](research.md). All choices are resolved from the current codebase.

## Phase 1: Design

See [data-model.md](data-model.md), [contracts/game-ui.md](contracts/game-ui.md), and [quickstart.md](quickstart.md).

## Post-Design Constitution Check

All gates remain passed. The design removes obsolete allowance/slope state instead of adding exceptions, retains one price source, and keeps visuals as projections around deterministic gameplay. No dependency, crate, rendering system, or UI framework is introduced.
