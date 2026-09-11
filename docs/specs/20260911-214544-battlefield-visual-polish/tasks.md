# Tasks: Battlefield Visual Polish — Tank Variety, Materials, Explosion Particles and Smoke

**Input**: Design documents from `docs/specs/20260911-214544-battlefield-visual-polish/`

**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [presentation-boundary.md](./contracts/presentation-boundary.md), and [quickstart.md](./quickstart.md)

**Tests**: State, lifecycle, cap, and deterministic-independence tests are required by the specification. Do not add pixel tests.

**Organization**: Tasks are grouped by user story. Presentation work is centred on `crates/azimuth-game/src/main.rs`.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Parallelizable only when files and incomplete dependencies are independent.
- **[Story]**: Story labels apply only to user-story phases.

## Phase 1: Setup

**Purpose**: Confirm approved boundaries and existing seams.

- [X] T001 Review `docs/specs/20260911-214544-battlefield-visual-polish/{plan,research,data-model}.md`, `docs/specs/20260911-214544-battlefield-visual-polish/contracts/presentation-boundary.md`, and current seams in `crates/azimuth-game/src/{main,battlefield,tank,weapon}.rs`.

---

## Phase 2: Foundational Presentation Seams

**Purpose**: Establish small reusable renderer-only helpers and fixed caps.

- [X] T002 Add documented presentation-only constants and pure cosmetic seed/hash and scale-budget helpers in `crates/azimuth-game/src/main.rs`; keep them outside terrain/start/wind/AI streams and fixed-step simulation.
- [X] T003 Add focused helper tests for cosmetic derivation isolation, finite profile budgets, and cap boundaries in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: Shared boundary and budget seams are executable; story work can begin.

---

## Phase 3: User Story 1 - Recognise Individual Tanks (Priority: P1) 🎯 MVP

**Goal**: Assign five clear primitive silhouettes while preserving player colour, canonical muzzle placement, and all tank gameplay.

**Independent Test**: Start 2-, 4-, and 8-player matches; verify unique-first assignment, allowed duplicates, visual hierarchy/aim, and unchanged firing origin.

### Tests for User Story 1

- [X] T004 [US1] Add 2–8-player tests for valid unique-first assignments, allowed duplicates, stable PlayerId/controller/tank state, and unchanged gameplay seed outputs in `crates/azimuth-game/src/main.rs`.
- [X] T005 [US1] Add transform tests proving every recipe retains coherent hull/turret/barrel/muzzle roles and canonical `TankFiringRepresentation` muzzle placement in `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 1

- [X] T006 [US1] Define five presentation-only tank models and match-local assignments in `crates/azimuth-game/src/main.rs`; do not add cosmetic fields to `crates/azimuth-game/src/tank.rs`.
- [X] T007 [US1] Replace the single tank mesh bundle with reusable primitive meshes and painted-armour, dark-track, dark-barrel, and muzzle presentation assets in `crates/azimuth-game/src/main.rs`.
- [X] T008 [US1] Implement Classic, Heavy, Low Profile, Compact, and Angular recipes under existing `TankVisual`, `TankTurret`, `TankBarrel`, and `TankMuzzle` hierarchy in `crates/azimuth-game/src/main.rs`.
- [X] T009 [US1] Wire assignment and recipe spawning into initial scene creation and match-start respawning in `crates/azimuth-game/src/main.rs`, preserving pose, aim, elimination, movement, and firing sync.
- [ ] T010 [US1] Run User Story 1 tests and tank-variety checks in `docs/specs/20260911-214544-battlefield-visual-polish/quickstart.md` for 2-, 4-, and 8-player graphical matches.

**Checkpoint**: Tanks are visibly individual without altered domain state or muzzle physics.

---

## Phase 4: User Story 2 - Read a More Characterful Battlefield (Priority: P1)

**Goal**: Make tanks, terrain, and water materially distinct without changing terrain or water rules.

**Independent Test**: Inspect normal cameras before/after craters and Dirt Bomb deposition; verify coherent regenerated surface treatment.

### Tests for User Story 2

- [X] T011 [P] [US2] Add deterministic elevation/mottle and crater/mound refresh assertions in `crates/azimuth-game/src/battlefield.rs`, proving presentation derivation does not change terrain height/query behaviour.
- [X] T012 [US2] Add mesh/material construction tests for terrain attributes, painted/dark tank materials, and opaque water response in `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 2

