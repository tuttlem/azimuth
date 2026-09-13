# Internal Game/UI Contract

This is an internal Azimuth desktop-game contract, not a public/network API.

## Accounting projection

1. `GameSession.players` and `GameSession.earnings` are sole sources for identity, wallet, and earnings.
2. Each row joins by stable player identity and derives total through `RoundEarnings::total()`.
3. Presentation may update text, colour, layout, and visibility only; it cannot alter cash, earnings, phase, inventory, or combat state.
4. Existing Enter flow advances Accounting to Shopping.

## Input contract

| Context | Input | Required effect | Must not do |
|---|---|---|---|
| Match Setup | Tab | Toggle selected slot through established transition | Change different slot, identity, visual, or count |
| Move mode | Arrow | Request one camera-relative cardinal step | Move another player or bypass bounds |
| Move mode | Space or Enter | Finish and request one handoff | Fire in finishing frame |
| Choosing/fire | Existing fire input | Preserve firing behaviour | Receive stale completion input |

## Visual contract

1. Wind presentation may change material/light contrast only; viewport, direction relationship, and simulation are unchanged.
2. Horizon may blend colours/use terrain-compatible material, but cannot change terrain extent, combat bounds, queries, collision, or movement validity.
3. Visual state synchronises from existing resources; no second wind, terrain, bounds, or accounting model is permitted.
