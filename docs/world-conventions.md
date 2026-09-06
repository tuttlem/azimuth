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
  terrain height, tank support, and projectile intersection. Impacts permanently lower that
  surface; affected living tanks then settle using that same current surface.
- A projectile that travels from above to on or below that in-bounds terrain surface during a fixed
  step impacts it. The impact point is refined deterministically along that travelled segment and
  ends flight; a lightweight development marker displays the result.
- Leaving a useful-volume or lifetime limit without an in-bounds terrain crossing ends the current
  development flight without a terrain-impact result.

See [projectile-model.md](projectile-model.md) for the deterministic fixed-step motion model and
development defaults.

## Basic Wind

- Wind is one match-constant horizontal acceleration vector. A random horizontal direction and a
  gentle 0.75–1.75 units/s² strength are selected once when a match begins. It affects projectile
  position and velocity on each fixed simulation step, so its displacement accumulates with flight
  time.
- The graphical tactical HUD shows wind strength plus a world-axis plot. `+X` is drawn to the
  right and `+Z` upward; its marker is displaced in the direction the wind accelerates a
  projectile. The plot is camera- and barrel-independent, so it does not reveal an opponent's
  position or change as the camera transitions. Calm wind leaves the marker at the origin.
- Wind has no Y component in this first model. Gravity remains the only vertical environmental
  acceleration, and wind does not affect tanks, movement, support settling, explosions, terrain,
  or camera presentation.
- Wind is intentionally not compensated automatically. Observe where a shot drifts, then adjust
  azimuth, elevation, or power manually on a later firing turn.

## Tactical Movement

- At the start of a turn, the current player chooses one primary action: Space starts the existing
  firing action, while M starts movement. The player cannot normally move and fire in one turn.
- A movement action starts with six one-unit cardinal requests. Its arrow bindings are relative to
  the current camera view: Up is into the view, Down is out, and Left/Right cross it. At a diagonal
  camera yaw, the input selects the nearest cardinal world step. Enter ends movement early and
  forfeits unused requests.
  This request-based model is deterministic and deliberately does not use render-frame duration.
- A requested destination must remain within the battlefield and differ from the current terrain
  surface at the starting horizontal position by no more than 0.75 units. Valid movement sets the
  tank base exactly to the current terrain height at its destination; invalid movement changes no
  pose, body direction, or allowance.
- The tank body faces its most recent accepted movement direction. Turret azimuth, elevation, and
  power remain retained player values, so a later shot begins at the new tank position without
  automatic aim compensation.
- Terrain deformation is the sole source of movement height and passability. A later impact may
  also lower a stationary living tank's support and thereby change the position from which its
  next movement or shot begins.

## Tank Support and Settling

- A living tank uses the terrain height directly below its authoritative base point as its first
  support model. If the base is within 0.05 world units of that height, it snaps to the surface and
  remains supported; a terrain rise likewise prevents embedding.
- If deformation lowers that support by more than 0.05 units, the tank falls vertically under the
  configured battlefield gravity on the same fixed 1/120-second cadence as projectile simulation.
  Each step queries current terrain and clamps contact exactly to it. Falling preserves X/Z and
  horizontal body/turret directions; there is no sliding, tilt, suspension, or wreck physics.
- A terrain impact resolves in this order: one existing damage evaluation, terrain deformation,
  support evaluation and any living-tank settling, then survivor/match evaluation and turn handoff.
  The firing turn stays resolving throughout settling; boom and camera timing never gates it.
- Settling does not cause fall damage. Eliminated tanks do not settle. Stored azimuth, elevation,
  and power remain intact, but the settled base changes a future barrel origin and firing solution.
- With a zero-gravity battlefield, a newly unsupported tank remains unresolved/floating and keeps
  the firing turn locked. A player-facing zero-gravity resolution policy remains future work.

## Tactical Controls and Presentation

- On a choosing turn, Left/Right decrease/increase azimuth, Up/Down decrease/increase elevation,
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
- The graphical tactical HUD is presentation-only as well. Its player/health, aim/wind, and
  contextual movement/control panels derive from authoritative state but cannot mutate it or gate
  fixed simulation.

## Conventional Weapons

- `WeaponId` is stable gameplay identity; display text and inventory position are not identity.
  The central conventional weapon catalogue is the only source of ordinary weapon values.
- Each player has an independent selected weapon and ammunition availability. `1` selects the
  unlimited Basic Shell; `2` selects available High Explosive. Selection is valid only while a
  player is choosing an action and never consumes ammunition.
- Space commits exactly one available weapon to a `FiredShot`, consuming one limited round at that
  point. The immutable shot carries its impact profile throughout flight, so later selection,
  inventory, HUD, camera, or turn changes cannot affect its consequence.
- Basic Shell retains the 6-unit radius / 40 maximum-damage / 4-radius, 1.8-depth crater profile.
  High Explosive starts at two rounds per player and uses an 8-unit radius / 60 maximum-damage /
  6-radius, 3-depth crater profile. Both use the existing gravity and wind ballistics.
- Ordinary weapons are data-defined. Do not generalise a new behaviour until a real weapon needs
  it: future unusual weapons may earn a narrow explicit extension, but conventional explosive
  differences continue through the shared fired-shot and impact path.

## First Duel Damage and Victory

- Every terrain impact uses its fired weapon's authoritative gameplay damage radius independent of
  the boom. Basic Shell uses `ceil(40 * (1 - distance / 6))`; High Explosive uses
  `ceil(60 * (1 - distance / 8))`. Damage is strictly inside the relevant radius, while its edge
  and exterior deal zero damage. Both tanks start at 100 health, and the firing tank is not immune.
- Damage for both tanks is calculated from the same impact before either health changes. Zero health
  eliminates a tank. One survivor wins; no survivors draw. Damage, crater, living-tank settling,
  survivor selection, and match result settle before any turn handoff or presentation timing.
