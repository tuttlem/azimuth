---

description: "Actionable task list for First Boom — Visible Projectile Explosion"
---

# Tasks: First Boom — Visible Projectile Explosion

**Input**: Design documents from `/docs/specs/20260905-181606-visible-projectile-explosion/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, and `quickstart.md`

**Tests**: Add focused Rust unit tests for extracted deterministic presentation lifecycle and
one-shot-consumption logic. Visual appearance is verified manually, not with screenshots.

**Organization**: Tasks are grouped by user story so each increment has a clear observable result.

## Phase 1: Setup (Shared Understanding)

**Purpose**: Establish the existing simulation-to-presentation handoff and the documentation
locations that this feature is allowed to change.

- [X] T001 Review the current impact handoff, marker lifecycle, and launch-result clearing in `crates/azimuth-game/src/main.rs` before adding presentation state.
- [X] T002 Review the approved explosion constraints and manual checks in `docs/specs/20260905-181606-visible-projectile-explosion/{spec,plan,research,data-model,quickstart}.md`.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Add the minimal local state needed to consume a persistent authoritative impact exactly
once without changing simulation or marker ownership.

**⚠️ CRITICAL**: Complete this phase before spawning any effect from a terrain impact.

- [X] T003 Add a small presentation-only current-impact consumption resource and its reset semantics to `crates/azimuth-game/src/main.rs`.

**Checkpoint**: The persistent `LatestTerrainImpact` can be observed once per shot without relying
on its position value or removing the result needed by the marker.

---

## Phase 3: User Story 1 - See an Impact Explode (Priority: P1) 🎯 MVP

**Goal**: An authoritative terrain impact produces one unmistakable expanding, temporary boom at
its exact resolved world-space position.

**Independent Test**: Run the focused unit tests for lifecycle progress, scale, and expiry; then
press `I` in the running game and confirm one bright boom begins at the marker, changes visibly,
and disappears.

### Tests for User Story 1

> **NOTE: Write these tests first and confirm they fail before implementing the helpers.**

- [X] T004 [US1] Add unit tests for initial, intermediate, terminal, and expired presentation lifetime values in `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 1

- [X] T005 [US1] Define presentation-only explosion duration, initial and maximum visual scales, bright primitive assets, and a tagged elapsed-lifetime component in `crates/azimuth-game/src/main.rs`.
- [X] T006 [US1] Implement pure lifetime-progress and visual-scale helpers used by the explosion update in `crates/azimuth-game/src/main.rs`.
- [X] T007 [US1] Spawn exactly one bright explosion primitive from the existing `LatestTerrainImpact` position in `crates/azimuth-game/src/main.rs` without rechecking terrain or changing projectile state.
- [X] T008 [US1] Advance active explosion presentation with frame delta, update its scale, and despawn it at explicit lifetime expiry in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: A terrain-impact result alone produces a visible, short-lived boom; flight and
out-of-bounds termination remain unchanged.

---

## Phase 4: User Story 2 - Keep Exact Impact Diagnostics Useful (Priority: P2)

**Goal**: The exact marker remains an independent, enduring diagnostic while the deliberately
exaggerated presentation effect starts and expires.

**Independent Test**: Fire `I`, wait until the boom is removed, and confirm the marker still shows
the resolved impact point; verify that disabling/removing marker presentation would not affect the
explosion's source or lifecycle.

- [X] T009 [US2] Keep explosion spawning and lifetime systems independent of `ImpactMarker` entity state while preserving the existing marker synchronization in `crates/azimuth-game/src/main.rs`.
- [X] T010 [US2] Add a regression test that the one-shot consumption decision is driven by impact presence rather than marker entity availability in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: The marker and boom coexist at the authoritative point, and expiration of the boom
does not remove or obscure the diagnostic marker.

---

## Phase 5: User Story 3 - Repeat the Impact Feedback (Priority: P3)

**Goal**: Each sequential terrain impact receives one new boom, and expired effect entities do not
accumulate.

**Independent Test**: Use deterministic unit tests to prove consumption resets after the current
impact clears; manually fire `I`, wait for expiry, fire `I` again, and observe exactly one new boom.

### Tests for User Story 3

- [X] T011 [US3] Add unit tests covering one spawn for a persistent impact, reset after impact clearance, and a new spawn for a later identical impact in `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 3

- [X] T012 [US3] Complete the consumption-reset integration so a new launch clearing `LatestTerrainImpact` re-arms one explosion without position comparison or stale presentation state in `crates/azimuth-game/src/main.rs`.
- [X] T013 [US3] Verify expired explosion entities are the only entities despawned by the effect lifecycle and repeated sequential impacts leave no obsolete effect state in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: Repeated impact shots produce one boom each, while the latest marker remains and
completed explosions leave no active presentation entities.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Document the finished presentation boundary, update only fulfilled roadmap work, and
validate the workspace and manual scenario.

- [X] T014 [P] Document the temporary presentation-only boom, its authoritative impact origin, and marker relationship in `README.md`.
- [X] T015 [P] Document the boom lifecycle and explicit non-gameplay visual scale in `docs/projectile-model.md`.
- [X] T016 Update only the fulfilled visible-impact and explosion-visual-effect checkboxes, and record any newly discovered nonessential work, in `docs/roadmap.md`.
- [X] T017 Run `cargo check --workspace --all-targets`, `cargo test --workspace`, `cargo fmt --all -- --check`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings` from the repository root.
- [ ] T018 Execute the complete desktop acceptance flow in `docs/specs/20260905-181606-visible-projectile-explosion/quickstart.md` and record any environment limitation or follow-up in that file only if needed.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies.
- **Phase 2 (Foundational)**: Depends on T001–T002 and blocks all story implementation.
- **US1 (Phase 3)**: Depends on T003; T004 precedes T006, and T005–T006 precede T007–T008.
- **US2 (Phase 4)**: Depends on the completed US1 effect; T009 precedes T010.
- **US3 (Phase 5)**: Depends on T003 and the US1 spawn path; T011 precedes T012–T013.
- **Polish (Phase 6)**: Depends on completed US1–US3 implementation; T014 and T015 can run in parallel, then complete T016–T018.

### User Story Dependencies

- **US1 (P1)**: The MVP; needs only the existing terrain-impact handoff plus foundational
  one-shot state.
- **US2 (P2)**: Builds on US1 because it validates the effect against the existing persistent
  marker, but does not add simulation behaviour.
- **US3 (P3)**: Builds on the one-shot tracker and US1 spawn path to validate reset and cleanup.

### Parallel Opportunities

- T014 and T015 touch different documentation files and can run in parallel after the feature
  behaviour is complete.
- The code tasks intentionally are not marked parallel: they modify one small, tightly coupled
  presentation lifecycle in `crates/azimuth-game/src/main.rs` and are safest in dependency order.

## Parallel Example: Polish Documentation

```text
Task: "Document the temporary presentation-only boom, its authoritative impact origin, and marker relationship in README.md"
Task: "Document the boom lifecycle and explicit non-gameplay visual scale in docs/projectile-model.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete T001–T003 to establish one-shot presentation consumption.
2. Complete T004–T008 to add and test the visible temporary boom.
3. Run the focused tests and manually fire `I` to verify the boom begins at the authoritative
   marker position and expires.

### Incremental Delivery

1. Deliver US1: fire → impact → one temporary visible boom.
2. Deliver US2: retain the diagnostic marker independently through effect expiry.
3. Deliver US3: re-arm consumption for the next shot and prove cleanup remains bounded.
4. Finish documentation, roadmap alignment, quality gates, and the quickstart acceptance flow.
