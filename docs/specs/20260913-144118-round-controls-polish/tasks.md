---

description: "Actionable task list for Round, Controls and Battlefield Polish"
---

# Tasks: Round, Controls and Battlefield Polish

**Input**: Design documents in `docs/specs/20260913-144118-round-controls-polish/`

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [game-ui.md](contracts/game-ui.md), and [quickstart.md](quickstart.md)

**Tests**: Required by FR-015 and the project constitution. Add focused module tests before or alongside each change, then run the final workspace and manual checks.

**Organization**: Tasks are grouped by user story. Each story remains independently testable once its phase is complete.

## Phase 1: Setup

**Purpose**: Establish a reproducible baseline and inspect the affected existing behaviour before changing it.

- [X] T001 Run the current focused and workspace baseline (`cargo test --workspace`) from `Cargo.toml` and record the result in `docs/specs/20260913-144118-round-controls-polish/quickstart.md`.
- [X] T002 Review the established UI/input and simulation boundaries in `docs/specs/20260913-144118-round-controls-polish/contracts/game-ui.md` before editing `crates/azimuth-game/src/main.rs`, `crates/azimuth-game/src/turn.rs`, `crates/azimuth-game/src/tank.rs`, `crates/azimuth-game/src/weapon.rs`, or `crates/azimuth-game/src/battlefield.rs`.

---

## Phase 2: Foundational

**Purpose**: Preserve the shared authority boundaries that all changes rely on.

- [X] T003 Verify and retain `GameSession`/`RoundEarnings` as the sole accounting and purchase authority in `crates/azimuth-game/src/session.rs`; do not add presentation-owned copies of cash, earnings, prices, or phase.
- [X] T004 Verify and retain the authoritative square terrain/bounds boundary in `crates/azimuth-game/src/battlefield.rs`; visual-horizon changes must not affect terrain queries, collision, spawning, or movement validity.

**Checkpoint**: Authority boundaries are explicit; user-story changes may proceed.

---

## Phase 3: User Story 1 - Review a Complete Multiplayer Round (Priority: P1) 🎯 MVP

**Goal**: Display all player earnings at once in an attractive, readable accounting view for every supported 2–8 player match.

**Independent Test**: Complete controlled 2-, 6-, 7-, and 8-player rounds at 1280×720 and wider; verify each player’s identity, damage, placement, total, and wallet are simultaneously visible before Enter continues to the shop.

- [X] T005 [US1] Add pure accounting-row/view-model formatting and 2/6/7/8-player mapping regressions in `crates/azimuth-game/src/main.rs`, including stable player-to-earnings joining and use of `RoundEarnings::total()`.
- [X] T006 [US1] Replace the single multiline accounting text overlay with a fixed themed title, header, up-to-eight compact rows, and continuation hint in `crates/azimuth-game/src/main.rs`.
- [X] T007 [US1] Synchronise accounting row text, player colours, row visibility, and Accounting-phase overlay visibility from `GameSession.players` and `GameSession.earnings` in `crates/azimuth-game/src/main.rs`, while preserving `advance_session_flow` Enter progression and blocking battlefield controls.
- [ ] T008 [US1] Manually verify the accounting layout and continuation flow for 2, 6, 7, and 8 players using `docs/specs/20260913-144118-round-controls-polish/quickstart.md`.

**Checkpoint**: User Story 1 is independently complete and demonstrable.

---

## Phase 4: User Story 2 - Buy a Varied Non-Nuclear Arsenal (Priority: P1)

**Goal**: Make every ordinary limited weapon affordable while preserving Nuke scarcity and exact atomic purchases.

**Independent Test**: Inspect the canonical table and shop with controlled balances; nine non-Nuke prices are one tenth of prior values, Nuke is $15,000, Basic Shell remains unavailable, and accepted/rejected purchases preserve exact cash/ammunition behaviour.

