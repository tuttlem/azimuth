---

description: "Implementation tasks for Presentation Polish — Full Player Scoreboard and Immersive Battlefield Background"
---

# Tasks: Presentation Polish — Full Player Scoreboard and Immersive Battlefield Background

**Input**: Design documents from `/docs/specs/20260909-074958-presentation-polish/`

**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md),
[data-model.md](./data-model.md), [presentation contract](./contracts/presentation.md), and
[quickstart.md](./quickstart.md)

**Tests**: Required. The specification explicitly requires automated coverage for scoreboard
projection, visual-horizon initialization, and preservation of authoritative terrain bounds. Keep
renderer acceptance manual; do not add pixel tests.

**Organization**: Tasks are grouped by user story so each increment can be implemented and tested
independently. `main.rs` tasks are deliberately ordered to avoid conflicting edits to the current
single scene/HUD module.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can be worked on in parallel because it affects a different file and has no incomplete
  prerequisite.
- **[Story]**: Identifies the user story served by the task.
- Every task uses an exact repository path.

## Phase 1: Setup (Shared Baseline)

**Purpose**: Establish the current feature-branch baseline before changing a live playable scene.

- [X] T001 Run the existing focused test suite from `crates/azimuth-game/Cargo.toml` and record any pre-existing failures before editing `crates/azimuth-game/src/main.rs` or `crates/azimuth-game/src/battlefield.rs`.

---

## Phase 2: Foundational (No New Shared Infrastructure)

**Purpose**: Confirm the architecture boundary that all increments must preserve.

No new shared framework, crate, dependency, or authoritative terrain type is permitted. The existing
`MatchConfiguration`, `Tank`, `TurnState`, and `BattlefieldTerrain` boundaries are the foundation;
the first task in each story adds only its focused read-only presentation seam.

**Checkpoint**: Do not introduce a generic HUD/environment layer, a second terrain authority, or
outer-world gameplay APIs.

---

## Phase 3: User Story 1 - Read Every Participant (Priority: P1) 🎯 MVP

**Goal**: Display one compact, truthful, ordered scoreboard entry for every configured participant
in supported 2–8-player matches.

**Independent Test**: Construct 2-, 3-, and 8-player configurations with distinctive names and
tanks; assert the pure scoreboard projection returns exactly that ordered collection with live
health, elimination, and active-state values. Run a 2-player match to confirm the retained panel
still looks compact.

### Tests for User Story 1

- [X] T002 [US1] Add failing pure projection tests for 2-, 3-, and 8-player entry count, configured order/names, live health, eliminated rows, and resolving/finished active-state absence in `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 1

- [X] T003 [US1] Replace the fixed two-player fields in `TacticalHudView` with a read-only ordered scoreboard-entry projection that iterates `MatchConfiguration.players`, joins tanks by stable `PlayerId`, and derives active status only from an in-progress turn in `crates/azimuth-game/src/main.rs`.
- [X] T004 [US1] Replace hard-coded Player One/Player Two HUD labels and fills with retained per-player scoreboard rows built for the accepted match configuration; sync row text, health fill, eliminated styling, active styling, configured names, and compact count-aware density in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: Start 2-, 3-, and 8-player local matches. Each has exactly one stable ordered row
per configured player, including eliminated players, and the active player is obvious without a
fixed two-player assumption.

---

## Phase 4: User Story 2 - See a Battlefield in a World (Priority: P1)

**Goal**: Replace normal-view void/floating-board presentation with simple blue sky, clouds, and a
connected low-detail exterior landscape while retaining the existing terrain as authority.

**Independent Test**: Build a generated battlefield and seed, then verify a pure visual-horizon
descriptor is finite, joins the sampled edge, and extends beyond the playable area. Run the scene
and inspect active-player and shot views toward all sides.

### Tests for User Story 2

- [X] T005 [P] [US2] Add failing unit tests for finite low-detail horizon geometry, shared initial terrain-edge positions/colours, exterior extent beyond `HALF_EXTENT`, and no interior presentation vertices other than the shared boundary in `crates/azimuth-game/src/battlefield.rs`.
- [X] T006 [US2] Add a focused pure scene-presentation test for blue sky configuration, non-empty static cloud planning, horizon initialization, and safe normal-view camera pitch bounds in `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 2

- [X] T007 [US2] Add an immutable engine-independent visual-horizon descriptor/mesh-data generator that welds to `BattlefieldTerrain` at ±`HALF_EXTENT`, uses the captured terrain seed and existing elevation palette outside the boundary, stays coarse out to the named exterior extent, and exposes no gameplay query or mutation operation in `crates/azimuth-game/src/battlefield.rs`.
- [X] T008 [US2] Add scene-only blue clear sky, a small fixed set of unlit high/distant cloud primitives, and one separately marked horizon mesh created from the visual-horizon descriptor; keep water bounds unchanged and exclude the horizon from the authoritative mesh-refresh query in `crates/azimuth-game/src/main.rs`.
- [X] T009 [US2] Replace the symmetric orbit-pitch cap with explicit lower and slightly downward/near-horizontal upper limits while preserving existing active-player, shot, mouse-orbit, wheel-zoom, and far-clip behavior in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: Normal aiming and shot views show blue sky, simple clouds, and landscape beyond the
playable terrain at edges and corners; ordinary orbit cannot readily reveal terrain underside or
void.

---

## Phase 5: User Story 3 - Keep Presentation Outside Gameplay (Priority: P2)

**Goal**: Demonstrate that outer landscape and sky remain visual-only and that all gameplay stays
on the existing bounded, mutable battlefield.

**Independent Test**: Construct visual-horizon data, then prove existing out-of-bounds terrain
queries remain absent and that crater/deformation updates only the authoritative terrain mesh.

