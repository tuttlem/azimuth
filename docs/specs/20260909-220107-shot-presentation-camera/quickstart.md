# Quickstart: Validate Shot Presentation Camera

## Prerequisites

- Use the repository current stable Rust toolchain.
- Work on branch 20260909-220107-shot-presentation-camera.
- Read [data-model.md](data-model.md) and [shot-presentation-camera.md](contracts/shot-presentation-camera.md).

## Automated validation

Run from repository root:

    cargo test -p azimuth-game
    cargo check --workspace --all-targets
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings

Expected: every command succeeds without warnings. Focused tests cover controller selection, Human apex widening, shared impact, final result, display-name independence, safe fallback, bounds, and unchanged authoritative results.

## Run the game

    cargo run --package azimuth-game

Create matches in the existing setup UI. Existing tactical HUD and controls remain useful throughout presentation.

## Manual validation matrix

| Scenario | Action | Expected result |
|---|---|---|
| Human learning | Fire short, medium, long, and high-arc shots across a ridge with wind | Comfortable offset tracking; readable clearance, apex widening, drift, descent, crater, and short/long result |
| AI awareness | Let AI fire near a Human tank | Broad shell/battlefield context, not close Human tracking |
| Consecutive AI | Run Human → AI → AI → AI → Human | Efficient transitions, clear impacts, clean return |
| All AI | Run several all-AI shots | Useful tactical spectator view; no Human-tank assumption |
| Impact variants | Observe miss, direct/near hit, splash, self-hit, elimination, settling, final impact | Common impact aftermath; no next-player view after final result |
| Terrain safety | Fire over mountains/ridges/valleys/water/distant bounds | No recurring terrain clipping, underground pose, or stage-edge exposure |

Complete several Human/AI matches before selecting another feature. Do not mark roadmap camera items complete until automated checks and this manual review pass.
