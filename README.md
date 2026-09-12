# Azimuth

Azimuth is a turn-based 3D artillery game. In the spirit of classic artillery games, players
choose a weapon, aim with azimuth, elevation, and power, fire, observe the result, and adjust for
the next shot. Terrain and environmental conditions will make those choices interesting.

The game values fun and learnable, internally consistent behaviour over real-world simulation
fidelity. Ridiculous weapons and strange battlefields are welcome when they make the game better.

## Status

Azimuth is a playable local, multi-round artillery game with configurable human/AI participants,
deformable terrain, random round starts and terrain, persistent match cash, and a between-round
weapon shop. Players take deterministic turns to move or fire, while wind, terrain impacts,
craters, settling tanks, and presentation effects make each battlefield distinct. The game ships
with spatial fire, impact, and turret-adjustment audio.

The arsenal now extends beyond the original Basic Shell, High Explosive, and Heavy Shell to include
weapons such as MIRV, Cluster Bomb, Roller, Bunker Buster, Dirt Bomb, Curve Ball, Bouncer, and
Nuke. Bunker Buster is a terrain-only deep penetrator: it reshapes the battlefield without harming
tanks. Round income is actual opponent health removed at $10 per point plus the $5,000/$2,500/$1,000
podium award. Weapon selection and ammunition are independent for each player; a fired shot captures
its weapon profile before flight, so later HUD/selection changes cannot alter it.

## Prerequisites

Use the current stable Rust toolchain. `rust-toolchain.toml` records this policy and requests the
standard `rustfmt` and Clippy components. Nightly Rust and unstable features are not required.

## Build, Run, and Validate

Run commands from the repository root:

