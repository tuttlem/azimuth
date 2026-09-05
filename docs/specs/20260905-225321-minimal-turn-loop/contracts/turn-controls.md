# Turn Controls Contract: Minimal Alternating Fire

This is the external keyboard and feedback contract for the running desktop game. It describes one
local input surface; it is not a configurable binding, networking protocol, or per-device player
assignment.

## Turn ownership

Player One begins. The game alternates Player One then Player Two indefinitely. The displayed
current player is the only player controlled by the keyboard. That player's tank supplies the
visible barrel/muzzle and the origin of a fired projectile.

## Ready controls

| Input | Result | Fine change | Shift coarse change |
|-------|--------|-------------|---------------------|
| Q / E | Decrease / increase current player's azimuth | 1 degree | 5 degrees |
| R / F | Increase / decrease current player's elevation | 1 degree | 5 degrees |
| T / G | Increase / decrease current player's launch velocity | 0.5 units/s | 2.5 units/s |
| Space | Fire the current player's configured shot | — | — |

Azimuth wraps in `[0, 360)`. Elevation clamps in `[5, 85]` degrees and launch velocity in
`[8, 30]` abstract units/s. Opposite simultaneous adjustment inputs cancel for that value. Each
player retains their own values across the other player's turn.

## Resolving lock and advancement

From Space until existing projectile termination, all aim and fire input is ignored: no current
aim changes, second projectile, queued action, or early player switch is allowed. A terrain impact
applies its crater before the other player becomes Ready. A non-impact useful-volume/lifetime
termination has no marker, boom, or crater but still switches to the other player once. A
presentation-only boom may remain visible after the transition.

## Feedback contract

The corner display identifies the current player, displays that player's azimuth, elevation, and
power with units, and states either Ready with controls or Resolving/locked. Tank colours may help
recognition, but the display is the explicit feedback. The displayed player and phase are derived
from authoritative turn state, not the other way around.
