# First Projectile Model

Azimuth's first projectile is deliberately small: a position and velocity moving under constant,
configurable gravity and a match-constant horizontal wind acceleration. It is gameplay physics,
not a model of real-world Earth.

## Fixed Steps

Simulation advances in fixed 1/120-second steps. Rendering can run faster or slower without
changing the sequence of simulation states for the same shot and number of steps.

For each step, acceleration is the sum of gravity `(0, -gravity, 0)` and the projectile's captured
wind response multiplied by match wind `(wind_x, 0, wind_z)`. Wind means the horizontal direction
it pushes a projectile **toward**. Basic Shell and High Explosive use response `1.0`; Heavy Shell
uses `0.40`, so it still drifts in wind but by 40% of the normal wind contribution. The projectile
uses the current velocity to update position with the constant-acceleration term, then updates
velocity:

```text
position = position + velocity * dt + 0.5 * acceleration * dt²
velocity = velocity + acceleration * dt
```

With calm wind, wind response produces no trajectory difference. With wind, horizontal influence
accumulates with airtime, so high and long shots drift more. Wind response is an explicit gameplay
multiplier, not projectile mass, drag, or an atmospheric model; vertical motion remains gravity's
responsibility. The swept terrain test uses this same wind-altered segment.

## Terrain Impact

The battlefield is a static grid of rendered triangles. Its local-height query uses those same
triangles, so tank grounding, projectile collision, and the visible surface agree.

The expanded 120-unit battlefield permits a small useful-volume margin beyond its edges and a
higher vertical ceiling for mountain and high-arc shots. This remains a fixed-step termination
bound, not a change to weapon launch speed, gravity, wind, or projectile behaviour. The flat water
plane and sparse building dressing are presentation-only and therefore do not participate in the
swept terrain test.

After calculating each candidate fixed-step position, the projectile compares its previous and
candidate positions with the terrain. When an in-bounds segment moves from above terrain to on or
below it, the simulation refines the crossing with 24 fixed bisection iterations. This prevents a
fast projectile from tunnelling through the terrain and produces a deterministic impact position
close to the visible surface.

An impact ends flight in that simulation step and records a terrain-impact position. The projectile
has no bounce or penetration behaviour. A fired shot captures an immutable conventional impact
profile before flight: its identity, radial damage values, crater values, and restrained visual
scale do not consult later player selection or ammunition state. That profile applies one
distance-based duel blast, creates a permanent gameplay crater, then resolves any living tanks
whose terrain support changed before the firing turn can hand control to the other player. A shot
that leaves the useful simulation volume without an in-bounds terrain crossing instead ends without
an impact result or crater and still completes the firing turn.

## Terrain Deformation

The battlefield owns one mutable grid of vertex elevations. Its current triangle interpolation is
used for rendered mesh positions, terrain-height queries, and every later swept projectile check.
Each impact lowers current grid vertices inside the fired weapon's crater radius using
`depth * (1 - distance² / radius²)²`; the result is deepest at the centre and reaches zero at the
edge. Overlapping impacts subtract from the already-deformed terrain, and edge craters are clipped
to valid grid vertices.

Crater radius and depth are explicit gameplay parameters, independent of the visual boom. A living
tank compares its base point with this same current surface after deformation. If the surface was
lowered by more than the documented support tolerance, it falls vertically under the existing
fixed-step gravity until terrain contact; it cannot pass through the current surface. This changes
the next firing origin but not retained aim values, and adds no fall damage, sliding, or wreck
physics.

## Impact Presentation

The scene observes the recorded terrain-impact position and creates one bright expanding sphere
there. Its scale grows over a short explicit visual lifetime before its renderer-owned entity is
removed. This boom is a deliberately exaggerated presentation effect: its duration and scale are
not an authoritative explosion radius and cannot affect terrain, tanks, or projectile simulation.

The terrain-impact result persists independently so the small orange marker still identifies the
exact simulation-resolved point after the boom disappears. A local presentation flag consumes each
current result once, resets when a new launch clears the result, and therefore permits a later shot
to boom even when it lands at the same world position.

## Player Aiming and Fire

