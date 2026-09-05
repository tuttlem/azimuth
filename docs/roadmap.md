# Azimuth Roadmap

Azimuth is a turn-based 3D artillery game built around understandable physics, deformable terrain, positional tactics, unusual battlefield environments, and ridiculous weapons.

The roadmap is intentionally broader than committed implementation scope.

Items recorded here represent ideas, milestones, experiments, and likely areas of development. Inclusion on the roadmap does **not** imply a commitment to implement every item.

Individual specifications should select coherent, bounded pieces of work from this roadmap.

Completed work should be checked off as its acceptance criteria are satisfied.

---

# Near-Term Milestones

These milestones represent the current intended development path.

* [x] Establish the Rust workspace and repository foundation.
* [x] Select the initial rendering/game technology deliberately.
* [x] Render a minimal 3D battlefield.
* [ ] Render two placeholder tanks/entities.
* [ ] Launch a projectile through a configurable gravity field.
* [ ] Detect projectile impact with the battlefield.
* [ ] Produce a visible impact/explosion.
* [ ] Deform terrain at the impact point.
* [ ] Add basic aiming controls: azimuth, elevation, and power.
* [ ] Implement a minimal turn loop.
* [ ] Implement the move-or-fire decision.
* [ ] Reach the first genuinely playable human-vs-human artillery match.

The first major gameplay target is:

**aim → fire → projectile arcs through the world → impact → terrain changes → next turn**

---

# 1. Project Foundation

## Repository and Workspace

* [x] Establish Azimuth as a Rust Cargo workspace.
* [x] Establish an initial minimal crate structure based on demonstrated architectural boundaries.
* [x] Ensure workspace structure remains easy to understand and navigate.
* [x] Add appropriate `.gitignore`.
* [x] Establish a useful root `README.md`.
* [x] Document how to build and run the project.
* [x] Document how to run tests and quality checks.
* [x] Establish workspace-wide development commands where useful.

## Rust Tooling

* [x] Establish the supported Rust toolchain policy.
* [x] Configure `rustfmt`.
* [x] Configure Clippy.
* [x] Establish workspace-wide build validation.
* [x] Establish workspace-wide test validation.
* [x] Establish workspace-wide lint validation.
* [x] Ensure warnings are treated intentionally rather than ignored.
* [x] Avoid unnecessary dependencies during foundation work.

## Testing Foundation

* [x] Establish unit testing conventions.
* [x] Establish integration testing conventions where needed.
* [x] Establish deterministic test patterns for simulation code.
* [x] Introduce shared testing infrastructure only when real duplication demonstrates a need.
* [x] Document how regression tests should be added for discovered gameplay bugs.

## Documentation

* [x] Create `docs/roadmap.md`.
* [x] Maintain `docs/specs/` as the location for SpecKit specifications.
* [x] Preserve timestamp-based specification naming.
* [x] Document relevant developer workflow guidance.
* [x] Keep architecture documentation lightweight and current.

## Automation / CI

CI was evaluated during the Project Foundation work and deliberately deferred. Direct local Cargo
checks currently provide the needed repository-health feedback; revisit CI when hosting or
collaboration needs create a concrete benefit.

* [ ] Decide whether CI is useful at the current project stage.
* [ ] Add automated build validation if justified.
* [ ] Add automated tests if justified.
* [ ] Add formatting checks if justified.
* [ ] Add Clippy checks if justified.

---

# 2. Rendering and Game Technology

Selection of rendering/game technology should be an explicit architectural decision rather than an incidental dependency.

## Technology Evaluation

* [x] Define the minimum technical requirements for Azimuth's first playable version.
* [x] Evaluate whether a game engine is appropriate.
* [x] Evaluate Bevy.
* [x] Evaluate lower-level alternatives where useful.
* [x] Consider direct `wgpu`-based rendering only if the additional complexity provides clear value.
* [x] Consider input, camera, UI, audio, asset handling, and terrain requirements during evaluation.
* [x] Consider build complexity and dependency footprint.
* [x] Prefer the technology that keeps the project understandable and fun to develop.
* [x] Record the rendering/game technology decision.

## Initial Rendering

