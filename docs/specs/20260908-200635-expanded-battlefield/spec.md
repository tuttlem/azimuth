# Feature Specification: Expanded Battlefield — Mountains, Valleys, Terrain Colour and World Dressing

**Feature Branch**: `20260908-200635-expanded-battlefield`

**Created**: 2026-09-08

**Status**: Draft

**Input**: Transform the small development arena into one substantially larger deterministic Azimuth battlefield with readable macro geography, elevation colour, presentation-only water and sparse primitive buildings, while preserving the playable 2–8 human match.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Read and Fight Across a Real Landscape (Priority: P1)

As a player starting a local match, I see a broad artillery landscape with mountains, ridges, valleys, bowls, lowlands, and flatter ground, so elevation and terrain shape affect where I move and how I aim.

**Why this priority**: The enlarged, strategically meaningful battlefield is the feature’s core player value; its geography must change artillery choices rather than merely decorate the old arena.

**Independent Test**: Start matches from several recorded terrain seeds, inspect the whole battlefield, and fire ordinary shells across short and long routes. Confirm that each inspected battlefield is larger than the former arena, contains readable macro geography and usable flatter ground, and that at least representative shots require a higher arc or altered aiming because of terrain.

**Acceptance Scenarios**:

1. **Given** a new valid human match, **When** the battlefield is first shown, **Then** it spans one 120-by-120-unit square centred on the existing world origin, rather than the former 40-by-40 development arena.
2. **Given** any generated battlefield, **When** a player views it from an appropriate general or active-player camera position, **Then** terrain includes broad elevation structure distinct from local rolling variation and remains visually readable at useful artillery distances.
3. **Given** a representative generated ridge, mountain, bowl, or valley, **When** an existing shell is fired through the relevant area, **Then** the existing terrain collision and ballistic rules make the feature part of the shot geometry without a bespoke terrain rule.
4. **Given** the same battlefield seed and relevant configuration, **When** matches are generated repeatedly, **Then** their authoritative terrain heights and gameplay terrain queries reproduce; a different tested seed produces a meaningfully different terrain sample or macro arrangement.

---

### User Story 2 - Read Elevation, Water, and Dressing at a Glance (Priority: P1)

As a player, I can quickly understand high ground, rocky slopes, green lower ground, flooded low areas, and sparse settlements from the battlefield’s visual language.

**Why this priority**: Elevation colour, visible water, and restrained dressing turn tactical terrain into a recognisable Azimuth landscape without adding unrelated simulation.

**Independent Test**: Inspect multiple seeded battlefields before and after representative craters. Verify coherent height colour, a visible flat water surface over low terrain, and sparse dry building groups; verify that neither water nor buildings alters authoritative movement, terrain, or projectile outcomes.

**Acceptance Scenarios**:

1. **Given** terrain ranging from lowlands to peaks, **When** it is rendered, **Then** the colour progression is smoothly readable from lowland green through grass/earth and rock to snow-like high elevations, without invalid or abrupt accidental bands.
2. **Given** terrain below the centrally defined water-table elevation, **When** it is rendered, **Then** a simple flat blue water surface visibly covers the low area while the terrain beneath remains the authoritative surface.
3. **Given** a crater that lowers terrain across an elevation boundary or below the water table, **When** the ground refreshes, **Then** its terrain colour remains coherent with current height and any newly low depression may visibly sit below the unchanged water plane.
4. **Given** a generated landscape containing suitable dry, comparatively flat patches, **When** its dressing is shown, **Then** sparse primitive grey building-like blocks or small clusters sit plausibly on those patches and do not dominate the landscape.
5. **Given** a visible building or water surface, **When** a tank moves or a projectile crosses it, **Then** gameplay continues to use only authoritative terrain; buildings do not collide, take damage, block line of fire, deform, or imply reliable cover, and water has no physics or movement effects.

---

### User Story 3 - Start a Valid Distributed Multiplayer Match (Priority: P1)

As a local player starting a 2-, 4-, or 8-human match, I receive a supported, dry, in-bounds tank position that is separated from other starts and distributed across the expanded landscape.

**Why this priority**: The landscape is only playable if existing multiplayer matches can start reliably and use the new space without putting a tank underwater or on unusable terrain.

