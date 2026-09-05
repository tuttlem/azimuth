# Quickstart: Verify First Crater — Terrain Deformation

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

All commands must succeed. Terrain-domain tests must prove deformation and later projectile
interaction without renderer output.

## Manual validation

1. Start Azimuth with `cargo run --package azimuth-game`.
2. Confirm battlefield, tanks, axes, camera, impact marker, and boom still appear.
3. Press I for the fixed impact shot. Confirm it arcs, impacts, booms, and leaves a visible
   permanent crater at the marker.
4. Inspect the crater. Confirm it is a readable depression, not a flat decal.
5. Press I again. Confirm the next projectile uses changed ground and leaves additional/overlapping
   deformation without corrupting the visible terrain.
6. Confirm focused development validation shows travel through removed height and impact on the
   new lower surface when descending into the crater.
7. Inspect a crater near a battlefield edge. Confirm safe clipping and continued responsiveness.
8. Confirm tanks remain present even if nearby terrain changes; do not expect them to settle.

## Scope check

Confirm no damage, health, tank displacement, dirt deposition, terrain building, debris, smoke,
sound, camera response, aiming, or turn behaviour was added.

## Validation Limitation

The 2026-09-05 automated environment reached Bevy application startup but could not open an X
display, so the visual checks above require a supported desktop graphics session.