Player One begins, then Player One and Player Two alternate persistent gameplay aiming states:
azimuth, elevation, and launch velocity. Left/Right decrease/increase azimuth by 1 degree,
Up/Down decrease/increase elevation by 1 degree, and -/= decrease/increase launch velocity by 0.5
abstract units per second. Holding Shift makes those changes 5 degrees or 2.5 units per second. Azimuth
wraps from 0 through less than 360 degrees; elevation clamps to 5–85 degrees; launch velocity
clamps to 8–30 units per second.

Each player selects an available weapon by clicking its compact bottom HUD box. The original
`1`–`8` shortcuts remain as convenience controls for the first eight weapons; the scalable strip,
rather than a growing set of keys, is the primary selection interface. Press Space commits the
selected weapon and fires the current player's
current barrel-end firing origin. Basic Shell uses the established 6-unit / 40-damage blast and
4-unit / 1.8-depth crater. High Explosive has two rounds per player and uses the same flight but an
8-unit / 60-damage blast and 6-unit / 3-depth crater. Heavy Shell has two rounds per player, the
Basic Shell impact profile, and a captured 0.40 wind response. A finite weapon's final round
returns the player's selection to Basic Shell.

The existing shot-parameter conversion is the only azimuth/elevation/velocity-to-launch-vector
calculation. The HUD and placeholder barrel derive from the same state. While the one projectile
or resulting tank settling is resolving, all aiming and fire input is ignored. Terrain impact
applies its authoritative crater and completes living-tank settling before control changes; normal
non-impact termination also changes control without an impact. Each player's selected settings
remain available when their next turn begins, enabling bracketing.

MIRV has two rounds per player. It launches one ordinary wind-affected carrier, then—if it survives
to the first fixed step with non-positive vertical velocity—replaces it with five deterministic
children. Each child inherits carrier state plus a small fixed horizontal separation, uses the same
gravity/wind/swept terrain test, and can impact or leave bounds independently. The firing turn does
not complete until every child is gone and any tank settling caused by the barrage is complete.

Cluster Bomb has two rounds and deploys ten small wind-affected bomblets during descent into a
compact ordered barrage. Bomb Net has one round and deploys a readable 4×4 two-axis pattern with
much wider spacing and lower individual blast power. Both remain one committed shot: a carrier is
replaced once, children use the ordinary fixed-step projectile/collision path, and the turn waits
for every child and consequent settling.

Roller has two rounds. Its ordinary ballistic carrier changes only on first terrain contact: it
retains horizontal landing momentum, samples the current authoritative terrain's downhill direction
each fixed step, stays surface-attached, and detonates after a fixed bounded rolling budget or
near-stop. Bunker Buster also has two rounds. It converts first contact into a short, bounded
displacement along its impact direction, then resolves one normal damage/deformation blast from
inside the terrain. Its crater is deliberately narrower and deeper than HE. Neither behaviour
creates a rigid-body or underground physics subsystem; both remain deterministic committed-shot
states that must resolve before turn handoff.

Arsenal Pack #2 adds four bounded behaviours. Dirt Bomb uses ordinary flight but deposits a broad,
deterministic radial mound instead of a crater and causes no conventional damage; existing tank
support reconciliation remains the sole response to raised ground. Curve Ball remains under
ordinary gravity and wind, while held Left/Right adds a small fixed-step lateral force relative to
its current horizontal travel. It never inspects targets: identical fixed-step input produces an
identical curve. Bouncer reflects velocity from the exact current terrain-triangle normal for up
to three energy-losing non-explosive contacts, then resolves its final contact normally. Nuke is
an ordinary one-round carrier with deliberately exceptional radial damage and crater parameters;
its large visual remains presentation-only.

The default gravity magnitude is 8 abstract units per second squared. At match start, wind is
sampled once with a random horizontal direction and a gentle strength from 0.75 through 1.75
units/s², then remains constant for that match. The sampled value is authoritative gameplay state;
the selection function is seed-reproducible for testing. Launch velocity remains a direct,
player-visible power value.

## Intentional Boundary

The projectile is rendered as a simple sphere driven entirely by simulation state. It has no mass,
drag, vertical wind, changing weather, or aim assistance. Its limited supported weapon-specific
property is captured horizontal wind response. Wind does not push tanks, terrain, explosions, or
the camera. It stops at terrain impact or ends when it leaves the documented simulation volume or
reaches its 20-second simulated lifetime. A small removable development marker shows the latest
terrain-impact position; it does not affect simulation.

Shot-angle and launch-origin conventions are defined in [world-conventions.md](world-conventions.md).
