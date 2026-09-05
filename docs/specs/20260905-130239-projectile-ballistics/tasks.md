---

description: "Task list for the first deterministic projectile ballistic arc"
---

# Tasks: First Projectile and Deterministic Ballistic Arc

**Input**: Design documents from `/docs/specs/20260905-130239-projectile-ballistics/`

**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md),
[data-model.md](./data-model.md), and [quickstart.md](./quickstart.md)

**Tests**: Focused domain tests are required by the feature specification. They must cover
observable deterministic projectile behaviour without renderer, GPU, or screenshot infrastructure.

**Organization**: Tasks are grouped by user story. The shared world-value refactor is foundational;
the P1 slice supplies the first visible arc, P2 records and protects its shared conventions, and P3
makes gravity and termination behaviour explicit and tunable.

## Phase 1: Setup

**Purpose**: Confirm the active feature context before changing the existing application.

- [X] T001 Review `docs/specs/20260905-130239-projectile-ballistics/spec.md`, `plan.md`, `data-model.md`, and the current `crates/azimuth-game/src/{main,tank,battlefield}.rs` boundary before implementation.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Create the smallest shared engine-independent coordinate boundary used by the existing
tank firing origin and the new projectile domain.

**⚠️ CRITICAL**: Complete this phase before projectile work begins.

- [X] T002 Add `WorldPosition` and `WorldVector` as clear scalar domain values in `crates/azimuth-game/src/world.rs`.
- [X] T003 Refactor `crates/azimuth-game/src/tank.rs` to use `world::WorldPosition` while retaining tank-local horizontal position and direction types.
- [X] T004 Register `world` in `crates/azimuth-game/src/main.rs` and update affected imports so the existing battlefield and tank scene continues to compile.
- [X] T005 Run focused existing tank/battlefield tests with `cargo test --workspace` after the `crates/azimuth-game/src/world.rs` refactor.

**Checkpoint**: The current scene retains its two terrain-grounded tanks and has one shared,
renderer-independent coordinate representation.

---

## Phase 3: User Story 1 - Fire and Observe the First Arc (Priority: P1) 🎯 MVP

**Goal**: Space launches one clearly visible projectile from Player One's firing origin; the
authoritative state advances with deterministic constant-gravity fixed steps and remains deliberately
non-colliding.

**Independent Test**: Run Azimuth, press Space once, and observe one sphere rise, reach an apex,
and descend. Pressing Space during the flight must not make another shot; crossing terrain must not
stop it.

### Tests for User Story 1

- [X] T006 [US1] Add failing deterministic launch-direction, launch-speed, fixed-step, horizontal-velocity, vertical-gravity, ascent/descent, and repeatability tests in `crates/azimuth-game/src/projectile.rs`.

### Implementation for User Story 1

- [X] T007 [US1] Implement the pure `ShotParameters`, `Gravity`, `Projectile`, fixed-step constant-acceleration advancement, and input validation in `crates/azimuth-game/src/projectile.rs` so T006 passes without Bevy types.
- [X] T008 [US1] Register `projectile` and configure one 120 Hz fixed schedule plus an optional authoritative active-projectile resource in `crates/azimuth-game/src/main.rs`.
- [X] T009 [US1] Add Space edge-triggered Player One development-shot creation in `crates/azimuth-game/src/main.rs`, deriving azimuth from its turret direction and ignoring requests while a shot is active.
- [X] T010 [US1] Advance only the active domain projectile in `FixedUpdate` and spawn, synchronise, and despawn one tagged primitive sphere in `crates/azimuth-game/src/main.rs` without calculating motion in presentation code.
- [ ] T011 [US1] Run `cargo test --workspace` and manually verify the P1 launch, apex, re-fire-ignore behaviour, terrain pass-through, existing camera, tanks, and clean close using `docs/specs/20260905-130239-projectile-ballistics/quickstart.md`.

**Checkpoint**: The first visibly ballistic, deterministic, non-colliding development shot works
from Player One's firing origin.

---

## Phase 4: User Story 2 - Reason About Every Shot (Priority: P2)

**Goal**: Contributors can use documented, engine-independent azimuth/elevation and launch-origin
rules and find regression protection for their cardinal and analytical behaviour.

**Independent Test**: Verify cardinal azimuths, level/vertical elevation, and turret-direction
conversion with domain tests; read the world and projectile documentation without consulting Bevy
transform conventions.

### Tests for User Story 2

- [X] T012 [US2] Add cardinal azimuth, level/vertical elevation, normalized direction, and tank-turret-to-azimuth regression tests in `crates/azimuth-game/src/projectile.rs`.

### Implementation for User Story 2

- [X] T013 [US2] Implement the documented degree-based azimuth/elevation conversion and horizontal-direction-to-azimuth helper in `crates/azimuth-game/src/projectile.rs` so T012 passes.
- [X] T014 [P] [US2] Extend `docs/world-conventions.md` with the X/Z plane, azimuth zero and rotation, elevation range, firing-origin handoff, and simulation-volume conventions.
- [X] T015 [P] [US2] Create `docs/projectile-model.md` describing the 1/120-second kinematic update, configurable Y-down gravity, default development shot, and intentional absence of terrain impact.
- [X] T016 [US2] Run `cargo test --workspace` and review `docs/world-conventions.md` plus `docs/projectile-model.md` against the P2 independent-test criteria.

