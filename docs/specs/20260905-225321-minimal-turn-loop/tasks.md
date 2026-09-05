# Tasks: Minimal Turn Loop

**Input**: Design documents from `/docs/specs/20260905-225321-minimal-turn-loop/`

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [turn-controls.md](contracts/turn-controls.md), and [quickstart.md](quickstart.md)

**Tests**: FR-014 explicitly requires deterministic gameplay-state coverage. Write focused unit and resolution tests; do not add renderer, HUD-text, screenshot, or frame-timing tests.

**Organization**: Tasks are grouped by user story. Shared turn state is established first because every story needs authoritative current-player and phase ownership.

## Phase 1: Setup

**Purpose**: Confirm the active design and current implementation seams before changing gameplay.

- [X] T001 Review the authoritative resolution order and keyboard contract in `docs/specs/20260905-225321-minimal-turn-loop/{plan.md,data-model.md,research.md,contracts/turn-controls.md}` before modifying source.

---

## Phase 2: Foundational Gameplay State

**Purpose**: Establish the small deterministic domain boundary required by every story.

**⚠️ CRITICAL**: Complete this phase before integrating user-story input, launch, or HUD work.

- [X] T002 Add a direct two-player `other` operation and unit coverage to `crates/azimuth-game/src/tank.rs`, reusing the existing `PlayerId` identity.
- [X] T003 Create pure current-player/Ready-or-Resolving state and exactly two retained `AimingState` values, with initial-state, legal-transition, and per-player access tests in `crates/azimuth-game/src/turn.rs`.
- [X] T004 Register the new turn domain module in `crates/azimuth-game/src/main.rs` without adding a crate, dependency, generic state machine, or event framework.

**Checkpoint**: The codebase has one engine-independent source of truth for current player, phase, and player-owned aim that starts Player One/Ready.

---

## Phase 3: User Story 1 - Take One Alternating Turn (Priority: P1) 🎯 MVP

**Goal**: Player One aims and fires; the action remains resolving through flight, impact, and crater application; Player Two then becomes visibly ready and controllable.

**Independent Test**: Start a match, fire Player One into terrain, and verify Player Two becomes Ready only after the authoritative crater has been applied.

### Tests for User Story 1

- [X] T005 [US1] Add focused terminal-resolution tests for active-flight retention, fire-to-Resolving transition, and terrain-crater-before-Player-Two advancement in `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 1

- [X] T006 [US1] Replace Player-One-only `ActiveAiming` resource initialization with authoritative turn/retained-aim resources initialized from both tanks in `crates/azimuth-game/src/main.rs`.
- [X] T007 [US1] Route existing aim input and Space launch through the Ready current player, selecting that player's tank, retained aim, visible muzzle, and one projectile in `crates/azimuth-game/src/main.rs`.
- [X] T008 [US1] Update fixed-step terminal resolution so terrain impact applies its crater and records impact before clearing flight and completing exactly one turn in `crates/azimuth-game/src/main.rs`.
- [X] T009 [US1] Replace Player-One-only tank visual tags/queries with player-associated turret, barrel, and muzzle synchronization derived from authoritative current-player/aim state in `crates/azimuth-game/src/main.rs`.
- [X] T010 [US1] Update HUD formatting and synchronization to show authoritative current player, that player's values, and Ready versus Resolving state in `crates/azimuth-game/src/main.rs`.
- [ ] T011 [US1] Run the P1 terrain-impact scenario in `docs/specs/20260905-225321-minimal-turn-loop/quickstart.md` and reconcile observed mismatches in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: Player One can take one terrain-impacting shot and Player Two receives the same controls only after the crater is authoritative.

---

## Phase 4: User Story 2 - Preserve Each Player's Shot Setup (Priority: P2)

**Goal**: Each player owns independent persistent aim values and sees the corresponding tank, barrel, muzzle, display, and later shot when their turn returns.

**Independent Test**: Give Player One non-default values, resolve their shot; give Player Two different values, resolve their shot; verify Player One's original values return.

### Tests for User Story 2

- [X] T012 [P] [US2] Add deterministic independent-aim retention and current-player-only adjustment tests, including a Player One → Player Two → Player One cycle, in `crates/azimuth-game/src/turn.rs`.
- [X] T013 [P] [US2] Add player-specific launch-parameter and firing-origin tests using both tanks in `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 2

- [X] T014 [US2] Complete retained-aim lookup and visual/launch selection so Player Two starts from its own turret azimuth and neither player overwrites the other's state in `crates/azimuth-game/src/main.rs`.
- [ ] T015 [US2] Run the independent-aim round-trip scenario in `docs/specs/20260905-225321-minimal-turn-loop/quickstart.md` and correct Player One/Two HUD, barrel, muzzle, or launch mismatches in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: Both players can independently bracket shots with their own persistent values.

