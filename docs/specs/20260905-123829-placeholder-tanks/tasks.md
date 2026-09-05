---

description: "Implementation tasks for Placeholder Tanks and Player Entities"
---

# Tasks: Placeholder Tanks and Player Entities

**Input**: Design documents from `docs/specs/20260905-123829-placeholder-tanks/`

**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md),
[data-model.md](./data-model.md), and [quickstart.md](./quickstart.md)

**Tests**: The specification explicitly requires focused deterministic tests for terrain placement,
spawn bounds, distinct players, and firing origins. Keep renderer behaviour under manual validation.

**Organization**: Shared terrain and concrete tank-domain facts are foundational because all user
stories require them. Rendering, domain inspection, and camera compatibility then remain separately
verifiable slices.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel because it changes a different file and has no unfinished dependency.
- **[Story]**: Identifies the user story served by the task.

## Phase 1: Setup

**Purpose**: Preserve the existing intentionally small workspace and dependency set.

- [X] T001 Confirm `crates/azimuth-game/Cargo.toml` needs no dependency or workspace-structure change for the existing Bevy implementation.

---

## Phase 2: Foundational Domain and Terrain Facts

**Purpose**: Create the minimal engine-independent facts required by all three user stories.

- [X] T002 Extract the battlefield extent, deterministic terrain-height query, and horizontal bounds check with focused tests into `crates/azimuth-game/src/battlefield.rs`.
- [X] T003 Add concrete player identity, position, horizontal-direction, pose, tank, deterministic initial-spawn, and derived firing-origin tests to `crates/azimuth-game/src/tank.rs`.
- [X] T004 Wire the new battlefield and tank modules into the existing application while preserving the current terrain mesh, camera, and origin axes in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: Two fixed in-bounds player tanks, terrain-resolved positions, and non-stale firing
origins can be tested without renderer state.

---

## Phase 3: User Story 1 - See the Beginnings of an Artillery Duel (Priority: P1) 🎯 MVP

**Goal**: The running battlefield contains two readable, distinct, terrain-grounded placeholder
tanks in separated opposing positions.

**Independent Test**: Run `cargo run --package azimuth-game`; orbit and pan to confirm exactly two
distinct tanks are visible, separated, in bounds, and grounded on terrain.

### Implementation for User Story 1

- [X] T005 [US1] Reuse simple primitive meshes and create distinct player materials for two placeholder tank silhouettes in `crates/azimuth-game/src/main.rs`.
- [X] T006 [US1] Spawn one terrain-resolved parent tank with body, turret, and forward barrel child geometry for each initial tank in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: The visual MVP is complete without projectiles, aiming, damage, movement, turns, or
external assets.

---

## Phase 4: User Story 2 - Use Clear Player and Firing References (Priority: P2)

**Goal**: Player identity, tank pose, and a firing origin are clear concrete domain facts that later
projectile work can consume without inspecting rendering entities.

**Independent Test**: Run the unit tests and review `tank.rs` to confirm unique owners, in-bounds
terrain-resolved positions, and firing origins that remain ahead of and above turret direction.

### Implementation for User Story 2

- [X] T007 [US2] Verify the rendered body and barrel transforms are derived from the corresponding domain body and turret directions in `crates/azimuth-game/src/main.rs`.
- [X] T008 [US2] Add or refine focused spawn, terrain-placement, and firing-origin assertions in `crates/azimuth-game/src/tank.rs` so they cover the fixed initial tanks rather than renderer internals.

**Checkpoint**: The next projectile feature has explicit owner, pose, direction, and firing-origin
inputs without a weapon system or aiming rules.

---

## Phase 5: User Story 3 - Inspect Tanks Without Losing Battlefield Context (Priority: P3)

**Goal**: The original camera and world-orientation aids remain useful for inspecting the new tanks,
without automatic camera behaviour or gameplay UI.

