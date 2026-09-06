# Implementation Plan: Basic Wind — First Environmental Gameplay

**Branch**: `20260906-161023-basic-wind` | **Date**: 2026-09-06 | **Spec**:
[spec.md](spec.md)

## Summary

Add one constant, authoritative, validated horizontal wind acceleration to the existing fixed-step
projectile calculation. The default condition pushes toward +X at 1.5 units/s². Combine that
horizontal acceleration with existing gravity before the current position/velocity update, so
drift accumulates naturally with airtime while the existing swept terrain collision consumes the
same altered segment.

Store the match condition as one app resource beside battlefield gravity and display its exact
toward-direction and magnitude in the current HUD during every in-progress turn phase. Wind is
read-only to players, constant for the match, and used only by active projectiles. Tank movement,
support/settling, explosion/deformation, turn completion, and camera presentation remain unchanged.

## Technical Context

**Language/Version**: Rust edition 2024 on the stable toolchain

**Primary Dependencies**: Bevy 0.18.1 for desktop application scheduling, rendering, input, and
HUD

**Storage**: In-memory domain values and Bevy resources; no persistence, networking, or external
service

**Testing**: Rust unit tests via `cargo test --workspace`; manual desktop validation via
`cargo run --package azimuth-game`

**Target Platform**: Desktop Linux development environment; portable Bevy desktop application

**Project Type**: Single Rust-workspace desktop game application

**Performance Goals**: Preserve the existing 120 Hz fixed projectile simulation and deterministic
terrain crossing. Each step adds one small horizontal acceleration vector without a new per-frame
or per-terrain query cost.

**Constraints**: Wind is finite, horizontal, constant per match, deterministic, and projectile
only. No drag, mass, atmosphere, vertical wind, weather, randomization, tank force, camera gate,
environment-preset system, new crate, or dependency.

**Scale/Scope**: One two-player local match, one shared wind condition, one active projectile, and
the existing bounded deformable terrain

## Constitution Check

| Principle | Design response | Status |
|---|---|---|
| I. Fun Over Realism | Constant readable drift and an explicit HUD convention favor learnable compensation over aerodynamic realism. | Pass |
| II/III. Simple, Coherent Systems | One validated vector joins the existing projectile acceleration; no force framework, weather engine, or new module. | Pass |
| V. Deterministic/Testable Simulation | Wind is part of fixed-step domain input; pure traces cover zero/reversal/magnitude/collision determinism. | Pass |
| VI. Presentation Must Not Own the Game | HUD reads the same resource passed to simulation; camera and visuals neither alter nor gate wind. | Pass |
| VII/VIII. Playable Progress and Scope | A visible environmental aiming variable is delivered without drag, presets, changing weather, or aim assistance. | Pass |
| IX/X. Roadmap and Specs | Timestamped feature artifacts and only demonstrated wind/HUD roadmap completion. | Pass |
| XI/XII. Quality and Dependencies | Existing Cargo checks apply; no dependency or workspace change is needed. | Pass |

**Post-design re-check**: Pass. Wind enters only the existing fixed projectile path and is not
consumed by tank, terrain, camera, or presentation systems.

## Project Structure

```text
crates/azimuth-game/src/
├── projectile.rs    # Wind domain value, validation, combined acceleration, trajectory tests
├── world.rs         # Existing engine-independent WorldVector reused by wind
├── main.rs          # Constant match resource, fixed projectile wiring, compact HUD wind line
├── tank.rs          # Existing gravity-only settling; no wind integration
├── battlefield.rs   # Existing terrain query consumed by altered projectile segment
├── combat.rs        # Existing impact consequences; no wind-specific rule
└── turn.rs          # Existing resolution/action gates; no wind-specific rule

README.md
docs/world-conventions.md
docs/projectile-model.md
docs/roadmap.md
docs/specs/20260906-161023-basic-wind/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/wind-display.md
```

**Structure Decision**: Retain the existing single game crate. `projectile.rs` already owns
validated gravity and constant-acceleration integration, making it the natural home for the small
wind value. `main.rs` remains the direct Bevy boundary for the one current-match resource and HUD;
no general environment abstraction is justified.

## Implementation Sequence

1. Add the pure horizontal `Wind` value, validation, accessors, and fixed-step projectile tests
   before app wiring. Extend the sole projectile advance API so no no-wind parallel path exists.
2. Add the one default battlefield-wind resource and pass it only into the fixed projectile system;
   retain gravity-only tank settling and all current impact resolution ordering.
3. Extend the existing single HUD formatter/synchronizer with the documented camera-independent
   toward-direction and numeric strength, including in choosing, movement, and resolving states.
4. Add/adjust integration tests proving the wind-altered terrain impact still drives exactly the
   existing damage, crater, settling, and handoff pipeline once.
5. Update world/ballistic/player documentation and only proven wind/HUD roadmap items; run all
   quality gates and a manual compensated-shot duel.

## Complexity Tracking

No constitution violations or complexity exceptions require justification.
