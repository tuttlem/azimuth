# Feature Specification: Graphical Tactical HUD

**Feature Branch**: `20260906-190352-graphical-tactical-hud`  
**Created**: 2026-09-06  
**Status**: Draft  
**Input**: User description: "Create the next Azimuth feature specification for Graphical Tactical HUD."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Read the Current Turn at a Glance (Priority: P1)

As a local player, I can identify the active player, their current action state, and both players'
condition without reading a development-style block of prose, so I can return my attention to the
battlefield quickly.

**Why this priority**: Turn ownership, action state, and survival are the minimum information
needed to play the existing duel correctly.

**Independent Test**: Start a match, change turns, resolve a damaging shot, and finish a match;
the tactical frame continues to identify the active player, both health values, elimination, and
the winner or draw without relying on the retired text dump.

**Acceptance Scenarios**:

1. **Given** an in-progress choosing turn, **When** Player One or Player Two becomes active,
   **Then** the tactical frame makes that player and the action-selection state immediately
   distinguishable using the established player identity colours and concise wording.
2. **Given** either tank has taken damage or been eliminated, **When** the HUD is shown,
   **Then** both players' health is shown as a proportional condition cue and a precise current
   value, with eliminated players clearly inert.
3. **Given** the match ends with a survivor or no survivors, **When** gameplay enters its finished
   state, **Then** the frame clearly presents the winner or draw and no ordinary action hint is
   presented as available.

---

### User Story 2 - Aim and Compensate Deliberately (Priority: P1)

As the active player, I can read my exact azimuth, elevation, power, and the current wind without
the HUD obscuring the battlefield, so I can make and remember deliberate artillery adjustments.

**Why this priority**: These values form the existing artillery skill loop; graphical framing must
improve their readability without replacing their useful precision.

**Independent Test**: On a choosing turn, adjust each aim value and observe the matching numeric
display; compare opposite world wind conditions and confirm their graphical indications are
opposite while the numeric strength remains visible.

**Acceptance Scenarios**:

1. **Given** a player is choosing an action or preparing a shot, **When** aim values change,
   **Then** azimuth, elevation, and power update with the authoritative player-facing values and
   retain their numeric precision.
2. **Given** a non-calm wind condition, **When** the tactical frame is shown, **Then** it displays
   a compact graphical world-axis direction indicator and its numeric strength; reversing the
   world wind reverses the indicator.
3. **Given** calm wind, **When** the tactical frame is shown, **Then** its wind presentation is
   visibly neutral and does not imply a direction.

---

### User Story 3 - Move Without Losing Tactical Context (Priority: P2)

As a player who chose movement, I can see remaining movement and only the controls relevant to the
movement action, so I can reposition deliberately without a persistent control manual covering the
screen.

**Why this priority**: Movement is the current alternative to firing and needs clear allowance
feedback, but should not clutter an aiming turn.

**Independent Test**: Enter movement, make valid and invalid steps, spend or forfeit allowance,
and return to a choosing turn; the movement readout appears only while relevant and contextual
hints change with the action state.

**Acceptance Scenarios**:

1. **Given** an active movement action, **When** the player spends or retains steps,
   **Then** the remaining allowance is prominent and any existing blocked-movement feedback is
   understandable.
2. **Given** a choosing/aiming turn, **When** movement is not active, **Then** movement allowance
   is absent or clearly subdued.
3. **Given** projectile flight, settling, or another resolving state, **When** the HUD is shown,
   **Then** no irrelevant aiming, movement, or action-selection hint dominates the frame.

---

### User Story 4 - Keep the Battlefield Primary (Priority: P2)

As a player using a normal resizable game window, I see a stable, compact tactical interface that
does not fight the tactical camera, obstruct common shot paths, or delay gameplay.

**Why this priority**: The visual upgrade only succeeds if it improves reading the game while
preserving its battlefield-first presentation and independent camera/gameplay timing.

**Independent Test**: Resize the running window through ordinary desktop sizes, alternate turns,
move, fire, and wait for a shot to resolve; the frame remains legible, gameplay and camera timing
continue normally, and no obsolete text HUD remains.

**Acceptance Scenarios**:

1. **Given** ordinary desktop window resizing, **When** the HUD is redrawn, **Then** its grouped
   information remains visible and does not overlap its own primary panels.
2. **Given** a camera transition, projectile flight, explosion, or tank settling, **When** the HUD
   updates, **Then** it observes the resulting state but cannot delay or change authoritative
   progression.
3. **Given** the graphical frame is active, **When** the scene is viewed during typical aiming,
   flight, and impact moments, **Then** the central battlefield remains substantially unobscured.

### Edge Cases

- A tank at zero health remains clearly eliminated even if it was the most recently active player.
- Mutual elimination displays a draw without selecting a false active player or offering controls.
- Calm wind uses a neutral graphical state; diagonal and cardinal wind vectors remain legible on
  the same world-axis indicator.
- A movement action with zero remaining steps, a rejected terrain step, or an early finish never
  shows stale allowance or an unavailable control as usable.
- A new player turn or match completion occurring while the camera is still transitioning updates
  HUD information immediately from gameplay state.
