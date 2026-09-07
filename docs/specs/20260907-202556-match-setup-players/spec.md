# Feature Specification: Match Setup — Named 2–8 Player Matches and Controller Slots

**Feature Branch**: `feature/match-setup-players`

**Created**: 2026-09-07

**Status**: Draft

**Input**: Add Match Setup that creates configured named 2–8 player local artillery matches and establishes Human/AI controller slots without implementing AI play.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Configure and Start a Human Match (Priority: P1)

As a local player, I launch into Match Setup, choose two through eight players, edit human names,
see slot colours and controller types, and start a valid all-human match.

**Why this priority**: Setup must create real authoritative gameplay rather than decorative options.

**Independent Test**: For every count from 2 through 8, create an all-human configuration, edit a
name, start it, and verify exactly those named players enter play.

**Acceptance Scenarios**:

1. **Given** Azimuth launches, **When** Match Setup appears, **Then** the default two-human configuration is valid and gameplay has not begun.
2. **Given** the user changes count from 2 through 8, **When** setup updates, **Then** exactly that many visible slots form the resulting match with no stale hidden players.
3. **Given** a human slot, **When** its display name is edited to a valid value, **Then** its name changes but stable identity and colour do not.
4. **Given** invalid count or name data, **When** Start Match is requested, **Then** gameplay does not start and the reason is clear.

---

### User Story 2 - Configure Future AI Slots Honestly (Priority: P1)

As a player, I can mark a slot Human or AI and see a playful generated AI name, while setup clearly
blocks AI-containing matches until AI turns exist.

**Why this priority**: Controller type belongs in configuration now, but fake or stuck AI matches
would undermine the game.

**Independent Test**: Change several slots to AI, verify identity/colour remain stable and names
come from the built-in pool with preferred uniqueness, then verify Start is disabled with an
AI-unavailable explanation. Change one back to Human and verify it is editable.

**Acceptance Scenarios**:

1. **Given** a Human slot, **When** it becomes AI, **Then** it retains stable identity/colour, receives a generated built-in AI name, and no other slot changes.
2. **Given** multiple AI slots and unused pool names, **When** names are assigned, **Then** names are distinct within the configuration where possible.
3. **Given** one or more AI slots, **When** setup validates, **Then** Start Match is unavailable with a concise AI-not-yet-available explanation.
4. **Given** an AI slot, **When** it becomes Human, **Then** it receives a valid editable human name without identity or colour change.

---

### User Story 3 - Play a Configured N-Player Match (Priority: P1)

As a player in a configured 3–8 human match, I take ordinary move-or-fire turns in slot order and
each player independently owns tank, health, aim, inventory, selected weapon, and movement state.

**Why this priority**: The feature succeeds only when the running game genuinely stops being a
hard-coded duel.

**Independent Test**: Start representative 2-, 3-, 4-, and 8-human matches. Verify configured
players, turn wrapping, and that one player's aim or finite ammunition never alters another's.

**Acceptance Scenarios**:

1. **Given** six configured humans, **When** their match starts, **Then** six distinct supported tanks with configured names/colours, default aim, and standard inventories exist.
2. **Given** a human turn, **When** the active player aims, moves, selects, or fires, **Then** only that player's existing legal state changes.
3. **Given** no eliminations, **When** turns resolve, **Then** slot order advances and wraps after the final player.
4. **Given** one player changes aim or uses a final Heavy Shell round, **When** another player acts later, **Then** all other independent state remains unchanged.

---

### User Story 4 - Resolve Multiplayer Elimination and Victory (Priority: P2)

As a player, I see all affected players take damage and settle after terrain deformation; eliminated
players are skipped and the final survivor's configured name wins.

**Why this priority**: Extra tanks need complete authoritative turn, elimination, and victory
semantics.

**Independent Test**: Drive explosions affecting several tanks, including multiple and self
elimination, and verify all consequences complete before next-player, win, or draw selection.

**Acceptance Scenarios**:

1. **Given** an explosion overlaps several tanks, **When** it resolves, **Then** every affected player takes ordinary damage before elimination/victory evaluation.
2. **Given** eliminated players in order, **When** a survivor completes a turn, **Then** all eliminated identities are skipped to the next survivor.
3. **Given** one survivor, **When** all damage/deformation/settling completes, **Then** that configured display name wins; with none, the match draws.

