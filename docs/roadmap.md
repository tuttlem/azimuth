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
* [x] Render two placeholder tanks/entities.
* [x] Launch a projectile through a configurable gravity field.
* [x] Detect projectile impact with the battlefield.
* [x] Produce a visible impact/explosion.
* [x] Deform terrain at the impact point.
* [x] Add basic aiming controls: azimuth, elevation, and power.
* [x] Implement a minimal turn loop.
* [x] Implement the move-or-fire decision.
* [x] Improve tactical controls and add presentation-only camera transitions.
* [x] Reach the first genuinely playable human-vs-human artillery match.

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

Hosting and collaboration now make CI useful. Every push to `master` validates the established
Cargo quality policy before producing downloadable native builds.

* [x] Decide whether CI is useful at the current project stage.
* [x] Add automated build validation.
* [x] Add automated tests.
* [x] Add formatting checks.
* [x] Add Clippy checks.

### Distribution Builds

* [x] Build native Windows x86_64, Linux x86_64, and macOS arm64 packages after a successful `master` quality gate.
* [x] Publish the three packages as downloadable artifacts on the corresponding GitHub Actions run.
* [x] Publish tagged `v*.*.*` builds as GitHub Releases with platform archives.

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
* [x] Render simple placeholder tank/player geometry.
* [x] Render a projectile.
* [x] Establish basic debug visualisation capabilities.

---

# 3. Coordinate System and World Model

A clear and consistent world model is required before projectile and terrain systems become complicated.

* [x] Define Azimuth's world coordinate conventions.
* [x] Define which axis represents vertical elevation.
* [x] Define world units.
* [x] Define angular conventions.
* [x] Define azimuth orientation and zero direction.
* [x] Define elevation-angle conventions.
* [x] Define projectile launch position conventions.
* [x] Define battlefield bounds.
* [x] Decide how out-of-bounds projectiles are handled.
* [x] Document conventions sufficiently for physics and rendering code to agree.
* [x] Avoid engine-specific coordinate assumptions leaking unnecessarily into game-domain logic.

---

# 4. Core Projectile Physics

The projectile system is one of Azimuth's central gameplay systems.

The objective is understandable, reproducible, tuneable behaviour rather than maximum physical realism.

## Basic Motion

* [x] Represent projectile position in 3D.
* [x] Represent projectile velocity in 3D.
* [x] Convert player azimuth, elevation, and power/velocity into a launch vector.
* [x] Apply configurable gravity.
* [x] Advance projectile motion deterministically.
* [x] Establish an appropriate simulation timestep strategy.
* [x] Test basic projectile trajectories.
* [x] Test symmetry and expected trajectory invariants where useful.

## Projectile Configuration

* [x] Support configurable launch velocity.
* [ ] Support configurable projectile mass where gameplay requires it.
* [ ] Support configurable projectile drag characteristics.
* [x] Support weapon-specific projectile behaviour without premature abstraction.
* [x] Allow projectiles to expose enough information for rendering/debugging.

## Trajectory Debugging

* [ ] Provide optional visible trajectory traces.
* [ ] Provide debug launch vectors.
* [x] Provide impact markers.
* [ ] Provide useful simulation diagnostics during development.
* [ ] Ensure development diagnostics can be disabled cleanly.

---

# 5. Gravity and Battlefield Environments

Gravity should be a first-class battlefield parameter.

## Configurable Gravity

* [x] Support configurable gravitational acceleration.
* [x] Keep gravity independent from assumptions about Earth.
* [x] Test projectile behaviour under different gravity values.
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

* [x] Represent wind as a 3D vector or appropriate environmental field.
* [x] Apply horizontal wind to projectile behaviour.
* [ ] Determine whether vertical wind should be supported.
* [x] Expose wind direction clearly to the player.
* [x] Expose wind strength clearly to the player.
* [x] Support deterministic wind conditions.
* [x] Test wind effects on projectile trajectories.

Basic wind deliberately remains a random-at-match-start but constant-during-match horizontal
projectile acceleration. Gentle shot-to-shot changes, vertical wind, environment presets, and
final balance conclusions need later play evidence.

## Wind Gameplay

* [x] Establish wind-strength ranges that remain playable.
* [x] Ensure wind effects are strong enough to matter without becoming arbitrary.
* [x] Make crosswind compensation understandable through repeated play.
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

