# Tasks: Projectile Terrain Impact Detection

**Input**: Design documents from `docs/specs/20260905-143022-projectile-terrain-impact/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, and `quickstart.md`

**Tests**: Required. The specification explicitly requires strong deterministic automated coverage
for terrain intersection; add and run renderer-independent Rust unit tests.

**Organization**: Tasks are grouped by user story. The authoritative terrain surface is a blocking
foundation because every story requires it.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel after its stated dependencies, because it changes a separate file.
- **[US#]**: User-story traceability label.

## Phase 1: Setup

**Purpose**: No initialization is required: the existing single crate, Bevy dependency, fixed-step
schedule, and Rust unit-test harness satisfy the plan. Confirm the implementation starts from the
active feature artifacts.

- [X] T001 Review implementation constraints and manual acceptance steps in docs/specs/20260905-143022-projectile-terrain-impact/{spec.md,plan.md,quickstart.md}

---

## Phase 2: Foundational — Authoritative Terrain Surface

**Purpose**: Establish the shared terrain definition that blocks all impact work. No user-story
implementation begins until the rendered mesh, height query, tank grounding, and collision input
can use the same static triangulated surface.

- [X] T002 Refactor grid resolution, vertex sampling, mesh-triangle layout, and piecewise-triangle local-height evaluation into crates/azimuth-game/src/battlefield.rs
- [X] T003 Update terrain mesh construction to consume the authoritative grid helpers from crates/azimuth-game/src/main.rs
- [X] T004 Add deterministic terrain tests for bounds, vertex agreement, mesh-triangle interpolation, and non-flat relief in crates/azimuth-game/src/battlefield.rs
- [X] T005 Confirm terrain-grounded tank placement continues to use the authoritative terrain height in crates/azimuth-game/src/tank.rs

**Checkpoint**: The current static rendered mesh and every local terrain-height consumer agree;
the foundation is ready for projectile work.

---

## Phase 3: User Story 1 — Observe a Shot Strike the Battlefield (Priority: P1) 🎯 MVP

**Goal**: A projectile detects its swept intersection with terrain, resolves a deterministic
surface position, and stops immediately instead of travelling through the ground.

**Independent Test**: Run pure projectile tests using flat, sloped, and non-flat terrain functions.
Verify above-terrain continuation, ordinary and high-speed crossing, endpoint contact, surface
proximity, fixed-step repeatability, and relevant gravity variation without renderer state.

### Tests for User Story 1

- [X] T006 [US1] Add failing pure-domain tests for above-terrain continuation, flat analytical descent, high-speed swept crossing, endpoint contact, and impact-surface proximity in crates/azimuth-game/src/projectile.rs
- [X] T007 [US1] Add failing pure-domain tests for non-flat/synthetic-sloped terrain, repeated identical impact positions, and valid gravity variation in crates/azimuth-game/src/projectile.rs

### Implementation for User Story 1

- [X] T008 [US1] Define the concrete terrain-impact value and three-way fixed-step outcome (`Active`, terrain impact, non-impact termination) in crates/azimuth-game/src/projectile.rs
- [X] T009 [US1] Preserve the current kinematic update while adding in-bounds above-to-on/below segment detection and 24-step deterministic bisection against a pure terrain-height input in crates/azimuth-game/src/projectile.rs
- [X] T010 [US1] Integrate swept terrain advancement with the existing fixed update so a terrain impact clears ProjectileFlight and the projectile visual cannot continue through terrain in crates/azimuth-game/src/main.rs

**Checkpoint**: The development shot follows its unchanged pre-impact arc and stops at the
authoritative terrain surface. User Story 1 is fully testable from domain tests and the running app.

---

## Phase 4: User Story 2 — Inspect Where the Simulation Registered Impact (Priority: P2)

**Goal**: A developer sees one simple impact marker at the simulation-derived surface position
after the projectile stops.

**Independent Test**: Launch a shot into distinct elevations, then confirm the projectile visual
is absent while one marker is visible at each reported impact; launch another shot and confirm the
marker is replaced without influencing flight.

### Implementation for User Story 2

- [X] T011 [US2] Add an optional latest-terrain-impact resource that is set only from the domain outcome and cleared on a new development launch in crates/azimuth-game/src/main.rs
- [X] T012 [US2] Add a distinct tagged primitive impact marker that observes the latest impact, replaces the previous marker, and is removable without simulation changes in crates/azimuth-game/src/main.rs

**Checkpoint**: A visible, presentation-only marker answers where the simulation registered the
most recent impact, including on non-flat terrain.

---

## Phase 5: User Story 3 — Distinguish Terrain Impact from Other Termination (Priority: P3)

**Goal**: Presentation/debugging and future gameplay can distinguish terrain impact from the
existing useful-volume or lifetime termination without invented collision data.

**Independent Test**: Run equivalent pure simulations where one crosses terrain and one leaves the
useful volume without crossing. Verify their different outcomes and that only the former exposes an
impact position.

### Tests for User Story 3

- [X] T013 [P] [US3] Add pure-domain tests that out-of-bounds/lifetime termination without terrain contact produces no terrain-impact result and remains repeatable in crates/azimuth-game/src/projectile.rs

### Implementation for User Story 3

- [X] T014 [US3] Ensure fixed-update lifecycle handling preserves the non-impact termination distinction and never leaves/stores an impact marker for it in crates/azimuth-game/src/main.rs

**Checkpoint**: Terrain impacts expose exactly one position, while non-impact termination is
observable as a separate outcome and leaves no fabricated impact data.

---

## Phase 6: Polish & Cross-Cutting Completion

**Purpose**: Document verified behaviour, update only fulfilled roadmap entries, and complete
quality/manual validation without expanding into explosion work.

- [X] T015 [P] Document the authoritative triangulated surface, swept impact, 24-step refinement, and distinct termination rules in docs/projectile-model.md and docs/world-conventions.md
- [X] T016 [P] Update current feature status, Space-control behaviour, and impact-marker guidance in README.md
- [X] T017 Update only acceptance-criteria-satisfied impact/terrain/marker roadmap checkboxes, and record any discovered nonessential follow-up without marking explosion/deformation/aiming work complete, in docs/roadmap.md
- [X] T018 Run the automated quality gate (`cargo check --workspace --all-targets`, `cargo test --workspace`, `cargo fmt --all -- --check`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings`) from Cargo.toml
- [ ] T019 Run every manual scenario in docs/specs/20260905-143022-projectile-terrain-impact/quickstart.md and resolve any issue within crates/azimuth-game/src/ before feature completion

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1**: No dependencies.
- **Phase 2**: Depends on T001 and blocks all stories.
- **US1 (Phase 3)**: Depends on T002–T005. T006 and T007 precede T008–T010.
- **US2 (Phase 4)**: Depends on the terrain-impact outcome and lifecycle from T008–T010.
- **US3 (Phase 5)**: Depends on the outcome from T008–T010; T013 precedes T014.
- **Polish (Phase 6)**: Depends on US1, US2, and US3 completion; documentation tasks may begin
  once their respective implementation behaviour is stable.

