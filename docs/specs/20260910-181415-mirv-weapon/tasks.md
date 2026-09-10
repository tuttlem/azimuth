# Tasks: MIRV — First Multi-Projectile Weapon

**Input**: Design documents in `docs/specs/20260910-181415-mirv-weapon/`

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [contract](contracts/mirv-shot-presentation.md)

**Tests**: Required by FR-019; write focused deterministic tests before their implementation tasks.

## Phase 1: Setup

- [X] T001 Review the committed-shot, deterministic ordering, and observer-boundary decisions in `docs/specs/20260910-181415-mirv-weapon/{plan,research,data-model}.md`
- [X] T002 [P] Record the MIRV authority/presentation and player-control contract as implementation comments/tests in `docs/specs/20260910-181415-mirv-weapon/contracts/mirv-shot-presentation.md`

## Phase 2: Foundational — Multi-Projectile Shot Authority

**Purpose**: Replace the single-active-projectile assumption without changing ordinary-shot behavior.

- [ ] T003 Add failing deterministic tests for one shot owning ordered active records, normal one-record conventional shots, and copied immutable per-record impact profiles in `crates/azimuth-game/src/weapon.rs`
- [X] T004 Add MIRV weapon identity, two-round availability, behavior/profile data, child impact profile, and loadout/commit support in `crates/azimuth-game/src/weapon.rs`
- [X] T005 Add minimal active-projectile role/ordered-shot ownership and fixed child-separation helpers with deterministic forward/right fallbacks in `crates/azimuth-game/src/weapon.rs`
- [ ] T006 Add failing fixed-step tests for apex-crossing detection and carrier/child ordinary projectile state preservation in `crates/azimuth-game/src/projectile.rs`
- [ ] T007 Expose only the small projectile observation/helper needed to identify the post-step apex and create children without changing gravity, wind, or collision integration in `crates/azimuth-game/src/projectile.rs`
- [ ] T008 Add failing game-state tests for stable ordered multi-projectile advancement, individual removal, and completion gated by empty shot plus settled tanks in `crates/azimuth-game/src/main.rs`
- [X] T009 Replace singular fixed-step shot advancement/resolution with stable ordered active-record processing, normal per-record impact work, and empty-shot-plus-settling handoff gating in `crates/azimuth-game/src/main.rs`

**Checkpoint**: Conventional shots remain one-record shots and no turn can finish while an active record remains.

## Phase 3: User Story 1 — Fire a Readable MIRV Barrage (P1) 🎯 MVP

**Goal**: Select and fire one normal carrier that visibly and deterministically becomes five wind-affected children at apex.

**Independent Test**: Fire a high MIRV arc and verify one pre-apex carrier, exactly five post-apex children, reproducible state/trajectories, and no additional inventory consumption.

- [ ] T010 [P] [US1] Add failing MIRV definition/loadout tests for two rounds, one-round commit, final-round fallback, and child profile weaker than HE in `crates/azimuth-game/src/weapon.rs`
- [ ] T011 [P] [US1] Add failing deterministic carrier tests for normal gravity/wind before split, exact five-child creation, inherited state plus separation, and stable repeated trajectories in `crates/azimuth-game/src/main.rs`
- [X] T012 [US1] Implement MIRV carrier launch and first-surviving-post-apex replacement with five fixed-index children in `crates/azimuth-game/src/main.rs`
- [X] T013 [US1] Add Digit-4 MIRV selection and existing-HUD name/ammunition/control-hint integration in `crates/azimuth-game/src/main.rs`
- [X] T014 [US1] Map carrier and each active child to distinct simple projectile visuals, removing only resolved visuals, in `crates/azimuth-game/src/main.rs`

**Checkpoint**: A Human can fire MIRV and see carrier → five deterministic children while Basic/HE/Heavy selection remains intact.

## Phase 4: User Story 2 — Resolve the Whole Barrage (P1)

**Goal**: Every child independently uses normal collision, impact, terrain, damage, and settling while the turn waits for all of them.

**Independent Test**: Use a fixture with staggered child impacts and an earlier crater affecting a later collision; verify no early handoff and normal final handoff/result.

- [ ] T015 [P] [US2] Add failing tests for independently timed child impacts, sibling survival, child wind response, and out-of-bounds removal without an invented impact in `crates/azimuth-game/src/main.rs`
- [ ] T016 [P] [US2] Add failing tests for ordered same-step crater mutation, overlapping normal damage/elimination, settling lock, and final-child-only completion in `crates/azimuth-game/src/main.rs`
- [X] T017 [US2] Complete ordered child resolution against current mutable terrain, retaining normal explosion/crater/damage/support behavior per record in `crates/azimuth-game/src/main.rs`
- [X] T018 [US2] Guard settling and `complete_fire_resolution` so individual impacts or bounds outcomes never advance a MIRV turn in `crates/azimuth-game/src/main.rs`

