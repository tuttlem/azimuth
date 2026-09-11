---

description: "Implementation tasks for randomized round starts"
---

# Tasks: Randomized Round Starts

**Input**: Design documents from `/docs/specs/20260912-091903-random-round-starts/`

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [round-generation.md](contracts/round-generation.md), and [quickstart.md](quickstart.md)

**Tests**: Required. The specification sets measurable deterministic-generation, reset, and valid-start outcomes, and the constitution requires regression coverage for important gameplay behavior.

**Organization**: Tasks are grouped by user story. Every story task can be validated by focused source tests plus the relevant quickstart scenario.

## Phase 1: Setup

**Purpose**: Establish the current behavior and implementation boundary before changing it.

- [X] T001 [P] Run and record the existing round/world-generation test baseline for `crates/azimuth-game/src/main.rs`, `crates/azimuth-game/src/battlefield.rs`, and `crates/azimuth-game/src/tank.rs`.
- [X] T002 [P] Review and preserve the session-versus-round ownership contract in `docs/specs/20260912-091903-random-round-starts/data-model.md` while implementing `crates/azimuth-game/src/session.rs` and `crates/azimuth-game/src/main.rs`.

---

## Phase 2: Foundational Deterministic Round Generation

**Purpose**: Provide the pure, reproducible, total round-generation boundary required by every story.

**⚠️ CRITICAL**: Complete this phase before wiring a transition to new-world generation.

- [X] T003 Add failing focused seed-derivation and candidate-world tests, including same-input reproduction and different-round variation, in `crates/azimuth-game/src/main.rs`.
- [X] T004 Add a non-panicking valid-start selection result and tests for unsuitable candidate terrain in `crates/azimuth-game/src/tank.rs`.
- [X] T005 Implement deterministic session-root-plus-round seed derivation and bounded candidate retry in the pure world generator in `crates/azimuth-game/src/main.rs`.
- [X] T006 Integrate the non-panicking valid-start result into the generated-world factory so it rejects unsuitable candidates and exposes only fully valid 2–8 player worlds in `crates/azimuth-game/src/main.rs`.
- [X] T007 Add representative multi-seed, 2–8 player regression coverage for in-bounds, dry/gentle, terrain-supported, separated starts and retry determinism in `crates/azimuth-game/src/tank.rs`.

**Checkpoint**: The factory can deterministically produce or controlledly reject a complete round world without panicking or relying on rendering.

---

## Phase 3: User Story 1 - Begin a Distinct New Round (Priority: P1) 🎯 MVP

**Goal**: A completed round transitions to a fresh battlefield and full-health tanks, while session identity and resources persist.

**Independent Test**: Seed a completed/damaged round, trigger the next-round boundary, and assert a new terrain/tank/turn world with retained player identity, controller, appearance, cash, wins, and ammunition.

### Tests for User Story 1

- [X] T008 [US1] Add a round-transition regression that proves a derived next-round world differs from the completed world while retaining session-owned player values in `crates/azimuth-game/src/main.rs`.
- [X] T009 [US1] Add session tests proving next-round accounting is cleared while cash, wins, configuration, and finite ammunition persist in `crates/azimuth-game/src/session.rs`.

### Implementation for User Story 1

- [X] T010 [US1] Add explicit session-root and one-based round-seed ownership, including fresh-root selection for a newly started local session, in `crates/azimuth-game/src/main.rs`.
- [X] T011 [US1] Route both initial setup and `Shopping → Transition → Playing` through the validated round-world factory, incrementing the round number exactly once on transition, in `crates/azimuth-game/src/main.rs`.
- [X] T012 [US1] Reset round accounting and restore per-player weapon availability from the retained session while resetting the active weapon selection for the new round in `crates/azimuth-game/src/session.rs` and `crates/azimuth-game/src/weapon.rs`.
- [X] T013 [US1] Replace all round-owned terrain, tanks, wind, turn/aim state, AI cursor, flight, impact, and temporary input/presentation resources at the shared round-start boundary in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: The second round is a new playable authoritative game state without losing continuing-session progress.

---

## Phase 4: User Story 2 - Receive Fair Playable Starting Positions (Priority: P1)

**Goal**: Every configured player enters every generated round at a distinct supported location, and no invalid candidate reaches active play.

**Independent Test**: Generate representative derived rounds for each supported player count and verify all tank poses against the terrain and separation rules before initial turn creation.

