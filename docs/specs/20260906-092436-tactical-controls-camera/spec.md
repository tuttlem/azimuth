# Feature Specification: Tactical Controls and Camera Flow

**Feature Branch**: `20260906-092436-tactical-controls-camera`  
**Created**: 2026-09-06  
**Status**: Draft  
**Input**: User description: "Create the next Azimuth feature specification for Tactical Controls and Camera Flow."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Aim Comfortably With Tactical Controls (Priority: P1)

The current player can make clear, precise, and fast aim adjustments without repeatedly tapping
keys or accidentally panning the camera.

**Why this priority**: Comfortable aiming is the most frequent player interaction and must improve
before camera presentation can make the artillery loop feel finished.

**Independent Test**: On a choosing turn, use left/right, up/down, and -/= to adjust the current
player's retained values; hold each input and verify predictable repeated changes, bounds, and
unchanged opposing-player aim.

**Acceptance Scenarios**:

1. **Given** a player is choosing an action, **When** they press or hold Left/Right, **Then** only
   that player's azimuth decreases/increases using the established angle convention.
2. **Given** a player is choosing an action, **When** they press or hold Up/Down, **Then** only
   that player's elevation increases/decreases within existing limits.
3. **Given** a player is choosing an action, **When** they press or hold -/=, **Then** only that
   player's firing power decreases/increases within existing limits.
4. **Given** an aiming or power key remains held, **When** 300 milliseconds have elapsed after the
   initial adjustment, **Then** the adjustment repeats every 100 milliseconds until release or a
   bound is reached.

---

### User Story 2 - Understand Whose Turn It Is Through Camera Presentation (Priority: P1)

When a new turn begins, the camera transitions to a useful behind-or-near view of the active
player's tank, giving enough context to aim or move.

**Why this priority**: A turn-focused camera makes hot-seat play legible and removes the need for
manual keyboard camera panning.

**Independent Test**: Advance a turn by either ending movement or resolving a shot, then verify the
camera begins moving toward the newly current player's tank while that player can act immediately.

**Acceptance Scenarios**:

1. **Given** a new choosing turn begins, **When** the active player changes, **Then** the camera
   begins a smooth, bounded transition to an artillery-oriented view associated with that tank.
2. **Given** the active player is choosing movement, **When** the tank takes valid steps, **Then**
   the tank remains identifiable and surrounding terrain remains useful for movement choices.
3. **Given** a camera transition is still underway, **When** gameplay advances to a newer player,
   **Then** the camera abandons the stale target and moves toward the newly active player without
   queuing or delaying gameplay.

---

### User Story 3 - Read a Fired Shot at Battlefield Scale (Priority: P2)

When a player fires, the camera pulls back to a stable general battlefield view so launch,
projectile arc, terrain, and impact remain understandable.

**Why this priority**: The player should be able to read the consequence of a shot rather than
losing the projectile in a close aiming view.

**Independent Test**: Fire from either player and observe a prompt pullback while the existing
projectile remains active; verify impact/deformation and player handoff still occur on their normal
authoritative schedule.

**Acceptance Scenarios**:

1. **Given** the current player commits firing, **When** the projectile launches, **Then** the
   camera begins moving to a wider bounded view without delaying launch.
2. **Given** a projectile is active, **When** the camera pullback has not completed, **Then** the
   projectile simulation, terrain collision, deformation, and turn-resolution behaviour continue
   normally.
3. **Given** a shot terminates, **When** authoritative resolution selects the next player, **Then**
   the camera begins presenting that new player even if the cosmetic explosion remains visible.

### Edge Cases

- Releasing a held control stops repeat immediately; simultaneous opposing aim inputs make no
  ambiguous adjustment.
- Repeated input at an aiming or power limit cannot exceed that existing limit or affect another
  player.
- Retired WASD/arrow panning must not compete with the new tactical arrow controls.
- A new turn or launched shot during a camera transition replaces the presentation target directly.
- Camera interpolation may use presentation time but must never control action availability,
  simulation, terrain, turn ownership, or resolution.
- No dedicated impact sequence is required; the wider shot view need only leave the existing
  explosion and deformation understandable.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: On a choosing turn, Left/Right MUST decrease/increase azimuth; Up/Down MUST
  increase/decrease elevation; -/= MUST decrease/increase firing power. These inputs MUST reuse
  existing aim values, bounds, visual barrel feedback, and launch semantics.
- **FR-002**: An eligible aim/power press MUST apply one established fine adjustment immediately.
  A continuously held eligible input MUST repeat after 300 milliseconds at 100-millisecond
  intervals until release, action-state lock, or an existing bound stops further change.
