# Azimuth

Azimuth is a turn-based 3D artillery game. In the spirit of classic artillery games, players
choose a weapon, aim with azimuth, elevation, and power, fire, observe the result, and adjust for
the next shot. Terrain and environmental conditions will make those choices interesting.

The game values fun and learnable, internally consistent behaviour over real-world simulation
fidelity. Ridiculous weapons and strange battlefields are welcome when they make the game better.

## Status

Minimal alternating artillery loop. Azimuth renders two distinct, terrain-grounded tank
placeholders on a bounded non-flat battlefield. Player One and Player Two take deterministic turns:
the current player adjusts azimuth, elevation, and power, fires one projectile from that tank's
barrel, observes its marker/boom/crater result, then hands control to the other player after the
shot resolves. Each player retains their own aiming values for bracketing later shots. The boom
remains presentation only: crater radius and depth are separate gameplay values. Azimuth still has
no damage, tank settling, movement, victory, audio content, or networking.

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

The current camera is a development inspection tool, not the final gameplay camera:

- Hold the right mouse button and drag to orbit the battlefield.
- Scroll the mouse wheel to move closer to or farther from it.
- Use WASD or arrow keys to pan across the battlefield.
- Player One (red) begins, then turns alternate with Player Two (blue). The corner display names
  the authoritative current player and whether their shot is ready or resolving. Use Q/E to
  decrease/increase that player's azimuth, R/F to increase/decrease elevation, and T/G to
  increase/decrease power. Hold Shift for coarse changes. Azimuth wraps through 0–359 degrees;
  elevation is limited to 5–85 degrees; power is launch velocity limited to 8–30 abstract units
  per second. Each player retains their own settings across the other player's turn.
- Press Space to fire the current player's aim from that tank's visible barrel-end marker. A
  projectile remains the only active shot while it resolves; aiming and fire input are locked.
  Terrain impact applies its crater before the other player becomes ready. A shot that leaves the
  useful simulation volume or expires without an impact still hands off normally without an
  invented marker, boom, or crater. The temporary boom is presentation-only and may overlap the
  next ready turn; tanks retain their original positions.

World-space and shot-angle conventions are documented in
[docs/world-conventions.md](docs/world-conventions.md). The fixed-step ballistic model and its
development defaults are documented in [docs/projectile-model.md](docs/projectile-model.md).

## Automation

Continuous integration has been evaluated and deliberately deferred. The direct local commands
above provide the current repository-health checks; CI will be reconsidered when hosting or
collaboration needs establish a concrete benefit.

## Workspace Layout

`crates/azimuth-game` is the initial game application member. The repository root holds workspace
and project metadata. Future sibling crates, such as `azimuth-math` or a physics crate, will be
introduced only when a concrete dependency, testability, reuse, or ownership boundary justifies
them.

See [the rendering technology decision](docs/adr/0001-initial-rendering-technology.md) for the Bevy rationale and
its trade-offs.