* [x] Open a game window.
* [x] Establish a controllable 3D camera.
* [x] Render a simple ground plane.
* [x] Render simple placeholder battlefield geometry.
* [ ] Render simple placeholder tank/player geometry.
* [ ] Render a projectile.
* [x] Establish basic debug visualisation capabilities.

---

# 3. Coordinate System and World Model

A clear and consistent world model is required before projectile and terrain systems become complicated.

* [ ] Define Azimuth's world coordinate conventions.
* [x] Define which axis represents vertical elevation.
* [x] Define world units.
* [ ] Define angular conventions.
* [ ] Define azimuth orientation and zero direction.
* [ ] Define elevation-angle conventions.
* [ ] Define projectile launch position conventions.
* [x] Define battlefield bounds.
* [ ] Decide how out-of-bounds projectiles are handled.
* [ ] Document conventions sufficiently for physics and rendering code to agree.
* [ ] Avoid engine-specific coordinate assumptions leaking unnecessarily into game-domain logic.

---

# 4. Core Projectile Physics

The projectile system is one of Azimuth's central gameplay systems.

The objective is understandable, reproducible, tuneable behaviour rather than maximum physical realism.

## Basic Motion

* [ ] Represent projectile position in 3D.
* [ ] Represent projectile velocity in 3D.
* [ ] Convert player azimuth, elevation, and power/velocity into a launch vector.
* [ ] Apply configurable gravity.
* [ ] Advance projectile motion deterministically.
* [ ] Establish an appropriate simulation timestep strategy.
* [ ] Test basic projectile trajectories.
* [ ] Test symmetry and expected trajectory invariants where useful.

## Projectile Configuration

* [ ] Support configurable launch velocity.
* [ ] Support configurable projectile mass where gameplay requires it.
* [ ] Support configurable projectile drag characteristics.
* [ ] Support weapon-specific projectile behaviour without premature abstraction.
* [ ] Allow projectiles to expose enough information for rendering/debugging.

## Trajectory Debugging

* [ ] Provide optional visible trajectory traces.
* [ ] Provide debug launch vectors.
* [ ] Provide impact markers.
* [ ] Provide useful simulation diagnostics during development.
* [ ] Ensure development diagnostics can be disabled cleanly.

---

# 5. Gravity and Battlefield Environments

Gravity should be a first-class battlefield parameter.

## Configurable Gravity

* [ ] Support configurable gravitational acceleration.
* [ ] Keep gravity independent from assumptions about Earth.
* [ ] Test projectile behaviour under different gravity values.
* [ ] Establish sensible gameplay ranges.
* [ ] Prevent pathological environment values from breaking gameplay unexpectedly.

## Environment Presets

Potential battlefield presets include:

* [ ] Earth-like gravity.
* [ ] Low-gravity battlefield.
* [ ] Very-low-gravity / moon-like battlefield.
* [ ] High-gravity battlefield.
* [ ] Experimental extreme-gravity environments.

## Gameplay Investigation

* [ ] Evaluate how gravity affects shot readability.
* [ ] Evaluate how gravity affects projectile hang time.
* [ ] Evaluate how gravity affects battlefield scale.
* [ ] Evaluate whether gravity should affect movement.
* [ ] Tune environments for fun rather than real-world accuracy.

---

# 6. Wind

Wind is a major source of artillery uncertainty and skill expression.

## Basic Wind

* [ ] Represent wind as a 3D vector or appropriate environmental field.
* [ ] Apply horizontal wind to projectile behaviour.
* [ ] Determine whether vertical wind should be supported.
* [ ] Expose wind direction clearly to the player.
* [ ] Expose wind strength clearly to the player.
* [ ] Support deterministic wind conditions.
* [ ] Test wind effects on projectile trajectories.

## Wind Gameplay

* [ ] Establish wind-strength ranges that remain playable.
* [ ] Ensure wind effects are strong enough to matter without becoming arbitrary.
* [ ] Make crosswind compensation understandable through repeated play.
* [ ] Explore battlefield presets with characteristic wind behaviour.

## Advanced Wind — Later / Experimental