**Independent Test**: Generate representative recorded seeds for 2, 4, and 8 players. For every start, verify the requested number of tanks, dry supported terrain, acceptable local slope, bounds, no overlap, and the specified minimum separation; inspect that eight tanks do not remain confined to the old development area.

**Acceptance Scenarios**:

1. **Given** a valid all-human configuration of two through eight players, **When** Start Match is selected, **Then** exactly that many tanks spawn on dry, supported, in-bounds terrain with no overlap and with at least 18 horizontal units between any two starts.
2. **Given** a candidate start on terrain that is underwater, outside bounds, too steep for the existing local movement/support model, or too close to another accepted start, **When** starts are selected, **Then** that candidate is rejected deterministically.
3. **Given** an eight-player match, **When** it begins, **Then** accepted starts use the expanded battlefield rather than clustering in one former development-area region; exact tactical equality is not asserted.
4. **Given** the same match/battlefield seed, player count, and relevant configuration, **When** starting positions are generated again, **Then** the same positions and orientations result without consuming or changing the wind or other gameplay-randomness stream.

---

### User Story 4 - Keep the Existing Artillery Game Playable (Priority: P2)

As a player, I retain understandable aim, wind, movement, projectile impacts, craters, settling, weapons, turn handoff, HUD, and camera behaviour while using the bigger battlefield.

**Why this priority**: The feature earns its scale only if normal weapons still reach useful opponents and every existing terrain-dependent system remains dependable.

**Independent Test**: In representative 2-, 4-, and 8-player seeded matches, fire Basic Shell, High Explosive, and Heavy Shell across short and long distances; create craters at high, low, centre, and edge terrain; then complete turns and verify existing gameplay outcomes.

**Acceptance Scenarios**:

1. **Given** the 120-unit battlefield and the existing maximum launch speed of 30 under the standard gravity of 8, **When** players use ordinary shell power and elevation, **Then** useful long-range engagement remains possible without an automatic weapon-physics redesign; any required global range or limit adjustment is documented and tested.
2. **Given** a terrain impact near a centre, edge, high peak, lowland, or water-table region, **When** the existing crater resolves, **Then** terrain query, visible surface, projectile collision, and tank support/settling remain mutually consistent and safe.
3. **Given** active-player and shot presentation on the larger battlefield, **When** camera framing changes, **Then** the player can understand distant terrain and players while retaining current aiming usability; camera timing remains presentation-only.
4. **Given** any existing weapon inventory, wind condition, movement allowance, turn state, or victory state, **When** the expanded terrain is used, **Then** its current semantics remain intact unless a narrowly documented global projectile-limit adjustment is necessary for ordinary engagement.

### Edge Cases

