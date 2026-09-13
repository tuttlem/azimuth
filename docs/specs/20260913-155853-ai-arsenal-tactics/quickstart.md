# Quickstart: Validate AI Arsenal Tactics

## Prerequisites

- Run from the repository root with the workspace Rust toolchain.
- Start the game with `cargo run -p azimuth-game`.
- Refer to [data-model.md](./data-model.md) and [the AI contract](./contracts/ai-behaviour.md).

## Automated validation

```sh
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo build --workspace
```

Automated evidence (2026-09-13): `cargo fmt --all -- --check`, `cargo test --workspace` (149 tests), `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo build --workspace` all passed. Coverage includes difficulty retention, legal/depleted fallbacks, controlled weapon contexts, difficulty ordering, determinism, and transactional shopping. Visual/manual checks remain recorded below until they are run.

## Manual setup and tactical checks

1. Configure 2–8 local slots with at least two AI opponents at different levels using the displayed difficulty command.
2. Toggle a selected AI to Human and back; confirm its previous difficulty remains. Start the match.
3. In repeatable trials, give AI mixed ammunition and observe close, strong-wind, and grouped-target shots.
4. Confirm AI never fires depleted ammunition, sometimes uses appropriate non-Basic weapons, and remains visibly fallible.
5. Repeat an identical seeded setup and compare a representative AI sequence.

Expected: Easy is forgiving, Normal sensible, Hard more consistent; all operate under human-equivalent information and rules.

## Manual shopping checks

1. Complete rounds with AI wallets at zero, below all prices, and sufficient for multiple purchases.
2. Advance through accounting and observe each automatic AI shop turn, then the next Human shop turn.
3. Inspect next-round AI wallet and ammunition.

Expected: unfunded AI buys nothing; funded AI buys affordable limited weapons only; no wallet is negative; no other player changes; and the round transition always completes.

## Acceptance record

- Automated unit suite: passed 2026-09-13 (149 tests).
- Manual 2–8 player, mixed-difficulty, shop, and seeded-repeat matrix: pending interactive playtest.
