# Feature Specification: First Boom — Visible Projectile Explosion

**Feature Branch**: `20260905-181606-visible-projectile-explosion`  
**Created**: 2026-09-05  
**Status**: Draft  
**Input**: User description: "Create First Boom — Visible Projectile Explosion."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - See an Impact Explode (Priority: P1)

A developer fires the existing terrain-impact development shot and sees an unmistakable temporary
explosion at the exact resolved impact location after the projectile stops.

**Why this priority**: A visible boom is the first satisfying game-like consequence of the
established artillery loop and makes an impact immediately legible.

**Independent Test**: Trigger a known terrain impact, then verify that one effect begins at the
existing impact position, visibly changes over a short lifetime, and is gone after it completes.

**Acceptance Scenarios**:

1. **Given** a projectile reaches a terrain impact, **When** presentation observes that result,
   **Then** a clearly visible temporary explosion starts at the resolved world-space position.
2. **Given** an explosion is active, **When** its explicit lifetime advances, **Then** it visibly
   expands or otherwise changes in a way that reads as an explosion and then disappears cleanly.
3. **Given** no terrain impact has occurred, **When** normal presentation updates run, **Then** no
   explosion is created from projectile flight or rendering guesses.

---

### User Story 2 - Keep Exact Impact Diagnostics Useful (Priority: P2)

A developer can still inspect the exact simulation-resolved impact point while the new explosion
adds readable gameplay feedback.

**Why this priority**: The existing marker remains valuable for verifying collision independently
of a deliberately exaggerated visual effect.

**Independent Test**: Observe an impact through the explosion lifecycle and confirm the marker
continues to identify the resolved point without altering or being replaced by the effect.

**Acceptance Scenarios**:

1. **Given** a terrain impact produces both marker and explosion, **When** the effect changes and
   expires, **Then** the marker remains available to identify the same impact position.
2. **Given** the marker is removed or disabled for development, **When** an impact occurs, **Then**
   the explosion still follows the simulation impact result and its lifecycle is unchanged.

---

### User Story 3 - Repeat the Impact Feedback (Priority: P3)

A developer can fire another shot after an impact and receives a new, clean explosion without
stale effects accumulating.

**Why this priority**: Repeated shots are the existing development workflow and must remain clean
before turns or multi-projectile systems exist.

**Independent Test**: Complete two sequential impact shots and verify each creates one temporary
effect that expires; no inactive presentation state remains after either lifecycle completes.

**Acceptance Scenarios**:

1. **Given** an earlier explosion has expired, **When** the next projectile impacts, **Then** a new
   explosion appears at the new impact position.
2. **Given** repeated impact shots, **When** each effect lifetime completes, **Then** obsolete
   explosion presentation is removed and does not accumulate.

### Edge Cases

- An impact occurs while no prior explosion exists: the first effect starts normally.
- An explosion expires before the next shot: its cleanup does not remove the enduring impact marker.
- A new impact follows an earlier expired or active effect: the current single-shot workflow
  remains clean without requiring general simultaneous-effect infrastructure.
- A projectile terminates out of bounds without terrain impact: no explosion is shown.
- The visual radius, duration, and intensity are presentation values only and never become damage,
  deformation, or authoritative gameplay radius.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The application MUST create a visible temporary explosion only by observing the
  existing terrain-impact result. Presentation MUST NOT re-detect terrain collision, derive a
  second impact point, or alter projectile simulation.
- **FR-002**: The explosion MUST originate at the resolved world-space impact position exposed by
  simulation and remain clearly associated with that location throughout its lifetime.
- **FR-003**: The effect MUST have an explicit, tuneable visual lifetime and a simple visible
  lifecycle: it starts, changes in an explosion-like way, and removes its presentation state when
  complete.
- **FR-004**: The effect MUST be readily visible from the existing normal development camera and
  communicate that a projectile hit there. Deliberate visual exaggeration is permitted.
- **FR-005**: The feature MUST use only presentation-oriented visual parameters. It MUST NOT
  introduce gameplay explosion radius, damage, tank effects, terrain deformation, impulse,
  collision, weapon-specific behaviour, or a physical explosion simulation.
- **FR-006**: The existing impact marker MUST remain useful as an independent diagnostic after the
  explosion has expired. Marker availability MUST NOT determine whether the explosion appears.
- **FR-007**: The existing launch controls, fixed-step flight, terrain impact detection, projectile
  termination, camera, tanks, and non-impact termination behaviour MUST remain unchanged.
- **FR-008**: Repeated sequential impacts MUST produce repeated temporary explosions without
  accumulating obsolete presentation entities or resources.
- **FR-009**: Any deterministic non-renderer lifecycle values introduced solely for effect timing
  MUST have proportionate automated tests for initial position, progression, expiry, and repeated
  cleanup. Screenshot and renderer-internal tests are not required.
- **FR-010**: Completion MUST update the README and relevant presentation/impact documentation,
  check only the near-term visible impact/explosion and explosion-presentation roadmap items whose
  acceptance criteria are met, and leave all damage, deformation, audio, and camera-response work
  unchecked.
- **FR-011**: Completion MUST leave the workspace buildable, relevant tests passing, formatting
  passing, and Clippy warning-free without unjustified exceptions.

### Key Entities

- **Terrain impact result**: The existing authoritative simulation result containing the resolved
  impact position; it is the sole source for an explosion's origin.
- **Explosion presentation**: A temporary visual effect at one impact position, with an explicit
  elapsed lifetime and visual configuration; it has no gameplay meaning.
- **Impact marker**: The existing enduring development diagnostic for the exact resolved position,
  intentionally separate from the temporary presentation effect.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: For every terrain impact in automated lifecycle coverage, exactly one explosion
  begins at that impact position and no effect begins for non-impact termination.
- **SC-002**: An explosion completes its visible lifecycle and removes its presentation state
  within its documented, tuneable duration in 100% of repeated test runs.
- **SC-003**: A developer can launch the documented impact shot, see an obvious boom, and identify
  the enduring impact marker within 30 seconds of application start.
- **SC-004**: After two sequential impacts and their lifetimes, no expired explosion remains while
  the current marker still identifies the latest impact.
- **SC-005**: Workspace build, test, formatting, and lint commands complete successfully without
  new warnings.

## Assumptions

- The current `LatestTerrainImpact` presentation handoff and marker are the appropriate direct
  input and diagnostic companion; no event bus is needed for the single-projectile workflow.
- A bright expanding primitive with a short lifetime is a sufficient first boom. Detailed visual
  effects, sound, smoke, particles, and camera response are later presentation work.
- The existing fixed impact development shot remains the practical manual way to inspect this
  feature; its ballistic and collision behaviour are not part of explosion presentation.

## Dependencies

- Completed feature: `docs/specs/20260905-143022-projectile-terrain-impact/`, which supplies the
  deterministic terrain-impact position, terminated projectile lifecycle, marker, and development
  impact shot.
- Existing project guidance: `docs/projectile-model.md`, `docs/world-conventions.md`,
  `docs/roadmap.md`, ADR 0001, and the Azimuth Constitution.

## Out of Scope

- Damage, health, tank hits/destruction, authoritative explosion radius, terrain deformation,
  craters, terrain updates, blast impulse, chain reactions, or non-terrain collision.
- Sound, camera shake/cinematics, persistent fire, meaningful smoke/debris systems, weapon types,
  aiming, turns, movement, wind, or drag.
- A physics engine, custom particle engine, generic VFX/animation/event framework, new crate, or
  dependency introduced solely for this effect.