- Terrain queries at every boundary, including exact corners, remain safe and return a current finite surface height when in bounds; out-of-bounds queries retain their established safe/rejected behaviour.
- Macro terrain may produce low depressions or high peaks, but it must retain enough suitable dry, moderate-slope candidates for every valid 2–8 player match; deterministic selection must fail clearly in development diagnostics rather than silently place an invalid tank.
- Water may cover a crater or existing terrain depression visually, but it never replaces terrain collision or creates water-specific gameplay.
- Building candidates on water, outside bounds, on unsuitable slopes, or within the tank-spawn exclusion distance are omitted; sparse dressing is permitted to be absent for an individual seed only if several inspected seeds still demonstrate it.
- Repeated or edge-adjacent craters retain finite terrain and coherent colour/mesh updates; dressing stays presentation-only and is not required to move, settle, or react.
- A seed’s terrain, starts, and dressing must not change because wind is sampled, a visual system runs, or rendering frame timing differs.
- Existing AI slots remain unavailable and AI behaviour is not introduced.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The game MUST replace the former 40-by-40 development arena with exactly one default, origin-centred 120-by-120-unit battlefield for all current matches. It MUST NOT expose selectable battlefield sizes, maps, terrain styles, or environment presets.
- **FR-002**: The battlefield MUST preserve the established authoritative mutable terrain contract: the same current surface governs terrain height, bounds, tank support/movement/spawning, projectile collision, crater deformation, and the rendered ground.
- **FR-003**: Battlefield generation MUST be deterministic from an explicit reproducible battlefield/match seed and relevant configuration. It MUST keep terrain generation, start selection, and dressing placement deterministic while isolating their random streams from wind and other gameplay randomness.
- **FR-004**: Generated terrain MUST combine independently meaningful broad geography and smaller local variation, producing mountains or large hills, ridges, valleys or lowlands, bowls, and flatter local areas across representative seeds. It MUST NOT be a uniform amplification or repetition of the prior small bump pattern.
- **FR-005**: The physical size and terrain sampling density MUST be deliberately independent. The implementation MUST use no more than 4,225 terrain vertices (a 65-by-65 vertex grid) while maintaining terrain detail sufficient for the existing 4-unit Basic Shell crater, 6-unit High Explosive crater, tank support, and one-unit tactical movement to remain readable and correct.
- **FR-006**: Representative generated terrain MUST have a sampled elevation span of at least 24 units, contain dry moderate-slope candidate areas for 2–8 starts, and avoid non-finite or out-of-bounds terrain data. Exact mountain shapes and elevations are intentionally not prescribed.
- **FR-007**: The game MUST define one central water-table elevation for this battlefield and render one simple, flat, blue water surface across the battlefield at that elevation. Terrain below it remains valid authoritative terrain and is visually submerged; water adds no collision, drag, drowning, buoyancy, splash, underwater, explosion, movement, or shoreline behaviour.
- **FR-008**: The rendered terrain MUST map current absolute world elevation through a bounded, coherent colour progression: snow-like at high elevations, rocky/earth tones below that, grass at middle elevations, richer/darker green in lowlands, and visually distinct water at the water table. Transitions MUST be smooth or intentionally blended rather than accidental hard contour bands.
- **FR-009**: Terrain mesh refresh after every crater MUST derive colour as well as geometry from current authoritative terrain height, so deformation cannot reveal uninitialised colours, material seams, or a stale elevation region.
- **FR-010**: The game MUST generate sparse deterministic presentation-only building-like dressing from the same relevant battlefield seed/configuration. Each structure or cluster MUST be within bounds, dry, grounded on comparatively flat terrain, away from initial tank starts, and visibly simple grey primitive blocks with modest deterministic dimension, height, and orientation variation.
- **FR-011**: Buildings and water MUST be explicit presentation-only elements. They MUST NOT enter authoritative terrain queries, projectile collision, tank collision/support/movement, damage, deformation, explosion, cover, or line-of-fire calculations; player-facing documentation must state this limitation.
- **FR-012**: For every valid human player count from 2 through 8, deterministic start selection MUST return exactly one start per player that is in bounds, dry (terrain height strictly above the water table), supported, no steeper than the existing movement step’s permitted elevation change when sampled locally, non-overlapping, and at least 18 horizontal units from every other accepted start.
- **FR-013**: Start selection MUST distribute starts across the 120-by-120 battlefield rather than reuse fixed positions local to the former 40-by-40 arena. It MAY use deterministic candidate selection and bounded retries, but MUST NOT claim tactical fairness based on line of sight, trajectories, blast reachability, or elevation advantage.
- **FR-014**: The current movement allowance remains a local positional adjustment and MUST NOT be enlarged merely because the battlefield is larger.
- **FR-015**: The existing projectile useful-volume limits, camera distance/framing limits, and far rendering limits MUST be reviewed and adjusted only as needed for the 120-unit battlefield, its elevations, and normal artillery shots. Any changed projectile limit MUST retain fixed-step determinism and be covered by range/bounds tests; weapon identities, gravity, and wind rules remain unchanged unless evidence shows a single modest global adjustment is required.
- **FR-016**: The camera MUST continue to focus active tanks and show a general shot view without controlling authoritative match progression. Its valid target and zoom/range behaviour MUST accommodate the expanded bounds and make distant terrain and players readable without reckless clipping or unnecessary global range inflation.
- **FR-017**: Automated coverage MUST verify same-seed reproducibility for terrain, starts, water-table value, and dressing; different-seed variation; finite/effective elevation range; bounds and edge terrain queries; representative terrain collision; height-colour mapping bounds and ordering; dry/suitable dressing; and valid 2-, 4-, and 8-player starts.
- **FR-018**: Automated regression coverage MUST verify that Basic Shell, High Explosive, Heavy Shell, gravity, wind, impacts, craters, terrain deformation, settling, movement, turn advancement, victory, inventories, HUD, and named multiplayer state continue to work on the expanded battlefield.
- **FR-019**: Completion MUST include manual inspection of multiple recorded terrain seeds and representative 2-, 4-, and 8-player matches. It MUST record visual and gameplay findings in the roadmap, update only roadmap items whose stated acceptance is genuinely demonstrated, and preserve unfinished fair-start, unwinnable-terrain, authored-map, obstacle, and AI work.
- **FR-020**: The implementation and documentation MUST comment the design intent of macro versus local structure, colour thresholds, water-table choice, spawn suitability, deterministic dressing, and the authoritative/presentation-only boundary. It MUST pass the workspace build, relevant tests, formatting, and lint checks without unjustified warnings.

