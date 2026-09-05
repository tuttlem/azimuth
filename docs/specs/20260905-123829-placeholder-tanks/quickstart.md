# Quickstart: Verify Placeholder Tanks and Player Entities

## Prerequisites

- A supported desktop environment with graphics support for Azimuth.
- The stable Rust toolchain requested by `rust-toolchain.toml`.

Run commands from the repository root.

## Automated validation

```sh
cargo check --workspace --all-targets
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

All commands must succeed without warnings promoted to errors. The test suite must cover distinct
in-bounds fixed spawns, terrain-resolved placement, and firing origins that are above and ahead of
their turret direction.

## Manual validation

1. Start the application with `cargo run --package azimuth-game`.
2. Confirm the existing non-flat battlefield and origin axes are visible.
3. Confirm exactly two placeholder tanks are visible at separate locations within the battlefield.
4. Confirm the players are immediately distinguishable by their simple visual treatment.
5. Orbit, pan, and zoom using the documented controls. Confirm both tanks sit on terrain instead
   of floating or being substantially buried.
6. Inspect from multiple directions. Confirm each body and barrel direction is understandable and
   that a firing-origin reference is visible if the implementation uses one.
7. Confirm the camera does not automatically focus, track, or move toward either tank.
8. Close the window normally and confirm the application exits cleanly.

## Scope check

Confirm the scene has no projectile, aiming input, firing behaviour, movement, damage, health,
turn controls, player-selection UI, or additional gameplay camera behaviour.
