# Feature Specification: First Crater — Terrain Deformation

**Feature Branch**: `20260905-183525-first-crater-terrain-deformation`  
**Created**: 2026-09-05  
**Status**: Draft  
**Input**: User description: "Create First Crater — Terrain Deformation."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - See an Impact Reshape the Battlefield (Priority: P1)

A developer fires the existing terrain-impact shot and sees its resolved impact leave a clear,
permanent crater in the battlefield after the existing boom begins.

**Why this priority**: A crater turns the established impact feedback into the first lasting,
gameplay-relevant change and completes the essential artillery loop.

**Independent Test**: Apply one deterministic crater to known terrain and verify that the centre is
lower, terrain beyond its radius is unchanged, and the rendered ground reflects the changed shape.

**Acceptance Scenarios**:

1. **Given** a projectile records a terrain impact, **When** the impact is processed, **Then** one
   permanent crater is centred on that resolved world position.
2. **Given** a crater is created, **When** the battlefield is viewed, **Then** its lowered shape is
   clearly visible while the existing explosion and impact marker remain useful.
3. **Given** crater radius or depth is changed in the default crater configuration, **When** an
   impact occurs, **Then** the visibly affected area or amount of lowering changes accordingly.

---

### User Story 2 - Fire Into Changed Ground (Priority: P2)

A developer fires a later projectile after a crater has formed and sees it interact with the
lowered battlefield rather than the pre-impact surface.

**Why this priority**: The terrain must be gameplay state, not a visual decal; later shots must
use the same changed ground players can see.

**Independent Test**: Deform deterministic terrain, then advance trajectories that pass through
removed space and descend into the crater; verify the former remains unblocked and the latter hits
the new surface.

**Acceptance Scenarios**:

1. **Given** a crater has lowered terrain at a horizontal position, **When** that position is
   queried, **Then** its returned surface height is the deformed height.
2. **Given** a later projectile passes through space removed by a crater, **When** it advances,
   **Then** it does not collide with the old surface.
3. **Given** a later projectile descends into a crater, **When** it reaches the lowered surface,
   **Then** it records an impact on that new surface.

---

### User Story 3 - Reliably Layer Battlefield Changes (Priority: P3)

A developer can create overlapping and edge-adjacent craters without invalid terrain, crashes, or
loss of prior terrain changes.

**Why this priority**: Repeated impacts are central to artillery play and must compose predictably
before tank response and other tactical consequences are added.

**Independent Test**: Apply the same sequence of flat, sloped, overlapping, and edge-adjacent
impacts twice; verify identical queried terrain, valid bounded surface data, and no panic.

**Acceptance Scenarios**:

1. **Given** a crater overlaps earlier deformation, **When** it is applied, **Then** it lowers the
   terrain as it currently exists rather than restoring the original surface.
2. **Given** an impact lies near or at a battlefield boundary, **When** its crater is applied,
   **Then** the crater is safely clipped to valid battlefield terrain.
3. **Given** identical initial terrain, crater settings, and impact sequence, **When** deformation
   is repeated, **Then** the resulting terrain heights are identical.

### Edge Cases

- A crater centre may fall on a slope, triangle boundary, or existing sharp terrain feature; the
  resulting surface remains finite and queryable.
- Terrain outside the configured crater radius remains unchanged.
- A crater partly beyond the battlefield affects only valid in-bounds terrain and never accesses
  invalid data.
- Repeated or overlapping impacts compose from the current terrain without unbounded or invalid
  values under representative development use.
- A projectile that ends out of bounds without terrain impact creates no crater.
- Tanks may temporarily remain at their existing positions when nearby terrain changes; their state
  must remain valid and the application must not crash.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: A terrain impact MUST create one permanent crater using the existing resolved
  world-space impact position as its sole centre. No visual effect may invent or alter that centre.
- **FR-002**: The battlefield MUST have one authoritative mutable terrain state from which the
  rendered surface, local terrain-height queries, and projectile-terrain intersection are derived.
- **FR-003**: A crater MUST lower the current terrain with one deterministic, understandable radial
  depression profile. The profile MUST be documented sufficiently to explain its centre, edge, and
  overlap behaviour.
- **FR-004**: The default crater configuration MUST expose explicit, independently tuneable
  gameplay radius and depth values. Neither value may derive from explosion presentation scale or
  duration.
- **FR-005**: Terrain at the crater centre MUST be lower than its pre-impact height for a positive
  configured depth; terrain at and beyond the configured radius MUST remain unchanged by that
  crater.
- **FR-006**: A later crater MUST apply to the terrain's current state so overlapping and repeated
  impacts compose predictably without reconstructing the original battlefield.
- **FR-007**: A crater near the battlefield edge MUST safely affect only valid terrain. It MUST NOT
  panic, corrupt terrain state, or create non-finite surface values.
- **FR-008**: The visible battlefield MUST update promptly after deformation and agree with the
  same authoritative terrain surface used by gameplay queries.
- **FR-009**: Existing terrain-height queries MUST return deformed surface heights immediately
  after a crater is applied.
