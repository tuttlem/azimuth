# Arsenal Pack #2 Gameplay/UI Contract

## Weapon strip

- The strip is the preferred selection surface and presents every weapon with available ammunition.
- A click selects one available weapon for the current human player; it neither fires nor consumes ammunition.
- A selected Curve Ball presents Left and Right controls. A click changes the current player's curve choice only.
- Fire captures weapon, profile, and Curve Ball direction. Later UI changes cannot change that shot.

## Authoritative shot behaviour

- Dirt Bomb follows normal flight and produces one mound consequence plus normal terrain/support reconciliation.
- Curve Ball follows normal flight plus fixed launch-relative lateral influence; it has no target input.
- Bouncer intermediate contacts retain the shot and produce no damage, crater, or final-impact signal. Its final contact resolves normally.
- Nuke follows normal flight and one large normal impact consequence. It may damage/eliminate the shooter and may produce a winner or draw.

## Determinism boundary

For equal match state, weapon choice, curve choice, aim, environment, and fixed-step count, terrain,
trajectories, impacts, damage, and handoff must be identical. Presentation timing, sound mixing, and
camera framing are non-authoritative.
