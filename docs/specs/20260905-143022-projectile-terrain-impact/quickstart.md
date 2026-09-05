# Quickstart: Verify Projectile Terrain Impact Detection

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

All commands must succeed. Tests must validate the behaviours in [data-model.md](./data-model.md):
swept crossings, high-speed non-tunnelling, flat/sloped accuracy, repeatability, gravity variation,
and non-impact termination.

## Manual validation

1. Start with `cargo run --package azimuth-game`.
2. Confirm the non-flat terrain, axes, and terrain-grounded tanks remain visible, and orbit, pan,
   and zoom still work.
3. Press Space. Confirm one projectile begins at Player One's firing origin and follows the
   established upward arc. It may leave the bounded battlefield before landing, as before.
4. After it ends, press I to fire the fixed development impact shot. On descent, confirm it stops
   at visible terrain rather than passing through it, and a distinct
   small marker remains at the landing point.
5. Change camera angle and inspect another terrain elevation or slope. Confirm the marker sits on
   the surface.
6. Fire I again after impact. Confirm one projectile flies and the marker updates for the current
   impact without changing the new trajectory.
7. Confirm a shot that leaves useful simulation volume without terrain contact disappears without
   an impact marker. Restore any temporary development setting before completion.
8. Repeat the default shot and confirm the observed impact location is unchanged.

## Scope check

Confirm no explosion, particles, sound, damage, tank-hit handling, terrain deformation, aiming,
turn state, movement, wind, drag, mass, ricochet, or non-terrain collision was added.
