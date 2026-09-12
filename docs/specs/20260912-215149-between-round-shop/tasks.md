# Tasks: Between-Round Shop — Weapon Purchasing and Visual Inventory

**Input**: Design documents from `/docs/specs/20260912-215149-between-round-shop/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, [shop UI contract](./contracts/shop-ui.md), quickstart.md

**Tests**: Required by the specification for catalogue, transactions, lifecycle, player isolation, AI, and presentation identity.

**Organization**: Tasks are grouped by user story; shared session and catalogue work is completed first.

## Phase 1: Setup

**Purpose**: Verify exact integration points before changing the live game loop.

- [X] T001 Review session transition, weapon catalogue, HUD card, mouse interaction, AI, and round-reset call sites in `crates/azimuth-game/src/{session.rs,weapon.rs,main.rs,ai.rs}`.
- [X] T002 Review current economy, UI, complete-match, mouse-control, and scalability entries in `docs/roadmap.md` before updating them.

---

## Phase 2: Foundational

**Purpose**: Build the shared authoritative shop and presentation vocabulary that all stories require.

- [X] T003 Add presentation-neutral weapon display identity and one authoritative active limited-weapon catalogue in `crates/azimuth-game/src/weapon.rs`.
- [X] T004 Add `ShopItem`, `ShopCategory`, configured-order shop-turn state, and authoritative ready/advance operations in `crates/azimuth-game/src/session.rs`.
- [X] T005 Add focused catalogue, Basic Shell exclusion, Bomb Net absence, item identity, and shared presentation metadata regression tests in `crates/azimuth-game/src/{weapon.rs,session.rs}`.

**Checkpoint**: Session owns legal shop progression and the active arsenal has one shared product/presentation source.

---

## Phase 3: User Story 1 - Spend round winnings before the next battle (Priority: P1) 🎯 MVP

**Goal**: Humans sequentially purchase ammunition after accounting, then only the final shopper enables the existing fresh-round transition.

**Independent Test**: Finish a two-human round, buy a round for each participant, choose Done twice, and observe a fresh Round 2 with both retained inventories.

- [X] T006 [P] [US1] Add atomic affordable, unaffordable, repeated, and player-isolated ammunition-purchase tests in `crates/azimuth-game/src/session.rs`.
- [X] T007 [P] [US1] Add configured-order human Shop turn, one-time Done, draw, zero-cash, and post-final-Done transition tests in `crates/azimuth-game/src/session.rs`.
- [X] T008 [US1] Restrict `GameSession::purchase` to the active Shop participant and wire successful purchases through its existing atomic cash/loadout boundary in `crates/azimuth-game/src/session.rs`.
- [X] T009 [US1] Replace placeholder Shopping transition behavior with session-owned Human turn advancement and final-ready signaling in `crates/azimuth-game/src/main.rs`.
- [X] T010 [US1] Create the active-shop overlay, shopper wallet, product-card grid, Buy controls, owned counts, and Done control in `crates/azimuth-game/src/main.rs`.
- [X] T011 [US1] Dispatch Buy and Done interactions only to the active Human shopper, refresh cash/count/affordability immediately, and block battlefield controls during Shopping in `crates/azimuth-game/src/main.rs`.
- [X] T012 [US1] Connect final shop completion to the existing `Transition` fresh-round path and verify session inventories are copied without resetting unused ammunition in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: A two-human player can complete a full accounting → shop → fresh-round loop without a keyboard shortcut bypassing the shop.

---

## Phase 4: User Story 2 - Browse a clear, honest weapon catalogue (Priority: P1)

**Goal**: The shop shows every current limited purchasable weapon as a visual, readable, affordable-or-dimmed card.

**Independent Test**: With cash for HE but not Nuke, show all 10 limited weapons exactly once, no Bomb Net, no purchasable Basic Shell, and only legal Buy actions.

- [X] T013 [P] [US2] Add product-card view-model tests for all limited weapons, positive prices, owned counts, and unavailable-but-visible affordability state in `crates/azimuth-game/src/{weapon.rs,main.rs}`.
- [X] T014 [US2] Add project-owned visual recipes for each active weapon and render those shared visuals on shop cards in `crates/azimuth-game/src/main.rs`.
- [X] T015 [US2] Make the Shop card grid wrap or otherwise fit the current 10-product catalogue with readable name, price, count, and Buy states in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: The P1 shop is a visually browsable catalogue rather than a text-only list and all transactions remain session-authoritative.

---

## Phase 5: User Story 3 - Let AI and all player counts progress through shopping (Priority: P2)

**Goal**: Every configured participant completes the shop loop; AI purchases are bounded and cannot stall local play.

**Independent Test**: In a four-player Human/AI mix with a zero-cash eliminated player, each Human receives one turn, AI completes automatically, and Round 2 begins with isolated valid inventories.

- [X] T016 [P] [US3] Add deterministic bounded AI shop-purchase and no-overspend tests in `crates/azimuth-game/src/ai.rs`.
- [X] T017 [P] [US3] Add 2–8 player shop-order, Human/AI, zero-cash, and inventory-isolation lifecycle tests in `crates/azimuth-game/src/{session.rs,main.rs}`.
- [X] T018 [US3] Implement bounded deterministic or seeded affordable AI shopping using the authoritative item catalogue and session purchase boundary in `crates/azimuth-game/src/ai.rs`.
- [X] T019 [US3] Integrate automatic AI completion with sequential shop progression and final transition signaling in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: Mixed-controller local matches cannot be blocked in Shopping and retain each participant's independent resources.

---

## Phase 6: User Story 4 - Scan the battlefield inventory visually (Priority: P2)

**Goal**: The existing clickable bottom strip becomes a compact visual inventory while retaining labels, ammunition, selected/unavailable state, and mouse behavior.

**Independent Test**: Start a round with varied ammunition, click available cards, and verify each card's shared icon, compact name, ammo state, selection, and exhaustion behavior.

- [X] T020 [P] [US4] Add weapon-strip shared-identity, unlimited, exhausted, selected-state, and mouse-selection regression tests in `crates/azimuth-game/src/main.rs`.
- [X] T021 [US4] Render the shared weapon visual recipe alongside compact labels and ammunition in existing `WeaponSlot` cards in `crates/azimuth-game/src/main.rs`.
- [X] T022 [US4] Resize/reflow the existing bottom strip for all 11 current weapons while preserving comfortable button hit targets and battlefield visibility in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: Shop and battle use the same visual weapon identity, while the battle strip remains readable and clickable.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Complete documentation, regression coverage, and end-to-end quality validation.

- [X] T023 Update economy, between-round Shop, visual inventory, mouse/UI, complete-match, and future Armour roadmap entries in `docs/roadmap.md`.
- [X] T024 [P] Update active shop, purchasing, and visual weapon-inventory guidance in `README.md`.
- [X] T025 Run formatting, Clippy, tests, check, build, and the four-player manual acceptance in `docs/specs/20260912-215149-between-round-shop/quickstart.md`.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (T001–T002)**: Start immediately.
- **Foundational (T003–T005)**: Depends on setup; blocks all stories.
- **US1 (T006–T012)**: Depends on foundational work; MVP.
- **US2 (T013–T015)**: Depends on shared presentation catalogue and the US1 shop-card surface.
- **US3 (T016–T019)**: Depends on US1 session shop progression; can proceed alongside US2 once US1 is complete.
- **US4 (T020–T022)**: Depends on shared presentation catalogue; can proceed alongside US1/US3 after T003–T005, subject to `main.rs` coordination.
- **Polish (T023–T025)**: Depends on all desired user stories.

### User Story Dependencies

```text
Setup → Foundational → US1 ─┬→ US2 → Polish
                             ├→ US3 → Polish
                             └→ US4 → Polish
