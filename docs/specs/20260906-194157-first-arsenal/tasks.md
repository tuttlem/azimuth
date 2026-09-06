---

description: "Dependency-ordered implementation tasks for First Arsenal"
---

# Tasks: First Arsenal — Extensible Weapon Framework, Selection and High Explosive

**Input**: Design documents from `/docs/specs/20260906-194157-first-arsenal/`

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md),
[data-model.md](data-model.md), [weapon contract](contracts/conventional-weapons.md), and
[quickstart.md](quickstart.md)

**Tests**: Required. Add domain-level tests before the corresponding implementation work; avoid
pixel, key-event, camera-timing, and renderer-entity tests unless no pure seam exists.

**Organization**: One small shared conventional-weapon boundary precedes independently demonstrable
selection, larger-consequence, immutable-shot, and extension-proof stories.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Different-file work with no incomplete-task dependency.
- **[Story]**: User-story traceability label.

## Phase 1: Setup

**Purpose**: The existing single crate, native HUD, and test conventions need no new dependency or
configuration work.

---

## Phase 2: Foundational Conventional Weapon Boundary

**Purpose**: Establish the shared data model and common blast parameter seam before either player
selection or HE-specific gameplay is wired into Bevy systems.

- [X] T001 Add failing catalogue, stable-identity, Basic baseline-profile, HE-profile, independent tests in `crates/azimuth-game/src/weapon.rs`.
  starting-loadout, unlimited-ammunition, and finite-ammunition tests in
  `crates/azimuth-game/src/weapon.rs`.
- [X] T002 Implement `WeaponId`, conventional `WeaponDefinition`/profiles, one central catalogue in `crates/azimuth-game/src/weapon.rs`.
  explicit unlimited/finite availability, and two-player loadout selection/consumption/fallback
  operations in `crates/azimuth-game/src/weapon.rs`.
- [X] T003 Add failing parameterised damage/falloff and simultaneous two-tank evaluation tests in `crates/azimuth-game/src/combat.rs`.
  a supplied conventional impact profile in `crates/azimuth-game/src/combat.rs`.
- [X] T004 Refactor the existing radial damage functions to accept a supplied ordinary impact profile in `crates/azimuth-game/src/combat.rs`.
  profile, preserving Basic Shell's current 6-unit/40-damage results in
  `crates/azimuth-game/src/combat.rs`.

**Checkpoint**: One catalogue and compact per-player loadout model represent both real weapons;
common deterministic damage accepts data rather than a weapon name.

---

## Phase 3: User Story 1 - Choose a Round Before Firing (Priority: P1) 🎯 MVP

**Goal**: The active player can select Basic Shell or available HE and truthfully see the choice
and ammunition without changing other authoritative turn state.

**Independent Test**: On a choosing turn select `1`/`2`, inspect the pure tactical HUD view, then
switch turns and verify the other player retains a separate selection and inventory.

### Tests for User Story 1

- [X] T005 [US1] Add failing selection-phase, inactive-player isolation, no-consumption, exhausted tests in `crates/azimuth-game/src/main.rs` and `crates/azimuth-game/src/weapon.rs`.
  HE rejection/fallback, HUD weapon/ammunition, and contextual-choice-hint tests in
  `crates/azimuth-game/src/main.rs` and `crates/azimuth-game/src/weapon.rs`.

### Implementation for User Story 1

- [X] T006 [US1] Add the authoritative two-player weapon-loadout resource in `crates/azimuth-game/src/main.rs`.
  players' starting loadouts, and expose narrow active-player selection helpers in
  `crates/azimuth-game/src/main.rs`.
- [X] T007 [US1] Add choosing-phase `1` Basic Shell / `2` High Explosive input validation in `crates/azimuth-game/src/main.rs`.
  movement/fire handling in `crates/azimuth-game/src/main.rs`.