**Independent Test**: Run Azimuth, use the documented orbit/pan/zoom controls from multiple
viewpoints, and confirm terrain relief, origin axes, and both tanks remain inspectable without
automatic focus or tracking.

### Implementation for User Story 3

- [X] T009 [US3] Preserve the existing direct camera controls and origin-axis update while integrating the tank scene in `crates/azimuth-game/src/main.rs`.
- [X] T010 [US3] Update the scene status and tank-visibility expectations without adding player-selection or camera controls in `README.md`.

**Checkpoint**: The development scene remains a freely inspectable battlefield rather than a
camera- or UI-driven gameplay system.

---

## Phase 6: Polish and Cross-Cutting Validation

**Purpose**: Verify the complete feature, accurately record fulfilled roadmap work, and leave the
repository healthy.

- [X] T011 Update only the fulfilled near-term, placeholder-tank, player-position/orientation, firing-origin, terrain-placement, and player-identity checkboxes in `docs/roadmap.md`.
- [X] T012 Run workspace build check, tests, formatting validation, and Clippy with warnings denied using `docs/specs/20260905-123829-placeholder-tanks/quickstart.md`.
- [X] T013 Run the manual terrain, tank distinction, orientation, camera, debug-axis, and clean-close scenarios in `docs/specs/20260905-123829-placeholder-tanks/quickstart.md` and record completion in `docs/specs/20260905-123829-placeholder-tanks/tasks.md`.

---

## Dependencies and Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Starts immediately.
- **Foundational (Phase 2)**: T002 and T003 create the terrain/tank domain facts; T004 integrates
  them and blocks scene work.
- **US1 (Phase 3)**: Depends on T004; T006 depends on T005's primitive/material setup.
- **US2 (Phase 4)**: Depends on T003 and T006 because it verifies both the domain handoff and its
  direct presentation mapping.
- **US3 (Phase 5)**: Depends on T006 because it verifies the complete scene remains inspectable.
- **Polish (Phase 6)**: Depends on all user-story work. Update the roadmap only after automated
  and manual acceptance criteria have passed.

### User Story Dependencies

- **US1 (P1)**: Requires the shared terrain and tank-domain foundation only.
- **US2 (P2)**: Builds on the same foundation and US1's visible mapping; it introduces no weapon or
  projectile behaviour.
- **US3 (P3)**: Verifies compatibility with the already completed scene and does not introduce new
  camera behaviour.

## Parallel Opportunities

- After T001, T002 (`battlefield.rs`) and the initial test scaffolding for T003 (`tank.rs`) can be
  prepared in parallel, but T003's final placement tests depend on the terrain facts from T002.
- After T006, T008 (`tank.rs` tests) and T010 (`README.md`) can proceed in parallel because they
  affect different files.
- T012 automated checks and the desktop portion of T013 can start independently after all code and
  documentation is complete, although both results are required before feature completion.

### Parallel Example: After Tank Rendering

```text
Task: "Refine fixed-spawn and firing-origin assertions in crates/azimuth-game/src/tank.rs"
Task: "Update scene status and tank-visibility expectations in README.md"
```

## Implementation Strategy

### MVP First

1. Complete T001–T004 to establish and integrate the small domain boundary.
2. Complete T005–T006 to render two grounded opposing tank placeholders.
3. Run the US1 independent test before adding any other work.

### Incremental Delivery

1. Complete US2 to make pose and firing-origin data dependable for the next projectile slice.
2. Complete US3 to prove the existing development camera still serves the richer scene.
3. Run all quality and manual checks, then update only actually fulfilled roadmap items.

## Notes

- No new crate, dependency, generic entity framework, terrain framework, weapon model, or gameplay
  system is authorised by these tasks.
- Do not mark roadmap work for projectiles, aiming, damage, destruction, movement, turns, or AI.
- Every task follows the required checkbox, sequential ID, optional parallel marker, story label,
  and explicit-path format.
