---

description: "Actionable task list for First Crater — Terrain Deformation"
---

# Tasks: First Crater — Terrain Deformation

**Input**: Design documents from `/docs/specs/20260905-183525-first-crater-terrain-deformation/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, and `quickstart.md`

**Tests**: Add deterministic terrain-domain and projectile interaction tests before the relevant
implementation. Visual crater appearance is verified manually rather than with renderer snapshots.

**Organization**: Tasks are grouped by user story so the crater, later-shot collision, and robust
overlap/boundary behaviour can be validated incrementally.

## Phase 1: Setup (Shared Understanding)

**Purpose**: Confirm the protected initial terrain, impact, boom, marker, and tank behaviour before
refactoring the terrain source of truth.

- [X] T001 Review the current fixed terrain query/mesh contract and its tests in `crates/azimuth-game/src/battlefield.rs`.
- [X] T002 Review the impact-to-marker/boom flow and initial tank construction in `crates/azimuth-game/src/{main,tank,projectile}.rs`.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Make the existing swept projectile interface able to read the application’s current
terrain without adding a collision framework.

**⚠️ CRITICAL**: Complete this phase before integrating terrain mutation with impact handling.

- [X] T003 Generalize the terrain-height input accepted by `Projectile::advance_with_terrain` while preserving deterministic swept-impact semantics and existing coverage in `crates/azimuth-game/src/projectile.rs`.

**Checkpoint**: Projectile simulation can query a captured current terrain value without exposing
renderer types or changing its kinematic model.

---

## Phase 3: User Story 1 - See an Impact Reshape the Battlefield (Priority: P1) 🎯 MVP

**Goal**: One resolved terrain impact permanently lowers the authoritative ground and promptly
shows the crater, while all existing impact presentation remains intact.

**Independent Test**: Apply one default crater to deterministic initial terrain and verify centre
lowering, unchanged exterior, radial transition, parameter effects, and mesh/query agreement; then
fire `I` and observe boom, marker, and permanent depression.

### Tests for User Story 1

> **NOTE: Write these tests first and confirm they fail before implementing the terrain type.**

- [X] T004 [US1] Add deterministic tests for crater centre lowering, outside-radius preservation, smooth transition, radius/depth effects, and current mesh-position/query agreement in `crates/azimuth-game/src/battlefield.rs`.

### Implementation for User Story 1

- [X] T005 [US1] Replace static height generation with a concrete mutable `BattlefieldTerrain` and explicit validated default `Crater` configuration in `crates/azimuth-game/src/battlefield.rs`.
- [X] T006 [US1] Implement the documented smooth radial lowering profile and current-triangle height/mesh-position derivation in `crates/azimuth-game/src/battlefield.rs`.
- [X] T007 [US1] Construct authoritative terrain before initial tanks and ground those initial poses from it in `crates/azimuth-game/src/{main,tank}.rs`.
- [X] T008 [US1] Store terrain as application state, apply one default crater from the resolved impact in fixed simulation, and refresh the tagged battlefield mesh only after terrain changes in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: The first impact produces a permanent visible crater sourced solely from the
authoritative impact position; marker, boom, tanks, and pre-impact flight remain available.

---

## Phase 4: User Story 2 - Fire Into Changed Ground (Priority: P2)

**Goal**: Subsequent projectiles and all terrain-height queries use the crater’s new lower surface,
not the original battlefield.

**Independent Test**: Apply a crater, then prove a deterministic path through removed space stays
active past the old surface and a descending path impacts the new queried surface.

### Tests for User Story 2

- [X] T009 [US2] Add mutable-terrain height-query tests proving affected positions report the new surface immediately in `crates/azimuth-game/src/battlefield.rs`.
- [X] T010 [US2] Add swept projectile regression tests for passing through crater-removed space and impacting the new lower crater surface in `crates/azimuth-game/src/projectile.rs`.

### Implementation for User Story 2

- [X] T011 [US2] Pass the current `BattlefieldTerrain` query into fixed-step projectile advancement and retain distinct non-impact termination in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: A later shot uses the same deformed terrain that is rendered, without stale
collision state or presentation-controlled physics.

---

## Phase 5: User Story 3 - Reliably Layer Battlefield Changes (Priority: P3)

**Goal**: Repeated, overlapping, sloped, and edge-adjacent impacts compose predictably without
invalid terrain or crashes.

**Independent Test**: Run the same ordered crater sequence twice, including a slope, overlap, and
boundary impact; compare all sampled heights and assert finite grid data with no panic.

### Tests for User Story 3

- [X] T012 [US3] Add terrain-domain tests for sloped terrain, overlapping/repeated craters, edge clipping, finite values, and 100-repeat deterministic sequences in `crates/azimuth-game/src/battlefield.rs`.

### Implementation for User Story 3

- [X] T013 [US3] Ensure crater application lowers current grid heights in fixed order, clips naturally to valid vertices, and rejects invalid crater parameters in `crates/azimuth-game/src/battlefield.rs`.
- [X] T014 [US3] Preserve one crater per terrain impact and ensure out-of-bounds projectile termination cannot mutate terrain in `crates/azimuth-game/src/main.rs`.
- [X] T015 [US3] Add a focused regression that initial tank state remains valid and unaffected by terrain mutation in `crates/azimuth-game/src/{main,tank}.rs`.

**Checkpoint**: Layered and edge-adjacent craters are deterministic, stable, bounded, and leave
existing tank state intact pending the later tank-reconciliation feature.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Document the gameplay/rendering boundary, update only fulfilled roadmap work, and run
the complete quality and manual acceptance flow.

- [X] T016 [P] Document mutable authoritative terrain, crater profile, and subsequent collision behaviour in `docs/projectile-model.md`.
- [X] T017 [P] Update battlefield/deformation status and controls guidance in `README.md` and `docs/world-conventions.md`.
- [X] T018 Update only the fulfilled near-term, terrain-deformation, explosion-gameplay, and demonstrated tactical-effects checkboxes in `docs/roadmap.md`.
- [X] T019 Run `cargo check --workspace --all-targets`, `cargo test --workspace`, `cargo fmt --all -- --check`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings` from the repository root.
- [X] T020 Execute the complete desktop acceptance flow in `docs/specs/20260905-183525-first-crater-terrain-deformation/quickstart.md` and record any environment limitation or follow-up there only if needed.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies.
- **Foundational (Phase 2)**: Depends on T001–T002 and blocks story implementation.
- **US1 (Phase 3)**: Depends on T003; T004 precedes T005–T006, T007 precedes T008.
- **US2 (Phase 4)**: Depends on the completed US1 mutable terrain and impact integration; T009–T010 precede T011.
- **US3 (Phase 5)**: Depends on US1 terrain mutation; T012 precedes T013–T015.
- **Polish (Phase 6)**: Depends on US1–US3 completion; T016 and T017 can run in parallel, then complete T018–T020.