- **FR-010**: Subsequent fixed-step projectile collision MUST use deformed terrain immediately:
  removed space is passable and descending projectiles impact the new lower surface.
- **FR-011**: Given identical initial terrain, ordered resolved impact positions, and crater
  parameters, deformation and all resulting terrain queries MUST be deterministic independently of
  rendering frame rate.
- **FR-012**: Existing projectile launch, gravity, fixed-step movement, impact resolution, impact
  marker, temporary boom, bounds handling, battlefield camera, and tanks MUST remain available.
  Tanks are not required to settle, slide, fall, move, or take damage after deformation.
- **FR-013**: Automated coverage MUST verify centre lowering, unchanged exterior, radius and depth
  effects, profile transition, sloped terrain, deterministic repetition, overlapping craters,
  edge safety, deformed height queries, and later projectile interaction with both removed space
  and the new crater surface. Tests MUST not depend on renderer output.
- **FR-014**: Completion MUST update affected battlefield/projectile documentation and only the
  roadmap items whose acceptance criteria are satisfied: near-term terrain deformation, basic
  crater creation, crater radius/depth, stability, collision update, subsequent projectile
  interaction, explosion terrain deformation, and altered future projectile lines if demonstrated.
  Tank repositioning, damage, and all unrelated roadmap work remain unchecked.
- **FR-015**: Completion MUST leave the workspace buildable, relevant tests passing, formatting
  passing, and Clippy warning-free without unjustified exceptions.

### Key Entities

- **Mutable authoritative terrain**: The bounded battlefield surface whose current elevation is the
  sole source for visible ground, height queries, and projectile intersection.
- **Crater configuration**: The small gameplay configuration containing default radius and depth.
- **Crater application**: One deterministic lowering of current terrain around an authoritative
  impact position, safely clipped at battlefield bounds.
- **Terrain impact result**: The existing deterministic projectile outcome that supplies the crater
  centre and remains separate from presentation.
- **Derived battlefield surface**: The promptly refreshed visible form of the authoritative terrain;
  it adds no independent collision or deformation state.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In deterministic tests, 100% of positive-depth craters lower their centre and leave
  sampled positions at or outside their configured radius unchanged by that crater.
- **SC-002**: In deterministic tests varying only default radius or depth, 100% show the expected
  change in affected extent or centre lowering respectively.
- **SC-003**: For at least 100 repetitions of an identical initial terrain and impact sequence,
  100% produce identical sampled terrain heights and projectile outcomes.
- **SC-004**: In automated later-shot cases, 100% of paths through space removed by a crater avoid
  the old surface, and 100% of descending paths into it impact the new queried surface.
- **SC-005**: A developer can launch the documented impact shot and observe the boom followed by a
  visible permanent crater within 30 seconds of application start; a subsequent shot visibly uses
  that changed ground.
- **SC-006**: Edge-adjacent and overlapping crater test cases complete without panic, invalid
  terrain values, or loss of prior deformation.
- **SC-007**: Workspace build, test, formatting, and lint checks complete with no new warnings.

## Assumptions

- The current bounded triangle-grid terrain, which already aligns rendering, queries, tanks, and
  collision, is the correct basis to evolve rather than replace.
- One smooth, radially symmetric depression with clipped boundary behaviour is sufficient for the
  first crater; deposition, noise, and physical excavation are deferred.
- Deformation occurs from the authoritative impact as part of gameplay processing; the existing
  visual boom may coexist but has no authority over terrain state or crater parameters.
- The existing tanks deliberately retain their initial poses after terrain changes until a focused
  terrain/tank reconciliation feature is specified.
- The small fixed battlefield can refresh its whole derived visible surface after each impact;
  chunking, level of detail, and GPU-first terrain work have no demonstrated need.

## Dependencies

- `docs/specs/20260905-115213-battlefield-camera/`, which introduced the rendered bounded
  battlefield and local terrain-height foundation.
- `docs/specs/20260905-123829-placeholder-tanks/`, which grounds initial tank poses on terrain.
- `docs/specs/20260905-143022-projectile-terrain-impact/`, which provides deterministic swept
  terrain intersection and resolved impact positions.
- `docs/specs/20260905-181606-visible-projectile-explosion/`, which presents the existing impact
  result without owning gameplay consequences.
- `docs/projectile-model.md`, `docs/world-conventions.md`, `docs/roadmap.md`, ADR 0001, and the
  Azimuth Constitution.

## Out of Scope

- Tank settling, falling, sliding, displacement, burial, terrain-related damage, explosion damage,
  health, player elimination, blast impulse, or direct tank collision.
- Dirt deposition, raised terrain, terrain-building weapons, collapse, erosion, procedural or
  seeded terrain generation, voxel terrain, terrain debris, smoke, sound, camera response, or
  advanced visual polish.
- Aiming, turns, movement, weapons or inventories, wind, drag, physics engines, generic terrain
  engines, generic weapon/effect frameworks, terrain chunking, level of detail, compute shaders,
  GPU-first deformation, new crates, or dependencies without demonstrated need.
