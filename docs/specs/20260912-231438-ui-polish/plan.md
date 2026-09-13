# Implementation Plan: Azimuth UI Polish — A Finished Visual Language for HUD, Shop and Game Flow

**Branch**: `20260912-231438-ui-polish` | **Date**: 2026-09-12 | **Spec**: [spec.md](spec.md)

## Summary

Polish Azimuth's existing Bevy interface into one restrained arcade-artillery visual language. Add a small shared theme and reusable UI construction helpers, replace the current Unicode weapon glyphs with original transparent weapon-icon assets shared by the HUD and shop, and restyle the battlefield HUD, shop, setup, winner, and accounting flow. Retain the existing session, inventory, economy, AI, input, and transition operations as the only authorities; presentation will request those operations and render their state.

## Technical Context

**Language/Version**: Rust 2024

**Primary Dependencies**: Bevy 0.18.1 (UI, asset loading, audio, rendering)

**Storage**: In-memory session/game resources; source-controlled crate-local assets packaged by Bevy

**Testing**: `cargo test`, focused Rust unit tests, `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, plus manual graphical acceptance

**Target Platform**: Native desktop builds supported by the existing CI packaging

**Project Type**: Single Rust desktop-game workspace with one game crate

**Performance Goals**: Preserve responsive interactive play; load static UI/icon assets once and update only state-dependent UI text/styles rather than rebuilding the UI tree every frame

**Constraints**: Preserve the existing authoritative domain/session operations; no new gameplay, shop categories, generic theme system, external unprovenanced art, or heavyweight icon/animation pipeline; keep the battlefield visually dominant at 1280×720, 1920×1080, and a wider desktop layout

**Scale/Scope**: 11 playable weapon identities, 10 purchasable shop products, 2–8 player HUD entries, and the existing setup → round → winner → accounting → shop → next-round flow

## Constitution Check

| Gate | Result | Design response |
|---|---|---|
| Fun and playable progress | Pass | Work is a visible gameplay-facing polish slice, with no delay caused by framework work. |
| Code coherence over cleverness | Pass | Use a small theme and a few local UI helpers in the existing game crate; do not add a design-system crate, plugin, or generic framework. |
| Presentation must not own the game | Pass | Weapon identity remains keyed by `WeaponId`; UI reads `GameSession`, loadouts, aim, wind, and match state and invokes existing purchase/selection/advance operations only. |
| Scope and roadmap discipline | Pass | Limit change to visual/UI/audio polish; record only verified roadmap outcomes and leave armour, mechanics, and new weapons out of scope. |
| Quality and dependency gates | Pass | Prefer original project assets and existing Bevy capabilities; no new dependency is planned. Run compile, formatting, Clippy, unit tests, and manual resolution/flow checks. |

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260912-231438-ui-polish/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/
    └── ui-presentation.md
```

### Source Code

```text
crates/azimuth-game/
├── assets/
│   ├── audio/                 # Existing game audio; add restrained UI cues only if needed
│   └── ui/
│       └── weapons/           # Original transparent weapon icon assets
└── src/
    ├── main.rs                # Theme, UI construction, interaction styling, HUD and flow presentation
    ├── weapon.rs              # Engine-neutral weapon display/icon identity keyed by WeaponId
    ├── session.rs             # Existing authoritative shop/session state; no rule ownership moves here
    └── match_setup.rs         # Existing setup state consumed by the polished setup presentation
docs/roadmap.md                # Honest completion updates after implementation/manual acceptance
```

**Structure Decision**: Keep the feature in the current single game crate. The existing `main.rs` owns Bevy presentation, while `weapon.rs` retains an engine-neutral stable presentation lookup. Add only the `assets/ui/weapons` asset boundary needed by shared graphical icons; do not split crates or create an independent UI framework.

## Phase 0: Research Decisions

See [research.md](research.md). All design choices have been resolved using the current codebase and the feature constraints.

## Phase 1: Design

See [data-model.md](data-model.md) for presentation entities/state mappings, [contracts/ui-presentation.md](contracts/ui-presentation.md) for the internal UI contract, and [quickstart.md](quickstart.md) for validation.

## Post-Design Constitution Check

All gates remain passed. The design has one earned new asset directory and one small presentation metadata extension, but neither gives presentation ownership of game state or introduces a general commerce, theme, animation, or asset-pipeline abstraction. Existing deterministic/session rules and domain tests remain the authority.
