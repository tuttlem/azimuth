# Data Model: Randomized Round Starts

| Entity | Fields / contents | Lifetime | Validation and relationship |
|---|---|---|---|
| Session Root Seed | Captured seed for one local game | Session | Chosen once for a new session; combines with the round number to reproduce a round. |
| Round Seed | Root-derived seed for one one-based round number | Round | Sole input to labelled terrain, starts, dressing, wind, and AI streams. |
| Generated Round World | Terrain, tanks, visual dressing, wind, selected candidate seed | Round | Created only when every configured player has a valid start; replaced wholesale at each round start. |
| Player Start | Player ID, terrain-resolved position, facing | Round | Must be in bounds, dry/gentle under current rules, terrain-supported, and at least the minimum separation from other starts. |
| Combat State | Tanks/health, turn/aim, selected weapon, flight, impacts, effects, temporary UI | Round | Reset before the first turn; cannot carry from a completed round. |
| Game Session | Identity/controller/visual identity, cash, wins, ammunition, round number | Session | Retained during transition; ammunition availability is retained but current combat selection is reset. |
| Round Accounting | Per-player earnings for the just-finished round | Round | Cleared when the next round begins; applied cash and wins remain. |

## State transitions

```text
new session → choose root + Round 1 → derive candidates → valid world → reset/rebuild → Playing
Shopping → next round number → derive candidates from same root → valid world
         → retain session values; clear round values → Playing
```

## Invariants

- No system outside round generation advances the terrain/start seed stream.
- A generated world is not published until all requested player starts pass validity and separation checks.
- Rendering mirrors committed round state and never determines gameplay validity.
- Root seed plus round number and participants is sufficient to reproduce terrain and starts.
