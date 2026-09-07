---

description: "Actionable implementation tasks for Heavy Shell wind resistance"
---

# Tasks: Heavy Shell Wind Resistance

**Input**: Design documents from `/docs/specs/20260907-194343-heavy-shell-wind/`

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md),
[data-model.md](data-model.md), [conventional-weapons.md](contracts/conventional-weapons.md), and
[quickstart.md](quickstart.md)

**Tests**: Automated tests are required by the specification. Add focused deterministic unit and
game-level tests before or alongside each implementation step, then run the complete workspace
validation suite in the final phase.

**Organization**: Tasks are grouped by user story. The shared projectile boundary is established
first because every story depends on it; each later phase has an independent acceptance check.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel with tasks in a different file after its listed dependencies.
- **[Story]**: Maps the task to a user story in [spec.md](spec.md).
- Every task includes an exact repository path.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish the focused test surface and preserve normal-shell compatibility before
changing the projectile launch contract.

- [X] T001 Update test-only projectile launch/advance helpers to make normal wind response explicit in `crates/azimuth-game/src/projectile.rs`.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Add the one generic projectile characteristic required by every Heavy Shell flow.

**⚠️ CRITICAL**: Complete this phase before adding Heavy Shell catalogue, inventory, HUD, or impact
coverage.

- [X] T002 Add failing validation and fixed-step tests for finite non-negative response, normal response, scaled horizontal wind, unchanged gravity, and zero-wind equivalence in `crates/azimuth-game/src/projectile.rs`.
- [X] T003 Implement validated `WindResponse`, capture it on `Projectile`, accept it at launch, and scale only horizontal wind in shared `advance_with_terrain` in `crates/azimuth-game/src/projectile.rs`.
- [X] T004 Replace the empty `ProjectileProfile` with normal-response profile data and update all normal projectile construction call sites in `crates/azimuth-game/src/weapon.rs`, `crates/azimuth-game/src/main.rs`, and `crates/azimuth-game/src/projectile.rs`.

**Checkpoint**: Any conventional projectile can carry immutable generic wind response, while all
existing Basic Shell and High Explosive behaviour remains normal.

---

## Phase 3: User Story 1 - Keep a Shot on Line in Strong Wind (Priority: P1) 🎯 MVP

**Goal**: Heavy Shell is a limited conventional projectile whose same-input trajectory experiences
40% of ordinary horizontal wind response without changing gravity, launch controls, or impact role.

**Independent Test**: Compare same-launch Basic and Heavy projectile traces under controlled
crosswind, reversed wind, aligned/reversed-aligned wind, strong/weak wind, and zero wind; Heavy
Shell drifts proportionally less but remains wind-affected and deterministic.

### Tests for User Story 1

- [X] T005 [P] [US1] Add failing catalogue/profile assertions for stable Heavy Shell identity, `HEAVY SHELL` metadata, two-round rule, 0.40 response, normal Basic/HE response, and Basic Shell-class impact values in `crates/azimuth-game/src/weapon.rs`.
- [X] T006 [P] [US1] Add deterministic same-launch trajectory comparisons for 40% crosswind displacement, reversed crosswind, aligned/reversed-aligned wind, stronger-versus-weaker Heavy wind, non-immunity, zero-wind equality, unchanged vertical gravity, and repeat traces in `crates/azimuth-game/src/projectile.rs`.

### Implementation for User Story 1

- [X] T007 [US1] Add `WeaponId::HeavyShell`, `HEAVY_SHELL_STARTING_ROUNDS`, the reduced-response Heavy projectile profile, and the Basic Shell-class Heavy definition to the central catalogue in `crates/azimuth-game/src/weapon.rs`.
- [X] T008 [US1] Build each launched projectile from the committed weapon definition's projectile profile before creating `FiredShot` in `crates/azimuth-game/src/main.rs`.
- [X] T009 [US1] Expose only the generic response required by the trajectory tests and confirm shared projectile simulation has no `HeavyShell` identity/display-name condition in `crates/azimuth-game/src/projectile.rs`.

