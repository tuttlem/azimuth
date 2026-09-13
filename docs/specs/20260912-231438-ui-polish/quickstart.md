# UI Polish Validation Guide

## Prerequisites

- Work from branch `20260912-231438-ui-polish`.
- Use the repository's Rust toolchain and Bevy-compatible desktop environment.
- Review [data-model.md](data-model.md) and [the UI presentation contract](contracts/ui-presentation.md) before changing UI/domain boundaries.

## Automated checks

Run from the repository root after implementation:

```text
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo check --workspace
cargo build --workspace
```

Focused tests should prove shared `WeaponId` presentation coverage, shop/HUD mapping, Basic Shell unlimited rendering, exhausted ammunition, affordability, and 2–8 player presentation mappings without relying on pixel screenshots.

## Manual graphical checks

Launch the game with `cargo run -p azimuth-game` and inspect at 1280×720, 1920×1080, and one wider desktop window.

1. Start 2-, 4-, and 8-player sessions with both Human and AI controllers. Confirm the battlefield remains visually dominant; active player, health, action, exact aim, wind, selected weapon, ammunition, active/eliminated player status, and mouse targets are readable without clipping or overlap.
2. Inspect every weapon slot. Confirm shared artwork is distinguishable before reading its short label; selected, zero-ammo, and Basic Shell unlimited states are unmistakable; Bomb Net is absent.
3. Complete a round. Confirm the winner remains visible behind a celebratory treatment, accounting makes damage/placement/total/wallet hierarchy obvious, and continuation leads to shop without a raw/debug-looking screen.
4. In shop, confirm active shopper identity/colour, wallet, full catalogue, shared large weapon art, price, owned ammunition, Buy, and Done are clear. Verify an unaffordable Nuke remains visible but cannot be bought.
5. Buy several weapons. Confirm authoritative cash and owned ammunition update immediately, card/button feedback is bounded and clear, and UI sound is restrained. Complete all shoppers (including AI) and verify the next round starts with purchased/unused inventory preserved.
6. Repeat the complete setup → round → winner → accounting → shop → round-two path. Look specifically for inconsistent colours, typography, button states, margins, panels, dead space, or unexpected text-only surfaces.

## Completion evidence

Record the representative manual layouts/session sizes exercised and update only roadmap boxes supported by those results. If an out-of-scope gameplay issue is discovered, capture it in `docs/roadmap.md` rather than expanding this feature.
