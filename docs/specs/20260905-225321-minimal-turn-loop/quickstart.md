# Quickstart: Validate Minimal Turn Loop

## Prerequisites

Run from the repository root with the supported stable Rust toolchain and a desktop environment
capable of the existing Bevy application. Read [turn-controls.md](contracts/turn-controls.md) and
[data-model.md](data-model.md) for the control and state contracts.

## Automated validation

```sh
cargo check --workspace --all-targets
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Expected: every command succeeds. Deterministic tests demonstrate Player One's initial Ready turn,
independent retained aiming values, Ready-to-Resolving fire, input rejection during resolution,
crater-before-advance terrain impact, normal non-impact completion, and repeated Player One/Two
alternation without renderer or frame-timing assertions.

## Manual alternating turn loop

1. Run `cargo run --package azimuth-game`.
2. Confirm both tanks and the non-flat battlefield render. Confirm the HUD says Player One is
   current and Ready, with Player One's aim values and the documented controls.
3. Change one or more Player One values with Q/E, R/F, or T/G. Note the values and verify Player
   One's barrel/muzzle changes. Press Space to fire.
4. While the projectile flies, use aim keys and Space. Confirm the displayed values and current
   player do not change and no second projectile launches.
5. Observe a terrain impact. Confirm its crater is visible before the HUD changes to Player Two
   Ready. A temporary boom may still be visible after the change.
6. Confirm the same controls now change Player Two's displayed values and tank barrel/muzzle.
   Fire Player Two and repeat the resolving-lock observation.
7. After Player Two resolves, confirm Player One is Ready again and has exactly the aim values
   noted in step 3. Change and fire them if desired; then confirm Player Two likewise retains
   their values on the next return.
8. Aim an outward shot that reaches the existing useful-volume/lifetime termination without terrain
   impact. Confirm no invented marker, boom, or crater appears and the other player becomes Ready
   exactly once.
9. Repeat at least ten full player changes. Confirm turns alternate reliably, each projectile comes
   from the displayed current player's muzzle, and later impacts use earlier terrain deformation.

## Documentation and roadmap review

Confirm the README and projectile model describe current-player ownership, the unchanged controls,
the resolving lock, retained independent aims, and terrain/non-impact advancement semantics. After
all validation above is demonstrated, check only the roadmap items identified by the specification:
the minimal turn loop; player order/start/action/full resolution; fire action; projectile and
deformation completion; clear next-player transition; current player/current state feedback; and
applicable local-multiplayer items. Leave surviving-player advancement, end-of-match, movement,
damage, elimination, camera focus, final HUD, and other deferred scope open.
