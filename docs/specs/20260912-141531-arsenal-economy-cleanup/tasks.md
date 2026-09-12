# Tasks: Arsenal and Economy Cleanup

**Input**: Design documents in this feature directory.

## Phase 1: Setup

- [X] T001 Review all Bomb Net, Bunker Buster, and earnings call sites in `crates/azimuth-game/src/weapon.rs`, `session.rs`, and `main.rs` before editing.

## Phase 2: Foundational

- [X] T002 Add deterministic placement/actual-health-delta test helpers beside current accounting tests in `crates/azimuth-game/src/session.rs`.

## Phase 3: User Story 1 - Focused Arsenal (P1)

**Goal**: Bomb Net is genuinely absent while Cluster Bomb remains.

**Independent Test**: Catalogue, loadout, price, shop/UI, and AI tests cannot find/select/purchase Bomb Net.

- [X] T003 [US1] Add removal regression coverage for catalogue, loadout, price, UI, and AI paths in `crates/azimuth-game/src/weapon.rs`, `ai.rs`, and `main.rs`.
- [X] T004 [US1] Remove Bomb Net ID, definition, ammunition, price, inventory, child pattern, selection, HUD/shop, AI, and tests from `crates/azimuth-game/src/weapon.rs`, `ai.rs`, and `main.rs` while retaining shared multi-projectile support.

## Phase 4: User Story 2 - Terrain-only Bunker Buster (P1)

**Goal**: Bunker Buster performs massive directional landscaping and zero health damage.

**Independent Test**: Direct, nearby, multi-tank, and self-impact cases preserve health while terrain is deeper/larger, stable, and collidable.

- [X] T005 [US2] Add Bunker Buster zero-damage, terrain-volume/depth, deterministic, settling, and later-collision regression tests in `crates/azimuth-game/src/main.rs`, `battlefield.rs`, `combat.rs`, and `tank.rs`.
- [X] T006 [US2] Redefine Bunker Buster price, impact behavior, and bounded deep/directional excavation profile in `crates/azimuth-game/src/weapon.rs`.
- [X] T007 [US2] Route Bunker Buster resolution through terrain-only damage handling and terrain-first presentation/camera behavior in `crates/azimuth-game/src/main.rs`.

## Phase 5: User Story 3 - Simple Earnings (P1)

**Goal**: Earnings equal actual opponent health removed × $10 plus podium award only.

**Independent Test**: Overkill, self-damage, multi-target, Bunker Buster, two-player, multi-player, simultaneous-elimination, and draw tests exactly match the equation.

- [X] T008 [US3] Add accounting tests for actual health deltas, $10/HP, overkill, self/terrain zero income, multi-target totals, podium awards, simultaneous elimination, and draws in `crates/azimuth-game/src/session.rs` and `turn.rs`.
- [X] T009 [US3] Replace elimination/winner reward fields and logic with damage and placement income in `crates/azimuth-game/src/session.rs`.
- [X] T010 [US3] Record deterministic elimination/placement order from authoritative round resolution in `crates/azimuth-game/src/main.rs` and `turn.rs`.
- [X] T011 [US3] Update round-accounting UI and wallet reconciliation in `crates/azimuth-game/src/main.rs` to show damage income, placement, and total only.

## Phase 6: Polish

- [X] T012 [P] Update active-arsenal and economy wording in `README.md`.
- [X] T013 [P] Update Bomb Net, Bunker Buster, and reward descriptions in `docs/roadmap.md`.
- [X] T014 Run formatter, Clippy, tests, check, and build from repository root; perform quickstart manual acceptance from `docs/specs/20260912-141531-arsenal-economy-cleanup/quickstart.md`.

## Dependencies & Execution Order

T001–T002 → US1 (T003–T004) and US2 (T005–T007) → US3 (T008–T011) → T012–T014. US1 and US2 may proceed in parallel after T002; US3 depends on Bunker Buster’s zero-damage authority.

## Implementation Strategy

Implement and validate Bomb Net removal first, then the terrain-only Bunker Buster, then consolidate accounting and placement. Finish with documentation and full workspace quality.
