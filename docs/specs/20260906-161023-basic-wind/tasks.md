---

description: "Dependency-ordered implementation tasks for Basic Wind"
---

# Tasks: Basic Wind — First Environmental Gameplay

**Input**: Design documents from `/docs/specs/20260906-161023-basic-wind/`

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md),
[data-model.md](data-model.md), [wind-display contract](contracts/wind-display.md), and
[quickstart.md](quickstart.md)

**Tests**: Required. Wind changes authoritative projectile simulation; write the listed tests
first and confirm they fail before implementation tasks.

**Organization**: The existing game crate and fixed projectile path are the shared foundation.
Tasks are grouped by independently testable player stories; no new project setup or dependency is
needed.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Different-file work with no incomplete-task dependency.
- **[US#]**: User-story traceability label; omitted for shared foundation and final polish.

## Phase 1: Setup

**Purpose**: No initialization task is required. The existing Rust workspace, fixed-step test
harness, single HUD text, and documented quality commands are ready for this feature.

---

## Phase 2: Foundational (Blocking Prerequisite)

**Purpose**: Establish one validated, engine-independent wind value and ensure the projectile has
one authoritative fixed-step path that can receive it.

- [X] T001 Add failing domain tests for finite horizontal wind validation, zero wind, reverse
  direction, magnitude, no vertical contribution, duration accumulation, and deterministic traces
  in `crates/azimuth-game/src/projectile.rs`.
- [X] T002 Implement the compact validated horizontal wind value, its strength/accessor behavior,
  and combined gravity-plus-wind fixed-step projectile acceleration in
  `crates/azimuth-game/src/projectile.rs`; update every existing projectile call site to use this
  one path.

**Checkpoint**: Domain simulation accepts one explicit horizontal wind condition and zero wind
retains existing ballistic behavior before any Bevy resource or HUD integration.

---

## Phase 3: User Story 1 - Read Wind and Learn From a Shot (Priority: P1) 🎯 MVP

**Goal**: Both players see a compact camera-independent wind line and can observe a predictable
default +X drift to inform a later manual correction.

**Independent Test**: Format the in-progress HUD with the default wind and simulate a fixed
crosswind trace; verify it says `toward +X, 1.5 units/s²` and moves only toward +X versus zero
wind.

### Tests for User Story 1

- [X] T003 [US1] Add failing HUD-formatting tests in `crates/azimuth-game/src/main.rs` for the
  documented default wind line in choosing, moving, and resolving-fire states, including its
  camera-independent `toward` wording.
- [X] T004 [US1] Add failing fixed-step crosswind/reversal regression coverage in
  `crates/azimuth-game/src/projectile.rs` proving the default +X condition visibly separates from
  zero wind and later against-wind launch changes the expected horizontal result without aim
  mutation.

### Implementation for User Story 1

- [X] T005 [US1] Add the one constant default battlefield-wind resource and pass it only to the
  fixed projectile advance in `crates/azimuth-game/src/main.rs`; keep it constant across turn
  handoff and exclude it from tank settling.
- [X] T006 [US1] Extend the existing HUD synchronizer/formatter in
  `crates/azimuth-game/src/main.rs` to render `Wind: toward +X, 1.5 units/s²` from the same
  authoritative resource during every in-progress action state, with no aim assist or HUD redesign.

**Checkpoint**: A standard local duel presents the same readable wind to both players, and default
wind produces a learnable +X projectile drift.

---

## Phase 4: User Story 2 - Preserve a Trustworthy Artillery Simulation (Priority: P1)

**Goal**: Wind-altered projectile segments continue to resolve one deterministic terrain impact
and all existing downstream duel consequences at that actual result.

**Independent Test**: Simulate a known wind-altered terrain crossing and pass its resolved impact
through the current gameplay resolver; verify one normal damage/crater/support/settling/handoff
flow and equal inputs produce equal outcomes.

### Tests for User Story 2

- [X] T007 [US2] Add failing wind-altered terrain-collision tests in
  `crates/azimuth-game/src/projectile.rs` for collision at the current terrain height and a
  changed X/Z impact compared with zero wind.
- [X] T008 [US2] Add failing gameplay-resolution regression tests in
  `crates/azimuth-game/src/main.rs` that use a wind-derived impact and verify exactly-once damage,
  crater deformation, living-tank settling/turn completion, and unchanged out-of-bounds behavior.

### Implementation for User Story 2

- [X] T009 [US2] Thread the authoritative battlefield wind through `advance_projectile` in
  `crates/azimuth-game/src/main.rs` into the existing projectile simulation without changing
  `resolve_projectile_advance`, combat, crater, tank-settling, or turn APIs beyond their actual
  wind input needs.
- [X] T010 [US2] Update fixed-step and integration assertions in
  `crates/azimuth-game/src/projectile.rs` and `crates/azimuth-game/src/main.rs` to preserve the
  zero-wind baseline and deterministic resulting-impact pipeline.

**Checkpoint**: Wind changes where a shot lands, never how the existing impact consequences are
resolved or when presentation is allowed to advance a turn.

---

## Phase 5: User Story 3 - Make Wind Matter Without Becoming a New Vehicle System (Priority: P2)

**Goal**: Longer/crosswind/parallel shots demonstrate understandable exposure-based drift, while
tanks, movement, settling, explosions, and camera behavior remain wind-free.

**Independent Test**: Compare short/long, parallel/opposing, and perpendicular fixed traces, then
assert tank support/movement state is unchanged by the wind condition.

### Tests for User Story 3

- [X] T011 [US3] Add fixed-trace tests in `crates/azimuth-game/src/projectile.rs` proving longer
  exposure accumulates more displacement, crosswind causes lateral drift, and parallel/opposing
  wind respectively increases/decreases downrange displacement.
- [X] T012 [US3] Add non-interaction regression coverage in
  `crates/azimuth-game/src/tank.rs` and `crates/azimuth-game/src/main.rs` proving wind is absent
  from deliberate movement and gravity-only support/settling while camera timing remains outside
  authoritative projectile results.

### Implementation for User Story 3

- [X] T013 [US3] Verify and preserve the gravity-only tank-settling call path in
  `crates/azimuth-game/src/main.rs` and the existing tank APIs in
  `crates/azimuth-game/src/tank.rs`; add no wind force, movement rule, or camera behavior.

**Checkpoint**: Wind is a projectile-only skill variable whose extra influence grows with airtime
without turning movement or settling into environmental physics.

---

## Phase 6: Polish and Cross-Cutting Validation

**Purpose**: Document the authoritative/player-facing convention, align only earned roadmap items,
and confirm a complete windy duel remains healthy.

- [X] T014 [P] Update fixed-step wind math, zero-wind behavior, collision behavior, and feature
  boundaries in `docs/projectile-model.md`.
- [X] T015 [P] Update wind world-axis/toward convention and player compensation guidance in
  `docs/world-conventions.md`, plus the visible default and controls/resolution summary in
  `README.md`.
- [X] T016 Update only validated Basic Wind, aiming-feedback/HUD, and possibly Milestone F entries
  in `docs/roadmap.md`; explicitly leave vertical wind and unproven balance/preset work unchecked.
- [X] T017 Run the automated and manual validation in
  `docs/specs/20260906-161023-basic-wind/quickstart.md`: `cargo check --workspace --all-targets`,
  `cargo test --workspace`, `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets
  --all-features -- -D warnings`, then a readable compensated-shot local duel.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1**: No work required; existing project setup is ready.
- **Phase 2**: T001 precedes T002 and blocks all gameplay work by defining the sole wind input.
- **Phase 3 / US1**: T003–T004 precede T005–T006 and depend on T002.
- **Phase 4 / US2**: T007–T008 precede T009–T010 and depend on the same projectile wind path from
  T002; its Bevy wiring can follow T005.
- **Phase 5 / US3**: T011–T013 depend on the core projectile and app integration from US1/US2.
- **Phase 6**: T014–T016 follow delivered behavior. T017 follows all implementation/docs work.

### User Story Dependencies

- **US1 (P1)**: Depends on the foundational wind value and independently proves readable
  direction/strength plus a predictable first drift.
- **US2 (P1)**: Depends on the same wind model and independently proves existing terrain/duel
  resolution still owns wind-altered impact consequences.
- **US3 (P2)**: Depends on US1/US2 and independently verifies the desired skill expression plus
  explicit non-interaction with tanks/presentation.

### Parallel Opportunities

- T014 and T015 can run in parallel after behavior is final because they modify distinct
  documentation files.
- Before implementation, test authoring can be split between `projectile.rs` (T001, T004, T007,
  T011) and `main.rs` (T003, T008) but each source file must be integrated in sequence.

## Parallel Example: Documentation

```text
Task: "Update wind math and projectile boundary docs in docs/projectile-model.md"
Task: "Update world convention and player-facing summary in docs/world-conventions.md and README.md"
```

## Implementation Strategy

### MVP First (US1)

1. Complete T001–T002 to establish the pure wind model.
2. Write T003–T004, then complete T005–T006.
3. Run focused projectile/HUD tests and manually confirm the default wind line and visible drift.

### Incremental Delivery

1. US1 adds readable, constant environmental aiming information and deterministic drift.
2. US2 demonstrates that the altered path retains one stable impact/duel-resolution pipeline.
3. US3 verifies airtime-based skill expression and guards the non-projectile boundary.
4. Polish documents the convention, updates earned roadmap entries, and validates a complete duel.

## Notes

- Every task has a sequential checklist ID, concrete path, and user-story label where required.
- Do not add drag, vertical wind, changing/random weather, tank force, camera control, trajectory
  prediction, or automatic compensation while completing this list.
- Wind-strength and compensation evaluation roadmap items require actual manual duel evidence;
  a default constant and automated trajectory tests alone do not satisfy them.
