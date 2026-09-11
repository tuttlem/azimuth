# Quickstart Validation: Battlefield Visual Polish

## Prerequisites

Run from the repository root on branch `20260911-214544-battlefield-visual-polish`, using the
pinned Rust toolchain and a desktop environment capable of opening the graphical game window.

> **Environment note (2026-09-11)**: automated checks pass in the current headless workspace, but
> graphical acceptance remains pending because its Linux session cannot open an X display. Run the
> graphical sections on a desktop session before claiming the manual roadmap outcomes.

## Automated Validation

1. Run focused and workspace tests:

   ```sh
   cargo test --workspace
   ```

2. Check formatting and linting:

   ```sh
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   ```

3. Confirm coverage described in [data-model.md](./data-model.md) and the independence rules in
   [presentation-boundary.md](./contracts/presentation-boundary.md): 2–8-player assignment,
   canonical muzzle/attachment relations, terrain refresh, capped profile-scaled effects,
   expiry/reset cleanup, and unchanged authoritative outcomes.

## Graphical Match Checks

Launch the game:

```sh
cargo run -p azimuth-game
```

### Tank Variety and Materials

1. Start 2-, 4-, and 8-player matches from normal setup.
2. Verify every tank reads as a tank; with enough players observe at least four silhouettes.
   Confirm duplicate models in eight-player play remain understandable.
3. Rotate turrets, change elevation, move, fire, and let tanks settle. Projectiles must appear at
   sensible muzzle locations and no silhouette may look mechanically advantaged.
4. Inspect player-coloured armour, dark tracks/barrels, grass, dirt, high/rocky areas, and water
   from ordinary camera and shot views. They should be readable and stylised, not photorealistic.
5. Fire Dirt Bomb and an ordinary crater-making weapon. Confirm changed terrain stays visually
   coherent without altering known terrain behaviour.

### Explosion Particles and Smoke

1. Fire Basic Shell and High Explosive at clear terrain. Confirm HE has a larger restrained burst,
   followed by small rising/fading smoke.
2. Fire MIRV, Cluster Bomb, and Bomb Net. Confirm impacts look lively, fade, remain readable, and
   do not fill the battlefield with opaque smoke or harm responsiveness.
3. Fire Nuke. Confirm substantially larger burst and longer-lived but finite plume; it should feel
   exaggerated rather than realistic.
4. Continue playing or fire repeated shots until effects expire. Confirm nothing persists and
   tanks/HUD remain readable. If smoke drift is implemented, confirm it broadly agrees with wind
   without changing projectile behaviour.

## Completion Record

Before marking roadmap items complete, record that observed results satisfy the feature spec. Only
then update demonstrated tank-model, debris, smoke, Visual Style, and Improved visuals entries;
leave unrelated roadmap work open.
