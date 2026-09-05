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

This makes horizontal velocity constant when gravity is the only force and leaves a direct,
understandable per-step boundary for later terrain collision work.

## Development Shot

Press Space to fire one development shot from Player One's firing origin. It uses the tank's turret
azimuth, 45 degrees of elevation, a launch speed of 18 abstract units per second, and a default
gravity magnitude of 8 abstract units per second squared. A second Space press while that shot is
in flight is ignored.

The gravity value is explicit and may be changed during development to compare trajectories; it is
not a declaration that Azimuth uses Earth gravity. The launch speed is likewise a development value,
not a final player-facing power scale.

## Intentional Boundary

The projectile is rendered as a simple sphere driven entirely by simulation state. It has no mass,
drag, wind, weapon properties, collision, impact, explosion, damage, or terrain deformation.
It can pass through the current terrain. The flight ends only when it leaves the documented
simulation volume or reaches its 20-second simulated lifetime.

Shot-angle and launch-origin conventions are defined in [world-conventions.md](world-conventions.md).