### Key Entities

- **Battlefield definition**: The one 120-by-120-unit, origin-centred playable area and its centrally defined water-table elevation.
- **Authoritative terrain surface**: The mutable height surface shared by gameplay queries, collision, tank support, craters, and ground presentation.
- **Battlefield seed**: The captured reproducibility input from which terrain, start selection, and dressing derive through isolated deterministic streams.
- **Macro terrain structure**: Broad elevation features that determine mountains, ridges, valleys, bowls, and lowlands independently of small local character.
- **Local terrain variation**: Smaller rolling variation that makes macro forms irregular without replacing their strategic shape.
- **Elevation colour mapping**: The bounded visual mapping from current absolute surface height to snow, rock/earth, grass, and lowland colour regions.
- **Water table**: The global visual elevation at which the simple water plane covers low terrain; it is not an authoritative gameplay surface.
- **World dressing placement**: A deterministic sparse set of dry, flat, spawn-clear primitive building presentations.
- **Spawn candidate**: A deterministic terrain location evaluated for bounds, water, slope/support, separation, and dressing exclusion before it becomes a tank start.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Every started match uses the single 120-by-120-unit battlefield, three times the former arena’s width and nine times its area, while ordinary maximum-power 45° artillery has an approximately 112-unit no-wind ground range under the existing standard gravity and can meaningfully engage representative opponents without a weapon-specific range change.
- **SC-002**: For at least five recorded seeds, automated generation reproduces 100% of sampled terrain heights, water-table values, starts, and dressing placements for repeated identical inputs; at least four of those seeds differ from the baseline in one or more sampled heights or dressing positions.
- **SC-003**: Across the same five inspected seeds, each battlefield has a sampled terrain elevation span of at least 24 units, at least one broad high feature and one broad low feature, and at least four accepted dry moderate-slope spawn candidates per player required for its tested player count.
- **SC-004**: In automated 2-, 4-, and 8-player cases across at least five recorded seeds, 100% of requested starts are in bounds, dry, supported, non-overlapping, and at least 18 horizontal units apart.
- **SC-005**: In automated representative elevation inputs, 100% of produced terrain colours are finite and bounded; higher samples progress toward snow/rock, middle samples toward grass/earth, and low dry samples toward lowland green without pixel-testing rendered output.
- **SC-006**: In automated dressing cases across at least five recorded seeds, 100% of placed buildings are in bounds, dry, grounded on suitable terrain, and outside the initial-tank exclusion distance; identical seeds reproduce the same placements.
- **SC-007**: In automated centre, edge, high, low, and water-table-region crater cases, 100% of terrain queries, subsequent terrain impacts, mesh refreshes, and living-tank support reconciliation remain finite, bounded, and consistent with the current surface.
- **SC-008**: Manual inspection of at least five seeds and playable 2-, 4-, and 8-player matches demonstrates readable distant geography, visibly coherent snow/rock/green/water progression, sparse non-authoritative buildings, usable active/shot camera framing, and at least one observed shot decision affected by a ridge, mountain, valley, or elevation difference.
- **SC-009**: Relevant build, test, formatting, and lint checks complete successfully with no new unjustified warnings, and existing projectile, wind, gravity, explosion, deformation, damage, settling, movement, turn, victory, weapon, HUD, and match-setup regressions remain absent.

## Assumptions

