---

description: "Actionable implementation tasks for controller-aware shot presentation"
---

# Tasks: Shot Presentation Camera

**Input**: Design documents in docs/specs/20260909-220107-shot-presentation-camera/

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/shot-presentation-camera.md, quickstart.md

**Tests**: Required. The feature specification explicitly requires focused mode-selection, state-transition, apex, impact, match-result, controller-independence, safety, and simulation-regression coverage. Use existing Rust module-local unit tests; do not add pixel tests.

**Organization**: Tasks are grouped by user story. All feature code currently belongs in crates/azimuth-game/src/main.rs, so state/pose tasks are intentionally sequential to avoid conflicting edits.

## Phase 1: Setup

**Purpose**: Confirm the exact existing extension points and retain a clean baseline before feature edits.

- [x] T001 Inspect and preserve current camera, launch, fixed-update, HUD, and resolution order in crates/azimuth-game/src/main.rs before editing
- [x] T002 Run the baseline focused suite and record its passing state with cargo test -p azimuth-game from the Cargo.toml workspace root

---

## Phase 2: Foundational Presentation State

**Purpose**: Create the shared, presentation-only foundation required by every story.

**CRITICAL**: Complete this phase before any user-story implementation.

- [x] T003 Add a small presentation-only ShotPresentation state, controller-aware mode selector, transition helpers, and central camera tuning constants in crates/azimuth-game/src/main.rs
- [x] T004 Add unit coverage for controller-type mode selection, display-name independence, and valid normal/shot/impact/result transition inputs in crates/azimuth-game/src/main.rs
- [x] T005 Capture immutable firing-player/controller presentation context at the shared successful launch boundary and initialise/reset it with the existing game resources in crates/azimuth-game/src/main.rs
- [x] T006 Update the existing camera intent/update boundary to derive presentation intent from ShotPresentation plus read-only flight, impact, tank, and turn state in crates/azimuth-game/src/main.rs

**Checkpoint**: Shared state is read-only with respect to authoritative projectile, terrain, tank, weapon, and turn resolution, and pure helpers have focused coverage.

---

## Phase 3: User Story 1 - Learn From a Human Shot (Priority: P1) MVP

**Goal**: A Human-fired projectile receives smooth offset follow coverage that widens at observed apex and prepares for impact, giving usable arc, wind, range, and terrain feedback.

**Independent Test**: In a Human match, short, medium, long, and high-arc shots across uneven terrain retain projectile/terrain context; tests select Human follow from controller type, widen once after observed vertical motion changes, and preserve safe fallbacks.

### Tests for User Story 1

- [x] T007 [US1] Add failing unit tests for Human launch selection, one-time ascent-to-descent apex widening, descent emphasis, immediate-impact handling, and near-vertical velocity fallback in crates/azimuth-game/src/main.rs
- [x] T008 [US1] Extend camera pose safety tests for Human follow target clamp, distance range, pitch range, zero roll, and smoothing behavior in crates/azimuth-game/src/main.rs

### Implementation for User Story 1

- [x] T009 [US1] Implement HumanShotFollow pose derivation using live projectile position/velocity, comfortable trailing/height offsets, look-ahead, and launch-direction fallback in crates/azimuth-game/src/main.rs
- [x] T010 [US1] Implement observed vertical-velocity apex transition and centralised Human follow/apex/descent zoom tuning in crates/azimuth-game/src/main.rs
- [x] T011 [US1] Recompute active Human shot pose each rendered frame through existing interpolation while retaining manual camera input behavior only where it cannot fight automatic shot presentation in crates/azimuth-game/src/main.rs

**Checkpoint**: Human-only shot presentation is independently testable and visually demonstrates learning-oriented follow coverage without modifying projectile simulation.

---

## Phase 4: User Story 2 - Understand an AI Shot Tactically (Priority: P1)

**Goal**: An AI-fired projectile receives broad, efficient tactical coverage that communicates shooter, route, terrain, relevant tanks, and likely impact region without exposing AI intent.

**Independent Test**: Rename a configured AI/Human and change only controller type; tests select tactical mode only for AI. Manual all-AI and consecutive-AI runs retain valid broad framing with no Human-tank assumption.

### Tests for User Story 2

- [x] T012 [US2] Add failing unit tests for AI tactical selection from captured controller type, all-AI composition fallback, and Human-name/controller independence in crates/azimuth-game/src/main.rs
- [x] T013 [US2] Add pose-decision tests proving AI tactical framing remains broader than Human early-flight framing and does not require an existing Human tank in crates/azimuth-game/src/main.rs

### Implementation for User Story 2

- [x] T014 [US2] Implement AiTacticalShot pose derivation from shooter, projectile direction, living tanks, and terrain-aware bounded fallback without consulting AI target calculations in crates/azimuth-game/src/main.rs
- [x] T015 [US2] Apply centralised tactical breadth and efficient-transition tuning while preserving existing active-AI turn establishment and HUD behavior in crates/azimuth-game/src/main.rs

**Checkpoint**: AI flight presentation is independently controller-selected, broad, safe in all-AI matches, and distinct from Human follow coverage.

---

## Phase 5: User Story 3 - See Shared Impact Consequences (Priority: P1)

**Goal**: Human and AI shots converge on one impact view that shows explosion, crater, nearby/affected tanks, settling, and final result when applicable.

**Independent Test**: For both firing modes, terrain impact transitions to the same impact state; terrain miss, multi-hit, settling, elimination, winner, and draw resolve to safe impact/next/result presentation as specified.

### Tests for User Story 3

