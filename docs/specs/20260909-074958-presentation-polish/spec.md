# Feature Specification: Presentation Polish — Full Player Scoreboard and Immersive Battlefield Background

**Feature Branch**: `20260909-074958-presentation-polish`  
**Created**: 2026-09-09  
**Status**: Draft  
**Input**: User description: "Presentation Polish — Full Player Scoreboard and Immersive Battlefield Background"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Read Every Participant (Priority: P1)

As a local-match player, I can see every configured participant’s name, condition, elimination
state, and current-turn indication in one compact scoreboard, so I can understand an entire
2–8-player match without guessing who is still participating.

**Why this priority**: A complete participant list is essential match information; hiding players
after the first two makes larger supported matches difficult to follow.

**Independent Test**: Start 2-, 3-, and 8-player matches with distinctive configured names,
advance turns, damage and eliminate players, and verify the scoreboard contains one correct,
ordered entry for every configured participant.

**Acceptance Scenarios**:

1. **Given** a two-player match, **When** its scoreboard is shown, **Then** it contains exactly
   two polished player entries in configured match order and retains the current HUD’s compact
   visual character.
2. **Given** a three- or eight-player match with configured display names, **When** its scoreboard
   is shown, **Then** it contains exactly one entry per configured player, in stable Player 1
   through Player N order, using those display names.
3. **Given** an in-progress match, **When** the active turn changes, **Then** the active player’s
   entry is immediately and clearly distinguishable from every other entry.
4. **Given** a player reaches elimination, **When** later players take turns, **Then** that
   player remains in their original scoreboard position with health and a clear eliminated state.

---

### User Story 2 - See a Battlefield in a World (Priority: P1)

As a player looking across the map, I see blue sky, simple clouds, and a distant continuation of
the landscape rather than empty space around a floating rectangular stage, so the match feels set
inside a world.

**Why this priority**: The void and exposed board edge are the most visible world-presentation
rough edges and detract from the playable artillery scene.

**Independent Test**: Start representative matches, use the ordinary aiming and shot-view camera
positions, and inspect views toward each battlefield edge; sky, clouds, and connected distant
landscape remain visible without changing gameplay boundaries.

**Acceptance Scenarios**:

1. **Given** normal gameplay camera views, **When** the player looks beyond the playable terrain,
   **Then** a coherent blue-sky background with simple cloud treatment is visible instead of an
   empty void.
2. **Given** a view toward a playable-terrain edge, **When** the camera faces the horizon,
   **Then** a lower-detail visual landscape continues from the playable terrain far enough to mask
   the obvious rectangular-stage edge at normal gameplay distances.
3. **Given** the visual extension meets playable terrain, **When** viewed at normal gameplay
   distances, **Then** their elevation and colour transition is not distractingly discontinuous.
4. **Given** a distant view, **When** haze is used, **Then** it subtly joins far landscape and sky
   without obscuring nearby tanks, terrain, or tactical information.

---

### User Story 3 - Keep Presentation Outside Gameplay (Priority: P2)

As a player, I receive the more immersive view without gaining extra terrain to drive on, fire
against, deform, spawn on, or collide with, so familiar artillery rules remain reliable.

**Why this priority**: The landscape extension must improve perception without silently changing
the established match rules.

**Independent Test**: Fire toward and beyond each playable boundary, attempt ordinary movement at
the boundary, create edge craters, and verify the existing authoritative terrain and bounds remain
the sole source of gameplay outcomes.

**Acceptance Scenarios**:

1. **Given** projectiles, tanks, movement, spawning, collision, and terrain deformation, **When**
   they reach the playable boundary, **Then** they continue to use the existing authoritative
   battlefield bounds and terrain rules.
2. **Given** an edge impact on playable terrain, **When** deformation is resolved, **Then** only
   the authoritative battlefield changes; the outer visual landscape does not become deformable
   gameplay terrain.
