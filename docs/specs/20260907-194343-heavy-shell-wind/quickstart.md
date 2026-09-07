# Quickstart: Validate Heavy Shell Wind Resistance

## Prerequisites

- Use the repository's stable Rust toolchain.
- Run commands from the repository root.
- Start from branch `feature/heavy-shell-wind-resistance` with the feature implemented.

## Automated validation

Run focused and repository-wide checks:

```sh
cargo test --workspace
cargo check --workspace --all-targets
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

The tests must prove the [data model](data-model.md) and [conventional-weapon contract](contracts/conventional-weapons.md):

- Catalogue metadata contains distinct Basic Shell, High Explosive, and Heavy Shell definitions.
- Each player begins with two independent Heavy Shell rounds; selection/rejected actions consume
  none; a legal Heavy Shell fire consumes exactly one; exhaustion falls back safely.
- The committed projectile, not later selection or catalogue state, retains wind response.
- Under identical fixed-step crosswind traces, Heavy Shell drift is 40% of normal-shell drift and
  remains non-zero; opposite and aligned winds reverse/scale influence consistently.
- Zero wind yields equivalent normal and Heavy Shell traces; gravity's vertical trajectory is
  unchanged; repeated inputs reproduce identical results.
- Terrain collision, ordinary damage/crater impact, settling, victory, and Basic Shell/High
  Explosive behaviours remain healthy.

## Manual local-play validation

Start the game:

```sh
cargo run --package azimuth-game
```

1. On each player's choosing turn, confirm the normal selector can reach Basic Shell, High
   Explosive, and `HEAVY SHELL x2`, and selection alone changes no ammunition.
2. In calm or very low wind, compare equivalent Basic Shell and Heavy Shell shots. Their ordinary
   arcs should be broadly indistinguishable where they share launch and impact configuration.
3. In noticeable crosswind, fire comparable Basic Shell and Heavy Shell shots. The Heavy Shell
   should remain pushed toward wind but land visibly closer to the intended line.
4. Repeat with reversed and strong valid wind. Direction of drift should reverse; Heavy Shell must
   not become wind-immune.
5. Confirm Heavy Shell decrements to `x1` on a legal shot and returns to Basic Shell after its
   final round. Confirm the other player's count remains unchanged.
6. Verify its terrain impact produces the ordinary blast/crater and that damage, tank settling,
   turn handoff, victory/draw, and a full two-player match remain correct. Verify High Explosive
   still has the visibly larger blast role.
7. Record whether strong wind led to a genuine Heavy Shell choice. Update only roadmap items whose
   acceptance criteria that play evidence satisfies; record any tuning discovery rather than
   expanding the feature.

## Execution Record

- Automated validation completed on 2026-09-07: workspace tests, check, formatting, and Clippy
  passed without warnings.
- Manual local-play validation was completed by the developer after implementation sign-off. Heavy
  Shell's wind resistance, ammunition, ordinary impact, and normal match flow were verified; the
  delivered Heavy Shell roadmap item is checked. Broader Arsenal tactical-choice milestones remain
  for later evidence across the growing weapon set.
