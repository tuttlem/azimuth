# Shop UI Contract

This internal application contract defines observable UI behavior; it is not a network API.

## Shop visibility and ownership

- Show the Shop only while the session is in its Shop phase.
- Show exactly one active shopper at a time, with name and cash.
- A Human shopper sees product cards and Done. An AI shopper completes its bounded pass without requiring interaction.
- Do not create or expose the next battlefield until all participants are complete.
- While visible, Shop controls own purchase/Done input and battlefield action controls reject input.

## Product card

Each active limited weapon has one card containing:

| Field | Required behavior |
|---|---|
| visual | Shared weapon presentation identity |
| name | Readable full weapon name |
| price | Current authoritative per-round price |
| owned | Active shopper's current ammunition count |
| Buy | Enabled exactly when the item can be bought; otherwise visible and unavailable |

One Buy activation requests one atomic purchase. On success, cash and owned count refresh immediately. On failure, both remain unchanged.

## Battlefield weapon card

Each selectable weapon card contains the same shared visual identity, a compact readable label, and remaining or unlimited ammunition. Selected and unavailable states remain visually distinct and mouse selection remains available only during a human choosing turn.

## Invariants

- Basic Shell never has a purchasable product.
- Bomb Net never has a product or visual card.
- A player can alter only their own session wallet and loadout.
- Done cannot be applied twice to the same Shop Turn.
- A missing visual identity is a development error, not a silent blank card.
