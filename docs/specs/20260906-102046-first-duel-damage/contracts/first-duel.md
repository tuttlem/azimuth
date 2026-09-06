# First Duel Player Contract

This contract describes the visible local-duel behavior, not a network or serialized interface.

| Situation | Required player-visible result |
|---|---|
| New duel | Player One and Player Two each show 100/100 health. |
| Terrain impact farther than 6 units from a tank | That tank loses no health. |
| Terrain impact strictly inside 6 units | That tank loses whole linear-falloff splash damage; closer is never less damaging. |
| Explosion near firing tank | The firing tank takes the same splash rule. |
| Health reaches zero | Tank is clearly hidden/inert; player is eliminated. |
| One survivor after blast | Match immediately reports that player wins; ordinary controls stop. |
| No survivors after one blast | Match immediately reports Draw; ordinary controls stop. |

The HUD always reports both players' current/maximum health and elimination status. Existing
move-or-fire controls are available only while the match is in progress and the current player is
alive. Camera and boom presentation cannot delay health changes, handoff, or result.
