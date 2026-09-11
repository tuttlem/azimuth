# Feature Specification: Rounds, Economy and Weapon Shop — The Continuing Game Loop

**Feature Branch**: `20260911-222730-rounds-economy-shop`  
**Created**: 2026-09-11  
**Status**: Draft  
**Input**: Continuing local game loop with round celebration, accounting, weapon shopping, and fresh rounds.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Continue a local game (Priority: P1)

Players finish an artillery round, see its winner or draw celebrated, then enter accounting and shopping without restarting Azimuth.

**Why this priority**: A complete battle becomes one round in a continuing game.

**Independent Test**: Finish a two-player round, select Continue from the result overlay, and reach accounting.

**Acceptance Scenarios**:

1. **Given** one surviving player, **When** final resolution settles, **Then** a central overlay names that player as round winner, the camera focuses their surviving tank, and a visible Continue action is available.
2. **Given** no surviving player, **When** final resolution settles, **Then** a draw overlay and continuation action are available.
3. **Given** celebration, accounting, shop, or round loading is active, **When** battlefield inputs are used, **Then** they cannot move, aim, fire, or select weapons.

---

### User Story 2 - Earn and understand cash (Priority: P1)

Every configured player sees clear earnings from actual opponent damage, eliminations, and a winner bonus where applicable.

**Why this priority**: Players need an understandable reason for each balance before buying ammunition.

**Independent Test**: Resolve controlled opponent damage, self-damage, elimination, and a winner; compare accounting with actual health removed.

**Acceptance Scenarios**:

1. **Given** opponent health is removed, **When** authoritative damage resolves, **Then** its attacker earns the configured reward for exactly that health removed.
2. **Given** an opponent is first eliminated, **When** accounting finalises, **Then** the responsible attacker earns exactly one elimination bonus.
3. **Given** a winner or draw, **When** accounting finalises, **Then** the winner receives exactly one round-win bonus and a draw receives none.
4. **Given** self-damage, terrain-only impact, or overkill, **When** accounting finalises, **Then** no inappropriate cash is awarded.

---

### User Story 3 - Shop between rounds (Priority: P1)

Humans sequentially buy individual limited weapon rounds, mark themselves ready, and AI shops immediately with a bounded valid choice.

**Why this priority**: Shopping turns round earnings into the next round's tactical choices.

**Independent Test**: Buy an affordable weapon, observe immediate cash/inventory change, ready all humans, and begin the next round with AI participants.

**Acceptance Scenarios**:

1. **Given** a human's shop turn and adequate cash, **When** they buy one limited weapon round, **Then** cash decreases by its listed price and that ammunition increases by one immediately.
2. **Given** insufficient cash or a non-purchasable weapon, **When** purchase is attempted, **Then** it is unavailable/rejected and cash and inventory are unchanged.
3. **Given** the Basic Shell, **When** the shop is displayed, **Then** it is marked Unlimited and has no finite purchase action.
4. **Given** all humans are READY and all AIs have shopped, **When** the shop completes, **Then** the next round begins without restart or developer tooling.

---

### User Story 4 - Preserve session, refresh round (Priority: P2)

Players start a fresh battlefield round with restored combat state while their identity, controller, colour, cash, unused ammunition, purchases, and wins persist.

**Why this priority**: This is the game/session boundary that makes continuing play work.

**Independent Test**: Consume one limited round, purchase another after result, start Round 2, and verify fresh tanks/terrain with preserved session values.

**Acceptance Scenarios**:

1. **Given** shop completion, **When** the next round starts, **Then** round number increments and all configured players receive full-health tanks on fresh terrain.
2. **Given** pre-transition cash and ammunition, **When** the next round starts, **Then** unused and newly bought ammunition plus cash are retained exactly.
3. **Given** Human and AI players, **When** rounds transition, **Then** PlayerIds, names, controller kinds, and visual identities stay stable.
4. **Given** previous projectiles, effects, aiming, eliminations, and temporary UI, **When** a fresh round begins, **Then** none remains active.

---

### User Story 5 - End a session (Priority: P3)

Players may end between-round play and return to normal setup without closing Azimuth.

**Independent Test**: Select END GAME from accounting/shop, then begin a new setup game and verify no prior cash or inventory remains.

**Acceptance Scenarios**:

1. **Given** accounting or shop, **When** END GAME is selected, **Then** the session is discarded and ordinary setup is available.
2. **Given** a new game after END GAME, **When** Round 1 begins, **Then** it uses standard starting inventory, not the former session.

### Edge Cases

