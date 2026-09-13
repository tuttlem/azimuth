# Quickstart Validation: Round, Controls and Battlefield Polish

Run from repository root with stable Rust and desktop graphics. See [data-model.md](data-model.md) and [game-ui.md](contracts/game-ui.md).

## Automated validation

```sh
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Expected: all pass. Focused coverage includes price table, no-budget movement/handoff, steep-terrain acceptance/bounds rejection, Space non-firing completion, Tab mapping, accounting rows, wind configuration, and horizon seam/invariants.

**Automated evidence (2026-09-13)**: `cargo fmt --all -- --check`, `cargo test --workspace` (146 tests), `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo check --workspace --all-targets`, and `cargo build --workspace` passed.

## Manual validation

```sh
cargo run --package azimuth-game
```

1. Complete 2-, 6-, 7-, and 8-player rounds. At 1280×720 and wider, Accounting shows every player’s name, damage, placement, total, and wallet without scroll; Enter opens Armoury.
2. Verify High Explosive through Bouncer prices are tenfold reduced, Basic Shell unlimited/unpurchasable, Nuke $15,000; verify affordable/unaffordable purchase atomicity.
3. Move more than 20 in-bounds steps over flat, steep, cratered terrain. No allowance/steep rejection; positions follow ground; bounds rejects. Enter and Space each finish once; Space does not fire.
4. In setup use Up/Down then Tab on several slots. Only selected controller changes; identity/colour/name rules remain; guidance names Tab.
5. Inspect wind directions against contrasting terrain/sky and all terrain edges/corners. Arrow is legible and square surface blends into scenery while movement boundary remains unchanged.

## Completion evidence

Record commands/manual results. Review `docs/roadmap.md`, checking only demonstrated criteria and recording broader discoveries as future work.

**Manual evidence**: Pending interactive desktop acceptance for the accounting layout, purchase display, movement controls, setup Tab control, wind brightness, and terrain-edge blend.
