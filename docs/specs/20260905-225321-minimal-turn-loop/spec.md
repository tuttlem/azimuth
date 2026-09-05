# Feature Specification: Minimal Turn Loop

**Feature Branch**: `20260905-225321-minimal-turn-loop`  
**Created**: 2026-09-05  
**Status**: Draft  
**Input**: User description: "Create the next Azimuth feature specification for Minimal Turn Loop."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Take One Alternating Turn (Priority: P1)

A local player can see whose turn it is, adjust that player's tank, fire one shot, and see control
pass to the other player only after the shot's gameplay result is complete.

**Why this priority**: Alternating control after a fully resolved shot establishes Azimuth's first
actual turn-based artillery game flow.

**Independent Test**: Start a two-player match, fire Player One's shot into terrain, and verify
that Player Two becomes controllable only after the crater has been applied to the battlefield.

**Acceptance Scenarios**:

1. **Given** a newly started match, **When** the aiming display appears, **Then** it identifies
   Player One as the current player and the normal aiming controls operate on Player One's tank.
2. **Given** Player One is ready to act, **When** Player One fires a terrain-impacting shot,
   **Then** the game remains in the resolving portion of Player One's turn until projectile flight,
   impact, and terrain deformation have completed.
3. **Given** Player One's terrain-impacting shot has fully resolved, **When** the next turn starts,
   **Then** Player Two is the current player, the display identifies Player Two, and the same
   aiming and fire controls operate on Player Two's tank.

---

### User Story 2 - Preserve Each Player's Shot Setup (Priority: P2)

Each player can retain a personal azimuth, elevation, and power setup across the other player's
turn, enabling deliberate bracketing rather than rebuilding an aim from defaults every time.

**Why this priority**: Remembering the prior shot is central to the learning and correction loop
of artillery play, and makes alternating turns meaningful from the first cycle.

**Independent Test**: Configure and fire Player One with non-default values, configure and fire
Player Two with different values, then confirm that Player One's original values return unchanged
when Player One's next turn begins.

**Acceptance Scenarios**:

1. **Given** Player One has changed one or more aiming values, **When** Player One fires and the
   turn advances to Player Two, **Then** Player One's values are retained without being reset.
2. **Given** Player Two changes one or more aiming values, **When** Player Two fires and the turn
   returns to Player One, **Then** Player One's display, barrel, and next shot use Player One's
   retained values rather than Player Two's values.
3. **Given** either player is current, **When** that player adjusts an aiming value, **Then** the
   other player's retained aiming values do not change.

---

### User Story 3 - Complete Every Fired Action Reliably (Priority: P3)

A fired shot always consumes exactly one turn, including a shot that leaves the existing useful
simulation area without hitting terrain, and a player cannot queue or alter a second action while
the first resolves.

**Why this priority**: A turn loop is only trustworthy if it cannot advance against pending
gameplay changes or get stuck when no terrain impact occurs.

**Independent Test**: For both a terrain impact and an existing non-impact termination, attempt
aiming and firing during resolution and verify no second shot starts; then verify the turn advances
once after the termination condition is reached.

**Acceptance Scenarios**:

1. **Given** a projectile is still active, **When** the local player uses an aiming control or
   presses fire, **Then** neither player's aiming state changes and no additional projectile
   launches.
2. **Given** a fired projectile leaves the existing useful simulation area or reaches its existing
   lifetime condition without terrain impact, **When** that termination is reached, **Then** the
   action resolves without an invented impact or crater and control advances to the other player.
3. **Given** identical starting gameplay state and identical ordered aiming/fire inputs, **When**
   the same sequence is played twice, **Then** the current-player sequence and turn transitions are
   identical.

### Edge Cases

- A terrain-impacting shot may create the existing temporary boom after gameplay resolution; the
  next player may begin once flight, impact, and authoritative deformation are complete, without
  waiting for that presentation-only effect to expire.
- A fire request during resolution cannot launch, queue, or consume an extra turn.
- An aiming request during resolution cannot alter the prior, current, or next player's retained
  values.
