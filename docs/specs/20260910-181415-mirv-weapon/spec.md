# Feature Specification: MIRV — First Multi-Projectile Weapon

**Feature Branch**: `20260910-181415-mirv-weapon`

**Created**: 2026-09-10

**Status**: Draft

**Input**: User description: "MIRV — First Multi-Projectile Weapon"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Fire a Readable MIRV Barrage (Priority: P1)

As a player with uncertain range to nearby opponents, I can select and fire a limited MIRV round that rises as one shell, breaks apart near its apex, and covers an area with several smaller warheads, so I can choose distributed coverage instead of a concentrated High Explosive blast.

**Why this priority**: The carrier-to-barrage transition is the weapon's defining tactical and visual identity.

**Independent Test**: Give a Human player one MIRV, fire a high arc across varied terrain, and observe one carrier followed by exactly five independently descending child warheads and several possible craters.

**Acceptance Scenarios**:

1. **Given** a player has MIRV ammunition and is choosing an action, **When** they select MIRV and fire with their current azimuth, elevation, and power, **Then** one visible carrier launches through the same authoritative firing path as an ordinary shell.
2. **Given** an airborne MIRV carrier, **When** it reaches the first observed transition from upward to downward vertical movement, **Then** it visibly disappears as a damaging carrier and exactly five child warheads begin from its current region with inherited forward motion and a modest, deterministic spread across both battlefield horizontal axes.
3. **Given** a high, windy MIRV shot, **When** the carrier and children fly, **Then** wind visibly affects both the split location and the subsequent child pattern without making the pattern arbitrary.
4. **Given** a MIRV is aimed badly or wind and terrain deflect its pattern, **When** all children finish flight, **Then** it remains possible for every child to miss tanks.

---

### User Story 2 - See Every Consequence Before the Turn Ends (Priority: P1)

As a match participant or observer, I see MIRV children strike independently and the game waits for the whole barrage before advancing, so each crater, hit, and late warhead remains meaningful.

**Why this priority**: Correct complete-shot resolution is essential to fair turns, damage, terrain state, and understandable multi-impact play.

**Independent Test**: Arrange a spread in which children land at distinct times, including one that lands after an earlier crater changes nearby terrain; verify impacts resolve normally and the next player begins only after the final child and resulting settling have resolved.

**Acceptance Scenarios**:

1. **Given** a MIRV carrier splits, **When** one or more children remain airborne, **Then** the firing turn stays in resolution and no new move, aim, or fire action is accepted.
2. **Given** one MIRV child impacts terrain while siblings remain airborne, **When** its explosion resolves, **Then** it applies the normal damage, crater, tank-support, elimination, and presentation consequences without removing or stopping its siblings.
3. **Given** earlier MIRV impacts have deformed terrain, **When** a later child crosses that area, **Then** its collision uses the current deformed battlefield surface.
4. **Given** every child has impacted or ended flight and all resulting tank settling has completed, **When** match survival is evaluated, **Then** the turn advances or the match ends exactly once under the existing rules.

---

### User Story 3 - Read a Multi-Impact Spectacle Comfortably (Priority: P1)

As a Human spectator of either a Human or AI MIRV shot, I can understand the split, spread, and overall threatened area without arbitrary child chasing, camera snapping, flashing, or a wall of clipping explosion audio.

**Why this priority**: The barrage must be spectacular while still teaching the player what happened and where to aim next.

**Independent Test**: Fire Human and AI MIRVs over a valley, ridge, and nearby tank group; confirm carrier-first coverage, widened split coverage, aggregate impact framing, restrained repeated pulses, and controlled repeated impact audio.

**Acceptance Scenarios**:

1. **Given** a Human fires MIRV, **When** the carrier launches, **Then** the existing Human golf-style shot view initially follows the carrier; at split it widens to favour the spread and likely impact region rather than choosing one child to chase.
2. **Given** an AI fires MIRV, **When** the carrier and children travel, **Then** the existing tactical-wide philosophy shows the route, split, and threatened region without a separate AI-only MIRV camera system.
3. **Given** several children impact close together, **When** impact presentation updates, **Then** it favours the aggregate affected region and does not rapidly jump among individual impact points.
4. **Given** multiple child impacts occur in quick succession, **When** visual and audio feedback is requested, **Then** each impact remains intelligible while screen pulses are coalesced or reduced and explosion audio remains controlled rather than strobing or clipping.

