# Feature Specification: Tank Support, Gravity and Terrain Settling

**Feature Branch**: `20260906-141906-tank-support-settling`

**Created**: 2026-09-06

**Status**: Draft

**Input**: User description: "Create the next Azimuth feature specification for Tank Support, Gravity and Terrain Settling."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Drop Into Newly Removed Ground (Priority: P1)

Two players can see a tank respond when a shell removes the terrain directly supporting its base:
the tank falls under the battlefield's gravity and comes to rest on the newly deformed surface
instead of floating at its old height.

**Why this priority**: This closes the visible inconsistency between deformable terrain and
gameplay tank positions, and makes craters a lasting tactical consequence.

**Independent Test**: Start a supported tank at a known horizontal position, lower the current
terrain beneath that same position, advance settling through deterministic simulation steps, and
verify that the tank ends at the new terrain height without passing through it.

**Acceptance Scenarios**:

1. **Given** a living tank rests on terrain, **When** a crater lowers the terrain at that tank's
   base by more than the support tolerance, **Then** the tank becomes unsupported and visibly
   descends under battlefield gravity.
2. **Given** an unsupported tank is descending, **When** its base reaches the current deformed
   terrain surface, **Then** it stops exactly at that surface and is again supported.
3. **Given** an explosion deforms terrain away from a supported tank's base, **When** the crater
   resolves, **Then** that tank's position and settling state do not change.
4. **Given** a crater only partly changes the rendered land around a tank but leaves the terrain
   at its representative base position within support tolerance, **When** support is evaluated,
   **Then** the tank remains stable rather than reacting merely because an explosion was nearby.

---

### User Story 2 - Finish a Shot Only After Ground Consequences Settle (Priority: P1)

Players do not receive the next turn while a fired shell is still authoritatively changing a
living tank's position. Once settling finishes, the next turn begins normally from the final tank
positions.

**Why this priority**: A player must never aim or move against a half-resolved battlefield state.

**Independent Test**: Resolve an impact that removes support beneath one tank and verify that the
firing turn remains resolving for every falling step, then advances exactly once after all living
affected tanks have settled.

**Acceptance Scenarios**:

1. **Given** a fired projectile impacts and leaves a living tank unsupported, **When** projectile,
   damage, and terrain deformation have resolved but that tank is still falling, **Then** no player
   can make an ordinary move, aim, or fire action and the current player does not change.
2. **Given** every affected living tank has reached supported terrain, **When** match outcome is
   evaluated, **Then** a continuing duel advances to the next eligible player exactly once.
3. **Given** an impact does not leave any living tank unsupported, **When** its terrain and damage
   consequences resolve, **Then** turn handoff retains the existing immediate post-resolution
   behaviour.
4. **Given** an impact eliminates a tank, **When** the duel result is evaluated, **Then** the
   eliminated tank does not require wreck or post-elimination falling simulation before a winner
   or draw is decided.

---

### User Story 3 - Keep Playing From the Settled Position (Priority: P2)

After terrain-induced settling, a surviving tank remains a normal participant: it can later move
and fire from its final grounded position while retaining its player's selected aiming values.

**Why this priority**: Settling must create a real positional consequence rather than a visual-only
correction.

**Independent Test**: Settle a tank after deformation, retain its aim values, then derive a later
movement destination and firing origin; verify both use the settled base position and current
terrain.

**Acceptance Scenarios**:

1. **Given** a tank has settled into a crater, **When** it next takes a movement turn, **Then**
   terrain passability and its movement destination are evaluated from the settled position.
2. **Given** a tank has settled, **When** its owner later fires without changing aim values,
   **Then** the shot launches from the settled tank position while preserving the stored azimuth,
   elevation, and power values.
3. **Given** repeated impacts lower the terrain beneath a surviving tank on separate shots,
   **When** each impact resolves, **Then** each support loss can cause a further deterministic
   descent to the latest terrain surface.

### Edge Cases

- A tank at the crater centre, a tank near a crater rim, and overlapping craters all use the same
  current-terrain support query; no crater-specific movement rule exists.
- If terrain is at or above a tank's base after a change, the tank is corrected to the current
  surface without remaining embedded. If it is below the base by no more than the support
  tolerance, the tank remains stable and is snapped to the surface to remove numerical drift.