### User Story Dependencies

```text
Foundational terrain surface
        └── US1: swept terrain impact (MVP)
                ├── US2: visible impact marker
                └── US3: distinct non-impact termination
                        └── Polish and acceptance validation
```

- **US1** is the MVP and delivers the simulation result required by the remaining stories.
- **US2** consumes US1's concrete impact result but does not alter simulation.
- **US3** consumes US1's outcome type and proves non-impact distinction; it can proceed alongside
  US2 once T010 is complete, subject to coordination because both touch `main.rs`.

## Parallel Opportunities

- T015 and T016 can run in parallel after the documented behaviour is stable because they change
  separate documentation files.
- T013 can run in parallel with marker design work after T008 establishes the outcome type, but it
  must complete before T014.
- No same-file Rust tasks are marked `[P]`; preserving a readable coherent sequence is safer than
  artificial parallelism in this compact crate.

## Parallel Example: Post-Implementation Documentation

```text
Task: "Document simulation conventions in docs/projectile-model.md and docs/world-conventions.md"
Task: "Update development controls and status in README.md"
```

## Implementation Strategy

### MVP First (US1 only)

1. Complete T001–T005 to unify the terrain surface.
2. Write T006–T007, implement T008–T010, and run the US1 tests.
3. Start the application and verify a projectile stops on terrain before proceeding.

### Incremental Delivery

1. Complete US1 for deterministic swept intersection and stopping.
2. Add US2 for visible impact feedback.
3. Add US3 to preserve an explicit non-impact termination distinction.
4. Finish documentation, roadmap updates, full quality gates, and manual acceptance.

## Notes

- All 19 tasks use the required checkbox, sequential ID, story label (where applicable), and exact
  file-path format.
- Keep impact data small and concrete. Do not introduce explosion, damage, deformation, physics,
  collider, event, or diagnostics abstractions while completing these tasks.