* [ ] Altitude-dependent wind.
* [ ] Multiple atmospheric wind layers.
* [ ] Different wind direction at different altitudes.
* [ ] Time-varying wind between turns.
* [ ] Gusting or turbulent environments.
* [ ] Environmental hazards involving unusual wind patterns.

Advanced wind should only be implemented if basic wind proves enjoyable and understandable.

---

# 7. Atmosphere and Drag

Atmospheric behaviour can give battlefields distinct identities.

It should remain a gameplay system rather than an atmospheric simulation project.

## Basic Atmospheric Model

* [ ] Introduce a simple atmospheric density/pressure parameter.
* [ ] Apply simple projectile drag.
* [ ] Make drag coefficients tuneable.
* [ ] Allow different projectile types to react differently to atmosphere.
* [ ] Test atmospheric effects independently from rendering.

## Gameplay Effects

* [ ] Thin-atmosphere environment.
* [ ] Earth-like atmosphere.
* [ ] Dense-atmosphere environment.
* [ ] Evaluate how density affects range.
* [ ] Evaluate how density changes wind influence.
* [ ] Evaluate how weapon choice interacts with atmosphere.

## Altitude Effects — Later / Experimental

* [ ] Investigate altitude-dependent atmospheric density.
* [ ] Consider simple exponential density reduction with altitude.
* [ ] Explore high-arc shots that pass through thinner atmosphere.
* [ ] Evaluate whether altitude-dependent effects are understandable enough to retain.
* [ ] Avoid implementing atmospheric complexity without demonstrated gameplay value.

---

# 8. Battlefield Terrain

Terrain should affect both aiming and tactical position.

## Basic Terrain

* [ ] Define initial battlefield representation.
* [x] Render non-flat terrain.
* [x] Support meaningful elevation changes.
* [ ] Detect projectile intersection with terrain.
* [ ] Position tanks correctly on terrain.
* [ ] Determine local terrain height.
* [ ] Determine local terrain slope.
* [x] Establish battlefield boundaries.

## Terrain Generation

* [ ] Support deterministic terrain generation.
* [ ] Support seeded procedural generation.
* [ ] Produce terrain that remains playable.
* [ ] Avoid tank spawn positions that are unusable.
* [ ] Avoid terrain configurations that create unwinnable matches.
* [ ] Support authored terrain eventually if useful.

## Terrain Styles

Potential terrain styles:

* [ ] Rolling hills.
* [ ] Mountainous battlefield.
* [ ] Caldera/bowl.
* [ ] Canyon battlefield.
* [ ] Plateau battlefield.
* [ ] Cratered wasteland.
* [ ] Low-relief open battlefield.
* [ ] Experimental alien terrain.

---

# 9. Terrain Deformation

Terrain deformation is a central Azimuth feature.

Explosions should change the tactical battlefield, not merely create visual effects.

## Craters

* [ ] Create terrain deformation at projectile impact.
* [ ] Generate a basic crater.
* [ ] Parameterise crater radius.
* [ ] Parameterise crater depth.
* [ ] Ensure terrain deformation remains stable.
* [ ] Update collision geometry after deformation.
* [ ] Update tank positioning where terrain changes underneath them.
* [ ] Ensure subsequent projectiles interact with deformed terrain.

## Tactical Effects

* [ ] Craters can create cover.
* [ ] Craters can expose previously protected players.
* [ ] Craters can obstruct movement.
* [ ] Craters can trap players where appropriate.
* [ ] Terrain deformation can alter future projectile lines.
* [ ] Repeated impacts can substantially reshape a battlefield.

## Advanced Terrain Effects

* [ ] Raised terrain / dirt deposition.
* [ ] Terrain-building weapons.
* [ ] Narrow penetrator-style deformation.
* [ ] Wide shallow explosive deformation.
* [ ] Cascading terrain changes if useful.
* [ ] Terrain erosion/collapse experiments.
* [ ] Destructible or unstable ridges if gameplay supports it.

---

# 10. Tanks / Player Entities

Tanks should initially be simple game pieces rather than realistic vehicles.

## Basic Tank Representation

* [ ] Represent player position.
* [ ] Represent player orientation.
* [ ] Represent turret orientation.
* [ ] Represent weapon launch position.
* [ ] Place tanks correctly on battlefield terrain.
* [ ] Render a simple tank placeholder.
* [ ] Associate player identity with a tank.

