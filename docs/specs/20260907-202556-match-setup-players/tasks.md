---

description: "Actionable task list for Match Setup — Named 2–8 Player Matches and Controller Slots"
---

# Tasks: Match Setup — Named 2–8 Player Matches and Controller Slots

**Input**: Design documents from docs/specs/20260907-202556-match-setup-players/

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/match-setup.md, quickstart.md

**Tests**: Required by the feature specification. Write focused domain tests beside their source modules before or with the corresponding implementation, then run the full workspace checks in the final phase.

**Organization**: Tasks are grouped by user story after a shared configuration and N-player foundation, so each story can be tested as a meaningful increment.

## Format: [ID] [P?] [Story] Description

- **[P]**: Task can proceed in parallel once its stated dependencies are complete and it edits a different file.
- **[USn]**: Task maps to the corresponding feature specification user story.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish the implementation boundary and retain a reproducible baseline before changing the fixed duel.

- [ ] T001 Inspect and record every fixed-two-player assumption in crates/azimuth-game/src/main.rs, tank.rs, turn.rs, weapon.rs, and combat.rs as implementation notes in docs/specs/20260907-202556-match-setup-players/plan.md.
- [ ] T002 Run the existing baseline quality suite from Cargo.toml: cargo fmt --all -- --check, cargo test --workspace, cargo clippy --workspace --all-targets -- -D warnings, and cargo build --workspace.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Create the authoritative configuration boundary and replace hard-coded duel-only domain representations. No user-story UI or game-flow work starts until this phase is complete.

- [ ] T003 Create MatchConfiguration, PlayerConfiguration, ControllerType, PlayerVisualIdentity, validation errors, player-count constants, default human slots, and configuration tests in crates/azimuth-game/src/match_setup.rs.
- [ ] T004 Implement setup-only curated AI-name pool selection with preferred in-configuration uniqueness and injectable/testable randomness in crates/azimuth-game/src/match_setup.rs.
- [ ] T005 Add validation tests in crates/azimuth-game/src/match_setup.rs for counts 2–8, invalid counts, unique stable IDs, valid visual/controller values, trimmed non-empty maximum-20-character human names, and display-name/identity separation.
- [ ] T006 Evolve PlayerId into an ordered stable identity suitable for 2–8 participants, remove duel-only other-player semantics, and make deterministic spawn construction return one supported tank per configured participant in crates/azimuth-game/src/tank.rs.
- [ ] T007 Add tank/spawn tests in crates/azimuth-game/src/tank.rs for representative 2-, 3-, 4-, and 8-player deterministic, in-bounds, supported, non-overlapping spawn layouts.
- [ ] T008 Replace fixed [Tank; 2] explosion damage traversal with collection-based all-tank traversal and preserve damage attribution in crates/azimuth-game/src/combat.rs.
- [ ] T009 Add multiplayer combat tests in crates/azimuth-game/src/combat.rs for damage to several tanks, self-damage, and all affected tanks receiving consequences.
- [ ] T010 Refactor PlayerWeaponLoadouts from two fixed loadouts to identity-addressed independent participant loadouts in crates/azimuth-game/src/weapon.rs.
- [ ] T011 Add loadout tests in crates/azimuth-game/src/weapon.rs covering standard three-weapon initialisation for 2/8 players and independent selection/finite ammunition consumption.
- [ ] T012 Refactor TurnState from player-one/player-two aim fields and toggling to ordered player identities, per-player aim lookup, wrapping rotation, and next-living-player selection in crates/azimuth-game/src/turn.rs.
- [ ] T013 Add turn-domain tests in crates/azimuth-game/src/turn.rs for 2-, 3-, and 8-player order, wrapping, one/multiple eliminated-player skipping, per-player aim independence, and no active turn after completion.
- [ ] T014 Update shared type imports, module declarations, and existing two-player tests to compile against collection-based player/tank/turn/loadout APIs in crates/azimuth-game/src/main.rs, tank.rs, turn.rs, weapon.rs, and combat.rs.

**Checkpoint**: Configuration is engine-independent and all core domain paths accept real participant collections. Existing two-player behaviour remains representable as a two-slot configuration.

---

## Phase 3: User Story 1 - Configure and Start a Human Match (Priority: P1) 🎯 MVP