- A non-impact termination completes the turn normally and never waits for a terrain impact.
- Repeated turns alternate Player One, Player Two, Player One indefinitely; there is no skipping,
  removal, or winner selection in this feature.
- Current-player feedback must change with authoritative turn state, rather than inferring a player
  from tank colour, the last projectile, or presentation timing.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The game MUST establish a fixed deterministic order containing the two existing
  players: Player One begins a newly started match, followed by Player Two, then Player One again
  indefinitely.
- **FR-002**: The game MUST maintain authoritative gameplay state that identifies the current
  player throughout normal turn play, using the existing player identities rather than a duplicate
  player-identity system.
- **FR-003**: The game MUST maintain only the explicit turn state needed to distinguish a player
  ready to aim or fire from that player's fired action resolving. Transitioning from resolution to
  the next ready player MUST be direct and readable; no generic action, workflow, or match
  framework is required.
- **FR-004**: At the start of a ready turn, the identified current player MUST own the common local
  aiming and firing controls. The controls MUST adjust that player's tank and use that player's
  firing origin and aiming values when firing.
- **FR-005**: Each of the two existing players MUST retain independent authoritative azimuth,
  elevation, and power values. A player's values MUST remain unchanged through the other player's
  turn unless that player is current and changes them.
- **FR-006**: The initial aiming values, adjustment ranges, adjustment sizes, keys, and shot-angle
  conventions MUST remain the established ones. The active tank's visible turret, barrel, and
  firing-origin reference MUST continue to reflect the current player's authoritative values.
- **FR-007**: A ready current player firing MUST create exactly one projectile from that player's
  current firing origin and current aiming values, then enter the resolving turn state. Firing is
  the sole primary action in this feature.
- **FR-008**: While a turn is resolving, the game MUST reject or ignore all aiming and fire input.
  It MUST not alter either player's aiming state, launch or queue another projectile, or advance
  the current player before the fired action completes.
- **FR-009**: A terrain-impacting action MUST remain resolving until the existing projectile flight
  has terminated, its authoritative impact is recorded, and the resulting authoritative terrain
  deformation has been applied. The next player MUST never aim or fire against terrain that is
  still pending a change from the prior action.
- **FR-010**: A projectile that terminates through the existing useful-volume or lifetime
  boundary without a terrain impact MUST complete the resolving action and advance normally. It
  MUST not create an impact, explosion, or crater solely to complete the turn.
- **FR-011**: Once a fired action has completed according to FR-009 or FR-010, the game MUST
  advance exactly once to the other existing player and start that player's ready turn. Existing
  temporary impact/explosion presentation may continue only when it has no pending authoritative
  gameplay consequence and does not make the player transition unclear.
- **FR-012**: The minimal aiming display MUST clearly identify the authoritative current player
  and communicate whether that player is ready to aim/fire or the fired action is resolving. It
  MUST continue to show the current player's established aiming values and controls, and it MUST
  remain presentation-only.
- **FR-013**: Turn order and transitions MUST be deterministic: identical initial gameplay state
  and identical ordered player input MUST yield the same active-player sequence, firing origins,
  independent aiming values, projectile-resolution outcomes, terrain changes, and next-turn
  transitions, independently of rendering timing.
- **FR-014**: Automated gameplay-state tests MUST cover Player One's initial turn; independent
  aiming retention; current-player-only aim changes; transition into resolution on fire; no player
  change or second launch while a projectile is active; terrain deformation before turn advance;
  non-impact completion; Player One-to-Two and Player Two-to-One transitions; repeated deterministic
  alternation; and identical action sequences producing identical turn progression. Tests SHOULD
  avoid renderer, UI-text, and frame-timing internals where practical.
- **FR-015**: Completion MUST update the relevant controls and gameplay documentation and only the
  roadmap items whose acceptance criteria are demonstrated: the minimal turn-loop milestone;
  player order; starting a turn; one primary fire action; full projectile and terrain-deformation
  resolution before advancement; clear next-player transition; current-player feedback; and the
  applicable two-local-player and independent-player-state items. The broader surviving-player
  advancement item remains open because damage and elimination do not yet exist.
