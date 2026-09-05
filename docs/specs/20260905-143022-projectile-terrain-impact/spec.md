# Feature Specification: Projectile Terrain Impact Detection

**Feature Branch**: `20260905-143022-projectile-terrain-impact`  
**Created**: 2026-09-05  
**Status**: Draft  
**Input**: User description: "Create Projectile Terrain Impact Detection."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Observe a Shot Strike the Battlefield (Priority: P1)

A developer fires the existing development shot and can see it follow its established ballistic arc
until it reaches the visible battlefield, where it stops rather than continuing below the ground.

**Why this priority**: Stopping at the battlefield completes the first essential artillery-loop
handoff from flight to an outcome and makes the simulation's result believable and inspectable.

**Independent Test**: Launch a deterministic shot that crosses from above to below a known terrain
surface, then verify that flight ends and the reported hit location is on that surface.

**Acceptance Scenarios**:

1. **Given** an active projectile whose travelled fixed-step path crosses the battlefield surface,
   **When** that step is simulated, **Then** the projectile stops at a resolved terrain impact
   location and is no longer an active flight.
2. **Given** an active projectile that remains above the terrain during a simulated step, **When**
   the step is simulated, **Then** it remains in flight and continues its unchanged ballistic arc.
3. **Given** a high-speed projectile that passes from above to below terrain in one fixed step,
   **When** the step is simulated, **Then** it records a terrain impact instead of travelling
   through the ground.

---

### User Story 2 - Inspect Where the Simulation Registered Impact (Priority: P2)

A developer can immediately see a lightweight marker at the location the simulation reports after
the projectile disappears, including on distinct elevations and slopes.

**Why this priority**: The marker makes the otherwise invisible simulation decision easy to verify
now and provides a useful handoff point for the next explosion feature.

**Independent Test**: Complete shots into two visibly different terrain regions and confirm that a
marker remains at each reported landing location while no projectile visual remains in flight.

**Acceptance Scenarios**:

1. **Given** a projectile has impacted terrain, **When** the application presents the current
   scene, **Then** a simple, clearly distinguishable impact marker identifies the resolved world
   position on the terrain surface.
2. **Given** a developer fires another shot after the prior flight has ended, **When** it impacts
   elsewhere, **Then** the marker represents the current impact and does not affect projectile
   simulation.
3. **Given** the marker is disabled or removed for development, **When** a projectile impacts,
   **Then** detection, termination, and the exposed impact result remain unchanged.

---

### User Story 3 - Distinguish a Terrain Impact from Other Termination (Priority: P3)

A future gameplay system or debugging view can tell whether a projectile ended by striking terrain
or by leaving the established useful simulation area, without inventing explosion or damage data.

**Why this priority**: Later explosion work needs a trustworthy terrain outcome, while existing
out-of-bounds protection must retain its distinct meaning.

**Independent Test**: Simulate one terrain-crossing flight and one flight that exits the useful
volume without crossing terrain; compare their observable termination results.

**Acceptance Scenarios**:

1. **Given** a projectile intersects terrain, **When** its flight ends, **Then** the result states
   that terrain was impacted and provides the world-space impact position.
2. **Given** a projectile leaves its useful simulation volume without intersecting terrain,
   **When** its flight ends, **Then** it has no terrain-impact result.
3. **Given** identical terrain, launch state, gravity, fixed-step inputs, and termination limits,
   **When** the simulation is repeated, **Then** both runs produce the same termination kind and,
   for an impact, the same position.

### Edge Cases

- A projectile starts above terrain and crosses the complete surface between fixed steps; it must
  not tunnel through solely because neither endpoint is exactly on ground.
- A projectile whose next position remains above terrain continues normally, including under zero
  or a different valid gravity setting.
- A projectile exits the useful volume without reaching terrain; that outcome remains distinct from
  terrain impact and has no invented impact position.
- A projectile approaches a non-flat region or a slope; its recorded location must correspond to
  the same terrain surface seen by the developer, not a materially displaced approximation.
- A projectile path reaches the battlefield edge or lies outside its horizontal terrain extent; the
  established out-of-bounds/lifetime rules remain authoritative unless it crosses defined terrain first.
- An impact exactly at a step endpoint is treated once, producing one termination and one result.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The feature MUST preserve the established Y-up, X/Z horizontal, origin-centred,
  abstract-unit, angular, gravity, launch-origin, and fixed 1/120-second projectile conventions.
- **FR-002**: The feature MUST retain the existing deterministic non-flat battlefield and its
  horizontal bounds. It MUST establish a single documented authoritative terrain surface for local
  height, tank placement, visible ground, and projectile impact so an observed hit is not
  materially above or below the rendered terrain.
- **FR-003**: Each active projectile advance MUST evaluate the portion of the trajectory travelled
  during that fixed step against the authoritative terrain surface. It MUST detect a crossing from
  above the surface to on or below it, including when the projectile moves completely through the
  surface within one step.
- **FR-004**: On detecting a terrain crossing, the feature MUST resolve a deterministic world-space
  impact position on, or within 0.01 abstract units of, the authoritative terrain surface. The
  chosen approximation must be documented when its assumptions are not self-evident.
- **FR-005**: An impact MUST end projectile flight in the same fixed simulation advancement that
  detects it. The projectile's displayed location MUST not continue beneath or through terrain.
- **FR-006**: The simulation MUST expose a small concrete terrain-impact result containing at least
  the fact that terrain was impacted and its world-space position. It MUST expose no explosion,
  damage, weapon, material, normal, energy, impulse, penetration, ricochet, or generic collision
  information in this feature.
