# Data Model: Minimal Turn Loop

## Existing player identity and order

The feature reuses the two existing player identities. Their order is fixed and does not support
insertion, removal, teams, or configuration.

| Player | Initial role | Next player |
|--------|--------------|-------------|
| Player One | Starts the match Ready | Player Two |
| Player Two | Becomes Ready after Player One resolves | Player One |

## Turn state

| Field | Values | Invariant |
|-------|--------|-----------|
| Current player | Existing Player One or Player Two identity | The only player whose tank and aim may receive local interaction. |
| Phase | Ready or Resolving | Ready accepts aim/fire; Resolving accepts neither. |

Initial state is Player One/Ready. The phase is explicit gameplay state; it is not inferred from
HUD text, visual tags, explosion entities, or rendering time.

| Event | Preconditions | Result |
|-------|---------------|--------|
| Aim adjustment | Ready | Adjust only current player's retained aim. |
| Fire | Ready | Capture current player and current aim for launch; enter Resolving. |
| Fire or aim input | Resolving | No state change and no queued action. |
| Projectile fixed step reports Active | Resolving | Retain current player and Resolving state. |
| Terrain-impact termination | Resolving | Apply crater and record impact, then select the other player and enter Ready. |
| Non-impact termination | Resolving | Leave terrain/impact absent, then select the other player and enter Ready. |

`complete_resolution` is valid only from Resolving. It switches directly between the two existing
identities and cannot skip or select a winner.

## Per-player aiming state

Each existing player owns a separate existing `AimingState`.

| Value | Player One initial source | Player Two initial source | Invariant |
|-------|---------------------------|---------------------------|-----------|
| Azimuth | That tank's initial turret direction | That tank's initial turret direction | Finite and normalized to `[0, 360)`. |
| Elevation | Existing default of 45 degrees | Existing default of 45 degrees | Finite and in `[5, 85]`. |
| Launch speed | Existing default of 18 units/s | Existing default of 18 units/s | Finite and in `[8, 30]`. |

The existing `AimingState` continues to own normalization, bounds, fine/coarse adjustment amounts,
and conversion to canonical shot parameters. Selecting a player returns that player's state; it
does not copy values into an alternative active-aim authority.

## Resolution boundary and presentation projections

| Value/projection | Reads | Authority/invariant |
|------------------|-------|---------------------|
| Projectile flight | Launching player's current tank and retained aim | Existing fixed simulation owns it until terminal outcome. |
| Terrain impact and crater | Terminal terrain-impact result | Crater applies to mutable authoritative terrain before turn completion. |
| Current-player tank firing representation | Current player, that tank, that player's aim | Turret, barrel, muzzle, and launch agree; it cannot decide current player. |
| HUD | Turn state and current player's retained aim | Identifies current player and Ready/Resolving state; presentation only. |
| Mesh, marker, boom | Terrain and recorded impact | Derived presentation only; boom lifetime never delays a turn. |

For terrain impact, stable authoritative gameplay state means the projectile has terminated, the
impact is recorded, and crater mutation has completed. For non-impact termination, stable state
means the projectile has terminated with no impact or crater. Both conditions permit exactly one
turn transition.