**Checkpoint**: A complete barrage is authoritative; late children can damage targets and hit terrain changed by earlier children.

## Phase 5: User Story 3 — Read a Comfortable Barrage (P1)

**Goal**: Carrier-first, aggregate-spread camera and bounded repeated-impact feedback make Human and AI MIRVs legible without presentation authority.

**Independent Test**: Run Human and AI MIRVs with rapid impacts; verify camera widens/aggregates, every impact receives bounded presentation, and changed presentation cannot change results.

- [ ] T019 [P] [US3] Add failing presentation tests for carrier versus child-envelope camera selection, aggregate impact region, Human/AI mode retention, and presentation-independent authority in `crates/azimuth-game/src/main.rs`
- [ ] T020 [P] [US3] Add failing event-consumption tests for ordered multiple impact visuals, pulse cooldown/coalescing, and bounded rapid child impact audio in `crates/azimuth-game/src/main.rs`
- [ ] T021 [US3] Add read-only active-shot snapshots and aggregate impact-region state; extend Human follow and AI tactical camera intents without arbitrary child tracking in `crates/azimuth-game/src/main.rs`
- [ ] T022 [US3] Replace single-latest-impact presentation consumption with ordered resolved-impact events, per-event visuals, coalesced pulse, and bounded child audio in `crates/azimuth-game/src/main.rs`

**Checkpoint**: The split and threatened region are readable; rapid impacts neither snap/strobe nor affect deterministic resolution.

## Phase 6: User Story 4 — Preserve Mixed-Weapon and AI Play (P1)

**Goal**: MIRV inventory is safe alongside conventional weapons and AI turns continue using their supported Basic Shell behavior.

**Independent Test**: Exercise exhausted MIRV, all conventional weapons, and Human/AI turns; verify conventional lifecycle and Basic-Shell AI decision remain unchanged.

- [X] T023 [P] [US4] Extend weapon-loadout regression coverage for MIRV exhaustion, selection rejection, per-player independence, and conventional weapon preservation in `crates/azimuth-game/src/weapon.rs`
- [ ] T024 [P] [US4] Add AI regression coverage that MIRV inventory does not change the deterministic Basic-Shell decision or shared launch path in `crates/azimuth-game/src/ai.rs`
- [ ] T025 [US4] Run and repair affected HUD, shared-fire, conventional impact, and turn-resolution fixtures in `crates/azimuth-game/src/main.rs`

**Checkpoint**: MIRV is a limited choice; existing weapons and autonomous AI turns retain their behavior.

## Phase 7: Polish and Cross-Cutting Validation

- [X] T026 Update controls, one-shot-to-many-projectile lifecycle, deterministic ordering, and MIRV inventory documentation in `docs/projectile-model.md`
- [ ] T027 Update only delivered MIRV/independent-child/wind roadmap checkboxes and leave configurable split timing and future weapons open in `docs/roadmap.md`
- [ ] T028 Run `cargo test -p azimuth-game`, `cargo fmt --all -- --check`, and `cargo clippy --workspace --all-targets -- -D warnings` from `README.md`
- [ ] T029 Perform and record the Human/AI, wind, terrain, tactical-identity, and presentation-safety scenarios in `docs/specs/20260910-181415-mirv-weapon/quickstart.md`

## Dependencies & Execution Order

- Setup → Foundational → US1 → US2 → US3 → US4 → Polish.
- US1 depends on the shared shot model. US2 depends on US1's carrier/child lifecycle. US3 depends on an active-shot snapshot and resolved impacts from US1/US2. US4 can begin inventory/AI test work after Foundational but its full regression checkpoint follows US1–US3.

## Parallel Opportunities

- T003 and T006 can proceed in parallel because they test separate modules.
- Within US1, T010 and T011 can proceed in parallel; within US2, T015 and T016 can proceed in parallel; within US3, T019 and T020 can proceed in parallel; within US4, T023 and T024 can proceed in parallel.

## Implementation Strategy

Deliver the MVP through Phase 3: committed inventory, one carrier, deterministic five-child split, and child visuals. Validate it before adding authoritative multi-impact resolution (Phase 4), then add presentation (Phase 5), mixed-play regressions (Phase 6), and documented manual/quality completion (Phase 7).
