# Quickstart: Verify First Projectile and Deterministic Ballistic Arc

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

All commands must succeed. The domain test suite must cover documented angles, launch velocity,
gravity, fixed-step repeatability, apex/descent, limits, and terrain-independent flight.

## Manual validation

1. Start the application with `cargo run --package azimuth-game`.
2. Confirm the existing relief battlefield, axes, and two placeholder tanks are visible and that
   the camera still orbits with right-drag, pans with WASD/arrow keys, and zooms with the wheel.
3. Press Space once. Confirm one bright projectile begins at Player One's firing-origin marker and
   follows a visible upward arc, apex, and descent.
4. Press Space again while the shot is visible. Confirm no second projectile appears and the
   current flight continues unchanged.
5. Observe that the projectile can pass through terrain without impact, damage, terrain change, or
   an explosion.
6. Change the documented development gravity value temporarily, rerun the same shot, and confirm
   its arc visibly differs. Restore the default before completing the feature.
7. Confirm the projectile disappears only after crossing its documented flight-volume or duration
   limit. Fire again after it disappears and confirm the same development shot can be observed.
8. Close the window normally and confirm clean application exit.

## Scope check

Confirm no aiming adjustment, power adjustment, player selection, turn state, movement, collision,
impact, explosion, deformation, wind, drag, weapon system, audio, or multiple simultaneous
projectile behaviour has been introduced.
