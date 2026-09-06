# Implementation Plan: Tank Support, Gravity and Terrain Settling

**Branch**: `20260906-141906-tank-support-settling` | **Date**: 2026-09-06 | **Spec**:
[spec.md](spec.md)

## Summary

Make the mutable battlefield surface support living tanks after every crater. Add a compact
support state directly to each tank: a base is either supported or falling with vertical velocity.
Use the existing 120 Hz fixed simulation cadence and configurable gravity to descend only at the
tank's current X/Z position, clamp contact to the current terrain height, and preserve horizontal
position, body direction, turret direction, and stored player aim.

Keep the existing `ResolvingFire` phase as the authoritative action lock. A terrain impact will
resolve existing splash damage once, deform terrain, re-evaluate living tank support, and only
complete the fire turn once every living tank is stable. Out-of-bounds shots retain their current
immediate no-impact completion. The visual tank, camera, boom, HUD, movement, and later firing
continue to observe this domain state; none controls settling or turn timing.

## Technical Context

**Language/Version**: Rust edition 2024 on the stable toolchain

**Primary Dependencies**: Bevy 0.18.1 for the desktop application, fixed scheduling, rendering,
input, camera, and HUD

**Storage**: In-memory domain values and Bevy resources; no persistence or external service

**Testing**: Rust unit tests via `cargo test --workspace`; manual desktop validation via
`cargo run --package azimuth-game`

**Target Platform**: Desktop Linux development environment; portable Bevy desktop application

**Project Type**: Single Rust-workspace desktop game application

**Performance Goals**: Preserve 120 Hz deterministic simulation. At the current two-tank scale,
re-evaluate and advance every living tank once per impact/fixed tick without perceptible overhead.

**Constraints**: Terrain is authoritative; settling is deterministic and renderer-independent;
only vertical response is in scope. Existing configurable gravity is reused. No physics engine,
vehicle physics, sliding, fall damage, tank collision, generic terrain-response system, new
dependency, or presentation-controlled resolution is permitted.

**Scale/Scope**: One bounded 20-by-20 terrain grid, two local human players, two domain tanks,
one active projectile, and one shared gravity configuration per application run

## Constitution Check

| Principle | Design response | Status |
|---|---|---|
| I. Fun Over Realism | A readable vertical drop into a crater matters more than physical vehicle fidelity. | Pass |
| II/III. Simple, Coherent Systems | One base-point support query and a two-state tank model use existing terrain and tank ownership; no crater special case or framework. | Pass |
| V. Deterministic/Testable Simulation | Fixed-step, pure tank operations accept terrain/gravity and have focused unit tests independent of rendering. | Pass |
| VI. Presentation Must Not Own the Game | Fixed gameplay resolution owns support, descent, contact, and handoff; visual sync/camera only read the result. | Pass |
| VII/VIII. Playable Progress and Scope | Completes a visible crater consequence without scope expansion into suspension, sliding, fall damage, or wreck physics. | Pass |
| IX/X. Roadmap and Specs | Uses this timestamped feature directory; only the two delivered support/handoff roadmap entries will be checked. | Pass |
| XI/XII. Quality and Dependencies | Existing Cargo quality gates apply; no dependency or workspace change is needed. | Pass |

**Post-design re-check**: Pass. The design keeps all settling state in the existing domain tank
and orders the fixed gameplay systems explicitly, without camera, visual, or new generic physics
dependencies.

## Project Structure

```text
crates/azimuth-game/src/
├── tank.rs          # Tank pose, health, supported/falling state, support and contact operations
├── projectile.rs    # Existing gravity/fixed-step constants; expose only the needed gravity value
├── battlefield.rs   # Existing current authoritative terrain-height query and crater mutation
├── turn.rs          # Existing ResolvingFire action gate and one-shot completion guard
├── main.rs          # Fixed-system ordering, impact-to-settling integration, visual synchronization
├── combat.rs        # Existing one-time splash damage before deformation
└── world.rs         # Existing world positions/vectors

README.md
docs/world-conventions.md
docs/projectile-model.md
docs/roadmap.md
docs/specs/20260906-141906-tank-support-settling/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/tank-support.md
```

**Structure Decision**: Retain the single game crate. `tank.rs` is the natural home for the
small authoritative support model because it already owns pose, grounding on deliberate movement,
health, and firing origin. No new crate or terrain/physics abstraction has a demonstrated need.

## Implementation Sequence

1. Add the compact tank support state and pure terrain reconciliation/fixed-step descent methods,
   including contact and gravity tests, without changing rendering or turns.
2. Expose the existing validated gravity magnitude through the smallest domain API required by
   tank settling; reuse the public fixed step instead of duplicating a gravity value or timestep.
3. Integrate support re-evaluation immediately after impact damage and crater deformation. Chain a
   settling fixed system after projectile advancement, holding `ResolvingFire` until all living
   tanks are stable; preserve the out-of-bounds path.
4. Update affected integration/turn tests for delayed and immediate handoff. Confirm tank visual
   and barrel origin already follow the authoritative pose; add regression coverage where needed.
5. Update player-facing terrain/shot/control documentation and only the proven roadmap entries;
   run all quality gates and the manual crater-under-tank validation.

## Complexity Tracking

No constitution violations or complexity exceptions require justification.
