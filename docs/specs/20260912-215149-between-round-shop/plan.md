# Implementation Plan: Between-Round Shop — Weapon Purchasing and Visual Inventory

**Branch**: `20260912-215149-between-round-shop` | **Date**: 2026-09-12 | **Spec**: [spec.md](./spec.md)

## Summary

Complete the existing placeholder Shop phase as a session-owned sequential purchasing flow. Reuse the authoritative weapon prices, loadouts, and atomic session purchase operation; introduce a compact Shop Item/category seam for future Armour; add deterministic bounded AI shopping; and use one shared presentation lookup to draw distinct in-project weapon icons in both the shop cards and clickable battle strip.

## Technical Context

**Language/Version**: Rust edition 2024, stable toolchain

**Primary Dependencies**: Bevy 0.18.1 and existing session, match setup, weapon, AI, and UI systems

**Storage**: In-memory `GameSession` player cash and `PlayerWeaponLoadout`; no persistence

**Testing**: `cargo test --workspace`, formatter, warning-denied Clippy, workspace check, build, and manual local session acceptance

**Target Platform**: Existing native desktop targets

**Project Type**: Single-crate desktop game

**Performance Goals**: Shop and weapon inventory remain immediately responsive in a 2–8 player local match; current 11-card arsenal fits the supported window without required controls overlapping

**Constraints**: No armour behavior, persistence, pre-round shop, resale, dynamic prices, generic commerce framework, external icon collection, or change to current economy rules

**Scale/Scope**: One session-owned shop phase, 10 limited weapon products, 11 shared weapon visual identities, and bounded AI purchase behavior for 2–8 participants

## Constitution Check

*Pre-research gate: PASS.* The plan extends an already-existing session seam rather than adding infrastructure, keeps gameplay state independent of presentation, maintains deterministic/testable AI behavior, uses no new dependency, and delivers a visible playable loop. The Shop Item/category seam is warranted by the stated future Armour category and remains intentionally small.

*Post-design gate: PASS.* The selected design has one authoritative purchase boundary, one session-flow transition authority, and a pure presentation lookup. It avoids a generic inventory/ecommerce hierarchy, external asset pipeline, and persistence while preserving the constitution's scope and quality gates.

## Project Structure

### Documentation

```text
docs/specs/20260912-215149-between-round-shop/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── shop-ui.md
└── tasks.md
```

### Source Code

```text
crates/azimuth-game/src/
├── session.rs       # Session phase, shop turn state, items, atomic purchases
├── weapon.rs        # Weapon catalogue, ammunition and shared presentation identity
├── ai.rs            # Bounded deterministic AI shop decision
├── main.rs          # Shop overlay/cards, input dispatch, strip visuals, session transition
└── match_setup.rs   # Existing player controller identity consumed by shop flow

docs/roadmap.md      # Economy/shop and future Armour roadmap status
```

**Structure Decision**: Keep the feature in the existing game modules. `session.rs` owns shop eligibility and transaction authority; `weapon.rs` remains the discoverable weapon catalogue and supplies presentation-neutral identity; `main.rs` projects that state into the shop and HUD; `ai.rs` supplies only the bounded automated pass. No new crate, asset pipeline, or generic inventory module is necessary.
