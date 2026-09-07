# Quickstart: Match Setup — Named 2–8 Player Matches and Controller Slots

## Automated verification

~~~text
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace
~~~

Cover configuration bounds/validation, identity/name/controller/visual separation, setup-only AI names, blocked AI start, startup for 2/3/4/8 players, independent state, rotation/skipping, multi-target impact/settling, victory/draw, inventory independence, and deterministic repeatability.

## Manual verification

~~~text
cargo run -p azimuth-game
~~~

1. Confirm Match Setup launches with a valid two-human default, then set all counts 2–8 and verify exact visible slots.
2. Edit human names. Switch several slots to AI, confirm distinct playful generated names and the clear Start block, then switch them back to Human.
3. Start two-, three-, and four-human matches; confirm current gameplay, names, turns, camera, independent state, terrain, weapons, and named victory.
4. Start eight humans; confirm eight supported distinct tanks, readable active HUD, wrapping/skipped turns, multi-player blast/settling, and correct winner/draw.
5. Update only roadmap claims shown by manual evidence; retain AI opponents and final battlefield balance/scale.