**Goal**: Launch into Match Setup, configure a valid all-human 2–8 player match with editable names and visible colours, and start exactly that authoritative configuration.

**Independent Test**: For every count from 2 through 8, create an all-human configuration, edit at least one name, start the match, and verify exactly those named identities/tanks enter gameplay.

### Tests for User Story 1

- [ ] T015 [US1] Add match-creation tests in crates/azimuth-game/src/main.rs proving valid all-human configurations for 2, 3, 4, and 8 slots create exactly matching participant metadata, tanks, default aim/movement, and standard loadouts.
- [ ] T016 [US1] Add setup-to-runtime contract tests in crates/azimuth-game/src/main.rs proving edited valid names survive creation without changing PlayerId or visual identity and invalid configuration cannot initialise gameplay.

### Implementation for User Story 1

- [ ] T017 [US1] Add explicit MatchSetup and Playing application states plus resources that hold prospective configuration separately from running state in crates/azimuth-game/src/main.rs.
- [X] T018 [US1] Implement compact Match Setup screen construction and teardown, including the default two-human configuration and a clearly visible player-count control, in crates/azimuth-game/src/main.rs.
- [X] T019 [US1] Implement dynamic Human slot rows showing slot number, curated colour identity, editable name, and Human/AI controller control in crates/azimuth-game/src/main.rs.
- [X] T020 [US1] Wire player-count and human-name UI events through MatchConfiguration mutation/validation, ensuring increasing/decreasing counts produces exactly the active slots and no hidden runtime players, in crates/azimuth-game/src/main.rs.
- [X] T021 [US1] Implement Start Match validation and conversion from a valid all-human MatchConfiguration into dynamic tanks, turn state, weapon loadouts, aiming state, and presentation resources in crates/azimuth-game/src/main.rs.
- [X] T022 [US1] Replace immediate fixed-duel startup and player-one/player-two initialisation with the configuration-driven startup path in crates/azimuth-game/src/main.rs.

**Checkpoint**: A user can launch, configure, and start a named 2–8 human match; invalid configurations do not reach gameplay.

---

## Phase 4: User Story 2 - Configure Future AI Slots Honestly (Priority: P1)

**Goal**: Let setup represent Human/AI controller choice, automatically assign playful stable AI names, and clearly prevent an AI match from starting until an AI controller exists.

**Independent Test**: Toggle several slots to AI, verify unique pool-derived names and preserved identity/colour, confirm Start is blocked with the stated reason, then return a slot to an editable Human state.

### Tests for User Story 2

- [ ] T023 [US2] Add controller-transition tests in crates/azimuth-game/src/match_setup.rs for Human→AI naming, preferred unique names, AI→Human editable defaults/history, identity/visual preservation, and setup-only name randomness.
- [ ] T024 [US2] Add validation tests in crates/azimuth-game/src/match_setup.rs proving AI-containing configurations report the explicit unavailable-AI start error while valid all-human configurations remain startable.

### Implementation for User Story 2

- [X] T025 [US2] Wire Human/AI controls in Match Setup to ControllerType transitions and generated AI names from the configuration domain in crates/azimuth-game/src/main.rs.
- [X] T026 [US2] Render the concise AI-not-yet-available explanation and disable Start Match whenever MatchConfiguration validation reports AI unavailability in crates/azimuth-game/src/main.rs.
- [X] T027 [US2] Implement an optional compact AI-name reroll control only if it fits the existing setup UI without complexity; otherwise record its deferral in docs/roadmap.md.

**Checkpoint**: AI is visible and honestly configured without a parallel player model or a match that can become stuck.

---

## Phase 5: User Story 3 - Play a Configured N-Player Match (Priority: P1)

**Goal**: Run ordinary move-or-fire turns over all configured humans, with completely independent player state and no two-player-only action path.

**Independent Test**: Play representative 2-, 3-, 4-, and 8-human matches; verify slot-order wrapping and that aim, movement, selected weapon, and finite ammunition of one player never alter another's.

### Tests for User Story 3

- [ ] T028 [US3] Add game-flow tests in crates/azimuth-game/src/main.rs for configured participant turn rotation, active-human-only input, and independent aim/movement/selection/inventory state in an eight-player match.
- [ ] T029 [US3] Add regression tests in crates/azimuth-game/src/main.rs proving Basic Shell, High Explosive, and Heavy Shell fire through the existing shared projectile path for configuration-created two- and eight-player matches.

