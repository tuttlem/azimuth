# Tank Support Player Contract

This contract defines observable local-duel behavior; it is not a network or serialized API.

| Situation | Required player-visible result |
|---|---|
| Living tank remains within 0.05 units of terrain at its base | Tank is stable and sits exactly on the current terrain surface. |
| Terrain under a living base lowers by more than tolerance | Tank visibly falls vertically under the battlefield's gravity; it does not remain suspended. |
| Falling tank reaches current terrain | Tank stops at the surface without penetrating it. |
| Crater changes terrain away from a base | That tank does not jiggle, slide, or relocate. |
| Fire impact leaves a living tank falling | Existing action feedback remains resolving; aim, movement, and fire input do nothing until living tanks settle. |
| All living tanks settle | The existing survivor-aware handoff occurs once, or the existing winner/draw result is shown. |
| Eliminated tank is above changed terrain | No wreck fall or new delay occurs. |
| Settled surviving tank's later turn | It can move and fire normally from its new position with retained azimuth, elevation, and power. |
| Zero gravity removes support | The tank remains visibly unresolved/floating; the resolving turn stays locked pending a future zero-gravity environment policy. |

Explosion boom and camera movement may overlap this sequence but never decide its timing. Settling
does not inflict fall damage, knockback, sliding, body tilt, or a second explosion-damage result.
