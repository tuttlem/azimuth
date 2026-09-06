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

- A projectile begins exactly at its active tank's derived firing origin: the placeholder barrel
  pivot is one unit above the tank base and its barrel end is 2.1 units along the canonical full
  shot direction. The firing-origin marker is a visual reference for that same domain point, so
  elevation moves the marker upward and shortens its horizontal advance with the visible barrel.
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

## Tactical Movement

- At the start of a turn, the current player chooses one primary action: Space starts the existing
  firing action, while M starts movement. The player cannot normally move and fire in one turn.
- A movement action starts with six one-unit cardinal requests: I is negative Z, J is negative X,
  K is positive Z, and L is positive X. Enter ends movement early and forfeits unused requests.
  This request-based model is deterministic and deliberately does not use render-frame duration.
- A requested destination must remain within the battlefield and differ from the current terrain
  surface at the starting horizontal position by no more than 0.75 units. Valid movement sets the
  tank base exactly to the current terrain height at its destination; invalid movement changes no
  pose, body direction, or allowance.
- The tank body faces its most recent accepted movement direction. Turret azimuth, elevation, and
  power remain retained player values, so a later shot begins at the new tank position without
  automatic aim compensation.
- Terrain deformation is the sole source of movement height and passability. Tanks intentionally
  do not settle after later impacts below a stationary pose; that remains separate work.

## Tactical Controls and Presentation

- On a choosing turn, Left/Right decrease/increase azimuth, Up/Down increase/decrease elevation,
  and `-`/`=` decrease/increase power. Each press applies one fine adjustment immediately; a held
  eligible key repeats after 300 ms and then every 100 ms. Shift uses the existing coarse amount.
  Opposite keys on the same axis cancel rather than selecting an arbitrary direction.
- Keyboard panning is deliberately retired. Right-mouse drag orbits and the mouse wheel zooms;
  neither control affects authoritative game state.
- Camera presentation observes the current player and flight state only. A choosing turn seeks a
  bounded behind-or-near view of the active tank based on the barrel's retained horizontal aim;
  changing azimuth smoothly carries that view around the horizontal axis. A fired shot seeks a
  wider bounded battlefield view. Render-time interpolation can be interrupted by newer turn or
  flight state, and never delays input, projectile simulation, terrain deformation, or handoff.

## First Duel Damage and Victory

- Every terrain impact has an authoritative 6-unit gameplay damage radius independent of the boom.
  Its full 3D distance to each tank base determines `ceil(40 * (1 - distance / 6))` damage strictly
  inside the radius; the edge and exterior deal zero damage. Both tanks start at 100 health, and
  the firing tank is not immune.
- Damage for both tanks is calculated from the same impact before either health changes. Zero health
  eliminates a tank. One survivor wins; no survivors draw. Damage, crater, survivor selection, and
  match result settle before any turn handoff or presentation timing.