```sh
# Fast workspace compilation check
cargo check --workspace --all-targets

# Full workspace build
cargo build --workspace

# Run the minimal 3D battlefield
cargo run --package azimuth-game

# Complete workspace test suite
cargo test --workspace

# Verify formatting
cargo fmt --all -- --check

# Lint every workspace target and fail on warnings
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## Development Workflow

Create and switch to a dedicated feature branch before writing a feature specification. Keep the
specification, planning, implementation, and feature-specific documentation on that branch until
the feature is integrated. Keep scope bounded, and update fulfilled roadmap items only after their
acceptance criteria are met.

The long-lived backlog is [docs/roadmap.md](docs/roadmap.md). Feature specifications live beneath
[docs/specs/](docs/specs/) in directories named `YYYYMMDD-HHMMSS-feature-name` using the local
development time. Read the active specification and its plan before implementing work.

Use readable standard Rust unit tests by default. When simulation work arrives, prefer
deterministic, reproducible inputs and explicitly control randomness. Add readable regression tests
for meaningful gameplay bugs where practical; introduce integration-test support only when a real
external interface warrants it.

## Battlefield Controls

The camera presents the current turn and fired shots without owning gameplay timing:

- Hold the right mouse button and drag to orbit the battlefield.
- Scroll the mouse wheel to move closer to or farther from it.
- Player One (red) begins, then turns alternate with Player Two (blue). The corner display names
  the authoritative current player and whether they are choosing, moving, or resolving a shot.
  At each choosing turn, the camera smoothly presents that player's tank behind their barrel's
  horizontal aim direction. A Human-fired shell receives an offset broadcast follow view that
  widens after its observed apex; an AI-fired shell uses a wider tactical view. Both use a shared
  impact view so crater formation and nearby tank consequences remain visible briefly before the
  next normal turn view or final result. These are presentation-only observations: they never
  delay projectile simulation, impact, terrain deformation, settling, damage, elimination, or
  winner/draw resolution. A short aftermath hold prevents a subsequent player action or automatic
  AI launch from immediately replacing the visible consequence.
  While choosing, Left/Right decrease/increase azimuth, Up/Down decrease/increase elevation, and
  `-`/`=` decrease/increase power. A press changes one fine increment immediately; holding a key
  begins repeat after 300 ms and repeats every 100 ms. Hold Shift for coarse changes. Azimuth
  wraps through 0–359 degrees; elevation is limited to 5–85 degrees; power is launch velocity
  limited to 8–30 abstract units per second. Each player retains their own settings across the
  other player's turn. WASD and the arrows do not pan the camera.
- On a choosing turn, press `1` for Basic Shell, `2` for High Explosive, or `3` for Heavy Shell.
  Selecting does not spend ammunition; Space commits the selected available weapon and spends one
  finite round only when the shot begins. After a player's final HE or Heavy Shell round, selection
  safely returns to Basic Shell.
- The graphical tactical HUD shows a compact, isolated 3D wind-arrow overlay at its top. It is
  oriented from the active player's turret perspective, includes visible depth for wind travelling
  into or out of the screen, and displays the intensity beneath it in KPH.
- Press M to choose movement. The arrow keys request one cardinal step relative to the current
  camera view: Up moves into the view, Down moves out, and Left/Right move across it. At diagonal
  camera angles the nearest cardinal world step is selected. A movement action starts with six
  steps; a valid step must stay within the
  battlefield and change terrain height by no more than 0.75 units. The display shows remaining
  steps and a concise reason when terrain is too steep or a boundary blocks a request. Press Enter
  to end movement early; using the final step also hands control to the other player. Movement
  changes only body facing, while the turret continues to use retained aim.
- Press Space to choose firing and launch the current player's aim from that tank's visible
  barrel-end marker. Moving and firing are mutually exclusive for a turn. A projectile remains the
  only active shot while it resolves; action, aiming, and movement input are locked.
  Terrain impact applies its crater and settles every affected living tank before the other player
  becomes ready. A shot that leaves the useful simulation volume or expires without an impact still
  hands off normally without an invented marker, boom, or crater. The temporary boom is
  presentation-only and may overlap the next choosing turn. A tank that falls into a crater keeps
  its aim values but fires later from its settled barrel origin; settling causes no fall damage.

## Tactical HUD

The HUD is a compact, read-only corner frame: the upper left shows the active player, current
action, and both players' colour-coded health bars; the upper right shows exact azimuth,
elevation, power, selected weapon/ammunition, wind strength, and the world-axis wind plot. The
lower left only shows movement allowance and the controls relevant to the current phase. It never
delays a projectile, settling, turn handoff, or camera transition.

World-space and shot-angle conventions are documented in
[docs/world-conventions.md](docs/world-conventions.md). The fixed-step ballistic model and its
development defaults are documented in [docs/projectile-model.md](docs/projectile-model.md).

## Automation

Every push to `master` runs the same repository-health policy documented above: formatting,
warning-denied Clippy, tests, workspace checking, and workspace build validation. After those
checks pass, the `CI / Build` workflow produces downloadable native artifacts for Windows x86_64,
Linux x86_64, and macOS arm64. Find them at the bottom of the completed run in the repository's
**Actions** tab.

Each artifact contains `azimuth-game` (or `azimuth-game.exe`), its required `assets/` directory,
and a small `BUILD.txt`. Extract it and launch from the package directory; Rust, Cargo, and a
source checkout are not required. Linux users may need `chmod +x azimuth-game` after extraction
and need ordinary local graphics/audio driver support. The macOS build is an unsigned Apple-silicon
binary, so normal Gatekeeper behavior can apply; signing and notarization are not included.

Pushing a semantic version tag such as `v0.2.0` runs the same native builds and then creates a
GitHub Release with the Windows ZIP and Linux/macOS tarballs attached. Normal `master` pushes do
not create releases; their packages remain available from the workflow run.

## Workspace Layout

`crates/azimuth-game` is the initial game application member. The repository root holds workspace
and project metadata. Future sibling crates, such as `azimuth-math` or a physics crate, will be
introduced only when a concrete dependency, testability, reuse, or ownership boundary justifies
them.

See [the rendering technology decision](docs/adr/0001-initial-rendering-technology.md) for the Bevy rationale and
its trade-offs.
