# Data Model: Tank Support, Gravity and Terrain Settling

## Tank support state

| Field | Rule |
|---|---|
| base position | Existing authoritative tank pose position. Its X/Z never change during settling. |
| support status | `Supported` or `Falling`; only living tanks are evaluated. |
| vertical velocity | Present only while falling; downward convention is documented and reset to zero on contact. |
| support tolerance | Shared tuneable 0.05 world units. |
| body/turret direction | Existing horizontal directions; settling preserves both. |
| health/elimination | Existing state; eliminated tanks skip support and settling. |

## Support reconciliation

The terrain height at the tank base's current X/Z is the only support input.

| Terrain relation to base | Result |
|---|---|
| Terrain at or above base | Snap base to terrain; supported; zero vertical velocity. |
| Base is at most 0.05 above terrain | Snap base to terrain; supported; zero vertical velocity. |
| Terrain is more than 0.05 below base | Mark falling with zero initial vertical velocity. |

Reconciliation occurs for every living tank immediately after terrain deformation. It does not
alter health, aim, X/Z, body direction, or turret direction.

## Fixed settling transition

| From | Condition | To | Result |
|---|---|---|---|
| Supported | Terrain lowers beyond tolerance | Falling | Begin vertical motion on later fixed settling updates. |
| Supported | Terrain remains within tolerance or rises | Supported | Base agrees exactly with current terrain. |
| Falling | Candidate base remains above current terrain | Falling | Commit deterministic downward position and velocity. |
| Falling | Candidate reaches/passes current terrain | Supported | Clamp base to terrain; clear velocity. |
| Falling | Gravity is zero and terrain remains lower | Falling | Velocity and base remain unchanged; resolving turn stays locked. |

Every falling update checks the current terrain height before committing contact, so terrain cannot
be crossed and a subsequent deformation can affect the next fixed update.

## Firing resolution state

```text
Choosing
  -> fire -> ResolvingFire / projectile active
  -> terrain impact -> existing damage once -> crater deformation
  -> reconcile every living tank
      -> no falling tanks -> survivor/match evaluation -> next player or Finished
      -> one or more falling tanks -> ResolvingFire / settling fixed updates
          -> all living tanks supported -> survivor/match evaluation -> next player or Finished

Out-of-bounds projectile -> existing immediate survivor/match evaluation -> next player or Finished
```

`ResolvingFire` remains the sole ordinary-action lock. It is not driven by the camera, explosion
visual, or renderer. The existing one-shot completion guard guarantees at most one handoff.

## Derived gameplay effects

| Consumer | Source | Required result |
|---|---|---|
| Tank visual root | tank base position | Renders the current falling or settled base height. |
| Future deliberate movement | settled base X/Y/Z plus current terrain | Starts from the final grounded position. |
| Future firing representation | settled tank pose plus retained aim | Muzzle origin changes with base height; azimuth/elevation/power do not reset. |
| Existing damage/match result | impact and health after one blast | No fall damage or duplicate blast damage. |
| Camera/boom/HUD | turn/tank/flight state | Read-only; cannot progress simulation. |
