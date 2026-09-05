# Quickstart: Validate Move or Fire — Basic Tactical Movement

Run from the repository root on the feature branch with the stable toolchain in `rust-toolchain.toml`:

```sh
cargo check --workspace --all-targets
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo run --package azimuth-game
```

Refer to [turn-controls.md](contracts/turn-controls.md) for inputs and [data-model.md](data-model.md) for authoritative invariants.

1. Start the game; confirm Player One is choosing with aim, M, and Space guidance.
2. Press M, verify six steps, make a valid I/J/K/L step, and confirm only Player One moves one unit, follows visible terrain, faces that direction, and has five remaining.
3. Mix valid steps, then press Enter before depletion. Confirm Player Two receives choosing state. On another turn, exhaust six steps and confirm the same single handoff.
4. Attempt an edge exit and a steep deformed-terrain step. Confirm a concise rejection while position, facing, and allowance stay unchanged.
5. Press Space on a choosing turn. Confirm movement cannot occur while projectile resolution, impact, and deformation retain their existing ordering.
6. Move Player One, complete Player Two's turn, then return to Player One. Confirm aim values are retained and a fired projectile launches from Player One's new position.
7. Complete ten alternating turns using early-end and exhausted movement plus terrain-impact and non-impact shots. Confirm exactly one primary action per turn and no duplicated or stuck handoff.

The automated unit suite must cover allowance lifecycle, valid/invalid/bound/slope/deformed-terrain movement, orientation, exclusivity, handoff, retained aim, resolution order, per-player ownership, and identical-input determinism without keyboard, HUD-text, or frame-timing assertions.