### Implementation for User Story 3

- [ ] T030 [US3] Replace fixed Tanks([Tank; 2]) resources, two-player material selection, and player-name helpers with collection/identity lookup in crates/azimuth-game/src/main.rs.
- [ ] T031 [US3] Refactor input, aiming, movement, weapon cycling, firing, and projectile ownership systems to resolve the active PlayerId against collection-backed state only in crates/azimuth-game/src/main.rs.
- [ ] T032 [US3] Refactor projectile impact, terrain deformation, and tank-support settling orchestration to process every relevant surviving tank before resolving the turn in crates/azimuth-game/src/main.rs.
- [ ] T033 [US3] Remove residual player1/player2, two-element, and opponent-is-other assumptions from crates/azimuth-game/src/main.rs, tank.rs, turn.rs, weapon.rs, and combat.rs.

**Checkpoint**: A valid configuration drives one ordinary 2–8-player game path with independent player state and existing weapons/movement/projectiles intact.

---

## Phase 6: User Story 4 - Resolve Multiplayer Elimination and Victory (Priority: P2)

**Goal**: Resolve every consequence of a multiplayer shot before skipping eliminated players, declaring any named survivor winner, or declaring a draw.

**Independent Test**: Trigger multi-target, multiple-elimination, and self-elimination cases and verify all damage/deformation/settling completes before the next active identity, winner, or draw is chosen.

### Tests for User Story 4

- [ ] T034 [US4] Add end-to-end resolution tests in crates/azimuth-game/src/main.rs for several affected tanks, simultaneous eliminations, self-elimination, all survivors, one arbitrary winning identity, zero-survivor draw, and post-match action prevention.
- [ ] T035 [US4] Add settling-order regression tests in crates/azimuth-game/src/main.rs proving terrain support is resolved for every surviving affected tank before survivor/turn/result evaluation.

### Implementation for User Story 4

- [ ] T036 [US4] Implement collection-based survivor evaluation, named winner lookup, draw result, and completion gating after full impact/deformation/settling resolution in crates/azimuth-game/src/main.rs.
- [ ] T037 [US4] Ensure turn advancement uses the next living configured identity after complete consequence resolution and make result presentation source its winner display name from configuration metadata in crates/azimuth-game/src/main.rs.

**Checkpoint**: Multi-player damage and terrain consequences produce correct skip, winner, and draw outcomes without “the other player” logic.

---

## Phase 7: User Story 5 - Read a Scalable Named Tactical Match (Priority: P2)

**Goal**: Keep the battlefield primary while making active player, all participant health/alive state, aim, weapons, wind, movement, camera focus, and named result readable for 2–8 players.

**Independent Test**: Inspect 2-, 4-, and 8-player HUDs with eliminations and a non-first active/winning player; every participant remains identifiable and the active player/camera is correct.

### Tests for User Story 5

- [ ] T038 [US5] Add presentation-state tests in crates/azimuth-game/src/main.rs for collection-derived status rows, active-player emphasis, eliminated state, configured winner names, and camera target lookup for later player identities.

### Implementation for User Story 5

- [ ] T039 [US5] Replace fixed two-player HUD health/name panels with a compact dynamically generated 2–8 player status list that preserves active weapon/ammunition, aim, wind, movement, and result information in crates/azimuth-game/src/main.rs.
- [ ] T040 [US5] Update HUD layout/styling and tank visual material lookup so configured colour/name identity remains legible at 2, 4, and 8 players while the battlefield stays visually primary in crates/azimuth-game/src/main.rs.
- [ ] T041 [US5] Refactor turn-boundary camera focus and post-resolution transition targeting to resolve any active surviving PlayerId's tank without authoritatively waiting for camera animation in crates/azimuth-game/src/main.rs.

**Checkpoint**: Presentation derives entirely from configured/running player collections and supports a readable eight-human local match.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Verify no regressions, document only demonstrated outcomes, and prepare the feature for manual sign-off.