* [x] Define initial battlefield representation.
* [x] Render non-flat terrain.
* [x] Support meaningful elevation changes.
* [x] Detect projectile intersection with terrain.
* [x] Position tanks correctly on terrain.
* [x] Determine local terrain height.
* [x] Determine local terrain slope.
* [x] Establish battlefield boundaries.

## Terrain Generation

* [ ] Support deterministic terrain generation.
* [ ] Support seeded procedural generation.
* [ ] Produce terrain that remains playable.
* [ ] Avoid tank spawn positions that are unusable.
* [ ] Avoid terrain configurations that create unwinnable matches.
* [ ] Support authored terrain eventually if useful.

## Fair Randomized Starts

* [ ] Select two deterministic, seed-reproducible starting positions for each match.
* [ ] Place tanks only on valid, supported, reasonably traversable terrain.
* [ ] Require useful minimum separation so a duel does not begin point-blank.
* [ ] Reject or regenerate starts that have an obviously unfair immediate tactical position.
* [ ] Resolve initial tank orientation and camera presentation from the selected starts.
* [ ] Preserve a match seed or equivalent development diagnostic for reproducible reports.

## Battlefield Scale and Features

Implementation note (2026-09-08): the expanded 120-unit seeded battlefield, height colouring,
presentation-only water/buildings, and valid 2–8 starts have automated coverage. The checkboxes
remain open until the required multi-seed visual and multiplayer playthroughs are performed in a
graphical session. Buildings are deliberately non-authoritative; reconsider authoritative or
destructible structures only if that presentation boundary proves confusing in play.

Implementation note (2026-09-12): a session root now derives a fresh deterministic world for each
round, with bounded retry for valid 2–8 player starts and full round-only state cleanup. Automated
coverage exercises 100 roots across all supported player counts; the visual-playthrough gate above
remains open.

* [ ] Increase the battlefield beyond the current small development arena while retaining readable
  camera framing, projectile limits, terrain queries, and deterministic simulation.
* [ ] Establish a useful playable size range for two-player duels before considering larger matches.
* [ ] Generate tactical hills, ridges, mountains, bowls, and valleys as part of terrain.
* [ ] Add simple terrain-integrated placeholder features such as buildings and trees where they
  improve line-of-fire, cover, or navigation decisions.
* [ ] Decide which features are authoritative obstacles and which are presentation-only before
  relying on them for gameplay.
* [ ] Avoid prematurely adding a general prop, foliage, destruction, or object-physics system.

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

## Large / Curved Battlefields — Later / Experimental

* [ ] Explore battlefield scales suitable for larger local multiplayer matches.
* [ ] Investigate curved or planetary battlefield geometry and the coordinate conventions it needs.
* [ ] Evaluate projectile readability, horizon visibility, and camera behaviour on curved worlds.

---

# 9. Terrain Deformation

Terrain deformation is a central Azimuth feature.

Explosions should change the tactical battlefield, not merely create visual effects.

## Craters

* [x] Create terrain deformation at projectile impact.
* [x] Generate a basic crater.
* [x] Parameterise crater radius.
* [x] Parameterise crater depth.
* [x] Ensure terrain deformation remains stable.
* [x] Update collision geometry after deformation.
* [x] Update tank positioning where terrain changes underneath them.
* [x] Ensure subsequent projectiles interact with deformed terrain.

## Tactical Effects

* [ ] Craters can create cover.
* [ ] Craters can expose previously protected players.
* [ ] Craters can obstruct movement.
* [ ] Craters can trap players where appropriate.
* [x] Terrain deformation can alter future projectile lines.
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

* [x] Represent player position.
* [x] Represent player orientation.
* [x] Represent turret orientation.
* [x] Represent weapon launch position.
* [x] Place tanks correctly on battlefield terrain.
* [x] Render a simple tank placeholder.
* [x] Associate player identity with a tank.

## Damage / Survival

Tank support settling deliberately adds no fall or terrain-related damage. Falling into a crater
already changes position and firing origin; revisit damage only if future duels show that it needs
its own readable gameplay consequence.

* [x] Establish basic health/damage rules.
* [x] Apply explosion damage based on useful gameplay rules.
* [ ] Determine direct-hit behaviour.
* [x] Determine splash-damage behaviour.
* [ ] Determine fall/terrain-related damage if appropriate.
* [x] Destroy/eliminate tanks when appropriate.

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

