---

description: "Dependency-ordered implementation tasks for tank support, gravity, and terrain settling"
---

# Tasks: Tank Support, Gravity and Terrain Settling

**Input**: Design documents from `/docs/specs/20260906-141906-tank-support-settling/`

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md),
[data-model.md](data-model.md), [tank-support contract](contracts/tank-support.md), and
[quickstart.md](quickstart.md)

**Tests**: Required. The specification makes authoritative deterministic simulation and turn
resolution testable behavior; write the listed tests first and confirm each fails before its
implementation task.

**Organization**: Tasks are grouped by independently testable user story. The shared gravity
accessor is the only foundation; all gameplay work stays in the existing game crate.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel because it touches a different file and has no incomplete-task
  dependency.
- **[US#]**: Story traceability label; omitted only for shared foundation and polish work.

## Phase 1: Setup

**Purpose**: No project or dependency setup is needed. The active feature documents, existing
single-crate workspace, fixed simulation schedule, and unit-test harness are ready to use.

---

## Phase 2: Foundational (Blocking Prerequisite)

**Purpose**: Expose the existing validated battlefield gravity to the tank domain without copying
its value or creating a second physics configuration.

- [X] T001 Add a minimal public read-only downward-acceleration accessor and focused validity
  coverage to `crates/azimuth-game/src/projectile.rs` so tank settling reuses `Gravity`.

**Checkpoint**: Tank-domain code can consume the existing configured gravity and fixed timestep;
no user-story implementation begins before this is complete.

---

## Phase 3: User Story 1 - Drop Into Newly Removed Ground (Priority: P1) 🎯 MVP

**Goal**: A living tank responds only to terrain loss at its own base, visibly falls
deterministically, and contacts the current deformed surface without penetration.

**Independent Test**: In pure tank/terrain tests, lower terrain beneath a known tank and advance
fixed settling steps until it grounds; contrast it with remote deformation and equal-or-within-
tolerance support.

### Tests for User Story 1

- [X] T002 [US1] Add failing support-reconciliation tests in
  `crates/azimuth-game/src/tank.rs` for stable terrain, 0.05 tolerance snapping, terrain rise,
  remote crater stability, lowered support, and preserved X/Z/body/turret directions.
- [X] T003 [US1] Add failing fixed-settling tests in `crates/azimuth-game/src/tank.rs` for
  predictable gravity position/velocity advancement, terrain contact clamping, no penetration,
  repeated deformation, zero-gravity suspended state, eliminated-tank exclusion, and identical
  traces.

### Implementation for User Story 1

- [X] T004 [US1] Add the compact supported/falling state, support tolerance, terrain
  reconciliation, and living-tank predicates to `crates/azimuth-game/src/tank.rs`; initialize
  every spawned tank as supported and preserve existing movement/firing behavior.
- [X] T005 [US1] Implement deterministic vertical fixed-step settling and current-terrain contact
  in `crates/azimuth-game/src/tank.rs`, reusing `Gravity` and `FIXED_STEP_SECONDS`; clear vertical
  motion on contact while preserving horizontal position and existing orientation.

**Checkpoint**: A pure domain scenario demonstrates that terrain beneath a tank, rather than blast
proximity, determines whether it drops and that the final base matches the authoritative terrain.

---

## Phase 4: User Story 2 - Finish a Shot Only After Ground Consequences Settle (Priority: P1)

**Goal**: A terrain-impact fire turn remains authoritatively resolving while any living tank is
falling, then completes exactly once only after the final grounded state or match result.

**Independent Test**: Drive an impact that creates a crater beneath a living tank through fixed
updates; verify `ResolvingFire` rejects ordinary actions until settlement and handoff occurs once.

### Tests for User Story 2

- [X] T006 [US2] Add failing turn-resolution regression tests in
  `crates/azimuth-game/src/main.rs` for immediate handoff when all living tanks remain supported,
  deferred handoff while a living tank falls, exactly-once completion after contact, and no
  out-of-bounds regression.
- [X] T007 [US2] Add failing gameplay integration tests in `crates/azimuth-game/src/main.rs` that
  prove impact damage occurs once before deformation/settling, ordinary actions remain locked,
  unaffected tanks create no delay, eliminated tanks do not settle, and zero gravity cannot
  accidentally advance a suspended resolving turn.

### Implementation for User Story 2

- [X] T008 [US2] Generalize the resolution-completion name and its existing tests in
  `crates/azimuth-game/src/turn.rs` so it describes completion after all authoritative firing
  consequences while retaining its one-shot `ResolvingFire` guard.
- [X] T009 [US2] Update `resolve_projectile_advance` in `crates/azimuth-game/src/main.rs` to apply
  existing damage once, deform terrain, reconcile every living tank, and complete immediately only
  when none require settling; preserve the no-impact path.
- [X] T010 [US2] Add and chain an authoritative fixed settling system after projectile advancement
  in `crates/azimuth-game/src/main.rs`; advance only living falling tanks and call the existing
  survivor/match completion exactly once after all living tanks are supported.

**Checkpoint**: A crater-under-tank shot blocks the next player until contact; a remote or
out-of-bounds shot retains the existing prompt handoff; neither camera nor boom timing participates.

---

## Phase 5: User Story 3 - Keep Playing From the Settled Position (Priority: P2)

**Goal**: A settled survivor retains aim and later moves/fires using its new authoritative base
position, making terrain collapse a lasting positional consequence.

**Independent Test**: Settle a known tank, preserve its `AimingState`, then derive one valid later
movement step and one firing representation; both must use the settled pose.

### Tests for User Story 3

- [X] T011 [US3] Add settled-pose regression coverage in `crates/azimuth-game/src/tank.rs` proving
  `step_on_terrain` starts from the final settled pose and retains the existing terrain-bound
  movement rules.
- [X] T012 [US3] Add settled firing-origin and retained-aim regression coverage in
  `crates/azimuth-game/src/main.rs` proving later launch parameters use the settled tank height
  without changing azimuth, elevation, or power.

### Implementation for User Story 3

- [X] T013 [US3] Ensure tank pose synchronization and existing firing-representation usage in
  `crates/azimuth-game/src/main.rs` continue to read the authoritative settling-updated pose,
  making the visible root and later muzzle origin agree without a separate transform state.

**Checkpoint**: A tank that drops into a crater can complete later move-or-fire turns from its
settled location and fires the same remembered shot values from a changed world-space origin.

---

## Phase 6: Polish and Cross-Cutting Validation

**Purpose**: Make the new authoritative behavior discoverable, align the roadmap honestly, and
verify the complete existing game remains healthy.

- [X] T014 [P] Update support, settling order, retained aim, no-fall-damage, and zero-gravity
  boundary documentation in `docs/world-conventions.md` and `docs/projectile-model.md`.
- [X] T015 [P] Update the playable-duel status and controls/resolution wording in `README.md` to
  replace the obsolete statement that stationary tanks do not settle.
- [X] T016 Update only proven support/handoff checkboxes and record the deliberate no-fall-damage
  decision in `docs/roadmap.md`.
- [X] T017 Run the documented automated and manual acceptance flow in
  `docs/specs/20260906-141906-tank-support-settling/quickstart.md`: `cargo check --workspace
  --all-targets`, `cargo test --workspace`, `cargo fmt --all -- --check`, and `cargo clippy
  --workspace --all-targets --all-features -- -D warnings`, then verify the crater-under-tank
  local-duel scenario.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1**: No work required; existing project structure is ready.
- **Phase 2**: T001 blocks all tank-settling implementation because it supplies the one gravity
  source.
- **Phase 3 / US1**: T002 and T003 precede T004 and T005; it depends on T001 only.
- **Phase 4 / US2**: T006 and T007 precede T008–T010; it depends on settled domain behavior from
  T004–T005.
- **Phase 5 / US3**: T011–T013 depend on the completed US1 settling state and US2 integration.
- **Phase 6**: T014–T016 follow the delivered behavior. T017 follows every implementation and
  documentation task.

### User Story Dependencies

- **US1 (P1)**: Starts after T001 and is independently demonstrable with pure domain tests.
- **US2 (P1)**: Depends on US1 because it advances the tank state through the fire-resolution
  boundary; it is independently demonstrable by fixed-update integration tests.
- **US3 (P2)**: Depends on the settled pose delivered by US1/US2; it independently verifies the
  later move/fire consequences of that pose.

### Parallel Opportunities

- T014 and T015 can proceed in parallel after the behavior and wording are known because they
  modify different documentation files.
- Test authoring can be divided by file before implementation: `tank.rs` coverage (T002–T003,
  T011) and `main.rs` integration coverage (T006–T007, T012). Integrate them in the stated order
  because each file is shared.

## Parallel Example: Documentation

```text
Task: "Update support and shot-model documentation in docs/world-conventions.md and docs/projectile-model.md"
Task: "Update playable-duel status and controls wording in README.md"
```

## Implementation Strategy

### MVP First (US1)

1. Complete T001.
2. Write T002–T003, then implement T004–T005.
3. Run the focused tank tests and demonstrate a tank settling onto a crater surface in a domain
   scenario before changing turn integration.

### Incremental Delivery

1. US1 makes terrain support physically coherent at the domain level.
2. US2 connects that coherent state to authoritative shot completion without presentation gates.
3. US3 proves the new position matters in later tactical movement and artillery firing.
4. Polish updates documentation/roadmap and runs the complete quality/manual validation flow.

## Notes

- Every task uses the required checkbox, sequential ID, and exact path format.
- The implementation must not add acceleration controls, suspension, sliding, fall damage, wreck
  physics, or a general physics framework.
- Do not mark crater tactical-effect or deliberate-movement-gravity roadmap work complete unless
  manual play demonstrates its separate acceptance criteria.
