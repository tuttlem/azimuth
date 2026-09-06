# Quickstart: Basic Wind — First Environmental Gameplay

## Prerequisites

From the repository root with the stable Rust toolchain:

```sh
cargo check --workspace --all-targets
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo run --package azimuth-game
```

Use the [wind display contract](contracts/wind-display.md) and [data model](data-model.md) as the
expected behavior.

## Automated validation

1. Run focused projectile tests for finite/horizontal validation, zero-wind equivalence, reversed
  and stronger horizontal vectors, crosswind/parallel behavior, no Y wind contribution, longer
  duration accumulation, deterministic repeated traces, and wind-altered terrain collision.
2. Run integration tests that pass the current match wind through the fixed projectile resolver and
   verify the resulting one impact still drives normal damage, crater, living-tank settling, and
   exactly-once handoff.
3. Confirm tank support/settling tests remain gravity-only and that no camera/HUD pixel or
   render-frame timing test is introduced.

## Manual validation

1. Start a normal duel. Before acting, verify the HUD clearly says `Wind: toward +X, 1.5 units/s²`
   in choosing, movement, and resolving-fire states.
2. Fire a clear crosswind shot and observe visible +X drift. On the next firing turn, aim against
   +X deliberately; verify the landing result moves back toward the intended line without any
   automatic correction.
3. Compare a high/long shot and a short/low shot under the same wind. Verify the longer flight
   visibly accumulates more drift.
4. Continue a normal duel through terrain impact, damage, crater deformation, any tank settling,
   and a winner/draw. Verify wind changes only projectile impact position; movement, settling,
   explosions, and presentation timing retain their documented behavior.
