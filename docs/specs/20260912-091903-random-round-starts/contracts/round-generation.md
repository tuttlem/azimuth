# Round Generation Contract

| Operation | Preconditions | Result | Required behavior |
|---|---|---|---|
| Start local session | Valid participant configuration | Session root and Round 1 world | Select fresh root, create valid terrain/start world, clear round state, and initialise player resources. |
| Start next round | Shopping complete; session exists | Next round world | Derive reproducible round seed, create valid world, clear former round state, retain session values, then permit first turn. |
| Generate candidate world | Round seed and 2–8 player IDs | Valid world or controlled candidate failure | Derive labelled terrain/start/dressing/wind streams; reject any candidate lacking valid separated starts. |
| Reproduce a round | Same root, round number, participants | Equivalent world | Produce identical terrain and starts regardless of cosmetic, UI, shopping, or AI activity. |

| Must retain | Must reset or replace |
|---|---|
| Player ID, name, controller kind, visual identity, cash, wins, finite ammunition | Terrain/deformation, tank/health/pose, turn/aim, current weapon selection, flight, impacts, effects, feedback, result, accounting, terrain/tank/dressing/horizon/HUD visuals |

- Candidate rejection is internal and deterministic. Combat never begins with missing, overlapping, out-of-bounds, wet, unsupported, or too-close starts.
- Candidate generation is bounded. Exhaustion uses a documented safe fallback or controlled setup failure, never a gameplay panic.