3. **Given** the normal aiming and shot-view camera controls, **When** a player tilts or zooms
   within their allowed range, **Then** ordinary views do not trivially reveal the terrain
   underside, empty void, or a sharp outer-world cutoff.

---

### User Story 4 - Preserve Readability and Performance (Priority: P2)

As a player in both a small duel and a full local match, I can still read essential match
information while the battlefield remains the visual focus and presentation remains responsive.

**Why this priority**: The feature succeeds only if added context does not replace one visual
rough edge with obstruction or a noticeable performance regression.

**Independent Test**: Manually compare 2-, 4-, and 8-player matches through ordinary aiming,
movement, firing, impact, and turn changes; confirm that the scoreboard stays readable and the
scene remains responsive.

**Acceptance Scenarios**:

1. **Given** a two-player match, **When** the scoreboard is shown, **Then** its player-summary
   area remains visually balanced rather than looking like an eight-player layout with empty
   space.
2. **Given** an eight-player match, **When** the scoreboard is shown, **Then** all eight entries
   remain readable and compact enough that the central battlefield remains substantially visible.
3. **Given** ordinary gameplay, **When** the background presentation is active, **Then** turn
   progression, input, projectile resolution, terrain deformation, and camera transitions remain
   responsive and visually coherent.

### Edge Cases

- A match at every supported participant count from two through eight produces the matching entry
  count without special-case omissions or duplicate entries.
- Names at the existing maximum display length remain identifiable in a full scoreboard without
  changing player identity, order, or vital state.
- A winner, draw, or resolving-shot state shows no misleading active-player entry while retaining
  all player condition and elimination information.
- An eliminated player who previously had the active turn is never styled as active after turn
  progression selects a surviving participant.
- Near each cardinal map edge and corner, the presentation layer masks normal-view stage edges
  without extending allowed movement, spawn, collision, projectile-impact, or deformation areas.
- If a constrained camera angle can expose an underside, void, or abrupt cutoff, only the minimal
  view-range or clipping adjustment needed to prevent that ordinary view is permitted.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The game MUST show a top-left scoreboard for every active match participant from the
  configured player collection; it MUST support every valid match size from 2 through 8 and MUST
  not assume two fixed player entries.
- **FR-002**: The scoreboard MUST present entries in stable configured match order, using each
  player’s configured display name rather than a generated label when a configured name exists.
- **FR-003**: Each scoreboard entry MUST show the player’s current health, an unambiguous alive or
  eliminated state, and the established player visual identity where it is already available.
- **FR-004**: The scoreboard MUST make the current active player clearly distinguishable during an
  in-progress turn, and MUST not identify an active player when the match is resolving or over.
- **FR-005**: Eliminated players MUST remain visible in their original ordered scoreboard entries
  for the rest of the match and MUST be clearly marked eliminated rather than merely dimmed into
  ambiguity.
- **FR-006**: The scoreboard MUST preserve the current HUD’s restrained panel, contrast, and
  player-colour visual language; the two-player case MUST remain compact and polished.
- **FR-007**: The scoreboard MUST adapt its local row spacing and/or type scale only as needed for
  larger supported matches, keep all eight entries readable, and leave the central battlefield
  substantially unobstructed. It MUST NOT introduce a general responsive-HUD framework or an
  unrelated HUD redesign.
- **FR-008**: Scoreboard presentation MUST be read-only: it MUST derive identity and condition
  from the existing player, tank, and turn state and MUST NOT duplicate, mutate, gate, or own
  authoritative match state.
- **FR-009**: The game MUST provide a simple, coherent blue-sky background with visible static or
  lightly stylised clouds. The treatment MUST remove normal-view empty void without requiring
  weather, time-of-day, moving clouds, or advanced atmospheric simulation.
- **FR-010**: The game MUST provide a lightweight, presentation-only visual landscape surrounding
  the playable battlefield that connects credibly to its terrain and extends far enough to hide
  the rectangular stage edge at normal camera ranges.
