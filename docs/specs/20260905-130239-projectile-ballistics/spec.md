# Feature Specification: First Projectile and Deterministic Ballistic Arc

**Feature Branch**: `feature/projectile-ballistics`  
**Created**: 2026-09-05  
**Status**: Draft  
**Input**: User description: "Introduce a visible, deterministic projectile launched from a
placeholder tank under configurable gravity, stopping before terrain impact."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Fire and Observe the First Arc (Priority: P1)

A developer running Azimuth can use one documented development control to fire a clearly visible
projectile from Player One's existing tank. The projectile begins at that tank's established firing
origin, rises, reaches an apex, and descends through the three-dimensional battlefield.

**Why this priority**: This is the smallest visible artillery interaction and establishes the core
gameplay loop's most important physical behaviour without pretending that terrain impact or aiming
already exists.

**Independent Test**: Start the application, use the documented fire control, and inspect the shot
with the existing development camera. Confirm the projectile visibly leaves the firing origin and
follows a recognisable arc before its documented lifetime ends.

**Acceptance Scenarios**:

1. **Given** the initial battlefield and tanks are visible and no shot is active, **When** the
   developer uses the documented development fire control, **Then** exactly one visible projectile
   starts at Player One's firing origin with the documented fixed development shot.
2. **Given** an active upward development shot, **When** time advances, **Then** its horizontal
   motion remains steady while it rises, reaches a highest point, and then descends.
3. **Given** a shot is already active, **When** the developer requests another launch, **Then** the
   request is ignored and no queue or second projectile is created.
4. **Given** a projectile crosses or falls below the visual terrain, **When** it continues to be
   in the useful simulation volume, **Then** it continues its flight without impact, damage, or
   terrain change.

---

### User Story 2 - Reason About Every Shot (Priority: P2)

A contributor can understand what a shot means without relying on presentation conventions: a
horizontal azimuth, an upward elevation, a launch speed, a firing origin, and a configurable
downward gravity value determine a repeatable trajectory.

**Why this priority**: Learnable and reproducible physics is a constitutional requirement. Clear
conventions let later aiming, collision, and environmental work agree on the same shot meaning.

**Independent Test**: Exercise the domain calculation with known launch parameters and fixed
simulation steps. Verify documented cardinal directions, elevation behaviour, unchanged horizontal
velocity, predictable vertical acceleration, and identical results from identical inputs.

**Acceptance Scenarios**:

1. **Given** azimuth zero and zero elevation, **When** a launch direction is derived, **Then** it
   points horizontally along the documented zero direction with no upward component.
2. **Given** a positive elevation, **When** a launch direction is derived, **Then** it has an
   upward component; at 90 degrees it points straight up.
3. **Given** identical launch state, gravity, and a sequence of fixed steps, **When** the sequence
   is simulated more than once, **Then** every resulting position, velocity, and active state is
   identical.
4. **Given** two otherwise identical upward shots with different positive gravity strengths,
   **When** both advance for the same duration, **Then** the stronger-gravity shot is lower and has
   a smaller vertical velocity.

---

### User Story 3 - Tune Gravity During Development (Priority: P3)

A contributor can supply an explicit gravity value rather than inheriting an Earth assumption and
can confirm that it changes the same development shot's visible arc. A shot also eventually ends
after clearly leaving the useful simulation area, preventing indefinite background flight.

**Why this priority**: Gravity is intended to become a readable battlefield condition. Making it
explicit now enables experimentation while keeping the first model small.

**Independent Test**: Run equivalent shots under the documented default and a different gravity
value, compare their trajectories, then advance a shot beyond its termination limits.

**Acceptance Scenarios**:

1. **Given** the default development shot, **When** positive gravity is reduced or increased,
   **Then** its apex and descent visibly change without changing the launch parameters.
2. **Given** zero gravity, **When** a projectile advances, **Then** both horizontal and vertical
   velocity remain constant.
3. **Given** an active projectile that passes beyond a documented horizontal, vertical, or elapsed
   time limit, **When** the next simulation step is processed, **Then** the projectile becomes
   inactive and is no longer rendered.

### Edge Cases

- A launch elevation of 0 degrees has no initial vertical speed; it is still a valid physics input
  even though it may quickly pass through the currently non-solid terrain.
