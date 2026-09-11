# Research: Battlefield Visual Polish

## Decision: Keep tank variety wholly presentation-owned

**Decision**: Define five primitive visual recipes—Classic, Heavy, Low Profile, Compact, and
Angular—and a presentation-only assignment from `PlayerId` to recipe. Use an isolated match-seed
derivation with a cosmetic label; assign distinct recipes first, then repeat.

**Rationale**: `Tank` currently contains authoritative owner, pose, health, and support state; its
firing representation supplies the canonical direction and muzzle position. Keeping recipes outside
that domain type preserves movement, support, damage, AI, and ballistic conventions.

**Alternatives considered**: Putting model selection or barrel length on `Tank` was rejected as
cosmetic authority leakage. External detailed models were rejected for asset/provenance cost. Tank
classes or selection were rejected because they change gameplay.

## Decision: Reuse current primitive hierarchy and canonical muzzle marker

**Decision**: Keep the current visual root plus turret, barrel, and muzzle roles, but assemble
recipes from shared primitive hull, track, turret, and barrel parts. Place the canonical muzzle
marker from the firing representation and fit visual barrels to it.

**Rationale**: Existing pose and aim sync already update these roles from tank/aim state. Recipe
local transforms create silhouettes without changing the gameplay firing origin.

**Alternatives considered**: Per-model ballistic/muzzle calculations and skeletal/track animation
were rejected because they either alter physics or exceed the visual goal.

## Decision: Use ordinary standard materials with no new renderer work

**Decision**: Use current standard materials for player-painted armour, dark mechanics, particles,
smoke, and water. Armour uses moderate metallic response/roughness; water is opaque and lower
roughness; smoke uses translucent dark puffs.

**Rationale**: Resolved Bevy 0.18.1 materials support base colour/texture, metallic response,
roughness, emissive colour, and alpha modes. The game already uses this path and directional
lighting, which is sufficient without custom shaders.

**Alternatives considered**: A custom terrain shader, splat mapping, water transparency/refraction,
reflections, animation, or a texture pack were rejected as unnecessary risk or scope.

## Decision: Preserve height-derived terrain colour and add restrained surface detail

**Decision**: Keep live height-derived terrain colours. Add modest deterministic world
coordinate/height mottling and, only if needed after visual review, one tiny generated neutral
repeat texture with world-scale terrain UVs. Keep water separate and horizon simple.

**Rationale**: The authoritative terrain already regenerates its render mesh after craters/mounds;
presentation from the same current surface prevents stale crater colour without changing height,
collision, or queries.

**Alternatives considered**: Decals, soil layering, dynamic dirt, separate rock geometry, and
material-dependent terrain physics were rejected as new systems outside scope.

## Decision: Use a one-way capped impact-presentation queue

**Decision**: Add a renderer-owned queue of compact requests containing impact position, existing
visual scale, and cosmetic serial. Append one after every resolved terrain impact, including split
children. Retain latest-impact state for marker, flash, audio, and camera.

**Rationale**: Latest-impact state is overwritten by same-tick MIRV, Cluster Bomb, and Bomb Net
impacts, so it cannot represent every transient request. A one-way queue observes results only and
is never used by simulation.

**Alternatives considered**: Reusing only latest impact loses barrage effects; fixed-step effect
simulation couples presentation to authority; authoritative debris/smoke violates scope.

## Decision: Spawn capped primitive particles and smoke puffs

**Decision**: Reuse small meshes and fixed material handles. Spawn hot/dirt particles with
cosmetic outward/upward velocity, visual gravity, fade/shrink, and finite life. Spawn flattened
translucent smoke puffs that rise, expand, fade, and optionally read wind. Cap queue at 64, active
particles at 128, active smoke puffs at 32, one impact at 32 particles/eight puffs.

**Rationale**: This matches the existing explosion lifecycle. Scale-based budgets naturally vary
Basic, HE, child explosions, and Nuke while safely bounding a 16-child Bomb Net.

**Alternatives considered**: GPU/general particle frameworks, VFX graph, custom shader, atlas,
camera-facing billboards, and unbounded spawning were rejected as disproportionate.

## Decision: Use pure cosmetic variation and Update-time effect motion

**Decision**: Hash request serial and particle index for visual variation, and advance effects in
ordinary presentation Update. Wind is read only for optional gentle smoke drift.

**Rationale**: Existing labelled seed streams isolate dressing from terrain, starts, wind, and AI.
Pure hashing is simpler than mutable presentation RNG and cannot disturb fixed-step resolution.

**Alternatives considered**: Consuming gameplay/AI RNG, exact wind physics, and fluid simulation
were rejected for determinism or scope.

## Test Strategy

- Test 2–8-player assignment, unique-first selection, permitted duplicates, stable player state,
  and isolated cosmetic derivation.
- Test recipe attachment relations and canonical muzzle transforms, not mesh vertices or pixels.
- Test terrain presentation refresh after craters/mounds without changing terrain state.
- Test profile monotonicity, split-impact recording, caps, motion/fade/expiry, and reset cleanup.
- Compare authoritative trajectory, impact, damage, crater/mound, settling, and turn results with
  visual presentation enabled and disabled.
