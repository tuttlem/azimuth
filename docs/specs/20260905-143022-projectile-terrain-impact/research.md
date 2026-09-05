# Research: Projectile Terrain Impact Detection

## Decision: Make the rendered triangle grid authoritative

The current battlefield samples a smooth authored elevation expression at a 20-by-20 grid and
renders triangles between those samples. Its current height query evaluates that smooth expression
directly, so it can differ from the visible triangle surface between vertices. Retain the expression
as the deterministic vertex-sampling source, but make local height queries use the same piecewise
planar grid triangles and diagonals as the mesh.

**Rationale**: Projectile impacts and tank placement then agree with what a player sees, without
replacing established terrain or introducing a general terrain system.

**Alternatives considered**:

- Use the smooth height query for collision: rejected because markers can float above or sink into
  the rendered surface.
- Increase mesh resolution: rejected because it masks two competing definitions and does not
  guarantee surface agreement.
- Add a physics engine or terrain framework: rejected because one static height surface has no
  demonstrated need for either.

## Decision: Test each fixed-step segment for an above-to-below crossing

Calculate the existing kinematic candidate position, compare terrain separation at the old and
candidate positions, and process an in-bounds crossing when the old point is above and the new
point is on/below terrain.

**Rationale**: A projectile can travel through ground between 1/120-second endpoints. Segment
testing prevents this tunnelling while preserving the established timestep and ballistic model.

**Alternatives considered**:

- Test only the candidate point: rejected because it loses a useful impact location and permits
  visibly through-ground high-speed shots.
- Globally reduce timestep or universally substep: rejected because it changes established flight
  behaviour when one local segment test solves the present problem.
- Build reusable continuous collision detection: rejected because only one projectile and one
  static terrain surface are in scope.

## Decision: Use fixed-count bisection for contact refinement

For a crossing, bisect the old-to-candidate segment 24 times. Classify
each midpoint by authoritative terrain separation and retain the above/below bracketing interval.

**Rationale**: This is direct, stable for the intended crossing, deterministic for identical
inputs, and accurate enough for the 0.01-world-unit marker/explosion handoff without complex
curve-versus-mesh intersection.

**Alternatives considered**:

- Interpolate endpoint separation once: rejected because terrain changes along X/Z and can place
  a non-flat impact visibly off surface.
- Exact curve against every triangle: rejected because it adds geometry complexity without current
  gameplay value.
- Tolerance-driven unbounded iteration: rejected because iteration counts can vary; fixed work
  makes the simulation boundary explicit.

## Decision: Return a concrete advancement outcome

Replace the active boolean with three direct outcomes: active flight, terrain impact with world
position, and non-impact termination.

**Rationale**: The application must retain, mark, or simply remove the projectile. This expresses
the current observable outcomes without a generic event bus or lifecycle framework.

**Alternatives considered**:

- Infer impact from an absent projectile: rejected because out-of-bounds and terrain impact become
  indistinguishable.
- Add generic collision events: rejected because one projectile and one consumer need a direct
  value, not infrastructure.

## Decision: Retain only the latest presentation-only impact marker

Store the latest terrain impact alongside the optional active flight. The scene uses it to manage
one tagged primitive marker and clears it when a new development shot launches.

**Rationale**: One marker answers the current inspection question and matches the single-shot
workflow; simulation is unchanged if the visual is removed.

**Alternatives considered**:

- Explosion effects: deferred to the next feature.
- Marker history: rejected because no replay or diagnostics use case exists.
- Render-side hit testing: rejected because it violates presentation-independent simulation.