- Splash, multi-projectile, and multi-stage weapons reward actual authoritative opponent health removed, never projectile/effect count; overkill caps at remaining health.
- Self-damage earns neither damage nor elimination cash; a final all-dead explosion is a draw.
- Eliminated players retain earned cash, receive accounting, shop, and respawn next round.
- 2–8 players, including all-AI and zero-cash configurations, always complete shop progression safely.
- A player with no limited ammunition always starts with the unlimited Basic Shell.
- Next-round reset clears old projectiles, effects, tanks, terrain, and temporary UI even after barrages.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST distinguish continuing in-memory game/session state from disposable round combat state without a broad terminology-only rename.
- **FR-002**: Session state MUST preserve every configured player's PlayerId, name, Human/AI controller kind, visual identity, integer cash, finite ammunition, and round wins across rounds.
- **FR-003**: Round state MUST include only the current battlefield, tanks/health, turns, aiming, projectiles, eliminations, result, and transient presentation.
- **FR-004**: A new session's first round MUST use the current standard starting inventory and MUST not require shopping first.
- **FR-005**: Limited ammunition consumed, unused, or purchased MUST persist correctly across rounds; Basic Shell MUST remain unlimited and non-purchasable.
- **FR-006**: The result phase MUST show a winner/draw overlay, winner name where applicable, winner camera focus, and a visible continuation action.
- **FR-007**: Battlefield controls MUST be locked outside active round combat.
- **FR-008**: The system MUST display Round 1 and increment the round number exactly once per fresh round.
- **FR-009**: Opponent damage MUST earn configured integer cash from actual applied health only; self-damage, terrain-only impact, firing, survival, visual effects, and overkill MUST not create excess rewards.
- **FR-010**: Eliminations MUST award one configured bonus to authoritative opponent damage owner only; winner MUST receive one configured bonus; draws receive none.
- **FR-011**: Every player MUST receive accounting separating damage reward, elimination bonus, winner bonus, total earnings, and available cash.
- **FR-012**: The shop MUST list all current limited weapons with name, price, owned count, and one-round buy action; each price MUST be central configuration.
- **FR-013**: Validated purchases MUST add exactly one round and deduct price; invalid, unlimited, or unaffordable purchases MUST change nothing and never yield negative cash.
- **FR-014**: Humans MUST shop sequentially in configured order with READY/DONE; AI MUST finish immediate bounded deterministic affordable-valid shopping or buy nothing.
- **FR-015**: A new round MUST recreate terrain, tanks, health, turns, movement, aiming as appropriate, projectiles, eliminations, and effects while preserving session state.
- **FR-016**: Controlled gameplay randomisation for terrain, starts, and AI MUST remain isolated from cosmetic and shop activity.
- **FR-017**: END GAME MUST return to setup and discard session cash/inventory.
- **FR-018**: Existing per-round setup, firing, all weapons, inventory consumption, damage, terrain, elimination, victory/draw, and AI turns MUST remain mechanically unchanged.

### Key Entities

- **Game Session**: In-memory configured participants, wallets, finite inventories, round number, and wins.
- **Round**: Disposable battlefield, tanks, health, turns, projectiles, result, and temporary effects.
- **Round Accounting**: Per-player damage, elimination, win earnings and resulting balance.
- **Weapon Price List**: Authoritative prices for limited weapons only.
- **Shop Progress**: Ordered human readiness and completed AI shopping state.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A two-player local game can complete a round, show result/accounting, buy ammunition, and begin Round 2 without restart or developer tools.
- **SC-002**: Controlled damage, overkill, self-damage, elimination, winner, and draw tests produce accounting totals matching configured rewards in 100% of cases.
- **SC-003**: Across 2-, 4-, and 8-player transitions, 100% of configured identities, controller kinds, colours, cash, and surviving ammunition are retained.
- **SC-004**: A next round after a barrage or large explosion has zero active projectile, temporary-impact, eliminated-tank, or former-terrain state.
- **SC-005**: Every valid human purchase updates balance and ammunition before the next interaction; invalid purchases change neither.
- **SC-006**: All-AI 2–8 player sessions complete shopping without input and no AI balance becomes negative.
- **SC-007**: In graphical play, users can identify result, explain accounting, finish local shopping, and start another round unaided.

## Assumptions

- Starting inventories remain current values. Initial tuning is $10 per actual opponent health, $250 per elimination, and $500 per win; weapon prices are centrally configured and tuned during implementation.
- Shop is sequential and purchases are visible to all local players; no secrecy, accounts, disk persistence, fixed session length, profiles, or network play is needed.
- AI shopping uses an isolated deterministic stream and simple bounded policy.
- Existing result/damage are authoritative; overlays and camera are presentation only.
- Existing terrain/world rebuild path is the dependency for fresh rounds.

## Dependencies

- Existing PlayerId configuration, controller types, colours, setup, authoritative damage/ownership, result detection, loadouts, HUD, weapon UI, camera, deterministic seeds, AI turns, and battlefield lifecycle.

## Out of Scope

- General economy/item systems, sales, stock, bundles, discounts, progression, upgrades, tank classes, new combat mechanics, persistence, networking, leaderboards, detailed statistics, sophisticated AI valuation, victory cinematics, or new environment/physics/rendering systems.