- A launch elevation of 90 degrees has no horizontal speed and remains a valid vertical test shot.
- A launch request while a projectile is active must not reset, duplicate, or otherwise alter the
  active projectile.
- A projectile can pass through or below terrain without terminating; only the defined simulation
  limits end it in this feature.
- A zero gravity value is valid and produces constant velocity; negative gravity values are invalid
  for this first model and must be rejected before simulation rather than silently reversing the
  world.
- An invalid elevation, non-positive launch speed, non-finite parameter, or zero-length horizontal
  firing direction must be rejected rather than creating an undefined trajectory.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The feature MUST preserve the established Y-up, origin-centred, abstract-unit
  battlefield conventions and define the remaining projectile-relevant conventions in
  `docs/world-conventions.md`.
- **FR-002**: The horizontal game plane MUST be X/Z. For shot semantics, azimuth 0 degrees MUST
  point along negative Z; positive azimuth MUST rotate clockwise when viewed from positive Y, so
  90 degrees points along positive X. This convention MUST not depend on a presentation API's
  naming of forward.
- **FR-003**: Elevation MUST be measured upward from the horizontal plane: 0 degrees is level and
  90 degrees is straight up. The first shot model MUST accept the inclusive 0 through 90 degree
  range; later player-facing aiming limits remain future work.
- **FR-004**: A launch direction MUST be derived from azimuth `a` and elevation `e` as
  `(sin(a) * cos(e), sin(e), -cos(a) * cos(e))`, with angles interpreted in degrees at the feature
  boundary. The resulting direction MUST be unit length within normal floating-point tolerance.
- **FR-005**: The existing tank turret's normalized horizontal direction MUST map to the same
  azimuth convention. A projectile launch MUST begin exactly at the owning tank's already-defined
  firing origin; that origin is the barrel-end reference, not an independently chosen render point.
- **FR-006**: The feature MUST introduce only the concrete projectile state needed for this slice:
  three-dimensional position, three-dimensional velocity, and whether the projectile remains in
  flight. It MUST NOT add mass, drag, wind response, damage, explosive properties, fuses, weapon
  identity, collision state, or a generic projectile/weapon hierarchy.
- **FR-007**: A shot description MUST explicitly contain launch position, azimuth, elevation, and
  launch speed. For this feature, launch power MAY be presented directly as speed in abstract world
  units per second; it MUST NOT define a final player-facing power scale.
- **FR-008**: Gravity MUST be an explicit non-negative battlefield simulation parameter expressed
  as downward acceleration, not an implicit Earth constant. The feature MUST choose and document a
  visible default suitable for development, while allowing a different valid value to produce a
  different trajectory from identical launch inputs.
- **FR-009**: Projectile motion MUST use a deterministic fixed simulation step of 1/120 second;
  rendering frame rate MUST NOT define physics results. Each step MUST use constant downward
  acceleration and the kinematic update `position += velocity * dt + 0.5 * acceleration * dt^2`,
  followed by `velocity += acceleration * dt`.
- **FR-010**: With gravity as the only force, horizontal velocity MUST remain unchanged and vertical
  velocity MUST change only by the configured downward acceleration. The implementation MUST keep
  the model small, direct, and independently testable without application or presentation state.
- **FR-011**: The application MUST provide one documented development launch control using a fixed
  shot from Player One. While a projectile is active, subsequent requests MUST be ignored. The
  feature MUST NOT add adjustable aiming, power controls, player selection, turn rules, queues, or
  support for multiple simultaneous projectiles.
- **FR-012**: An active projectile MUST be rendered as clear, simple placeholder geometry whose
  displayed location follows authoritative simulation state. No external art, visual effects, or
  presentation-owned motion calculation may be introduced.
- **FR-013**: A projectile MUST terminate only when it leaves the useful simulation volume or its
  maximum flight time expires. The initial limits are horizontal X or Z beyond 60 units from the
  origin, Y below -30 or above 100 units, or 20 simulated seconds. Terrain crossings MUST NOT
  terminate, alter, or collide with the projectile in this feature.
- **FR-014**: Deterministic domain tests MUST cover cardinal azimuth/elevation directions, launch
  velocity, gravity's effect on vertical velocity, invariant horizontal velocity, zero gravity,
  repeatability, ascent followed by descent, stronger-versus-weaker gravity, and useful analytical
  trajectory invariants. Renderer-internal or screenshot tests are not required.
