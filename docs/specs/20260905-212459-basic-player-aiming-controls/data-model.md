# Data Model: Basic Player Aiming Controls

## Aiming limits

| Field | Initial value | Validation / meaning |
|-------|---------------|----------------------|
| Minimum / maximum elevation | 5.0 / 85.0 degrees | Inclusive finite player-facing bounds. |
| Minimum / maximum launch speed | 8.0 / 30.0 units/s | Inclusive finite player-facing bounds. |
| Fine / coarse angle change | 1.0 / 5.0 degrees | Normal / Shift press change. |
| Fine / coarse speed change | 0.5 / 2.5 units/s | Normal / Shift press change. |

## Aiming state

The sole mutable configuration for active Player One.

| Field | Initial value | Invariant |
|-------|---------------|-----------|
| `azimuth_degrees` | Player One startup turret azimuth | Finite and normalized to `[0, 360)`. |
| `elevation_degrees` | 45.0 | Finite and in `[5, 85]`. |
| `launch_speed` | 18.0 | Finite and in `[8, 30]`. |

| Event | Preconditions | Result |
|-------|---------------|--------|
| Fine/coarse adjustment | No active projectile | Apply matching increment; wrap azimuth or clamp bound. |
| Opposing adjustment keys | No active projectile | No change for that parameter. |
| Any aiming input during flight | Active projectile | No change. |
| Fire | No active projectile | Construct current shot; retain aim state. |
| Fire during flight | Active projectile | No change or queued projectile. |

## Derived firing representation

This is a short-lived pure value, not mutable state.

| Field | Derivation / invariant |
|-------|------------------------|
| `full_direction` | Existing canonical `ShotParameters` angle conversion. |
| `turret_forward` | Normalized X/Z projection of full direction. |
| `barrel_pivot` | Tank base position plus one vertical unit. |
| `muzzle_position` | `barrel_pivot + full_direction * 2.1`; visible marker and launch position. |
| `shot_parameters` | Muzzle plus current azimuth, elevation, and speed. |

At zero elevation the muzzle equals the old level relationship. At positive elevation horizontal advance shortens and height rises with the barrel; no independent rendering-origin calculation is allowed.

## Presentation projections

| Projection | Reads | Authority |
|------------|-------|-----------|
| Active turret/barrel/marker transforms | Derived firing representation | None |
| Aiming HUD text | Aiming state and flight activity | None |
| Projectile launch | Derived muzzle and shot parameters | Existing flight owns projectile afterward |

`InitialTanks[Player One]` supplies fixed base pose → `AimingState` supplies current values → canonical shot conversion supplies direction/velocity → derived firing representation supplies visible muzzle and launch position → existing `ProjectileFlight` owns active flight. Player Two has no aiming state in this feature.
