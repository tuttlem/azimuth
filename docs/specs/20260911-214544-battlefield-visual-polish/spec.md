# Feature Specification: Battlefield Visual Polish — Tank Variety, Materials, Explosion Particles and Smoke

**Feature Branch**: `20260911-214544-battlefield-visual-polish`  
**Created**: 2026-09-11  
**Status**: Draft  
**Input**: User description: "Battlefield Visual Polish — Tank Variety, Materials, Explosion Particles and Smoke"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Recognise Individual Tanks (Priority: P1)

As a player in a local match, I can distinguish several little tank silhouettes at a glance while
still recognising each player's established colour, so the battlefield feels populated by
individual combatants rather than repeated placeholders.

**Why this priority**: Clear tank identity is the most immediate improvement to the battlefield's
character and must not weaken tactical player recognition.

**Independent Test**: Start 2-, 4-, and 8-player matches, inspect each tank while it moves, turns,
aims, fires, settles, and is eliminated, and confirm the visual variety remains cosmetic.

**Acceptance Scenarios**:

1. **Given** any valid new match of two through eight players, **When** tanks appear, **Then**
   every player has one valid cosmetic model from a modest family of four to six clearly distinct
   tank silhouettes.
2. **Given** a match with more players than distinct silhouettes, **When** models are assigned,
   **Then** duplicates are permitted and every player's existing colour remains immediately visible.
3. **Given** a tank turns its hull, rotates its turret, elevates its barrel, fires, moves, settles,
   or is eliminated, **When** its presentation updates, **Then** its model stays visually coherent
   with the established pose and launch conventions without changing gameplay behaviour.
4. **Given** equivalent matches with cosmetic tank presentation enabled and disabled, **When** the
   same authoritative inputs are resolved, **Then** player identity, controller, starting state,
   movement, aiming, launch, damage, terrain, and turn outcomes are identical.

---

### User Story 2 - Read a More Characterful Battlefield (Priority: P1)

As a player looking over the battlefield, I see simple painted-metal tanks, grassy ground, dirt,
rocky higher terrain, and water that catches the light, so the game reads as a cohesive stylised
low-poly world rather than a collection of flat-colour primitives.

**Why this priority**: Material character and restrained surface variation make the existing large,
deformable battlefield feel like a game world without asking for realism or an art-production
pipeline.

**Independent Test**: Inspect ordinary gameplay camera views before and after craters and terrain
deposition, including tank and water views under the normal battlefield light.

**Acceptance Scenarios**:

1. **Given** normal battlefield lighting, **When** a player views any tank, **Then** its
   player-coloured armour reads as restrained painted metal while tracks, barrels, and mechanical
   parts remain visibly darker or more neutral.
2. **Given** ordinary terrain and terrain changed by craters or Dirt Bomb deposition, **When** it
   is rendered, **Then** grass, exposed earth, and higher or steeper rocky terrain retain sensible
   stylised colour and fine surface variation coherent with the current surface shape.
3. **Given** a normal camera view of the water surface, **When** it receives ordinary lighting,
   **Then** blue variation and surface response make it read as water rather than a flat blue floor.
4. **Given** the upgraded presentation, **When** players use normal camera, HUD, and shot views,
   **Then** terrain shape, tanks, projectiles, player colours, and tactical information remain
   readable; the feature does not pursue photorealism.

---

### User Story 3 - Feel Impact Energy (Priority: P1)

As a player watching a projectile hit, I see a short outward and upward burst of hot fragments and
terrain debris in addition to the existing impact presentation, so each resolved explosion has
clear force and energy.

**Why this priority**: Weapon impacts are the payoff of every turn; a modest burst adds immediate
visual charm without changing the established authoritative explosion.

**Independent Test**: Fire Basic Shell, High Explosive, MIRV, Cluster Bomb, Bomb Net, and Nuke at
terrain and compare their burst scale, lifetime, cleanup, and unchanged gameplay result.

**Acceptance Scenarios**:

1. **Given** a resolved terrain explosion, **When** impact presentation begins, **Then** a bounded
   short-lived burst originates near the impact, visibly travels outward and upward, visually falls
   or slows, fades or shrinks, and cleans itself up.
2. **Given** explosions with different existing profile magnitudes, **When** their bursts appear,
   **Then** small impacts remain restrained, High Explosive is visibly larger than a Basic Shell,
   and Nuke is substantially more dramatic without introducing weapon-name-only presentation
   rules where profile magnitude supplies the distinction.
3. **Given** a multi-projectile barrage, **When** many child impacts resolve near one another,
   **Then** each remains readable but total concurrent particle/debris presentation is bounded.
