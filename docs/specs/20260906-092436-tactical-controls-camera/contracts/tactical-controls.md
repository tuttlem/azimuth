# Tactical Controls Contract

This is the player-visible keyboard and camera contract for the Bevy desktop game, not a network
or serialized API.

| Context | Input | Required result |
|---|---|---|
| Choosing | Left / Right | Decrease / increase active player's azimuth. |
| Choosing | Up / Down | Increase / decrease active player's elevation. |
| Choosing | `-` / `=` | Decrease / increase active player's power. |
| Choosing | Shift + aim/power input | Use the existing coarse adjustment amount. |
| Choosing | M | Begin existing movement action. |
| Choosing | Space | Begin existing firing action. |
| Moving | I / J / K / L | Request existing -Z / -X / +Z / +X tactical step. |
| Moving | Enter | End movement early and hand off turn. |
| Any presentation state | Right mouse drag | Orbit camera only. |
| Any presentation state | Mouse wheel | Zoom camera only. |

During a choosing turn, the active-player camera aligns behind the active barrel's horizontal aim.
Changing azimuth updates that presentation direction smoothly; it never changes aiming, turn, or
simulation state.

An eligible press changes aim once immediately. Continued holding repeats after 300 ms at one
adjustment per 100 ms until release, ambiguity, bounds, or action-state lock. Holding both keys of
one directional pair produces no adjustment.

WASD and arrows must not pan the camera. Camera transitions cannot accept, reject, defer, or
complete a movement, fire, projectile, terrain, or turn action.
