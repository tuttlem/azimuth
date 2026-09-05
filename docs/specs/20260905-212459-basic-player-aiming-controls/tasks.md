# Tasks: Basic Player Aiming Controls

**Input**: Design documents from `/docs/specs/20260905-212459-basic-player-aiming-controls/`

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [player-controls.md](contracts/player-controls.md), and [quickstart.md](quickstart.md)

**Tests**: Deterministic unit tests are required by FR-012; write each before its implementation and keep renderer output out of the assertions.

## Phase 1: Setup

**Purpose**: Confirm the active feature boundary and source locations.

- [X] T001 Confirm the active branch and review accepted design artifacts plus the existing launch/tank ownership points in `crates/azimuth-game/src/main.rs`, `crates/azimuth-game/src/projectile.rs`, and `crates/azimuth-game/src/tank.rs`

---

## Phase 2: Foundational Gameplay Prerequisites

**Purpose**: Establish pure values and one shared direction/muzzle derivation for all stories.

**⚠️ CRITICAL**: Complete this phase before user-facing input, visuals, or HUD state.

- [X] T002 Add canonical reusable direction access without duplicate angle math, plus direction regression tests, in `crates/azimuth-game/src/projectile.rs`
- [X] T003 Add failing deterministic normalization, bounds, fine/coarse adjustment, and unchanged-value tests in `crates/azimuth-game/src/aiming.rs`
- [X] T004 Implement pure `AimingState`, tuneable limits, adjustments, initial construction, and current `ShotParameters` handoff in `crates/azimuth-game/src/aiming.rs`
- [X] T005 Add failing tests for level/elevated muzzle positions, canonical barrel direction, and tank-relative geometry in `crates/azimuth-game/src/tank.rs`
- [X] T006 Implement pure tank firing representation with pivot and shared visual/gameplay muzzle origin in `crates/azimuth-game/src/tank.rs`

**Checkpoint**: Current aim creates canonical shot parameters and a matching muzzle without Bevy input or presentation state.

---

## Phase 3: User Story 1 - Configure a Deliberate Shot (Priority: P1) 🎯 MVP

**Goal**: Player One views and adjusts authoritative azimuth, elevation, and velocity, while turret, barrel, muzzle, and HUD stay in agreement.

**Independent Test**: Run Q/E, R/F, and T/G with and without Shift; verify wrapping/bounds, values, Player One geometry, and HUD change together.

### Tests for User Story 1

- [X] T007 [US1] Add deterministic input-to-adjustment selection tests for Q/E, R/F, T/G, Shift coarse changes, and opposing-key cancellation in `crates/azimuth-game/src/main.rs`

### Implementation for User Story 1

- [X] T008 [US1] Register `aiming` and insert Player One's authoritative initial aiming resource derived from its existing turret azimuth in `crates/azimuth-game/src/main.rs`
- [X] T009 [US1] Tag Player One's turret, barrel, and muzzle marker and synchronize their transforms from derived firing representation in `crates/azimuth-game/src/main.rs`
- [X] T010 [US1] Implement idle-only Q/E, R/F, T/G, and Shift input handling that applies one net adjustment without altering camera controls in `crates/azimuth-game/src/main.rs`
- [X] T011 [US1] Spawn/update one corner `AimingHud` text entity showing Player One, current values/units, controls, and ready status in `crates/azimuth-game/src/main.rs`

**Checkpoint**: User Story 1 independently delivers visible, bounded fine/coarse shot configuration without renderer-owned aiming state.

---

## Phase 4: User Story 2 - Fire the Selected Shot (Priority: P2)

**Goal**: Space fires exactly the displayed current shot from the visible muzzle through existing projectile, impact, boom, and deformation behavior.

**Independent Test**: For known settings, projectile position equals derived muzzle and velocity is current aim; vary each input and observe expected initial conditions.

### Tests for User Story 2

- [X] T012 [US2] Add deterministic tests for identical current-aim launches, azimuth/elevation/speed effects, and rejection of stale development values in `crates/azimuth-game/src/main.rs`

### Implementation for User Story 2

