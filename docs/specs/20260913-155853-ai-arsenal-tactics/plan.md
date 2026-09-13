# Implementation Plan: AI Arsenal Tactics

**Branch**: `20260913-155853-ai-arsenal-tactics` | **Date**: 2026-09-13 | **Spec**: [spec.md](./spec.md)

## Summary

Add per-opponent Easy, Normal, and Hard difficulty settings; replace Basic-Shell-only AI turns with deterministic, inventory- and situation-aware decisions; and replace fixed AI shopping with bounded, difficulty-aware purchase planning. Existing session purchase and shared firing paths remain authoritative.

## Technical Context

**Language/Version**: Rust 2024 edition  
**Primary Dependencies**: Bevy 0.17 desktop runtime; existing Cargo workspace only  
**Storage**: In-memory match setup and session state  
**Testing**: `cargo test --workspace`, `cargo fmt`, and `cargo clippy`  
**Target Platform**: Existing Bevy desktop platforms  
**Project Type**: Single desktop game application crate  
**Performance Goals**: Each decision completes in one normal game update with no perceptible delay  
**Constraints**: 2–8 local participants, mixed AI levels, shared human/AI rules, reproducible decisions, no new dependencies or trajectory solver  
**Scale/Scope**: Setup configuration, pure AI policy, shop/session integration, UI guidance, and focused regression tests

## Constitution Check

| Principle / gate | Design response | Status |
|---|---|---|
| Fun over realism | Difficulty changes fallibility and judgement, not physics or hidden advantages. | Pass |
| Simple emergent systems | Decisions use current weapon roles, wind, target distance, grouping, cash, and ammunition. | Pass |
| Code coherence | Pure policy remains in the existing AI module; setup, session, and firing retain their ownership. | Pass |
| Deterministic/testable simulation | Tactical randomness uses the existing labelled seed; shopping is state-derived and pure. | Pass |
| Presentation does not own game | Setup text edits configuration only; UI calls existing authoritative action paths. | Pass |
| Playable progress / scope | Tactical movement, personality systems, and perfect ballistic search remain deferred. | Pass |
| Quality | Focused tests plus workspace format, test, lint, and build gates are required. | Pass |

**Post-design re-check**: Pass. The design adds no dependency, persistence layer, or engine-specific domain coupling.

## Project Structure

### Documentation

```text
docs/specs/20260913-155853-ai-arsenal-tactics/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/ai-behaviour.md
└── tasks.md                 # Created by $speckit-tasks
```

### Source Code

```text
crates/azimuth-game/src/
├── ai.rs              # Pure firing and shopping recommendations; direct policy tests
├── match_setup.rs     # Per-slot controller and difficulty configuration
├── session.rs         # Authoritative, atomic cash/ammunition purchase boundary
├── weapon.rs          # Existing catalogue, availability, price, and definitions
└── main.rs            # Setup input/presentation and orchestration into existing flows
```

**Structure Decision**: Keep the feature in the single existing game crate. AI owns decision policy, setup owns durable per-slot preference, session owns transactions, and `main.rs` connects these boundaries to UI and scheduling.