- [X] T013 [US2] Enrich height-derived grass, dirt, and high-terrain colour with deterministic presentation-only mottling in `crates/azimuth-game/src/battlefield.rs` while retaining terrain APIs.
- [X] T014 [US2] Update terrain mesh material setup and water material in `crates/azimuth-game/src/main.rs` to retain live terrain colours/normals, use sensible world-scale variation, and give water a restrained wet response.
- [X] T015 [US2] Tune existing material parameters and directional light only as needed for painted metal and terrain readability in `crates/azimuth-game/src/main.rs`; do not add a shader or lighting overhaul.
- [ ] T016 [US2] Run User Story 2 tests and material/crater/Dirt Bomb checks in `docs/specs/20260911-214544-battlefield-visual-polish/quickstart.md`.

**Checkpoint**: Terrain is more characterful and stays coherent after live deformation.

---

## Phase 5: User Story 3 - Feel Impact Energy (Priority: P1)

**Goal**: Add bounded profile-scaled hot/debris bursts to every resolved terrain impact, including split children.

**Independent Test**: Fire Basic Shell, HE, MIRV, Cluster Bomb, Bomb Net, and Nuke; verify responsive bounded bursts and unchanged authority.

### Tests for User Story 3

- [X] T017 [US3] Add tests for monotonic profile particle budgets, per-impact/global caps, cosmetic velocity/gravity/fade, and expiry in `crates/azimuth-game/src/main.rs`.
- [X] T018 [US3] Add resolver-facing tests recording ordinary/split-child impacts without changing health, deformation, settling, projectile state, or turn completion in `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 3

- [X] T019 [US3] Define bounded renderer-only impact request, queue, particle component, and shared debris/hot assets in `crates/azimuth-game/src/main.rs` per `docs/specs/20260911-214544-battlefield-visual-polish/data-model.md`.
- [X] T020 [US3] Append one request after every ordinary and split-child terrain impact in `crates/azimuth-game/src/main.rs`, retaining `LatestTerrainImpact` for marker, flash, audio, and camera behaviour.
- [X] T021 [US3] Add startup resources and request-draining particle spawning in `crates/azimuth-game/src/main.rs`; reuse handles and presentation-local hashing only.
- [X] T022 [US3] Implement Update-time particle motion, visual gravity, fade/shrink, caps, and despawn in `crates/azimuth-game/src/main.rs`; add no debris collision or fixed-step simulation.
- [X] T023 [US3] Schedule impact queue and particle lifecycle after authoritative impact observation in the presentation chain in `crates/azimuth-game/src/main.rs`.
- [ ] T024 [US3] Run User Story 3 tests and every named burst check in `docs/specs/20260911-214544-battlefield-visual-polish/quickstart.md`.

**Checkpoint**: Every terrain impact can add bounded energy; barrages retain multiple effects without changing results.

---

## Phase 6: User Story 5 - Preserve the Existing Game (Priority: P1)

**Goal**: Demonstrate tank/material/effect work never perturbs deterministic gameplay, RNG, or lifecycle hygiene.

**Independent Test**: Compare controlled authoritative fixtures with cosmetic presentation enabled/disabled, then reset after repeated effects.

### Tests for User Story 5

- [X] T025 [US5] Add comparison tests for unchanged trajectories, impacts, damage, crater/mound deformation, settling, turn order, AI seed, and weapons with cosmetic work enabled/bypassed in `crates/azimuth-game/src/main.rs`.
- [ ] T026 [US5] Add reset/expiry tests proving queue, particle, and smoke-ready presentation resources leave no entities or mutable state after match restart in `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 5

- [X] T027 [US5] Audit and document renderer-only ordering around world generation, tank spawn, projectile resolution, and Update/FixedUpdate in `crates/azimuth-game/src/main.rs` against `docs/specs/20260911-214544-battlefield-visual-polish/contracts/presentation-boundary.md`.

