# Feature Specification: Placeholder Tanks and Player Entities

**Feature Branch**: `feature/placeholder-tanks`

**Created**: 2026-09-05

**Status**: Draft

**Input**: Add two distinct player-owned placeholder tanks, placed correctly on the existing
battlefield terrain, with clear orientations and identifiable firing origins but no artillery
gameplay.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - See the Beginnings of an Artillery Duel (Priority: P1)

A developer running Azimuth sees two clearly different placeholder tanks in separate, sensible
positions on the three-dimensional battlefield. Each tank visibly belongs to a distinct player and
faces a meaningful direction, so the scene reads as the start of an artillery duel rather than
unrelated scene geometry.

**Why this priority**: The two pieces give the battlefield its first game-specific spatial context
and allow the next gameplay slice to begin from recognisable player positions.

**Independent Test**: Start Azimuth and inspect the scene using the existing camera. Confirm two
visually distinguishable tanks are visible at separate locations, remain inside the battlefield,
and do not float or substantially intersect the ground.

**Acceptance Scenarios**:

1. **Given** Azimuth starts on the current battlefield, **When** the scene is visible, **Then** it
   contains exactly two placeholder tanks at distinct locations within the visible battlefield.
2. **Given** both tanks are visible, **When** the developer compares them, **Then** their player
   identities are immediately distinguishable using a simple visual difference.
3. **Given** a tank is placed on uneven ground, **When** the developer inspects it from multiple
   camera angles, **Then** its body rests on the terrain instead of visibly floating or being
   substantially buried.
4. **Given** the two spawn locations, **When** the developer views the battlefield from above or
   the side, **Then** the tanks are separated enough to suggest opposing positions in a duel.

---

### User Story 2 - Use Clear Player and Firing References (Priority: P2)

A contributor can identify each player's minimal game-piece information—identity, position, body
direction, turret/firing direction, and firing origin—without inferring it from presentation-only
objects or implementing aiming and projectiles.

**Why this priority**: The next projectile work needs a dependable, inspectable origin and
orientation while the project avoids prematurely turning the placeholder into a weapon system.

**Independent Test**: Review the deterministic player/tank data and verify that each player has a
unique identity, a bounded terrain-resolved position, an explicit body/turret direction, and a
firing origin that follows that direction.

**Acceptance Scenarios**:

1. **Given** the two initial players, **When** a contributor inspects their game-piece data,
   **Then** each has a distinct stable identity and a world position inside the established bounds.
2. **Given** a tank's body and turret directions, **When** its placeholder is inspected, **Then**
   the visible body and barrel direction agree with those directions.
3. **Given** a tank's position and firing direction, **When** the contributor obtains its firing
   origin, **Then** it is a deterministic point ahead of and above the tank that changes
   consistently with the tank's pose.
4. **Given** terrain height is requested at an initial spawn location, **When** placement is
   resolved, **Then** the tank uses that terrain height rather than an unrelated fixed elevation.

---

### User Story 3 - Inspect Tanks Without Losing Battlefield Context (Priority: P3)

A developer can continue using the existing orbit, pan, and zoom camera to inspect both tanks,
their orientations, the firing references, and the origin axes without automatic camera behaviour
or gameplay UI being introduced.

**Why this priority**: The scene remains a useful development view while the project deliberately
defers player selection, current-player focus, and cinematic camera work.

**Independent Test**: Use the documented camera controls to view both tanks, their terrain contact,
and their body/barrel directions from more than one viewpoint; confirm the orientation aid remains
available and no automatic camera movement occurs.

**Acceptance Scenarios**:

1. **Given** either tank is visible, **When** the developer uses the existing camera controls,
   **Then** they can inspect that tank without automatic focus or tracking.
2. **Given** the complete scene, **When** the developer changes viewpoint, **Then** the existing
   battlefield, terrain relief, and origin-axis visualisation remain visible and useful.

### Edge Cases

- A fixed spawn location lies on a locally high or low part of the terrain: its resolved placement
  still uses the terrain-height query and stays visibly grounded.
- A proposed spawn point is outside the established battlefield bounds: deterministic validation
  rejects or prevents that placement rather than silently placing a tank outside the scene.
- A body or turret direction is changed later: the derived firing origin remains consistent with
  the updated pose rather than remaining at an obsolete world position.
- The camera is panned or orbited away from one tank: no tank movement, player selection, or
  automatic camera tracking occurs.
- The placeholder geometry is viewed from behind or above: player distinction and forward/barrel
  direction remain understandable without labels or gameplay UI.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The feature MUST introduce exactly two deterministic, player-owned tank entities on
  the existing battlefield.
- **FR-002**: Each initial player MUST have a distinct stable identity and a corresponding tank
  entity; the feature MUST NOT introduce teams, customisation, configuration, AI, or networking.