- [X] T013 [US2] Replace fixed Space and I development launch selection with one Space path using current aim and shared muzzle origin in `crates/azimuth-game/src/main.rs`
- [X] T014 [US2] Preserve projectile visual, fixed-step advancement, terrain impact, boom, marker, crater, and terrain-mesh synchronization after the new handoff in `crates/azimuth-game/src/main.rs`

**Checkpoint**: A visibly configured shot fires from the shown barrel end and retains all downstream impact feedback.

---

## Phase 5: User Story 3 - Bracket a Target With Repeated Shots (Priority: P3)

**Goal**: Active flight locks aiming/fire; after resolution, retained state permits corrections and repeated shots over deformed terrain.

**Independent Test**: Fire, try controls during flight, verify no change/second projectile, then adjust and fire again into the changed battlefield.

### Tests for User Story 3

- [X] T015 [US3] Add deterministic flight-lock tests for ignored aim/fire with an active projectile and retained state after terrain-impact/out-of-bounds resolution in `crates/azimuth-game/src/main.rs`

### Implementation for User Story 3

- [X] T016 [US3] Gate aim and fire systems on `ProjectileFlight`, retain aim across resolution, and show locked/ready HUD status in `crates/azimuth-game/src/main.rs`
- [X] T017 [US3] Keep `advance_projectile` free of aiming-state resets so later launches retain current aim and use the existing mutable terrain after the input lock releases in `crates/azimuth-game/src/main.rs`

**Checkpoint**: Player One can aim, fire, observe deformation, correct, and fire again without turn switch or overlapping flight.

---

## Phase 6: Documentation, Roadmap, and Quality

**Purpose**: Make controls discoverable, record only demonstrated roadmap progress, and keep the workspace healthy.

- [X] T018 [P] Replace development controls with player aiming controls, values, flight lock, and active-tank choice in `README.md`
- [X] T019 [P] Document pitch-aware barrel-end firing origin and shared canonical direction in `docs/world-conventions.md`
- [X] T020 [P] Replace fixed development/inspection shot text with authoritative aiming, fire, and repeated-shot model in `docs/projectile-model.md`
- [X] T021 Update only demonstrated aiming, feedback/player-skill, aiming-HUD, keyboard-control, clear-feedback, and readable-value checkboxes in `docs/roadmap.md`
- [X] T022 Run manual scenarios in `docs/specs/20260905-212459-basic-player-aiming-controls/quickstart.md` and record nonessential discoveries in `docs/roadmap.md`
- [X] T023 Run `cargo check --workspace --all-targets`, `cargo test --workspace`, `cargo fmt --all -- --check`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings` from the workspace root defined by `Cargo.toml`

---

## Dependencies & Execution Order

- **Phase 1** starts immediately.
- **Phase 2** depends on T001 and blocks all story implementation.
- **US1** depends on T002–T006 and independently demonstrates controlled aim, visible direction, and feedback.
- **US2** depends on US1 because firing consumes its authoritative state and visible muzzle.
- **US3** depends on US2 because it gates the new fire path and validates repeated shots.
- **Phase 6** depends on all stories; T018–T020 can run in parallel after behavior stabilizes, then T021–T023 follow verification.

## Parallel Opportunities

- T018, T019, and T020 modify separate documentation files and can proceed in parallel after behavior stabilizes.
- There are intentionally no earlier `[P]` code tasks: compact module boundaries have direct data dependencies, and avoiding `main.rs` conflicts is clearer than speculative parallelism.

## Parallel Example: Documentation

```text
Task: "T018 update player controls in README.md"
Task: "T019 update firing-origin convention in docs/world-conventions.md"
Task: "T020 update aiming launch model in docs/projectile-model.md"
```

## Implementation Strategy

### MVP First

1. Complete T001–T006 for pure aim/muzzle invariants.
2. Complete T007–T011 and run the US1 independent test.
3. Stop here for the first playable aiming configuration slice.

### Incremental Delivery

1. Add US1: configure and inspect the next shot.
2. Add US2: replace development launch with current aim and verify physical effects.
3. Add US3: lock input during flight and validate repeated bracketing over deformation.
4. Finish documentation, roadmap accuracy, quickstart, and quality gates.
