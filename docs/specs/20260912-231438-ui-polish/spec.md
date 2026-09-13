# Feature Specification: Azimuth UI Polish — A Finished Visual Language for HUD, Shop and Game Flow

**Feature Branch**: `20260912-231438-ui-polish`

**Created**: 2026-09-12

**Status**: Draft

**Input**: Polish the existing functional interface into one coherent, graphical, arcade-artillery visual language without changing gameplay authority.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Read the battlefield at a glance (Priority: P1)

A player in a live round sees a compact, intentional HUD that frames rather than covers the battlefield. The active player, health, action, aiming values, wind, player status, and selected weapon are easy to find. The weapon bar is a graphical inventory with recognisable weapon icons, readable labels, ammunition, and unambiguous selected and disabled states.

**Why this priority**: This is the interface players use continuously during the core artillery game.

**Independent Test**: Run 2-, 4-, and 8-player rounds; identify the active player, selected weapon, ammunition, aim, and wind without ambiguity, and select an available weapon by mouse.

**Acceptance Scenarios**:

1. **Given** every current playable weapon, **When** it appears in the battle bar, **Then** it has a distinct graphical icon, compact readable name, and authoritative ammunition state; Bomb Net remains absent.
2. **Given** a weapon is selected or exhausted, **When** its slot is shown, **Then** selected and unavailable states are visually distinct without relying on text colour alone.
3. **Given** an active turn, **When** a player views the HUD, **Then** player/action, health, exact aim values, and wind direction/strength remain readable while the battlefield stays dominant.

---

### User Story 2 - Browse a beautiful artillery shop (Priority: P1)

Between rounds, a shopper sees a visually satisfying catalogue of weapon products. Icon artwork is the visual focus; names, price, owned ammunition, affordability, purchase feedback, shopper identity, cash, and Done action form a clear hierarchy. The existing shop rules and persistent inventory remain unchanged.

**Why this priority**: The shop is where the existing economy becomes emotionally meaningful and is the strongest opportunity to showcase Azimuth's arsenal.

**Independent Test**: Complete a round, enter the shop with enough money for some but not all products, buy an item, and confirm the card/wallet/count respond immediately while unaffordable cards remain visible and subdued.

**Acceptance Scenarios**:

1. **Given** an active shopper, **When** the shop opens, **Then** their identity, colour accent, current cash, round context, product grid, and Done action are immediately clear.
2. **Given** affordable and unaffordable products, **When** cards are viewed or hovered, **Then** all cards remain browsable, affordable Buy actions respond clearly, and unavailable Buy actions remain legible but subdued.
3. **Given** a successful purchase, **When** it completes, **Then** existing authoritative cash/ammunition changes are reflected immediately with restrained visual and audio feedback.
4. **Given** a weapon appears in shop and battle, **When** a player compares them, **Then** both use the same proper icon and visual identity.

---

### User Story 3 - Experience one coherent match flow (Priority: P2)

From match setup through winner presentation, accounting, shopping, and the next round, screens feel like parts of one arcade-artillery game. Buttons, panels, headings, spacing, typography, player colour accents, hover/pressed/disabled feedback, and continuation cues follow a shared visual language.

**Why this priority**: Functional flow currently crosses multiple successful but visually inconsistent development screens.

**Independent Test**: Run Setup → Round → Winner → Accounting → Shop → Round 2 and verify that no screen reverts to raw/debug-like presentation and mouse controls work naturally where buttons exist.

**Acceptance Scenarios**:

1. **Given** match setup, result, accounting, and shop screens, **When** the player progresses through them, **Then** they share panel, button, typography, spacing, and accent conventions.
2. **Given** a round winner, **When** result presentation appears, **Then** the winner and round are celebrated while the battlefield and winning tank remain visible.
3. **Given** accounting, **When** values are displayed, **Then** damage income, placement award, total, and wallet have clear numerical hierarchy without changing economy rules.
4. **Given** buttons across existing UI-heavy screens, **When** they are normal, hovered, pressed, selected, or disabled, **Then** their states are consistently distinguishable and mouse feedback is immediate.

### Edge Cases

