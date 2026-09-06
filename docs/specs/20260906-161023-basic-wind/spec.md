# Feature Specification: Basic Wind — First Environmental Gameplay

**Feature Branch**: `20260906-161023-basic-wind`

**Created**: 2026-09-06

**Status**: Draft

**Input**: User description: "Create the next Azimuth feature specification for Basic Wind —
First Environmental Gameplay."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Read Wind and Learn From a Shot (Priority: P1)

Two local players can see the battlefield's constant wind direction and strength before firing,
then observe the projectile drift in that stated direction and use the result to correct a later
shot.

**Why this priority**: A player-facing, learnable cause-and-effect loop is the entire value of
basic wind. Invisible or arbitrary trajectory changes would add frustration rather than artillery
skill.

**Independent Test**: Launch otherwise identical crosswind shots with zero, positive, and negative
wind. Verify that zero wind retains the familiar path, while the two non-zero paths visibly and
deterministically separate in opposite horizontal directions. Verify the same direction and
strength are visible before either player acts.

**Acceptance Scenarios**:

1. **Given** a new standard duel, **When** either player views the aiming display, **Then** it
   clearly states that wind is blowing **toward +X** at **1.5 units/s²**.
2. **Given** two otherwise identical shots under zero and default wind, **When** both advance for
   the same simulated duration, **Then** only the default-wind shot is displaced toward +X.
3. **Given** a player observes a shot drift toward the displayed wind direction, **When** that
   player later deliberately aims against that direction, **Then** the later impact changes in the
   expected compensating direction without automatic aim adjustment.
4. **Given** Player One's turn changes to Player Two's, **When** the new player views the HUD,
   **Then** the same match wind remains clearly displayed and unchanged.

---

### User Story 2 - Preserve a Trustworthy Artillery Simulation (Priority: P1)

Players continue to receive deterministic terrain impacts, damage, deformation, tank settling,
and turn handoff; wind changes only the projectile's horizontal path and therefore its resulting
impact location.

**Why this priority**: Wind must enrich the existing duel without undermining the stable rules
players already understand.

**Independent Test**: Drive identical fixed-step projectile traces with known gravity, terrain,
and wind. Verify identical traces are equal; verify a wind-altered impact still produces the
normal downstream damage, crater, settling, match, and turn-resolution consequences at its actual
impact position.

**Acceptance Scenarios**:

1. **Given** zero-strength wind, **When** a projectile advances, **Then** its position, velocity,
   terrain collision, and outcome match the current no-wind behavior within normal numeric
   tolerance.
2. **Given** horizontal wind, **When** a projectile advances, **Then** gravity remains the sole
   vertical acceleration and wind adds no vertical velocity or displacement component.
3. **Given** a wind-altered trajectory strikes terrain, **When** impact resolves, **Then** the
   existing explosion damage, crater, tank-support evaluation, settling, and handoff use that
   resolved wind-altered impact position.
4. **Given** the same launch, terrain, gravity, wind, and fixed-step sequence, **When** it is
   simulated more than once, **Then** every trajectory state and outcome is identical.

---

### User Story 3 - Make Wind Matter Without Becoming a New Vehicle System (Priority: P2)

Players can recognize that longer airborne shots accumulate more drift than short shots, while
tanks, movement, gravity-driven settling, explosions, terrain shape, camera behavior, and turn
rules remain unchanged by wind itself.

**Why this priority**: Time-dependent drift creates useful aim choices, but the first
environmental variable must stay tightly bounded and readable.

**Independent Test**: Compare short and long equal-wind fixed-step traces, plus parallel and
perpendicular launch directions. Verify longer exposure produces greater displacement, crosswind
produces lateral drift, downrange wind changes range, and no non-projectile gameplay state changes.

**Acceptance Scenarios**:

1. **Given** equal non-zero wind, **When** a long/high projectile and a short/low projectile are
   compared, **Then** the longer airborne shot has greater horizontal wind displacement.
2. **Given** wind perpendicular to launch direction, **When** a projectile advances, **Then** it
   gains clear lateral drift toward wind.
3. **Given** wind parallel or opposite to launch direction, **When** a projectile advances, **Then**
   its downrange displacement respectively increases or decreases in accordance with wind.
