# Feature Specification: Between-Round Shop — Weapon Purchasing and Visual Inventory

**Feature Branch**: `20260912-215149-between-round-shop`

**Created**: 2026-09-12

**Status**: Draft

**Input**: Add a session-level between-round shop where every player can buy weapon ammunition, and give the shared battlefield/shop weapon inventory a readable visual identity.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Spend round winnings before the next battle (Priority: P1)

After a round's result and accounting have been shown, every local human player takes a sequential shop turn. They see their own cash, current ammunition, prices, and a card-based weapon catalogue; each purchase immediately changes only that player's wallet and inventory. They choose Done when ready, and the next battlefield begins only after every player has completed shopping.

**Why this priority**: The current economy is meaningful only if players can convert earnings into choices that affect later rounds.

**Independent Test**: Complete a two-human round, buy one affordable weapon round for each player, choose Done for both, and verify the next round starts with each player's purchased and unused ammunition intact.

**Acceptance Scenarios**:

1. **Given** a round has completed and accounting is acknowledged, **When** the shop opens, **Then** it appears before a new battlefield is created and identifies the active human shopper and their current cash.
2. **Given** an active human shopper has enough cash for a limited weapon, **When** they activate Buy, **Then** exactly one round is added, exactly that weapon's price is deducted, and the card updates immediately.
3. **Given** the active shopper presses Done, **When** another configured human has not shopped, **Then** that human receives the next shop turn with their own wallet and inventory.
4. **Given** every configured player has completed shopping, **When** the final turn finishes, **Then** the shop closes and the existing fresh-round flow starts once with retained session resources.
5. **Given** the shop is active, **When** a player uses normal battlefield input, **Then** it cannot fire, move, or otherwise change battlefield gameplay.

---

### User Story 2 - Browse a clear, honest weapon catalogue (Priority: P1)

A shopper can browse every currently purchasable limited weapon as visually distinct cards. Each card retains a readable name and shows a representative weapon image, price, owned ammunition, and purchase affordance. Unaffordable weapons remain visible but clearly unavailable; Basic Shell remains unlimited and cannot consume money.

**Why this priority**: The shop should create tactical choice rather than hide the arsenal or force players to decode a text spreadsheet.

**Independent Test**: Open the shop with less cash than a Nuke and enough for High Explosive; verify both cards are visible, only the affordable Buy control works, and Basic Shell cannot be purchased.

**Acceptance Scenarios**:

1. **Given** the current active arsenal, **When** the catalogue is shown, **Then** every limited purchasable weapon appears exactly once, Bomb Net is absent, and Basic Shell is not purchasable.
2. **Given** a player lacks a weapon's price, **When** they view its card, **Then** its name, representative image, price, and owned ammunition remain visible while Buy is unmistakably unavailable.
3. **Given** a weapon is bought repeatedly while affordable, **When** each purchase succeeds, **Then** one ammunition unit and one price deduction occur per activation without a negative wallet.
4. **Given** any current selectable weapon, **When** it is shown in the shop and battlefield strip, **Then** both contexts use the same identity, image, and readable label.

---

### User Story 3 - Let AI and all player counts progress through shopping (Priority: P2)

Configured AI players complete a small, deterministic or seeded set of affordable ammunition purchases automatically, while humans retain their sequential turns. Players with no cash still see a valid shop turn and can choose Done. No configured participant is skipped for elimination, placement, controller type, or wallet balance.

**Why this priority**: A 2–8 player continuing match must reliably reach the next round without the shop becoming a multiplayer dead end.

**Independent Test**: Complete a four-player round with humans and AI, including a zero-cash eliminated player; verify both humans receive a turn, AI finishes automatically, and the next round begins with valid independent inventories.

**Acceptance Scenarios**:

1. **Given** a mix of human and AI players, **When** the shop reaches an AI player, **Then** its bounded purchasing completes automatically and never requires a human input to continue.
2. **Given** a human player has no affordable weapon, **When** their shop turn opens, **Then** the catalogue, wallet, disabled purchase controls, and Done action are still available.
3. **Given** an AI's available cash and inventory, **When** its shop turn resolves, **Then** it cannot overspend or obtain invalid ammunition.
4. **Given** two through eight configured players, **When** shopping completes, **Then** each player's cash and inventory remain isolated from every other player's.

---

### User Story 4 - Scan the battlefield inventory visually (Priority: P2)

During a battle, the player can recognise weapon cards in the existing clickable bottom strip from small, coherent Azimuth-specific images before reading every label. Compact names and ammunition information remain visible, while selected and exhausted states and comfortable mouse selection remain clear.

**Why this priority**: The existing growing arsenal is becoming visually dense; shared visual identity makes choice faster without replacing readable text.

**Independent Test**: Start a round with several distinct weapons, inspect and click the strip, and verify each card has a distinct image, readable compact name, correct remaining/unlimited ammunition, and unchanged selection behavior.

**Acceptance Scenarios**:

1. **Given** a battle is in a human choosing turn, **When** the weapon strip is visible, **Then** every current selectable weapon has a distinct representative image plus its short label and ammunition state.
2. **Given** a player selects a card by mouse, **When** the weapon is available, **Then** the selected state is obvious and the same weapon is selected as before the visual upgrade.
3. **Given** a limited weapon has no remaining ammunition, **When** it appears in the strip, **Then** it is visibly unavailable while Basic Shell visibly remains unlimited.

### Edge Cases

