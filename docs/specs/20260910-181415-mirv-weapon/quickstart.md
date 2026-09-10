# MIRV Validation Quickstart

Read [data-model.md](data-model.md) and [the authority/presentation contract](contracts/mirv-shot-presentation.md), then run:

```sh
cargo test -p azimuth-game
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p azimuth-game
```

Automated coverage must prove normal carrier gravity/wind, deterministic five-child apex split, independent child wind/collision, ordered crater-after-impact terrain, one-round consumption/exhaustion, no early handoff, and Basic/HE/Heavy regression.

In Human play select `4`, aim high, and confirm carrier → five-child 3D spread → distinct craters → final-child handoff. Compare weak and crosswind shots; test valley/ridge/slope/nearby tanks. Confirm Human view widens at split, AI stays tactical-wide, impacts use an aggregate region, pulses do not strobe, and audio does not become a wall. With presentation disabled/unavailable, deterministic impact, damage, terrain, settling, survivors, and turn result must match.
