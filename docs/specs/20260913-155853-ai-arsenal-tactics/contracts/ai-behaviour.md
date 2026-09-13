# Internal Contract: AI Behaviour Boundaries

The game has no external service API. This records its internal boundaries.

## Setup → match/session

- Every configured participant has a valid difficulty, defaulting to Normal.
- Only an AI slot consumes its difficulty; it persists through the existing configuration/session copies.

## AI policy → authoritative gameplay

- Tactical policy is pure and may inspect public battlefield state, actor loadout, difficulty, and isolated tactical seed.
- The runtime selects it through the existing loadout then fires through the shared firing path. Policy never consumes ammunition, launches, advances turns, or changes cash.

## AI policy → shop transaction

- Shopping policy returns a bounded ordered list of limited catalogue weapons.
- The runtime submits each candidate to existing `GameSession::purchase`, which remains authoritative for phase, player, affordability, price, and ammunition.
- The runtime completes an AI shopper once and never skips or alters a Human shopper.

## Fairness and reproducibility

- Difficulty improves judgement and bounded error only—not information, physics, resources, or actions.
- Identical configuration, seed, public world state, and inventory yield identical tactical proposals and shopping plans.
