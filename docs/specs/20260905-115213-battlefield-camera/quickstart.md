# Quickstart: Verify the Minimal 3D Battlefield and Camera

## Prerequisites

- A supported desktop environment with graphics support for the existing Azimuth application.
- The stable Rust toolchain requested by `rust-toolchain.toml`.

Run commands from the repository root.

## Automated validation

```sh
cargo check --workspace --all-targets
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

All commands must succeed without warnings promoted to errors.

## Manual validation

1. Start the application with `cargo run --package azimuth-game`.
2. Confirm the window shows bounded, non-flat ground with an obvious high region, lower region, connecting slope, and origin orientation axes.
3. Hold the right mouse button and drag. Confirm the view orbits the battlefield and shows relief from different sides.
4. Scroll the mouse wheel in both directions. Confirm the camera moves closer to and farther from the battlefield without becoming unusable.
5. Use WASD or arrow keys. Confirm the inspection target pans laterally while remaining within the useful battlefield area.
6. Repeat steps 3–5 from at least three viewpoints. Confirm terrain elevation, slope, bounds, and world orientation remain understandable.
7. Close the window using normal desktop window controls. Confirm the process exits cleanly.

## Documentation check

1. Read [the world conventions](../../../world-conventions.md). Confirm it identifies Y-up, origin-centred placement, abstract units, and approximate 40 by 40 unit bounds.
2. Confirm it defers azimuth, elevation-angle, launch-vector, spawn-position, and out-of-bounds rules.
3. Confirm the root README contains the run command and the same camera controls.
