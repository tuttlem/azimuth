# Quickstart: Graphical Tactical HUD Validation

From the repository root:

```sh
cargo check --workspace --all-targets
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo run --package azimuth-game
```

1. Confirm the graphical frame replaces the legacy text dump and identifies current player, action,
   both health values, aim values, and wind.
2. Adjust aim; enter movement and make valid/invalid steps; fire and resolve a shot. Confirm values,
   movement allowance, phase-relevant hints, and removal of hints while resolving.
3. Confirm the world wind plot labels `+X` right and `+Z` up, reverses for opposite wind, and is
   neutral when calm.
4. Complete a win and draw case, resize the window normally, and confirm clear result, no action
   hints after completion, readable corner panels, and an unobstructed battlefield centre.

Automated tests should target the state-to-view mapping in [data-model.md](data-model.md), not
pixels, screenshots, font metrics, or camera timing.
