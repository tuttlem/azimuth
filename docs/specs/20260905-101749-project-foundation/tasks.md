# Tasks: Project Foundation

**Input**: Design documents from `docs/specs/20260905-101749-project-foundation/`

**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md),
[data-model.md](./data-model.md), and [quickstart.md](./quickstart.md)

**Tests**: A focused unit test is required to demonstrate normal workspace test discovery. No
integration-test or shared-test infrastructure is needed.

**Organization**: Tasks are grouped by user story so each outcome is independently verifiable.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish the deliberately minimal virtual workspace and its local development policy.

- [X] T001 Create the virtual workspace manifest and sole `azimuth-game` member manifest in `Cargo.toml` and `crates/azimuth-game/Cargo.toml`.
- [X] T002 [P] Add the stable Rust toolchain declaration with formatter and linter components in `rust-toolchain.toml`.
- [X] T003 [P] Add focused Cargo, editor, and platform artifact exclusions in `.gitignore`.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Confirm that the workspace shape and toolchain are ready before feature outcomes are
implemented.

**⚠️ CRITICAL**: Complete this phase before user-story work.

- [X] T004 Verify workspace membership declarations and stable-toolchain resolution using `Cargo.toml`, `crates/azimuth-game/Cargo.toml`, and `rust-toolchain.toml`; run Cargo target validation after T006 creates the target.

**Checkpoint**: The repository has one virtual workspace and one purposeful application member,
with no dependency or future-subsystem scaffolding.

---

## Phase 3: User Story 1 - Validate a Clean Checkout (Priority: P1) 🎯 MVP

**Goal**: Provide a minimal runnable and testable `azimuth-game` member and prove all local health
checks work for the workspace.

**Independent Test**: From the repository root, run the commands in `quickstart.md`; workspace
check/build, test, formatting, and linting all succeed.

- [X] T005 [US1] Add a failing readable unit test for the foundation executable's observable identity in `crates/azimuth-game/src/main.rs`.
- [X] T006 [US1] Implement the minimal non-game executable that satisfies the unit test in `crates/azimuth-game/src/main.rs`.
- [X] T007 [US1] Generate and commit the dependency lockfile by checking the workspace, then run all repository-health commands from `docs/specs/20260905-101749-project-foundation/quickstart.md` against `Cargo.lock`.

**Checkpoint**: The workspace builds, tests, formats, and lints cleanly with zero runtime
dependencies and no gameplay or rendering code.

---

## Phase 4: User Story 2 - Understand the Project and Its Workflow (Priority: P2)

**Goal**: Give a new contributor concise, accurate orientation and the exact local workflow.

**Independent Test**: A contributor can locate the game concept, status, toolchain policy,
validation commands, roadmap, and timestamped specification convention by reading `README.md`.

- [X] T008 [US2] Write the concise project overview, current status, prerequisites, standard validation commands, CI deferral, and development workflow in `README.md`.
- [X] T009 [US2] Add roadmap and timestamped-specification location guidance plus deterministic and regression-test conventions in `README.md`.
- [X] T010 [US2] Manually validate all commands and documentation links in `README.md` against `Cargo.toml`, `rust-toolchain.toml`, `docs/roadmap.md`, and `docs/specs/`.

**Checkpoint**: A new developer can understand and validate the repository without inspecting
source files.

---

## Phase 5: User Story 3 - Begin the Next Deliberate Feature (Priority: P3)

**Goal**: Confirm that the foundation stays neutral and leaves a clear path for future purposeful
members such as `azimuth-math`.

**Independent Test**: Inspect the manifests and application source to confirm `azimuth-game` is the
only member, there are no runtime dependencies, and no future game technology or subsystem exists.

- [X] T011 [US3] Review and remove any speculative dependencies, workspace members, abstractions, or game technology from `Cargo.toml`, `crates/azimuth-game/Cargo.toml`, and `crates/azimuth-game/src/main.rs`.
- [X] T012 [US3] Confirm the future-extension rationale for sibling crates is accurate in `docs/specs/20260905-101749-project-foundation/plan.md` and `README.md` without precreating those crates.

**Checkpoint**: The next technology-selection specification can proceed without inheriting an
engine, renderer, simulation model, or unearned crate boundary.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Complete the feature only after its quality and roadmap obligations are met.

- [X] T013 Re-run every end-to-end validation and reconcile documentation with results using `docs/specs/20260905-101749-project-foundation/quickstart.md`, `README.md`, and `docs/roadmap.md`.
- [X] T014 Update satisfied Project Foundation and near-term repository-foundation checkboxes, while retaining deferred CI work, in `docs/roadmap.md`.
- [X] T015 Review the completed feature for constitution compliance and record any newly discovered nonessential work in `docs/roadmap.md`.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Starts immediately. T002 and T003 can run in parallel with each other.
- **Foundational (Phase 2)**: Depends on T001 and T002; blocks user stories.
- **US1 (Phase 3)**: Depends on T004. T005 must precede T006, which must precede T007.
- **US2 (Phase 4)**: Depends on T007 so the documented commands match the working workspace.
- **US3 (Phase 5)**: Depends on T007 and T009 so the neutral-boundary implementation and its
  explanation can be reviewed together.
- **Polish (Phase 6)**: Depends on US1, US2, and US3. T013 is the final validation gate; T014
  updates the roadmap only after that gate passes; T015 is the final completion review.

### User Story Dependencies

- **US1 (P1)**: Requires the foundational workspace validation; no dependency on US2 or US3.
- **US2 (P2)**: Requires US1 so documented commands are verified rather than aspirational.
- **US3 (P3)**: Requires US1 and the extension guidance in US2; it introduces no new code.

## Parallel Opportunities

- T002 (`rust-toolchain.toml`) and T003 (`.gitignore`) may proceed in parallel after or alongside
  T001 because they modify independent files.
- After T007, documentation work in T008 and the source/manifests scope review in T011 can proceed
  in parallel; T012 waits for T009.

## Parallel Example: Setup and User Story 3

```text
Task: "Add the stable Rust toolchain declaration in rust-toolchain.toml"
Task: "Add focused exclusions in .gitignore"

After User Story 1 is validated:
Task: "Write project orientation and validation commands in README.md"
Task: "Review Cargo.toml, crates/azimuth-game/Cargo.toml, and crates/azimuth-game/src/main.rs for speculative scope"
```

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete the virtual-workspace, toolchain, and hygiene setup.
2. Validate the foundational workspace.
3. Complete the minimal `azimuth-game` target and its unit test.
4. Run all workspace health commands from `quickstart.md`.
5. Stop and verify that the project is a clean, dependency-free foundation before adding
   contributor documentation.

### Incremental Delivery

1. Setup + Foundational: a valid empty-technology workspace.
2. US1: a buildable, testable, format-checked, lint-clean foundation.
3. US2: a contributor can orient and validate themselves.
4. US3: the boundary remains neutral for future features.
5. Polish: roadmap accurately reflects completion and CI remains intentionally deferred.

## Notes

- Every task follows the required checkbox, ID, optional parallel marker, story label, and exact
  path format.
- No task creates a game engine, renderer, ECS, physics system, gameplay system, extra crate,
  dependency, task runner, or CI workflow.
- Mark roadmap work complete only after T014's validation passes.
