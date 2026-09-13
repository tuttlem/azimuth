# Tasks: Environment Presets

**Input**: Design documents in `docs/specs/20260913-170556-environment-presets/`

## Phase 1: Setup

- [X] T001 Verify current world-generation, setup, wind, terrain, and presentation boundaries in `crates/azimuth-game/src/main.rs` and `crates/azimuth-game/src/battlefield.rs`

## Phase 2: Foundational Environment Model

- [X] T002 Create deterministic preset, wind-policy, terrain-profile, and presentation-profile definitions with unit tests in `crates/azimuth-game/src/environment.rs`
- [X] T003 Add retained selected-environment configuration, default Earth, cycling, labels, and tests in `crates/azimuth-game/src/match_setup.rs`

## Phase 3: User Story 1 - Choose a Recognisable Battlefield World (P1) 🎯 MVP

**Independent Test**: Cycle all six preset names/descriptions in setup and start/restart a match retaining the selection.

- [X] T004 [P] [US1] Add setup selection and retention regression coverage in `crates/azimuth-game/src/main.rs`
- [X] T005 [US1] Add setup keyboard control, selected-world summary, and guidance in `crates/azimuth-game/src/main.rs`
- [X] T006 [US1] Thread selected environment through deterministic match and round creation in `crates/azimuth-game/src/main.rs`

## Phase 4: User Story 2 - Play Distinct Artillery Worlds (P1)

**Independent Test**: Compare identical shots and starts across all six worlds, including 20 deterministic Turnwind handoffs.

- [X] T007 [P] [US2] Add gravity, wind-policy, Turnwind timing, and deterministic world-generation tests in `crates/azimuth-game/src/environment.rs`
- [X] T008 [P] [US2] Add Bowl profile and valid 2–8 player start-invariant tests in `crates/azimuth-game/src/battlefield.rs`
- [X] T009 [US2] Apply preset gravity, stable/rerolled wind policy, and Turnwind turn-boundary updates in `crates/azimuth-game/src/main.rs`
- [X] T010 [US2] Implement deterministic Bowl terrain profile while retaining authoritative terrain/start rules in `crates/azimuth-game/src/battlefield.rs`

## Phase 5: User Story 3 - Recognise Each World at a Glance (P2)

**Independent Test**: Inspect each world from normal tactical views and identify it while tanks, shots, explosions, and HUD remain readable.

- [X] T011 [P] [US3] Add palette/sky-profile configuration tests for all six presets in `crates/azimuth-game/src/environment.rs`
- [X] T012 [US3] Make battlefield mesh and horizon palettes consume Earth, Moon, Storm, and Crusher presentation profiles in `crates/azimuth-game/src/battlefield.rs`
- [X] T013 [US3] Implement preset sky, cloud, and deterministic Moon-star presentation in `crates/azimuth-game/src/main.rs`
- [X] T014 [US3] Preserve HUD/tank/projectile/explosion contrast under every presentation profile in `crates/azimuth-game/src/main.rs`

## Phase 6: Polish & Validation

- [X] T015 Update verified environment/setup/Milestone F items in `docs/roadmap.md`
- [X] T016 Update automated and manual environment validation evidence in `docs/specs/20260913-170556-environment-presets/quickstart.md`
- [X] T017 Run format, workspace tests, Clippy warnings-denied, and workspace build from `Cargo.toml`
- [ ] T018 Perform and record manual visual/playability checks for all six presets in `docs/specs/20260913-170556-environment-presets/quickstart.md`

## Dependencies & Execution Order

`T001 → T002/T003 → US1 → US2 → US3 → validation`. T007/T008 and T011 may run in parallel once the environment model exists. US3 depends on selected presentation state but not on Bowl implementation.

## Implementation Strategy

MVP is Earth plus selectable/retained preset definitions and setup presentation. Then add physics/terrain worlds, followed by visual identity. Every task follows the required checkbox, ID, story label, and file-path format.
