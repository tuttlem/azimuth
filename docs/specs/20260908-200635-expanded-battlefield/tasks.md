---

description: "Dependency-ordered tasks for Expanded Battlefield — Mountains, Valleys, Terrain Colour and World Dressing"
---

# Tasks: Expanded Battlefield — Mountains, Valleys, Terrain Colour and World Dressing

**Input**: Design documents from `docs/specs/20260908-200635-expanded-battlefield/`

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [battlefield-generation contract](contracts/battlefield-generation.md), [quickstart.md](quickstart.md)

**Tests**: Automated tests are required by the feature specification. Add focused unit/regression tests first, maintain existing regression coverage, and execute the full quickstart validation before completion.

**Organization**: Tasks are grouped by user story. `[P]` means the task can proceed in parallel with other marked tasks after its stated prerequisites; story labels provide traceability.

## Phase 1: Setup

**Purpose**: Establish the reproducible validation inputs and implementation boundary without adding a new framework.

- [X] T001 Define the fixed representative seed table, expected 120-unit map constants, and manual seed-recording procedure in `docs/specs/20260908-200635-expanded-battlefield/quickstart.md`
- [X] T002 [P] Document the authoritative-terrain versus presentation-only water/building boundary as implementation comments and test expectations in `docs/specs/20260908-200635-expanded-battlefield/contracts/battlefield-generation.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Create the shared deterministic world definition that every story uses.

**⚠️ CRITICAL**: Complete this phase before story implementation. It replaces the old terrain construction and makes deterministic streams available without coupling presentation to gameplay.

- [X] T003 Add failing deterministic-contract tests for labelled master-seed derivation and independent terrain/start/dressing/wind streams in `crates/azimuth-game/src/main.rs`
- [X] T004 Add failing battlefield-domain tests for 120-by-120 bounds, 64 cells/65 vertices, finite corner queries, and maximum vertex count in `crates/azimuth-game/src/battlefield.rs`
- [X] T005 Implement the captured `BattlefieldSeed` resource, pure labelled sub-seed derivation, and explicit seed diagnostic/logging path in `crates/azimuth-game/src/main.rs`
- [X] T006 Implement the 120-unit bounds, 64-cell grid constants, water-table constant/accessor, and fixed-seed terrain-construction entry point while retaining one mutable authoritative height field in `crates/azimuth-game/src/battlefield.rs`
- [X] T007 Replace startup’s pre-setup terrain/wind construction with one match-generation function that derives terrain, starts, dressing, and wind from the captured seed in `crates/azimuth-game/src/main.rs`
- [X] T008 Update the Enter/Start Match reset path to rebuild generated terrain and match-constant resources from its captured seed rather than retaining the old development terrain in `crates/azimuth-game/src/main.rs`

**Checkpoint**: The game can construct a reproducible 120-unit empty/generated world through one seed flow; all later systems have a single source of terrain truth.

---

## Phase 3: User Story 1 - Read and Fight Across a Real Landscape (Priority: P1) 🎯 MVP

**Goal**: Deliver a broad deterministic landscape whose independently meaningful macro features change shot geometry while keeping current terrain authority intact.

**Independent Test**: Generate the fixed seed table, verify repeated sampled heights match, different seeds vary, every map has at least a 24-unit sampled elevation span and broad high/low forms, then fire existing shots across representative ridges/valleys using normal collision.

### Tests for User Story 1

- [X] T009 [US1] Add failing fixed-seed tests for macro/local reproducibility, different-seed sampled variation, finite heights, 24-unit elevation span, and broad high/low region presence in `crates/azimuth-game/src/battlefield.rs`
- [X] T010 [US1] Add failing regression tests that current height queries, triangle interpolation, craters, and edge craters remain consistent on generated high, low, centre, and boundary terrain in `crates/azimuth-game/src/battlefield.rs`
- [X] T011 [US1] Add failing representative generated-terrain projectile-impact tests that resolve swept collision on the current surface near a ridge/highland and near an edge in `crates/azimuth-game/src/projectile.rs`

### Implementation for User Story 1

- [X] T012 [US1] Implement bounded seed-derived mountain, ridge, bowl/valley, baseline, and low-amplitude local-height composition with gameplay-intent comments in `crates/azimuth-game/src/battlefield.rs`
- [X] T013 [US1] Ensure crater application and `mesh_positions` continue to derive entirely from the generated terrain’s current mutable heights in `crates/azimuth-game/src/battlefield.rs`
- [X] T014 [US1] Update terrain construction callers and existing terrain regression fixtures to use deterministic generated terrain where map-scale coverage is required in `crates/azimuth-game/src/main.rs`
- [X] T015 [US1] Run the focused terrain and projectile tests and tune only bounded macro parameters until all representative seeds meet the elevation/spawnable-land invariants in `crates/azimuth-game/src/battlefield.rs`

**Checkpoint**: A player can inspect a reproducible broad mountain/ridge/valley landscape and existing shells collide with it through the normal authoritative terrain path.

---

## Phase 4: User Story 2 - Read Elevation, Water, and Dressing at a Glance (Priority: P1)

**Goal**: Make elevation visually readable with coherent crater refreshes, a simple water plane, and sparse non-authoritative building dressing.

**Independent Test**: For several generated seeds, verify height-colour ordering/bounds and deterministic dressing validity; inspect a rendered seed containing low water and a building cluster, then crater it and confirm visible terrain refresh remains coherent without gameplay interaction.

### Tests for User Story 2

- [X] T016 [P] [US2] Add failing pure tests for bounded smooth elevation-colour interpolation, representative low/mid/high ordering, and current-height colour changes after a crater in `crates/azimuth-game/src/battlefield.rs`
- [X] T017 [P] [US2] Add failing deterministic dressing tests for in-bounds, dry, locally suitable, grounded, sparse, and start-clear building records in `crates/azimuth-game/src/battlefield.rs`
- [X] T018 [US2] Add failing scene-data regression tests proving water/building records are absent from terrain, projectile, tank, combat, and turn authority paths in `crates/azimuth-game/src/main.rs`

### Implementation for User Story 2

- [X] T019 [US2] Implement pure current-height terrain colour stops/blending and mesh vertex-colour generation so every terrain refresh recreates positions and colours together in `crates/azimuth-game/src/battlefield.rs`
- [X] T020 [US2] Extend `create_battlefield_mesh` and `sync_battlefield_mesh` to use terrain vertex colours after initial generation and every crater without creating a second terrain/material authority in `crates/azimuth-game/src/main.rs`
- [X] T021 [US2] Add one flat blue water-plane scene entity at the central water-table elevation with no gameplay component or collision path in `crates/azimuth-game/src/main.rs`
- [X] T022 [US2] Implement small deterministic presentation-only building placement records from the dressing sub-seed, with dry/flat/bounds checks and explicit no-authority comments in `crates/azimuth-game/src/battlefield.rs`
- [X] T023 [US2] Spawn and reset simple grey cuboid building entities from dressing records after final match generation; keep them out of all projectile, tank, explosion, terrain, and turn queries in `crates/azimuth-game/src/main.rs`
- [ ] T024 [US2] Verify rendered colour, water, and sparse-dressing acceptance across the representative seed table and record any presentation confusion for the final roadmap task in `docs/specs/20260908-200635-expanded-battlefield/quickstart.md`

**Checkpoint**: Elevation is legible, low terrain is visibly submerged, crater colours update coherently, and sparse buildings add identity without becoming simulated obstacles.

---

## Phase 5: User Story 3 - Start a Valid Distributed Multiplayer Match (Priority: P1)

**Goal**: Replace old fixed locations with seed-reproducible, dry, supported, separated starts that visibly use the expanded map for all 2–8 human matches.

**Independent Test**: For each fixed seed and player counts 2, 4, and 8, create starts twice and verify exact equality, count, bounds, dryness, support, one-unit local-slope validity, 18-unit separation, and wide-map distribution.

### Tests for User Story 3

- [X] T025 [US3] Add failing tests for candidate rejection at water, bounds, steep one-unit cardinal samples, overlap, and insufficient separation in `crates/azimuth-game/src/tank.rs`
- [X] T026 [US3] Add failing fixed-seed 2-, 4-, and 8-player tests for exact repeatability, supported/dry/in-bounds starts, 18-unit pairwise separation, and sector distribution in `crates/azimuth-game/src/tank.rs`
- [X] T027 [US3] Add failing integration tests proving the match-start boundary regenerates tanks and presentation dressing from the same captured seed without changing derived wind in `crates/azimuth-game/src/main.rs`

### Implementation for User Story 3

- [X] T028 [US3] Replace the fixed `POSITIONS` array with bounded deterministic sector-aware candidate selection that samples current terrain at each one-unit cardinal neighbour using the existing 0.75 movement threshold in `crates/azimuth-game/src/tank.rs`
- [X] T029 [US3] Ground accepted tank starts on authoritative terrain, apply deterministic centre/opponent-facing orientation, and expose a clear bounded-selection diagnostic rather than an invalid fallback in `crates/azimuth-game/src/tank.rs`
- [X] T030 [US3] Pass the start sub-seed through both initial match resources and Enter/Start Match setup, then generate dressing only after final tank starts exist in `crates/azimuth-game/src/main.rs`
- [X] T031 [US3] Keep `MatchConfiguration`’s 2–8 Human/AI validation unchanged while integrating seed-driven starts only for valid all-human launch in `crates/azimuth-game/src/match_setup.rs`

**Checkpoint**: Every valid 2–8 human match starts with distinct dry, supported tanks distributed across the new terrain, without claiming tactical fairness or enabling AI.

---

## Phase 6: User Story 4 - Keep the Existing Artillery Game Playable (Priority: P2)

**Goal**: Adjust only map-scale limits/camera presentation needed for the 120-unit terrain and prove all established artillery systems remain intact.

**Independent Test**: Run the existing weapon, wind, gravity, impact, deformation, settling, movement, turn, victory, inventory, HUD, and N-player regressions on generated terrain; manually complete 2-, 4-, and 8-player matches while firing all three weapons at short and long ranges.

### Tests for User Story 4

- [X] T032 [P] [US4] Add failing tests for named expanded-map projectile useful-volume limits, high-peak arcs, edge terrain impacts, and distinct out-of-bounds termination in `crates/azimuth-game/src/projectile.rs`
- [X] T033 [P] [US4] Add failing camera tests for 60-unit target clamping, active-player usability, and expanded shot-overview distance/far-clip bounds in `crates/azimuth-game/src/main.rs`
- [X] T034 [US4] Add generated-terrain regression cases for Basic Shell, High Explosive, Heavy Shell, wind response, crater settling, movement, turn handoff, victory, inventory, and scalable HUD in `crates/azimuth-game/src/main.rs`

### Implementation for User Story 4

- [X] T035 [US4] Replace `SimulationLimits::DEVELOPMENT` at gameplay call sites with a named expanded-battlefield limit that preserves fixed-step collision and uses a deliberate horizontal/vertical envelope in `crates/azimuth-game/src/projectile.rs`
- [X] T036 [US4] Adjust only existing camera maximum distance, shot pose, and explicit far clip needed to frame the new bounds and peaks while retaining presentation-only camera control in `crates/azimuth-game/src/main.rs`
- [X] T037 [US4] Run and repair all existing projectile, combat, tank, turn, weapon, HUD, and match-setup regression tests against generated terrain in `crates/azimuth-game/src/{projectile.rs,combat.rs,tank.rs,turn.rs,weapon.rs,main.rs,match_setup.rs}`

**Checkpoint**: Existing artillery remains understandable and complete on the expanded terrain; no weapon identity, movement allowance, water gameplay, or camera-system redesign has been introduced.

---

## Phase 7: Polish & Cross-Cutting Completion

**Purpose**: Execute the manual acceptance matrix, document actual behaviour, and leave the repository healthy.

- [X] T038 [P] Update 120-unit bounds, generated terrain, water-table presentation-only semantics, and deterministic seed conventions in `docs/world-conventions.md`
- [X] T039 [P] Update map-aware projectile limits and the fact that water/buildings do not alter flight in `docs/projectile-model.md`
- [ ] T040 Perform the five-seed visual inspection plus representative 2-, 4-, and 8-player weapon/match playthroughs from `docs/specs/20260908-200635-expanded-battlefield/quickstart.md` and record the actual seed findings in `docs/roadmap.md`
- [X] T041 Update only roadmap checkboxes whose acceptance was demonstrated; record presentation-only-structure confusion or follow-up work without marking fair starts, unwinnable-terrain prevention, water gameplay, authoritative buildings, or AI complete in `docs/roadmap.md`
- [X] T042 Run `cargo test -p azimuth-game`, `cargo fmt --check`, and `cargo clippy --workspace --all-targets -- -D warnings`, then fix all feature-caused failures in `crates/azimuth-game/` and `docs/`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Starts immediately.
- **Foundational (Phase 2)**: Depends on T001–T002 and blocks all story code because it defines map dimensions, seed isolation, and match-generation wiring.
- **US1 (Phase 3)**: Depends on T003–T008. It is the MVP terrain increment.
- **US2 (Phase 4)**: Depends on US1 terrain generation and the seed/dressing stream; building finalisation also depends on US3 starts.
- **US3 (Phase 5)**: Depends on US1 terrain generation and the shared seed flow; it can proceed in parallel with US2 colour/water work through T021, but T023/T024 must follow T030.
- **US4 (Phase 6)**: Depends on US1–US3, because map limits and regression coverage must exercise final terrain, starts, and presentation integration.
- **Polish (Phase 7)**: Depends on every selected story.

### User Story Dependency Graph

```text
Foundation
    └── US1: generated macro/local authoritative terrain (MVP)
          ├── US2: elevation colour + water (building finalisation waits for US3)
          └── US3: valid distributed starts
                └── US2: final spawn-clear dressing
                      └── US4: map-scale camera/projectile regression
                            └── Polish and manual acceptance