### Tests for User Story 3

- [X] T010 [P] [US3] Extend authoritative-boundary regression tests to prove `is_within_bounds`, `height_if_within_bounds`, and crater mutation remain unchanged after visual-horizon construction in `crates/azimuth-game/src/battlefield.rs`.
- [X] T011 [US3] Add a regression test that terrain refresh selects only `BattlefieldVisual` and never the separately marked horizon visual in `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 3

- [X] T012 [US3] Review and tighten the presentation/gameplay boundary at horizon construction and terrain refresh so no horizon object enters projectile, tank support, movement, spawning, collision, damage, or deformation paths in `crates/azimuth-game/src/main.rs` and `crates/azimuth-game/src/battlefield.rs`.

**Checkpoint**: Edge movement, projectile limits, collision, spawning, and craters continue to use
only the original playable bounds; the exterior mesh never deforms or affects gameplay outcomes.

---

## Phase 6: User Story 4 - Preserve Readability and Performance (Priority: P2)

**Goal**: Keep the two-player panel polished, eight-player presentation readable, and the extra
scene geometry/responsiveness restrained.

**Independent Test**: Verify scoreboard density decisions for two and eight entries without pixels,
then manually play 2-, 4-, and 8-player turns, shots, impacts, and handoffs with the new scene.

### Tests for User Story 4

- [X] T013 [US4] Add pure layout/presentation-state regression coverage for the compact two-player and readable eight-player scoreboard density rules, preserving active/eliminated visual state mapping, in `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 4

- [X] T014 [US4] Tune only the scoreboard panel’s row gap, type scale, health-bar height, and horizon/cloud count or extent as supported by the pure tests and manual inspection; retain unobstructed central play space and avoid increasing authoritative terrain density in `crates/azimuth-game/src/main.rs` and `crates/azimuth-game/src/battlefield.rs`.

**Checkpoint**: The 2-player panel remains balanced, all eight entries are readable, and normal
2-, 4-, and 8-player play has no noticeable input, turn, projectile, deformation, or camera
responsiveness regression.

---

## Phase 7: Polish & Cross-Cutting Validation

**Purpose**: Complete required validation, documentation, and roadmap discipline without expanding
feature scope.

- [X] T015 Update only demonstrated HUD/world-presentation roadmap items and retain the next AI-controller feature plus unrelated future work in `docs/roadmap.md`.
- [X] T016 Run formatting, focused tests, workspace build, and lint checks specified by `docs/specs/20260909-074958-presentation-polish/quickstart.md`, fixing only feature-caused failures in `crates/azimuth-game/src/main.rs` and `crates/azimuth-game/src/battlefield.rs`.
- [ ] T017 Perform and record the 2-, 4-, and 8-player scoreboard, sky, horizon, edge-boundary, camera, and responsiveness manual acceptance checks in `docs/specs/20260909-074958-presentation-polish/quickstart.md`.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: T001 first establishes the baseline.
- **Foundational (Phase 2)**: Architectural constraints apply throughout; it creates no code task.
- **US1 (Phase 3)**: T002 → T003 → T004. It is the recommended first MVP and has no dependency on
  the visual-world work.
- **US2 (Phase 4)**: T005 and T006 establish tests; T007 → T008 → T009 implements the visual
  world. T005 may proceed alongside US1 work because it affects `battlefield.rs` only.
- **US3 (Phase 5)**: Depends on the horizon descriptor and scene marker from US2: T010 and T011 →
  T012.
- **US4 (Phase 6)**: Depends on US1’s dynamic rows and US2’s visual scene: T013 → T014.
- **Polish (Phase 7)**: T015–T017 depend on all desired story work and verified acceptance.

### User Story Dependencies

- **US1 (P1)**: Independent scoreboard increment; it is the suggested MVP.
- **US2 (P1)**: Independent visual-world increment once its own descriptor/scene tests are in
  place; it can be delivered after US1 to avoid `main.rs` merge conflicts.
- **US3 (P2)**: Requires US2’s horizon implementation because it verifies that exact boundary.
- **US4 (P2)**: Requires US1 and US2 because it tunes and validates their combined readability and
  performance.

### Parallel Opportunities

- T005 can proceed in parallel with T002–T004 because it changes only
  `crates/azimuth-game/src/battlefield.rs`.
- T010 can proceed in parallel with T011 after T007/T008 because their regression work begins in
  different files.
- T015 documentation preparation can begin after implementation evidence exists, but its checkbox
  must not be completed until manual acceptance genuinely supports the roadmap update.

## Parallel Example: User Story 2

```text
Task: "Add finite/coarse/welded visual-horizon tests in crates/azimuth-game/src/battlefield.rs"
Task: "Continue completed scoreboard work in crates/azimuth-game/src/main.rs"
```

After the pure descriptor is complete, keep scene and camera tasks sequential because both modify
`crates/azimuth-game/src/main.rs`.

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Run T001.
2. Complete T002–T004.
3. Start 2-, 3-, and 8-player matches and validate the scoreboard projection and retained rows.
4. Stop for review: the full multiplayer scoreboard is independently usable and testable.

### Incremental Delivery

1. Deliver US1 for truthful full-match readability.
2. Deliver US2 for the sky/cloud/horizon visual world.
3. Deliver US3 to lock the authority boundary with regressions.
4. Deliver US4 and Phase 7 only after manual readability/performance evidence.

## Notes

- All tasks follow the required checkbox, sequential ID, optional parallel marker, story-label, and
  exact-path format.
- Do not add a generic UI framework, asset pipeline, weather system, second authoritative terrain,
  outer-world gameplay, or AI while completing these tasks.
- Update `docs/roadmap.md` only for acceptance criteria that manual and automated validation have
  actually demonstrated.
