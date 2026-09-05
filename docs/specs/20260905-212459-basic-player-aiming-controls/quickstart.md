# Quickstart: Validate Basic Player Aiming Controls

## Prerequisites

Run from the repository root with the supported stable Rust toolchain and a desktop environment capable of the existing Bevy application. Read [player-controls.md](contracts/player-controls.md) and [data-model.md](data-model.md).

## Automated validation

```sh
cargo check --workspace --all-targets
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Expected: all commands succeed. Unit coverage demonstrates normalization, bounds, fine/coarse changes, derived muzzle alignment, canonical launch conditions, changed direction/velocity behavior, and the flight input lock without renderer assertions.

## Manual aiming loop

1. Run `cargo run --package azimuth-game`.
2. Confirm both existing tanks and the non-flat battlefield are visible. Identify red Player One and the corner aiming display.
3. Confirm the display shows azimuth, elevation, launch velocity, units, and controls.
4. Press Q/E, R/F, and T/G separately. Confirm each displayed value changes only as specified and the red turret/barrel/muzzle marker follows the relevant angle change.
5. Hold Shift while repeating each adjustment. Confirm the coarse change is larger. Drive elevation and velocity to their ends and confirm values stop at 5/85 and 8/30; rotate azimuth across zero and confirm it wraps.
6. Set a useful shot and press Space. Confirm exactly one projectile begins at the visible muzzle, follows the displayed settings, stops at terrain or existing bounds, and preserves marker, boom, and crater feedback.
7. During flight, press adjustment keys and Space. Confirm no value changes and no second projectile appears. When flight ends, confirm prior values remain displayed and editable.
8. Change one setting, fire again, and compare trajectory/impact. Confirm the second terrain impact uses the battlefield deformation from the first.

## Documentation and roadmap review

Confirm README, world conventions, and projectile model document the same controls, ranges, barrel-end firing-origin rule, and flight lock. Check only the aiming and HUD roadmap items listed by FR-013 after all behavior above is demonstrated; leave turn, weapon-selection, movement, damage, health, AI, environmental, and final-HUD work open.
