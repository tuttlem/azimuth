# First Projectile Model

Azimuth's first projectile is deliberately small: a position and velocity moving under a constant,
configurable downward gravity value. It is gameplay physics, not a model of real-world Earth.

## Fixed Steps

Simulation advances in fixed 1/120-second steps. Rendering can run faster or slower without
changing the sequence of simulation states for the same shot and number of steps.

For each step, acceleration is `(0, -gravity, 0)`. The projectile uses the current velocity to
update position with the constant-acceleration term, then updates velocity:

```text
position = position + velocity * dt + 0.5 * acceleration * dt²
velocity = velocity + acceleration * dt
```

This makes horizontal velocity constant when gravity is the only force and gives terrain impact a
direct, understandable per-step boundary.

## Terrain Impact

The battlefield is a static grid of rendered triangles. Its local-height query uses those same
triangles, so tank grounding, projectile collision, and the visible surface agree.

After calculating each candidate fixed-step position, the projectile compares its previous and
candidate positions with the terrain. When an in-bounds segment moves from above terrain to on or
below it, the simulation refines the crossing with 24 fixed bisection iterations. This prevents a
fast projectile from tunnelling through the terrain and produces a deterministic impact position
close to the visible surface.

An impact ends flight in that simulation step and records only a terrain-impact position. The
projectile has no bounce, penetration, or damage behaviour. That same authoritative impact
immediately applies a permanent gameplay crater to the current battlefield before the firing turn
can hand control to the other player. A shot that leaves the useful simulation volume without an
in-bounds terrain crossing instead ends without an impact result or crater and still completes the
firing turn.

## Terrain Deformation

The battlefield owns one mutable grid of vertex elevations. Its current triangle interpolation is
used for rendered mesh positions, terrain-height queries, and every later swept projectile check.
Each impact lowers current grid vertices inside the default crater radius using
`depth * (1 - distance² / radius²)²`; the result is deepest at the centre and reaches zero at the
edge. Overlapping impacts subtract from the already-deformed terrain, and edge craters are clipped
to valid grid vertices.

Crater radius and depth are explicit gameplay parameters, independent of the visual boom. Tanks
remain at their startup poses when terrain changes in this feature.

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
azimuth, elevation, and launch velocity. Q/E decrease/increase azimuth by 1 degree, R/F
increase/decrease elevation by 1 degree, and T/G increase/decrease launch velocity by 0.5 abstract
units per second. Holding Shift makes those changes 5 degrees or 2.5 units per second. Azimuth
wraps from 0 through less than 360 degrees; elevation clamps to 5–85 degrees; launch velocity
clamps to 8–30 units per second.

Press Space to fire the current player's state from that player's current barrel-end firing origin.
The existing shot-parameter conversion is the only azimuth/elevation/velocity-to-launch-vector
calculation. The HUD and placeholder barrel derive from the same state. While the one projectile
is resolving, all aiming and fire input is ignored. Terrain impact applies its authoritative crater
before control changes; normal non-impact termination also changes control without an impact. Each
player's selected settings remain available when their next turn begins, enabling bracketing.

The default gravity magnitude is 8 abstract units per second squared. It is explicit rather than
an Earth declaration; launch velocity is a direct, player-visible power value for this first model.

## Intentional Boundary

The projectile is rendered as a simple sphere driven entirely by simulation state. It has no mass,
drag, wind, weapon properties, gameplay explosion, damage, or terrain deformation. It stops at
terrain impact or ends when it leaves the documented simulation volume or reaches its 20-second
simulated lifetime. A small removable development marker shows the latest terrain-impact position;
it does not affect simulation.

Shot-angle and launch-origin conventions are defined in [world-conventions.md](world-conventions.md).