4. **Given** any wind condition, **When** tanks move, settle, take explosion damage, or camera
   presentation updates, **Then** wind does not directly alter those systems.

### Edge Cases

- A zero vector is valid and exactly preserves existing no-wind projectile behavior; it is useful
  for deterministic tests even though the ordinary development duel begins with noticeable wind.
- Reversing a horizontal wind vector reverses its X/Z effect. A larger magnitude produces a larger
  horizontal displacement for the same fixed-step trace.
- Wind has no vertical component. A non-finite value or a vector with any vertical component is
  invalid rather than silently changing gravity or producing an undefined trajectory.
- A projectile leaving the existing useful volume still follows its normal out-of-bounds resolution;
  wind neither creates a second projectile lifetime nor changes turn-completion rules.
- Wind is constant from application start through the finished match. It does not change between
  turns, during flight, after impacts, or while a tank settles.
- A stronger wind may move an impact to a different hill, crater, tank, or out-of-bounds result.
  Those ordinary resulting terrain/damage consequences remain the existing rules, not wind-specific
  effects.
- Tank movement, tank support/settling, explosions, crater geometry, damage radius, health,
  camera, and visual boom timing do not receive wind force or a wind-dependent rule.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The game MUST own one explicit authoritative wind condition for the whole
  battlefield. It MUST have finite horizontal direction and magnitude, no vertical component, and
  remain constant for the full local match.
- **FR-002**: The initial development condition MUST blow **toward +X** with horizontal projectile
  acceleration magnitude **1.5 world units/s²**. This is an explicit tuneable starting value;
  automated scenarios MAY supply zero, reversed, or alternate finite horizontal values without
  adding player match configuration or randomization.
- **FR-003**: The first wind model MUST add its constant horizontal acceleration directly to each
  projectile's existing fixed-step motion. It MUST not add vertical acceleration, drag, mass,
  atmosphere, altitude dependence, turbulence, gusts, or a separate projectile path.
- **FR-004**: With zero-strength wind, projectile position, velocity, collision, impact, and
  downstream gameplay consequences MUST remain equivalent to current no-wind behavior within the
  simulation's established numeric tolerance.
- **FR-005**: For the same launch and fixed-step count, reversing wind direction MUST reverse its
  horizontal influence; a larger magnitude MUST produce a greater horizontal displacement; and a
  longer airborne duration MUST accumulate greater influence than a shorter duration.
- **FR-006**: Wind direction MUST mean the direction the wind pushes a projectile **toward**, never
  its origin. The HUD MUST state both the horizontal direction using the documented world axes and
  its numeric magnitude in world units/s²; for the default it MUST communicate `toward +X` and
  `1.5 units/s²` without relying on camera orientation.
- **FR-007**: The existing minimal aiming HUD MUST show the same current wind information while the
  match is in progress, including choosing, moving, and resolving-fire states, without a HUD
  redesign or a new action-selection surface.
- **FR-008**: Wind MUST be authoritative fixed-step gameplay state. It MUST not depend on render
  frames, camera timing, visual effects, HUD state, or input timing beyond the existing launch
  state. Identical initial projectile, terrain, gravity, wind, and step inputs MUST yield identical
  trajectory and impact results.
- **FR-009**: Existing terrain collision must continue to use the wind-altered swept trajectory.
  When impact occurs, existing explosion damage, crater deformation, living-tank support/settling,
  survival/match evaluation, and handoff MUST occur exactly once at the resolved impact position.
- **FR-010**: Wind MUST affect projectiles only. It MUST NOT alter tank aiming values, deliberate
  movement, movement allowance, terrain passability, tank orientation, tank support/settling,
  gravity, explosions, damage rules, terrain deformation, camera state, or presentation timing.
- **FR-011**: Wind direction/strength MUST be documented with world-axis convention, default value,
  mathematical effect, and compensation intent. Documentation MUST state that wind is constant
  during a match and excludes vertical wind, weather, environmental animation, and automatic
  compensation.
