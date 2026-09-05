# World Conventions

These are the limited conventions Azimuth needs for its first visible battlefield. They provide a
shared reference for rendering and later work without defining the projectile model prematurely.

## Current Conventions

- **Vertical elevation**: Y is vertical; positive Y points upward.
- **Battlefield centre**: `(0, 0, 0)` is the centre of the initial battlefield.
- **Units**: One world-space unit is an abstract game-space unit. It has no real-world metre
  equivalence.
- **Initial bounds**: The visible battlefield is approximately 40 units wide on X and 40 units deep
  on Z, centred at the origin. Its horizontal extent is therefore approximately -20 to +20 on both
  axes.

## Deliberately Deferred

This document does not define azimuth orientation or zero direction, elevation-angle conventions,
launch-vector conversion, projectile spawn position, projectile bounds, or out-of-bounds results.
Those choices affect gameplay and will be made with the projectile and world-model work that
requires them.