## Damage / Survival

* [ ] Establish basic health/damage rules.
* [ ] Apply explosion damage based on useful gameplay rules.
* [ ] Determine direct-hit behaviour.
* [ ] Determine splash-damage behaviour.
* [ ] Determine fall/terrain-related damage if appropriate.
* [ ] Destroy/eliminate tanks when appropriate.

## Later Tank Variety

* [ ] Different visual tank models.
* [ ] Different movement capabilities.
* [ ] Different armour characteristics.
* [ ] Different weapon carrying capabilities.
* [ ] Different environmental strengths/weaknesses.

Tank differentiation should only be added if it improves gameplay rather than creating balancing overhead.

---

# 11. Aiming

Aiming is the player's primary interaction with the physics system.

## Core Controls

* [ ] Adjust azimuth.
* [ ] Adjust elevation.
* [ ] Adjust firing power / launch velocity.
* [ ] Select weapon.
* [ ] Fire.
* [ ] Clearly present current aiming values.
* [ ] Provide sufficiently fine aiming control.
* [ ] Provide sufficiently fast coarse adjustment.

## Aiming Feedback

* [ ] Display azimuth.
* [ ] Display elevation.
* [ ] Display power.
* [ ] Display selected weapon.
* [ ] Display relevant wind information.
* [ ] Display gravity/environment information.
* [ ] Display atmospheric information where applicable.
* [ ] Consider showing previous shot settings.
* [ ] Consider showing previous impact location.
* [ ] Avoid providing so much assistance that aiming becomes automatic.

## Player Skill

* [ ] Preserve the ability to bracket shots.
* [ ] Reward learning previous shot results.
* [ ] Make changes in power/elevation produce understandable effects.
* [ ] Make movement capable of disrupting established firing solutions.
* [ ] Keep aiming satisfying without requiring external calculation.

---

# 12. Turn System

Azimuth should remain fundamentally turn-based.

## Basic Turn Flow

* [ ] Establish player order.
* [ ] Start player turn.
* [ ] Permit one primary turn action.
* [ ] Resolve action fully.
* [ ] Advance to next surviving player.
* [ ] Detect end-of-match state.

## Move-or-Fire Decision

The intended core tactical choice is:

**move OR fire**

* [ ] Implement fire as a primary turn action.
* [ ] Implement movement as a primary turn action.
* [ ] Prevent normal movement and firing during the same turn.
* [ ] Allow the design to revisit this rule if playtesting shows a better alternative.
* [ ] Make the choice clear in the UI.

## Turn Resolution

* [ ] Complete projectile flight before turn advancement.
* [ ] Complete terrain deformation before turn advancement.
* [ ] Complete damage resolution before turn advancement.
* [ ] Complete resulting tank displacement/destruction before turn advancement.
* [ ] Provide a clear transition to the next player.

---

# 13. Movement

Movement creates positional tactics and breaks established firing solutions.

## Basic Movement

* [ ] Establish movement allowance.
* [ ] Allow player-controlled repositioning.
* [ ] Restrict movement by terrain.
* [ ] Prevent movement through impassable slopes.
* [ ] Prevent movement outside battlefield bounds.
* [ ] Resolve final tank orientation and position.
* [ ] Ensure movement remains turn-based and deliberate.

## Movement Model

* [ ] Decide between continuous movement and grid/tile/step-based movement.
* [ ] Evaluate movement readability.
* [ ] Evaluate whether distance, slope, or terrain should consume movement allowance.
* [ ] Ensure movement does not become a vehicle-driving subgame.

## Tactical Movement

* [ ] Use terrain for cover.
* [ ] Escape a bracketed firing solution.
* [ ] Move toward advantageous elevation.
* [ ] Move out of craters.
* [ ] Move behind ridges.
* [ ] Move into firing positions.

---

# 14. Weapons

Weapons should range from simple artillery shells to absurd toys.

Every weapon should create a meaningful gameplay difference.

## Initial Weapons

* [ ] Basic explosive shell.
* [ ] High-explosive shell.
* [ ] Large-radius explosive.
* [ ] Small precise projectile.
* [ ] Heavy projectile with reduced wind sensitivity.