**Checkpoint**: A programmatically committed Heavy Shell has a reproducibly reduced but non-zero
wind effect under the common ballistic simulation.

---

## Phase 4: User Story 2 - Use a Third Conventional Weapon Naturally (Priority: P1)

**Goal**: Both players can select, see, fire, exhaust, and independently own Heavy Shell through
the ordinary selector, loadout, fired-shot, and tactical HUD paths.

**Independent Test**: Begin two default loadouts, select and fire Heavy Shell for one player,
verify only that count changes, exhaust it, and confirm generic fallback and HUD/selector behaviour.

### Tests for User Story 2

- [X] T010 [P] [US2] Add failing loadout tests for independent two-round Heavy Shell availability, selection non-consumption, one-round commitment, final-round Basic fallback, and rejected/exhausted selection in `crates/azimuth-game/src/weapon.rs`.
- [X] T011 [P] [US2] Add failing game-level selection and HUD assertions for the third normal selector, `HEAVY SHELL: x2`/remaining-round text, and no mutation outside a choosing turn in `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 2

- [X] T012 [US2] Extend the explicit per-player loadout availability, selection, commit, and fallback matching for Heavy Shell in `crates/azimuth-game/src/weapon.rs`.
- [X] T013 [US2] Extend normal direct weapon selection and concise choosing-phase controls so all three weapons are reachable, while retaining generic catalogue-backed weapon/ammunition HUD formatting in `crates/azimuth-game/src/main.rs`.
- [X] T014 [US2] Extend fired-shot snapshot regression coverage so changing later selection or availability cannot alter a committed Heavy Shell response or impact profile in `crates/azimuth-game/src/weapon.rs`.

**Checkpoint**: A player can naturally select `HEAVY SHELL x2`, fire to `x1`, and exhaust it with
no effect on the other player, Basic Shell, or the existing action locks.

---

## Phase 5: User Story 3 - Preserve the Ordinary Impact Game (Priority: P2)

**Goal**: Heavy Shell uses the ordinary swept terrain collision and Basic Shell-class impact,
damage, deformation, settling, victory, and handoff path without a separate effect mechanic.

**Independent Test**: Resolve a controlled Heavy Shell terrain impact and compare its shared
consequences with Basic Shell, while confirming High Explosive retains the larger blast/crater role.

### Tests for User Story 3

- [X] T015 [US3] Add a controlled Heavy Shell terrain-impact regression that proves one shared collision/impact result, Basic Shell-class damage and crater, tank support/settling, winner/draw handoff, and preserved High Explosive distinction in `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 3

- [X] T016 [US3] Keep `resolve_projectile_advance` and the shared combat/deformation pipeline identity-agnostic while making any test fixture use the Heavy Shell catalogue definition in `crates/azimuth-game/src/main.rs` and `crates/azimuth-game/src/combat.rs`.

**Checkpoint**: Heavy Shell's different landing location is its only environmental distinction; its
impact remains an ordinary Basic Shell-class consequence.

---

## Phase 6: User Story 4 - Extend Projectile Behaviour Without Weapon-Name Logic (Priority: P2)

**Goal**: The framework demonstrates the authoritative flow from weapon profile to fired projectile
to common simulation, with no mutable catalogue dependency while a shot is in flight.

**Independent Test**: Inspect a committed Heavy Shell and prove its projectile retains reduced
response after player selection/inventory changes; compare a test conventional profile through the
same flight boundary without adding identity-specific simulator logic.

### Tests for User Story 4

- [X] T017 [US4] Add a profile-to-projectile snapshot test and a distinct test-conventional-response trace proving the common simulator reads captured generic response rather than weapon identity in `crates/azimuth-game/src/weapon.rs` and `crates/azimuth-game/src/projectile.rs`.

### Implementation for User Story 4

- [X] T018 [US4] Refactor only if required by T017 to keep the authoritative `WeaponDefinition → ProjectileProfile → Projectile → FiredShot → shared simulation` flow explicit and free of catalogue lookup during flight in `crates/azimuth-game/src/weapon.rs`, `crates/azimuth-game/src/projectile.rs`, and `crates/azimuth-game/src/main.rs`.

