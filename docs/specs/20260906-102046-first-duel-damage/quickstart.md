# Quickstart: First Duel — Damage, Elimination and Victory

## Prerequisites

From the repository root with the stable Rust toolchain:

```sh
cargo check --workspace --all-targets
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo run --package azimuth-game
```

## Manual validation

Use the [first-duel contract](contracts/first-duel.md) and [data model](data-model.md) as the
expected behavior.

1. Start the application. Verify each player has 100/100 health and the existing first choosing
   turn, controls, active-player camera, and movement option remain available.
2. Fire a shot outside both tanks' 6-unit radius. Verify no health changes, normal terrain/flight
   behavior, and normal handoff.
3. Fire near a tank, then closer on a later turn. Verify the closer impact causes greater visible
   health loss, terrain still deforms, and the shooter can take damage from a short shot.
4. Place both tanks near a blast where practical. Verify both health values update from one impact.
5. Continue accurate impacts until one tank reaches zero. Verify that tank becomes clearly inert,
   its player receives no turn, and the survivor result is obvious without waiting for camera/boom.
6. Validate a mutual-elimination scenario through focused automated tests; if manually achieved,
   verify Draw and no controls afterward.
7. Attempt aim, M, movement, and Space after a finished result. Verify no new ordinary action or
   projectile begins. Restarting the application begins a fresh 100/100 duel.