- **FR-011**: The visual landscape MUST use broadly compatible elevation and colour treatment at
  the playable boundary and MAY use subtle distance haze to soften far terrain into the horizon;
  the transition MUST not be distractingly seamed at normal gameplay distances.
- **FR-012**: The outer visual landscape, sky, clouds, and haze MUST remain presentation-only.
  They MUST NOT alter existing authoritative battlefield bounds, terrain queries, spawning,
  movement, tank support, projectile collision/limits, damage, or terrain deformation.
- **FR-013**: The feature MUST retain the existing playable terrain as the single authoritative
  mutable battlefield surface. It MUST NOT create a second authoritative terrain system, outer
  collision world, infinite terrain, terrain streaming, or world chunking.
- **FR-014**: The camera MUST continue to use the current gameplay-oriented controls and framing.
  It MAY receive only small range, angle, or clipping refinements required to prevent ordinary
  views from trivially exposing terrain undersides, empty void, or sharp horizon cutoffs.
- **FR-015**: The background and horizon treatment MUST remain lightweight and must not
  significantly increase playable-terrain detail solely to conceal stage edges.
- **FR-016**: The feature MUST provide automated coverage, where meaningful without pixel testing,
  for player-collection-derived scoreboard entries at 2, 3, and 8 players; stable ordering;
  configured names; health/elimination state; active-player representation; background/horizon
  presentation initialization; and preservation of authoritative battlefield bounds.
- **FR-017**: The feature MUST retain existing behaviour for match setup, turn advancement,
  projectiles, weapons, damage, terrain deformation, tank settling, movement, and camera
  presentation except for the narrowly required scoreboard and visual-world polish.
- **FR-018**: The feature MUST keep AI, weapons, terrain-generation changes, a full
  atmosphere/environment system, weather, rain, cloud motion, day/night, sun simulation, shadow
  overhaul, minimaps, outer-world physics, outer-world deformation, and scoreboard redesign
  beyond 2–8-player support out of scope.

### Key Entities

- **Scoreboard entry**: A read-only, ordered presentation of one configured match participant’s
  display name, visual identity, health, survival state, and current-turn status.
- **Player collection**: The existing ordered configured participant set that supplies match order,
  stable identity, and configured display names for a running match.
- **Authoritative battlefield**: The current bounded mutable terrain surface that alone supplies
  terrain queries, collision, movement support, spawning, projectile impacts, and deformation.
- **Visual horizon landscape**: A lower-detail, non-playable landscape continuation used solely to
  conceal the authoritative battlefield’s visible edge and merge it into the distance.
- **Sky presentation**: The static blue-sky, cloud, and optional subtle-haze visual treatment
  behind the battlefield and horizon landscape.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Automated 2-, 3-, and 8-player presentation-state cases each produce exactly 2, 3,
  and 8 scoreboard entries respectively; 100% of entries retain configured order and display names
  and correctly represent current health, elimination, and active-turn state.
- **SC-002**: In manual 2-, 4-, and 8-player matches, all configured participants are visible in
  the scoreboard throughout play; active players and eliminated players can be correctly
  identified in every inspected turn, including after multiple eliminations.
- **SC-003**: In manual two-player inspection, the scoreboard presents exactly two readable entries
  without conspicuous unused player-row space; in manual eight-player inspection, all eight
  entries are readable while the central battlefield remains substantially visible.
- **SC-004**: In normal aiming and shot-view inspection toward each battlefield side, the view
  contains sky, clouds, and connected distant landscape rather than a visible empty void or a
  plainly floating rectangular terrain edge.
- **SC-005**: In automated and manual boundary checks, 100% of sampled out-of-bounds movement,
  spawning, terrain querying, projectile resolution, and deformation cases continue to use the
  pre-feature playable battlefield boundary; no visual-horizon surface affects those outcomes.
- **SC-006**: Manual normal-distance inspection finds no distracting playable-to-horizon seam or
  sharp outer cutoff, including near battlefield corners, while nearby terrain, tanks, and HUD
  remain clear.
