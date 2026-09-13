---

description: "Actionable tasks for AI Arsenal Tactics"
---

# Tasks: AI Arsenal Tactics

**Input**: Design documents from `/docs/specs/20260913-155853-ai-arsenal-tactics/`

**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [AI behaviour contract](./contracts/ai-behaviour.md), and [quickstart.md](./quickstart.md)

**Tests**: Automated coverage is required by FR-012. Write focused tests with or before each policy boundary, then run the full workspace gates in the final phase.

**Organization**: Tasks are grouped by user story. The shared AI-difficulty configuration is foundational because tactical and shopping policies both consume it.

## Phase 1: Setup

**Purpose**: Establish a clean, reproducible implementation baseline.

- [X] T001 Verify the current workspace test baseline and relevant AI/setup/shop test locations with `Cargo.toml`, `crates/azimuth-game/src/ai.rs`, `crates/azimuth-game/src/match_setup.rs`, `crates/azimuth-game/src/session.rs`, and `crates/azimuth-game/src/main.rs`

---

## Phase 2: Foundational Configuration

**Purpose**: Add the durable per-slot difficulty data required by all AI behaviour.

**⚠️ CRITICAL**: Complete this phase before tactical or shopping work.

- [X] T002 Add the `Easy`/`Normal`/`Hard` difficulty value, Normal default, cycling/label behaviour, and per-player configuration retention tests in `crates/azimuth-game/src/match_setup.rs`
- [X] T003 Verify configuration/session cloning retains each AI slot's difficulty across new-round setup in `crates/azimuth-game/src/main.rs` tests

**Checkpoint**: Each configured participant has a stable, valid difficulty available to later policy work.

---

## Phase 3: User Story 1 - Set an Opponent's Challenge Level (Priority: P1) 🎯 MVP

**Goal**: A player can view and change an individual AI opponent's difficulty in match setup without disturbing Human setup controls.

**Independent Test**: Configure every supported slot as AI, cycle Easy/Normal/Hard using the displayed keyboard command, toggle a slot Human then AI, and start a match while confirming the chosen setting persists.

### Tests for User Story 1

- [X] T004 [US1] Add focused setup-input regression coverage for AI-only difficulty cycling, Human name-entry isolation, and controller-toggle retention in `crates/azimuth-game/src/main.rs`

### Implementation for User Story 1

- [X] T005 [US1] Implement contextual `D` difficulty cycling for the selected AI slot and preserve existing setup input precedence in `crates/azimuth-game/src/main.rs`
- [X] T006 [US1] Render the selected AI difficulty in setup rows and update visible setup guidance in `crates/azimuth-game/src/main.rs`

**Checkpoint**: Difficulty selection is usable and independently validated before an AI decision consumes it.

---

## Phase 4: User Story 2 - Face AI That Uses Its Arsenal Sensibly (Priority: P1)

**Goal**: AI selects legal owned weapons based on visible range, grouping, and wind conditions instead of always firing Basic Shell.

**Independent Test**: Supply controlled loadouts and tank arrangements for close, long, grouped, windy, and depleted cases; verify a chosen weapon is legal, ammunition is only consumed by a successful shared fire, and suitable scenarios choose more than one limited weapon type.

### Tests for User Story 2

- [X] T007 [P] [US2] Add pure AI policy tests for deterministic target/weapon selection, legal/depleted fallback, grouped-target choices, wind-aware Heavy Shell choices, and Basic Shell fallback in `crates/azimuth-game/src/ai.rs`
- [X] T008 [P] [US2] Add runtime regression coverage that an AI proposal is selected through the existing loadout and shared firing boundary without premature ammunition consumption in `crates/azimuth-game/src/main.rs`

### Implementation for User Story 2

- [X] T009 [US2] Extend the pure firing-decision inputs and tactical context with configured difficulty, public wind, and the acting player's loadout in `crates/azimuth-game/src/ai.rs`
- [X] T010 [US2] Implement the fixed, deterministic Normal tactical ranking for Basic, HE, Heavy, MIRV, Cluster, and Roller roles with Nuke and unsupported terrain-special heuristics excluded in `crates/azimuth-game/src/ai.rs`
- [X] T011 [US2] Pass the active AI's stored difficulty, current wind, and authoritative player loadout into the tactical policy while retaining the existing shared fire path in `crates/azimuth-game/src/main.rs`

**Checkpoint**: A Normal AI can use a varied owned arsenal fairly and reproducibly.

---

## Phase 5: User Story 3 - Meet Opponents Whose Judgement Matches Difficulty (Priority: P2)

**Goal**: Easy, Normal, and Hard are visibly ordered in tactical consistency while all use the same public game rules.

**Independent Test**: Repeat identical seeded tactical contexts across all three levels and confirm ordered error/ranking quality, deterministic repeated results, and unchanged legal-action restrictions.

