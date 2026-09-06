# Data Model: Basic Wind — First Environmental Gameplay

## Wind condition

| Field | Rule |
|---|---|
| horizontal acceleration | Existing world vector with finite X and Z components and exactly zero Y. It is the direction wind pushes a projectile **toward**. |
| strength | Horizontal magnitude in world units/s²; zero is valid. |
| default | `(+1.5, 0, 0)`, displayed as `toward +X, 1.5 units/s²`. |
| lifetime | One immutable condition for one application match; no player configuration, turn changes, or randomization. |
| invalid values | Non-finite values or any non-zero Y component are rejected. |

## Projectile fixed-step transition

| Input | Effect |
|---|---|
| gravity | Existing vertical downward acceleration only. |
| wind | Constant horizontal acceleration only. |
| combined acceleration | Used by the existing position and velocity update for every active projectile fixed step. |
| zero wind | Combined acceleration equals existing gravity acceleration exactly. |
| terrain collision | Existing swept segment/refinement consumes the resulting candidate position unchanged. |

The existing impact result remains the only input to explosion damage, crater deformation, tank
support/settling, survivor evaluation, and turn handoff.

## Presentation projection

| Presentation | Source | Authority |
|---|---|---|
| HUD wind line | current battlefield wind | Read-only |
| Projectile path | fixed-step projectile state | Authoritative simulation rendered normally |
| Camera/boom | existing turn/flight/impact state | Read-only; no wind timing control |

The HUD line is available during every in-progress turn phase. Finished winner/draw text may retain
its existing compact result-only display because wind is no longer needed for a new action.

## Explicit non-relations

| Existing state | Wind relationship |
|---|---|
| tank pose, aim, movement | No direct change. |
| tank support/settling | Uses gravity only; no wind. |
| explosion damage/crater | Unchanged rules; only the altered impact location can differ. |
| turn/match state | No new state or transition. |
| camera/presentation | Observes resulting path; cannot affect it. |