- **FR-003**: Each tank MUST record a world position, body direction, turret or firing direction,
  and a firing origin suitable for later projectile-launch work.
- **FR-004**: Initial spawn locations MUST be distinct, inside the documented battlefield bounds,
  and separated enough to appear as opposing positions when viewed in the running scene.
- **FR-005**: Tank placement MUST resolve elevation from the current battlefield terrain at its
  horizontal spawn location. Tank geometry MUST appear grounded rather than visibly floating or
  substantially buried.
- **FR-006**: The feature MUST provide the smallest deterministic terrain-height query needed for
  initial tank placement. It MUST NOT introduce collision, movement, terrain editing, deformation,
  chunking, LOD, or a generic terrain framework.
- **FR-007**: Each tank MUST use simple placeholder geometry that communicates body position,
  body direction, and firing/turret direction. External art assets and an asset-production pipeline
  MUST NOT be introduced.
- **FR-008**: The two tanks MUST be visually distinguishable using a simple presentation difference
  and MUST remain readable from ordinary development-camera viewpoints.
- **FR-009**: Each tank's firing origin MUST be explicit or deterministically derivable from its
  pose, lie ahead of and above the tank's body in its firing direction, and remain consistent when
  the pose changes. This feature MUST NOT launch or render projectiles.
- **FR-010**: The player/tank representation MUST use clear concrete data with obvious ownership.
  It MUST avoid generic entity, actor, vehicle, weapon, player, spawn, or event abstractions and
  must not make rendering objects the sole owner of player-game meaning.
- **FR-011**: The existing battlefield camera and origin-axis aid MUST remain usable. The feature
  MUST NOT add player focus, camera tracking, target selection, cinematic transitions, or gameplay
  UI.
- **FR-012**: Deterministic terrain placement, spawn-bound validation, and firing-origin behaviour
  MUST have focused automated tests where practical. Brittle renderer-internal tests are not
  required.
- **FR-013**: Completion MUST preserve workspace build, test, formatting, and lint health; update
  affected documentation and only the roadmap items whose acceptance criteria are actually met.

### Key Entities

- **Player identity**: A stable, distinct identifier for one of the two initial human-controlled
  game pieces. It exists independently of visual material or geometry.
- **Tank entity**: The concrete initial player-owned piece, containing its owner identity, terrain
  resolved position, body direction, firing/turret direction, and firing origin.
- **Tank pose**: The position and directions that locate and orient a tank in the world and from
  which its visible placeholder and firing origin are derived.
- **Firing origin**: A deterministic world-space point ahead of and above the tank, intended for a
  later projectile feature but without launch behaviour in this scope.
- **Terrain-height query**: The limited deterministic operation that returns the current
  battlefield elevation for a horizontal location used by initial tank placement.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: On each application launch, exactly two distinct placeholder tanks are visible
  within the battlefield bounds and can be identified as different players within 30 seconds.
- **SC-002**: From at least three camera viewpoints, a developer can identify both tanks' body and
  barrel/firing directions and confirm they are grounded on terrain.
- **SC-003**: Repeated initial placement with the same inputs produces the same two player
  identities, spawn locations, orientations, terrain-resolved elevations, and firing origins.
- **SC-004**: Automated tests verify that both initial spawns are in bounds, terrain-resolved, and
  distinct, and that each firing origin is ahead of and above its tank.
- **SC-005**: The workspace-wide build, test, formatting, and lint commands complete successfully
  after the feature is implemented.

## Assumptions

- The completed minimal-battlefield feature remains the foundation for placement: it supplies the
  40 by 40 unit bounds, Y-up convention, deterministic visual terrain-height function, camera, and
  origin axes.
- Initial spawn positions are fixed and deterministic; random placement and spawn balancing are
  separate future work.
- A straightforward body, turret, and barrel silhouette made from simple geometry is sufficient to
  establish orientation and a useful firing reference; no final visual design is implied.
- The tank body may remain upright if slope alignment would add disproportionate complexity. It
  must still resolve its vertical placement from the terrain and appear grounded.
- Initial player identities are implementation-stable identifiers intended for future game-domain
  work, not player-facing names or a multiplayer account model.

## Dependencies

- Completed feature: `docs/specs/20260905-115213-battlefield-camera/`, which established the
  existing battlefield, bounds, limited world conventions, controllable camera, and debug axes.
- Existing guidance: `docs/world-conventions.md`, `docs/roadmap.md`, and the Azimuth constitution.

## Out of Scope

- Projectile rendering, launch, aiming, firing controls, gravity, collision, explosions, terrain
  deformation, health, damage, destruction, movement, turns, weapons, AI, UI, animation, player
  setup, persistence, networking, and final tank art.
- Procedural spawning, generic entity or vehicle frameworks, generic terrain queries, external art
  assets, and asset pipelines.
