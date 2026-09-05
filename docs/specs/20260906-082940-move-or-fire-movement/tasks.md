# Tasks: Move or Fire — Basic Tactical Movement

**Input**: Design documents in `/docs/specs/20260906-082940-move-or-fire-movement/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`, and `quickstart.md`

**Tests**: Required by FR-016. Write domain tests before implementation; do not substitute keyboard, HUD, renderer, or frame-timing tests.

## Phase 1: Setup

**Purpose**: Confirm the feature branch starts healthy. No project or dependency setup is needed.

- [X] T001 Run baseline check, test, format, and lint commands from `README.md` before changing `crates/azimuth-game/`

---

## Phase 2: Foundational Movement Vocabulary

**Purpose**: Define small engine-independent vocabulary shared by movement stories.

- [X] T002 Add cardinal direction, one-unit step, six-step allowance, 0.75 rise/run limit, and rejection vocabulary in `crates/azimuth-game/src/tank.rs`
- [X] T003 Add direct unit tests for cardinal offsets/directions and finite shared movement constants in `crates/azimuth-game/src/tank.rs`

**Checkpoint**: One readable vocabulary exists; no grid, physics, generic action, or navigation framework is added.

---

## Phase 3: User Story 1 - Choose One Tactical Action (Priority: P1) 🎯 MVP

**Goal**: Authoritative state lets the current player choose move or fire and prevents both in one turn while retaining existing shot resolution.

**Independent Test**: In pure `TurnState`, choose either action and verify the opposite action and disallowed aim changes are rejected until handoff.

### Tests for User Story 1

- [X] T004 [US1] Add failing state-transition tests for choosing, begin-movement, begin-fire, action/aim exclusivity, and resolving-shot input rejection in `crates/azimuth-game/src/turn.rs`
- [X] T005 [US1] Update projectile terminal-resolution tests to assert renamed resolving state holds player through crater application in `crates/azimuth-game/src/main.rs`

### Implementation for User Story 1

- [X] T006 [US1] Replace `Ready` with choosing, moving-with-remaining-steps, and resolving-fire states plus narrow transitions in `crates/azimuth-game/src/turn.rs`
- [X] T007 [US1] Gate existing aim/fire operations through choosing and retain one-time post-resolution handoff in `crates/azimuth-game/src/turn.rs`
- [X] T008 [US1] Add M/Space action selection and state-aware choosing-versus-resolving HUD guidance in `crates/azimuth-game/src/main.rs`

**Checkpoint**: Move/fire choice is authoritative and firing retains the complete existing artillery loop.

---

## Phase 4: User Story 2 - Reposition Deliberately Within an Allowance (Priority: P1)

**Goal**: A selected movement action moves only the active tank in one-unit requests, spends six steps, ends early, and hands off once.

**Independent Test**: Move Player One through valid requests until early finish or exhaustion and verify budget, opposing pose, and Player Two handoff.

### Tests for User Story 2

- [X] T009 [US2] Add failing tests for initial allowance, accepted-step consumption, early forfeiture, zero-allowance handoff, and deterministic movement traces in `crates/azimuth-game/src/turn.rs`
- [X] T010 [US2] Add failing tests for one-unit cardinal pose changes, terrain grounding, body facing, unchanged turret direction, and unaffected opposing tank in `crates/azimuth-game/src/tank.rs`

### Implementation for User Story 2

- [X] T011 [US2] Add atomic accepted-step pose construction that grounds a tank and updates only `body_forward` in `crates/azimuth-game/src/tank.rs`
- [X] T012 [US2] Add accepted-step, early-finish, exhausted-budget handoff, and independent-player invariants in `crates/azimuth-game/src/turn.rs`
- [X] T013 [US2] Replace immutable `InitialTanks` with mutable authoritative tank state and current-player lookup/update helpers in `crates/azimuth-game/src/main.rs`
- [X] T014 [US2] Add deterministic I/J/K/L selection, Enter completion, allowance integration, and movement-status HUD feedback in `crates/azimuth-game/src/main.rs`

**Checkpoint**: A six-step movement turn is playable; only the active tank moves, and ending movement never fires.

---

## Phase 5: User Story 3 - Let Terrain Shape Repositioning (Priority: P2)

**Goal**: Current normal/cratered terrain grounds valid movement while bounds, steepness, and exhausted budget preserve state.

**Independent Test**: Move on ordinary and cratered terrain, then compare rejected-bound/steep pose and budget to their prior values.

### Tests for User Story 3

