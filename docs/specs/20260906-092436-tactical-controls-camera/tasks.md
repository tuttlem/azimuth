---

description: "Implementation tasks for Tactical Controls and Camera Flow"
---

# Tasks: Tactical Controls and Camera Flow

**Input**: Design documents from `/docs/specs/20260906-092436-tactical-controls-camera/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, and
`contracts/tactical-controls.md`

**Tests**: Required by FR-012. Add focused Rust unit tests to the existing `main.rs` test module;
avoid input-event, pixel, screenshot, and exact non-invariant camera-coordinate tests.

**Organization**: Work is grouped by user story. All authoritative gameplay remains outside
camera/repeat presentation state.

## Phase 1: Setup (Shared Understanding)

**Purpose**: Establish the existing integration seams and test baseline before modifying controls.

- [X] T001 Review the relevant camera/input/turn system order and existing unit-test module in `crates/azimuth-game/src/main.rs` against `docs/specs/20260906-092436-tactical-controls-camera/plan.md`.
- [X] T002 [P] Confirm the player-visible mappings and manual acceptance commands in `docs/specs/20260906-092436-tactical-controls-camera/contracts/tactical-controls.md` and `docs/specs/20260906-092436-tactical-controls-camera/quickstart.md` remain the implementation source of truth.

---

## Phase 2: Foundational (Shared Input and Presentation Boundaries)

**Purpose**: Create the small, reusable local representations needed by controls and camera work.

**⚠️ CRITICAL**: Complete this phase before implementing either camera story.

- [X] T003 Add fixed tactical key mapping data/helpers, repeat-delay constants, and a local repeat-state resource with reset semantics in `crates/azimuth-game/src/main.rs`.
- [X] T004 Add a presentation-only camera intent and bounded desired-pose representation, distinct from `TurnState`, in `crates/azimuth-game/src/main.rs`.
- [X] T005 Add focused unit tests for key-pair ambiguity, immediate/repeated interval calculation, release/ineligible reset, and bounded camera-pose helpers in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: The shared helpers express the 300 ms/100 ms control contract and camera state has
no write path into turn, tank, terrain, or projectile resources.

---

## Phase 3: User Story 1 - Aim Comfortably With Tactical Controls (Priority: P1) 🎯 MVP

**Goal**: The current player aims with directional keys and dedicated power keys, with controllable
held-key repeat and no keyboard camera-pan collision.

**Independent Test**: On a choosing turn, press/hold each contract aim key and verify immediate
and repeated active-player-only adjustment, bounds, release behavior, retained Shift coarse
amounts, and no camera panning.

### Tests for User Story 1

- [X] T006 [US1] Add unit tests for Left/Right azimuth, Up/Down elevation, `-`/`=` power, repeated bounds, Shift coarse repeat, active-player isolation, and retired arrow/WASD pan behavior in `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 1

- [X] T007 [US1] Replace Q/E, R/F, and T/G press-only input with contract directional/power mappings and invoke the local repeat tracker only while `TurnPhase::Choosing` in `crates/azimuth-game/src/main.rs`.
- [X] T008 [US1] Remove the keyboard camera-pan branch and `CAMERA_PAN_SPEED`, retaining only right-mouse orbit and wheel zoom in `crates/azimuth-game/src/main.rs`.
- [X] T009 [US1] Update choosing-turn HUD hints to show arrows, `-`/`=`, repeat/Shift guidance, M, and Space without stale controls in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: US1 is independently playable: aiming is comfortable and its input cannot move the
camera or affect the non-current player.

---

## Phase 4: User Story 2 - Understand Whose Turn It Is Through Camera Presentation (Priority: P1)

**Goal**: Every choosing turn presents the active tank with a smooth, bounded, replaceable camera
transition while gameplay stays immediately actionable.

**Independent Test**: End a movement turn or resolve a shot and verify the camera seeks the new
active tank without waiting for transition completion; begin the next action during the transition.

### Tests for User Story 2