- At a smaller ordinary desktop window size, low-priority contextual hints may be reduced before
  essential player, health, turn, aiming, wind, and active-movement information is obscured.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The game MUST replace the temporary text-heavy gameplay HUD with one coherent,
  compact graphical tactical frame; redundant legacy HUD text MUST NOT remain visible.
- **FR-002**: The frame MUST show the active player's name, established player identity colour,
  and readable authoritative turn/action state during an in-progress match.
- **FR-003**: The frame MUST show both players' current health as precise numeric values and a
  proportional graphical condition cue.
- **FR-004**: The frame MUST make a zero-health player visibly eliminated and distinguish that
  state from a merely damaged player.
- **FR-005**: During an in-progress firing-capable turn, the frame MUST show the active player's
  current azimuth, elevation, and power using the existing player-facing units and useful numeric
  precision.
- **FR-006**: The frame MUST show wind strength numerically and a compact graphical world-axis
  wind-direction indicator derived from the authoritative horizontal wind vector.
- **FR-007**: The world-axis wind indicator MUST use one documented, camera-independent reference
  frame, show a neutral state for calm wind, and reverse direction for opposite wind vectors.
- **FR-008**: The frame MAY additionally show a concise shot-relative wind cue only if it remains
  visually subordinate to the required world-axis indication and does not obscure precise values.
- **FR-009**: During an active movement action, the frame MUST show remaining movement allowance
  and the applicable action state; outside active movement it MUST not present the allowance as a
  primary active value.
- **FR-010**: The frame MUST present concise contextual control hints matching the actual current
  controls: action selection while choosing, aim/fire while relevant, movement/end-movement while
  moving, and no dominant action hints while resolving or finished.
- **FR-011**: On match completion, the frame MUST clearly present winner or draw and suppress
  ordinary action controls.
- **FR-012**: HUD layout MUST use stable screen grouping and preserve a substantially unobstructed
  central battlefield under normal desktop window resizing.
- **FR-013**: HUD presentation MUST be derived from existing authoritative match, turn, tank,
  aiming, movement, and wind state; it MUST NOT own duplicate gameplay values or mutate gameplay.
- **FR-014**: HUD updates, layout, and any visual polish MUST NOT gate or alter input, projectile
  simulation, terrain deformation, damage, tank settling, turn advancement, or camera timing.
- **FR-015**: The feature MUST use a restrained, internally consistent tactical visual language
  with grouped panels, readable contrast, established player colours, and no required external art
  assets.
- **FR-016**: The feature MUST keep mouse, controller, keybinding configuration, weapon selection,
  environment selection, trajectory prediction, minimaps, menus, and HUD animation systems out of
  scope.

### Key Entities *(include if feature involves data)*

- **Tactical HUD presentation**: Read-only screen representation of the current match and active
  player state, grouped into match/player, shot/environment, movement, and contextual-control
  information.
- **Player condition display**: Presentation of one existing player's identity, health, and
  elimination status; it has no independent health or survival state.
- **Aim display**: Presentation of the active player's existing azimuth, elevation, and power.
- **Wind display**: Presentation of the existing match-constant wind's strength and horizontal
  direction in the documented world-axis frame.
- **Contextual hint set**: A read-only, state-dependent concise description of controls currently
  useful to the player.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In every in-progress turn phase, a player can identify the active player and current
  action state from one stable tactical-frame location without relying on the retired text dump.
- **SC-002**: The frame presents both players' health, active aim values, and wind strength from
  the current authoritative state with no stale values after a normal game-state update.
- **SC-003**: In manual checks using cardinal and opposite wind conditions, the world-axis graphic
  communicates the correct direction in all tested cases and a calm condition communicates no
  direction.
- **SC-004**: During movement, the displayed allowance equals the current authoritative remaining
  allowance after every accepted step and is not shown as an active value on non-movement turns.
- **SC-005**: A complete match through victory or draw retains a clear result and accepts no
  ordinary action through the HUD presentation.
- **SC-006**: Across normal desktop window resizing, all essential frame groups remain visible and
  the central battlefield remains free of persistent primary HUD panels.
- **SC-007**: Existing automated gameplay tests continue to demonstrate identical projectile,
  movement, damage, settling, turn, wind, and camera-authority behaviour with the HUD present.

## Assumptions

- The existing two-player local match, player colours, health maximum, aim units, movement rules,
  constant-per-match wind, and keyboard controls remain authoritative and unchanged.
- A compact graphical world-axis wind indicator is the required first reference frame; a
  shot-relative cue is optional only when it remains clear and small.
- Existing engine-native UI primitives can provide panels, text, simple bars, borders, and an arrow
  without new assets or dependencies.
- The initial graphical HUD is intentionally static apart from authoritative value updates; a full
  visual theme, animation system, menus, and settings are later work.
- The current tactical camera already protects the central battlefield enough that corner-anchored
  player/match and shot/environment groups are a reasonable starting layout, subject to manual
  readability checks.

## Dependencies

- Existing read-only turn, match, tank, aiming, movement, and wind state.
- Existing player identity colours and presentation-only camera policy.
- The roadmap's Graphical Tactical HUD and graphical world-axis wind-indicator work.

## Out of Scope

- Mouse or controller controls, configurable keybindings, weapons or inventory, gravity and
  environment selection, trajectory prediction, minimaps, target markers, menus, settings,
  restart/rematch, AI, audio, elaborate animation, or a final visual theme.
