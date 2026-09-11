# Quickstart: Validate Randomized Round Starts

## Automated validation

Run from the repository root:

1. `cargo test` — expect reproducibility, round variation, bounded retry, valid 2–8 starts, and reset-ownership tests to pass.
2. `cargo fmt --check`
3. `cargo clippy --workspace --all-targets -- -D warnings`

## Local graphical smoke test

1. Run `cargo run -p azimuth-game` and start a 2-player local game.
2. Complete a round, then advance result/accounting/shop to the next round.
3. Verify terrain shape, tank locations, horizon/dressing, and first-turn state are new. No old impact marker, projectile, explosion, smoke, crater, dead tank, aim, or selected limited weapon remains.
4. Verify identities, controllers, colours, cash, wins, and unused/purchased ammunition remain correct.
5. Repeat at least five rounds and include an 8-player setup. Every player must receive a supported, distinct in-bounds start and immediately use normal controls.

See [data-model.md](data-model.md) and [round-generation.md](contracts/round-generation.md) for precise ownership and reset rules.