4. **Given** an impact burst, **When** it plays or expires, **Then** it does not change damage,
   projectile collision, terrain deformation, tank support, turn resolution, AI decisions, wind,
   or authoritative visibility.

---

### User Story 4 - See a Brief Aftermath (Priority: P2)

As a player after a suitable impact, I see a modest translucent smoke plume rise, spread, and fade,
so the battlefield keeps a short visual memory of the shot without becoming difficult to read.

**Why this priority**: Smoke makes the sequence of impact and aftermath satisfying while remaining
secondary to tanks, terrain, and active play.

**Independent Test**: Fire representative weapons repeatedly, observe smoke from creation through
expiry, and confirm tanks and gameplay results remain readable and unchanged.

**Acceptance Scenarios**:

1. **Given** a suitable resolved explosion, **When** its initial burst completes, **Then** a
   translucent smoke effect appears near the impact, rises gently, expands, fades, and disappears
   within a few seconds.
2. **Given** a Nuke impact, **When** its smoke appears, **Then** it is visibly larger and longer
   lived than ordinary smoke and may suggest an exaggerated rising plume without requiring realism.
3. **Given** existing battlefield wind is available at negligible additional complexity, **When**
   smoke is active, **Then** it may drift broadly with that wind while remaining purely cosmetic.
4. **Given** repeated shots during a long or AI-heavy match, **When** smoke effects accumulate,
   **Then** lifetime and concurrent-effect limits prevent permanent battlefield filling or opaque
   tactical obstruction.

---

### User Story 5 - Preserve the Existing Game (Priority: P1)

As a player, I get a considerably less placeholder-looking game while all existing tactical and
deterministic behaviour remains exactly the same.

**Why this priority**: This feature is expressly presentation-only; visual charm must not cause a
hidden rules change or make tests and replays less trustworthy.

**Independent Test**: Compare deterministic gameplay fixtures and controlled matches before and
after disabling cosmetic model, material, particle, and smoke presentation.

**Acceptance Scenarios**:

1. **Given** the same gameplay seed and inputs, **When** cosmetic model assignment or effect
   variation is performed, **Then** it does not consume or perturb the random stream used for
   terrain, starts, wind, AI, or other authoritative simulation.
2. **Given** any cosmetic presentation element is disabled, **When** the same match is played,
   **Then** projectile trajectories and impacts, damage, terrain deformation, tank settling, turn
   order, controller behaviour, and weapon availability remain unchanged.
3. **Given** a long match, **When** many impacts and match resets occur, **Then** expired temporary
   effects are removed reliably and no persistent visual entities or per-effect resources remain.

### Edge Cases

- Every supported match size from two through eight receives valid model assignments; visual
  duplicates are allowed when necessary and assignment never changes PlayerId or controller type.
- A model with an unusually short or long-looking barrel still presents its projectile at the same
  authoritative launch point and direction; no model changes collision or firing physics.
- Terrain craters and Dirt Bomb mounds refresh surface appearance from current terrain rather than
  leaving stale colour or stretched-looking surface treatment.
- Particle and smoke limits cover simultaneous MIRV, Cluster Bomb, and Bomb Net impacts without
  preventing the authoritative impacts from resolving.
- Nuke presentation is conspicuously larger than ordinary impacts but remains bounded, cleans up,
  and does not leave a permanent plume.
- Eliminated tanks hide or show exactly as before regardless of their assigned appearance.
- Smoke stays translucent enough that normal tank, terrain, projectile, HUD, and camera reading is
  not materially obscured.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The game MUST provide a family of four to six simple, visually distinct cosmetic
  tank models, all unmistakably readable as tanks at normal gameplay camera distances.
- **FR-002**: At each match creation, the game MUST assign one valid cosmetic model to every
  configured player, prefer non-duplicate models while available, and safely allow duplicates
  thereafter.
- **FR-003**: Cosmetic model assignment MUST be isolated from all authoritative randomness and
  MUST NOT alter PlayerId, configured order, player name, controller type, tank gameplay state,
  weapon state, or turn state.
- **FR-004**: Every model MUST retain coherent visual hull, turret, barrel, and muzzle attachment
  relationships while the established authoritative tank pose, turret aim, barrel elevation, and
  projectile launch origin remain the source of gameplay truth.
- **FR-005**: Tank armour presentation MUST retain the established player colour identity and pair
  it with visibly darker or neutral mechanical parts; cosmetic variety MUST NOT reduce player
  readability.
- **FR-006**: Tanks MUST have a restrained painted-metal visual response, avoiding mirror-like
  chrome, photorealistic weathering, paint customisation, damage decals, or a skin system.
