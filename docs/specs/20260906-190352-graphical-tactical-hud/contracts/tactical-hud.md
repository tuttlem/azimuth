# Tactical HUD Presentation Contract

| Group | Required content | Visibility |
|---|---|---|
| Player/match | active player, action state, two health bars and exact values | always; result replaces ordinary state at match end |
| Shot/environment | azimuth, elevation, power, numeric wind strength, X/Z wind plot | in-progress turns |
| Movement | remaining allowance and compact rejection | moving only |
| Controls | concise useful keyboard hints | choosing or moving only |
| Result | winner or draw | finished only |

## Invariants

- HUD inputs are read-only and no HUD system alters gameplay.
- Winner/draw suppresses ordinary action hints.
- Health fill is current health divided by the shared maximum.
- Movement allowance is never presented as active outside movement.
- Wind plot uses ASCII `+X`/`+Z` labels and geometry, not font-dependent arrow glyphs.
