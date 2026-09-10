# Tasks: Arsenal Pack #2 — Dirt Bomb, Curve Ball, Bouncer and Nuke

**Input**: Design documents from `docs/specs/20260910-224611-arsenal-pack-two/`

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md),
[data-model.md](data-model.md), and [gameplay/UI contract](contracts/arsenal-pack-two-contract.md)

**Tests**: Deterministic tests are required by the feature specification and must be written before
the corresponding implementation.

## Phase 1: Setup and Foundation

**Purpose**: Establish the small shared vocabulary required by the four concrete weapons.

- [X] T001 Review the mound, curve, surface-normal, bounded-bounce, strip, and presentation decisions in `docs/specs/20260910-224611-arsenal-pack-two/{plan,research,data-model}.md`
- [X] T002 [P] Add terrain deformation/normal tests for deterministic mounds, unchanged exterior samples, edge validity, and triangle normals in `crates/azimuth-game/src/battlefield.rs`
- [X] T003 [P] Add projectile tests proving zero extra acceleration preserves ordinary flight and fixed lateral acceleration composes with gravity/wind in `crates/azimuth-game/src/projectile.rs`
- [X] T004 Add the smallest validated mound deformation and exact current-surface normal query in `crates/azimuth-game/src/battlefield.rs`
- [X] T005 Add the narrow optional fixed-step extra-acceleration path while preserving existing projectile behaviour in `crates/azimuth-game/src/projectile.rs`
- [X] T006 Add bounded Bouncer committed-shot configuration in `crates/azimuth-game/src/weapon.rs`
- [X] T007 Refactor common final terrain consequence handling so crater or mound, damage, impact record, support reconciliation, settling, and handoff stay ordered in `crates/azimuth-game/src/main.rs`

**Checkpoint**: Existing shots remain unchanged with zero added acceleration and shared consequence handling is ready.

---

## Phase 2: User Story 1 — Build and Bury with Dirt (Priority: P1) 🎯 MVP

**Goal**: A player can create a substantial, deterministic mound with an ordinary ballistic Dirt Bomb instead of a damaging crater.

**Independent Test**: Fire onto flat terrain, a crater, slope, and tank-adjacent ground; terrain rises only in the mound region, tanks remain valid, and one round is consumed.

- [X] T008 [P] [US1] Add Dirt Bomb inventory, ordinary-flight, deterministic mound, exterior-unchanged, low-damage, and support/settling tests in `crates/azimuth-game/src/{weapon,main}.rs`
- [X] T009 [US1] Add Dirt Bomb definition, limited loadout, visual identity, and strip label/order entry in `crates/azimuth-game/src/{weapon,main}.rs`
- [X] T010 [US1] Route Dirt Bomb terrain impact through mound application and ordinary reconciliation/complete-shot resolution in `crates/azimuth-game/src/main.rs`
- [X] T011 [US1] Add Dirt Bomb impact framing and restrained terrain-growth feedback without introducing a conventional blast in `crates/azimuth-game/src/main.rs`

**Checkpoint**: Dirt Bomb is independently playable as terrain creation rather than damage.

---

## Phase 3: User Story 2 — Bend a Learned Shot (Priority: P1)

**Goal**: A player can choose a visible left/right Curve Ball setting and learn a deterministic non-homing curved trajectory.

**Independent Test**: Equal left/right fixtures finish on opposite sides of their launch line under calm and crosswind conditions, while vertical gravity and ordinary weapons remain unchanged.

- [X] T012 [P] [US2] Add Curve Ball inventory, lateral-force, gravity/wind, non-homing, and deterministic fixed-step tests in `crates/azimuth-game/src/{weapon,projectile,main}.rs`
- [X] T013 [US2] Add Curve Ball definition/loadout and apply held Left/Right launch-relative lateral acceleration only to its authoritative flight in `crates/azimuth-game/src/{weapon,main}.rs`
- [X] T014 [US2] Add Curve Ball to the 12-slot strip and flight steering help without requiring another keyboard shortcut in `crates/azimuth-game/src/main.rs`
- [X] T015 [US2] Verify Human/AI camera observation and AI safe behaviour without AI weapon strategy changes in `crates/azimuth-game/src/{main,ai}.rs`

**Checkpoint**: Curve Ball is independently playable as a learnable around-terrain shot.