### User Story Dependencies

- **US1 (P1)**: Delivers the MVP permanent crater and establishes the authoritative mutable
  terrain required by later behaviour.
- **US2 (P2)**: Builds on US1 because later projectile collision must read its changed terrain.
- **US3 (P3)**: Builds on US1’s crater model to validate and harden composition and bounds.

### Parallel Opportunities

- T016 and T017 update independent documentation files and can run in parallel after feature
  behaviour is complete.
- Code and test tasks are intentionally sequential: they change the same small terrain, projectile,
  and application handoff and must preserve one coherent source of truth.

## Parallel Example: Polish Documentation

```text
Task: "Document mutable authoritative terrain, crater profile, and subsequent collision behaviour in docs/projectile-model.md"
Task: "Update battlefield/deformation status and controls guidance in README.md and docs/world-conventions.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete T001–T003 to preserve the terrain-query boundary.
2. Complete T004–T008 to create, render, and test one permanent crater from an existing impact.
3. Run focused terrain tests and manually press `I` to verify boom, marker, and visible crater.

### Incremental Delivery

1. Deliver US1: impact permanently lowers the authoritative and visible battlefield.
2. Deliver US2: later projectile queries and collision immediately use that lowered surface.
3. Deliver US3: prove stable overlap, repeatability, edge safety, and preserved tank state.
4. Finish documentation, roadmap alignment, quality gates, and the quickstart acceptance flow.
