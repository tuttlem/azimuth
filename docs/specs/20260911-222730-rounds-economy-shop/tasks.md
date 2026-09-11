# Tasks: Rounds, Economy and Weapon Shop — The Continuing Game Loop

**Input**: Design documents in docs/specs/20260911-222730-rounds-economy-shop/

**Tests**: Required by the feature specification. Add focused domain/state tests; avoid pixel tests.

## Phase 1: Setup

- [X] T001 Add the session module declaration and preserve the existing module layout in crates/azimuth-game/src/main.rs and crates/azimuth-game/src/session.rs
- [ ] T002 [P] Review and update the feature validation guide in docs/specs/20260911-222730-rounds-economy-shop/quickstart.md for actual controls and test commands

## Phase 2: Foundational

- [X] T003 Create GameSession, SessionPlayer, SessionPhase, RoundAccounting, economy constants, and pure state-transition helpers with unit tests in crates/azimuth-game/src/session.rs
- [X] T004 Extend the weapon catalogue with centrally defined limited-weapon prices, purchasable enumeration, and validated add-one-ammunition support plus unit tests in crates/azimuth-game/src/weapon.rs
- [ ] T005 Change authoritative explosion resolution to return per-target applied damage and newly-eliminated transitions, retaining existing combat behaviour and tests in crates/azimuth-game/src/combat.rs
- [ ] T006 Carry firing PlayerId through FiredShot construction and all projectile resolution paths in crates/azimuth-game/src/weapon.rs and crates/azimuth-game/src/main.rs
- [ ] T007 Integrate GameSession resource creation from valid setup and isolate per-round, tactical-AI, and shop-AI seed derivations in crates/azimuth-game/src/main.rs
- [ ] T008 Add a central session-phase guard to battle input, tactical AI, fixed projectile/settling completion, and HUD projection in crates/azimuth-game/src/main.rs

**Checkpoint**: authoritative session, applied-damage, price, purchase, ownership, and phase foundations pass unit tests.

## Phase 3: User Story 1 - Continue after a completed round (Priority: P1) MVP

**Goal**: Finish a round, celebrate a winner/draw, continue to accounting, and lock combat controls.

**Independent Test**: Complete a two-player round and reach accounting through a visible continuation action.

- [ ] T009 [P] [US1] Add pure tests for winner/draw finalisation, winner-bonus idempotence, and phase input lock in crates/azimuth-game/src/session.rs
- [ ] T010 [US1] Finalise round accounting from applied damage/owner data at both ordinary and split-projectile impact paths in crates/azimuth-game/src/main.rs
- [ ] T011 [US1] Implement winner/draw celebration overlay, Continue action, and winner-tank camera focus in crates/azimuth-game/src/main.rs
- [ ] T012 [US1] Implement accounting overlay projecting every player's damage, eliminations, win bonus, total, and available cash in crates/azimuth-game/src/main.rs
- [ ] T013 [US1] Add regression tests proving self-damage, overkill, splash, multi-projectile, elimination, winner, and draw accounting correctness in crates/azimuth-game/src/session.rs and crates/azimuth-game/src/combat.rs

**Checkpoint**: winner/draw celebration and explained authoritative accounting work without battlefield input leakage.

## Phase 4: User Story 2 - Earn understandable combat cash (Priority: P1)

**Goal**: Ensure rewards are correct and remain understandable for all configured players.

**Independent Test**: Use controlled damage/elimination cases and compare each accounting row to applied health.

- [ ] T014 [US2] Add per-player cash/win projection to the tactical scoreboard and Round N display in crates/azimuth-game/src/main.rs
- [ ] T015 [US2] Add 2-, 4-, and 8-player accounting identity/order regression tests in crates/azimuth-game/src/session.rs and crates/azimuth-game/src/main.rs

**Checkpoint**: every participant can explain their balance, including eliminated players.

## Phase 5: User Story 3 - Buy ammunition between rounds (Priority: P1)

