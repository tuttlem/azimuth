# Implementation Plan: Basic Player Aiming Controls

**Branch**: `20260905-212459-basic-player-aiming-controls` | **Date**: 2026-09-05 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/docs/specs/20260905-212459-basic-player-aiming-controls/spec.md`

## Summary

Replace the fixed Space/I development launches with Player One's persistent, authoritative aiming
state: normalized azimuth, bounded elevation, and bounded launch velocity. Add a compact pure
`aiming` module that owns ranges and fine/coarse adjustments; retain `ShotParameters` as the sole
angle-to-vector conversion. Derive the active tank's turret yaw, barrel direction, muzzle, and
launch parameters from that state so the rendered barrel end is the projectile origin. Use direct
Bevy input and a single corner text entity for presentation. Existing fixed-step flight, impact,
boom, crater, and terrain deformation continue unmodified after launch.

## Technical Context

**Language/Version**: Rust edition 2024 on the repository's current stable toolchain

**Primary Dependencies**: Bevy 0.18.1 (existing direct desktop application/rendering/input/UI dependency)

**Storage**: N/A; aim state is in-memory gameplay state retained for the running match

**Testing**: `cargo test --workspace`; pure deterministic unit tests in `aiming.rs`, `tank.rs`, and
`projectile.rs`, plus focused launch/input helpers in `main.rs`

**Target Platform**: Desktop development platforms supported by the existing Bevy application

**Project Type**: Single-crate desktop game application

**Performance Goals**: Maintain interactive rendering and the existing deterministic 120 Hz
projectile fixed step; aim updates and one HUD text refresh are constant-size work per frame

**Constraints**: One active projectile; all aim/fire input ignored while that flight is active;
no new dependencies, crates, generic input/UI/weapon architecture, turns, or player switching

**Scale/Scope**: One controllable existing tank, three tunable values, six adjustment keys with a
Shift coarse modifier, one fire key, one HUD text entity, and existing two-tank battlefield

## Constitution Check

*GATE: Passed before Phase 0 research. Re-checked after Phase 1 design: passed.*

| Principle | Design response | Status |
|-----------|-----------------|--------|
| Fun over realism; simple, emergent systems | Explicit 5–85 degree and 8–30 units/s game limits, plus fine/coarse increments, make correction learnable without modelling realism. | Pass |
| Code coherence; purposeful boundaries | One small `aiming.rs` domain module and focused changes to existing tank/projectile/application code; no new crate, traits, or framework. | Pass |
| Deterministic, testable simulation | Aim normalization, bounds, adjustments, canonical launch conditions, and muzzle derivation are pure deterministic tests. | Pass |
| Presentation must not own game | `AimingState` is authoritative; tank transforms and HUD read derived data only. `ShotParameters` remains the sole angle conversion. | Pass |
| Playable progress and scope discipline | Delivers aim → fire → impact → adjust for Player One only; turns, damage, weapons, AI, and aiming assistance remain excluded. | Pass |
| Roadmap and quality discipline | Plan includes required README/world/projectile documentation, precise roadmap updates only after demonstrated completion, and full Cargo quality gates. | Pass |
| Dependencies earn their place | Existing Bevy UI/input facilities suffice. No dependency change. | Pass |

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260905-212459-basic-player-aiming-controls/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/player-controls.md
└── tasks.md                 # Created later by $speckit-tasks
```

### Source Code (repository root)

```text
crates/azimuth-game/
├── src/
│   ├── main.rs          # Bevy resources, direct input, launch handoff, tank/HUD visual sync
│   ├── aiming.rs        # New pure aim state, limits, adjustments, shot construction helpers
│   ├── projectile.rs    # Existing canonical shot-angle conversion; minor reuse helper if needed
│   ├── tank.rs          # Tank-relative, direction-derived muzzle representation
│   ├── battlefield.rs   # Existing authoritative mutable terrain (unchanged behaviour)
│   └── world.rs         # Existing engine-independent positions and vectors
└── Cargo.toml           # Unchanged dependency set

docs/
├── projectile-model.md  # Replace development-shot description with player controls/launch rule
├── world-conventions.md # Document pitch-aware barrel-end firing-origin convention
├── roadmap.md           # Check only demonstrated aiming/HUD entries at completion
└── specs/20260905-212459-basic-player-aiming-controls/
    ├── plan.md
    ├── research.md
    ├── data-model.md
    ├── contracts/player-controls.md
    ├── quickstart.md
    └── tasks.md          # Created later by $speckit-tasks
```

**Structure Decision**: Keep the existing single application crate. A new pure `aiming.rs` earns
its place because it concentrates gameplay-owned, independently testable state and validation;
it does not create another crate or an engine abstraction. `main.rs` remains the direct Bevy
integration boundary, while `tank.rs` owns tank-relative geometry.
