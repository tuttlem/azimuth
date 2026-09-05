---

description: "Implementation tasks for the Minimal 3D Battlefield and Camera feature"
---

# Tasks: Minimal 3D Battlefield and Camera

**Input**: Design documents from `docs/specs/20260905-115213-battlefield-camera/`

**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), and [quickstart.md](./quickstart.md)

**Tests**: Add only the small unit coverage justified by the specification for deterministic terrain and boundary behaviour. Rendering and input are manually verified through the quickstart steps.

**Organization**: Tasks are grouped by user story. The static terrain and bounded camera form the P1 slice; documented conventions and orientation aids build on it without adding game-domain work.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel because it changes a different file and has no unfinished task dependency.
- **[Story]**: Identifies the user story served by the task.

## Phase 1: Setup

**Purpose**: Confirm the existing game crate remains the only required implementation boundary.

- [X] T001 Confirm `crates/azimuth-game/Cargo.toml` needs no dependency or crate-structure change for the existing Bevy 0.18.1 implementation.

---

## Phase 2: Foundational

**Purpose**: Define small deterministic values shared by terrain, camera bounds, and documentation before rendering the feature.

- [X] T002 Define named battlefield extent, camera-limit, and deterministic height-calculation helpers with focused unit tests in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: The feature has one readable source of truth for the 40 by 40 unit battlefield, camera bounds, and visual height variation.

---

## Phase 3: User Story 1 - Inspect the Battlefield (Priority: P1) 🎯 MVP

**Goal**: A developer can run Azimuth, see a clearly non-flat bounded battlefield, and orbit, pan, and zoom around it using documented development controls.

**Independent Test**: Run `cargo run --package azimuth-game`; use right-mouse drag, scroll, and WASD or arrow keys to inspect relief, slopes, and edges from at least three viewpoints.

### Implementation for User Story 1

- [X] T003 [US1] Replace the flat proof geometry with one static indexed 40 by 40 unit terrain mesh, smooth normals, simple material, and lighting in `crates/azimuth-game/src/main.rs`.
- [X] T004 [US1] Add the bounded target-centred development-camera state and direct right-mouse orbit, wheel zoom, and WASD/arrow pan input in `crates/azimuth-game/src/main.rs`.
- [X] T005 [US1] Document the Azimuth run command and development-camera control mapping in `README.md`.

**Checkpoint**: The P1 battlefield can be inspected interactively and is independently demonstrable without tanks, projectiles, collision, or gameplay input.

---

## Phase 4: User Story 2 - Understand a Shared Starter World (Priority: P2)

**Goal**: A contributor can find and apply the limited visible-world conventions without mistaking them for a projectile or gameplay model.

**Independent Test**: Read the conventions document and compare it to the running scene: it states Y-up, origin-centred placement, abstract units, and approximately 40 by 40 unit bounds while explicitly deferring projectile and aiming semantics.

### Implementation for User Story 2

- [X] T006 [US2] Create the limited world-space convention record, including explicit deferred decisions, in `docs/world-conventions.md`.
- [X] T007 [US2] Link the world-conventions record from the relevant development guidance in `README.md`.

**Checkpoint**: The visible world has a clear, documented starting point without prematurely defining azimuth, elevation angles, launch vectors, or projectile boundaries.

---

## Phase 5: User Story 3 - Orient Within the Development Scene (Priority: P3)

**Goal**: A developer can recognise the origin and axis directions in the otherwise placeholder scene using a minimal visual aid.

**Independent Test**: Run Azimuth and confirm origin axes are visible and useful from camera viewpoints without obscuring terrain or adding a general debug system.

### Implementation for User Story 3

- [X] T008 [US3] Draw only an origin-axis orientation gizmo as part of the scene update in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: The scene communicates orientation with one local diagnostic and no reusable debug-rendering framework.

---

## Phase 6: Polish and Cross-Cutting Validation

**Purpose**: Verify the full feature, accurately record completed roadmap work, and leave the repository healthy.

- [X] T009 Update completed camera, battlefield-rendering, coordinate-convention, terrain-relief, and justified debug-visualisation checkboxes—while leaving later gameplay work unchecked—in `docs/roadmap.md`.
- [X] T010 [P] Reconcile the feature status and run/control guidance in `README.md` after final scene behaviour is verified.
- [X] T011 Run automated workspace checks from `docs/specs/20260905-115213-battlefield-camera/quickstart.md`: check, test, fmt check, and Clippy with warnings denied.
- [X] T012 Run manual camera, terrain, orientation, and clean-close scenarios in `docs/specs/20260905-115213-battlefield-camera/quickstart.md` and record the result in the feature's `tasks.md` checkboxes.

---

## Dependencies and Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Can start immediately.
- **Foundational (Phase 2)**: Depends on T001 and blocks terrain construction, camera bounds, and documented dimensions.
- **User Story 1 (Phase 3)**: Depends on T002. It is the MVP and must be visually verified before later feature work is declared complete.
- **User Story 2 (Phase 4)**: Depends on T002 and should be checked against the completed scene.
- **User Story 3 (Phase 5)**: Depends on T003 because it visualises the completed battlefield.
- **Polish (Phase 6)**: Depends on all user-story work. T009 occurs only after acceptance criteria are met; T011 precedes final completion; T012 requires a desktop session.

### User Story Dependencies

- **US1 (P1)**: Depends only on foundational constants and calculations (T002).
- **US2 (P2)**: Depends on T002 for documented facts and US1's scene for manual comparison; it adds no gameplay state.
- **US3 (P3)**: Depends on US1's battlefield scene but is otherwise self-contained.

## Parallel Opportunities

- After T002, T003 (terrain/camera source) and T006 (world-conventions document) can proceed in parallel because they modify different files.
- After T003, T005 (initial control guidance) and T008 (origin gizmo) can proceed in parallel only if README guidance follows the agreed control mapping.
- T011 automated checks and desktop T012 can start independently once code and documentation tasks are complete, although both results are required before completion.

### Parallel Example: After Foundational Work

```text
Task: "Replace flat proof geometry with the static terrain mesh in crates/azimuth-game/src/main.rs"
Task: "Create the limited world-space convention record in docs/world-conventions.md"
```

## Implementation Strategy

### MVP First

1. Complete T001–T002.
2. Complete T003–T005 for User Story 1.
3. Launch the application and inspect terrain using all three control families.
4. Stop here if necessary: the navigable battlefield is already a coherent increment.

### Incremental Delivery

1. Add US2 documentation so the visible world can support the next feature without guessing.
2. Add US3 origin axes only after confirming the terrain itself is readable.
3. Run quality and manual checks, then update only satisfied roadmap items.

## Notes

- No dependency, crate, generic terrain system, or gameplay abstraction is authorised by these tasks.
- Do not check roadmap items for tanks, projectiles, terrain collision/deformation, aiming, or final camera behaviour.
- Every task follows the required checkbox, ID, optional parallel marker, story label, and explicit path format.