- **FR-007**: Battlefield surfaces MUST give grass, dirt, rock/high terrain, and water visibly
  distinct stylised character through modest colour and/or surface variation at a sensible world
  scale.
- **FR-008**: Surface presentation MUST remain derived from or compatible with the current mutable
  terrain surface so craters and Dirt Bomb deposition remain visually coherent without changing
  terrain physics.
- **FR-009**: Water presentation MUST improve its wet appearance through restrained colour and
  surface response only; it MUST NOT add water physics, gameplay waves, buoyancy, reflections, or
  refraction requirements.
- **FR-010**: A resolved terrain explosion MUST request one bounded, profile-scaled cosmetic burst
  containing short-lived hot and/or debris-like fragments that visibly disperse from the impact.
- **FR-011**: Cosmetic burst behaviour MUST communicate outward and upward force, short visual
  gravity or falloff, fading/shrinking, and automatic cleanup; fragments MUST NOT become gameplay
  projectiles, collisions, damage sources, or terrain modifiers.
- **FR-012**: Particle/debris count and concurrent presentation effects MUST be bounded so
  multi-projectile weapons remain lively but do not create unbounded transient work.
- **FR-013**: Suitable explosions MUST create a temporary, translucent cosmetic smoke aftermath
  that rises, expands, fades, and expires automatically after a few seconds.
- **FR-014**: Smoke scale and duration MUST reflect existing explosion magnitude, with a Nuke
  visibly more extravagant than an ordinary impact while remaining bounded and readable.
- **FR-015**: Optional smoke drift MAY broadly follow current battlefield wind only when it does
  not modify that wind or couple smoke to gameplay.
- **FR-016**: All models, material treatments, particle/debris effects, and smoke MUST be
  presentation-only and must not change projectile trajectories, impact positions, damage, terrain
  deformation, tank support/settling, movement, AI decisions, turn resolution, collision, or
  authoritative visibility.
- **FR-017**: The feature MUST reuse or share presentation resources where practical and reliably
  remove temporary effect entities/resources after expiry and match reset.
- **FR-018**: Lighting MAY receive only modest tuning needed for the new materials and effects to
  read clearly; the feature MUST NOT redesign the lighting or rendering architecture.
- **FR-019**: Automated coverage MUST verify valid 2–8-player cosmetic assignment, duplicate
  safety, gameplay/randomness independence, model attachment validity, profile-responsive and
  bounded effect requests, smoke expiry/cleanup, and unchanged authoritative explosion results.
- **FR-020**: Manual graphical acceptance MUST cover 2-, 4-, and 8-player tank variety and
  materials plus Basic Shell, High Explosive, MIRV, Cluster Bomb, Bomb Net, Dirt Bomb, and Nuke
  visual effects and readable long-match behaviour.
- **FR-021**: On completion, the roadmap MUST be updated only for demonstrated outcomes: Different
  visual tank models, Terrain debris, Smoke, and applicable Visual Style or Improved visuals
  items. All unrelated tank classes, visual-style aspirations, and polish work remain open.
- **FR-022**: The feature MUST NOT introduce tank gameplay classes or selection UI, asset-heavy
  detailed models, skeletal/track animation, cosmetic progression, large texture packs, foliage,
  advanced water, volumetric/fluid smoke, authoritative debris, visibility mechanics, a general
  particle editor, VFX graph, or rendering-engine rewrite.

### Key Entities

- **Cosmetic tank model**: A presentation-only silhouette and attachment layout assigned to one
  player for a match, separate from the authoritative tank.
- **Tank presentation identity**: The combination of a player's existing colour identity and their
  cosmetic model, used solely to improve recognition.
- **Battlefield surface treatment**: The visual colour and surface character for existing grass,
  dirt, rock/high terrain, and water, which never changes terrain authority.
- **Impact burst**: A bounded temporary group of cosmetic fragments requested by one resolved
  explosion and scaled from its existing magnitude.
- **Smoke plume**: A bounded temporary cosmetic aftermath of a suitable explosion that rises,
  expands, fades, and expires.
- **Authoritative explosion result**: The existing resolved impact outcome—damage, deformation,
  support consequences, and turn progress—that visual effects may observe but never modify.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Automated cases for every player count from 2 through 8 assign exactly one valid
  cosmetic model per player; each case preserves 100% of player IDs, configured order,
  controller types, and authoritative tank state, with duplicates accepted when required.
- **SC-002**: In manual 2-, 4-, and 8-player matches, observers can identify at least four
  distinct tank silhouettes when enough players are present, and player colours remain identifiable
  for every visible tank during hull movement, turret rotation, elevation, firing, and settling.
