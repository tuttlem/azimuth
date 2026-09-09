---
description: "Dependency-ordered tasks for First AI Controller — A Computer Player That Can Complete a Turn"
---

# Tasks: First AI Controller — A Computer Player That Can Complete a Turn

**Input**: Design documents from docs/specs/20260909-192622-ai-controller/

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/ai-controller.md, quickstart.md

**Tests**: Automated deterministic coverage is explicitly required. Write listed focused tests first and confirm each fails for missing behavior before implementation.

**Organization**: Tasks are grouped by user story after the minimal shared authoritative firing boundary. AI movement, advanced weapon evaluation, wind compensation, memory, and solver work are not tasks in this feature.

## Phase 1: Setup

**Purpose**: Record the current baseline and review the existing seam before modification.

- [X] T001 Run the focused game test suite and record baseline results before editing crates/azimuth-game/src/main.rs, crates/azimuth-game/src/match_setup.rs, or adding crates/azimuth-game/src/ai.rs.

---

## Phase 2: Foundational — Shared Authoritative Action Boundary

**Purpose**: Separate tactical input detection from the existing legal current-player fire operation without a generic action framework. This blocks every AI story.

- [ ] T002 Add failing shared-action and controller-gating regression tests in crates/azimuth-game/src/main.rs proving a non-keyboard current-player request follows the same available-weapon commitment, aiming, fire phase, and projectile-flight lifecycle as the Human launch path, while controller-gated tactical input cannot mutate an AI turn.
- [X] T003 Add the smallest legal current-player aim-setting operation in crates/azimuth-game/src/turn.rs and its unit tests, preserving existing aim bounds and requiring an in-progress choosing turn.
- [X] T004 Refactor the launch operation in crates/azimuth-game/src/main.rs into one shared validated current-player fire boundary; retain Human keyboard detection as a source-specific caller, with no duplicated inventory commitment, projectile construction, impact reset, or fire-resolution transition.

**Checkpoint**: A Human and any future controller can request the exact same authoritative firing operation, while presentation input no longer owns launch rules.

---

## Phase 3: User Story 1 - Play Against a Computer Opponent (Priority: P1) 🎯 MVP

**Goal**: Start a mixed Human/AI match and have the active AI choose a living opponent, apply a valid ordinary firing intent, and launch through the shared action boundary.

**Independent Test**: Configure one Human and one AI, complete the Human turn, then verify the AI becomes active, ignores tactical keyboard input, and issues an ordinary shot that enters the existing resolving lifecycle.

### Tests for User Story 1

- [X] T005 [P] [US1] Replace the AI-unavailable validation regression with 2–8 mixed-controller acceptance, identity/colour/name preservation, and all-AI acceptance tests in crates/azimuth-game/src/match_setup.rs.
- [X] T006 [P] [US1] Add pure two-player AI decision tests for living non-self target selection, valid aim, normal Basic Shell request, deterministic same-seed output, and display-name-independent output in crates/azimuth-game/src/ai.rs.
- [ ] T007 [US1] Add an integration-style dispatch test in crates/azimuth-game/src/main.rs for Human-to-AI handoff, AI input isolation, AI intent application, and ordinary shared projectile launch.

### Implementation for User Story 1

- [X] T008 [US1] Remove the AI-start restriction and AI-unavailable setup message while retaining count, identity, name, and controller validation in crates/azimuth-game/src/match_setup.rs and crates/azimuth-game/src/main.rs.
- [X] T009 [US1] Create crates/azimuth-game/src/ai.rs with a pure match-seeded firing-first decision policy: nearest stable-tie-broken living non-self target, valid rough geometry aim, bounded error, ignored wind, and Basic Shell request; expose no rendering, input, projectile, terrain-mutation, or resolution API.
- [X] T010 [US1] Add dedicated match-derived AI decision state, initialise/reset it on configured match start, gate Human tactical input by active configured controller, and dispatch/revalidate/apply the active AI intent before calling the shared launch operation in crates/azimuth-game/src/main.rs.

