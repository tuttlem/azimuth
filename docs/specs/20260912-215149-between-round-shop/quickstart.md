# Quickstart: Validate Between-Round Shop — Weapon Purchasing and Visual Inventory

Run from the repository root:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo check --workspace --all-targets
cargo build --workspace
cargo run -p azimuth-game
```

Use the [Shop UI contract](./contracts/shop-ui.md) and [data model](./data-model.md) as the expected behavior reference.

## Automated checks

Confirm coverage for catalogue derivation, valid and invalid atomic purchase outcomes, player isolation, retained inventory across fresh rounds, configured-order human shop turns, non-blocking AI turns, and shared weapon presentation identity.

## Manual four-player acceptance

1. Configure a mix of Humans and AI, complete Round 1, and acknowledge result/accounting.
2. Confirm the first Human shop shows their name, cash, visual weapon-card grid, readable names, prices, owned counts, and Done; confirm Bomb Net is absent and Basic Shell cannot consume money.
3. Buy several affordable rounds. Confirm cash and owned counts update immediately; confirm an unaffordable Nuke stays visible but unavailable.
4. Choose Done. Confirm the next Human receives their own independent wallet/inventory. Confirm AI turns resolve without manual clicks and do not stall.
5. Confirm normal battle controls do nothing while a shop is shown. After the final participant completes, confirm a fresh Round 2 begins only then.
6. In Round 2, confirm bought ammunition and unused Round 1 ammunition appear in the bottom strip, fired limited ammunition is consumed normally, and each card has an image, compact label, and clear selected/unavailable/unlimited state.
7. Complete Round 2 and verify accounting adds new money, remaining inventory survives, and the Shop appears again.