- **SC-003**: In manual normal-camera inspection, grass, dirt, rock/high terrain, water, and tanks
  all present distinguishable stylised material character; craters and Dirt Bomb deposits show no
  conspicuous stale or incoherent surface appearance.
- **SC-004**: Automated effect-state cases show that a larger existing explosion magnitude requests
  a larger or longer-lived cosmetic effect than a smaller magnitude, while per-impact and
  concurrent effect requests remain within documented bounds.
- **SC-005**: In manual Basic Shell, High Explosive, MIRV, Cluster Bomb, Bomb Net, and Nuke
  scenarios, every impact produces a readable burst; Nuke is substantially more dramatic; and
  barrage effects remain readable without persistent opaque battlefield coverage.
- **SC-006**: 100% of observed smoke and particle/debris effects expire within their configured
  finite lifetimes, including repeated-shot and match-reset scenarios.
- **SC-007**: Controlled automated comparisons with cosmetic presentation enabled versus disabled
  produce identical projectile trajectories and impacts, damage, terrain deformation, tank
  settling, turn order, and AI decisions for the same seed and gameplay inputs.
- **SC-008**: Relevant automated tests and repository quality checks pass with no new unjustified
  warnings, and representative graphical matches remain responsive through the stated stress
  cases.

## Assumptions

- The existing player-colour palette, authoritative tank pose/firing representation, impact
  magnitude, terrain mesh/colour update path, water surface, directional light, and camera framing
  are retained as the feature's compatibility baseline.
- Four to six variants is sufficient; five is an appropriate default target if it produces clear
  silhouettes without needless model proliferation.
- Cosmetic assignment can use presentation-local deterministic derivation or another isolated
  source, provided it never consumes an authoritative random stream and remains safe on match
  restart.
- Small generated or procedural surface variation is preferred; any external asset must be compact,
  permissively licensed, and recorded with its provenance.
- Smoke wind drift is desirable but optional; lack of it does not block completion.
- Existing impact scale is the default source for proportional particle and smoke presentation,
  avoiding weapon-name-specific exceptions except where an existing profile cannot express a
  required visual distinction.

## Dependencies

- Current two-to-eight-player match configuration, player identity, and controllers from
  `docs/specs/20260907-202556-match-setup-players/`.
- Current terrain, height-colour, water, world-dressing, and horizon presentation boundary from
  `docs/specs/20260908-200635-expanded-battlefield/` and
  `docs/specs/20260909-074958-presentation-polish/`.
- Current tank support, authoritative firing origin, and presentation hierarchy from the completed
  tank, aiming, movement, and settling features.
- Current profile-scaled impact pulse, shot camera, multi-projectile arsenal, Nuke, and Dirt Bomb
  presentation boundaries from the completed arsenal and impact-presentation features.
- The Azimuth Constitution, `docs/roadmap.md`, and current repository tests.

## Out of Scope

- Gameplay tank variants, armour, movement, environmental strengths, weapon restrictions, tank
  selection, or cosmetic customisation.
- Externally authored high-detail tanks, skeletal animation, tracks, damage models, decals, skins,
  unlocks, or large asset packs.
- Foliage, grass geometry, terrain physics changes, separate rock geometry, water physics,
  reflections, refraction, buoyancy, or waves that affect play.
- Volumetric smoke, fluid simulation, authoritative debris, smoke visibility/AI effects, a general
  particle system/editor, VFX graph, or a rendering-engine/lighting overhaul.
- New weapons, AI logic, camera-control redesign, audio work, HUD redesign, or terrain generation
  changes unrelated to coherent surface presentation.

## Roadmap Alignment

- This deliberately off-roadmap polish feature may complete **Different visual tank models** and
  the **Terrain debris** and **Smoke** presentation items only after their stated acceptance is
  demonstrated.
- Reassess only the Visual Style items genuinely met by the completed cohesive pass, including
  stylised low-poly battlefield, strong tank/player silhouettes, differentiated terrain, visually
  distinct players, clear explosion readability, and weapon-specific effects where delivered.
- Mark **Improved visuals** in Milestone I only if the combined implemented and accepted result
  warrants that claim. Do not mechanically check unrelated roadmap items.
- Record genuinely useful follow-on ideas—such as additional environments, richer effects, or
  measured performance work—separately rather than expanding this feature.

## Architecture Review

Completion requires affirmative review that cosmetic tank models can change without gameplay
changes; surface treatments can change without terrain-simulation changes; particles and smoke can
be disabled without explosion-result changes; cosmetic randomness is isolated from deterministic
gameplay; and the result did not create an unnecessary asset or rendering framework.
