# Feature Specification: Basic Player Aiming Controls

**Feature Branch**: `20260905-212459-basic-player-aiming-controls`  
**Created**: 2026-09-05  
**Status**: Draft  
**Input**: User description: "Create the next Azimuth feature specification for Basic Player Aiming Controls."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Configure a Deliberate Shot (Priority: P1)

A player can see Player One's current azimuth, elevation, and launch velocity, adjust each with
documented keyboard controls, and see the red placeholder tank's turret and barrel communicate the
selected direction before firing.

**Why this priority**: This replaces the fixed development launch with the first genuine, learnable
artillery decision and makes the selected shot understandable before its result is observed.

**Independent Test**: Start the game, use each fine and coarse adjustment control, and verify that
the visible values and Player One's firing indicator change while Player Two remains a visible
reference target.

**Acceptance Scenarios**:

1. **Given** no projectile is resolving, **When** the player presses Q or E, **Then** Player One's
   azimuth respectively decreases or increases by the documented fine amount, wraps in the
   documented 0 through less-than-360-degree range, and rotates its visible horizontal firing
   direction accordingly.
2. **Given** no projectile is resolving, **When** the player presses R or F, **Then** Player One's
   elevation respectively increases or decreases by the documented fine amount, remains within the
   documented player-facing bounds, and visibly raises or lowers its barrel/firing indicator.
3. **Given** no projectile is resolving, **When** the player presses T or G, **Then** Player One's
   launch velocity respectively increases or decreases by the documented fine amount and remains
   within the documented player-facing bounds.
4. **Given** the player holds Shift with any aiming-adjustment key, **When** that key is pressed,
   **Then** the corresponding documented coarse amount is applied in the same direction without
   exceeding a bound or changing a different aiming value.
5. **Given** Player One is the active tank, **When** the aiming display is visible, **Then** it
   identifies the active tank and shows current azimuth, elevation, and launch velocity with units
   and the available adjustment and fire controls.

---

### User Story 2 - Fire the Selected Shot (Priority: P2)

A player presses Space to fire Player One's configured shot. The projectile visibly starts at the
same barrel-end reference shown by the tank, then follows the existing deterministic projectile,
impact, boom, and terrain-deformation behaviour using the selected values.

**Why this priority**: A displayed aim setting only becomes gameplay when it controls the exact
initial conditions of the shot the player sees.

**Independent Test**: Choose known aim values, fire, and compare the projectile's initial position
and velocity with the values derived from those settings; repeat with a changed azimuth, elevation,
and velocity.

**Acceptance Scenarios**:

1. **Given** Player One has configured an aim and no projectile is active, **When** the player
   presses Space, **Then** exactly one projectile launches from Player One's current visible firing
   origin with the current azimuth, elevation, and launch velocity.
2. **Given** only azimuth differs between two otherwise equal shots, **When** they are launched,
   **Then** their initial horizontal directions differ according to the established clockwise
   positive-azimuth convention.
3. **Given** elevation is increased while azimuth and velocity are unchanged, **When** the shot is
   launched, **Then** its initial upward velocity increases according to the established elevation
   convention.
4. **Given** launch velocity is increased while both angles are unchanged, **When** the shot is
   launched, **Then** its initial velocity magnitude increases by the selected amount without
   changing its direction.
5. **Given** a projectile reaches terrain, **When** it resolves, **Then** the existing impact
   marker, boom, and authoritative crater behaviour remain available for that aimed shot.

---

### User Story 3 - Bracket a Target With Repeated Shots (Priority: P3)

After a shot finishes resolving, the same player can retain the previous settings, make a focused
change, fire again, and observe the changed result against the currently deformed battlefield.

**Why this priority**: Repeating a shot with an intentional correction is the first core artillery
skill loop; it must be available before turns, damage, or weapon choice are introduced.

**Independent Test**: Fire a shot, wait for its projectile to terminate and its impact feedback to
appear, change one parameter, fire again, and confirm both shots use their respective values and
the second evaluates the terrain left by the first.

**Acceptance Scenarios**:

1. **Given** a projectile is active, **When** the player uses an aiming key or Space, **Then** the
   aim state and active projectile remain unchanged and no second projectile is created.
2. **Given** a projectile has ended through terrain impact or the existing non-impact limit,
   **When** the player next uses an aiming key, **Then** controls are available again and retain the
   values selected for the prior shot until changed.
3. **Given** the player fires a later shot after an earlier terrain impact, **When** it descends
   through the battlefield, **Then** it uses the existing deformed terrain for collision and
   produces the established feedback if it impacts.

### Edge Cases

- Azimuth below zero or at/above 360 degrees wraps to the established normalized range; it never
  creates an alternative horizontal convention.
- Elevation and launch velocity inputs at their respective limits clamp at that limit; normal
  controls cannot introduce non-finite, negative, or out-of-range aiming values.