### Tests for User Story 3

- [X] T012 [US3] Add seeded cross-difficulty regression tests for aim-error bounds, tactical-ranking ordering, legal fallback, and repeatability in `crates/azimuth-game/src/ai.rs`

### Implementation for User Story 3

- [X] T013 [US3] Apply Easy/Normal/Hard bounded-error and ranking differences to the pure tactical decision without adding hidden information, extra actions, resources, or altered physics in `crates/azimuth-game/src/ai.rs`
- [X] T014 [US3] Verify runtime AI orchestration reads only the active slot's difficulty and cannot be perturbed by setup or presentation state in `crates/azimuth-game/src/main.rs`

**Checkpoint**: Mixed-difficulty AI matches are fair, deterministic, and clearly differentiated.

---

## Phase 6: User Story 4 - Watch AI Participate in the Between-Round Shop (Priority: P2)

**Goal**: AI spends its own earnings on a bounded, useful arsenal through the same safe shop transaction as humans.

**Independent Test**: Run zero-cash, insufficient-cash, and multi-purchase shop turns for every difficulty; confirm affordable limited purchases only, no negative or cross-player changes, and uninterrupted progression to the next shopper/round.

### Tests for User Story 4

- [X] T015 [P] [US4] Add pure deterministic AI shopping-plan tests for zero/insufficient funds, affordable fallback, bounded difficulty-specific counts, varied inventory, and exclusion of Basic Shell/Nuke in `crates/azimuth-game/src/ai.rs`
- [X] T016 [P] [US4] Add session-flow regression coverage for AI purchases through the active-shopper transaction, human isolation, and guaranteed shopper completion in `crates/azimuth-game/src/main.rs`

### Implementation for User Story 4

- [X] T017 [US4] Implement a pure state-derived difficulty-aware shopping plan that returns only bounded candidate limited weapons from the existing catalogue in `crates/azimuth-game/src/ai.rs`
- [X] T018 [US4] Replace the fixed AI shop loop by applying every planned candidate through `GameSession::purchase` and completing the active AI shopper exactly once in `crates/azimuth-game/src/main.rs`
- [X] T019 [US4] Preserve and extend atomic purchase/inventory invariants required by AI shopping in `crates/azimuth-game/src/session.rs`

**Checkpoint**: AI joins the round-to-round economy without bypassing affordability, inventory, or Human shopping rules.

---

## Phase 7: Polish and Cross-Cutting Validation

**Purpose**: Complete user-facing guidance, documentation, roadmap alignment, and quality evidence.

- [X] T020 [P] Update AI/difficulty/shop user guidance and manual validation evidence in `docs/specs/20260913-155853-ai-arsenal-tactics/quickstart.md`
- [X] T021 Update verified AI, match setup, shop, and complete-match checkboxes while preserving future work in `docs/roadmap.md`
- [X] T022 Run `cargo fmt --all -- --check`, `cargo test --workspace`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo build --workspace` from `Cargo.toml`
- [ ] T023 Perform and record the 2–8 player mixed-difficulty setup, tactical, shopping, and seeded-repeat manual acceptance matrix in `docs/specs/20260913-155853-ai-arsenal-tactics/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1** → **Phase 2**: establish baseline, then create shared durable difficulty data.
- **US1** starts after Phase 2 and delivers setup configuration independently.
- **US2** starts after Phase 2; it consumes the difficulty data but does not require setup UI completion.
- **US3** depends on US2 because it refines the tactical decision already introduced there.
- **US4** starts after Phase 2 and can proceed in parallel with US2/US3; it consumes difficulty but not tactical-fire implementation.
- **Phase 7** depends on every desired story.

### User Story Dependency Graph

```text
Foundational difficulty data
├── US1: setup selection (MVP)
├── US2: weapon-aware tactical AI ──> US3: differentiated tactical difficulty
└── US4: difficulty-aware AI shopping
```

### Parallel Opportunities

- T007 and T008 cover separate policy/runtime files and can proceed in parallel after foundational data exists.
- T015 and T016 cover separate policy/runtime files and can proceed in parallel after foundational data exists.
- US4 can proceed alongside US2/US3 after T002–T003.
- T020 can be prepared while final implementation work is under review; it must be updated with actual results before completion.

## Implementation Strategy

### MVP First

1. Complete T001–T003.
2. Complete US1 (T004–T006) and manually verify per-slot difficulty selection.
3. Stop here if only setup configuration is needed; the configuration is usable independently.

### Incremental Delivery

1. Add US2 to make Normal AI use the existing arsenal.
2. Add US3 to make difficulty labels tactically meaningful.
3. Add US4 to complete the AI economy loop.
4. Complete the cross-cutting validation and roadmap update.

### Format Validation

All 23 tasks use the required checkbox, sequential ID, optional parallel marker, required user-story label within story phases, and exact repository file path format.
