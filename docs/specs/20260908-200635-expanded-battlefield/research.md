# Phase 0 Research: Expanded Battlefield

## Decision: Use a 120-unit square with a 64-cell terrain grid

**Decision**: Set `HALF_EXTENT` to 60 and use 64 cells per side (65 by 65 vertices, 1.875-unit cells).

**Rationale**: The current arena is 40 units wide with 20 cells. Current aiming reaches a maximum speed of 30 under gravity 8, yielding an approximately 112.5-unit no-wind 45-degree range. A 120-unit world makes long shots, mountains, and 8-player spacing meaningful without requiring a weapon redesign. A 65-vertex edge is the specification’s 4,225-vertex ceiling and keeps the 4-unit Basic Shell and 6-unit HE craters readable, while avoiding a much denser proportional mesh.

**Alternatives considered**:

- Keep 20 cells over 120 units: rejected because 6-unit cells make one-unit movement and current craters too coarse.
- Increase density proportional to map area or beyond 65 vertices: rejected because it increases crater refresh work without a demonstrated gameplay need.
- Change weapon velocity/gravity first: rejected because existing maximum range already supports representative engagements; test first and change only a named global limit if evidence requires it.

## Decision: Compose bounded macro geography with separate local variation

**Decision**: Generate a positive baseline around 6, one broad side mountain (approximately +18 to +22), one rotated elongated ridge (+5 to +8), and one broad valley/bowl (approximately -9 to -12), each with smooth bounded falloff. Add only a few seed-derived low-amplitude local waves/depressions (combined amplitude at most about 2, wavelengths about 10–24 units).

**Rationale**: The current authored sine/cosine bumps have no independent macro form. Deliberate broad, smooth forms guarantee recognisable geography, a sampled elevation span above 24 units, and flatter land for spawning. Small variation makes slopes irregular without turning mountains into spikes or merely repeating the old pattern.

**Alternatives considered**:

- Increase amplitude of the existing wave: rejected because it produces chaotic slopes rather than tactically readable mountains and valleys.
- Generic octave-noise/biome framework: rejected as unnecessary scope and architecture for one terrain style.
- Fully random feature positions without bounds: rejected because it can erase dry, moderate-slope spawn areas.

## Decision: Water table at world elevation zero with continuous colour stops

**Decision**: Use one central water table at `y = 0`. Blend terrain colours by current absolute elevation through approximate stops: lowland green immediately above water, grass around 4–11, earth/rock around 11–25, and snow at/above 25. Terrain below zero is still coloured safely but visually covered by water.

**Rationale**: A +6 baseline preserves broad dry land while the bounded bowl reliably creates visible lowland water. Colour derived during each terrain-mesh refresh means crater floors naturally use their new elevation colour and may appear flooded without any water simulation.

**Alternatives considered**:

- Paint low terrain blue: rejected because it does not show a flat surface over submerged terrain.
- Per-biome or texture-driven materials: rejected because height alone meets the readability goal.
- Dynamic water fill: rejected because a global water plane gives the desired visual result with no physics/state model.

## Decision: Capture a master seed and derive isolated labelled streams

**Decision**: Introduce a captured `BattlefieldSeed(u64)` at match construction. Use a small pure deterministic derivation with distinct labels for terrain, starts, dressing, and wind; feed `BattlefieldTerrain::generated`, spawn selection, dressing generation, and `wind_from_seed` separately. Log or otherwise expose the captured seed for reproducible diagnostics, but add no setup UI.

**Rationale**: The project currently time-samples only wind and already has a pure `wind_from_seed`. Independent labelled streams ensure a visual building-placement change cannot alter authoritative terrain, starts, or wind. The seed is captured once, so rendering frame timing has no influence.

**Alternatives considered**:

- One shared mutable random stream: rejected because call-order changes would couple presentation to gameplay conditions.
- Time-derived terrain without storage: rejected because failures and attractive battlefields would not reproduce.
- External random/procedural dependency: rejected because the existing small deterministic arithmetic is sufficient.

## Decision: Use bounded deterministic distributed start selection

**Decision**: Replace the fixed eight-position array with a deterministic candidate scan/traversal across broad map sectors. A candidate must be in a safe in-bounds margin, finite, above water, have each one-unit cardinal neighbour within the existing 0.75 elevation-change allowance, and be at least 18 horizontal units from every accepted start. Select one candidate per sector before filling remaining places; face starts toward map centre or a deterministic opponent-facing direction.

**Rationale**: It rejects obvious invalid starts and visibly uses the expanded landscape while preserving current base-point support and movement semantics. A fixed candidate lattice and bounded attempt count make failure diagnosable and repeatable without claiming line-of-sight or tactical fairness.

**Alternatives considered**:

- Scale the old fixed array: rejected because it is not terrain-aware or seed-reproducible and does not use varied geography reliably.
- Tactical score/trajectory fairness solver: rejected as the separately-roadmapped Fair Randomized Starts feature.
- Free unbounded random retries: rejected because it harms deterministic tests and can hang on bad terrain.

## Decision: Keep water/buildings outside authority and generate dressing after starts

**Decision**: Represent water and building placements as presentation data only. Place simple cuboids only on dry, locally suitable terrain and outside an approximately 8-unit initial-start exclusion zone. Generate those records after start selection and never pass them to terrain, projectile, tank, combat, or turn code.

**Rationale**: This provides readable settlement cues without accidental collision or implied cover. Separating records from gameplay data makes the authority boundary obvious in code and tests.

**Alternatives considered**:

- Colliding/destructible structures: rejected as a future authoritative-structures feature.
- Generic prop system: rejected because buildings are the only accepted dressing type.
- Buildings before starts: rejected because visual placement could block otherwise valid starts or affect deterministic selection.

## Decision: Use named map-aware simulation and camera limits

**Decision**: Replace the development-only useful-volume constant with a named battlefield limit based on the new 60-unit half extent, with a small intentional horizontal margin only if needed to let a boundary-crossing trajectory resolve predictably. Keep fixed-step/bisection logic unchanged. Raise the vertical ceiling only enough for the current 30-speed high arc plus mountain elevation. Increase shot overview and maximum camera range deliberately; retain the active-player close view and make a modest explicit far clip sufficient for the map and peaks.

**Rationale**: The existing horizontal projectile limit of 60 and max camera distance of 60 belong to the old arena. The map must remain visible and projectiles must still reach terrain close to edges, but unbounded limits would worsen readability and depth precision.

**Alternatives considered**:

- Keep development limits: rejected because shots leave the useful volume at the new boundary.
- Set very large universal limits/clipping: rejected because it hides map-scale mistakes and risks depth/readability issues.
- Redesign the camera: rejected because adjusting existing poses/ranges satisfies the feature.

## Integration Evidence

- `battlefield.rs` already stores mutable vertex heights, triangle-interpolates current terrain, applies craters directly, and supplies render positions; it is the correct authority to extend.
- `tank.rs` currently grounds tanks with the terrain query but starts from an eight-entry fixed-position array; that is the smallest replacement point for validity-aware starts.
- `main.rs` currently constructs terrain before Match Setup, reuses it when Enter starts a match, samples only wind from time, creates a one-material terrain mesh, and refreshes it on terrain change. It must rebuild the seeded battlefield, starts, and presentation at the actual match-start boundary.
- `projectile.rs` currently uses a development horizontal extent of 60 and a vertical maximum of 100. The same swept terrain test can remain intact when given the named map limit.
