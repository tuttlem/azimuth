---

description: "Implementation tasks for First Duel — Damage, Elimination and Victory"
---

# Tasks: First Duel — Damage, Elimination and Victory

**Input**: Design documents from `/docs/specs/20260906-102046-first-duel-damage/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, and
`contracts/first-duel.md`

**Tests**: Required by FR-013. Add readable pure domain tests before integration tests; avoid
renderer/HUD text/pixel assertions for authoritative rules.

**Organization**: Each story delivers a testable increment. Damage and match authority must never
depend on the camera, boom, or other presentation timing.

## Phase 1: Setup (Shared Understanding)

**Purpose**: Confirm the current fixed impact/handoff seam and feature contract before editing.

- [X] T001 Review impact resolution, turn gates, tank visual root, and current tests in `crates/azimuth-game/src/main.rs`, `crates/azimuth-game/src/turn.rs`, and `crates/azimuth-game/src/tank.rs` against `docs/specs/20260906-102046-first-duel-damage/plan.md`.
- [X] T002 [P] Confirm values and manual outcomes in `docs/specs/20260906-102046-first-duel-damage/contracts/first-duel.md` and `docs/specs/20260906-102046-first-duel-damage/quickstart.md` are the implementation source of truth.

---

## Phase 2: Foundational (Combat and Match Domain Boundaries)

**Purpose**: Add the small authoritative representations shared by every duel story.

**⚠️ CRITICAL**: Complete this phase before UI or fixed-update integration.

- [X] T003 Add `combat` module registration and renderer-independent 3D world-position distance support in `crates/azimuth-game/src/main.rs`, `crates/azimuth-game/src/combat.rs`, and `crates/azimuth-game/src/world.rs`.
- [X] T004 Add shared 100-health tank survival state, saturating damage application, and elimination predicate in `crates/azimuth-game/src/tank.rs`.
- [X] T005 Add pure 6-unit/40-maximum linear-ceiling blast configuration and same-event two-tank damage calculation in `crates/azimuth-game/src/combat.rs`.
- [X] T006 Add explicit in-progress/winner/draw match state and survivor-aware turn transition primitives, with action gates, in `crates/azimuth-game/src/turn.rs`.
- [X] T007 Add unit tests for health initialization/clamping, full 3D distance, radius edge, falloff, pre-mutation multi-tank damage, and deterministic resolution in `crates/azimuth-game/src/combat.rs` and `crates/azimuth-game/src/tank.rs`.
- [X] T008 Add unit tests for next-survivor advancement, elimination/finished-action rejection, winner, and draw transitions in `crates/azimuth-game/src/turn.rs`.

**Checkpoint**: Domain state can calculate both blast damages and determine legal survivor-only turn
or finished result without Bevy entities, camera, or visual effects.

---

## Phase 3: User Story 1 - Hurt Tanks With Accurate Shots (Priority: P1) 🎯 MVP

**Goal**: Existing terrain impacts apply understandable deterministic splash damage, including
self-damage and a shared blast affecting both tanks.

**Independent Test**: Resolve fixed impacts outside, at, and inside the 6-unit radius and verify
the documented health deltas, crater result, and normal handoff independent of visual state.

### Tests for User Story 1

- [X] T009 [US1] Add impact-resolution integration tests for no-damage out-of-radius shots, reduced near-edge damage, centre-near/self damage, both-tank damage, and unchanged out-of-bounds-shot damage in `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 1

- [X] T010 [US1] Thread mutable tank survival state through fixed projectile terrain-impact resolution and apply one pure blast result before handoff in `crates/azimuth-game/src/main.rs`.
- [X] T011 [US1] Preserve the existing authoritative terrain crater and no-impact behavior while explicitly ordering blast damage, terrain deformation, and turn completion in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: A terrain impact can hurt either/both tanks deterministically; visual boom/camera
remain observational and a non-impact shot still does no damage.

---

## Phase 4: User Story 2 - Eliminate a Player and Continue Only With Survivors (Priority: P1)

**Goal**: Lethal health changes make a tank inert and prevent eliminated players from receiving or
using ordinary turns.

**Independent Test**: Apply a lethal impact to one tank while another survives, then attempt all
ordinary actions and verify only a surviving player may ever be selected.

### Tests for User Story 2

- [X] T012 [US2] Add integration tests proving lethal terrain-impact damage occurs before survivor handoff, eliminated players cannot aim/move/fire, and retained move-or-fire rules still work for a living player in `crates/azimuth-game/src/main.rs` and `crates/azimuth-game/src/turn.rs`.

