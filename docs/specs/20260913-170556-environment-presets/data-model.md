# Data Model: Environment Presets

| Entity | Fields | Rules |
|---|---|---|
| Environment preset | identity, gravity, wind policy, terrain profile, presentation profile | One of six; Earth default; retained for a match. |
| Wind policy | stable match or reroll each turn; strength range | Turnwind changes only before an aimable turn. |
| Terrain profile | standard or Bowl | Must preserve in-bounds, grounded, separated starts. |
| Presentation profile | terrain/horizon palette, sky, cloud/star treatment | Render-only; must preserve readability. |
