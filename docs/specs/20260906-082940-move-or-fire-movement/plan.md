# Implementation Plan: Move or Fire — Basic Tactical Movement

**Branch**: `20260906-082940-move-or-fire-movement` | **Date**: 2026-09-06 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/docs/specs/20260906-082940-move-or-fire-movement/spec.md`

## Summary

Evolve the existing two-player `Ready`/`Resolving` loop into a compact explicit choice state:
choose an action, move with a six-step allowance, or resolve a fired shot. Movement is a
deterministic one-world-unit cardinal request against the current authoritative terrain. A pure
tank operation validates bounds and a 0.75 rise/run limit before producing a new grounded pose;
the turn state owns action exclusivity, remaining steps, and player handoff. Bevy remains a direct
adapter for conflict-free keyboard input, HUD feedback, and transforms derived from gameplay state.
The projectile's fixed-step impact/deformation-before-handoff ordering is unchanged.

## Technical Context

**Language/Version**: Rust edition 2024 on the repository's current stable toolchain

**Primary Dependencies**: Existing Bevy 0.18.1 desktop rendering, input, fixed-time, and UI facilities; no additions

**Storage**: In-memory authoritative two-tank, two-aim, turn-action, and mutable terrain state

**Testing**: `cargo test --workspace`; pure unit tests in `battlefield.rs`, `tank.rs`, and `turn.rs`, plus focused pure helpers in `main.rs`

**Target Platform**: Existing desktop development platforms supported by the Bevy application

**Project Type**: Single-crate desktop game application in a Rust Cargo workspace

**Performance Goals**: Preserve interactive rendering and deterministic 120 Hz projectile simulation. Each movement request performs constant-size terrain queries and updates at most one of two tank poses; no per-frame movement simulation is added.

**Constraints**: Exactly two fixed players; one active projectile; six one-unit cardinal movement requests per movement action; maximum 0.75 rise/run; movement input must not depend on render-frame duration; current terrain is the sole ground source; one action per turn; no vehicle physics, pathfinding, collision, tank settling, damage, or new dependencies.

**Scale/Scope**: One pure movement operation and failure result; three explicit turn action states; two mutable tank poses; direct keyboard/HUD/transform projections; documentation and focused regression coverage for the existing one-crate game.

## Constitution Check

*GATE: Passed before Phase 0 research. Re-checked after Phase 1 design: passed.*

| Principle | Design response | Status |
|-----------|-----------------|--------|
| Fun over realism; simple emergent systems | Six discrete steps, a readable slope threshold, and changed terrain make location matter without simulating a vehicle. Craters affect movement solely because they change the same terrain surface. | Pass |
| Code coherence; purposeful boundaries | Extend existing `turn`, `tank`, and `battlefield` domain modules; use a small movement request/result type rather than an action or navigation framework. Keep one application crate. | Pass |
| Deterministic and testable simulation | A step is an ordered event, not a delta-time motion. Terrain queries, validation, pose changes, allowance, and handoff are pure, reproducible domain operations. | Pass |
| Presentation must not own the game | Bevy reads authoritative tanks and turn state for transforms and text. HUD feedback and visual boom cannot authorize a step, fire, or handoff. | Pass |
| Playable progress and scope discipline | Delivers the explicit sacrifice of shot for position. It excludes damage, settling, physics, collision, AI, pathfinding, and tactical claims requiring playtesting. | Pass |
| Roadmap and quality discipline | Update only demonstrated move-or-fire, slope, and basic movement items after validation. Keep tank-settling and tactical-effect work open. Full Cargo quality gates and controls documentation are required. | Pass |
| Dependencies earn their place | Existing Rust and Bevy facilities are sufficient. No crate or dependency is warranted. | Pass |

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260906-082940-move-or-fire-movement/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── turn-controls.md
├── checklists/
│   └── requirements.md
└── tasks.md                 # Created later by $speckit-tasks
```

### Source Code (repository root)

```text
crates/azimuth-game/src/
├── main.rs          # Direct Bevy input, resources, HUD, and tank-pose visual projections
├── turn.rs          # Two-player action choice, movement allowance, aim ownership, and handoff
├── tank.rs          # Tank poses, cardinal movement request/result, and terrain-grounded step
├── battlefield.rs   # Existing mutable terrain plus local slope/passability query
├── aiming.rs        # Existing independent player aiming values and shot construction
├── projectile.rs    # Existing fixed-step flight and terminal outcomes
└── world.rs         # Existing engine-independent positions and vectors

README.md                    # Update playable status and controls
docs/world-conventions.md    # Document tactical step and surface-following conventions
docs/roadmap.md              # Check only completed demonstrated items at implementation completion
```

**Structure Decision**: Keep the single game crate and existing pure-domain modules. `tank.rs` earns the movement operation because a tank pose is changed there and it remains independently testable without Bevy. `turn.rs` continues to own exactly the fixed two-player action lifecycle. `battlefield.rs` remains the only terrain representation and owns the local slope query. No new module, crate, trait hierarchy, ECS layer, or navigation abstraction is needed.

## Design Sequence

1. Add the current-surface slope/passability query to `BattlefieldTerrain`, reusing existing in-bounds height interpolation. It compares current-surface heights over the one-unit request and produces no alternate terrain data.
2. Add a cardinal movement request, fixed allowance/default limit, accepted pose update, and clear rejection reasons at the tank-domain boundary. Validate bounds and slope before copying a changed pose; update only position and `body_forward`.
3. Replace `Ready` with explicit action-choice state and add a movement state carrying remaining steps. Keep fired resolving state. Gate aim, firing, movement start, step consumption, and completion through this small turn model.
4. Make the application tank resource mutable and make launch/turret/muzzle lookup read current poses. Add M, I/J/K/L, and Enter adapters that issue one ordered domain request and record concise feedback; do not put movement in the camera or render-time path.
5. Add tank-root/body visual tags so translation and body direction follow changed pose while turret/barrel orientation continues to derive from retained aim independently.
6. Add domain tests, preserve projectile resolution ordering tests, document controls, and update only verified roadmap checkboxes.

## Complexity Tracking

No constitution violations or exceptions require justification.