---

### User Story 5 - Read a Scalable Named Tactical Match (Priority: P2)

As a player, I can read active named player, all player condition, weapon/aim/wind, and result in
a clean 2–8 player HUD, while the camera focuses the active configured tank.

**Why this priority**: A multiplayer match is unusable if identity or current control is ambiguous.

**Independent Test**: Inspect presentation for 2, 4, and 8 players including eliminations and a
non-first active player; verify named status, active emphasis, and winner text derive from players.

**Acceptance Scenarios**:

1. **Given** any configured human turn, **When** HUD updates, **Then** it shows active display name, selection/ammunition, aim, wind, movement, and all player health/alive state.
2. **Given** eight players with eliminations, **When** status is displayed, **Then** every player remains identifiable, active state is clear, and the battlefield remains the focus.
3. **Given** a later configured player becomes active or wins, **When** presentation updates, **Then** camera and human-facing text use that player without Player One/Two paths.

### Edge Cases

- Counts below 2 or above 8 cannot create running matches; reducing count removes excess prospective slots.
- Stable identity is independent of name, controller type, colour, turn position, and collection index where practical.
- Human names are trimmed, non-empty, maximum 20 characters, and safely rendered; duplicate human names are allowed.
- AI names are setup-only random values captured in configuration; they never consume or affect gameplay simulation randomness.
- AI slots never silently become Human and cannot start until a later AI-controller feature.
- Spawn positions are deterministic, supported, in bounds, non-overlapping, and sufficient for 2–8 validation; this does not claim final balance.
- Several simultaneous eliminations, self-elimination, and several settling tanks complete all authoritative consequences before survivor/turn selection.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Launch MUST show Match Setup before gameplay with a valid default two-human configuration.
- **FR-002**: Setup MUST authoritatively create a prospective configuration of 2–8 slots and clearly prevent unsupported counts.
- **FR-003**: Every player configuration MUST contain stable internal identity, display name, explicit Human/AI controller type, and one of eight curated visual identities; display text is never identity.
- **FR-004**: Human names MUST be directly editable, trimmed, non-empty, limited to 20 characters, and preserved into gameplay without changing identity.
- **FR-005**: Switching Human to AI MUST preserve identity/colour, assign a playful curated-pool AI name, and prefer an unused configuration name. Switching AI to Human MUST restore a valid editable human name.
- **FR-006**: AI name randomness MUST be setup-only and captured once; name generation/reroll MUST not affect gameplay RNG, player identity, other slots, inventory, or future simulation state.
- **FR-007**: AI behaviour MUST remain unimplemented. Any AI slot MUST disable Start Match with a clear explanation and never silently convert to Human.
- **FR-008**: Start Match MUST validate count, unique stable identities, valid names, controller types, visual identities, and AI availability before creating running state.
- **FR-009**: A valid all-human configuration MUST initialise every configured player's tank, health/alive state, aim, inventory/selection, movement state, and turn participation from configuration without `player1`/`player2` initialisation.
- **FR-010**: Running matches MUST operate over a 2–8 player collection. Existing two-player play remains a valid configuration, not a separate game path.
- **FR-011**: Every supported count MUST use deterministic, supported, in-bounds, non-overlapping, separated starting positions.
- **FR-012**: Turn order MUST initially follow slot order, wrap, and advance to next living player, skipping one or multiple eliminated players without an “other player” rule.
- **FR-013**: Existing human input MUST submit only legal current-player actions. Controller type MUST remain separate from gameplay state and new N-player match semantics MUST not depend on keyboard concepts.
- **FR-014**: Projectile, wind, damage, terrain deformation, and settling MUST process every configured tank. All shot consequences MUST resolve before next-turn, survivor, win, or draw decisions.
- **FR-015**: More than one survivor MUST continue; exactly one MUST win; zero MUST draw. Winner presentation MUST use configured display name.
- **FR-016**: All configured players' health, elimination, aim, inventory, selection, movement, tank position, and turn ownership MUST be independent.
- **FR-017**: Every player MUST receive existing Basic Shell, High Explosive, and Heavy Shell inventories via the standard weapon framework, with no new weapon or special case.
- **FR-018**: At each human choosing turn, camera MUST focus the active configured tank without camera timing gating authoritative actions or resolution.
- **FR-019**: HUD MUST derive a scalable 2–8 player status list from running players, show configured names/health/alive state, emphasise active player, and retain weapon/aim/wind/movement/result without obscuring the battlefield.
- **FR-020**: Automated tests MUST cover configuration bounds/validation, names/identity/controller/colour separation, setup AI naming and blocked AI start, 2/3/4/8 startup, independent state, rotation/skipping, multiplayer damage/settling/elimination/victory/draw, weapons, HUD, and deterministic traces.
- **FR-021**: Completion MUST update documentation and only roadmap items demonstrated by manual play, retain unfinished AI and final-balance work, and pass build, tests, formatting, and linting.

