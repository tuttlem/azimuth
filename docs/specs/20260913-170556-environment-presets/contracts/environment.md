# Internal Contract: Environment Ownership

- Setup selects exactly one preset and passes it unchanged into round creation.
- Gameplay world creation consumes its gravity, wind, and terrain profile deterministically.
- Turnwind may replace wind only at a turn boundary before player input.
- Presentation consumes only the preset presentation profile; it cannot change terrain authority, combat bounds, projectile collision, or turn state.