---

### User Story 4 - Use MIRV Without Breaking Existing Play (Priority: P1)

As a player in a mixed weapon match, I can see MIRV and its remaining ammunition in normal weapon selection while Basic Shell, High Explosive, Heavy Shell, and AI turns continue to work.

**Why this priority**: Arsenal expansion must add a meaningful choice without regressing the established artillery loop.

**Independent Test**: Run Human and AI matches with MIRV selected, exhausted, and unselected; exercise every conventional weapon and verify their normal launch, impact, inventory, and turn behaviour.

**Acceptance Scenarios**:

1. **Given** a new match, **When** each player views their available weapons, **Then** MIRV is named in the existing selection and inventory presentation with two starting rounds.
2. **Given** a player fires MIRV, **When** the carrier launches, **Then** exactly one MIRV round is consumed; its child warheads consume no additional ammunition.
3. **Given** a player has no MIRV rounds remaining, **When** they try to select or fire it, **Then** the action is unavailable and the existing fallback selection behaviour remains safe.
4. **Given** an AI has MIRV in its loadout, **When** it takes a turn, **Then** it completes its supported existing weapon behaviour autonomously; sophisticated MIRV selection is not required.

### Edge Cases

- A low arc that encounters terrain before reaching an apex resolves as an ordinary carrier impact and creates no children.
- An out-of-bounds carrier or child terminates safely; it neither creates an invented explosion nor leaves the shot unresolved.
- A split at the simulation-step apex happens exactly once and cannot leave the carrier active alongside children.
- Simultaneous or near-simultaneous child impacts use a stable deterministic ordering, and each later collision observes terrain changes already resolved earlier in that ordering.
- Child impacts may overlap in position or damage range; ordinary repeated explosion and elimination rules apply without a MIRV-only damage shield.
- A match-ending child impact still allows no remaining child or settling consequence to be skipped before the final result is established.
- Missing, muted, delayed, or unavailable split/impact presentation cannot alter child trajectories, collision, damage, terrain, settling, or turn progression.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The game MUST add MIRV as a limited weapon with two starting rounds per player, normal weapon selection/HUD naming, and no MIRV-specific player controls.
- **FR-002**: A committed MIRV round MUST launch exactly one visible carrier using the existing authoritative azimuth, elevation, power, gravity, fixed-step ballistic, and captured wind-response rules.
- **FR-003**: A carrier that reaches its first observed ascent-to-descent transition before termination MUST split once into exactly five child warheads; a carrier that terminates before that transition MUST not split.
- **FR-004**: At split, each child MUST begin at the carrier's resolved split region, inherit its current velocity, and receive a fixed, modest deterministic separation velocity that creates a readable fan across both horizontal axes while retaining forward travel.
- **FR-005**: Child separation MUST be reproducible from authoritative match/shot inputs and MUST NOT use presentation randomness, audio timing, camera state, or unseeded randomness.
- **FR-006**: The carrier MUST cease being an active damaging projectile when it splits; children MUST be independently active projectiles that use the ordinary gravity, wind, swept terrain collision, bounds, and impact rules.
- **FR-007**: Every active child MUST continue responding to the match wind after split. Wind influence on the carrier before split and each child after split MUST be independently observable in deterministic tests.
- **FR-008**: Each child terrain impact MUST use the established authoritative explosion path, including normal radial damage, elimination, crater deformation, support reconciliation, and tank settling. Each child impact profile MUST be weaker than High Explosive in concentrated blast strength and crater scale.
- **FR-009**: The system MUST permit naturally overlapping MIRV explosions, damage events, and craters according to ordinary impact rules; it MUST not add a MIRV-only overlap immunity.
- **FR-010**: A fired shot MUST be able to own more than one active projectile using the smallest coherent relationship/state necessary to identify one shot's carrier or children. It MUST NOT introduce a generic scripted projectile graph or arbitrary workflow engine.
- **FR-011**: Turn resolution MUST remain active after a split, an individual child impact, or an individual child out-of-bounds termination. It MUST complete only after the fired shot has no active projectiles and all authoritative settling/consequences have resolved.
- **FR-012**: Child processing order and the relationship between successive impacts and mutable terrain MUST be stable and deterministic, so identical authoritative inputs yield identical split positions, child velocities, trajectories, impact positions, damage, terrain, survivors, and turn result.
- **FR-013**: Human MIRV presentation MUST follow the carrier initially, then widen at split to favour the overall child spread and likely impact region; it MUST not tightly follow an arbitrary child.
- **FR-014**: AI MIRV presentation MUST extend the existing tactical-wide shot philosophy to show carrier route, split, child pattern, and threatened region without a separate AI-specific MIRV camera mode.
- **FR-015**: During multiple impacts, camera presentation MUST favour an aggregate impact region and avoid rapid child-to-child camera snaps. Presentation MUST observe rather than delay or alter authoritative resolution.
- **FR-016**: Repeated MIRV child impacts MUST use restrained, comfortable presentation: screen pulse feedback MUST be coalesced, cooled down, or reduced as needed, and impact audio MUST use bounded per-child/concurrency/spatial tuning so rapid impacts do not form a harsh strobe or uncontrolled identical full-volume wall.
- **FR-017**: The split MUST be visually recognisable as one carrier becoming five visible child warheads, using presentation consistent with existing simple projectile visuals; production particle effects are not required.
- **FR-018**: AI turns MUST remain autonomous and complete safely with MIRV present in inventories. This feature MUST NOT add AI weapon-selection strategy; any such selection work remains roadmap work.
- **FR-019**: Automated coverage MUST verify normal carrier launch, carrier gravity/wind, apex split, child count/removal/initial state, deterministic spread and trajectories, independent child timing/collision/wind, stable terrain-after-crater collision, normal damage/elimination/craters, one-round inventory consumption, exhausted-ammunition rejection, complete-shot turn resolution, and regression behaviour for Basic Shell, High Explosive, and Heavy Shell.
- **FR-020**: The feature MUST update the roadmap only for MIRV capabilities actually delivered and record future arsenal concepts without treating them as implementation commitments.
- **FR-021**: The feature MUST NOT add configurable split timing/count, Cluster Bomb, Bomb Net, Death Sphere, Death's Head, Funky Bomb, nuclear or napalm weapons, bouncing/rolling/tunnelling/laser/homing/remote-detonation mechanics, persistent hazards, economy/progression, generic weapon scripting, or generic multi-stage behaviour graphs.