- A failed purchase due to insufficient cash, unlimited ammunition, an invalid product, or a completed shop turn changes neither wallet nor inventory.
- A player may retain unused starting or bought ammunition over any number of rounds; only firing consumes limited ammunition.
- A player eliminated early, placed last, or with zero cash still receives one human shop turn if configured as Human.
- Simultaneous UI input must not turn one purchase activation into multiple ammunition units or permit a player to buy after Done.
- A product without valid visual presentation metadata must fail visibly during development rather than silently producing an unusable card.
- The current eleven-weapon arsenal must fit the supported game window without overlapping controls or obscuring essential shop actions; any overflow uses a simple browse mechanism.
- A draw still enters the normal accounting-to-shop flow; shopping must not depend on a winner.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The game MUST use a distinct session-level Shop phase after result presentation and accounting and before fresh-round creation.
- **FR-002**: The next battlefield MUST NOT be created until all configured players have completed their between-round shop processing.
- **FR-003**: Normal battlefield movement, firing, aiming, and weapon-selection controls MUST be inactive while the Shop phase is active.
- **FR-004**: Each configured Human player MUST receive one sequential shop turn per completed round, in stable configured-player order, regardless of round outcome, cash, or retained inventory.
- **FR-005**: Each Human shop turn MUST show the shopper identity, current cash, current owned ammunition, an obvious Done action, and the active weapon catalogue.
- **FR-006**: Each configured AI player MUST complete a bounded valid purchasing pass automatically and MUST NOT block progression to the next shopper or round.
- **FR-007**: The active catalogue MUST derive from authoritative current weapon purchasing data and include every limited purchasable weapon exactly once; it MUST exclude Bomb Net and prohibit Basic Shell purchases.
- **FR-008**: Every active weapon product MUST show a representative Azimuth-specific visual, readable name, positive price, current owned ammunition, and Buy state.
- **FR-009**: A Buy activation MUST be atomic: it adds exactly one round of the selected limited weapon and deducts exactly its price, or changes neither wallet nor inventory.
- **FR-010**: Unaffordable products MUST remain visible with price and inventory information while their Buy affordance is clearly unavailable.
- **FR-011**: Shop purchases and unused ammunition MUST remain in the owning player's session inventory across fresh-round creation; used limited ammunition MUST remain consumed.
- **FR-012**: The shop's authoritative product concept MUST identify category, presentation, price, and purchase effect so a later Armour category can be added without replacing the shop model; only the Weapons category is active in this feature.
- **FR-013**: The battlefield weapon strip and shop MUST resolve the same weapon presentation identity; presentation MUST supplement rather than replace readable weapon names.
- **FR-014**: The battlefield strip MUST retain usable mouse hit targets and make selected, unavailable, remaining, and unlimited ammunition states visually distinguishable.
- **FR-015**: The shop catalogue and battlefield strip MUST lay out the current active arsenal without overlapping controls, unreadably small text, or off-screen required actions at the supported game window.
- **FR-016**: The Shop phase MUST consume current session wallet balances without recalculating or changing the existing damage-income and placement-award rules.
- **FR-017**: The game MUST update the roadmap with completed economy/shop behavior and record Armour only as a future shop category, not an implemented system.

### Key Entities

- **Shop Item**: A purchasable session preparation product with stable identity, category, presentation, price, and a defined purchase outcome.
- **Shop Category**: A small grouping of products; Weapons is active now and Armour is reserved for a future feature.
- **Shop Turn**: One configured player's bounded opportunity to review their wallet and make purchases before the next round.
- **Weapon Presentation Identity**: The shared visual and textual identity used by a weapon product and its battlefield inventory card.
- **Session Inventory**: The player-owned, continuing set of unlimited and limited weapon availability that survives fresh battlefield creation.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In a two-to-eight-player completed round, 100% of configured Human players receive exactly one shop turn before the next round starts, and no AI player requires manual input to complete shopping.
- **SC-002**: For every affordable purchase tested, cash decreases by the displayed price and the matching ammunition count increases by exactly one; for every rejected purchase, both values remain unchanged.
- **SC-003**: The catalogue displays all 10 currently limited purchasable weapons, displays no Bomb Net product, and provides no purchasable Basic Shell product.
- **SC-004**: A four-player mixed Human/AI session can complete accounting, shopping, and fresh-round start without developer tooling or a stalled player turn.
- **SC-005**: In Round 2, 100% of ammunition bought in the preceding shop and unused ammunition retained from Round 1 are visible in the owning player's inventory, while ammunition fired in Round 1 remains consumed.
- **SC-006**: Every current battlefield-selectable weapon appears with one shared representative visual, compact readable name, and ammunition state in both the battlefield strip and shop catalogue.
- **SC-007**: At the supported game window, all required Shop actions and the current weapon strip remain visible, clickable, and legible without overlap.

## Assumptions

- The existing session wallet, price table, loadout ownership, accounting, deterministic fresh-round generation, and mouse button system are authoritative foundations for this feature.
- The current limited purchasable arsenal is High Explosive, Heavy Shell, MIRV, Cluster Bomb, Roller, Bunker Buster, Dirt Bomb, Curve Ball, Bouncer, and Nuke; Basic Shell remains unlimited.
- AI shopping may use a simple bounded deterministic or seeded choice and does not need economic strategy or visible per-click animation.
- Visual identities will be a cohesive, small, Azimuth-specific set with clear provenance created within the project; external stock icon collections are not needed.
- The first shop has only Weapons as an active category. Armour behavior, pricing, repair, mitigation, tabs, persistence, resale, and other shop categories remain out of scope.
- The existing Round 1 starting inventory remains unchanged; no pre-match or starting shop is introduced.
