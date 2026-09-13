# Feature Specification: Environment Presets

**Feature Branch**: `20260913-170556-environment-presets`  
**Created**: 2026-09-13  
**Status**: Draft  
**Input**: User description: "Add selectable Earth, Moon, Storm, Crusher, Turnwind, and Bowl environments with distinct gameplay and visual identities. Moon uses grey terrain and a starry night sky; Crusher uses orange/yellow/red terrain; Earth, Turnwind, and Bowl retain the normal palette; Storm is a darker normal world."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Choose a recognisable battlefield world (Priority: P1)

A player chooses one of six environment presets during match setup and can understand its physics and visual character before starting.

**Why this priority**: Environment choice should make a new match immediately feel intentional rather than be a hidden random rules change.

**Independent Test**: Cycle every preset in setup, inspect its name and concise gravity/wind summary, start a match, and confirm the selected preset remains active through rounds.

**Acceptance Scenarios**:

1. **Given** match setup is open, **When** a player cycles environment selection, **Then** Earth, Moon, Storm, Crusher, Turnwind, and Bowl are each selectable and visibly described.
2. **Given** a preset is selected, **When** the match starts and later transitions to another round, **Then** that preset's rules and visual identity remain in effect.
3. **Given** Earth is selected, **When** a match begins, **Then** it provides the current familiar gravity, moderate stable wind, and default terrain presentation.

---

### User Story 2 - Play worlds with different artillery character (Priority: P1)

Players learn distinct but simple artillery behaviour from gravity, wind, and terrain shape without trajectory aids or hidden simulation rules.

**Why this priority**: Different worlds should create new tactical stories while preserving the pleasure of discovering shots by play.

**Independent Test**: Run repeatable shots in every preset with identical aim and compare gravity, wind cadence, and terrain shape against the announced world rules.

**Acceptance Scenarios**:

1. **Given** Moon is active, **When** a player fires, **Then** shots have conspicuously longer arcs than Earth and no meaningful wind influence.
2. **Given** Storm is active, **When** a player fires, **Then** its gravity remains Earth-like while strong stable wind has a clearly greater influence than Earth.
3. **Given** Crusher is active, **When** a player fires, **Then** high gravity produces visibly shorter, more direct arcs than Earth.
4. **Given** Turnwind is active, **When** each player turn begins, **Then** wind is rerolled once, announced before aiming, and remains stable until that turn ends.
5. **Given** Bowl is active, **When** a match begins, **Then** participants occupy terrain shaped around a broad central depression that creates recognizable cover and indirect-fire opportunities.

---

### User Story 3 - Recognise each world at a glance (Priority: P2)

Players can identify their current environment from the battlefield, sky, and horizon without reducing tank, projectile, or explosion readability.

**Why this priority**: Visual identity makes the physics memorable and makes every world feel like a place.

**Independent Test**: Start each environment from normal tactical camera positions and identify it using only presentation, then verify gameplay objects remain legible.

**Acceptance Scenarios**:

1. **Given** Moon is active, **When** the scene loads, **Then** the terrain is grey/dusty and the sky is a dark star field with no clouds.
2. **Given** Crusher is active, **When** the scene loads, **Then** terrain and horizon use a readable orange, yellow, and red-brown palette with a warm hazy sky.
3. **Given** Storm is active, **When** the scene loads, **Then** the familiar terrain palette is darker and more desaturated under a cloud-heavy blue-grey sky.
4. **Given** Earth, Turnwind, or Bowl is active, **When** the scene loads, **Then** the familiar default palette remains readable; Bowl's terrain shape, not a disruptive recolour, is its primary identity.

### Edge Cases

