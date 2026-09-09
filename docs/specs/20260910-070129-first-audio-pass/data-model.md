# Data Model: First Audio Pass

## Audio Cue

| Field | Meaning | Validation |
|---|---|---|
| kind | Fire, flight, impact, destruction, ambience | Finite known set |
| position | Optional world origin | Finite when present |
| profile | Existing weapon/impact identity | Read-only; no name/controller dependency |
| style | One-shot, bounded loop, stopped | One flight and ambience loop maximum |

## State Transitions

```text
successful launch → Fire once + Flight active
active projectile → Flight active
terrain impact → Flight stops + Impact once + optional destruction accent
out of bounds → Flight stops
match wind → Ambience gain updates
```

Audio records only consumed presentation cues. It never changes projectile, terrain, tank, weapon, turn, or controller authority.

## Invariants

- Human and AI share physical cue selection.
- Rejected launches produce no fire cue.
- One resolved impact produces one impact cue.
- Missing assets/playback never change authoritative state.
- Presentation variation never uses match RNG.