**Goal**: Sequential human weapon shop and bounded AI shopping start the next round.

**Independent Test**: Buy one affordable round, reject an unaffordable purchase, ready humans, and begin the next round with AI.

- [ ] T016 [P] [US3] Add purchase success/failure, Basic Shell exclusion, one-round quantity, and deterministic bounded AI-shopping tests in crates/azimuth-game/src/session.rs and crates/azimuth-game/src/weapon.rs
- [ ] T017 [US3] Implement deterministic AI shop policy and configured-order human ShopProgress transitions in crates/azimuth-game/src/session.rs
- [ ] T018 [US3] Implement full-screen sequential shop overlay with weapon name/price/owned count, buy buttons, READY/DONE, and immediate feedback in crates/azimuth-game/src/main.rs
- [ ] T019 [US3] Route shop button requests through validated GameSession purchase/ready operations and advance automatically past AI shoppers in crates/azimuth-game/src/main.rs

**Checkpoint**: local humans and AI finish shopping safely; money/inventory cannot be mutated by UI directly.

## Phase 6: User Story 4 - Preserve session, refresh round (Priority: P2)

**Goal**: Fresh combat world with persistent session values.

**Independent Test**: Consume/purchase ammunition, start Round 2, and verify fresh world plus preserved session data.

- [ ] T020 [P] [US4] Add session transition tests for 2–8 players, stable metadata, preserved cash/loadouts, and reset combat state in crates/azimuth-game/src/session.rs
- [ ] T021 [US4] Extract the existing setup world-generation/despawn/spawn sequence into a reusable fresh-round operation in crates/azimuth-game/src/main.rs
- [ ] T022 [US4] Reset all projectile, impact, particle/smoke, camera, movement, aim-repeat, tank/building, HUD, terrain, and turn state while restoring session loadouts in crates/azimuth-game/src/main.rs
- [ ] T023 [US4] Derive fresh deterministic terrain/start/dressing/wind/tactical-AI seeds per round without shop/cosmetic perturbation in crates/azimuth-game/src/main.rs
- [ ] T024 [US4] Add regression coverage for prior-round barrage/Nuke effects and eliminated tanks being absent after transition in crates/azimuth-game/src/main.rs

**Checkpoint**: Round 2 is genuinely fresh while player/session identity and purchases persist.

## Phase 7: User Story 5 - End a session (Priority: P3)

**Goal**: End between-round play and return to setup.

**Independent Test**: END GAME, configure/start again, and verify standard inventory/no former balance.

- [ ] T025 [P] [US5] Add session-discard/new-session default-state tests in crates/azimuth-game/src/session.rs
- [ ] T026 [US5] Add END GAME action to accounting/shop and return to normal setup while despawning session/round overlays and visuals in crates/azimuth-game/src/main.rs

## Phase 8: Polish and validation

- [ ] T027 Verify no normal battle control can act during every non-Playing phase and simplify duplicated guards in crates/azimuth-game/src/main.rs
- [ ] T028 Run cargo fmt, cargo test --workspace, cargo clippy --workspace --all-targets --all-features -- -D warnings, and graphical quickstart scenarios; record any headless limitation in docs/specs/20260911-222730-rounds-economy-shop/quickstart.md
- [ ] T029 Update only roadmap items genuinely satisfied by the continuing loop in docs/roadmap.md

## Dependencies & Execution Order

T001–T008 block all stories. US1 requires the foundation; US2 builds on US1 accounting presentation; US3 requires prices/session; US4 requires US3 completion; US5 can follow the foundation but is best integrated after US4. Polish follows all selected stories.

## Parallel Opportunities

T002 may run with T001. T003–T005 are parallel after module setup. T009 and T013 can proceed around overlay work; T016 and T020/T025 are focused domain-test work parallel with UI implementation where their prerequisites are complete.

## Implementation Strategy

Deliver T001–T013 as the MVP: finish, celebrate, account, and continue. Then add shop, fresh round reset, and END GAME incrementally. Validate each checkpoint before continuing.