### Key Entities

- **Fired shot**: The authoritative committed weapon use, which now owns all active carrier/child projectiles until complete resolution.
- **MIRV carrier**: The initially launched ordinary ballistic projectile that either impacts/terminates before apex or separates once at apex.
- **MIRV child warhead**: One of five independently simulated projectiles created at the split and resolved through the normal impact path.
- **Child separation pattern**: The fixed deterministic velocity offsets that distribute inherited carrier motion across the horizontal battlefield.
- **Aggregate barrage presentation**: Read-only camera, pulse, and audio response centred on the spread/affected region rather than an arbitrary individual warhead.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 20 deterministic repeats of each of three representative MIRV launch fixtures, all 20 repeats per fixture produce identical split step, five child initial states, ordered impact outcomes, terrain result, survivors, and turn result.
- **SC-002**: In 10 open-terrain MIRV manual shots, 10 of 10 visibly show one carrier followed by five distinguishable children that spread across both horizontal axes and retain forward descent.
- **SC-003**: In fixtures where child impacts occur at distinct times, 100% keep the firing turn resolving after split and after every non-final child; 100% advance exactly once after the final child and settling complete.
- **SC-004**: In 10 crosswind-versus-calm comparisons, all 10 show a changed carrier split location or child impact pattern under crosswind, while repeated runs with the same wind remain identical.
- **SC-005**: In 10 representative ridge, valley, slope, and nearby-tank-group MIRV shots, at least one fixture demonstrates multiple separate craters and one demonstrates a later child colliding against terrain altered by an earlier impact.
- **SC-006**: In Human and AI manual reviews containing five rapid child impacts, reviewers can identify the split and threatened region in 9 of 10 shots, with no recurring violent camera hopping, uncomfortable full-screen strobing, or uncontrolled audio clipping.
- **SC-007**: Existing automated regression coverage for Basic Shell, High Explosive, and Heavy Shell passes unchanged, and each retains its normal single-projectile resolution behaviour in 100% of its fixtures.