### Tests for User Story 2

- [X] T014 [US2] Add a generated-round regression over 100 controlled round seeds and every supported player count that verifies full-health, distinct, in-bounds, terrain-supported starts in `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 2

- [X] T015 [US2] Enforce the `round-generation.md` candidate acceptance contract before publishing tanks or a first turn, including a bounded controlled exhaustion path, in `crates/azimuth-game/src/main.rs`.
- [X] T016 [US2] Keep the start selector’s terrain validity, minimum separation, inward orientation, and deterministic ordering explicit and documented beside the selection logic in `crates/azimuth-game/src/tank.rs`.

**Checkpoint**: No supported player count can enter normal combat from an invalid start arrangement.

---

## Phase 5: User Story 3 - Experience Round-to-Round Variety (Priority: P2)

**Goal**: Consecutive normal rounds visibly and authoritatively vary without cosmetics, shopping, or AI perturbing terrain/start generation.

**Independent Test**: Generate five consecutive round numbers from one root and compare terrain or start arrangements; reproduce each selected round from the same root and number.

### Tests for User Story 3

- [X] T017 [US3] Add five-consecutive-round and repeated-root/round reproduction tests, including isolation from cosmetic and AI streams, in `crates/azimuth-game/src/main.rs`.

### Implementation for User Story 3

- [X] T018 [US3] Regenerate the terrain-dependent horizon, terrain mesh, buildings, tank visuals, and HUD from the committed round world, and despawn stale projectile, impact-marker, explosion, particle, and smoke entities in `crates/azimuth-game/src/main.rs`.
- [X] T019 [US3] Update the captured-seed diagnostic and generated-world comments so they identify the session root and selected round/candidate seed in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: Five sequential rounds use distinct reproducible worlds and show no stale visual evidence of the preceding round.

---

## Phase 6: Polish and Cross-Cutting Validation

**Purpose**: Verify the entire feature, document completed roadmap items only when evidence supports it, and preserve a healthy workspace.

- [X] T020 Run `cargo fmt --check`, `cargo test`, and `cargo clippy --workspace --all-targets -- -D warnings` from `Cargo.toml`, fixing only feature-related failures in `crates/azimuth-game/src/`.
- [ ] T021 Perform the 2-player and 8-player multi-round graphical smoke scenarios in `docs/specs/20260912-091903-random-round-starts/quickstart.md` and record any nonessential discoveries in `docs/roadmap.md`.
- [X] T022 Update only validated terrain-generation, fair-start, and continuing-round acceptance items in `docs/roadmap.md`, and align `docs/world-conventions.md` with the final seed/round contract.

---

## Dependencies and Execution Order

```text
Setup (T001–T002)
  → Foundational deterministic generation (T003–T007)
    → US1 fresh authoritative round (T008–T013)  [MVP]
      → US2 valid start enforcement (T014–T016)
        → US3 visible sequential variety (T017–T019)
          → Polish and validation (T020–T022)
```

- **US1** depends on the pure seed/candidate factory because transition must commit a valid new world.
- **US2** relies on the same foundation and independently proves the generator’s start invariants.
- **US3** relies on US1’s lifecycle boundary and applies the presentation refresh and round-sequence proof.

## Parallel Opportunities

- T001 and T002 can run in parallel.
- T003 and T004 can start in parallel after setup because they establish focused tests/contracts in different modules.
- After T005–T006 establish the factory, T007 and T008–T009 can proceed in parallel; they cover different modules and evidence.
- Within US1, T008 and T009 can run in parallel before T010–T013.
- Within US3, T017 can proceed alongside the implementation preparation for T018, but T018 must follow the committed round-start boundary.

## Implementation Strategy

### MVP first

1. Complete T001–T007 to make generation reproducible and total.
2. Complete T008–T013 to deliver a clean fresh next round with preserved session resources.
3. Run the US1 independent test before adding presentation polish.

### Incremental delivery

1. Foundation makes world generation safe and testable.
2. US1 fixes the player-visible repeated-round defect.
3. US2 proves valid starts at all supported player counts.
4. US3 ensures visible terrain-adjacent presentation matches authoritative round variation.
5. Final validation confirms no regression to existing combat behavior.

## Format Validation

All 22 tasks use the required checkbox, sequential ID, optional parallel marker, required user-story label for story-phase work, and exact file path format.
