# Wind Display Contract

This contract describes visible local-duel behavior, not a network or serialized API.

| Situation | Required player-visible result |
|---|---|
| New default match | HUD states `Wind: toward +X, 1.5 units/s²`. |
| Choosing, moving, or resolving-fire turn | The same current-match wind line remains visible with normal player/action/aim feedback. |
| Direction wording | `toward` always means the direction the projectile is pushed, never the origin and never a camera-relative screen direction. |
| Zero-wind domain scenario | HUD/value can truthfully state zero strength; projectiles follow the familiar ballistic path. |
| Firing | HUD does not adjust aim or show a corrected solution; the projectile visibly receives the stated drift. |
| Turn handoff | Wind remains unchanged and is shown to the next player. |
| Tank movement/settling/explosion/camera | No direct wind effect or new wind control appears. |

For the initial default, +X is the documented horizontal world axis associated with 90° azimuth.
Future non-cardinal conditions, if added, must retain the same authoritative “toward” convention
rather than inventing screen-relative terminology.