* [x] Adjust azimuth.
* [x] Adjust elevation.
* [x] Adjust firing power / launch velocity.
* [x] Select weapon.
* [x] Fire.
* [x] Clearly present current aiming values.
* [x] Provide sufficiently fine aiming control.
* [x] Provide sufficiently fast coarse adjustment.
* [x] Remap aiming to directional keys and power keys that remain clear after manual camera panning is retired.
* [x] Support held-key repeat for deliberate aim adjustments without making aiming automatic.

## Aiming Feedback

* [x] Display azimuth.
* [x] Display elevation.
* [x] Display power.
* [x] Display selected weapon.
* [x] Display relevant wind information.
* [ ] Display gravity/environment information.
* [ ] Display atmospheric information where applicable.
* [ ] Consider showing previous shot settings.
* [ ] Consider showing previous impact location.
* [ ] Avoid providing so much assistance that aiming becomes automatic.

## Player Skill

* [x] Preserve the ability to bracket shots.
* [x] Reward learning previous shot results.
* [x] Make changes in power/elevation produce understandable effects.
* [x] Make movement capable of disrupting established firing solutions.
* [ ] Keep aiming satisfying without requiring external calculation.

---

# 12. Turn System

Azimuth should remain fundamentally turn-based.

## Basic Turn Flow

* [x] Establish player order.
* [x] Start player turn.
* [x] Permit one primary turn action.
* [x] Resolve action fully.
* [x] Advance to next surviving player.
* [x] Detect end-of-match state.

## Move-or-Fire Decision

The intended core tactical choice is:

**move OR fire**

* [x] Implement fire as a primary turn action.
* [x] Implement movement as a primary turn action.
* [x] Prevent normal movement and firing during the same turn.
* [ ] Allow the design to revisit this rule if playtesting shows a better alternative.
* [x] Make the choice clear in the UI.

## Turn Resolution

* [x] Complete projectile flight before turn advancement.
* [x] Complete terrain deformation before turn advancement.
* [x] Complete damage resolution before turn advancement.
* [x] Complete resulting tank displacement/destruction before turn advancement.
* [x] Provide a clear transition to the next player.

---

# 13. Movement

Movement creates positional tactics and breaks established firing solutions.

## Basic Movement

* [x] Establish movement allowance.
* [x] Allow player-controlled repositioning.
* [x] Restrict movement by terrain.
* [x] Prevent movement through impassable slopes.
* [x] Prevent movement outside battlefield bounds.
* [x] Resolve final tank orientation and position.
* [x] Ensure movement remains turn-based and deliberate.

## Movement Model

* [x] Decide between continuous movement and grid/tile/step-based movement.
* [x] Evaluate movement readability.
* [x] Evaluate whether distance, slope, or terrain should consume movement allowance.
* [x] Ensure movement does not become a vehicle-driving subgame.

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

The next deliberate phase is **Arsenal**: add small, playtested weapon slices that create genuinely
different choices, beginning with multi-projectile coverage rather than merely larger explosions.

## Initial Weapons

* [x] Basic explosive shell.
* [x] High-explosive shell.
* [x] Large-radius explosive.
* [ ] Small precise projectile.
* [x] Heavy projectile with reduced wind sensitivity.

Heavy Shell has a two-round conventional implementation with a 0.40 horizontal wind-response
multiplier and Basic Shell-class impact profile. Deterministic comparison coverage confirms reduced,
non-zero response in every horizontal direction, and manual validation confirmed that its
wind-resistance is visible and useful without making it wind-immune.

## Cluster / Multi-Projectile Weapons

* [x] MIRV-style projectile.
* [x] Cluster bomb.
* [ ] Configurable split altitude or split timing if gameplay supports it.
* [x] Independent child-projectile trajectories.
* [x] Wind interaction for child projectiles.
* [ ] Death Sphere: an unusual looping or figure-eight carrier that periodically drops small explosives.

## Terrain Weapons

* [ ] Crater-focused weapon.
* [x] Deep/narrow terrain-only penetrator (Bunker Buster): massive directional excavation with no tank damage.
* [x] Terrain-building dirt bomb.
* [ ] Terrain-flattening weapon.
* [ ] Ridge/wall-forming weapon if fun.

## Terrain-Following Weapons

* [x] Rolling bomb.
* [x] Projectile affected by terrain slope after landing.
* [x] Bouncing projectile.
* [ ] Experimental tunnelling weapon if feasible.
* [ ] Expanded rolling and terrain-following weapons that turn slopes and valleys into the weapon's path.

## Area-Denial Weapons