## Cluster / Multi-Projectile Weapons

* [ ] MIRV-style projectile.
* [ ] Cluster bomb.
* [ ] Configurable split altitude or split timing if gameplay supports it.
* [ ] Independent child-projectile trajectories.
* [ ] Wind interaction for child projectiles.

## Terrain Weapons

* [ ] Crater-focused weapon.
* [ ] Deep/narrow penetrator.
* [ ] Terrain-building dirt bomb.
* [ ] Terrain-flattening weapon.
* [ ] Ridge/wall-forming weapon if fun.

## Terrain-Following Weapons

* [ ] Rolling bomb.
* [ ] Projectile affected by terrain slope after landing.
* [ ] Bouncing projectile.
* [ ] Experimental tunnelling weapon if feasible.

## Area-Denial Weapons

* [ ] Napalm/fire-like persistent hazard.
* [ ] Mine.
* [ ] Delayed explosive.
* [ ] Hazardous terrain zone.
* [ ] Temporary environmental field.

## Ridiculous Weapons

Potential ideas:

* [ ] Extremely large bomb.
* [ ] Deliberately unreliable experimental weapon.
* [ ] Reverse-gravity projectile.
* [ ] Projectile that changes gravity locally.
* [ ] Wind-generating weapon.
* [ ] Terrain-swapping or displacement weapon.
* [ ] Implosion weapon.
* [ ] Chain-reaction explosive.
* [ ] Weapon that fragments repeatedly.
* [ ] Weapon whose behaviour depends strongly on atmosphere.
* [ ] Weapon designed specifically for low-gravity worlds.
* [ ] Weapon designed specifically for high-gravity worlds.
* [ ] Other ridiculous ideas discovered during play.

Weapons should remain fun, readable, and distinguishable rather than merely numerous.

---

# 15. Explosions

Explosions should be satisfying visually and mechanically.

## Gameplay

* [ ] Explosion radius.
* [ ] Damage falloff.
* [ ] Direct impact behaviour.
* [ ] Terrain deformation.
* [ ] Force/impulse effects if useful.
* [ ] Chain reactions if introduced later.
* [ ] Weapon-specific explosion profiles.

## Presentation

* [ ] Explosion visual effect.
* [ ] Terrain debris.
* [ ] Smoke.
* [ ] Camera response.
* [ ] Sound.
* [ ] Deliberately exaggerated presentation where appropriate.

---

# 16. Environmental Battlefield Identity

Battlefields should differ through physical rules as well as terrain.

## Environment Parameters

* [ ] Gravity.
* [ ] Atmospheric density.
* [ ] Wind strength.
* [ ] Wind direction.
* [ ] Wind profile where supported.
* [ ] Battlefield size.
* [ ] Terrain characteristics.

## Example Environment Concepts

### Standard

* [ ] Familiar gravity.
* [ ] Moderate atmosphere.
* [ ] Moderate wind.
* [ ] General-purpose battlefield.

### The Moon

* [ ] Very low gravity.
* [ ] Essentially no atmosphere.
* [ ] No meaningful wind.
* [ ] Extremely long projectile arcs.

### The Storm

* [ ] Normal-ish gravity.
* [ ] Dense atmosphere.
* [ ] Strong wind.
* [ ] Potential layered wind.

### The Crusher

* [ ] High gravity.
* [ ] Shorter, more violent projectile arcs.
* [ ] Strong emphasis on terrain and direct trajectories.

### Thin World

* [ ] Low gravity.
* [ ] Thin atmosphere.
* [ ] Weak drag.
* [ ] Huge ballistic ranges.

### The Bowl

* [ ] Standard environmental physics.
* [ ] Players positioned around a large caldera or depression.
* [ ] Terrain itself creates unusual trajectories and cover.

## Experimental Environment Concepts

* [ ] Extremely dense atmosphere.
* [ ] Strong vertical winds.
* [ ] Alternating wind layers.
* [ ] Very low gravity with enormous terrain.
* [ ] Changing gravity between rounds.
* [ ] Unusual environmental hazards.
* [ ] Tiny-world/orbital-style projectile experiments only if practical and fun.