- [X] T008 [US1] Extend the read-only tactical HUD view, top-right shot panel, and choosing-only hints in `crates/azimuth-game/src/main.rs`.
  controls hint with selected weapon and ASCII-safe truthful availability in
  `crates/azimuth-game/src/main.rs`.

**Checkpoint**: Players can see and select their own available conventional weapon; selection
cannot consume ammunition, alter aim, or leak between players.

---

## Phase 4: User Story 2 - Spend High Explosive for a Bigger Consequence (Priority: P1)

**Goal**: HE travels through the familiar ballistic model but applies its distinct captured blast,
crater, and restrained visual scale using the common consequence path.

**Independent Test**: Resolve comparable Basic and HE impacts in a domain scenario and verify the
HE damage profile and terrain height change are larger while wind/gravity flight and tank support
behaviour remain shared.

### Tests for User Story 2

- [X] T009 [US2] Add failing Basic-versus-HE impact, crater, terrain-height, visual-scale, and settling tests in `crates/azimuth-game/src/main.rs`.
  support/settling regression tests through the common resolver in
  `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 2

- [X] T010 [US2] Introduce immutable `FiredShot` state around the existing projectile in `crates/azimuth-game/src/main.rs` and `crates/azimuth-game/src/weapon.rs`.
  the active bare-projectile resource with it in `crates/azimuth-game/src/main.rs` and
  `crates/azimuth-game/src/weapon.rs`.
- [X] T011 [US2] Commit a selected available weapon atomically at fire time in `crates/azimuth-game/src/main.rs`.
  profile, consume a finite round once, safely fall back after final HE, and begin normal resolving
  fire—in `crates/azimuth-game/src/main.rs`.
- [X] T012 [US2] Advance the captured projectile and route terrain impact through its captured profile in `crates/azimuth-game/src/main.rs`.
  parameterised damage/crater profile, existing support/settling/turn completion, and proportionate
  existing explosion visual scale in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: Basic Shell preserves the original round, and HE is a clearly larger limited
ordinary explosive without a separate projectile or impact system.

---

## Phase 5: User Story 3 - Trust a Fired Weapon to Stay the Same (Priority: P1)

**Goal**: A committed shot remains independent of later selection, inventory, HUD, camera, and
presentation timing until its deterministic consequence sequence finishes.

**Independent Test**: Commit HE, mutate/select later player state in a pure test, then resolve its
impact and verify HE data was used and only one HE round was spent.

### Tests for User Story 3

- [X] T013 [US3] Add failing fired-shot identity/profile snapshot and mutation tests in `crates/azimuth-game/src/main.rs` and `crates/azimuth-game/src/weapon.rs`.
  rejected-fire non-consumption, no-impact committed-ammunition, and deterministic repeated-trace
  tests in `crates/azimuth-game/src/main.rs` and `crates/azimuth-game/src/weapon.rs`.

### Implementation for User Story 3

- [X] T014 [US3] Make every flight/impact resolver read only captured `FiredShot` data in `crates/azimuth-game/src/main.rs`.
  the existing authoritative damage → deformation → support/settling → survivor/handoff order in
  `crates/azimuth-game/src/main.rs`.
- [X] T015 [US3] Update camera, projectile visual, impact-marker, explosion, and HUD observation in `crates/azimuth-game/src/main.rs`.
  observation paths to observe the new active-shot representation without making presentation own
  weapon or turn state in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: In-flight conventional shots cannot be changed by any later mutable player or
presentation state.

---

## Phase 6: User Story 4 - Grow Ordinary Weapons Locally (Priority: P2)

**Goal**: A third conventional definition proves the catalogue-to-shot-to-impact path has no
scattered weapon-name rules.

**Independent Test**: A test-only distinct conventional definition creates a valid fired shot and
resolves common damage/crater consequences without adding a special core branch.

### Tests for User Story 4

- [X] T016 [US4] Add a test-only third conventional weapon definition in `crates/azimuth-game/src/weapon.rs` and `crates/azimuth-game/src/main.rs`.
  profile values and prove it uses the shared catalogue/loadout/fired-shot/impact path in
  `crates/azimuth-game/src/weapon.rs` and `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 4