```

### Parallel Opportunities

- T006 and T007 can be written in parallel because they exercise distinct session behaviors.
- T013 can run alongside the shop card implementation once T003 exposes presentation identity.
- T016 and T017 can run in parallel because AI decision coverage and multi-player lifecycle coverage have separate primary files.
- T020 can run in parallel with AI work after the shared catalogue exists; coordinate `main.rs` edits sequentially with other UI work.
- T023 and T024 can run in parallel after behavior is finalized.

## Implementation Strategy

### MVP First

1. Complete setup and foundational catalogue/shop-state work.
2. Implement and test US1: sequential Human shopping and retained session inventory.
3. Validate the two-human full lifecycle before adding AI or visual refinements.

### Incremental Delivery

1. Add US2 to turn the working shop into a complete visual catalogue.
2. Add US3 so all 2–8 player controller mixes continue automatically.
3. Add US4 so the same visual vocabulary improves in-battle decisions.
4. Finish docs and the four-player manual acceptance only after all automated quality gates pass.

## Notes

- Every task follows the required checkbox, identifier, story-label, and file-path format.
- Tests are deliberately scheduled before their corresponding behavior because the specification explicitly requires strong automated coverage.
- Keep `main.rs` UI and lifecycle edits sequential where they overlap despite otherwise parallel story opportunities.
