# Data Model: Graphical Tactical HUD

## Read-only inputs

| Source | Display use |
|---|---|
| Current turn/match | active player, phase, active aim, movement allowance, finished result |
| Both tanks | identity, health ratio/value, eliminated condition |
| Match wind | strength, normalized X/Z direction, calm state |
| Movement feedback | compact bounds/slope warning while moving |

## Derived tactical view

| Field | Rules |
|---|---|
| match result | in progress, winner, draw; result takes precedence over active turn |
| active player | present only while in progress |
| action state | choose action, moving, resolving shot, match over |
| player conditions | two records: player, current/max health, ratio, eliminated, active |
| active aim | current azimuth, elevation, power; no duplicate state |
| wind | numeric strength, normalized X/Z direction, calm flag |
| movement | remaining steps and rejection only while moving |
| controls | choose, move, or none while resolving/finished |

## Wind plot convention

- Top-down and camera-independent: `+X` is right, `+Z` is up.
- The marker means the world direction wind pushes a projectile **toward**.
- Opposite vectors place the marker opposite; calm is a neutral origin marker.
- Numeric strength remains separate from direction.

## Ownership

The retained UI tree owns only renderer-facing text, panel visibility, bar width, marker position,
and colour. It does not own or mutate turn, tank, aiming, wind, projectile, terrain, or camera
state.