- Selecting a preset never changes player configuration, inventories, AI difficulty, or match seed semantics beyond the documented world parameters.
- Turnwind changes wind only at a turn boundary, never during projectile flight, resolution, shopping, or setup.
- Moon's calm wind indicator remains truthful and readable.
- All six presets remain deterministic for the same match seed, selected preset, and turn order.
- Visual changes never alter combat bounds, terrain collision, HUD legibility, or player/tank identity colours.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Setup MUST offer exactly six initial presets: Earth, Moon, Storm, Crusher, Turnwind, and Bowl, with a concise visible description of each selected preset.
- **FR-002**: A selected preset MUST be retained through match start and every round transition; it MUST be included in deterministic world generation.
- **FR-003**: Earth MUST retain the current familiar gravity, moderate stable wind, default terrain character, and default visual palette.
- **FR-004**: Moon MUST use low gravity, negligible wind, grey/dust terrain presentation, and a cloudless dark star field.
- **FR-005**: Storm MUST use Earth-like gravity, stronger stable wind than Earth, a darker/desaturated Earth terrain treatment, and a cloud-heavy blue-grey sky.
- **FR-006**: Crusher MUST use higher gravity than Earth, a readable warm orange/yellow/red-brown terrain and horizon treatment, and a warm hazy sky.
- **FR-007**: Turnwind MUST use Earth-like gravity and terrain presentation, reroll deterministic wind exactly once at the start of each player turn, and show that wind before aiming.
- **FR-008**: Bowl MUST use Earth-like gravity/wind and generate a broad central depression that supplies cover while retaining valid, separated, grounded participant starts.
- **FR-009**: Every preset MUST use the existing gameplay rules for weapons, movement, turn resolution, shop, and victory; no preset may introduce trajectory previews, atmospheric drag, layered winds, or hidden player advantages.
- **FR-010**: Terrain, horizon, sky, cloud/star presentation, HUD contrast, tank visibility, projectile visibility, and explosions MUST remain readable in all six presets.
- **FR-011**: Automated coverage MUST verify preset selection/retention, relative gravity and wind behaviour, Turnwind timing/determinism, Bowl terrain/start invariants, and visual-palette configuration.
- **FR-012**: Before completion, roadmap status and user guidance MUST be updated; relevant tests, formatting, linting, build checks, and manual visual checks for every preset MUST pass.

### Key Entities

- **Environment preset**: A named, selected world definition that owns its gravity, wind policy, terrain character, and presentation identity.
- **Wind policy**: The stable-per-match or rerolled-per-turn rule governing the existing wind value.
- **Terrain character**: The deterministic terrain generation shape used by a preset, including Bowl's broad depression.
- **Environment presentation**: The preset's terrain/horizon palette and sky/cloud/star treatment, which displays but does not control gameplay.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In setup tests, all six presets are selectable, named, and retained through 100% of match start and round-transition trials.
- **SC-002**: Repeated controlled shots show Moon arcs last materially longer than Earth, Crusher arcs materially shorter than Earth, and Storm wind displacement is materially greater than Earth in 100% of deterministic comparisons.
- **SC-003**: In Turnwind trials across 20 turns, wind changes exactly once before every aimable turn, remains unchanged through the turn's shot/resolution, and repeats for the same seed and turn order.
- **SC-004**: In Bowl trials for every supported player count, 100% of starts are in bounds, separated, grounded, and placed on terrain with a recognizable central depression.
- **SC-005**: In manual normal-camera review, all six worlds are identifiable at a glance and all active tanks, projectiles, terrain contours, wind UI, and explosions remain readable.

## Assumptions

- Earth is the default selected preset and preserves the game's current behaviour.
- "Heavy environment" is named Crusher.
- The initial release uses only gravity, existing horizontal wind, terrain generation, and presentation; atmosphere/drag, vertical/layered winds, and changing gravity are deferred.
- Turnwind's next wind is visible at the start of the turn, before the active player commits aiming or movement.
- Bowl is a terrain-generation profile, not a new map-size or gameplay-bounds rule.

## Out of Scope

- Trajectory/impact aids, ballistic auto-correction, atmospheric drag, altitude wind layers, weather hazards, gravity changes during a shot, or environment-specific weapons.
- Additional presets such as Thin World or Crosswind, team/objective modes, persistent configuration, online synchronization, and a full art/asset pipeline.

## Roadmap Alignment

- This feature advances Environmental Battlefield Identity, Match Setup environment selection, and Milestone F. Earth, Moon, Storm, Crusher, Turnwind, and Bowl become the initial curated worlds; later experimental concepts remain roadmap work.

## Constitution Compliance

- The feature composes existing understandable systems—gravity, wind, terrain, and visual presentation—into varied play rather than adding opaque simulation. Presentation remains non-authoritative, all random world effects remain seeded, and no trajectory aid diminishes player discovery.