- [X] T015 [P] [US3] Add current-surface endpoint slope/passability tests, including cratered terrain and repeatable steep fixture, in `crates/azimuth-game/src/battlefield.rs`
- [X] T016 [US3] Add failing tests for bounds, slope, deformed grounding, rejection preservation, and identical-input results in `crates/azimuth-game/src/tank.rs`
- [X] T017 [US3] Add failing tests proving invalid requests do not consume allowance or hand off in `crates/azimuth-game/src/turn.rs`

### Implementation for User Story 3

- [X] T018 [US3] Add bounded current-surface endpoint slope/passability query without another terrain representation in `crates/azimuth-game/src/battlefield.rs`
- [X] T019 [US3] Integrate bounds/current-surface slope validation into atomic tank step, sampling both horizontal endpoints rather than stale tank Y, in `crates/azimuth-game/src/tank.rs`
- [X] T020 [US3] Consume a step only after accepted tank result and expose bounds/slope/exhaustion feedback to HUD integration in `crates/azimuth-game/src/main.rs`

**Checkpoint**: Craters constrain movement only through changed authoritative terrain; invalid requests cannot corrupt pose, orientation, or allowance.

---

## Phase 6: User Story 4 - Retain a Player's Firing Knowledge After Moving (Priority: P3)

**Goal**: Movement preserves player aim, changes firing origin through position, and keeps body visual direction independent from turret aim.

**Independent Test**: Move Player One, complete Player Two's turn, then compare Player One's retained aim with changed launch position.

### Tests for User Story 4

- [X] T021 [US4] Add tests that moved tank plus unchanged aim keeps angles/power but changes launch position while opponent aim remains independent in `crates/azimuth-game/src/main.rs`
- [X] T022 [US4] Add tests that only final accepted direction changes body facing and invalid movement preserves body/turret directions in `crates/azimuth-game/src/tank.rs`

### Implementation for User Story 4

- [X] T023 [US4] Add tank-root/body player tags and synchronize root translation plus body-only orientation from current tank poses in `crates/azimuth-game/src/main.rs`
- [X] T024 [US4] Update firing lookup and turret/barrel/muzzle sync to read mutable current poses while retaining aim-derived turret orientation in `crates/azimuth-game/src/main.rs`

**Checkpoint**: The moved tank and new firing origin are visible; movement changes the world-space firing solution without clearing aim.

---

## Phase 7: Polish and Cross-Cutting Completion

**Purpose**: Document, manually validate, update only proven roadmap work, and run quality gates.

- [X] T025 [P] Update status, controls, allowance, slope restriction, terrain following, and retained aim in `README.md`
- [X] T026 [P] Document tactical steps, current-terrain grounding, slope rule, and stationary-tank-settling boundary in `docs/world-conventions.md`
- [X] T027 Update only validated move-or-fire, slope, basic-movement, model/readability/distance-consumption, player-skill, and HUD checkboxes in `docs/roadmap.md`
- [ ] T028 Run all scenarios in `docs/specs/20260906-082940-move-or-fire-movement/quickstart.md` and record nonessential discoveries in `docs/roadmap.md`

  Pending interactive desktop verification: the Bevy executable starts, but this environment's X11
  event loop fails with `XOpenDisplayFailed`, so keyboard/visual scenarios require a graphical
  desktop session.
- [X] T029 Run check, test, format, and Clippy commands from `README.md`

---

## Dependencies and Execution Order

```text
T001 → T002–T003 → US1 (T004–T008) → US2 (T009–T014) → US3 (T015–T020)
                                                       └→ US4 (T021–T024)
US3 + US4 → T025–T029
```

| Story | Depends on | Why |
|-------|------------|-----|
| US1 | Foundational vocabulary | Establishes authoritative choice before physical movement. |
| US2 | US1 | Movement must be a selected primary action before spending allowance. |
| US3 | US2 | Terrain constraints apply to established movement lifecycle. |
| US4 | US2 | A changed authoritative pose is required to prove changed launch origin. |

## Parallel Opportunities

- T015 may run independently after US2 because it changes only `battlefield.rs`; complete it before T019.
- T021 and T022 may run in parallel after US2 because they target `main.rs` and `tank.rs`; T023/T024 follow their results.
- T025 and T026 may run in parallel after behavior is stable; T027 follows verified implementation and manual validation.

## Implementation Strategy

### MVP First

1. Complete T001–T008 for authoritative move/fire choice while guarding artillery resolution.
2. Complete T009–T014 for a six-step, early-endable movement turn.
3. Validate US1 and US2 independently before adding terrain constraints.

### Incremental Delivery

1. Add US3 so changed terrain constrains the playable movement action.
2. Add US4 so movement changes launch origin while aim remains retained.
3. Complete docs, quickstart validation, roadmap updates, and quality gates.

## Format Validation

All 29 tasks use a checkbox, sequential ID, exact path, and required user-story label for story work. `[P]` is used only for separate-file work after its stated dependencies.