### Key Entities

- **Match configuration**: Authoritative pre-match description used once to create running state.
- **Player configuration**: Stable identity, display name, controller type, and visual identity; distinct from mutable gameplay state.
- **Controller type**: Explicit Human or AI decision-source designation; it does not create a separate tank/weapon/player model.
- **Running player state**: A configured player's tank, health, survival, aim, inventory, selection, movement, and turn participation.
- **Player collection**: Ordered participants used for spawning, effects, HUD, survivors, and turn rotation.
- **AI-name pool**: Curated setup-only names that become captured configuration display values.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A user can create and start valid all-human matches for each count 2–8, and every match contains exactly the selected number of distinct named supported tanks.
- **SC-002**: Automated cases accept all seven valid counts and reject every tested count below 2 or above 8; all invalid names and AI-containing setups are prevented from starting with stated reasons.
- **SC-003**: In automated eight-player traces, changing one player's aim, finite ammunition, health, or elimination changes none of the other seven corresponding state.
- **SC-004**: Deterministic 2-, 3-, and 8-player traces visit living identities in slot order, wrap correctly, and skip all eliminated identities with no duplicate or extra turn.
- **SC-005**: Controlled multiplayer explosions resolve all damage and settling before 100% of resulting next-player/winner/draw decisions; every configured identity can be the displayed winner.
- **SC-006**: Setup and tactical UI show configured identities legibly for 2, 4, and 8 players, with active player clear and central battlefield substantially unobstructed.
- **SC-007**: Manual 3–8 human matches demonstrate weapons, movement, camera, HUD, terrain, settling, elimination, and named victory without two-player-only paths; AI remains visibly unavailable.

## Assumptions

- Default Human names are `Player 1` through `Player 8`; the built-in AI pool has at least 16 short, distinct Azimuth-flavoured names.
- AI-name reroll is desirable but may be deferred to the roadmap if it makes the compact setup UI materially more complex; automatic naming is required.
- Slot order is initial turn order. The smallest battlefield/layout adjustment needed for functional eight-player validation is allowed, while final scale/fairness remains roadmap work.
- This feature depends on existing deterministic projectiles, wind/gravity, terrain, combat, settling, move-or-fire, camera, HUD, and three-weapon inventory systems.

## Out of Scope

- AI decisions, aiming, movement, weapons, targeting, difficulty, memory, or ballistic solving.
- Teams, networking, online lobby, profiles, persistence, accounts, avatars, skins, colour picker, custom tanks, or cosmetics.
- Battlefield/environment/loadout/health/modifier/turn-order selection, generic menu/settings framework, procedural terrain/spawning, final fair multiplayer balance, or rematch flow.
- New weapons, trajectory prediction, controller-device support, audio/VFX, or unrelated gameplay changes.

## Roadmap Alignment

- On completion, review and potentially check **Match Setup / Select number of players** and **Select human/AI players** only at configuration level; retain **AI Opponents** as unimplemented.
- Check scalable HUD, turn order, local multiplayer, and independent-state items only where manual 3–8 player evidence supports them.
- Do not mark final battlefield scale/balance complete because eight tanks fit. Record AI, spawn, camera, and scale discoveries rather than expanding scope.

## Constitution Compliance

- This introduces the smallest real configuration and player-collection model needed for local multiplayer, not AI simulation or a generic menu/controller framework.
- Setup naming randomness is isolated from deterministic gameplay; UI reads/edits configuration but does not own running authoritative state.
- Existing simple projectile, weapon, terrain, and turn systems are preserved while demonstrated two-player assumptions are removed.