- A zero-gravity battlefield cannot create an infinite or frame-rate-dependent resolution loop:
  a newly unsupported tank remains explicitly unresolved and ordinary turn advancement remains
  blocked until an explicit future environment policy handles that configuration. The development
  battlefield's positive gravity remains fully playable.
- Support and contact queries only use in-bounds battlefield coordinates. Existing tank positions
  remain in bounds, so settling cannot move a tank horizontally out of bounds.
- Falling is vertical only. A steep crater wall may make a later deliberate movement request
  impassable, but it does not cause slope sliding, tumbling, or horizontal displacement here.
- Explosion damage is applied once from the impact before terrain-induced settling. Settling adds
  no fall damage, duplicate explosion damage, blast impulse, or terrain-cover calculation.
- Camera and boom lifetime may continue visually while gameplay settles, but neither presentation
  timing nor camera completion changes support, falling, contact, damage, match result, or turn
  advancement.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The current deformable battlefield surface MUST remain the sole authoritative source
  of tank support and contact height. Support, falling, movement grounding, and firing position
  MUST not use a separate terrain representation or stale pre-impact heights.
- **FR-002**: Each living tank MUST use its authoritative base horizontal position as the initial
  support point. A base is supported when its vertical separation from the current terrain surface
  is at most an explicit, tuneable 0.05-world-unit support tolerance; this first model does not
  require wheel, track, or multi-contact simulation.
- **FR-003**: After every authoritative terrain deformation, the game MUST re-evaluate support for
  every living tank. A tank whose current terrain surface remains within support tolerance MUST
  remain stable; terrain changes elsewhere MUST not reposition it.
- **FR-004**: When terrain below a living tank's support point is lower by more than the support
  tolerance, the tank MUST enter an explicit unsupported/settling state and descend vertically
  under the existing configurable battlefield gravity.
- **FR-005**: Settling MUST advance through explicit deterministic simulation steps independent of
  render-frame duration. Given identical initial tank state, terrain state, gravity, and number of
  simulation steps, it MUST produce identical vertical positions, velocities, and settled result.
- **FR-006**: A falling tank MUST not cross below the current terrain surface. On contact it MUST
  be placed at the current terrain height, clear its falling state and vertical velocity, and
  become stable. Terrain that rises into a tank MUST likewise correct its base to that current
  height rather than leave it embedded.
- **FR-007**: The initial settling model MUST use only vertical gravity response. It MUST preserve
  each tank's horizontal position and existing horizontal body-facing direction; it MUST NOT add
  vehicle suspension, traction, sliding, tumbling, rolling, blast knockback, or terrain-collapse
  physics. The existing placeholder's horizontal body orientation is the sufficient initial
  resting orientation.
- **FR-008**: A firing turn MUST resolve in this authoritative order: projectile flight; terrain
  impact and one existing explosion-damage evaluation; terrain deformation; living-tank support
  evaluation and any required settling; survivor/match evaluation; then either turn handoff or
  finished match. An out-of-bounds shot retains its existing no-deformation completion path.
- **FR-009**: Ordinary aiming, movement, firing, and turn advancement MUST remain unavailable
  while a fired turn has unsettled living tanks. Tanks unaffected by the impact MUST not create a
  settling delay. Presentation state MUST never delay or complete this gameplay resolution.
- **FR-010**: Eliminated tanks MUST remain excluded from settling and from any new wreck physics.
  Existing elimination, winner, and draw rules continue to use authoritative explosion damage;
  this feature adds no fall or terrain-related damage.
- **FR-011**: Once a living tank settles, its visible base, authoritative position, future movement
  origin, and future firing origin MUST agree with the final current-terrain position. Its stored
  azimuth, elevation, and power MUST remain unchanged, so only the launch location disrupts an
  established firing solution.
- **FR-012**: The tank descent MUST be visibly understandable in normal play rather than an
  unexplained instant relocation, while its timing remains authoritative and independent of camera
  or render-frame timing. No dedicated fall camera, camera gate, or cinematic sequence is required.