---

## Phase 4: User Story 3 — Make a Bank Shot (Priority: P1)

**Goal**: Bouncer uses current 3D terrain orientation for bounded energy-losing ricochets before one normal final explosion.

**Independent Test**: Flat, slope, and valley fixtures show a first non-explosive bounce, reduced reflected velocity, gravity/wind continuation, three-or-fewer bounces, and one eventual ordinary impact.

- [X] T016 [P] [US3] Add Bouncer inventory, normal, energy-loss, bounded-count, and deterministic regression coverage in `crates/azimuth-game/src/{weapon,projectile,main}.rs`
- [X] T017 [US3] Add Bouncer definition/loadout/strip identity and a bounded bounce state to committed shots in `crates/azimuth-game/src/weapon.rs`
- [X] T018 [US3] Implement outward-offset terrain-normal reflection and final-impact fall-through to the common resolver in `crates/azimuth-game/src/main.rs`
- [X] T019 [US3] Preserve ordinary terrain-context flight presentation without a full impact flash or explosion on intermediate bounces in `crates/azimuth-game/src/main.rs`

**Checkpoint**: Bouncer is independently playable as a genuine 3D bank-shot weapon.

---

## Phase 5: User Story 4 — Spend a Nuke (Priority: P1)

**Goal**: One limited Nuke produces a dramatic but deterministic wide-area ordinary impact, including self-damage and stable end-of-match semantics.

**Independent Test**: A representative populated-region impact consumes one round, materially exceeds HE radius/crater scale, can affect multiple tanks including the shooter, and preserves winner/draw resolution.

- [X] T020 [P] [US4] Add Nuke inventory, wide-profile, deterministic-deformation, and winner/draw regression coverage in `crates/azimuth-game/src/{weapon,combat,main}.rs`
- [X] T021 [US4] Add one-round Nuke definition/loadout/strip identity with a deliberately exceptional but bounded impact profile in `crates/azimuth-game/src/{weapon,main}.rs`
- [X] T022 [US4] Extend high-scale impact presentation with capped comfortable flash/audio and a large visual while preserving the common authoritative resolver in `crates/azimuth-game/src/main.rs`

**Checkpoint**: Nuke is independently playable as the arsenal's catastrophic single-round decision.

---

## Phase 6: Polish and Completion

**Purpose**: Validate the entire twelve-weapon arsenal, document accepted work, and preserve future boundaries.

- [X] T023 Update click-first controls, flight steering, terrain addition, curve, bounce, Nuke lifecycle, and presentation behaviour in `docs/projectile-model.md`
- [X] T024 Update accepted Dirt Bomb, Bouncer, Curve Ball, large-scale destructive, and Milestone G roadmap items while retaining Death Sphere and excluded work in `docs/roadmap.md`
- [X] T025 Add/repair twelve-weapon strip, availability, conventional, MIRV/Cluster/Net/Roller/Bunker, and Basic-Shell-AI regression tests in `crates/azimuth-game/src/{weapon,main,ai}.rs`
- [X] T026 Run formatting, full game tests, Clippy, and document the manual quickstart acceptance for dirt, curve, bounce, Nuke, strip, and camera review in `docs/specs/20260910-224611-arsenal-pack-two/quickstart.md`

## Dependencies and Execution Order

- T001–T007 form the shared foundation and block story implementation.
- US1–US4 are separate weapon increments after the foundation; sequential delivery is recommended because each extends the same committed-shot resolver.
- T023–T026 follow all accepted stories.

### Parallel opportunities

- T002 and T003 can run together; they cover distinct terrain and projectile modules.
- Within each story, its test task can begin once its direct foundation is available; implementation in `main.rs` remains sequential with other story work.
- Documentation T023 and roadmap T024 can proceed together after implementation stabilizes.

## Implementation Strategy

1. Complete the foundation and prove ordinary ballistics remain unchanged.
2. Deliver Dirt Bomb as the MVP terrain-creation slice and validate support/settling.
3. Deliver Curve Ball, then Bouncer, then Nuke as independently testable increments.
4. Finish only after full regression, manual twelve-weapon strip review, and roadmap/documentation updates.

## Format Validation

All 26 tasks use the required checkbox, sequential ID, optional parallel marker, user-story label for story work, and concrete file path format.