**Checkpoint**: Code review can identify a generic response property in common flight and no
Heavy-Shell-specific simulation branch.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Document the completed gameplay model, validate regressions, and record only genuine
manual-play roadmap evidence.

- [X] T019 [P] Document Heavy Shell's two-round, 0.40 wind-response gameplay identity; normal gravity; ordinary impact; and selector controls in `README.md`.
- [X] T020 [P] Document projectile wind-response composition, zero-wind invariance, and explicit non-mass/non-drag boundary in `docs/projectile-model.md`.
- [X] T021 Update only genuinely satisfied Heavy Shell, wind-gameplay, and Arsenal roadmap items; record manual tuning observations without checking unsupported milestones in `docs/roadmap.md`.
- [X] T022 Run the complete automated validation commands from `docs/specs/20260907-194343-heavy-shell-wind/quickstart.md` against `Cargo.toml` and resolve all failures or warnings.
- [X] T023 Perform and record the calm/low-wind, noticeable-crosswind, reversed/strong-wind, inventory, impact, settling, victory, and complete-two-player-match checks from `docs/specs/20260907-194343-heavy-shell-wind/quickstart.md` in `docs/specs/20260907-194343-heavy-shell-wind/quickstart.md`.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1**: Starts immediately.
- **Phase 2**: Depends on T001; blocks every story because all flight needs generic captured wind response.
- **US1 (Phase 3)**: Depends on T002–T004; establishes the core Heavy Shell ballistic value.
- **US2 (Phase 4)**: Depends on T007–T008; makes the already-defined Heavy Shell player-accessible.
- **US3 (Phase 5)**: Depends on US1 and US2, because it resolves the real catalogue weapon through normal play state.
- **US4 (Phase 6)**: Depends on US1; it validates the complete profile/snapshot/common-simulation boundary after it exists.
- **Polish (Phase 7)**: Depends on all desired story phases; T019 and T020 can run in parallel, then T021–T023 run in order as evidence becomes available.

### User Story Dependencies

```text
Foundational generic wind response
        └── US1: reduced common-flight response
              ├── US2: selector, inventory, HUD, snapshots
              │     └── US3: ordinary full impact pipeline
              └── US4: architecture-proof regression
                       └── Polish and manual validation
```

### Parallel Opportunities

- T019 and T020 edit different documentation files and can proceed together after the behaviour is stable.
- After T004, T005 and T006 can be prepared in parallel if their changes are coordinated because
  they target different modules; T007 must land before either test can pass.
- After T008, T010 and T011 can be prepared in parallel in `weapon.rs` and `main.rs` respectively.
- T015 and T017 can be prepared in parallel after their prerequisites, but their final integration
  should wait until T012–T014 preserve the complete real-shot boundary.

## Parallel Example: User Story 2

```text
After the Heavy Shell definition and launch snapshot exist:

Task: "Add failing loadout tests in crates/azimuth-game/src/weapon.rs"       # T010
Task: "Add failing selector/HUD tests in crates/azimuth-game/src/main.rs"   # T011
```

## Implementation Strategy

### MVP First

1. Complete T001–T004 to establish the generic deterministic property.
2. Complete T005–T009 to prove the reduced ordinary ballistic response.
3. Complete T010–T014 so players can actually make the Heavy Shell choice.
4. Run the US1/US2 independent tests before progressing to impact and architectural regressions.

### Incremental Delivery

1. Generic projectile response → no weapon-specific physics branch.
2. Heavy Shell catalogue and captured flight → demonstrable wind-resistant trajectory.
3. Loadout, selection, and HUD → playable limited-ammunition tactical choice.
4. Ordinary impact and framework regressions → safe integration with all existing duel systems.
5. Documentation, full quality checks, and manual matches → evidence-based roadmap update.

## Notes

- Tasks intentionally do not create a mass, drag, generic weapon scripting, camera, or prediction
  system.
- The golf-style projectile camera is a separately recorded roadmap item and must not be absorbed
  into this feature.
- Do not mark Arsenal tactical-choice milestones solely because a third weapon exists; use T023
  play evidence.
