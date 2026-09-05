# Research: First Crater — Terrain Deformation

## Decision: Make existing grid heights mutable authoritative state

Store the existing 21-by-21 sampled grid elevations in one terrain value. Interpolate its fixed
triangles for local height and generate visible mesh positions from those same elevations.

**Rationale**: The current terrain already has fixed triangle topology and all consumers agree
through interpolation. Retaining that topology while making only heights mutable gives rendering,
collision, and queries one source without replacing the battlefield.

**Alternatives considered**:

- Separate visual crater data or decal: rejected because collision and queries would remain stale.
- A physics-engine height field or general collision system: rejected because the existing swept
  query is adequate and a second terrain representation would violate the source-of-truth boundary.
- Voxels, terrain chunks, or procedural regeneration: rejected because one compact grid is enough.

## Decision: Lower grid vertices with a smooth radial bowl

For each vertex within radius `r`, lower its current height by `depth * (1 - d² / r²)²`, where
`d` is horizontal distance from the authoritative impact. Vertices at or outside the radius receive
zero lowering.

**Rationale**: The profile has maximum depth at the centre, a smooth zero-height edge, simple
bounded arithmetic, and naturally composes by subtracting from current values. Applying it at
vertices preserves the existing piecewise-triangle terrain representation.

**Alternatives considered**:

- Hard-edged cylinder: rejected because abrupt grid discontinuities are less readable.
- Physically based excavation, debris, or deposition: rejected as unnecessary simulation.
- Noise or irregularity: rejected because it complicates deterministic regression coverage.

## Decision: Apply crater immediately from fixed-step impact

Apply the default crater in the fixed simulation update that receives a resolved terrain impact.
The boom continues to observe the persistent impact result.

**Rationale**: The projectile resolves contact on the pre-crater surface; immediate application
establishes the new surface for subsequent steps without asking presentation to gate gameplay.

**Alternatives considered**:

- Wait for boom expiry: rejected because presentation timing must not gate gameplay terrain.
- Re-detect collision in presentation: rejected because simulation is authoritative.
- Add a generic impact event bus: rejected because the direct single-shot handoff is sufficient.

## Decision: Refresh the compact mesh only after terrain changes

After a crater mutates terrain, rebuild the battlefield mesh from current positions and fixed
indices. Do no mesh work on frames without deformation.

**Rationale**: The battlefield has only 441 vertices and 800 triangles. One impact-triggered
refresh is direct and simpler than in-place GPU updates or mesh streaming.

**Alternatives considered**:

- Per-frame mesh regeneration: rejected because terrain changes only at impact.
- Chunking, level of detail, compute shaders, or GPU deformation: rejected for lack of measured need.