- **FR-007**: Flight termination due to terrain impact MUST remain observably distinct from
  termination due to the established useful-volume or lifetime limits. A non-impact termination
  MUST NOT fabricate a terrain-impact result.
- **FR-008**: Identical terrain, projectile initial state, gravity, fixed timestep, limits, and
  advancement inputs MUST yield identical impact/termination results independently of rendering
  frame rate. Rendering MUST observe simulation state and MUST NOT decide impact.
- **FR-009**: The existing one-shot development launch, launch vector, gravity behaviour,
  trajectory calculation, fixed-step strategy, visible projectile, camera, tanks, and axes MUST
  remain available. A shot's pre-impact ballistic path MUST not be retuned by this feature.
- **FR-010**: The application MUST provide a lightweight, removable or disableable development
  marker for the most recent terrain impact. It MUST be placed from the simulation's exposed
  impact position and MUST NOT be an explosion, particle effect, sound, camera effect, or a new
  diagnostics framework.
- **FR-011**: Deterministic automated tests MUST cover: no impact while above terrain; ordinary and
  high-speed swept crossings; an analytically understandable flat-terrain descent; representative
  non-flat/sloped terrain; impact position proximity to terrain; repeatability; relevant gravity
  variation; and out-of-bounds termination without terrain impact. Tests MUST not depend on
  renderer state or assert a particular intersection algorithm.
- **FR-012**: Completion MUST update `docs/projectile-model.md`, `docs/world-conventions.md`, and
  the README wherever their current no-collision/no-impact statements become inaccurate. It MUST
  update only roadmap items whose acceptance criteria are met: Detect projectile impact with the
  battlefield; Detect projectile intersection with terrain; and, if the marker is delivered,
  Provide impact markers. It MAY mark Define initial battlefield representation complete only when
  the completed shared terrain definition and documentation genuinely satisfy that item's intent.
- **FR-013**: Completion MUST leave the workspace buildable, relevant automated tests passing,
  formatting passing, and Clippy warning-free without unjustified exceptions.

### Key Entities

- **Authoritative terrain surface**: The bounded non-flat battlefield surface used consistently for
  local ground height, rendered ground, tank grounding, and projectile intersection.
- **Projectile flight**: The existing active position, velocity, and elapsed simulated time until a
  terrain impact or non-impact termination ends it.
- **Terrain impact result**: The small deterministic outcome that states terrain was struck and
  identifies the resolved world-space impact position.
- **Impact marker**: A presentation-only development aid that shows the terrain impact result and
  cannot influence simulation.
- **Non-impact termination**: The existing useful-volume or lifetime conclusion, deliberately
  separate from terrain impact.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In automated deterministic terrain-crossing cases, 100% of projectiles that move
  from above to below the authoritative surface in one fixed step terminate with a terrain-impact
  result; none remain active below that surface.
- **SC-002**: In automated flat and representative non-flat terrain cases, every reported impact
  position is within 0.01 abstract units of the authoritative terrain surface.
- **SC-003**: For at least 100 repeated simulations with identical inputs, 100% produce identical
  termination kinds and, when applicable, identical impact positions.
- **SC-004**: A developer can launch the documented development shot, observe it stop at terrain,
  and identify its impact marker within 30 seconds of application start.
- **SC-005**: In deterministic tests of paths that leave the useful volume without terrain contact,
  100% retain a non-impact termination and expose no terrain-impact result.
- **SC-006**: The workspace build, relevant tests, formatting check, and lint check all complete
  successfully with no new warnings.

## Assumptions

- The current deterministic battlefield height representation and fixed rendered grid are the
  starting point; this feature will align them rather than replace terrain merely to obtain collision.
- The existing 1/120-second kinematic update remains the simulation boundary. A small direct
  swept-segment refinement is sufficient for this single static terrain surface; a general
  continuous-collision or physics framework is not justified.
- The latest impact is sufficient development feedback for the current single-projectile workflow;
  marker history, persistence, or a generic event system are unnecessary.
- The existing useful-volume and lifetime rules continue to protect against indefinitely active shots.
- Explosion placement, damage, deformation, and all other post-impact gameplay remain later work
  that consumes this result rather than expanding this feature.

## Dependencies

- Completed projectile foundation: `docs/specs/20260905-130239-projectile-ballistics/`, which
  provides deterministic flight, gravity, limits, launch semantics, projectile presentation, and
  the development firing control.
- Existing battlefield and tank work: `docs/specs/20260905-115213-battlefield-camera/` and
  `docs/specs/20260905-123829-placeholder-tanks/`, which provide the visible bounded terrain,
  local height usage, tank grounding, and firing origin.
- Current project guidance: `docs/projectile-model.md`, `docs/world-conventions.md`,
  `docs/roadmap.md`, ADR 0001, and the Azimuth Constitution.

## Out of Scope

- Explosions, blast visuals, damage, health, direct or splash tank hits, terrain deformation,
  craters, post-deformation collision, and tank displacement.
- Projectile collision with tanks, projectiles, destructible objects, or any non-terrain object;
  bouncing, ricochet, penetration, rolling, or generic collision abstractions.
- Aiming controls, turns, movement, wind, drag, mass, weapon inventories/types, camera impact
  cinematics, sound, or final presentation effects.
- Replacing the current terrain system solely for collision, adding a physics engine or dependency,
  or building event-bus, surface-material, or diagnostics infrastructure without a demonstrated
  present need.