- A fine or coarse adjustment that would cross a limit lands exactly at the limit.
- Simultaneous opposite adjustment keys produce no change for that parameter in that update.
- All aiming and fire inputs are ignored while the single projectile is active, including while its
  visual effect continues after a terrain impact; controls resume once projectile resolution ends.
- A projectile ending outside the useful volume still unlocks aiming and firing but does not create
  an invented terrain impact, boom, or crater.
- Changing elevation changes the shown barrel-end firing origin along with its visible barrel so a
  launched projectile never visibly starts from a stale level-only marker.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The game MUST introduce one small authoritative aiming state for the deterministically
  selected active tank, Player One. It MUST contain normalized azimuth, bounded elevation, and
  bounded launch velocity; rendering and the aiming display MUST derive from it and MUST NOT own a
  second copy of those values. Player Two remains visible but is not controllable in this feature.
- **FR-002**: Aiming and launch calculations MUST preserve the documented world convention: Y is up;
  azimuth 0 degrees is negative Z; positive azimuth turns clockwise from above; and elevation is
  measured upward from horizontal. The existing projectile shot-parameter conversion remains the
  sole conversion from azimuth, elevation, and launch velocity into launch direction and velocity.
- **FR-003**: Initial Player One azimuth MUST be derived from its established initial turret
  direction and normalized to 0 through less than 360 degrees. Normal azimuth adjustments MUST
  wrap in that range rather than clamp or use a second angular convention.
- **FR-004**: The initial player-facing elevation range MUST be 5 through 85 degrees inclusive, and
  the initial launch-velocity range MUST be 8 through 30 abstract units per second inclusive.
  These named gameplay limits and all initial adjustment amounts MUST be straightforward to tune.
- **FR-005**: The initial fine keyboard adjustments MUST be 1 degree for azimuth and elevation and
  0.5 abstract units per second for launch velocity. Holding Shift with the same control MUST use
  coarse adjustments of 5 degrees and 2.5 abstract units per second respectively. The controls
  MUST be Q/E for azimuth down/up, R/F for elevation up/down, T/G for velocity up/down, and Space
  to fire; these keys intentionally avoid the existing camera's WASD and arrow controls.
- **FR-006**: Each input update MUST apply at most one net fine or coarse adjustment to each
  parameter, preserve other parameters, and clamp bounded values. The implementation MUST not add
  a configurable binding, command, input-acceleration, or generic input framework.
- **FR-007**: The active tank's placeholder geometry MUST visibly derive its horizontal turret
  direction from aiming azimuth and its barrel/firing indicator pitch from aiming elevation.
  Its gameplay firing origin MUST be the same current barrel-end reference represented on screen;
  no independent presentation launch point or level-only stale origin is permitted.
- **FR-008**: The game MUST show a deliberately minimal readable aiming display while the game is
  running. It MUST identify Player One as controllable and show current azimuth and elevation in
  degrees plus current launch velocity in abstract units per second. It MUST also show the
  documented keys, update whenever aiming changes, and remain presentation-only.
- **FR-009**: When no projectile is active, Space MUST create the existing single projectile using
  Player One's current authoritative firing origin and its current aiming state. This supersedes
  both the fixed Space development launch and the separate I inspection-shot path; there MUST be
  one coherent player fire path, not parallel development and aiming launches.
- **FR-010**: While the current projectile remains active, the game MUST ignore aiming adjustments
  and fire requests. It MUST restore these inputs immediately once fixed-step projectile resolution
  ends, whether through terrain impact or established out-of-bounds/lifetime termination. No turn,
  player switch, queue, or simultaneous projectile support may be added.
- **FR-011**: Existing deterministic projectile fixed steps, gravity, terrain collision, impact
  result, marker, boom, terrain deformation, and later-shot interaction with deformed terrain MUST
  remain authoritative and unchanged except that launch inputs now come from aiming state.
- **FR-012**: Deterministic automated tests MUST cover azimuth wrapping and expected increments;
  elevation and launch-velocity lower/upper bounds; fine and coarse adjustment magnitudes;
  unchanged parameters producing identical initial shot conditions; azimuth changing horizontal
  launch direction; increased elevation increasing initial upward velocity; increased velocity
  increasing launch-vector magnitude; firing using current rather than stale development values;
  and rejection of aiming/fire input while a projectile is active. Renderer screenshot or
  renderer-internal text tests are not required.
- **FR-013**: Completion MUST document the player controls, initial ranges, adjustment amounts,
  units, active-tank choice, and projectile-flight input rule in the relevant gameplay and project
  documentation. It MUST update only roadmap items whose demonstrated acceptance criteria are met:
  the near-term basic-aiming milestone; azimuth, elevation, velocity, fire, visible values, fine
  control, coarse control; azimuth/elevation/power feedback; bracketing and previous-shot learning;
  understandable power/elevation effects; and aiming-HUD plus usability items for keyboard controls,
  clear feedback, and readable values. It MUST NOT mark weapon selection, player switching,
  turn-system, movement, damage, health, match, wind, atmosphere, or final-HUD items complete.