- [ ] T042 [P] Audit crates/azimuth-game/src/ for remaining hard-coded Player One/Two text, two-element arrays, and other-player branches; remove only residual paths that violate 2–8 semantics.
- [ ] T043 [P] Update docs/roadmap.md with only Match Setup, human/AI-selection-at-configuration, local multiplayer, scalable-HUD, and battlefield discoveries demonstrated by implementation/manual play; explicitly retain AI Opponents and final battlefield balance work.
- [ ] T044 Run cargo fmt --all, cargo test --workspace, cargo clippy --workspace --all-targets -- -D warnings, and cargo build --workspace from Cargo.toml; fix all feature-caused failures.
- [ ] T045 Execute every manual scenario in docs/specs/20260907-202556-match-setup-players/quickstart.md, including complete 3–8 human matches, and record any nonessential follow-up discoveries in docs/roadmap.md.
- [ ] T046 Review crates/azimuth-game/src/main.rs, match_setup.rs, tank.rs, turn.rs, weapon.rs, and combat.rs against spec.md architecture questions: real collection-based running match, identity distinct from name, controller distinct from player state, and AI-ready ordinary action path.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1** has no dependencies.
- **Phase 2** depends on the baseline and blocks all user stories.
- **US1** depends on Phase 2.
- **US2** depends on Phase 2 and can follow its configuration foundation; it integrates into the US1 setup UI.
- **US3** depends on Phase 2 and the US1 configuration-to-runtime startup path.
- **US4** depends on US3 because it completes its multiplayer resolution path.
- **US5** depends on US3/US4 runtime participant/result state.
- **Polish** depends on every desired story.

### User Story Completion Order

~~~text
Phase 1 → Phase 2 → US1 (start all-human match) → US2 (honest AI slots)
                         └──────────────────────→ US3 (N-player play)
                                                     → US4 (elimination/victory)
                                                     → US5 (HUD/camera)
                                                     → Polish
~~~

### Parallel Opportunities

- T003/T004/T005 can share the new configuration module sequentially; T006/T008/T010/T012 can proceed in parallel after the configuration API shape is agreed because they use separate domain files.
- T007, T009, T011, and T013 can be prepared in parallel with their corresponding domain refactors, but each test must run after its implementation compiles.
- T015 and T016 can proceed together after Phase 2; T018 and T019 can proceed together only after app-state/UI ownership is established.
- T023/T024 can proceed together after the configuration API supports controller transitions.
- T028/T029, T034/T035, and T038 target different test concerns but all edit main.rs; keep them sequential unless split into test modules during implementation.
- T042 and T043 can run in parallel after all functional work.

## Parallel Example: Foundational Domain Refactor

~~~text
After T003 establishes shared configuration types:

Task T006: Generalise PlayerId and deterministic spawns in crates/azimuth-game/src/tank.rs
Task T008: Generalise explosion traversal in crates/azimuth-game/src/combat.rs
Task T010: Generalise loadouts in crates/azimuth-game/src/weapon.rs
Task T012: Generalise turn rotation/aim state in crates/azimuth-game/src/turn.rs
~~~

## Parallel Example: Setup Configuration Coverage

~~~text
After T003/T004:

Task T005: Configuration bounds/name/identity validation tests in crates/azimuth-game/src/match_setup.rs
Task T007: Deterministic spawn tests in crates/azimuth-game/src/tank.rs
Task T011: Independent inventory tests in crates/azimuth-game/src/weapon.rs
Task T013: Turn order/skip tests in crates/azimuth-game/src/turn.rs
~~~

## Implementation Strategy

### MVP First

1. Complete Phase 1 and Phase 2 so a real N-player domain exists.
2. Complete US1 through T022.
3. Validate all-human 2–8 setup/start independently before adding the AI-slot UX or polished HUD.

### Incremental Delivery

1. Foundation: configuration plus dynamic domain state.
2. US1: valid all-human Match Setup and configuration-driven match creation.
3. US2: clearly unavailable AI configuration.
4. US3: full N-player controls, weapons, terrain, and settling.
5. US4: robust multiplayer elimination/victory/draw.
6. US5: scalable tactical HUD and camera.
7. Polish only after complete local-match evidence.

## Format Validation

All 46 tasks follow the required checkbox, sequential task ID, optional parallel marker, required story label for story work, and explicit file-path format.