- [X] T009 [US2] Add a table-driven price regression covering all `SHOP_WEAPONS`, nine exact discounted ordinary prices, Nuke at $15,000, Nuke’s greater-than-twentyfold separation, and Basic Shell’s absent price in `crates/azimuth-game/src/weapon.rs`.
- [X] T010 [US2] Change only the nine non-Nuke entries in `weapon_price` in `crates/azimuth-game/src/weapon.rs` to their exact one-tenth prices, retaining Basic Shell `None` and Nuke $15,000.
- [X] T011 [US2] Verify shop item display and atomic charging continue to derive the same changed canonical price through existing `weapon_shop_items`/purchase tests in `crates/azimuth-game/src/session.rs`.
- [ ] T012 [US2] Manually verify ordinary/Nuke affordability and successful/rejected purchase behaviour using `docs/specs/20260913-144118-round-controls-polish/quickstart.md`.

**Checkpoint**: User Story 2 is independently complete and demonstrable.

---

## Phase 5: User Story 3 - Move Freely Until Ending the Turn (Priority: P1)

**Goal**: Let the active player make unlimited in-bounds cardinal terrain-following steps and end with Space or Enter, without a slope gate or accidental Space-fired shot.

**Independent Test**: On flat, steep, and cratered terrain, make 7 and 20 consecutive in-bounds moves, reject an out-of-bounds move unchanged, then separately end with Enter and Space without firing.

- [X] T013 [P] [US3] Replace budgeted `TurnPhase::Moving` state with fieldless move mode and add unlimited-step/explicit-handoff/skip-eliminated regression coverage in `crates/azimuth-game/src/turn.rs`.
- [X] T014 [P] [US3] Remove movement allowance and slope rejection from `Tank::step_on_terrain`, retain bounds-only rejection and terrain-height grounding, and replace slope tests with steep/crater acceptance regressions in `crates/azimuth-game/src/tank.rs`.
- [X] T015 [US3] Update movement HUD view data and controls text to describe unlimited move mode, bounds feedback only, and `SPACE/ENTER END` in `crates/azimuth-game/src/main.rs`.
- [X] T016 [US3] Update movement input in `crates/azimuth-game/src/main.rs` to use fieldless move mode, accept both Space and Enter as completion, keep one camera-relative cardinal request at a time, and prevent a Space completion from firing the next player’s weapon in that frame.
- [X] T017 [US3] Add `main.rs` regressions for both completion keys, Space non-firing handoff, unlimited HUD state, and unchanged camera-relative arrow mapping in `crates/azimuth-game/src/main.rs`.
- [ ] T018 [US3] Manually validate 20-step steep/crater movement, bounds rejection, both completion keys, and preserved move-only restrictions using `docs/specs/20260913-144118-round-controls-polish/quickstart.md`.

**Checkpoint**: User Story 3 is independently complete and demonstrable.

---

## Phase 6: User Story 4 - Operate Setup and Battlefield Cues Comfortably (Priority: P2)

**Goal**: Make controller toggling ergonomic and make wind/battlefield cues visually legible without changing their gameplay meaning.

**Independent Test**: Toggle several setup slots with Tab, inspect all wind directions over contrasting views, and inspect all four terrain edges/corners while confirming bounds remain unchanged.

- [X] T019 [US4] Replace the Control-C controller-toggle trigger and visible guidance with Tab in `crates/azimuth-game/src/main.rs`, retaining selected-slot targeting, Control-R behaviour, and `MatchConfiguration::set_controller` semantics.
- [X] T020 [US4] Add focused setup-input mapping coverage proving Tab changes only the selected slot controller and preserves count/name/identity/visual state in `crates/azimuth-game/src/main.rs`.
- [X] T021 [US4] Increase wind-arrow shaft/head contrast through its existing dedicated material/light configuration and add a focused presentation-configuration regression while preserving rotation/camera tests in `crates/azimuth-game/src/main.rs`.
- [X] T022 [P] [US4] Match `VisualHorizon` inner-ring colours to terrain surface colour, blend outward deterministically, retain render-only bounds, and update visual-horizon regressions in `crates/azimuth-game/src/battlefield.rs`.
- [X] T023 [US4] Use one terrain-compatible horizon material for initial scene creation and later-round rebuilding in `crates/azimuth-game/src/main.rs`, without changing mesh extent or terrain authority.
- [ ] T024 [US4] Manually validate Tab behaviour, wind-arrow visibility, and all-edge/corner blending against the unchanged movement boundary using `docs/specs/20260913-144118-round-controls-polish/quickstart.md`.