- **FR-012**: Automated tests MUST cover invalid wind validation, zero behavior, horizontal
  direction/reversal, magnitude, perpendicular/parallel influence, no vertical wind acceleration,
  duration accumulation, deterministic equal traces, wind-altered terrain impact, and preservation
  of existing gameplay resolution. Tests MUST avoid pixel, camera, or render-frame assertions.
- **FR-013**: The workspace build, relevant tests, formatting, and linting MUST pass without
  unjustified warnings. Roadmap checkboxes MUST only be checked where the delivered behavior and
  manual play genuinely satisfy their intended meaning.

### Key Entities

- **Battlefield wind condition**: The single constant horizontal push for the active local match,
  expressed in the existing world axes as a finite direction and acceleration magnitude.
- **Wind contribution**: The horizontal acceleration applied alongside gravity to every active
  projectile on each authoritative fixed step; it has no Y component.
- **Wind display**: The read-only player-facing statement of the direction wind blows toward and
  its numeric strength, derived from the same authoritative condition used by simulation.
- **Wind-altered impact**: The existing terrain-impact result after projectile motion includes wind;
  it remains the sole source of explosion, deformation, and later turn consequences.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In automated fixed-step traces, zero wind produces trajectory position and velocity
  equal to the prior ballistic trace within established numeric tolerance, and identical non-zero
  inputs produce 100% identical repeat traces.
- **SC-002**: In automated comparisons, opposite equal-strength wind produces displacement with
  opposite horizontal sign; a 3.0-units/s² condition produces greater displacement than the default
  1.5-units/s² condition after the same number of steps.
- **SC-003**: In manual play, both players can identify the stated toward-direction and numeric
  strength from the normal HUD before firing, without consulting source code or inferring it from a
  shot.
- **SC-004**: In manual play with the default wind, a high/long test shot shows visibly more drift
  than a shorter test shot, and a later deliberate against-wind aim correction moves the landing
  result toward the intended line.
- **SC-005**: In automated wind-altered impact scenarios, 100% of explosions, damage, craters,
  tank settling, winner/draw results, and turn handoffs use the one resolved impact with no
  duplicate or presentation-timed consequence.
- **SC-006**: A full two-player local duel remains playable with constant default wind: aiming,
  move-or-fire choice, terrain movement, tank settling, projectile collision, damage, victory,
  and camera presentation retain their documented behavior outside projectile drift.

## Assumptions

- The existing world convention is sufficient for wind presentation: `toward +X` is more precise
  than screen-relative `right`, which would vary with the presentation-only camera. A simple HUD
  line such as `Wind: toward +X, 1.5 units/s²` is sufficient for the first release.
- The direct horizontal acceleration model is intentionally gameplay-oriented. It means wind
  continuously nudges projectile velocity toward its displayed direction, so drift grows with
  flight duration without requiring drag or atmospheric simulation.
- The default +X / 1.5-units/s² value is a development tuning start chosen to be visible over a
  typical 2–3 second shot while remaining smaller than the existing 8-units/s² downward gravity.
  Playtesting will determine whether it genuinely completes the wind-gameplay evaluation items.
- Gravity remains an independent vertical acceleration and continues to govern tank settling.
  Wind does not move tanks or modify any non-projectile system.
- This feature depends on existing world vectors, fixed-step projectile/terrain collision, one
  current battlefield-gravity resource, HUD text, turn resolution, damage/deformation, and
  presentation-only camera. It adds no environment presets, weather framework, configuration
  format, new crate, controller, or automatic aiming help.

## Roadmap Alignment

- On completion, review **Wind / Basic Wind** and check only the implemented vector,
  horizontal-projectile, visible direction/strength, deterministic-condition, and trajectory-test
  items.
- Keep **Determine whether vertical wind should be supported** unchecked: this feature explicitly
  excludes it rather than deciding it for future gameplay.
- Check wind-strength/playability/crosswind-compensation evaluation items only after actual manual
  duels demonstrate those criteria. Do not equate a numeric default with completed balance work.
- The aiming-feedback and HUD wind items may be checked if the normal in-game display is clear and
  truthful. Do not automatically check broader environmental-identity or environment-preset work.
- Check **Milestone F — Wind affects projectiles** only if the integrated, readable, deterministic
  local-duel behavior meets this feature's manual acceptance, not merely because a force exists.