---

## Phase 5: User Story 3 - Complete Every Fired Action Reliably (Priority: P3)

**Goal**: No resolving action accepts or queues input, and both terrain impact and existing non-impact termination reliably consume exactly one turn.

**Independent Test**: During both active flight and an out-of-bounds/lifetime termination, attempt aim/fire and verify no state changes until one normal handoff occurs.

### Tests for User Story 3

- [X] T016 [P] [US3] Add pure tests rejecting repeated fire and all aim changes while Resolving, plus at least 20 exact Player One/Player Two completion transitions, in `crates/azimuth-game/src/turn.rs`.
- [X] T017 [P] [US3] Add fixed-resolution tests proving active flight preserves current player/phase and an out-of-bounds outcome advances once without impact or terrain change in `crates/azimuth-game/src/main.rs`.
- [X] T018 [US3] Add a deterministic identical-action-sequence regression comparing player order, retained aims, launch conditions, and terminal transitions in `crates/azimuth-game/src/turn.rs`.

### Implementation for User Story 3

- [X] T019 [US3] Harden guards so only Ready creates a projectile and only an existing Resolving terminal projectile outcome completes a turn in `crates/azimuth-game/src/main.rs`.
- [ ] T020 [US3] Run resolving-lock, out-of-bounds, and repeated-alternation scenarios in `docs/specs/20260905-225321-minimal-turn-loop/quickstart.md` and correct extra launches, stalls, or premature handoffs in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: Every shot completes once, no input is accepted while resolving, and long two-player alternation remains deterministic.

---

## Phase 6: Documentation, Roadmap, and Quality

**Purpose**: Document the completed controls/boundary, accurately record roadmap progress, and leave the workspace healthy.

- [X] T021 [P] Update current status and Battlefield Controls for two-player ownership, retained aims, resolving lock, and terrain/non-impact handoff in `README.md`.
- [X] T022 [P] Replace Player-One-only firing/control wording with current-player turn and authoritative-resolution semantics in `docs/projectile-model.md`.
- [X] T023 Update only satisfied turn-flow, fire-action, local-player, and HUD checkboxes—leaving surviving-player, movement, damage, elimination, and match-state work open—in `docs/roadmap.md`.
- [ ] T024 Run every automated/manual validation in `docs/specs/20260905-225321-minimal-turn-loop/quickstart.md`, resolve failures in owning source files, and record no unjustified exceptions.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup**: No dependency.
- **Foundational**: Depends on T001 and blocks user stories; T002 → T003 → T004.
- **US1**: Depends on Phase 2. T005 defines core assertions before T006–T010; T011 validates the MVP.
- **US2**: Depends on US1's working handoff. T012 and T013 can run in parallel; T014 integrates them; T015 validates.
- **US3**: Depends on US1 and follows US2. T016 and T017 can run in parallel; T018 follows the turn-model tests; T019 hardens integration; T020 validates.
- **Polish**: Depends on completed stories. T021 and T022 can run in parallel; T023 follows demonstrated behavior; T024 is the final quality gate.

### User Story Dependencies

- **US1 (P1)**: Independently demonstrable MVP after foundational current-player/phase state exists.
- **US2 (P2)**: Uses US1's handoff to prove independent retained settings and player-specific origins.
- **US3 (P3)**: Uses the complete handoff/retained-state loop to cover input locks, non-impact completion, and deterministic long play.

### Parallel Opportunities

- T012 and T013 operate in different files after US1's integration seam stabilizes.
- T016 and T017 split pure turn-model and fixed-resolution testing after US2; T018 follows T016 in the same turn-model file.
- T021 and T022 update separate documentation files after behavior is demonstrated.

## Parallel Examples

### User Story 2

```text
Task: "Add independent-aim retention tests in crates/azimuth-game/src/turn.rs"
Task: "Add player-specific launch tests in crates/azimuth-game/src/main.rs"
```

### User Story 3

```text
Task: "Add resolving-input/alternation tests in crates/azimuth-game/src/turn.rs"
Task: "Add active/out-of-bounds fixed-resolution tests in crates/azimuth-game/src/main.rs"
```

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete T001–T004 to establish explicit two-player turn state.
2. Complete T005–T010 to route Player One's action through fixed-step crater-before-handoff logic and expose Player Two's ready turn.
3. Complete T011 and validate the first visible alternating artillery turn.

### Incremental Delivery

1. Add US1 for a fully resolving Player One → Player Two turn.
2. Add US2 for both players' retained artillery setups.
3. Add US3 for non-impact completion, resolving input rejection, and deterministic long play.
4. Finish documentation, roadmap, and quality checks after demonstrated completion.
