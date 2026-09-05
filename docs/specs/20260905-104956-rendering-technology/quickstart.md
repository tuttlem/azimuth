# Quickstart: Validate the Rendering Technology Proof

## Prerequisites

- Use the checked-in stable Rust toolchain.
- Use a supported desktop environment with graphics drivers and system libraries required by Bevy.
  Linux developers should consult the linked platform notes in the technology decision record.

## Build and Quality Validation

Run from the repository root:

```sh
cargo check --workspace --all-targets
cargo build --workspace
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

All commands must complete successfully. This feature adds no specialised graphics-test harness.

## Manual Rendering Validation

```sh
cargo run --package azimuth-game
```

Expected result:

1. A desktop window opens and remains running.
2. A fixed perspective camera shows a flat visual ground and simple geometry at visibly different
   depths, with enough lighting to distinguish the forms.
3. No external game asset is required.
4. Closing the window through normal operating-system behaviour exits the application cleanly.

## Documentation Review

1. Read [the technology decision](../../adr/0001-initial-rendering-technology.md).
2. Confirm it compares Bevy with `wgpu` plus `winit`, records trade-offs, and limits the decision.
3. Confirm the README links to that decision and does not claim this proof implements gameplay,
   terrain, physics, permanent coordinate rules, or a controllable camera.