---

# 17. Match Setup

* [ ] Select number of players.
* [ ] Select human/AI players.
* [ ] Select battlefield.
* [ ] Select environment preset.
* [ ] Select random seed where appropriate.
* [ ] Configure starting weapons.
* [ ] Configure health.
* [ ] Configure turn order rules.
* [ ] Configure optional gameplay modifiers.
* [ ] Start match.

---

# 18. Match Rules and Victory

## Basic Victory

* [ ] Last surviving player wins.
* [ ] Detect winner.
* [ ] End match cleanly.
* [ ] Present result.

## Possible Variants

* [ ] Team matches.
* [ ] Score-based matches.
* [ ] Limited-turn matches.
* [ ] Objective modes.
* [ ] Environmental survival modes.
* [ ] Weapon-restricted matches.
* [ ] Low-gravity-specific modes.
* [ ] Experimental party modes.

Do not add game modes until the core artillery loop is strong.

---

# 19. Human-vs-Human Play

Local multiplayer should be the simplest path to the first complete game.

* [ ] Support multiple local players.
* [ ] Clearly identify current player.
* [ ] Maintain independent player state.
* [ ] Maintain weapon inventories independently.
* [ ] Support multiple players on one machine.
* [ ] Ensure hidden information is not required for the basic game.

Potential later options:

* [ ] Hot-seat play.
* [ ] Controller support.
* [ ] Multiple input-device support.

---

# 20. AI Opponents

AI should initially exist to make the game playable alone.

It does not need to behave like a sophisticated military planner.

## Basic AI

* [ ] Select targets.
* [ ] Choose weapon.
* [ ] Choose azimuth.
* [ ] Choose elevation.
* [ ] Choose firing power.
* [ ] Fire.
* [ ] Occasionally move instead of firing.

## AI Skill Levels

Potential skill differences:

* [ ] Aim error.
* [ ] Memory of previous shots.
* [ ] Ability to bracket targets.
* [ ] Wind compensation ability.
* [ ] Environmental understanding.
* [ ] Tactical movement ability.
* [ ] Weapon-selection quality.

## AI Personality

Potential later personalities:

* [ ] Aggressive.
* [ ] Conservative.
* [ ] Terrain manipulator.
* [ ] Heavy-weapons enthusiast.
* [ ] Mobile/evasive.
* [ ] Wildly inaccurate chaos player.

The AI should be entertaining before it is optimal.

---

# 21. Camera

Watching the shot is part of the game.

## Battlefield Camera

* [x] Orbit battlefield.
* [x] Pan.
* [x] Zoom.
* [ ] Focus current player.
* [ ] Focus selected target area where useful.

## Projectile Camera

* [ ] Track projectile in flight.
* [ ] Maintain awareness of surrounding terrain.
* [ ] Transition naturally toward impact.
* [ ] Avoid nausea-inducing camera behaviour.
* [ ] Allow player to skip or accelerate long trajectories eventually if necessary.
* [ ] Preserve the drama of watching shots rather than instantly resolving them.

## Impact Camera

* [ ] Frame explosion.
* [ ] Show affected players.
* [ ] Show terrain deformation.
* [ ] Return clearly to the next player's perspective.

---

# 22. User Interface / HUD

## Aiming HUD

* [ ] Current player.
* [ ] Weapon.
* [ ] Azimuth.
* [ ] Elevation.
* [ ] Power.
* [ ] Wind.
* [ ] Gravity.
* [ ] Atmospheric information where relevant.
* [ ] Health.
* [ ] Movement allowance where relevant.

## Match HUD

* [ ] Remaining players.
* [ ] Turn order.
* [ ] Current turn/action state.
* [ ] Environmental summary.
* [ ] Weapon inventory.

## Usability

* [ ] Keyboard controls.
* [ ] Mouse controls where appropriate.
* [ ] Controller evaluation later.
* [ ] Clear input feedback.
* [ ] Readable values.
* [ ] Scalable UI.
* [ ] Avoid clutter.
* [ ] Preserve focus on battlefield action.

---

# 23. Audio

Audio should make shots and impacts satisfying.

