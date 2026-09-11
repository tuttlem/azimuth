# Quickstart Validation

## Prerequisites

From repository root in a graphical desktop session:

cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo run -p azimuth-game

## End-to-end checks

1. Configure Human/AI players, finish Round 1, and verify winner focus/overlay with controls locked.
2. Continue to accounting and compare damage, elimination, win, total, and cash values with resolved play.
3. Buy an affordable limited round, attempt an unaffordable one, complete human READY actions, and verify AI does not wait.
4. Verify Round 2 has fresh terrain/tanks/health with preserved identity, cash, unused ammunition, and purchases.
5. Test a multi-projectile weapon and self-damage; only actual opponent health earns money.
6. END GAME from between-round flow, start a new session, and verify standard starting inventory.

See [data-model.md](data-model.md) and [game-flow contract](contracts/game-flow.md).