- **FR-003**: Shift MUST retain the established coarse adjustment amount for both initial and
  repeated adjustments. No charging, analogue power, configurable binding, or input-repeat
  framework is introduced.
- **FR-004**: Existing M, I/J/K/L, Enter, and Space movement/action controls MUST retain their
  existing meanings. Keyboard camera panning MUST be retired so it cannot conflict with tactical
  controls; mouse orbit and wheel zoom remain available.
- **FR-005**: The HUD and documentation MUST accurately show current tactical controls and must
  not retain obsolete panning or aiming hints.
- **FR-006**: The camera MUST maintain a small presentation-only intent sufficient to distinguish
  active-player presentation from fired-shot presentation. It MUST derive its target from current
  authoritative turn, tank, and projectile state without duplicating or owning those states.
- **FR-007**: At startup and whenever a new choosing turn begins, the camera MUST smoothly seek a
  bounded, tuneable artillery-oriented view behind or near the active tank, with sufficient local
  terrain context for aiming and movement.
- **FR-008**: When projectile flight begins, the camera MUST smoothly seek a bounded wider
  battlefield view. It MUST seek the new active player as soon as authoritative resolution begins
  that player's choosing turn.
- **FR-009**: A newer authoritative turn or flight state MUST replace any stale camera target;
  camera transitions MUST not queue behind old transitions.
- **FR-010**: Camera state, transition completion, and presentation timing MUST NOT delay or
  determine input acceptance, firing, movement, projectile advancement, impact, terrain
  deformation, action completion, or player handoff.
- **FR-011**: Camera transitions MUST avoid abrupt per-frame snaps in normal use, remain within
  documented distance/pitch bounds, and avoid extreme zoom changes. Sophisticated obstruction
  avoidance, projectile lock-on, cinematic cuts, and impact sequences are out of scope.
- **FR-012**: Automated tests MUST cover directional mappings, repeated held input and bounds,
  active-player isolation, retired keyboard-pan conflict, camera intent reactions to turn/flight
  changes, replacement of stale presentation targets, and the invariant that gameplay progression
  is independent of camera transition completion. They MUST avoid pixel, screenshot, and exact
  camera-coordinate assertions unless testing an explicit bounded invariant.
- **FR-013**: The feature MUST update controls documentation and only roadmap items demonstrably
  completed by implementation; workspace build, relevant tests, formatting, and linting MUST pass
  without unjustified warnings.

### Key Entities

- **Tactical input state**: The current player's directional aiming/power input and its repeat
  timing, valid only while the authoritative turn permits aiming.
- **Camera presentation intent**: A presentation-local indication to show either the active
  player or a fired-shot battlefield view; it observes gameplay state and never changes it.
- **Camera target**: The current bounded desired view derived from an active tank or wider
  battlefield, replaceable whenever newer authoritative state requires it.
- **Turn and flight state**: Existing authoritative state that controls player action and
  projectile resolution; it is read-only to camera presentation.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In manual validation, a player can make a one-step directional aim adjustment and a
  ten-step held adjustment without changing camera position or consulting documentation after the
  first displayed hint.
- **SC-002**: In repeated-input tests, 100% of adjustments remain within existing azimuth,
  elevation, and power limits and change only the current player's retained aim.
- **SC-003**: In manual validation of five alternating turns, each new choosing turn visibly begins
  a transition toward the current player's tank, and each fired turn visibly begins a wider shot
  view.
- **SC-004**: In automated turn/flight tests, identical gameplay state transitions complete with
  the same player/terrain/projectile results whether camera transition progress is zero, partial,
  or complete.
- **SC-005**: All documented build, test, formatting, and lint commands succeed after the feature.

## Assumptions

- -/= are the default power decrease/increase keys because they sit beside each other and do not
  conflict with arrows, movement, or fire. Numpad aliases may be added only if trivial and do not
  complicate the primary control contract.
- A 300-millisecond initial delay and 100-millisecond repeat interval are shared initial tuning
  values: enough time for a fine press, then ten fine adjustments per second while held. Shift
  remains the existing intentional coarse mode.
- The current camera's orbit target, yaw, pitch, and distance are sufficient starting data for a
  compact presentation intent and interpolation. Existing keyboard panning can be removed from
  player controls without removing mouse orbit or wheel zoom.
- The active-player view is deliberately tuneable and need not be a rigid literal rear view if a
  nearby artillery-oriented angle provides better terrain context.
- This feature depends on existing authoritative turn phases, tank poses, projectile flight,
  movement, retained aim, and the current camera; it does not depend on damage, victory, weapons,
  wind, or a full match system.
- Full projectile following, impact cinematics, configurable bindings, controller support, and
  obstruction-aware camera navigation remain roadmap work.
