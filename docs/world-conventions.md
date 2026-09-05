# World Conventions

These conventions give rendering, projectile simulation, and later aiming work one shared meaning.
They describe Azimuth's game world rather than a rendering API's coordinate naming.

## World Space

- **Vertical elevation**: Y is vertical; positive Y points upward.
- **Horizontal plane**: X and Z form the level battlefield plane.
- **Battlefield centre**: `(0, 0, 0)` is the centre of the initial battlefield.
- **Units**: One world-space unit is an abstract game-space unit. It has no real-world metre
  equivalence.
- **Initial visible bounds**: The visible battlefield is approximately 40 units wide on X and 40
  units deep on Z, centred at the origin. Its horizontal extent is approximately -20 to +20 on
  both axes.

## Shot Angles

- **Azimuth** is a horizontal direction in degrees. Zero degrees points along negative Z.
- **Positive azimuth** turns clockwise when viewed from above (+Y): 90 degrees points +X, 180
  degrees points +Z, and 270 degrees points -X.
- **Elevation** is measured upward from the horizontal plane. Zero degrees is level and 90 degrees
  points straight upward. The first projectile model accepts the inclusive 0 through 90 degree
  range; final player-facing aiming limits remain future work.
- Given azimuth `a` and elevation `e`, both converted from degrees to radians, the launch direction
  is `(sin(a) * cos(e), sin(e), -cos(a) * cos(e))`. It is a unit direction before launch speed is
  applied.
- A tank's normalized horizontal turret direction uses the same convention. Its azimuth is derived
  as `atan2(x, -z)`, normalized to 0 through less than 360 degrees.

## Projectile Handoff and Lifetime

- A projectile begins exactly at its tank's derived firing origin: the point ahead of and above the
  tank body at the placeholder barrel end. The firing-origin marker is a visual reference for that
  same domain point.
- The first projectile simulation has a generous useful volume: X and Z must remain within 60
  units of the origin, Y must remain from -30 through 100, and flight lasts at most 20 simulated
  seconds.
- The visible battlefield terrain is bounded from -20 to +20 on X and Z. It is a mutable 20-by-20
  grid of rendered triangles whose current piecewise planar surface is authoritative for local
  terrain height, initial tank placement, and projectile intersection. Impacts permanently lower
  that surface; tanks do not yet settle onto later terrain changes.
- A projectile that travels from above to on or below that in-bounds terrain surface during a fixed
  step impacts it. The impact point is refined deterministically along that travelled segment and
  ends flight; a lightweight development marker displays the result.
- Leaving a useful-volume or lifetime limit without an in-bounds terrain crossing ends the current
  development flight without a terrain-impact result.

See [projectile-model.md](projectile-model.md) for the deterministic fixed-step motion model and
development defaults.