- **FR-016**: Completion MUST leave the workspace buildable, relevant tests passing, formatting
  passing, and Clippy warning-free without unjustified exceptions.

### Key Entities

- **Current player**: The existing Player One or Player Two identity that authoritatively owns the
  current ready turn or the action currently resolving.
- **Turn state**: The minimal gameplay state distinguishing a player ready to configure/fire one
  shot from a fired shot that is still resolving.
- **Player aiming state**: One retained azimuth, elevation, and power set per existing player;
  values are authoritative and selected by current-player state.
- **Fired action resolution**: The bounded sequence from launch through projectile termination and,
  where applicable, recorded impact and applied terrain deformation, before turn advancement.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: From a fresh launch, a local player can identify Player One as current, adjust an
  aiming value, and fire a shot within 30 seconds.
- **SC-002**: In deterministic automated tests, 100% of terrain-impacting actions keep their
  firing player current until the projectile has terminated and the corresponding crater is
  reflected in authoritative terrain state.
- **SC-003**: In deterministic automated tests, 100% of non-impact terminations advance exactly
  once to the other player without creating terrain-impact consequences or leaving the turn in a
  resolving state.
- **SC-004**: In a two-player manual cycle, each player can take one shot from that player's tank,
  and control returns to Player One after Player Two's action completes; neither player's prior
  aiming values are reset or replaced by the other player's values.
- **SC-005**: Across at least 20 alternating deterministic actions, the active-player sequence is
  exactly Player One, Player Two, Player One, Player Two in order, with no skipped, repeated, or
  extra turns.
- **SC-006**: During deterministic resolution tests, 100% of attempted aiming or firing inputs
  leave both retained aiming states, the active projectile, and current player unchanged.
- **SC-007**: Workspace build, relevant tests, formatting, and lint validation complete with no
  new warnings.

## Assumptions

- The established two tanks and their Player One/Player Two identities are the entire fixed player
  order for this feature; Player One is the deterministic first player.
- Projectile termination plus authoritative impact/deformation processing is the turn-resolution
  boundary. The existing boom is presentation-only and does not delay the next ready turn.
- One local keyboard controls whichever player owns the active turn; networking, per-player devices,
  and final hot-seat concealment are not required.
- Retaining each player's last valid aiming values is preferable to resetting them because it
  supports observable artillery correction and bracketing.
- The existing manually controlled battlefield camera remains sufficient; focusing the current
  player is deferred to the dedicated camera work.

## Dependencies

- `docs/specs/20260905-123829-placeholder-tanks/`, which established the two stable player
  identities, player-owned tank positions, and firing-origin conventions.
- `docs/specs/20260905-130239-projectile-ballistics/` and
  `docs/specs/20260905-143022-projectile-terrain-impact/`, which established deterministic flight,
  terrain impact, and non-impact termination.
- `docs/specs/20260905-181606-visible-projectile-explosion/` and
  `docs/specs/20260905-183525-first-crater-terrain-deformation/`, which established
  presentation-only impact feedback and authoritative mutable terrain deformation.
- `docs/specs/20260905-212459-basic-player-aiming-controls/`, which established the shared local
  controls, persistent Player One aiming state, player-facing bounds, and input lock during flight.
- `docs/world-conventions.md`, `docs/projectile-model.md`, `docs/roadmap.md`, ADR 0001, and the
  Azimuth Constitution.

## Out of Scope

- Movement, a move-or-fire choice, movement allowances, tank repositioning, damage, health,
  direct or splash damage, destruction, elimination, surviving-player selection, victory, or an
  end-of-match state.
- Weapons, inventories, multiple weapons, AI, configurable player counts or order, teams, match
  setup, networking, per-player input devices, controller support, or final hot-seat presentation.
- Projectile-follow or impact cameras, a full match HUD, remaining-player display, audio, wind,
  drag, atmosphere, or new environmental rules.
- Generic state-machine, command, event, rules-engine, player-management, input, networking, or
  turn-action abstractions; new crates or dependencies without a demonstrated need.
