# Validation Guide: Expanded Battlefield

## Prerequisites

- Work from branch `20260908-200635-expanded-battlefield`.
- Use the repository’s supported Rust toolchain and the existing local graphics environment.

## Automated Validation

Run the focused game tests while implementing, then repository health checks before completion:

```sh
cargo test -p azimuth-game
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
```

The terrain/start tests use the fixed seed table `3`, `17`, `42`, `99`, and `4242`; record the
printed battlefield seed during any manual session. Validate the entities/invariants in [data-model.md](data-model.md) and [battlefield-generation.md](contracts/battlefield-generation.md):

1. Re-run identical seed/configuration generation and compare sampled heights, starts, dressing records, and wind exactly.
2. Compare different seeds at selected macro and local samples; require meaningful variation without asserting one exact mountain shape.
3. Confirm 120-unit bounds, 65-by-65 vertex maximum, finite heights, at least 24 sampled elevation units, and safe edge/corner queries.
4. For 2, 4, and 8 players, verify exact start count, dry/support/slope/bounds validity, no overlap, and 18-unit minimum horizontal separation.
5. Verify colour mapping is finite, bounded, and ordered from lowland through grass/earth/rock to snow; regenerate mesh data after centre, edge, high, low, and water-table crater cases.
6. Verify every building record is in bounds, dry, grounded, slope-suitable, and outside the start exclusion zone; verify buildings/water have no projectile or tank contract test.
7. Re-run existing projectile, wind, weapon, terrain impact/deformation, settling, movement, turn, victory, inventory, HUD, and multiplayer tests with the generated terrain.

## Manual Seed Inspection

Run the game using a recorded seed mechanism supplied by the implementation. Inspect at least five distinct recorded seeds before approval.

For each seed, confirm:

- The map is visibly much larger than the old arena and contains broad features rather than repeated small bumps.
- Mountains/highlands, valleys/lowlands, ridges or bowls, and some flatter local ground are readable at artillery distance.
- Peaks transition coherently through snow/rock/earth/grass/lowland colour, including after a crater.
- Water visibly occupies low terrain as a flat plane; no water physics is implied.
- Any buildings are sparse, grey, grounded, dry, simple, and clearly not collision/cover objects.
- Active-player and shot camera framing retains aim usability and shows useful geography without obvious clipping.

## Multiplayer Playthroughs

Start and inspect all-human 2-, 4-, and 8-player matches for multiple seeds.

- Confirm every tank is dry, supported, separated, within bounds, and distributed beyond the former development region.
- Fire Basic Shell, High Explosive, and Heavy Shell at both short and long ranges. Confirm wind remains readable and Heavy Shell remains visibly wind-resistant.
- Create craters near the centre, edge, high terrain, low terrain, and water. Confirm impacts, terrain refresh, support/settling, and turn handoff still work.
- Look for at least one artillery decision changed by geography: lob over a ridge, fire into/out of a valley, use high ground, or adjust power/elevation for distant terrain.
- Complete representative matches and verify names, HUD, inventories, eliminations, victory, and normal movement remain correct.

## Completion Record

After successful automated and manual validation, update `docs/world-conventions.md`, `docs/projectile-model.md`, and `docs/roadmap.md` only with observed delivered behaviour. Record the seeds inspected and any confusion caused by presentation-only structures. Do not check fair-start, unwinnable-terrain, authoritative-building, water-gameplay, or AI roadmap work unless it was actually implemented and validated.
