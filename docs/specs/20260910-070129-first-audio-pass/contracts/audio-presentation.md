# Audio Presentation Contract

| Authoritative observation | Presentation outcome |
|---|---|
| Successful shared weapon launch | One fire cue; activate flight cue |
| Live projectile | Maintain one restrained flight cue |
| Terrain impact with captured profile | Stop flight; one scaled impact cue |
| Newly eliminated tank | Optional restrained destruction accent |
| Match wind strength | Bounded wind ambience level |

## Invariants

- Audio observes authoritative state; it never validates, predicts, or resolves gameplay.
- Muted, failed, delayed, or absent playback cannot delay or change simulation.
- Camera changes listener perspective only; it does not decide whether a cue exists.
- Controller type, display name, and AI target calculations do not select physical cue identity.

## Safe Fallback

If an asset cannot load or playback cannot start, discard that presentation request and continue normally. Tests validate mappings, not audible output.