* [ ] Napalm/fire-like persistent hazard.
* [ ] Mine.
* [ ] Delayed explosive.
* [ ] Hazardous terrain zone.
* [ ] Temporary environmental field.

## Ridiculous Weapons

Potential ideas:

* [x] Extremely large bomb.
* [ ] Deliberately unreliable experimental weapon.
* [ ] Reverse-gravity projectile.
* [ ] Projectile that changes gravity locally.
* [ ] Wind-generating weapon.
* [ ] Terrain-swapping or displacement weapon.
* [ ] Implosion weapon.
* [ ] Chain-reaction explosive.
* [ ] Weapon that fragments repeatedly.
* [x] Curve Ball: a projectile with deliberate lateral flight curvature for shots around terrain.
* [ ] Weapon whose behaviour depends strongly on atmosphere.
* [ ] Weapon designed specifically for low-gravity worlds.
* [ ] Weapon designed specifically for high-gravity worlds.
* [ ] Large area explosive, temporary hazard, terrain-destroying projectile, piercing directed weapon, and multi-warhead families inspired by classic artillery play but developed for Azimuth's own identity.
* [ ] Other ridiculous ideas discovered during play.

Weapons should remain fun, readable, and distinguishable rather than merely numerous.

---

# 15. Explosions

Explosions should be satisfying visually and mechanically.

## Gameplay

* [x] Explosion radius.
* [x] Damage falloff.
* [ ] Direct impact behaviour.
* [x] Terrain deformation.
* [ ] Force/impulse effects if useful.
* [ ] Chain reactions if introduced later.
* [x] Weapon-specific explosion profiles.

## Presentation

* [x] Explosion visual effect.
* [ ] Terrain debris.
* [ ] Smoke.
* [x] Camera response.
* [ ] Sound.
* [x] Deliberately exaggerated presentation where appropriate.

Implementation note (2026-09-10): resolved terrain impacts now produce one short, profile-scaled
screen pulse. A hit that damages any tank uses red as an immediate confirmation; terrain-only
impacts remain white. The pulse is presentation-only and does not repeat for splash damage or
elimination consequences.

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

* [x] Select number of players.
* [x] Select human/AI players at match configuration time; AI opponents are available.
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

* [x] Last surviving player wins.
* [x] Detect winner.
* [x] End match cleanly.
* [x] Present result.

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

* [x] Support multiple local players.
* [x] Clearly identify current player.
* [x] Maintain independent player state.
* [x] Maintain weapon inventories independently.
* [x] Support multiple players on one machine.
* [x] Ensure hidden information is not required for the basic game.

Potential later options:

* [ ] Hot-seat play.
* [ ] Controller support.
* [ ] Multiple input-device support.

---

# 20. AI Opponents

AI should initially exist to make the game playable alone.

It does not need to behave like a sophisticated military planner.

## Basic AI

* [x] Select targets.
* [ ] Choose weapon.
* [x] Choose azimuth.
* [x] Choose elevation.
* [x] Choose firing power.
* [x] Fire.
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
* [ ] Pan.
* [x] Zoom.
* [x] Focus current player.
* [ ] Focus selected target area where useful.
* [x] Transition behind the active player's tank when that player's turn begins.
* [x] Retire manual keyboard camera panning when its keys are needed for tactical controls.

## Projectile Camera

* [x] Track projectile in flight.
* [x] Add a golf-style shot camera: follow the fired projectile from a comfortable offset through
  landing so players can read arc and wind drift; at its apex, smoothly widen to include all living
  players and the likely impact area, then frame the explosion and its aftermath without changing
  authoritative simulation or turn timing.
* [x] Maintain awareness of surrounding terrain.
* [x] Pull back to a readable general battlefield view when a projectile launches.
* [x] Transition naturally toward impact.
* [x] Avoid nausea-inducing camera behaviour.
* [ ] Allow player to skip or accelerate long trajectories eventually if necessary.
* [x] Preserve the drama of watching shots rather than instantly resolving them.

## Impact Camera

* [x] Frame explosion.
* [x] Show affected players.
* [x] Show terrain deformation.
* [x] Return clearly to the next player's perspective.

## Tactical Camera and Repeating Controls

Directional aim controls, held repeat, active-player presentation, controller-aware Human follow
and AI tactical-wide shot coverage, shared impact framing, and shot pullback are complete.
Camera timing remains presentation-only: it does not delay firing, movement, projectile simulation,
terrain deformation, or turn advancement. Any future non-conflicting panning and trajectory-skip
control remain separate work.

