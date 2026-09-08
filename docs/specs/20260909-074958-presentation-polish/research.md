# Phase 0 Research: Presentation Polish

## Decision: Project the scoreboard from configured-player order

**Decision**: Add a pure scoreboard-entry projection that iterates `MatchConfiguration.players`
in stored order, joins each stable `PlayerId` to its tank, and derives display name, colour
identity, health, eliminated state, and active status. Make `TacticalHudView` consume the resulting
ordered collection instead of fixed Player One/Player Two fields.

**Rationale**: `MatchConfiguration.players` is the existing canonical source for configured names
and order; tanks supply live condition, and `TurnState` supplies active status. A pure projection
keeps the one-way gameplay-to-presentation boundary visible and gives focused non-pixel tests for
2, 3, and 8 players.

**Alternatives considered**:

- Iterate tanks as the display collection: rejected because the configured collection owns the
  required match order and display names.
- Keep eight fixed optional HUD fields: rejected because it duplicates the existing two-player
  assumption and makes count changes fragile.
- Store scoreboard condition in HUD resources: rejected because state can become stale or mutate
  gameplay ownership.

## Decision: Retain one HUD tree with rows created for the accepted match configuration

**Decision**: Keep the current full-screen HUD and top-left panel. At match start, create one
tagged player row and nested health fill per configured player; sync those retained rows by stable
`PlayerId`. Use a small count-based row density rule within that panel only, with the current
two-player density retained for a duel and a tighter readable density for larger matches.

**Rationale**: Match setup can change player count before a match begins, while the HUD is currently
created before setup is accepted. Building the scoreboard rows at accepted match start avoids
per-frame entity churn and covers the selected player count without a generic responsive framework.

**Alternatives considered**:

- Rebuild scoreboard UI every update: rejected as needless entity churn.
- Build maximum rows at startup and hide extras: rejected because it retains a hard-coded maximum
  UI layout and undermines collection-derived presentation.
- Redesign all HUD panels around responsive layout: rejected as broader UI work outside the feature.

## Decision: Use clear blue background, static cloud clusters, and one visual-horizon mesh

**Decision**: Set the scene’s clear colour to the selected blue-sky tone; add a small fixed set of
non-interactive simple white cloud clusters; generate one low-detail `HorizonLandscape` mesh that
surrounds the 120-unit battlefield and joins its inner perimeter to the authoritative terrain’s
current edge heights and elevation palette.

**Rationale**: The existing scene already creates meshes, untextured materials, lights, water, and
presentation-only buildings. A clear colour supplies the sky at effectively no geometry cost;
simple clouds provide readable atmosphere; one static ring/outer landscape masks the board edge
without new assets, a sky system, or high-density terrain.

**Alternatives considered**:

- Skybox/skydome assets: rejected because an asset pipeline is unnecessary for the requested
  simple blue sky and clouds.
- Volumetric clouds, dynamic layers, weather, or scattering: rejected as explicit out-of-scope
  environment systems.
- Expanding the authoritative terrain mesh: rejected because it would silently expand the gameplay
  surface and increase mutable mesh/deformation work.

## Decision: Keep the horizon descriptor engine-independent and non-authoritative

**Decision**: Define an engine-independent `HorizonLandscape` data value in the terrain module
with only render-ready positions, colours, and indices (or equivalent immutable mesh data). Its
construction samples the authoritative terrain boundary to weld the inner ring and evaluates the
existing deterministic generated-height rule from the captured terrain seed for the exterior.
It exposes no height query, crater, collision, spawning, movement, or support operation.

**Rationale**: The terrain module already owns height sampling, height colour mapping, and mesh
index conventions. Keeping the descriptor pure lets unit tests demonstrate continuity and that
the existing `is_within_bounds`/`height_if_within_bounds` contract has not expanded. `main.rs`
alone converts it into an engine mesh.

**Alternatives considered**:

- A second mutable `BattlefieldTerrain`: rejected because it risks gameplay authority leakage and
  live deformation duplication.
- A large flat skirt: rejected because it would visibly disconnect from the varied edge terrain.
- Copy/update horizon heights after every crater: rejected because the feature needs static distant
  scenery, not a second deformation surface; the inner seam is built from the initial current edge.

## Decision: Use only minimal camera safeguards after visual inspection

**Decision**: Retain the active-player and shot camera intents, mouse orbit, and wheel zoom, but
replace the symmetric orbit pitch cap with separate bounds whose maximum remains slightly downward
or near-horizontal. Keep the existing far clip initially; adjust it only if manual validation
finds a normal permitted view that still exposes the void or horizon cutoff.

**Rationale**: The current camera already frames an expanded 120-unit battlefield and uses a
300-unit far clip, but its symmetric pitch range can put it below the single-sided terrain. The
normal active and shot poses are already downward, so a narrow upper-pitch safety cap preserves
intended inspection while preventing the obvious underside view without a camera redesign.

**Alternatives considered**:

- Prohibit user orbit/zoom: rejected because it removes useful established tactical inspection.
- Add obstruction-aware navigation or a camera director: rejected as unrelated camera-system work.

## Decision: Test observable pure presentation state and authority boundaries, not pixels

**Decision**: Unit-test scoreboard entry count/order/names/health/elimination/active state for
2, 3, and 8-player configurations. Unit-test visual-horizon initialization, finite geometry,
edge continuity, and unchanged authoritative bounds/query rejection outside the battlefield. Use
manual 2-, 4-, and 8-player graphical runs for composition, clouds, seam quality, and performance.

**Rationale**: The pure projection and pure mesh descriptor are stable behavioral seams. Pixel or
renderer entity-coordinate tests would be brittle and would not better demonstrate that the visual
world does not own gameplay.

**Alternatives considered**:

- Pixel snapshots: rejected as renderer/font/platform-sensitive.
- No automated tests for presentation: rejected because collection correctness and authority
  boundaries are important deterministic regressions.
