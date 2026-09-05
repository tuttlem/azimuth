# Quickstart: Verify First Boom — Visible Projectile Explosion

## Prerequisites

- Supported desktop graphics environment for Azimuth.
- Stable Rust toolchain requested by `rust-toolchain.toml`.

Run commands from the repository root.

## Automated validation

```sh
cargo check --workspace --all-targets
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

All commands must succeed. Proportionate unit coverage must validate extracted effect lifetime and
scale calculations without depending on renderer output.

## Manual validation

1. Start Azimuth with `cargo run --package azimuth-game`.
2. Confirm battlefield, tanks, axes, and camera controls remain available.
3. Press I for the fixed impact shot. Confirm arc, terrain impact, projectile removal, and marker.
4. Confirm a bright obvious explosion starts at the marker and visibly expands or changes.
5. Wait for expiry. Confirm it disappears cleanly while the marker remains useful.
6. Press I again. Confirm exactly one new boom and no stale earlier effect.
7. Press Space. Confirm the original long shot remains unchanged and creates no boom when it ends
   outside terrain.

## Scope check

Confirm terrain is unchanged and tanks unharmed. Confirm no damage, deformation, sound, camera
response, smoke system, weapon model, or gameplay explosion radius was added.

## Validation Limitation

The 2026-09-05 automated environment reached Bevy application startup but could not open an X
display, so the manual visual checks above still require a supported desktop graphics session.