* [ ] Weapon firing sound.
* [ ] Projectile flight sound.
* [ ] Explosion sound.
* [ ] Terrain/debris sound.
* [ ] Tank destruction sound.
* [ ] UI sounds.
* [ ] Environmental ambience.
* [ ] Wind ambience where appropriate.
* [ ] Music only if it improves the game's tone.

Potential stylistic direction:

* [ ] Slightly exaggerated arcade-like effects.
* [ ] Distinctive audio identity for unusual weapons.
* [ ] Dramatic silence / anticipation during long projectile arcs where effective.

---

# 24. Visual Style

The game should favour readability and charm over visual realism.

Potential direction:

* [ ] Stylised low-poly battlefield.
* [ ] Strong tank/player silhouettes.
* [ ] Clear projectile visibility.
* [ ] Clear explosion radius/readability.
* [ ] Clearly differentiated terrain.
* [ ] Strong environmental visual identity.
* [ ] Visually distinct players.
* [ ] Weapon-specific effects.

Avoid allowing asset production requirements to delay the playable game.

Placeholder geometry is acceptable for as long as it supports gameplay development.

---

# 25. Determinism and Replays

Determinism may enable useful debugging and later gameplay features.

## Reproducibility

* [ ] Seed procedural terrain generation.
* [ ] Seed gameplay randomness.
* [ ] Record environment parameters.
* [ ] Reproduce projectile behaviour from the same inputs.
* [ ] Make bug scenarios easy to recreate.

## Possible Replay Features

* [ ] Record player actions.
* [ ] Replay completed matches.
* [ ] Replay individual shots.
* [ ] Share interesting seeds.
* [ ] Share interesting shots.
* [ ] Slow-motion replay of impacts.

Replay features are optional unless they prove particularly useful for debugging or gameplay.

---

# 26. Save and Configuration

Later, once the game has meaningful persistent configuration:

* [ ] Player preferences.
* [ ] Input bindings.
* [ ] Audio settings.
* [ ] Graphics settings.
* [ ] Match presets.
* [ ] Environment presets.
* [ ] Custom battlefield seeds.
* [ ] Save/load configuration cleanly.

Avoid introducing persistence infrastructure before there is meaningful state to persist.

---

# 27. Performance

Performance work should be evidence-driven.

* [ ] Establish acceptable frame-rate target.
* [ ] Establish reasonable maximum battlefield complexity.
* [ ] Measure terrain deformation cost.
* [ ] Measure projectile simulation cost.
* [ ] Measure particle/effect cost.
* [ ] Profile before optimising.
* [ ] Avoid premature optimisation of simple systems.
* [ ] Preserve deterministic simulation behaviour where practical during optimisation.

---

# 28. Networking / Multiplayer — Future

Network multiplayer is intentionally not an early requirement.

Turn-based gameplay may make networking practical later, but it should not influence the initial architecture unnecessarily.

Potential future work:

* [ ] Define multiplayer requirements.
* [ ] Determine authoritative simulation model.
* [ ] Share deterministic turn inputs rather than continuous world state where practical.
* [ ] Synchronise seeds and environmental configuration.
* [ ] Synchronise terrain deformation.
* [ ] Handle player disconnection.
* [ ] Matchmaking / direct connection if desired.
* [ ] Spectator support if useful.

Do not build networking infrastructure until the local game is genuinely worth playing.

---

# 29. Modding / Data-Driven Content — Future

Only consider this after real content demonstrates what needs to be configurable.

Potential later work:

* [ ] Data-driven weapons.
* [ ] Data-driven environment presets.
* [ ] Data-driven battlefields.
* [ ] Custom weapon definitions.
* [ ] Custom physics parameters.
* [ ] Community battlefield presets.
* [ ] Mod-loading strategy.

Avoid designing a generic mod framework before the game itself establishes stable concepts.

---

# 30. Experimental Gameplay Backlog

These ideas are deliberately speculative.

They should remain here until playtesting demonstrates a reason to pursue them.