**Checkpoint**: One Human versus one AI starts, the AI acts without fake keyboard input, and its shot is an ordinary Azimuth shot.

---

## Phase 4: User Story 2 - Watch Consecutive Computer Turns (Priority: P1)

**Goal**: Mixed and all-AI matches continue across ordinary resolution and skip eliminated Human or AI slots without waiting for a Human turn.

**Independent Test**: In deterministic four- and eight-player traces, repeatedly resolve ordinary AI firing turns and verify AI-to-AI, AI-to-Human, and Human-to-AI rotation reaches only living configured players and ends normally.

### Tests for User Story 2

- [ ] T011 [US2] Add deterministic 4-player and 8-player controller-rotation regressions in crates/azimuth-game/src/main.rs for consecutive AI dispatch, Human/AI handoffs, eliminated Human/AI skipping, stale-decision discard, and no AI action after winner or draw.
- [ ] T012 [US2] Add all-AI autonomous-progress coverage using the ordinary decision/action/turn lifecycle, without elapsed-time dependence or a graphical test, in crates/azimuth-game/src/main.rs.

### Implementation for User Story 2

- [ ] T013 [US2] Tighten AI readiness and dispatch invalidation in crates/azimuth-game/src/main.rs so resolving flight, settling, changed current player/controller, eliminated actor/target, unavailable selection, and finished match cannot leave a stale queued AI action or a turn waiting for Human input.

**Checkpoint**: A mixed or eight-AI match can visibly progress through consecutive ordinary turns and never selects an eliminated or finished participant.

---

## Phase 5: User Story 3 - Recognise Imperfect Artillery Decisions (Priority: P2)

**Goal**: AI shots visibly attempt to reach living opponents without becoming exact solvers or random nonsense.

**Independent Test**: Pure representative near, far, uphill, downhill, and tie-distance decision tests prove valid target-directed azimuth, monotonic distance tendency, bounded deterministic error, and ignored-wind policy.

### Tests for User Story 3

- [ ] T014 [P] [US3] Extend crates/azimuth-game/src/ai.rs tests for 2-, 4-, and 8-player living target sets, one remaining opponent, self/eliminated exclusion, stable tie ordering, valid aim bounds, near/far power tendency, same-seed repeatability, suitable-seed variation, and bounded azimuth/elevation/power error.
- [X] T015 [US3] Tune only named baseline decision constants and explanatory intent comments in crates/azimuth-game/src/ai.rs until pure tests demonstrate broadly target-directed imperfect shots that intentionally ignore wind and contain no analytic or search-based solver.

**Checkpoint**: The first controller is legible and fallible: target-directed enough to be fun to watch, yet still exposed to terrain and wind.

---

## Phase 6: User Story 4 - Preserve Ordinary Weapons and Human Play (Priority: P2)

**Goal**: Both controller types remain bound to the one ordinary action, inventory, projectile, and turn model; all-Human behavior is preserved.

**Independent Test**: Compare Human and AI requests through the shared fire path, validate normal limited-ammunition semantics, and run existing all-Human aiming/movement/fire/impact regressions.

### Tests for User Story 4

- [ ] T016 [US4] Add shared-path regressions in crates/azimuth-game/src/main.rs and crates/azimuth-game/src/weapon.rs proving AI-selected available weapons use ordinary commitment/fallback and projectile profiles, while unavailable weapons cannot be committed.
- [ ] T017 [US4] Add all-Human controller-dispatch regression coverage in crates/azimuth-game/src/main.rs proving Human aim, selection, movement, firing, projectile resolution, and existing movement/firing handoffs remain available and unchanged.

### Implementation for User Story 4

- [X] T018 [US4] Review the shared action and controller guards in crates/azimuth-game/src/main.rs, crates/azimuth-game/src/turn.rs, and crates/azimuth-game/src/weapon.rs to remove controller-specific mutation paths, preserve normal availability/fallback/profile behavior, and retain the Human move-or-fire flow.