- [X] T010 [US2] Add unit tests for deriving active-player intent from choosing-turn/tank state, replacing an in-progress stale active-player target, and keeping turn progression independent of camera transition state in `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 2

- [X] T011 [US2] Derive a tuneable behind-or-near active-player desired pose from authoritative tank position/body direction and clamp it to documented camera bounds in `crates/azimuth-game/src/main.rs`.
- [X] T012 [US2] Update `update_battlefield_camera` to select/reselect active-player presentation intent and interpolate toward its desired pose with render `Time`, preserving non-conflicting mouse orbit/zoom in `crates/azimuth-game/src/main.rs`.
- [X] T013 [US2] Add an intent comment and system-order assertion/documentation that camera progress neither gates nor writes authoritative gameplay in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: US2 visibly identifies the active player on startup and handoff, including a
movement turn, with no authoritative camera dependency.

---

## Phase 5: User Story 3 - Read a Fired Shot at Battlefield Scale (Priority: P2)

**Goal**: Projectile launch selects a wider stable battlefield presentation and normal resolution
returns camera intent to the next active player.

**Independent Test**: Fire from either tank and verify launch/fixed resolution proceeds immediately
while the camera pulls back; once resolution hands off, confirm it starts presenting the next tank.

### Tests for User Story 3

- [X] T014 [US3] Add unit tests for selecting `WatchingShot` during active flight, replacing it with the new active-player intent at normal resolution, and proving projectile/turn outcomes do not depend on arbitrary camera transition completion in `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 3

- [X] T015 [US3] Define a tuneable, bounded wide-shot desired pose centered on readable battlefield context in `crates/azimuth-game/src/main.rs`.
- [X] T016 [US3] Extend camera intent selection so `ProjectileFlight::Some` promptly replaces active-player intent with wide shot, and normal flight completion/handoff promptly replaces it with the next player's intent in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: US3 adds readable launch/arc/impact framing without projectile-follow lock, impact
cinematics, or a delay to authoritative resolution.

---

## Phase 6: Polish & Cross-Cutting Completion

**Purpose**: Keep player guidance, roadmap, validation, and repository health truthful.

- [X] T017 Update exact tactical controls, held repeat behavior, retained mouse controls, and presentation-only camera behavior in `README.md`.
- [X] T018 [P] Update the tactical control section and relevant presentation boundary wording in `docs/world-conventions.md`.
- [X] T019 Update only the demonstrably completed control/camera/HUD items and preserve deferred projectile/impact-camera work in `docs/roadmap.md`.
- [X] T020 Reconcile `docs/specs/20260906-092436-tactical-controls-camera/contracts/tactical-controls.md` and `docs/specs/20260906-092436-tactical-controls-camera/quickstart.md` with the implemented constants and bindings.
- [X] T021 Run the complete manual acceptance sequence and record any genuinely discovered deferred work in `docs/roadmap.md`.
- [X] T022 Run `cargo check --workspace --all-targets`, `cargo test --workspace`, `cargo fmt --all -- --check`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings` from the repository root.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Starts immediately.
- **Foundational (Phase 2)**: Depends on T001; provides the direct helpers/state required by all
  feature work.
- **US1 (Phase 3)**: Depends on T003 and T005. This is the MVP.
- **US2 (Phase 4)**: Depends on T004 and T005; can be implemented after foundational work, but
  should be integrated after US1 to avoid conflicting `main.rs` input/camera edits.
- **US3 (Phase 5)**: Depends on US2's camera intent/transition integration (T011–T013).
- **Polish (Phase 6)**: Depends on implemented US1–US3 behavior.

### User Story Dependencies

```text
Foundational
 ├── US1: tactical controls (MVP)
 └── US2: active-player presentation ──> US3: shot presentation
                                       
US1 + US2 + US3 ──> polish, documentation, validation
```

### Parallel Opportunities

- T002 can proceed alongside source-code seam review in T001.
- T017 and T018 edit distinct documentation files once the bindings/behavior are stable; T019 must
  follow verified outcomes.
- T006, T010, and T014 are intentionally not marked parallel because all extend the same existing
  `main.rs` test module; consolidating them avoids merge conflicts.

## Implementation Strategy

### MVP First

1. Complete T001–T005.
2. Complete US1 (T006–T009).
3. Run its independent controls test before camera work.

### Incremental Delivery

1. Deliver directional/repeating controls without changing simulation.
2. Add active-player presentation as a read-only observer.
3. Add shot pullback as a second presentation intent.
4. Finish documentation and quality validation only after all behavior is demonstrated.
