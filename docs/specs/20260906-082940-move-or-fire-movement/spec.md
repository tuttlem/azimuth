# Feature Specification: Move or Fire — Basic Tactical Movement

**Feature Branch**: `20260906-082940-move-or-fire-movement`  
**Created**: 2026-09-06  
**Status**: Draft  
**Input**: User description: "Create the next Azimuth feature specification for Move or Fire — Basic Tactical Movement."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Choose One Tactical Action (Priority: P1)

At the beginning of a turn, the current player can clearly choose either to fire from their current
position or to spend that turn repositioning. The choice consumes the turn's sole primary action.

**Why this priority**: The move-or-fire choice is the tactical decision this feature introduces;
without it, moving would not create a meaningful sacrifice.

**Independent Test**: Start a turn and select each action in separate runs. Verify that firing
starts the established shot resolution and prohibits movement, while choosing movement prohibits
aim adjustments and firing for that turn.

**Acceptance Scenarios**:

1. **Given** a player has begun a new turn, **When** they press Space, **Then** the game selects
   firing, launches exactly one shot using the retained aim, and rejects movement until that shot
   and its existing consequences have resolved.
2. **Given** a player has begun a new turn, **When** they press M, **Then** the game enters that
   player's movement action, identifies it in the on-screen status, and rejects aim adjustments
   and firing until the movement turn ends.
3. **Given** a projectile is resolving, **When** either action-selection or movement input is
   requested, **Then** it has no effect on the resolving action, either tank, or retained aim.

---

### User Story 2 - Reposition Deliberately Within an Allowance (Priority: P1)

A player who chose movement can make a short, understandable series of tactical repositioning
steps over the battlefield, see how much movement remains, and end the turn when satisfied.

**Why this priority**: Position becomes a meaningful trade-off only when movement is finite,
controlled, and visibly committed as a turn action.

**Independent Test**: Select movement for Player One, make valid steps, verify that only Player
One changes position and each accepted step reduces the displayed allowance; then end movement and
verify Player Two's turn begins.

**Acceptance Scenarios**:

1. **Given** a player has selected movement, **When** they press I, J, K, or L, **Then** the active
   tank attempts one fixed one-world-unit step respectively toward negative Z, negative X, positive
   Z, or positive X.
2. **Given** a newly selected movement action, **When** the player makes accepted steps, **Then**
   the action starts with six steps available and each accepted step consumes exactly one step.
3. **Given** a player has movement remaining, **When** they press Enter, **Then** unused movement
   is forfeited, that player's final position is retained, and the other player's ready turn begins.
4. **Given** a player has used all six accepted steps, **When** the final step completes, **Then**
   the movement action ends and the other player's ready turn begins without requiring another
   action.

---

### User Story 3 - Let Terrain Shape Repositioning (Priority: P2)

A moving tank follows the battlefield's current surface, including craters, but cannot cross a
location that is too steep or outside the battlefield.

**Why this priority**: Reusing the changed battlefield makes earlier shots affect future tactical
choices without special crater rules.

**Independent Test**: Move over ordinary and cratered ground, then attempt an out-of-bounds and a
too-steep step. Confirm valid positions follow current ground height, while rejected attempts leave
position and allowance unchanged.

**Acceptance Scenarios**:

1. **Given** a valid destination on ordinary or previously deformed terrain, **When** the active
   tank takes a step, **Then** its base ends exactly on the current terrain surface at that
   destination.
2. **Given** a requested destination rises or falls by no more than 0.75 world units over the
   one-unit step, **When** the player requests it, **Then** the step is accepted.
3. **Given** a requested destination rises or falls by more than 0.75 world units over the
   one-unit step, **When** the player requests it, **Then** the step is rejected with clear status
   feedback and neither position nor allowance changes.
4. **Given** a requested destination lies beyond the battlefield boundary or the active tank has
   no movement remaining, **When** the player requests it, **Then** the step is rejected cleanly
   and neither position nor allowance changes.

---

### User Story 4 - Retain a Player's Firing Knowledge After Moving (Priority: P3)

Each player keeps their personal azimuth, elevation, and power across a movement turn. A later
shot therefore starts from the new tank position while preserving the player's prior settings.

**Why this priority**: This turns repositioning into a real artillery trade-off: it disrupts a
known firing solution through position, rather than arbitrarily deleting the player's knowledge.