---

# 22. User Interface / HUD

## Graphical Tactical HUD

* [x] Replace the temporary text-only HUD with a compact graphical tactical frame.
* [x] Show the active player's name and current turn/action state.
* [x] Show both players' health and elimination state.
* [x] Show current azimuth, elevation, and firing power.
* [x] Show movement allowance while a movement action is active.
* [x] Show wind strength and a graphical world/shot-relative direction indicator.
* [x] Keep controls as concise contextual hints rather than the HUD's primary content.
* [x] Keep HUD presentation read-only so it cannot delay or alter authoritative gameplay.

## Aiming HUD

* [x] Current player.
* [x] Weapon.
* [x] Azimuth.
* [x] Elevation.
* [x] Power.
* [x] Wind.
* [ ] Gravity.
* [ ] Atmospheric information where relevant.
* [x] Add a graphical world-axis wind indicator when the HUD gains a graphical frame.
* [x] Health.
* [x] Movement allowance where relevant.

## Match HUD

* [x] Remaining players.
* [ ] Turn order.
* [x] Current turn/action state.
* [ ] Environmental summary.
* [x] Weapon inventory.

Implementation note (2026-09-09): the player panel now derives all 2–8 ordered configured
participants, including configured names, health, elimination, and active-turn indication. A
lightweight presentation-only sky, cloud, and horizon treatment also now masks the finite
battlefield edge without changing terrain authority. Automated coverage and a two-player graphical
smoke run pass; complete the recorded 2-, 4-, and 8-player manual acceptance before claiming the
remaining HUD usability work complete.

## Usability

* [x] Keyboard controls.
* [ ] Mouse controls where appropriate.
* [ ] Controller evaluation later.
* [x] Clear input feedback.
* [x] Readable values.
* [ ] Scalable UI.
* [ ] Avoid clutter.
* [ ] Preserve focus on battlefield action.

---

# 23. Audio

Audio should make shots and impacts satisfying.

* [x] Weapon firing sound.
* [ ] Projectile flight sound.
* [x] Explosion sound.
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

Implementation note (2026-09-10): Basic, High Explosive, and Heavy Shell launches now use the
shared successful-fire boundary for an audible report, and resolved terrain impacts produce one
scaled explosion cue. Azimuth/elevation adjustments also have restrained mechanical feedback.
Projectile flight, wind ambience, destruction, UI, and terrain/debris audio remain open.

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
* [x] Projectile can be launched.
* [x] Configurable gravity affects it.
* [x] Projectile flight is visible.
* [ ] Projectile impacts the battlefield.

**At this point Azimuth should start being fun to work on.**

## Milestone C — First Boom

* [x] Projectile impact produces an explosion.
* [x] Explosion damages a target.
* [x] Explosion creates a crater.
* [x] Terrain collision updates correctly.

## Milestone D — First Duel

* [x] Two players exist.
* [x] Players can aim.
* [x] Players can fire.
* [x] Turns alternate.
* [x] Damage/elimination works.
* [x] One player can win.

**At this point Azimuth is a game.**

## Milestone E — Tactical Movement

* [x] Player can move instead of firing.
* [x] Terrain affects movement.
* [x] Movement breaks established firing solutions.
* [ ] Craters and ridges influence positional choices.

## Milestone F — Environment

* [ ] Gravity varies by battlefield.
* [x] Wind affects projectiles.
* [ ] Atmospheric drag exists.
* [ ] Environment parameters are clearly shown to players.
* [ ] At least three meaningfully different battlefield environments exist.

## Milestone G — Arsenal

* [x] Several distinct weapons exist.
* [x] At least one cluster/MIRV weapon exists.
* [x] At least one terrain-manipulation weapon exists.
* [x] At least one ridiculous weapon exists.
* [x] Weapon selection creates meaningful tactical choices.

## Economy and Between-Round Shop

* [x] Session-level player cash, actual-damage income, and placement awards.
* [x] Round accounting presentation.
* [x] Sequential between-round Human shopping with persistent weapon ammunition.
* [x] Bounded automatic AI shopping.
* [x] Per-weapon prices and a visual product catalogue shared with battlefield inventory.
* [ ] Armour as a future Shop category; its rules remain to be designed.

## Milestone H — Complete Match Experience

* [x] Match setup exists.
* [ ] Human-vs-human game flow is polished.
* [ ] AI opponent exists.
* [x] Camera transitions support the action.
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
