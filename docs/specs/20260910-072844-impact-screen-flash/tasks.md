---

description: "Actionable tasks for one bounded impact screen flash"
---

# Tasks: Impact Screen Flash

**Input**: Design documents in `docs/specs/20260910-072844-impact-screen-flash/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/impact-screen-flash.md, quickstart.md

**Tests**: Required. Use existing module-local tests for pure pulse profile, once-only consumption, and non-authority; do not use pixel tests.

## Phase 1: Setup

- [x] T001 Inspect the current visual explosion, impact consumption, HUD overlay, and update ordering in crates/azimuth-game/src/main.rs
- [x] T002 Run cargo test -p azimuth-game from the workspace root and retain the passing baseline

## Phase 2: Foundational Pulse State

- [x] T003 Add small presentation-only impact-flash state, central bounded tuning constants, and pure profile/fade helpers in crates/azimuth-game/src/main.rs
- [ ] T004 Add unit tests for restrained baseline/high-scale profile variation, rapid fade bounds, and non-authority in crates/azimuth-game/src/main.rs
- [x] T005 Add a single full-screen white UI overlay that is hidden whenever no pulse is active in crates/azimuth-game/src/main.rs

**Checkpoint**: One overlay can safely represent a bounded pulse without reading or mutating simulation state.

## Phase 3: User Story 1 - Feel an Explosion's Force (Priority: P1) 🎯 MVP

**Goal**: Each resolved terrain explosion produces one short, scale-aware pulse.

**Independent Test**: Basic, Heavy, and HE terrain impacts produce one bounded pulse; out-of-bounds produces none.

- [ ] T006 [US1] Add failing tests for once-per-resolved-impact selection, persistent-impact non-repetition, out-of-bounds silence, and High Explosive scale variation in crates/azimuth-game/src/main.rs
- [x] T007 [US1] Start/reset flash state from the existing once-only resolved terrain-impact presentation boundary, using only existing impact visual scale in crates/azimuth-game/src/main.rs
- [x] T008 [US1] Update overlay opacity every rendered frame and hide it after the bounded rapid fade in crates/azimuth-game/src/main.rs

**Checkpoint**: The pulse is visible, one-per-impact, short, profile-led, and independently usable.

## Phase 4: User Story 2 - Keep the Battle Comfortable and Authoritative (Priority: P1)

**Goal**: Consecutive and multiplayer impacts remain comfortable, without gameplay coupling.

**Independent Test**: Consecutive AI and multi-tank impact fixtures create no extra pulses and retain identical terrain, tank, turn, and result state.

- [ ] T009 [US2] Add tests proving multi-tank damage/elimination cannot create additional flash starts and pulse evaluation leaves authoritative fixtures unchanged in crates/azimuth-game/src/main.rs
- [x] T010 [US2] Audit flash/impact ordering so camera, audio, terrain, settling, and turn systems never await or inspect flash completion in crates/azimuth-game/src/main.rs

## Phase 5: Polish and Validation

- [ ] T011 Update the impact presentation description in README.md after manual review
- [x] T012 Run the automated checks and manual scenarios in docs/specs/20260910-072844-impact-screen-flash/quickstart.md
- [x] T013 Update only the demonstrated exaggerated-presentation roadmap item in docs/roadmap.md after T012 evidence
- [ ] T014 Run cargo fmt --all -- --check, cargo test --workspace, cargo check --workspace --all-targets, and cargo clippy --workspace --all-targets --all-features -- -D warnings from the workspace root

## Dependencies & Execution Order

- T001–T002 precede the shared pulse state.
- T003–T005 block both stories.
- US1 (T006–T008) delivers the MVP.
- US2 (T009–T010) validates the completed pulse under repeated impacts.
- Final validation follows both stories.

## Parallel Opportunities

- T011 documentation may run in parallel with T012 manual review after T010.
- Automated quality commands in T014 may run while manual review begins.

## Implementation Strategy

Implement and validate US1 first, then add the non-authority regression coverage of US2. Do not mark the roadmap item until manual review confirms the pulse remains restrained.

## Format Validation

All 14 tasks use the required checkbox, sequential ID, story label, and exact path format.
