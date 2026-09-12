# Implementation Plan: Arsenal and Economy Cleanup

**Branch**: `20260912-141531-arsenal-economy-cleanup` | **Date**: 2026-09-12 | **Spec**: [spec.md](./spec.md)

## Summary

Remove Bomb Net, redefine Bunker Buster as massive zero-damage terrain carving using existing deformation, and replace old bonuses with actual opponent health removed at $10 plus deterministic podium awards.

## Technical Context

**Language/Version**: Rust edition 2024, stable toolchain

**Primary Dependencies**: Bevy 0.18.1 and existing weapon, terrain, tank, turn, session, AI, and presentation modules

**Storage**: In-memory match/session state

**Testing**: `cargo test --workspace`, formatter, and warning-denied Clippy

**Target Platform**: Existing native desktop targets

**Project Type**: Single-crate desktop game

**Constraints**: No new terrain engine, migration, replacement weapon, fall damage, team scoring, or broad economy rebalance

## Constitution Check

*Pre-research and post-design gate: PASS.* Existing terrain/settling, deterministic player order, focused removal, bounded scope, documentation, and tests satisfy the applicable constitution principles.

## Project Structure

Implementation modifies existing `crates/azimuth-game/src/weapon.rs`, `session.rs`, `main.rs`, `ai.rs`, `battlefield.rs`, `combat.rs`, `tank.rs`, and `turn.rs`, plus `README.md` and `docs/roadmap.md`.

**Structure Decision**: Reuse current weapon, terrain, tank, turn, and session boundaries; add only minimal placement/accounting state.

## Implementation Approach

1. Remove Bomb Net catalogue/loadout/price/child-pattern/UI/AI paths while retaining generic multi-projectile support.
2. Apply zero health damage and a much larger bounded deep/directional terrain outcome for Bunker Buster, then use existing settlement.
3. Credit actual opponent health deltas and replace elimination/winner rewards with placement income.
4. Derive podium positions from survivor/elimination state in configured-player order; award only $5,000/$2,500/$1,000 and no draw winner.
5. Update UI, tests, README, roadmap, and workspace quality.

## Complexity Tracking

Not applicable.