- **FR-013**: Automated gameplay tests MUST cover stable support, remote deformation, support loss,
  gravity advancement, safe zero-gravity handling, terrain contact/no penetration, repeated
  settling, current-terrain usage, retained aiming, updated firing origin, eliminated-tank policy,
  blocked turn handoff, and deterministic identical scenarios without renderer or camera tests.
- **FR-014**: Documentation MUST explain the support-point rule, gravity/settling behaviour,
  resolution order, no-fall-damage decision, and resulting firing-origin behaviour. The workspace
  build, relevant tests, formatting, and linting MUST pass without unjustified warnings. Roadmap
  checkboxes MUST be updated only for behaviour actually delivered.

### Key Entities

- **Tank support state**: A living tank's base position, supported or falling status, and only the
  transient vertical motion necessary to reach valid terrain contact.
- **Support query**: The comparison of a tank base with the current authoritative terrain height
  at the same horizontal location, using the explicit tolerance.
- **Settling result**: The stable tank state produced when vertical descent contacts the current
  deformed surface.
- **Resolving fire turn**: The existing firing action extended to include any authoritative tank
  settling before survivor selection and turn handoff.
- **Battlefield gravity**: The existing explicit gameplay acceleration that governs both projectile
  motion and this feature's vertical tank descent.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In automated fixed scenarios, 100% of tanks supported within the 0.05-unit tolerance
  remain stationary after deformation, and 100% of tanks whose terrain is lowered beyond it end
  exactly on the final queried terrain height without an underground state.
- **SC-002**: In identical automated terrain, gravity, and fixed-step traces, 100% of tank vertical
  position, velocity, support status, and final turn state results are identical regardless of
  render-frame timing.
- **SC-003**: In manual play, a shell that removes terrain directly beneath a visible surviving tank
  produces an observable descent into the crater, while a shell that deforms terrain away from that
  tank does not move it.
- **SC-004**: In automated firing-resolution scenarios with settling, no player can take an ordinary
  action before all living affected tanks settle; after settlement, a continuing match hands off
  exactly once or reports its existing winner/draw result.
- **SC-005**: In a manual follow-up turn after settling, the affected tank can move and fire from
  its new location with unchanged stored aim values, and its projectile visibly begins from that
  settled location.
- **SC-006**: Workspace build, relevant automated tests, formatting, and linting complete without
  unjustified warnings, and no existing projectile, damage, elimination, victory, movement, or
  presentation acceptance behaviour regresses.

## Assumptions

- The current two-player game has no tank footprint or terrain-normal body-tilt representation.
  Its authoritative base point is therefore the smallest readable support model; a later feature
  may revisit footprint support or slope alignment only if play demonstrates a need.
- The existing fixed 1/120-second projectile simulation cadence is the appropriate initial
  deterministic cadence for settling, but the implementation may keep its small settling logic
  separate rather than forcing a speculative shared physics abstraction.
- The existing positive development battlefield gravity is the normal playable configuration.
  Zero gravity is a valid safe edge case but deliberately requires an explicit future policy before
  it can resolve a newly unsupported tank into a continuing turn.
- Fall damage is deliberately excluded. Falling into a crater is already tactically meaningful
  because it changes position, future movement choices, and firing origin without erasing aim.
- This feature depends on current terrain height queries, crater deformation, tank pose/firing
  representation, fixed projectile resolution, two-player survival rules, and the
  presentation-only camera. It does not add a physics engine, generic terrain-response framework,
  new weapon, direct tank collision, or camera cinematic.

## Roadmap Alignment

- On completion, check **Terrain Deformation / Craters — Update tank positioning where terrain
  changes underneath them** and **Turn System / Turn Resolution — Complete resulting tank
  displacement/destruction before turn advancement** if the delivered behaviour satisfies this
  specification.
- Record the deliberate no-fall-damage decision while leaving **Tanks / Damage / Survival —
  Determine fall/terrain-related damage if appropriate** unchecked.
- Do not mark **Gravity / Gameplay Investigation — Evaluate whether gravity should affect
  movement** complete solely because gravity affects involuntary vertical settling; deliberate
  tactical movement remains a separate design question.
- Do not mark crater tactical-effect items complete unless play verifies their broader tactical
  purpose beyond this support response.
