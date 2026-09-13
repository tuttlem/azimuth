# Feature Specification: Round, Controls and Battlefield Polish

**Feature Branch**: `20260913-144118-round-controls-polish`  
**Created**: 2026-09-13  
**Status**: Draft  
**Input**: Improve round accounting, shop balance, movement, setup controls, wind visibility, and battlefield-edge presentation.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Review a complete multiplayer round (Priority: P1)

After a round, local players can see every participant's earnings and resulting balance in an attractive accounting view, including an eight-player match, without losing rows off-screen or needing to scroll.

**Why this priority**: Accounting is a required bridge to shopping and must remain readable at the supported player limit.

**Independent Test**: Complete controlled 2-, 6-, 7-, and 8-player rounds and verify every player's identity, income breakdown, total, and wallet are simultaneously visible and legible before continuing.

**Acceptance Scenarios**:

1. **Given** an eight-player round has ended, **When** accounting appears, **Then** all eight players and their damage income, placement income, total earnings, and resulting wallet are visible within the view with no clipped or scroll-only content.
2. **Given** an accounting view with any supported player count, **When** a player compares values, **Then** player identity, income categories, round total, and wallet are visually distinct and readily associated with the same player.
3. **Given** accounting is displayed, **When** the player acknowledges it, **Then** the existing progression to shopping remains available and unambiguous.

---

### User Story 2 - Buy a varied non-nuclear arsenal (Priority: P1)

During shopping, a player can afford ordinary limited weapons much more readily, while the Nuke remains a rare strategic purchase rather than an item that can be stockpiled after ordinary earnings.

**Why this priority**: The shop should encourage weapon variety without making its highest-consequence weapon routine.

**Independent Test**: Inspect every shop item and attempt affordable and unaffordable purchases with controlled balances; confirm ordinary weapon prices are exactly one tenth of their prior prices and the Nuke remains $15,000.

**Acceptance Scenarios**:

1. **Given** any limited non-Nuke weapon is shown in the shop, **When** its price is compared to the prior price list, **Then** it is exactly one tenth of that prior price.
2. **Given** the Nuke is shown in the shop, **When** its price is inspected, **Then** it remains $15,000 and is greater than twenty times the highest-priced non-Nuke weapon.
3. **Given** a player buys a weapon, **When** the purchase succeeds or is rejected for insufficient cash, **Then** the displayed price, wallet, and ammunition retain the existing exact and atomic purchase behaviour.

---

### User Story 3 - Move freely until ending the turn (Priority: P1)

After choosing move mode, a player can take as many directional movement steps as desired across any in-bounds terrain, then explicitly finish movement with Space or Enter.

**Why this priority**: Movement should be a player-controlled positioning choice, not be constrained by an arbitrary allowance or terrain steepness gate.

**Independent Test**: Start a movement turn on steep and cratered terrain, make more than six valid in-bounds steps including steep transitions, then finish with both Space and Enter in separate trials.

**Acceptance Scenarios**:

1. **Given** the active player enters move mode, **When** they make any number of in-bounds directional requests, **Then** each request moves only their tank and no six-step or other movement allowance ends or rejects the turn.
2. **Given** an in-bounds destination with any terrain elevation difference, **When** the player requests a movement step to it, **Then** the tank may move there and is placed on the current terrain surface.
3. **Given** move mode is active, **When** the player presses Space or Enter, **Then** movement ends immediately, the final tank position is retained, and the next eligible player's turn begins.
4. **Given** a requested destination is outside the battlefield, **When** movement is requested, **Then** it is rejected without changing tank position or ending movement.

---

### User Story 4 - Operate setup and battlefield cues comfortably (Priority: P2)

A local player can change the selected setup slot between Human and AI using Tab, can clearly read the top wind-direction arrow, and sees a battlefield that blends naturally into its surrounding scene rather than ending at an obvious square boundary.

**Why this priority**: These recurring controls and visual cues should feel intentional and legible during ordinary play.

**Independent Test**: Select setup slots and toggle each controller type using Tab, exercise all wind directions against light and dark terrain, and inspect all four battlefield edges and corners from normal tactical camera positions.

**Acceptance Scenarios**:

1. **Given** a setup slot is selected, **When** Tab is pressed, **Then** that slot toggles between Human and AI while preserving its stable identity and visual identity; no modifier chord is required.
2. **Given** any supported wind direction or strength, **When** the tactical HUD is visible, **Then** its top wind arrow is clearly distinguishable from the background and remains directionally consistent with the existing wind display.
3. **Given** the normal tactical camera views an edge or corner of the playable surface, **When** the surrounding scene is visible, **Then** colour and tonal transition soften the boundary so the playable area appears to continue naturally beyond its true square limits.

### Edge Cases

- Accounting remains fully usable for a draw, eliminated players, zero earnings, large wallet values, and long supported display names.
- Basic Shell stays unlimited and unpurchasable; it has no price change.
- Repeated movement input can continue indefinitely while the tank remains in bounds, but never moves another tank or permits firing/aim changes during the movement turn.
- A tank may cross steep terrain and crater walls, but cannot leave the battlefield; terrain deformation remains authoritative for the ground height it occupies.
- Tab affects only the selected editable setup slot and does not accidentally trigger setup navigation or a gameplay action.
- Brighter wind presentation must remain visible across representative day/night and terrain-colour conditions without obscuring the wind label or other HUD information.
- The perceived extended battlefield is presentation only: it does not enlarge combat bounds, alter terrain, hide edge rejection, or change projectile/settling rules.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The accounting view MUST display all accounting fields for every configured player in supported 2–8 player matches simultaneously, without vertical scrolling, clipping, or overlapping essential content.
- **FR-002**: Accounting MUST clearly group each player's identity, damage income, placement income, total earnings, and resulting wallet, and MUST use a deliberate visual hierarchy consistent with the game's round-flow presentation.
- **FR-003**: The accounting view MUST preserve its existing acknowledge/continue path and MUST not accept battlefield controls while active.
- **FR-004**: Every currently purchasable limited weapon other than Nuke MUST have its prior shop price divided by exactly 10; the Basic Shell MUST remain unlimited and without a purchase price.
- **FR-005**: Nuke MUST retain a shop price of $15,000, which MUST exceed twenty times the highest non-Nuke limited weapon price after this feature's reduction.
- **FR-006**: A valid or rejected shop purchase MUST continue to change ammunition and cash together exactly as indicated by the displayed price, and MUST never create negative cash.
- **FR-007**: Entering move mode MUST provide no finite movement-step allowance or automatic movement-turn completion.
- **FR-008**: While move mode is active, the current player's tank MUST accept any number of ordered in-bounds directional steps across terrain regardless of elevation difference, remain grounded on the current terrain at each accepted position, and preserve existing ownership and turn safety.
- **FR-009**: Battlefield bounds MUST remain the sole terrain-based rejection for movement; steepness MUST not reject an otherwise in-bounds requested step.
- **FR-010**: Space and Enter MUST each explicitly complete an active movement turn; while moving, neither key may fire a weapon. Existing move-mode restrictions on firing and aim adjustment remain in effect until movement ends.
- **FR-011**: Match Setup MUST allow Tab to toggle the selected slot's controller type between Human and AI without a modifier chord, preserving existing controller-transition behaviour for names, identity, colour, and start validation.
- **FR-012**: Setup's visible control guidance MUST identify Tab as the controller-toggle command and MUST no longer require Control-C to make this change.
- **FR-013**: The top tactical wind arrow MUST have sufficient brightness and contrast to remain immediately visible against representative battlefield and HUD backgrounds while preserving its existing direction and wind relationship.
- **FR-014**: Battlefield-edge presentation MUST create a continuous-looking colour and tonal transition between the square playable surface and its surrounding background at all four edges and corners, improving the illusion of a larger landscape without changing gameplay bounds.
- **FR-015**: Automated coverage MUST verify the 2–8 accounting layout data, all changed prices and the Nuke price separation, unlimited free movement including steep terrain and bounds rejection, both movement-completion keys, Tab controller toggling, wind-arrow visibility configuration, and unchanged gameplay boundaries.
- **FR-016**: Before completion, relevant documentation and roadmap status MUST be reviewed; the workspace build, relevant automated tests, formatting, and lint checks MUST pass without unjustified warnings.

### Key Entities