```

### Parallel Opportunities

- T001 and T002 can proceed independently.
- After the seed/terrain interface exists, T009–T011 are separate test files/modules only where no same-file edit collision occurs; implement US1 sequentially in `battlefield.rs`.
- T016 and T017 can proceed in parallel because they cover independent colour and dressing helpers in `battlefield.rs` only if coordinated; otherwise complete sequentially to avoid same-file conflicts.
- T032 and T033 can proceed in parallel in `projectile.rs` and `main.rs`.
- T038 and T039 can proceed in parallel; T040–T042 remain final sequential verification.

## Parallel Examples

### User Story 2

```text
Task: "T016 [US2] Add colour-mapping tests in crates/azimuth-game/src/battlefield.rs"
Task: "T018 [US2] Add visual-authority boundary regression tests in crates/azimuth-game/src/main.rs"
```

### User Story 4

```text
Task: "T032 [US4] Add expanded projectile-limit tests in crates/azimuth-game/src/projectile.rs"
Task: "T033 [US4] Add expanded camera tests in crates/azimuth-game/src/main.rs"
```

## Implementation Strategy

### MVP First

1. Complete T001–T008 to establish deterministic map generation.
2. Complete T009–T015 to deliver and validate authoritative large-scale terrain.
3. Stop and manually inspect the fixed seed table: confirm the geography changes shots before adding presentation/dressing.

### Incremental Delivery

1. Add US1: large deterministic terrain with all prior projectile/crater/tank authority intact.
2. Add US3: valid distributed multiplayer starts so the landscape supports real matches.
3. Add US2: height readability, water, and strictly visual dressing after starts are stable.
4. Add US4 and Phase 7: map-scale limits, full regression, documentation, and manual acceptance.

### Format Validation

Every task uses the required checkbox, sequential task ID, optional `[P]` only where parallelism is safe, story label for story work, and an exact project file path.