**Independent Test**: Record Player One's aim, complete a movement turn, return to Player One's
next firing turn, and compare the retained settings and the changed firing origin.

**Acceptance Scenarios**:

1. **Given** a player has non-default aiming values, **When** that player completes a movement
   turn, **Then** their azimuth, elevation, and power remain unchanged on their next turn.
2. **Given** a player has moved since their last shot, **When** they later choose firing, **Then**
   the projectile launches from their new tank position using their retained aiming values.
3. **Given** a tank completes an accepted movement step, **When** the final position is shown,
   **Then** the tank body faces the direction of its most recent accepted step while its retained
   aiming direction remains independent.

### Edge Cases

- Simultaneous or opposing movement keys do not produce an ambiguous diagonal or multiple step;
  they result in at most one deterministic requested step.
- A rejected step never moves a tank below or above terrain, changes body orientation, or spends
  movement allowance.
- A tank that has not accepted a step retains its prior body orientation when ending movement.
- Movement has no delayed gameplay consequence: ending early advances immediately and does not
  wait for presentation effects.
- Terrain may change below a stationary tank after a later projectile impact; updating stationary
  tank grounding remains separate roadmap work, while every deliberately moved tank must use the
  current terrain at its destination.
- The feature does not introduce tank collision; each player operates only their own tank on their
  own turn.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: At every ready turn, the game MUST present the current player with the two explicit
  primary choices: press M to begin movement or Space to commit to firing.
- **FR-002**: The game MUST represent the selected action in authoritative turn state. A ready
  turn, movement action, and fired-action resolution MUST be distinguishable without relying on
  visible controls or renderer state.
- **FR-003**: Selecting firing MUST preserve the existing aim, projectile, terrain-impact,
  explosion, deformation, and post-resolution handoff behaviour. Movement MUST be unavailable
  from firing selection until the next ready turn.
- **FR-004**: Selecting movement MUST make firing and aim adjustment unavailable until movement
  ends. A projectile-resolving turn MUST reject all selection, movement, aim, and fire requests.
- **FR-005**: A movement action MUST begin with a shared allowance of six one-world-unit steps for
  the current player. The allowance MUST reset to six only when that player's next movement action
  is selected; it is not a persistent tank statistic.
- **FR-006**: Movement MUST use discrete, one-world-unit cardinal steps requested by I/J/K/L.
  This is the initial movement-model decision: it is deliberately turn-based and deterministic,
  avoids render-frame timing, and keeps the interaction a tactical placement choice rather than
  vehicle driving.
- **FR-007**: For each requested step, the game MUST determine validity from the authoritative
  current terrain and battlefield boundary before changing tank state or allowance.
- **FR-008**: A valid destination MUST be within the battlefield and have an absolute elevation
  change of at most 0.75 world units from the tank's current position over the requested one-unit
  horizontal step. This is the configurable initial maximum passable slope.
- **FR-009**: An accepted step MUST place the active tank's base at the authoritative terrain
  height of its destination, consume exactly one remaining step, and set the body-facing direction
  to that step's direction.
- **FR-010**: An invalid request due to bounds, slope, or exhausted allowance MUST leave the
  active tank's complete pose and remaining allowance unchanged, and MUST provide concise status
  feedback that identifies the rejection reason.
- **FR-011**: The player MUST be able to end movement early with Enter. The final accepted pose is
  authoritative, unused allowance is forfeited, and turn ownership advances once to the other
  player's ready turn. Exhausting the allowance MUST produce the same one-time handoff.
- **FR-012**: The existing two players MUST retain independent tank positions, body orientations,
  and aiming values. A movement turn may change only the current player's tank pose and must not
  reset either player's azimuth, elevation, or power.
- **FR-013**: The on-screen turn feedback MUST identify the current player, whether the player is
  choosing, moving, or resolving fire, the remaining movement steps while moving, and the relevant
  controls for the current state. It MUST explain how to end movement and how to fire.
- **FR-014**: Given the same terrain state, tank pose, allowance, and ordered movement requests,
  movement acceptance, resulting pose, orientation, allowance, and player handoff MUST be
  identical.