- [x] T016 [US3] Add failing unit tests for Human/AI convergence to ImpactView, terrain-miss fallback, crater/nearby-tank composition inputs, and impact-hold timing in crates/azimuth-game/src/main.rs
- [x] T017 [US3] Add failing unit tests that winner/draw retains result framing, continuing matches select the next survivor only after impact hold, and no next-player presentation is selected for final shots in crates/azimuth-game/src/main.rs

### Implementation for User Story 3

- [x] T018 [US3] Capture read-only terrain-impact aftermath into ImpactContext and implement common ImpactView pose derivation using impact, crater scale, nearby living/settling tanks, and existing camera safety clamps in crates/azimuth-game/src/main.rs
- [x] T019 [US3] Implement a presentation-only aftermath hold that permits existing authoritative impact, terrain, damage, settling, handoff, and result logic to complete while preventing only a subsequent Human action or autonomous AI launch in crates/azimuth-game/src/main.rs
- [x] T020 [US3] Implement post-hold next-player/result intent selection, including out-of-bounds fallback and preservation of existing scoreboard/winner/draw HUD behavior in crates/azimuth-game/src/main.rs

**Checkpoint**: Either controller produces a shared, consequence-first impact view; final shots do not show another player and consecutive AI shots do not erase the preceding aftermath.

---

## Phase 6: User Story 4 - Preserve Authoritative Play (Priority: P1)

**Goal**: Camera behavior can be altered or disabled without changing deterministic shot results or owning match progression.

**Independent Test**: Compare identical authoritative shots under alternative presentation evaluation and verify impact, damage, terrain, settling, elimination, result, and turn outcome remain equal.

### Tests for User Story 4

- [x] T021 [US4] Add regression tests proving presentation selectors, phase progression, and pose derivation do not mutate cloned Projectile, BattlefieldTerrain, Tank, or TurnState data in crates/azimuth-game/src/main.rs
- [x] T022 [US4] Extend existing deterministic projectile-resolution fixtures to compare identical impact position, damage, crater/terrain, settling, survivor, and winner/draw outcomes with presentation observation active or bypassed in crates/azimuth-game/src/main.rs

### Implementation for User Story 4

- [x] T023 [US4] Audit and revise camera/launch/AI gating code so presentation writes only its own state and cannot gate fixed-step projectile, impact, terrain, settling, elimination, winner, draw, or authoritative turn completion in crates/azimuth-game/src/main.rs
- [x] T024 [US4] Add intent-focused comments documenting Human feedback versus AI awareness, apex widening, shared impact framing, aftermath input gate, simulation independence, and invalid-framing fallbacks in crates/azimuth-game/src/main.rs

**Checkpoint**: The non-authority contract has direct regression coverage and code-level documentation.

---

## Phase 7: Polish and Cross-Cutting Validation

**Purpose**: Validate the complete feature, document only demonstrated roadmap completion, and retain repository health.

- [x] T025 [P] Update current shot-presentation controls and behavior documentation after implementation in README.md
- [x] T026 [P] Run the automated commands and complete every manual scenario in docs/specs/20260909-220107-shot-presentation-camera/quickstart.md
- [x] T027 Update only satisfied projectile/impact camera checkboxes and the AI tactical-wide refinement note in docs/roadmap.md after T026 evidence
- [x] T028 Run cargo fmt --all -- --check, cargo test --workspace, cargo check --workspace --all-targets, and cargo clippy --workspace --all-targets --all-features -- -D warnings from Cargo.toml workspace root

---

## Dependencies and Execution Order

### Phase dependencies

- Phase 1 has no prerequisites.
- Phase 2 depends on Phase 1 and blocks every user story.
- US1, US2, US3, and US4 share the same camera orchestration file and should be implemented in the listed order: US1 → US2 → US3 → US4.
- Phase 7 depends on all four stories.

### User-story dependencies

- **US1**: Depends on foundational controller-aware presentation state; delivers the MVP.
- **US2**: Depends on shared state and reuses the live-flight pose path established by US1, but has its own controller-selection and tactical-composition tests.
- **US3**: Depends on shared state and both flight-mode entry points so it can prove common impact convergence.
- **US4**: Depends on complete presentation integration so it can compare authoritative results across all modes.

### Parallel opportunities

The implementation hotspot is one source file, so primary code tasks are deliberately serial. Limited parallel work is possible after a stable code checkpoint:

- T025 documentation may proceed in parallel with T026 manual validation once T024 is complete.
- A reviewer may execute manual Human/AI cases from quickstart.md while another developer runs T028 quality commands after T026 begins.
- A separate reviewer can inspect the contract and roadmap evidence while implementation is complete, without editing main.rs.

## Parallel Example: Final Validation

    Task: "Update current shot-presentation controls and behavior documentation in README.md"
    Task: "Run manual Human/AI validation scenarios in docs/specs/20260909-220107-shot-presentation-camera/quickstart.md"

## Implementation Strategy

### MVP first

1. Complete T001–T006.
2. Complete T007–T011 for Human learning-oriented follow coverage.
3. Run the US1 independent test and a Human manual shot review before continuing.

### Incremental delivery

1. Add Human coverage and validate it independently.
2. Add AI tactical coverage and validate consecutive/all-AI behavior.
3. Add shared impact/aftermath and validate all impact variants.
4. Add non-authority regression proof.
5. Complete documentation, roadmap evidence, full checks, and several Human/AI matches before selecting another feature.

## Format Validation

All 28 implementation tasks use the required checklist format: checkbox, sequential task ID, optional parallel marker only where appropriate, required user-story label in story phases, and an explicit file path or repository-root command location.
