# Quickstart: First Arsenal Validation

From the repository root, run:

```sh
cargo check --workspace --all-targets
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo run
```

Use the [data model](data-model.md) and [conventional weapon contract](contracts/conventional-weapons.md)
as the source of expected state and values.

1. On Player One's choosing turn, confirm the tactical HUD shows Basic Shell as `UNLIMITED`.
   Press `2`; confirm it changes to High Explosive with 2 rounds and no aim/turn change. Press `1`
   to return to Basic Shell.
2. Fire Basic Shell and confirm the familiar flight, normal-sized impact, crater, health effect,
   wind drift, settling, and handoff still work.
3. On a later choosing turn select HE with `2`, fire it, and confirm it uses the same flight but a
   noticeably larger explosion/crater consequence. Confirm that only the firing player's HE count
   becomes 1 after commitment.
4. Spend the remaining HE round. On that player's subsequent turn, confirm Basic Shell is selected
   and `2` cannot create or display a deceptive exhausted HE shot. Confirm the other player still
   has their own HE count.
5. Aim either weapon into terrain beneath a tank where safe to observe. Confirm the weapon’s crater
   naturally uses current support and settling rules, while turn advancement waits for settling.
6. Complete a duel to winner or draw. Confirm weapon display and controls do not delay projectile,
   deformation, settlement, elimination, victory, camera presentation, or turn handoff.
