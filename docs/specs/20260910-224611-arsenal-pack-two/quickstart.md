# Arsenal Pack #2 Validation Quickstart

## Automated checks

Run:

```text
cargo fmt --all --check
cargo test -p azimuth-game
cargo clippy -p azimuth-game -- -D warnings
```

Verify deterministic fixtures for mound deformation, opposite Curve Ball paths, terrain-normal
Bouncer reflection/bounded completion, Nuke damage/deformation, inventory, and all existing weapons.

## Manual match review

1. Use the bottom strip only: all twelve available weapons appear and depleted ones disappear.
   During a Curve Ball flight, hold Left/Right to apply its gentle directional bend.
2. Fire Dirt Bomb at a crater, slope, and tank-adjacent location. Confirm visible terrain growth,
   coherent tank support, and low damage.
3. Curve shots both ways around a ridge under calm and crosswind conditions. Confirm no target
   steering, a learnable direction, and that releasing the arrow stops additional bend.
4. Bank Bouncer off a slope and valley face. Confirm readable non-explosive contacts, continued
   wind/gravity flight, energy loss, and only one final blast.
5. Fire one Nuke in a populated region. Confirm a comfortable single large event, wide aftermath
   framing, major persistent crater, self-damage, and stable winner/draw resolution.

Use [data-model.md](data-model.md) and the [gameplay/UI contract](contracts/arsenal-pack-two-contract.md)
as the expected authority boundaries.