- Icons must exist for every actual current weapon and never invent or revive removed weapons.
- Unlimited Basic Shell, zero-ammunition weapons, unavailable shop products, eliminated players, and eight-player scoreboards remain readable and visually distinct.
- A missing required visual asset fails visibly in development rather than silently degrading to an unusable blank control.
- The UI remains usable at 1280×720, 1920×1080, and one wider desktop layout without required controls clipping or covering critical battle space.
- Presentation reads authoritative state but does not own cash, inventory, health, winner, or round transition behavior.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The game MUST define a small reusable Azimuth visual language for panels, borders, text hierarchy, spacing, colours, buttons, cards, active, disabled, and warning states.
- **FR-002**: The game MUST provide original, coherent, transparent-background graphical icons for every current playable weapon and use no emoji or inconsistent external icon collection.
- **FR-003**: The battlefield weapon bar and shop catalogue MUST use the same weapon icon identity, readable name, and authoritative ammunition state.
- **FR-004**: The battle bar MUST make selected, limited, unlimited, and exhausted states unmistakable while preserving comfortable mouse hit targets and battlefield focus.
- **FR-005**: The shop MUST present product artwork prominently with shopper identity, cash, price, owned ammunition, Buy/Done hierarchy, hover/pressed/disabled states, and bounded purchase feedback.
- **FR-006**: The UI MUST preserve existing authoritative gameplay, economy, shop, AI, and round-transition rules; presentation actions may only request existing domain operations.
- **FR-007**: Setup, HUD, winner, accounting, shop, and continuation screens MUST apply the shared visual language and clear next-action cues.
- **FR-008**: Winner presentation MUST celebrate player identity and round while preserving view of the battlefield.
- **FR-009**: Accounting MUST clearly distinguish damage income, placement income, total earnings, and current wallet using consistent numeric formatting.
- **FR-010**: The 2–8 player status panel, health, active, and eliminated states MUST remain compact and readable.
- **FR-011**: Aim and wind displays MUST retain exact gameplay values while gaining intentional instrumentation/visual hierarchy.
- **FR-012**: UI-heavy controls MUST provide consistent mouse hover, pressed, selected, and disabled feedback; existing useful keyboard shortcuts remain available.
- **FR-013**: A restrained UI sound pass MUST cover successful primary UI actions and purchases without overwhelming artillery audio or beeping on pointer movement.
- **FR-014**: New assets MUST be organised with clear project provenance and packaged with existing desktop builds.
- **FR-015**: The roadmap MUST be updated only for UI, mouse, scale, clutter, focus, sound, visual, and flow outcomes verified by implementation and manual acceptance.

### Key Entities

- **Azimuth UI Theme**: Shared visual constants and reusable presentation rules for existing UI surfaces.
- **Weapon Icon Identity**: Original graphical artwork linked to the authoritative current weapon identity and shared by shop and battle inventory.
- **UI Interaction State**: Normal, hover, pressed, selected, and disabled visual responses for controls.
- **Flow Presentation**: The setup, result, accounting, shop, and transition views that project existing session state.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All current playable weapons have distinct graphical icons in both battle inventory and shop; no removed or non-playable weapon is shown.
- **SC-002**: In 2-, 4-, and 8-player manual sessions, players can identify active player, health, selected weapon, ammunition, aim, and wind without UI overlap or clipping at the three representative desktop layouts.
- **SC-003**: In a completed round/shop test, 100% of product cards show icon, name, price, owned count, and correct affordability state; successful purchases visibly update cash and ammunition immediately.
- **SC-004**: A full Setup → Round → Winner → Accounting → Shop → Round 2 flow uses consistent panel, button, typography, and colour treatment with no raw text-only functional screen.
- **SC-005**: Automated checks cover icon/catalogue identity, authoritative ammo/affordability mapping, unlimited/exhausted states, and 2–8 player presentation mappings.

## Assumptions

- The current playable arsenal is Basic Shell, High Explosive, Heavy Shell, MIRV, Cluster Bomb, Roller, Bunker Buster, Dirt Bomb, Curve Ball, Bouncer, and Nuke.
- The existing session shop, weapon presentation identity, HUD, Bevy UI controls, audio assets, and CI packaging are the foundations to polish rather than replace.
- One compact arcade-artillery type hierarchy and one coherent icon style are sufficient; no theme, skin, localisation, accessibility-suite, controller-navigation, or settings-system work is introduced.
- Armour, weapons, economy rules, statistics screens, trajectory/damage previews, and gameplay changes remain out of scope.