**Checkpoint**: Angles, launch vectors, firing origins, and fixed-step mathematics have one
testable, readable project meaning independent of presentation.

---

## Phase 5: User Story 3 - Tune Gravity During Development (Priority: P3)

**Goal**: Gravity remains an explicit non-Earth parameter, visibly changes equivalent shots, and
the one active projectile always reaches a simple documented end state.

**Independent Test**: Compare equal shots under zero, default, and stronger gravity in domain tests;
confirm a flight ends after a horizontal, vertical, or duration boundary, while terrain crossing
does not end it.

### Tests for User Story 3

- [X] T017 [US3] Add zero-gravity, stronger-versus-weaker-gravity, simulation-limit, and terrain-independent-lifetime tests in `crates/azimuth-game/src/projectile.rs`.

### Implementation for User Story 3

- [X] T018 [US3] Implement non-negative configurable gravity and the X/Z 60, Y -30/100, and 20-second termination rules in `crates/azimuth-game/src/projectile.rs` so T017 passes.
- [X] T019 [US3] Keep the default gravity explicit at the application boundary in `crates/azimuth-game/src/main.rs` and verify it can be changed without altering launch parameters or presentation-owned state.
- [ ] T020 [US3] Run `cargo test --workspace` and manually compare the documented default with a temporary different gravity value, restoring the default after verification per `docs/specs/20260905-130239-projectile-ballistics/quickstart.md`.

**Checkpoint**: Gravity is an explicit, testable gameplay parameter and projectiles end only by the
intentional non-impact lifetime rules.

---

## Phase 6: Polish and Cross-Cutting Completion

**Purpose**: Deliver accurate user guidance, roadmap progress, and repository health after all
feature behaviour is verified.

- [X] T021 Update the status, Space launch control, and projectile-model link in `README.md` while preserving the existing development-camera guidance.
- [X] T022 Update only satisfied coordinate, rendering, projectile, gravity, and Milestone B checkboxes in `docs/roadmap.md`; leave terrain impact, aiming, and later gameplay unchecked.
- [X] T023 Run the complete validation sequence from `docs/specs/20260905-130239-projectile-ballistics/quickstart.md`: `cargo check --workspace --all-targets`, `cargo test --workspace`, `cargo fmt --all -- --check`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- [X] T024 Review the final diff against `docs/specs/20260905-130239-projectile-ballistics/spec.md` and confirm no collision, impact, aiming UI, turn, weapon, environmental, or speculative architecture scope has entered the feature.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Starts immediately.
- **Foundational (Phase 2)**: Starts after T001 and blocks all projectile work.
- **User Story 1 (Phase 3)**: Starts after T005; it is the MVP and produces the first visible arc.
- **User Story 2 (Phase 4)**: Starts after T007 because it protects and documents the P1 domain
  calculation; its documentation tasks T014 and T015 can proceed in parallel once conventions are
  implemented.
- **User Story 3 (Phase 5)**: Starts after T007 because gravity and limits extend the same concrete
  projectile type; it has no dependency on P2 documentation.
- **Polish (Phase 6)**: Starts only after all desired story checkpoints and manual verification pass.

### User Story Dependencies

- **US1 (P1)**: Depends only on the shared world-value refactor.
- **US2 (P2)**: Depends on the pure projectile model introduced for US1; it does not depend on the
  rendered sphere.
- **US3 (P3)**: Depends on the pure projectile model introduced for US1; it does not depend on the
  angle-documentation tasks in US2.

### Parallel Opportunities

- T014 and T015 can run in parallel because they edit different documentation files after the
  convention decisions exist.
- After T007, a contributor can prepare the pure-domain P2/P3 tests while another integrates the
  P1 visual, provided concurrent edits to `projectile.rs` are coordinated.
- Documentation review and manual visual verification can occur alongside the final automated
  validation, but T022 must wait until implementation is genuinely complete.

## Parallel Example: User Story 2

```text
Task: "Extend world conventions in docs/world-conventions.md"
Task: "Create projectile model reference in docs/projectile-model.md"
```

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete T001 through T005 to establish the shared world values.
2. Complete T006 through T010 to make the pure calculation and one visible Space-fired sphere.
3. Complete T011 and manually prove the first arc before adding documentation or gravity-tuning
   refinements.

### Incremental Delivery

1. Foundation plus US1 yields the core visible artillery arc.
2. US2 makes that arc unambiguous and safe for future aiming work.
3. US3 makes gravity a deliberate gameplay parameter with safe flight termination.
4. Polish records only genuine roadmap progress and validates the workspace as a whole.

## Notes

- Every task uses an exact feature or source path and follows the required checklist format.
- No external interface contract is required: this is an internal desktop application feature; the
  user-facing development control and manual verification contract are captured in `quickstart.md`.
- Do not mark any task complete until its stated work and applicable verification have been done.