**Checkpoint**: Controller type changes decision source only; it never changes weapon, projectile, damage, terrain, or match rules.

---

## Phase 7: Polish, Documentation, and Acceptance

**Purpose**: Demonstrate only completed roadmap work, validate quality, and play intended scenarios without expanding into smarter AI.

- [X] T019 Update demonstrated Match Setup, Basic AI, and Milestone H items only; record useful nonessential AI discoveries while retaining wind compensation, shot memory, bracketing, tactical movement, weapon-quality, and personality work in docs/roadmap.md.
- [X] T020 Run formatting, focused/full tests, workspace build, and lint checks from docs/specs/20260909-192622-ai-controller/quickstart.md; fix only feature-caused failures in crates/azimuth-game/src/.
- [ ] T021 Perform and record the one-Human/one-AI, one-Human/several-AI, eight-AI, and all-Human manual acceptance scenarios in docs/specs/20260909-192622-ai-controller/quickstart.md, including readable turn/camera/HUD behavior, ordinary flight/impact, eliminated-slot rotation, match completion, and entertaining imperfect-shot evidence.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1**: Starts immediately.
- **Phase 2**: Depends on T001 and blocks all story work; T002 guides T003–T004.
- **US1 (Phase 3)**: Depends on T004. T005 and T006 may run in parallel; T007 follows their shared requirements; T008–T010 implement in order.
- **US2 (Phase 4)**: Depends on US1 shared dispatch. T011 and T012 may run in parallel; T013 follows them.
- **US3 (Phase 5)**: Depends on T009. T014 precedes tuning T015.
- **US4 (Phase 6)**: Depends on T004 and T010. T016 and T017 may run in parallel; T018 follows.
- **Phase 7**: Depends on all desired user-story checkpoints. T019 and T020 may be prepared in parallel; T021 follows a successful build and test suite.

### User Story Dependencies

- **US1** is the MVP and introduces a start-valid, firing-first AI.
- **US2** extends that controller to consecutive and all-AI turns; it depends on US1's dispatcher.
- **US3** adds decision-quality coverage/tuning to US1's policy without smarter behavior.
- **US4** protects the shared action contract and all-Human play; it depends on the extracted shared boundary and controller dispatch.

### Parallel Opportunities

- T005 and T006 use separate source files and can run in parallel.
- T011 and T012 are independent trace scenarios in the same target file; integrate them serially.
- T014 can proceed after T009 while T011–T013 complete.
- T016 and T017 target separate behavior concerns and can be designed in parallel.
- Documentation review T019 can begin once implementation evidence exists while T020 runs quality checks, but roadmap checkboxes wait for demonstrated/manual evidence.

## Parallel Example: User Story 1

Task: "Replace AI-unavailable validation with mixed-controller acceptance tests in crates/azimuth-game/src/match_setup.rs"

Task: "Add pure two-player deterministic AI decision tests in crates/azimuth-game/src/ai.rs"

## Implementation Strategy

### MVP First

1. Record the baseline and extract the shared current-player firing operation.
2. Make AI configuration start-valid.
3. Implement only pure nearest-living-target, bounded-error, Basic-Shell decisions.
4. Dispatch a valid active AI decision through the shared action boundary.
5. Stop and validate one Human-versus-one-AI before adding all-AI hardening or tuning.

### Incremental Delivery

1. Setup + foundational shared launch boundary.
2. US1 produces a playable first opponent.
3. US2 proves controller rotation and autonomous spectator play.
4. US3 verifies/tunes intentional imperfection.
5. US4 protects ordinary player and weapon behavior.
6. Perform quality checks and manual matches; use evidence to update the roadmap without beginning the next AI-memory feature.

## Notes

- Every task uses the required checkbox, sequential ID, applicable story label, and exact file path.
- [P] tasks are distinct enough to plan independently; shared main.rs edits still require careful ordered integration.
- Do not add movement, a perfect solver, or an AI framework while completing this list.
