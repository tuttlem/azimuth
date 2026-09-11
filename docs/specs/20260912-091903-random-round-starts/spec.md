# Feature Specification: Randomized Round Starts

**Feature Branch**: `master`  
**Created**: 2026-09-12  
**Status**: Draft  
**Input**: User description: "when a new round starts, each of the players positions should be randomized. The terrain should also be randomized. It should be a completely new game for each round."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Begin a distinct new round (Priority: P1)

After completing a round and proceeding to the next, local players enter a newly generated battlefield rather than replaying the former battlefield state.

**Why this priority**: Fresh starts prevent later rounds from being predetermined by destroyed terrain and former tank placement.

**Independent Test**: Complete a round, begin the following round, and verify that the battlefield has new terrain and that every player has a valid, newly assigned starting location.

**Acceptance Scenarios**:

1. **Given** a completed round with changed terrain and defeated tanks, **When** the next round begins, **Then** the former terrain changes, tanks, projectiles, effects, eliminations, turn state, and temporary combat state are absent.
2. **Given** two or more configured players, **When** a new round begins, **Then** every player has one living, full-health tank at a valid starting location on the new terrain.
3. **Given** a player identity and session-owned resources, **When** that player is assigned a new start, **Then** their identity, controller kind, visual identity, cash, wins, and ammunition persist while their combat position and health are renewed.

---

### User Story 2 - Receive fair playable starting positions (Priority: P1)

Players start each round in locations that vary from round to round without overlapping, spawning outside the battlefield, or being placed where normal play cannot begin.

**Why this priority**: Random placement only improves a round when every participant can immediately take part.

**Independent Test**: Generate rounds for every supported player count using a range of controlled random seeds; inspect that each player has a distinct valid start on terrain.

**Acceptance Scenarios**:

1. **Given** any supported player count, **When** a round is generated, **Then** no two player tanks occupy overlapping starting positions.
2. **Given** generated terrain and starts, **When** the first turn begins, **Then** each tank is within the playable battlefield and rests on a valid terrain-supported location.
3. **Given** the same explicit gameplay-randomness seed and participant configuration, **When** a round is generated again, **Then** terrain and starting positions are reproduced exactly.

---

### User Story 3 - Experience round-to-round variety (Priority: P2)

Players see meaningful variation in the battlefield layout and start arrangement across a continuing local game.

**Why this priority**: Variety makes each round a new tactical problem instead of a reset of the same one.

**Independent Test**: Start several consecutive rounds with normal non-fixed gameplay randomness and compare the terrain profile and start-position arrangement.

**Acceptance Scenarios**:

1. **Given** normal gameplay randomness, **When** consecutive rounds begin, **Then** their terrain or their player-start arrangement differs in each observed sequence of five rounds.
2. **Given** a newly generated round, **When** active combat begins, **Then** players can use the existing movement, aiming, firing, terrain, and turn rules without special restrictions introduced by generation.

### Edge Cases

- The minimum and maximum supported player counts must receive distinct valid starts.
- A terrain candidate that cannot support all configured player starts must be discarded and regenerated or safely replaced before combat begins.
- Generation must complete with a playable round even when repeated candidate layouts are unsuitable.
- New-round generation must not retain craters, destroyed terrain, projectiles, effects, tank locations, selected weapons, aiming, or turn ownership from the prior round.
- Fixed-seed generation must remain isolated from cosmetic, interface, shopping, and AI randomness so unrelated activity does not alter the battlefield.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST create a new battlefield terrain layout whenever a new round begins, including the first round of a new game.
- **FR-002**: The system MUST assign every configured player a newly generated starting position whenever a new round begins.
- **FR-003**: The system MUST ensure each starting position is within the playable battlefield, terrain-supported, and non-overlapping with every other starting position.
- **FR-004**: The system MUST create each player's round tank with full health at its newly assigned starting position.
- **FR-005**: The system MUST fully discard prior round-only state before active combat starts, including modified terrain, tanks, health, turns, aiming, selected weapons, projectiles, effects, eliminations, result state, and temporary combat interface state.
- **FR-006**: The system MUST preserve session-owned player state across round regeneration: player identity, name, controller kind, visual identity, cash, wins, and ammunition.
- **FR-007**: The system MUST use controllable gameplay randomness such that the same explicit seed and player configuration produces the same terrain and starting positions.
- **FR-008**: The system MUST keep terrain and starting-position randomness isolated from cosmetic, interface, shopping, and AI randomness.
- **FR-009**: The system MUST reject unsuitable generated layouts and produce a valid playable layout before allowing the first turn.
- **FR-010**: The system MUST preserve existing movement, aiming, firing, weapon, damage, terrain-deformation, victory, and turn rules after a generated round begins.

### Key Entities

- **Round Battlefield**: The disposable terrain layout and all combat-only state for one round.
- **Player Start**: A player's valid, distinct terrain-supported tank location for a specific round.
- **Gameplay Randomness Seed**: The reproducible source governing terrain and player-start generation for a round.
- **Game Session**: Persistent player identity, controller, appearance, cash, wins, and ammunition retained across rounds.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Across 100 generated rounds for every supported player count, 100% begin with every configured player alive, within bounds, terrain-supported, and not overlapping another player.
- **SC-002**: Across 100 transitions from completed rounds containing terrain damage and active effects, 100% of next rounds contain no prior-round terrain deformation, tank, projectile, effect, elimination, aiming, or turn state.
- **SC-003**: For 20 repeated generations per tested seed and participant configuration, terrain and player starts match their original generation in 100% of cases.
- **SC-004**: In five consecutive normally randomized rounds, each round differs from its predecessor in terrain layout or player-start arrangement.
- **SC-005**: In a local play session, users can begin the first turn of a regenerated round with all players able to use existing combat controls without setup recovery or developer intervention.

## Assumptions

- A "new game for each round" means a wholly fresh battlefield and combat state, while continuing-session values such as identities, controllers, visual identities, cash, wins, and ammunition remain owned by the session as defined by the continuing game-loop feature.
- The current supported player-count range and battlefield bounds remain unchanged.
- Existing terrain-generation and tank-placement rules provide the baseline meaning of a valid terrain-supported position; this feature strengthens them to guarantee a valid configuration for every round.
- Normal play uses a changing gameplay seed per round; tests and debugging may supply an explicit seed for reproduction.

## Dependencies

- Existing continuing game-loop session/round boundary and next-round transition.
- Existing terrain generation, battlefield bounds, tank spawning, and gameplay seed facilities.
- Existing combat lifecycle cleanup for tanks, projectiles, effects, turn state, and temporary interface state.

## Out of Scope

- New terrain biomes, terrain themes, weather, environment hazards, changes to terrain deformation mechanics, player-selected maps, spawn balancing based on player skill, altered economy or shopping rules, persistence, network play, and changes to combat rules.
