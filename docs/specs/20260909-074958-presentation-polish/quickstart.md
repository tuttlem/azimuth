# Quickstart: Validate Presentation Polish

## Prerequisites

- Work from branch `20260909-074958-presentation-polish`.
- Use the repository’s supported Rust toolchain and desktop graphics environment.
- Review [presentation contract](./contracts/presentation.md) and [data model](./data-model.md) for
  the authority boundary before validating behavior.

## Automated Validation

Run the focused game tests while developing, then the repository checks:

```sh
cargo test -p azimuth-game
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace
```

Expected evidence:

- Pure scoreboard tests cover exactly 2, 3, and 8 configured entries; stable configured order and
  names; live health; eliminated rows; and active/inactive state.
- Horizon tests demonstrate valid initialization/edge continuity while existing out-of-bounds
  terrain queries and authoritative bound checks remain unchanged.
- Existing match setup, turn, terrain, projectile, damage, and HUD tests remain green.

## Manual Scoreboard Validation

1. Run `cargo run -p azimuth-game` and start a 2-player match. Confirm exactly two compact rows,
   configured names, health, clear active indication, and no conspicuous empty scoreboard space.
2. Repeat with 4 and 8 human player slots and distinctive names. Confirm every participant appears
   once in selected match order; all eight remain readable and the central battlefield is open.
3. Damage and eliminate several players. Confirm their ordered rows remain visible, read as
   eliminated, and never retain active styling after the turn advances.
4. Observe resolving and finished states. Confirm no misleading active row is shown while player
   condition information remains truthful.

## Manual World Validation

1. In 2-, 4-, and 8-player matches, inspect normal active-player and shot views toward every map
   side and corner. Confirm blue sky, simple clouds, and distant terrain are visible rather than
   empty void or a floating rectangle.
2. Orbit and zoom normally. Confirm the allowed view does not trivially expose terrain underside,
   a sharp horizon cutoff, or outer void; retain normal tactical usability.
3. Inspect the playable-to-horizon join from ordinary camera distances. Confirm compatible colour
   and elevation read as a landscape continuation, with any far haze subtle enough to leave tanks
   and nearby terrain clear.
4. Fire and deform terrain near an authoritative edge, and attempt boundary movement/shots.
   Confirm existing projectile, movement, spawn, collision, and deformation outcomes still obey
   the original playable bounds; exterior terrain never behaves as gameplay terrain.
5. Play representative turns, shots, impacts, and handoffs. Confirm no noticeable presentation
   responsiveness regression.

## Completion Evidence

Record which 2-, 4-, and 8-player runs were inspected and any needed narrow camera adjustment.
Update only roadmap items whose acceptance was actually demonstrated; record unrelated discoveries
in `docs/roadmap.md` rather than expanding this feature.

### 2026-09-09 Implementation Record

- `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo build --workspace`, and `cargo test -p azimuth-game` passed; the focused suite has 110
  tests.
- A desktop two-player startup smoke run opened successfully after the new HUD/scene systems were
  installed and visually confirmed the scoreboard, blue sky, and exterior terrain render.
- Interactive 2-, 4-, and 8-player acceptance, including configured-name changes, eliminations,
  active-row transitions, all-edge camera inspection, and gameplay-boundary exercise, remains for
  a human graphical playthrough.
