# Player Controls Contract: Basic Aiming

This is the external keyboard and visual-feedback contract for the running desktop game. It is not a configurable binding system.

## Active tank

Player One (the existing red tank) is the only controllable tank. Player Two remains visible as a reference and target. No turn starts, ends, or switches from these controls.

## Idle controls

| Input | Result | Fine change | Shift coarse change |
|-------|--------|-------------|---------------------|
| Q / E | Decrease / increase azimuth | 1 degree | 5 degrees |
| R / F | Increase / decrease elevation | 1 degree | 5 degrees |
| T / G | Increase / decrease launch velocity | 0.5 units/s | 2.5 units/s |
| Space | Fire Player One's current shot | — | — |

Azimuth wraps in `[0, 360)`. Elevation clamps to `[5, 85]` degrees. Launch velocity clamps to `[8, 30]` abstract units/s. Opposite simultaneous adjustment inputs cancel for that value.

## Flight lock

From launch until terrain impact or established non-impact termination, all aim and fire inputs are ignored. The HUD clearly indicates this locked state. Once flight ends, prior values remain available and controls resume; a lingering presentation-only boom does not keep the lock active.

## Feedback contract

The corner display identifies Player One and always shows azimuth and elevation in degrees, launch velocity in abstract units/s, the controls above, and flight-lock status. The red tank's turret yaws with azimuth; its barrel and muzzle marker pitch with elevation. A projectile starts at that visible muzzle and uses exactly the displayed values.
