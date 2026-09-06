# Quickstart: Tactical Controls and Camera Flow

## Prerequisites

From the repository root with the stable Rust toolchain:

```sh
cargo check --workspace --all-targets
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo run --package azimuth-game
```

## Manual validation

Use the [control contract](contracts/tactical-controls.md) and [data model](data-model.md) for
expected behavior.

1. During Player One's choosing turn, tap Left, Right, Up, Down, `-`, and `=`. Confirm the
   matching HUD axis changes and the camera does not pan.
2. Hold each aim/power key. Confirm an immediate fine change, a short pause, then deliberate
   repeated changes. Release and verify changes stop. Repeat with Shift for coarse changes.
3. Hold both keys on an axis: confirm no adjustment. Hold values at elevation/power limits and
   confirm bounds. Hand off and confirm Player Two's retained values stay independent.
4. Verify right-mouse orbit and wheel zoom remain useful; verify WASD cannot pan.
5. Start/complete a movement turn and confirm movement inputs are unchanged and the active tank is
   identifiable.
6. On each player's turn, observe a smooth bounded transition near/behind the active barrel. Change
   azimuth and verify the camera follows its horizontal direction smoothly. Select M or Space
   before a transition ends and confirm input acts immediately.
7. Fire during a player transition. Confirm projectile launch/fixed resolution begin immediately
   while camera pulls back to a readable battlefield view.
8. Confirm crater/handoff behavior remains normal. When the new player begins choosing, the camera
   targets them even if the cosmetic explosion still exists.
9. Repeat alternating move/fire turns; no presentation transition may block play or resolution.