- **FR-015**: The feature MUST document the movement controls, fixed-step movement-model decision,
  allowance, slope restriction, and move-or-fire rule, and update only roadmap items whose
  acceptance criteria are demonstrated by the completed implementation.
- **FR-016**: Automated gameplay tests MUST cover initial and reset allowance, accepted and
  rejected steps, boundary and slope handling, current-terrain grounding including deformation,
  orientation, early and exhausted-budget completion, per-player ownership, retained aim, action
  exclusivity, preserved firing resolution, and identical-input determinism. They MUST exercise
  gameplay state rather than keyboard events, visible text, or render-frame timing.
- **FR-017**: Before the feature is considered complete, the workspace build, relevant automated
  tests, formatting, and lint checks MUST pass without unjustified warnings; affected controls and
  movement behaviour MUST be documented.

### Key Entities

- **Turn action state**: The authoritative current player's state: choosing an action, performing
  movement with remaining steps, or resolving a fired shot.
- **Movement allowance**: The six-step, per-movement-action budget that limits repositioning and is
  discarded when movement ends.
- **Movement request**: One ordered cardinal step request, whose validity and result are determined
  from the current terrain and active tank state.
- **Tank pose**: A player's grounded position and body-facing direction; firing aim remains a
  separate retained player value.
- **Terrain passability**: The local elevation change between a tank's current position and a
  requested destination, compared with the shared passable-slope limit.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In manual verification, both players can complete ten alternating turns in sequence,
  choosing either action each turn, with exactly one primary action per turn and no stuck handoff.
- **SC-002**: A movement turn visibly reports an initial allowance of six steps; across six valid
  requests it accepts exactly six one-unit steps and cannot accept a seventh.
- **SC-003**: Across valid normal-terrain and deformed-terrain movement checks, 100% of accepted
  destinations place the tank base on the current battlefield surface; rejected bound, steepness,
  and exhausted-budget requests leave pose and allowance unchanged.
- **SC-004**: In deterministic automated tests, two runs using the same terrain, initial tank
  pose, and ordered requests produce identical final poses, allowances, and current players.
- **SC-005**: A player who moves can, on their following firing turn, use unchanged stored aiming
  values and launch from the moved position in every tested case.

## Manual Acceptance

1. The game starts and clearly identifies the current player and action state.
2. Each new turn clearly offers move or fire; selecting fire preserves the existing artillery
   interaction and excludes movement for that turn.
3. Selecting movement allows only the active tank to reposition, displays allowance, consumes it
   only for accepted steps, and excludes firing and aim changes for that turn.
4. A tank follows normal and previously cratered terrain at every accepted destination; steep
   ground, battlefield edges, and exhausted allowance prevent a step without corrupting state.
5. The player can end movement before the allowance is spent, after which the next player can make
   their own independent move-or-fire choice.
6. Stored aim survives a movement turn; a later shot launches from the moved tank, so the prior
   world-space firing solution has naturally changed.
7. Repeated alternating turns remain stable, and existing projectile, impact, explosion, and
   deformation behaviour continues to resolve fully before a firing-turn handoff.

## Assumptions

- The initial fixed cardinal-step model is preferred over continuous or tile-grid movement because
  the existing battlefield is a compact bounded height surface, current input is keyboard-based,
  and one request per step makes player intent, budget use, and deterministic testing clear.
- A one-unit step and a six-step allowance are shared development defaults. They are intentionally
  small, readable gameplay constants rather than player attributes; later playtesting may tune
  their values without changing the move-or-fire rule.
- The 0.75 rise/run limit is an initial gameplay-oriented passability threshold. It allows the
  authored relief and sufficiently gentle crater paths while allowing steep crater walls and sharp
  terrain changes to block progress; no traction, sliding, or vehicle simulation is implied.
- I/J/K/L are reserved for movement because existing camera controls use WASD and arrows, existing
  aiming controls use Q/E, F/R, G/T, and Space already fires. Enter is the explicit early-end
  control.
- This feature depends on the existing bounded mutable terrain, terrain-height queries, two-player
  alternating turn state, independent aiming state, and firing-resolution ordering.
- Damage, victory, tank settling after later terrain deformation, movement pathfinding, collision,
  vehicle physics, and firing after movement remain out of scope. The roadmap must retain any
  nonessential discoveries, including whether initial movement produces genuinely useful cover or
  escape tactics.