## Assumptions

- Two MIRV rounds per player are sufficient for initial playtesting and use the existing limited-inventory fallback behaviour.
- Five children are the smallest visually legible initial barrage; their exact separation and child impact tuning are balance values to be established during planning and manual play.
- Apex-relative splitting means the first authoritative fixed simulation step at which the carrier transitions from upward to non-upward vertical motion, avoiding a new player-facing timing control.
- A pre-apex terrain or bounds termination is a normal unsplit carrier resolution, preserving intuitive collision rules.
- Existing fixed-step projectile simulation, mutable terrain surface, impact/explosion path, settling, Human/AI presentation modes, and successful launch/impact audio boundaries are dependencies to extend, not replace.
- Presentation may group close impacts for comfort, but never changes their authoritative sequence or outcomes.

## Dependencies

- Existing weapon definitions, player loadouts, selection controls, HUD inventory display, and shared Human/AI firing path.
- Existing deterministic projectile model: fixed steps, gravity, captured horizontal wind response, swept terrain collision, and bounds termination.
- Existing authoritative terrain deformation, damage, tank settling/elimination, and resolving-fire turn lifecycle.
- Existing Human follow, AI tactical-wide, shared impact camera, impact pulse, and launch/impact audio presentation boundaries.
- Existing Basic-Shell AI firing decision, which remains the supported AI selection behaviour in this feature.

## Roadmap Alignment

This feature directly advances **Weapons → Cluster / Multi-Projectile Weapons**: MIRV-style projectile, independent child-projectile trajectories, and child wind interaction. Configurable split altitude/timing remains incomplete because it is not player-configurable.

It advances **Milestone G — Arsenal** by delivering one cluster/MIRV weapon. It may mark “Several distinct weapons exist” and “Weapon selection creates meaningful tactical choices” only after the existing conventional weapons plus MIRV have passed the specified playtesting and show the intended HE-versus-MIRV distinction. It does not complete terrain-manipulation or ridiculous-weapon milestones.

Future concepts recorded in the roadmap—including Death Sphere, Bomb Net, Curve Ball, expanded terrain-following weapons, large explosives, hazard weapons, terrain destroyers, directed piercing weapons, and further multi-warhead weapons—remain explicitly out of scope.

## Manual Acceptance

- **Basic barrage**: Fire a Human MIRV over open terrain. Confirm one carrier, understandable arc, obvious split, five visible warheads, three-dimensional spread, several independent impacts/craters, and turn handoff only after the last consequence.
- **Wind and terrain**: Compare weak wind with strong crosswind, then fire into a valley, ridge, mountain slope, and nearby tank group. Confirm carrier and children drift naturally; terrain intercepts children at different times; earlier craters can affect later collision; coverage is useful but misses remain possible.
- **Human and AI presentation**: Confirm Human carrier follow widens at split, AI remains tactical-wide, aggregate impact framing avoids arbitrary child pursuit and rapid snaps, and threatened areas remain understandable.
- **Presentation safety**: Confirm clustered impacts communicate repeated explosions without harsh flashing or clipping, while disabled or unavailable presentation leaves deterministic gameplay unchanged.
- **Tactical identity**: Across several matches, confirm MIRV is selected for distributed coverage, uncertain range, groups, and terrain disruption, while High Explosive remains the choice for concentrated blast power. If MIRV reads only as “HE but more damage,” tune before completion.

## Architecture Review

- **Can one fired shot own more than one active projectile without special-casing the entire turn system around MIRV?** It must be yes.
- **Do child projectiles use the same authoritative physics, collision, and impact systems as ordinary projectiles?** It must be yes.
- **Does the turn wait for the complete shot rather than one arbitrary projectile?** It must be yes.
- **Is deterministic behaviour preserved?** It must be yes.
- **Did the design add only abstractions MIRV actually required?** It must be yes.