**Checkpoint**: All user stories are independently complete and demonstrable.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Verify the complete feature, preserve repository health, and record only evidence-backed documentation changes.

- [X] T025 Run `cargo fmt --all -- --check`, `cargo test --workspace`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo check --workspace --all-targets`, and `cargo build --workspace` from `Cargo.toml`; resolve feature-caused failures in `crates/azimuth-game/src/`.
- [ ] T026 Re-run every scenario in `docs/specs/20260913-144118-round-controls-polish/quickstart.md` and record actual automated/manual evidence in that file.
- [ ] T027 Review and update only fulfilled checklist items in `docs/roadmap.md`, then update affected control/movement documentation in `README.md` or `docs/` if it describes replaced Control-C, allowance, slope, or completion-key behaviour.
- [ ] T028 Review `docs/specs/20260913-144118-round-controls-polish/spec.md`, `plan.md`, `data-model.md`, and `contracts/game-ui.md` against the delivered implementation; record nonessential balance, art, or movement discoveries in `docs/roadmap.md` rather than expanding scope.

---

## Dependencies & Execution Order

### Phase Dependencies

- Phase 1 → Phase 2 → each user-story phase → Phase 7.
- US1, US2, and US3 are independent after Phase 2 and may be implemented in any order; US4 is likewise independent except that T023 follows its horizon decision in T022.
- Within US3, T013 and T014 may proceed in parallel; T015–T017 follow their resulting interfaces.
- Within US4, T022 can proceed in parallel with T019–T021; T023 follows T022.

### User Story Dependencies

- **US1 (P1)**: No dependency on other stories; MVP candidate.
- **US2 (P1)**: No dependency on other stories; uses existing price-to-session contract.
- **US3 (P1)**: No dependency on other stories; modifies authoritative move state/terrain validation.
- **US4 (P2)**: No dependency on other stories; presentation/input-only except its explicit render-only horizon invariant.

### Parallel Opportunities

- T013 and T014 touch independent domain modules and can run together.
- T022 touches `battlefield.rs` and can run alongside T019–T021 in `main.rs`.
- After Phase 2, one developer can take each of US1, US2, and US3; US4 can be assigned separately if main.rs coordination is managed.

## Parallel Examples

### User Story 3

```text
Task: "T013 fieldless movement state/tests in crates/azimuth-game/src/turn.rs"
Task: "T014 bounds-only terrain movement/tests in crates/azimuth-game/src/tank.rs"
```

### User Story 4

```text
Task: "T019–T021 setup/wind work in crates/azimuth-game/src/main.rs"
Task: "T022 horizon colour blending/tests in crates/azimuth-game/src/battlefield.rs"
```

## Implementation Strategy

### MVP First

1. Complete Phases 1–2.
2. Complete US1 (T005–T008).
3. Validate its independent 2–8 accounting scenario before continuing.

### Incremental Delivery

1. Deliver US1 for usable multiplayer accounting.
2. Deliver US2 for immediately useful economy tuning.
3. Deliver US3 for the gameplay-control change and its safety regressions.
4. Deliver US4 for setup and visual polish.
5. Finish with Phase 7 quality and documentation evidence.

## Format Validation

All 28 tasks use the required `- [ ] T### [P?] [US?] description with exact path` checklist format. User-story tasks carry the required story label; setup, foundational, and cross-cutting tasks intentionally do not.