- **FR-015**: Lightweight projectile diagnostics MAY be added only if they immediately help inspect
  launch or trajectory behaviour, remain non-authoritative, and can be disabled or removed without
  affecting the simulation. This feature MUST NOT create a general diagnostics framework.
- **FR-016**: Completion MUST preserve the existing battlefield, tanks, camera, and axes; pass
  workspace build, test, format, and lint checks; document the development fire control and
  projectile conventions; and update only roadmap items actually satisfied.

### Key Entities

- **Shot parameters**: The explicit, engine-independent description of one launch: firing origin,
  azimuth, elevation, and speed.
- **Launch direction**: The normalized three-dimensional direction calculated from documented shot
  angles before speed is applied.
- **Projectile state**: The in-flight object's current position, velocity, and active status. It
  has no collision or weapon meaning in this slice.
- **Gravity setting**: The explicit non-negative downward acceleration used for all fixed steps of
  this first ballistic model.
- **Fixed simulation step**: The shared 1/120-second advancement interval that makes an identical
  state and inputs produce an identical trajectory.
- **Simulation volume**: The deliberately generous region and maximum duration that determine when
  a non-colliding development projectile stops being simulated.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: From a fresh application launch, a developer can fire exactly one visible projectile
  from Player One's firing origin using the documented control within 10 seconds.
- **SC-002**: For the fixed development shot under default positive gravity, the developer can
  observe a rise, one clear apex, and a descent before the projectile reaches its termination
  limit.
- **SC-003**: Automated tests show that 100% of repeated runs using identical launch parameters,
  gravity, and fixed-step counts yield identical state, and that zero gravity preserves velocity.
- **SC-004**: Automated tests verify all four documented cardinal azimuth directions, level and
  vertical elevation directions, constant horizontal velocity, downward vertical acceleration, and
  a lower equal-duration trajectory under stronger positive gravity.
- **SC-005**: The active projectile terminates after crossing any documented simulation-volume or
  duration limit, while an equivalent projectile crossing terrain inside that volume remains active.
- **SC-006**: The completed workspace build, test, formatting, and lint validations all succeed
  without new warnings or ignored checks.

## Assumptions

- The completed battlefield-camera and placeholder-tanks features provide the current Y-up world,
  40 by 40 battlefield, development camera, debug axes, tank poses, and deterministic firing
  origins that this feature extends.
- Azimuth 0 degrees uses negative Z because it gives a clear, engine-neutral game convention while
  keeping a positive 90-degree turn visually intuitive toward positive X. This is a project rule,
  not a rendering convention.
- A fixed 1/120-second step is sufficiently smooth for the initial visible arc and sufficiently
  small, reproducible, and collision-ready for the next terrain-impact feature; tuning it later is
  permitted when evidence warrants it.
- The initial default gravity and fixed development shot will be selected for a clearly observable,
  readable arc within the existing battlefield view. They are development defaults, not a realism
  claim or final balance decision.
- The stated simulation limits deliberately extend beyond the 20-unit battlefield half-extent so a
  shot remains observable after crossing its visible edge. They are a lifetime safeguard, not
  terrain impact or final out-of-bounds gameplay policy.

## Dependencies

- Completed feature: `docs/specs/20260905-115213-battlefield-camera/`, which established the
  visible terrain, world bounds, camera, and debug axes.
- Completed feature: `docs/specs/20260905-123829-placeholder-tanks/`, which established two
  deterministic player/tank poses and explicit firing origins.
- Existing guidance: `docs/world-conventions.md`, `docs/roadmap.md`, the rendering decision record
  at `docs/adr/0001-initial-rendering-technology.md`, and the Azimuth constitution.

## Out of Scope

- Terrain collision, projectile impact resolution, explosions, terrain deformation, damage,
  health, tank destruction, player elimination, or impact markers.
- Player-controlled aiming, adjustable azimuth/elevation/power UI, turns, movement, weapon
  selection, weapon inventories, multiple projectile types, multiple concurrent projectiles,
  bounces, fuses, or projectile cameras.
- Wind, drag, atmosphere, projectile mass, variable gravity directions, altitude-dependent gravity,
  audio, final visual effects, AI, networking, persistence, or external art assets.
- A physics engine, generic simulation framework, generic projectile or weapon abstraction,
  renderer abstraction, custom ECS, event bus, or unrelated workspace restructuring.