* [ ] Destructible terrain that can bury tanks.
* [ ] Earthquake weapon.
* [ ] Local gravity manipulation.
* [ ] Projectile teleportation.
* [ ] Wormhole weapon.
* [ ] Ricochet weapons.
* [ ] Homing weapon with deliberately limited correction.
* [ ] Remote-detonated projectile.
* [ ] Delayed fuse.
* [ ] Airburst.
* [ ] Proximity fuse.
* [ ] Bounce count configuration.
* [ ] Parachute mine.
* [ ] Wind-sensitive lightweight projectile.
* [ ] Extremely heavy wind-resistant projectile.
* [ ] Weapon that creates temporary cover.
* [ ] Weapon that removes cover without dealing much direct damage.
* [ ] Temporary shields.
* [ ] Player-created defensive terrain.
* [ ] Moving environmental hazards.
* [ ] Lava or damaging terrain.
* [ ] Ice/slippery terrain.
* [ ] Explosive terrain features.
* [ ] Weather events.
* [ ] Environmental changes between rounds.
* [ ] Tanks affected by blast impulse.
* [ ] Tanks falling into newly created craters.
* [ ] Multi-stage weapons.
* [ ] Chain reaction weapons.
* [ ] Extremely long-duration high-altitude shots.
* [ ] Strange worlds with unusual physics.
* [ ] Completely ridiculous weapons invented during development.

---

# 31. Development Milestones

These are conceptual milestones rather than fixed specification sequences.

## Milestone A — Foundation

* [x] Rust workspace exists.
* [x] Repository builds cleanly.
* [x] Tests and quality checks run successfully.
* [x] Documentation and roadmap exist.
* [x] SpecKit workflow is established.

## Milestone B — First Arc

* [x] Game window opens.
* [x] Basic 3D world renders.
* [ ] Projectile can be launched.
* [ ] Configurable gravity affects it.
* [ ] Projectile flight is visible.
* [ ] Projectile impacts the battlefield.

**At this point Azimuth should start being fun to work on.**

## Milestone C — First Boom

* [ ] Projectile impact produces an explosion.
* [ ] Explosion damages a target.
* [ ] Explosion creates a crater.
* [ ] Terrain collision updates correctly.

## Milestone D — First Duel

* [ ] Two players exist.
* [ ] Players can aim.
* [ ] Players can fire.
* [ ] Turns alternate.
* [ ] Damage/elimination works.
* [ ] One player can win.

**At this point Azimuth is a game.**

## Milestone E — Tactical Movement

* [ ] Player can move instead of firing.
* [ ] Terrain affects movement.
* [ ] Movement breaks established firing solutions.
* [ ] Craters and ridges influence positional choices.

## Milestone F — Environment

* [ ] Gravity varies by battlefield.
* [ ] Wind affects projectiles.
* [ ] Atmospheric drag exists.
* [ ] Environment parameters are clearly shown to players.
* [ ] At least three meaningfully different battlefield environments exist.

## Milestone G — Arsenal

* [ ] Several distinct weapons exist.
* [ ] At least one cluster/MIRV weapon exists.
* [ ] At least one terrain-manipulation weapon exists.
* [ ] At least one ridiculous weapon exists.
* [ ] Weapon selection creates meaningful tactical choices.

## Milestone H — Complete Match Experience

* [ ] Match setup exists.
* [ ] Human-vs-human game flow is polished.
* [ ] AI opponent exists.
* [ ] Camera transitions support the action.
* [ ] UI clearly communicates aiming/environment state.
* [ ] Audio gives shots and impacts satisfying weight.
* [ ] A full match can be started, played, won, and restarted without developer tooling.

## Milestone I — Polish and Expansion

* [ ] More environments.
* [ ] More weapons.
* [ ] Improved visuals.
* [ ] Improved audio.
* [ ] More capable AI.
* [ ] Additional match options.
* [ ] Performance tuning based on measurement.

## Milestone J — Optional Future Expansion

* [ ] Network multiplayer evaluation.
* [ ] Replay sharing.
* [ ] Modding/data-driven content.
* [ ] Additional game modes.
* [ ] Community-oriented features if the project reaches that stage.

---

# Guiding Test

When deciding whether an item from this roadmap should become active work, ask:

**Will this make Azimuth more fun to play, more fun to build, or materially easier to understand and maintain?**

If not, it can stay on the roadmap.

The project does not need every idea recorded here.

It needs the right ones.