- [X] T017 [US4] Refine catalogue and common-path APIs for the third-definition proof in `crates/azimuth-game/src/weapon.rs`.
  proof, and document the concrete-behaviour-only extension boundary in
  `crates/azimuth-game/src/weapon.rs`.

**Checkpoint**: A developer can add another normal explosive by defining its supported data rather
than modifying fire, projectile, combat, terrain, or HUD branches.

---

## Phase 7: Polish and Cross-Cutting Validation

- [X] T018 [P] Update actual weapon controls, Basic/HE values, ammunition rules, and snapshot docs in `README.md`, `docs/projectile-model.md`, and `docs/world-conventions.md`.
  flow, and extension rule in `README.md`, `docs/projectile-model.md`, and
  `docs/world-conventions.md`.
- [X] T019 Update only earned weapon selection/display and arsenal roadmap checkboxes in `docs/roadmap.md`.
  Basic/HE, weapon-specific explosion, independent inventory, and HUD roadmap checkboxes in
  `docs/roadmap.md`.
- [X] T020 Run all quality commands and manual validation recorded in `docs/specs/20260906-194157-first-arsenal/quickstart.md`.
  `cargo fmt --all -- --check`, and `cargo clippy --workspace --all-targets --all-features -- -D
  warnings`; perform the manual Basic/HE, exhaustion, settling, and full-duel checks in
  `docs/specs/20260906-194157-first-arsenal/quickstart.md`.

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1**: No setup changes are required.
- **Phase 2**: T001–T004 establish the catalogue/loadout and shared blast seam; they block all
  weapon-selection and impact work.
- **US1 (Phase 3)**: Depends on Phase 2 and is the visible selection/HUD MVP.
- **US2 (Phase 4)**: Depends on Phase 2 and integrates selected data into the existing fire and
  impact route.
- **US3 (Phase 5)**: Depends on US2's active fired-shot representation.
- **US4 (Phase 6)**: Depends on the common path established through US2–US3.
- **Polish (Phase 7)**: Follows all feature implementation.

### User Story Dependencies

- **US1 (P1)**: Delivers independent selection and truthful HUD state after the shared model.
- **US2 (P1)**: Uses the same model to deliver distinct HE gameplay.
- **US3 (P1)**: Locks down the correctness of the committed shot created by US2.
- **US4 (P2)**: Demonstrates that all earlier work is a reusable conventional-weapon seam.

### Parallel Opportunities

- T003 can begin after the profile shape from T002 is established; it is isolated to `combat.rs`.
- T018 documentation files can proceed in parallel once final displayed wording and values are
  settled.
- Tests are intentionally sequential within `main.rs`; that file is the current sole Bevy wiring,
  input, HUD, and resolver seam.

## Implementation Strategy

### MVP First

1. Complete the Phase 2 data boundary and its pure tests.
2. Complete US1 to demonstrate selection, independent inventory state, and HUD truthfulness.
3. Validate that MVP before changing the actual projectile resource or impact resolver.

### Incremental Delivery

1. Phase 2 removes hard-coded ordinary blast values from the common combat seam.
2. US1 makes weapon choice visible and controllable.
3. US2 delivers the actual larger limited HE consequence through the existing gameplay loop.
4. US3 proves immutable authoritative commitment.
5. US4 proves another conventional definition is localised.
6. Polish documents and validates only verified work.

## Notes

- Every task identifies the exact target path and uses the mandatory checklist form.
- No task authorises a generic item/effects framework, exotic weapon, dependency, or visual polish
  beyond a scaled existing explosion.
- Mark tasks complete only after their tests/validation actually pass; leave manual acceptance open
  until it has been run on a graphical desktop.
