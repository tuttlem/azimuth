# Tasks: Rendering Technology Selection

**Input**: Design documents from `docs/specs/20260905-104956-rendering-technology/`

**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md),
[data-model.md](./data-model.md), and [quickstart.md](./quickstart.md)

**Tests**: Keep existing ordinary unit tests passing. Do not add screenshot, GPU, headless, or
automated graphical tests for this manual rendering proof.

**Organization**: Tasks are grouped by user story to preserve independently reviewable outcomes.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Integrate only the selected technology into the existing application boundary.

- [X] T001 Add the selected Bevy 0.18.1 dependency and update its lockfile resolution in `crates/azimuth-game/Cargo.toml` and `Cargo.lock`.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Verify the selected dependency is compatible with the workspace before proof-scene work.

**⚠️ CRITICAL**: Complete this phase before user-story implementation.

- [X] T002 Validate Bevy dependency resolution and workspace compilation using `crates/azimuth-game/Cargo.toml`, `Cargo.lock`, and `Cargo.toml`.

**Checkpoint**: The existing application crate directly owns the selected presentation dependency;
no workspace or abstraction change has been introduced.

---

## Phase 3: User Story 1 - Make a Deliberate Technology Decision (Priority: P1) 🎯 MVP

**Goal**: Give contributors one lightweight, evidence-backed decision record explaining why Bevy is
selected now instead of a lower-level Rust rendering stack.

**Independent Test**: Review the decision record against `research.md` and confirm it evaluates
Bevy and `wgpu` plus `winit` across all required capability and trade-off areas.

- [X] T003 [P] [US1] Create the lightweight technology decision record with requirements, alternatives, selection, trade-offs, limitations, and reconsideration conditions in `docs/adr/0001-initial-rendering-technology.md`.
- [X] T004 [US1] Validate the decision record's Bevy, `wgpu`, and `winit` claims and terrain/presentation-boundary rationale against `docs/specs/20260905-104956-rendering-technology/research.md`.

**Checkpoint**: The rendering direction is deliberate, understandable, and does not create an ADR
framework or promise a backend-neutral architecture.

---

## Phase 4: User Story 2 - See a Minimal 3D World (Priority: P2)

**Goal**: Start a direct Bevy desktop app that renders a deliberately simple perspective 3D scene.

**Independent Test**: Run `cargo run --package azimuth-game`, observe a window with ground,
depth-visible geometry, fixed camera, and readable lighting, then close it normally.

- [X] T005 [US2] Replace the foundation executable with direct Bevy application setup and normal close-event handling in `crates/azimuth-game/src/main.rs`.
- [X] T006 [US2] Add a startup proof scene with a fixed perspective camera, flat visual ground, simple elevated geometry, and light in `crates/azimuth-game/src/main.rs`.
- [X] T007 [US2] Remove or adapt the obsolete foundation-only unit test in `crates/azimuth-game/src/main.rs` without adding graphics-test infrastructure.
- [X] T008 [US2] Manually validate normal desktop startup, visible 3D depth, and clean window close using `docs/specs/20260905-104956-rendering-technology/quickstart.md`.

**Checkpoint**: Azimuth opens a window and renders the minimal proof without implementing any
gameplay, controllable camera, permanent coordinate convention, asset, or terrain system.

---

## Phase 5: User Story 3 - Extend from a Clear, Neutral Starting Point (Priority: P3)

**Goal**: Make the direct proof runnable and understandable while preserving its deliberately
small presentation boundary.

**Independent Test**: Read `README.md`, follow its run command and decision-record link, then
inspect the source and dependencies for absent gameplay and speculative abstraction.

- [X] T009 [US3] Update the run instructions, current status, and decision-record link in `README.md`.
- [X] T010 [US3] Review `crates/azimuth-game/src/main.rs`, `crates/azimuth-game/Cargo.toml`, and `docs/adr/0001-initial-rendering-technology.md` to remove gameplay representations, external assets, and speculative presentation abstractions.

**Checkpoint**: A contributor can run the proof and understand its decision without mistaking it
for Azimuth's future game-domain or engine architecture.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Complete quality, roadmap, and constitutional obligations after implementation.

- [X] T011 Run the complete workspace check, build, test, formatting, and Clippy validation from `docs/specs/20260905-104956-rendering-technology/quickstart.md`.
- [X] T012 Update only satisfied technology-selection, window-proof, ground-plane, and Milestone A items while retaining later work in `docs/roadmap.md`.
- [X] T013 Review final scope, README links, decision record, and roadmap accuracy against `docs/specs/20260905-104956-rendering-technology/spec.md` and `docs/roadmap.md`.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Starts immediately.
- **Foundational (Phase 2)**: Depends on T001 and blocks the rendering proof.
- **US1 (Phase 3)**: T003 can proceed with setup; T004 depends on T003 and research completion.
- **US2 (Phase 4)**: Depends on T002. T005 precedes T006 and T007; T008 follows the completed
  proof scene.
- **US3 (Phase 5)**: T009 depends on the selected decision record and working run command; T010
  follows T006 and T009.
- **Polish (Phase 6)**: T011 is the final quality gate. T012 updates the roadmap only after T011
  passes. T013 is the final scope and documentation review.

### User Story Dependencies

- **US1 (P1)**: Can be documented independently but must be complete before README linking.
- **US2 (P2)**: Requires the dependency-resolution foundation; it does not require new domain code.
- **US3 (P3)**: Depends on the completed decision record and proof scene.

## Parallel Opportunities

- T003 (`docs/adr/0001-initial-rendering-technology.md`) can proceed in parallel with T001 because the decision
  record and dependency manifest are independent files.
- After T006, T009 (`README.md`) and preparation for T008 (`quickstart.md`) can proceed in parallel;
  T010 waits for the README update.

## Parallel Example: Decision and Setup

```text
Task: "Add the Bevy dependency in crates/azimuth-game/Cargo.toml and Cargo.lock"
Task: "Create the technology decision record in docs/adr/0001-initial-rendering-technology.md"
```

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Add and validate the selected Bevy dependency.
2. Complete the decision record and verify it against the research.
3. Stop and confirm the architectural choice is deliberate before writing rendering code.

### Incremental Delivery

1. Setup + Foundational: Bevy is a healthy dependency in the existing application crate.
2. US1: The selected technology and its limits are documented.
3. US2: A minimal desktop 3D proof is visible and closes cleanly.
4. US3: Contributors can run it and understand the narrowly scoped decision.
5. Polish: Quality and roadmap accurately reflect only this completed work.

## Notes

- Every task follows the required checkbox, ID, optional parallel marker, story label, and exact
  path format.
- No task adds a second renderer, external asset, gameplay system, permanent coordinate model,
  custom ECS, engine wrapper, or future-game abstraction.
- Mark roadmap work complete only after T011's full validation passes.
