# Tasks: Arsenal Pack #1

## Phase 1: Setup and Foundation

- [X] T001 Review deployment, contact-state, and observer-boundary decisions in `docs/specs/20260910-214604-arsenal-pack-one/{plan,research,data-model}.md`
- [X] T002 Add focused coverage for bounded behavior-owned shots and ordinary-shot compatibility in `crates/azimuth-game/src/{weapon,main}.rs`
- [X] T003 Generalise MIRV-only active child ownership into the minimum explicit behavior/contact state needed by the four weapons in `crates/azimuth-game/src/weapon.rs`
- [X] T004 Refactor ordered fixed-shot advance/resolution to retain existing conventional/MIRV behavior while supporting deployment and deferred contact states in `crates/azimuth-game/src/main.rs`

## Phase 2: User Story 1 — Cluster Bomb (P1) 🎯 MVP

**Independent Test**: One descending carrier deterministically deploys a dense 8–12-child local barrage; every child uses normal wind/collision and final-child resolution.

- [X] T005 [P] [US1] Add Cluster inventory and bounded-shot coverage in `crates/azimuth-game/src/weapon.rs`
- [X] T006 [US1] Add Cluster Bomb definition, two-round loadout, HUD selection/control hint, and distinct visual identity in `crates/azimuth-game/src/{weapon,main}.rs`
- [X] T007 [US1] Implement one-time descending Cluster deployment, ordered concentrated offsets, and normal child impact handling in `crates/azimuth-game/src/main.rs`

## Phase 3: User Story 2 — Bomb Net (P1)

**Independent Test**: A carrier deterministically produces a readable 4×4 two-axis pattern whose footprint is materially wider and less dense than Cluster.

- [X] T008 [P] [US2] Add Bomb Net inventory and bounded 4×4 ownership coverage in `crates/azimuth-game/src/weapon.rs`
- [X] T009 [US2] Add Bomb Net definition, one-round selection/HUD, readable deployment visual, and broad ordered pattern generation in `crates/azimuth-game/src/{weapon,main}.rs`
- [X] T010 [US2] Reuse existing aggregate multi-projectile presentation for the larger Net pattern in `crates/azimuth-game/src/main.rs`

## Phase 4: User Story 3 — Roller (P1)

**Independent Test**: Normal flight transitions at contact into bounded current-surface rolling whose downhill, uphill, and cross-slope paths are deterministic.

- [X] T011 [P] [US3] Cover Roller inventory/contact state through the shared bounded-shot tests in `crates/azimuth-game/src/weapon.rs`
- [X] T012 [US3] Add the smallest deterministic terrain-slope sampling helper required by Roller in `crates/azimuth-game/src/battlefield.rs`
- [X] T013 [US3] Add Roller definition, inventory/selection/visual state, contact transition, fixed-step terrain following, and final normal explosion in `crates/azimuth-game/src/{weapon,main}.rs`
- [X] T014 [US3] Reuse the projectile-follow camera for Roller so it retains local slope context without owning resolution in `crates/azimuth-game/src/main.rs`

## Phase 5: User Story 4 — Bunker Buster (P1)

**Independent Test**: Normal flight contact enters finite impact-direction penetration, creates no surface blast, then produces one deeper/narrower internal result.

- [X] T015 [P] [US4] Add Bunker Buster inventory and bounded contact-state coverage in `crates/azimuth-game/src/weapon.rs`
- [X] T016 [US4] Add Bunker Buster definition, inventory/selection/visual state, bounded penetration transition, and internal normal explosion in `crates/azimuth-game/src/{weapon,main}.rs`
- [X] T017 [US4] Reuse the existing impact/deep-result camera and profile-scaled feedback in `crates/azimuth-game/src/main.rs`

## Phase 6: Polish and Completion

- [X] T018 Update Arsenal controls, lifecycle, terrain-contact behavior, and projectile documentation in `docs/projectile-model.md`
- [X] T019 Update only accepted Cluster/Net/Roller/Bunker-Buster roadmap boxes and record remaining discoveries in `docs/roadmap.md`
- [X] T020 Preserve and rerun Basic/HE/Heavy/MIRV and Basic-Shell-AI regressions in `crates/azimuth-game/src/{weapon,main,ai}.rs`
- [X] T021 Run Cargo tests, formatting, Clippy, and record the remaining manual quickstart acceptance in `docs/specs/20260910-214604-arsenal-pack-one/quickstart.md`

## Dependencies and Parallel Work

T001–T004 block every story. US1 is the MVP; US2 follows shared deployment; US3 and US4 follow the contact-state refactor and can then proceed independently. Tests T005/T008/T011/T015 are parallel opportunities because they target distinct concerns before their implementation tasks.