### Implementation for User Story 2

- [X] T013 [US2] Connect post-impact survivor information to authoritative `TurnState` completion and movement completion so ordinary handoff selects only living players in `crates/azimuth-game/src/main.rs` and `crates/azimuth-game/src/turn.rs`.
- [X] T014 [US2] Add a read-only eliminated-tank visual synchronization system that hides/clearly disables the matching `TankVisual` root without deleting tank gameplay identity in `crates/azimuth-game/src/main.rs`.
- [X] T015 [US2] Extend the existing HUD projection with both players' current/maximum health and eliminated status while retaining live-player action feedback in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: Lethal shots leave an obvious eliminated tank; an eliminated player cannot act or
become current in an ongoing duel.

---

## Phase 5: User Story 3 - Finish and Announce the Duel (Priority: P2)

**Goal**: The final survivor wins, mutual elimination draws, and a finished duel rejects ordinary
controls while displaying its result.

**Independent Test**: Resolve controlled one-survivor and no-survivor blast scenarios; verify the
stored outcome, finished turn state, rejected actions, and result-focused HUD.

### Tests for User Story 3

- [X] T016 [US3] Add integration tests for winner and mutual-draw impact resolution, no post-match projectile launch/action, and identical outcomes regardless of presentation state in `crates/azimuth-game/src/main.rs` and `crates/azimuth-game/src/turn.rs`.

### Implementation for User Story 3

- [X] T017 [US3] Finalize match result during the same authoritative impact resolution that applies damage, stopping handoff when winner/draw is determined in `crates/azimuth-game/src/main.rs` and `crates/azimuth-game/src/turn.rs`.
- [X] T018 [US3] Present a prominent Player One win, Player Two win, or Draw status through the existing HUD and ensure the camera safely remains presentation-only after match completion in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: A local duel visibly ends with winner/draw and cannot start another ordinary turn
until the application is restarted.

---

## Phase 6: Polish & Cross-Cutting Completion

**Purpose**: Document actual rules, reconcile roadmap milestones honestly, and validate the full
duel.

- [X] T019 Update first-duel status, health/damage values, self-damage, elimination, victory, and restart boundary in `README.md`.
- [X] T020 [P] Update splash-damage distance and authoritative-resolution conventions in `docs/world-conventions.md` and `docs/projectile-model.md`.
- [X] T021 Update only demonstrated health/damage/turn/explosion/victory/HUD and reconciled Milestones B–E checkboxes in `docs/roadmap.md`.
- [X] T022 Reconcile the exact implemented behavior in `docs/specs/20260906-102046-first-duel-damage/contracts/first-duel.md` and `docs/specs/20260906-102046-first-duel-damage/quickstart.md`.
- [X] T023 Run the complete first-duel manual acceptance sequence and record only genuinely discovered later work in `docs/roadmap.md`.
- [X] T024 Run `cargo check --workspace --all-targets`, `cargo test --workspace`, `cargo fmt --all -- --check`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings` from the repository root.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Starts immediately.
- **Foundational (Phase 2)**: Depends on T001 and blocks all feature integration.
- **US1 (Phase 3)**: Depends on T003–T008; this is the damage MVP.
- **US2 (Phase 4)**: Depends on US1 impact integration and its domain state.
- **US3 (Phase 5)**: Depends on US2 survivor/match turn integration.
- **Polish (Phase 6)**: Depends on all three user stories.

### User Story Dependencies

```text
Foundational combat + match state
        └── US1: splash damage (MVP)
              └── US2: elimination and survivor-only turns
                    └── US3: winner/draw and finished-match lock
                          └── documentation and full validation
```

### Parallel Opportunities

- T002 can run with the source-seam review in T001.
- T004 and T005 touch distinct domain files after module registration, but T005 must use the final
  tank survival interface before its tests are finalized.
- T019 and T020 modify distinct documentation files once gameplay values are stable; roadmap work
  in T021 follows demonstrated behavior.
- Story test tasks are not marked parallel because each extends the same existing integration test
  module and should be kept conflict-free.

## Implementation Strategy

### MVP First

1. Complete T001–T008, including pure test coverage.
2. Complete US1 (T009–T011).
3. Validate repeatable damage, self-damage, and unchanged non-impact behavior before eliminations.

### Incremental Delivery

1. Damage makes accuracy meaningful.
2. Elimination makes damage alter turn ownership.
3. Winner/draw makes the loop a complete duel.
4. Documentation and full validation only follow verified behavior.