- The current coordinate model remains Y-up with X/Z horizontal, origin-centred bounds, and abstract world units.
- The 120-by-120-unit square is selected from current evidence: it is three times the old width, supports geography and an 18-unit start separation for eight players, and remains near the existing 30-speed/8-gravity maximum 45° range of about 112 units. It is one default, not a public size-selection system.
- A terrain grid of at most 65 by 65 vertices is a practical initial upper bound: it grows physical scale substantially without blindly matching the area increase in mesh density. Planning may select a lower resolution if crater, support, and movement tests demonstrate it is sufficient.
- Existing water and building primitives can be rendered with the current scene facilities. No assets, external procedural-generation dependency, generic object layer, material framework, physics system, or world engine is justified by this feature.
- The current time-derived wind seed is reproducible only when recorded by diagnostics. This feature establishes/captures the relevant battlefield/match seed needed to reproduce terrain, starts, and dressing without making a seed-selection UI.
- A building-free individual seed is acceptable only where placement constraints leave no suitable patch; multiple-seed manual acceptance must still demonstrate sparse buildings.
- Start suitability deliberately rejects obvious invalidity, not strategic imbalance. Fair randomized starts, unwinnable-terrain prevention, authored maps, selectable terrain styles, and final terrain quality remain roadmap work.

## Dependencies

- The current authoritative terrain, triangle interpolation, crater deformation, bounds, and mesh refresh work in `docs/specs/20260905-143022-projectile-terrain-impact/` and `docs/specs/20260905-183525-first-crater-terrain-deformation/`.
- Tank support/settling and local movement from `docs/specs/20260906-141906-tank-support-settling/` and `docs/specs/20260906-082940-move-or-fire-movement/`.
- Current projectile, gravity, wind, camera, tactical HUD, weapon, and multiplayer semantics, especially `docs/specs/20260907-202556-match-setup-players/`.
- The Azimuth Constitution, `docs/roadmap.md`, `docs/world-conventions.md`, and `docs/projectile-model.md`.

## Out of Scope

- Selectable battlefield sizes, authored maps, terrain-style presets, multiple biome/climate/moisture systems, erosion, rivers, flowing water, water physics, swimming, drowning, buoyancy, splash, underwater effects, sophisticated water shaders, or shoreline simulation.
- Authoritative/destructible/colliding buildings; building damage, interiors, cities, roads, procedural architecture, foliage, forests, trees, generic prop/object systems, scenery physics, or terrain-cover rules.
- Sophisticated tactical fairness, line-of-sight or trajectory evaluation, complete unwinnable-terrain prevention, final fair randomized starts, or seed-selection UI.
- AI controllers, AI movement/pathfinding/terrain knowledge, new weapons, atmosphere/drag, environmental presets, major camera redesign, or vehicle movement redesign.

## Roadmap Alignment

- On completion, update only the demonstrated **Terrain Generation** items for deterministic and seeded generation, playable generated terrain, and unusable-spawn avoidance; leave unwinnable-terrain prevention, authored terrain, and final fair randomized starts unchecked.
- Update demonstrated **Battlefield Scale and Features** items for enlarged battlefield scale, readable camera/projectile/query behaviour, tactical hills/ridges/mountains/bowls/valleys, and the explicit presentation-only status of primitive buildings. Do not claim buildings improve authoritative line-of-fire, cover, or navigation.
- Record that this feature establishes one useful larger 2–8-player battlefield scale, while leaving selectable size ranges and large/curved battlefield investigation for later.
- Record the water plane and presentation-only building findings. If manual play shows that non-authoritative buildings mislead players, add authoritative/destructible structures as a future roadmap candidate rather than expanding this feature.
- Update crater/tactical-effect evidence only if repeated manual play verifies that the new geography and deformation materially affect later shot choices; do not complete unrelated deformation, terrain-style, environment, or AI items.

## Constitution Compliance

- The feature extends one authoritative terrain surface and composes existing projectile, wind, crater, movement, and settling systems to create terrain-driven tactics; it adds no bespoke mountain rules.
- Reproducible terrain, starts, and dressing honour deterministic simulation, while presentation-only water/buildings cannot own gameplay state.
- The chosen scale, constrained mesh budget, simple colour map, water plane, and primitive dressing keep implementation coherent and focused rather than creating a world engine, biome system, or physics framework.
- The feature remains a playable vertical slice: multiple local human matches stay completable with the existing weapons and turns, while nonessential discoveries are retained on the roadmap.