- **SC-007**: During representative 2-, 4-, and 8-player manual matches, ordinary input, turn
  handoff, shots, impacts, deformation, and camera transitions remain responsive with the added
  presentation active.
- **SC-008**: Relevant automated tests and repository quality checks pass with no new unjustified
  warnings, and existing match, HUD, camera, terrain, projectile, deformation, movement, damage,
  and turn-flow regressions remain absent.

## Assumptions

- The existing configured-player sequence is the authoritative stable match order, and its current
  maximum name length remains sufficient for this presentation feature.
- The current player identity colours and tactical-panel visual language are retained; this feature
  refines only the player-summary portion needed for all supported participants.
- “Normal camera views” means the existing active-player and shot presentation views plus ordinary
  user orbit/zoom positions within the supported camera range, not deliberately unsupported or
  debug-camera views.
- A static or lightly stylised cloud treatment and inexpensive horizon haze are sufficient if they
  make the world read as sky and distance; no external art pipeline is required by the feature.
- The horizon landscape may be lower detail and need not mirror live craters or other terrain
  changes, provided its boundary transition remains convincing at normal gameplay distances.
- Existing camera framing should be preserved unless a small adjustment is demonstrably necessary
  to meet the no-obvious-void and no-floating-board acceptance outcomes.

## Dependencies

- Current multiplayer match configuration and player identity/presentation state, including the
  supported two-to-eight participant collection, from
  `docs/specs/20260907-202556-match-setup-players/`.
- Current graphical tactical HUD, health, elimination, turn, player-colour, and read-only
  presentation boundary from `docs/specs/20260906-190352-graphical-tactical-hud/`.
- Current tactical camera controls and presentation transitions from
  `docs/specs/20260906-092436-tactical-controls-camera/`.
- Current expanded authoritative battlefield, terrain colouring, water, presentation-only world
  dressing, bounds, deformation, and 2–8-player starts from
  `docs/specs/20260908-200635-expanded-battlefield/`.
- The Azimuth Constitution, `docs/roadmap.md`, and `docs/world-conventions.md`.

## Out of Scope

- AI controllers or computer turns; the next major planned feature remains **First AI Controller —
  A Computer Player That Can Complete a Turn**.
- New weapons, gameplay terrain-generation algorithms, new terrain styles, new player controls,
  full HUD redesign, minimap, or generic responsive UI framework.
- Volumetric or moving clouds, weather, rain, cloud shadows, atmospheric scattering, day/night,
  sun simulation, full fog/atmosphere systems, or a shadow overhaul.
- Infinite, streamed, chunked, or second authoritative terrain; outer-world collision, movement,
  spawning, physics, projectile impacts, or deformation.
- Broad camera-system redesign beyond narrowly required visibility safeguards.

## Roadmap Alignment

- On completion, update only roadmap work that is demonstrably satisfied by full 2–8-player match
  readability and the lightweight visual-world presentation. Do not claim completion of general
  terrain-style, curved/infinite-world, full-atmosphere, generic UI, or AI work.
- Preserve the next major feature as **First AI Controller — A Computer Player That Can Complete a
  Turn**.
- Record any discovered but nonessential presentation, terrain, camera, or performance work in
  `docs/roadmap.md` rather than expanding this feature.

## Constitution Compliance

- The feature is a bounded playable-presentation improvement: it makes existing local multiplayer
  understandable and the existing battlefield visually coherent without adding gameplay systems.
- The scoreboard and visual world extension are explicitly read-only presentation; the bounded,
  mutable battlefield remains the sole authority for deterministic gameplay.
- A collection-derived scoreboard and lightweight background/horizon treatment favour direct,
  understandable work over a generic UI or world-engine abstraction.
- Scope excludes sophisticated environmental simulation and outer-world mechanics, preserving
  incremental playable progress and leaving unrelated discoveries on the roadmap.
