# Quickstart: Tank Support, Gravity and Terrain Settling

## Prerequisites

From the repository root with the stable Rust toolchain:

```sh
cargo check --workspace --all-targets
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo run --package azimuth-game
```

Use the [tank support contract](contracts/tank-support.md) and [data model](data-model.md) as the
expected behavior.

## Automated validation

1. Run the unit suite. Confirm focused domain cases cover stable support, terrain lowered beneath
   a base, remote deformation, support tolerance, gravity descent, terrain contact, terrain rise,
   repeated settling, eliminated-tank exclusion, zero gravity, and identical fixed-step traces.
2. Confirm integration cases cover a terrain impact that hands off immediately when no living tank
   falls and a terrain impact that remains resolving until a living tank settles.
3. Confirm a settled tank's later movement and firing calculation uses its final position while its
   stored aim values remain equal to their pre-settling values.

## Manual validation

1. Start a normal duel. Verify the existing current-player camera, controls, move-or-fire choice,
   health, and shot behavior are available.
2. Arrange a terrain impact directly beneath a surviving tank. Verify crater deformation leaves it
   unsupported, it visibly descends vertically, and it comes to rest on the crater surface rather
   than floating or entering terrain.
3. During the descent, try arrow aiming, M, movement keys, and Space. Verify no ordinary action is
   accepted and no player handoff occurs. Once it settles, verify exactly one normal handoff occurs.
4. Fire a crater away from either tank's base. Verify both tanks retain their positions and no
   unrelated settling delay occurs.
5. On the settled tank's later turn, move it and then, on a later firing turn, fire without
   resetting aim. Verify movement queries its settled position and the projectile begins at the
   new barrel origin with the retained values.
6. Confirm a lethal impact retains existing health, elimination, winner/draw, and camera behavior
   without waiting for wreck settling. Verify there is no fall-damage feedback.
7. Validate the zero-gravity edge case through automated tests: an unsupported tank remains
   unresolved/floating and cannot accidentally advance the turn or gain a frame-rate-dependent
   position.