- **Accounting entry**: One player's stable identity, income categories, round total, and post-round wallet shown in the end-of-round view.
- **Shop price list**: The authoritative price for each limited weapon, distinct from player-owned ammunition and cash.
- **Movement turn**: The active player's move-only turn, which has no step budget and ends only by explicit completion or ordinary game-state transition.
- **Setup controller selection**: The selected match-setup slot whose Human/AI controller type can be changed without altering player identity.
- **Tactical wind cue**: The visual arrow and associated wind information that communicate the existing current wind.
- **Battlefield surround**: Non-playable scenery adjacent to the fixed combat surface, used solely to make its visual boundary feel less abrupt.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In manual 2-, 6-, 7-, and 8-player accounting checks, 100% of player entries and their five required fields are visible and readable without scrolling, clipping, or overlap.
- **SC-002**: In price-table verification, 100% of the nine limited non-Nuke weapons cost exactly one tenth of their previous prices, Basic Shell remains free/unlimited, and Nuke costs $15,000—more than twenty times the highest non-Nuke price.
- **SC-003**: Across controlled flat, steep, and cratered-terrain movement trials, a player can complete 20 consecutive in-bounds steps and a separate 7-step trial without allowance rejection; 100% of resulting tank positions remain on current terrain.
- **SC-004**: In manual move-mode trials, both Space and Enter end movement and hand off exactly one turn in 100% of attempts, while neither fires a shot during movement.
- **SC-005**: In setup trials covering every supported slot, Tab changes only the selected slot's controller type in 100% of attempts without a modifier key.
- **SC-006**: In representative tactical views spanning all wind directions and battlefield edge/corner views, the wind arrow is recognizable at a glance and no hard, conspicuous playable-surface-to-background colour seam dominates the scene.

## Assumptions

- The supported match range remains 2–8 players; the accounting layout must solve every count in that established range rather than introduce scrolling or reduce displayed data.
- “Cheaper by a factor of 10” means exactly one tenth of each existing non-Nuke limited weapon price; Nuke's present $15,000 price is deliberately retained as the requested stockpiling deterrent.
- “Anywhere” means any in-bounds location on the existing battlefield surface, not movement outside game bounds, through other tanks, or changes to unrelated combat restrictions.
- Space and Enter are both completion commands only while move mode is active; the established fire command remains available in its normal firing state.
- The existing wind direction/strength semantics are correct; this feature changes only its visual legibility.
- Battlefield blending is a visual treatment around the current combat surface, not procedural terrain expansion, new map size, altered projectile collision, or a change to the visible boundary when a move is rejected.
- This feature depends on the existing 2–8 match setup, round accounting/shop loop, weapon catalogue, move-or-fire state, tactical wind HUD, and battlefield presentation.

## Out of Scope

- New accounting reward categories, currencies, shop product types, inventories, weapons, or broad economy rebalance beyond the stated price changes.
- AI decision-making, setup restructuring, mouse/controller remapping, persistent custom key bindings, or other setup controls beyond the controller-toggle command.
- Pathfinding, collision avoidance, vehicle physics, movement animation redesign, movement attacks, out-of-bounds traversal, or firing during move mode.
- Changing wind simulation, wind strength, direction semantics, HUD layout unrelated to visibility, battlefield dimensions, terrain generation, combat bounds, projectile collision, or terrain deformation rules.
- A full art overhaul, new environments, dynamic sky, or visual changes unrelated to the battlefield boundary transition and wind cue legibility.

## Roadmap Alignment

- On completion, review the existing multiplayer readability, shop/economy, movement, setup controls, tactical HUD, and battlefield-presentation roadmap items; mark only demonstrated acceptance criteria complete.
- Record broader economy balancing, movement design, control customization, and environment/art discoveries separately rather than expanding this focused polish feature.

## Constitution Compliance

- The feature improves an already playable local artillery loop through visible, bounded gameplay and presentation changes. It deliberately removes an arbitrary movement gate in favour of understandable player control while retaining deterministic in-bounds terrain following.
- It preserves the existing authoritative game rules beneath presentation: accounting and wind cues display state, and battlefield blending does not own collision or combat bounds.
- Price changes are a small, explicit tuning pass; broader balance remains outside scope. Existing simple systems and tested, deterministic gameplay continue to be preferred over speculative frameworks.