**Checkpoint**: The visual pass is shown not to own or perturb the game.

---

## Phase 7: User Story 4 - See a Brief Aftermath (Priority: P2)

**Goal**: Add finite translucent profile-scaled smoke that rises, expands, fades, optionally drifts with read-only wind, and stays readable.

**Independent Test**: Repeatedly fire ordinary shots and Nuke; observe finite smoke and unchanged gameplay/readability.

### Tests for User Story 4

- [X] T028 [US4] Add smoke cap, scale/lifetime, rise/expand/fade, optional read-only wind drift, and expiry tests in `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 4

- [X] T029 [US4] Define shared translucent smoke assets and renderer-only puff state in `crates/azimuth-game/src/main.rs`, reusing impact requests and avoiding billboards, volumetrics, or fluid simulation.
- [X] T030 [US4] Spawn profile-scaled smoke from drained requests and update rise, optional wind drift, expansion, fade, caps, and cleanup in `crates/azimuth-game/src/main.rs`.
- [ ] T031 [US4] Run smoke/Nuke/repeated-shot acceptance in `docs/specs/20260911-214544-battlefield-visual-polish/quickstart.md`.

**Checkpoint**: Impacts leave a finite visual memory with no gameplay meaning.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Complete quality checks, graphical acceptance, and accurate roadmap work.

- [X] T032 Reconcile implementation comments/tests with `docs/specs/20260911-214544-battlefield-visual-polish/{spec,plan,research,data-model}.md` and `docs/specs/20260911-214544-battlefield-visual-polish/contracts/presentation-boundary.md`.
- [X] T033 Run `cargo test --workspace`, `cargo fmt --all -- --check`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings` from `/home/michael/src/games/azimuth` and fix feature-caused failures in `crates/azimuth-game/src/`.
- [ ] T034 Complete and record full 2-, 4-, and 8-player acceptance with every named weapon in `docs/specs/20260911-214544-battlefield-visual-polish/quickstart.md`.
- [ ] T035 Update only demonstrated visual-polish items and genuine follow-ups in `docs/roadmap.md`; do not complete unrelated work.

---

## Dependencies & Execution Order

### Phase Dependencies

- Setup starts immediately; foundational work depends on T001.
- US1, US2, and US3 start after foundation. US1/US2 are functionally independent; US3 uses shared caps.
- US5 depends on US1, US2, and US3 to compare all implemented presentation elements.
- US4 depends on US3's request queue/assets/lifecycle; extend US5 reset comparison after smoke exists.
- Polish depends on all completed stories and automated checks.

### User Story Dependencies

```text
Foundation
├── US1: Tank variety ───────────┐
├── US2: Materials ──────────────┼── US5: Simulation independence
└── US3: Explosion debris ──┬─────┘
                            └── US4: Smoke
US1 + US2 + US3 + US4 + US5 ─── Polish and roadmap review
```

### Parallel Opportunities

- After T003, US1 and US2 may proceed in parallel; coordinate shared `main.rs` changes before merge.
- T011 can run independently in `crates/azimuth-game/src/battlefield.rs` while US1 changes `main.rs`.
- Same-file `main.rs` implementation tasks omit `[P]` to avoid merge conflicts and preserve clear ordering.

## Implementation Strategy

### MVP First

1. Complete T001–T003.
2. Complete US1 (T004–T010): five cosmetic tanks with unchanged firing/gameplay.
3. Validate its automated and 2-/4-/8-player graphical acceptance before surface/effect work.

### Incremental Delivery

1. US1 delivers individual combatants.
2. US2 delivers material/world polish.
3. US3 establishes bounded energetic impacts.
4. US5 proves the presentation boundary before smoke complexity accumulates.
5. US4 adds finite aftermath; Phase 8 validates the complete experience.

## Format Validation

All 35 tasks use required checkbox, sequential ID, applicable story label, and exact file-path format.
Tests are included because the specification explicitly requires automated coverage.




