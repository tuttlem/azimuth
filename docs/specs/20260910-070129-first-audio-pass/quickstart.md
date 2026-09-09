# Quickstart: Validate First Audio Pass

## Prerequisites

- Work on `20260910-070129-first-audio-pass`.
- Use headphones or speakers and read [data-model.md](data-model.md) and the [contract](contracts/audio-presentation.md).

## Automated validation

```sh
cargo test -p azimuth-game
cargo fmt --all -- --check
cargo test --workspace
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Verify focused tests cover successful/rejected launch, shared Human/AI selection, impact scale, flight cleanup, optional destruction, wind bounds, and unchanged authoritative outcomes without playback.

## Run

```sh
cargo run --package azimuth-game
```

| Scenario | Action | Expected outcome |
|---|---|---|
| Human Basic | Fire short, long, high arcs | Weighty launch, restrained flight, satisfying impact |
| High Explosive | Fire near tanks | Larger-feeling boom; no sound wall |
| Heavy Shell | Fire into wind | Optional heavier launch; truthful baseline impact |
| AI/all-AI | Observe consecutive turns | Same physical cues, useful distant activity |
| Distance | Compare near/far/crossing shots | Coherent positional treatment |
| Wind | Compare weak/strong conditions | Subtle bounded ambience |
| No audio | Run presentation-bypass tests | Identical projectile, terrain, damage, turns |

Do not mark Milestone H or optional tank/debris roadmap entries complete until manual review passes.