- **FR-014**: Completion MUST leave the workspace buildable, relevant tests passing, formatting
  passing, and Clippy warning-free without unjustified exceptions.

### Key Entities

- **Aiming state**: The active tank's authoritative normalized azimuth, bounded elevation, and
  bounded launch velocity, retained between shots.
- **Aiming limits and adjustment amounts**: The explicit, tuneable player-facing bounds plus fine
  and coarse changes that keep controls both precise and fast enough for correction.
- **Active tank**: Player One's existing tank, the sole receiver of aiming and fire input until a
  later turn-loop feature establishes ownership changes.
- **Derived firing representation**: The active tank's turret, pitched barrel, and barrel-end
  firing origin, all derived from authoritative aim and used consistently for presentation and
  launch.
- **Aim display**: Minimal presentation of the active tank's current values and controls; it has no
  gameplay authority.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: From a fresh launch, a player can identify Player One, read all three current aiming
  values and their controls, make a fine adjustment to each, and fire within 30 seconds.
- **SC-002**: In deterministic adjustment tests, 100% of fine inputs change the appropriate value
  by 1 degree or 0.5 units per second, and 100% of Shift-modified inputs change it by 5 degrees or
  2.5 units per second, unless the documented limit is reached.
- **SC-003**: In deterministic boundary tests, 100% of normal inputs keep elevation in 5 through
  85 degrees and launch velocity in 8 through 30 abstract units per second; azimuth always remains
  normalized to 0 through less than 360 degrees.
- **SC-004**: In deterministic launch tests, identical aim yields identical initial projectile
  conditions; changing azimuth changes horizontal direction, increasing elevation increases the
  upward initial component, and increasing velocity increases initial speed in 100% of cases.
- **SC-005**: In manual verification, the visible turret/barrel and launch point agree with the
  displayed settings for each of at least 10 varied shots, and no projectile appears to begin away
  from the visible barrel-end reference.
- **SC-006**: A player can complete two shots separated by an adjustment after the first has
  resolved; during each active flight, 100% of attempted adjustment and fire inputs leave both the
  configured aim and the single active projectile unchanged.
- **SC-007**: Existing terrain impact, boom, and crater feedback remains visible for aimed shots,
  and a second aimed terrain-impact shot uses the battlefield deformation from the first.
- **SC-008**: Workspace build, relevant tests, formatting, and lint validation complete without new
  warnings.

## Assumptions

- Player One is the deterministic active tank because existing launch code already selects it and
  no turn or current-player system exists. Player Two is intentionally a stationary reference.
- Fixed key presses plus Shift coarse adjustment are sufficient for the first interaction: they
  provide 1-degree/0.5-unit precision and five-times-faster coarse correction without building a
  held-key acceleration or rebinding system.
- The 5 through 85 degree and 8 through 30 units-per-second limits avoid level, vertical, weak, and
  excessive player shots while leaving meaningful low, high, short, and long arcs; they are initial
  gameplay tuning values, not world-model constraints.
- The existing single-projectile lifecycle is the clearest point at which to disable controls.
  Resolution ends when flight ends; a presentation-only boom does not itself block the next aim.
- A simple corner text display is sufficient feedback for this vertical slice. Final HUD appearance,
  turn state, weapon selection, and environmental information remain later work.

## Dependencies

- `docs/specs/20260905-123829-placeholder-tanks/`, which provides Player One and Player Two,
  placeholder tank geometry, player identity, turret direction, and a firing-origin convention.
- `docs/specs/20260905-130239-projectile-ballistics/`, which provides deterministic shot
  parameters, the shared angular-to-launch-vector conversion, gravity, and single-flight lifecycle.
- `docs/specs/20260905-143022-projectile-terrain-impact/`,
  `docs/specs/20260905-181606-visible-projectile-explosion/`, and
  `docs/specs/20260905-183525-first-crater-terrain-deformation/`, which provide authoritative
  impact, presentation feedback, mutable terrain, and subsequent-shot terrain interaction.
- `docs/world-conventions.md`, `docs/projectile-model.md`, `docs/roadmap.md`, ADR 0001, and the
  Azimuth Constitution.

## Out of Scope

- Turn order, automatic player switching, current-player turn UI, turn consumption, move-or-fire
  decisions, player movement, or any Player Two control.
- Damage, health, elimination, victory, match setup, weapon selection, weapon types, inventory,
  wind, drag, atmospheric behaviour, AI, or terrain-aware aiming assistance.
- Mouse aiming, click-to-aim, target selection or locking, trajectory previews, ballistic solutions,
  aim assist, projectile-follow/impact cameras, sound, controller support, configurable bindings,
  final HUD styling, graphical gauges, menus, or elaborate panels.
- Tank animation systems, polished articulation, generic weapon/input/aiming abstractions, event
  buses, new crates, or dependencies without a demonstrated current boundary.
