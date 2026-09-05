# Implementation Plan: Minimal Turn Loop

**Branch**: `20260905-225321-minimal-turn-loop` | **Date**: 2026-09-05 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/docs/specs/20260905-225321-minimal-turn-loop/spec.md`

## Summary

Add an explicit two-player turn loop without expanding the game into a general match framework.
One small pure turn model owns Player One/Player Two order and Ready/Resolving phase; a matching
small player-aim collection retains one established `AimingState` per existing player. Direct Bevy
systems select the current player's tank and aim for input, launch, HUD, and tank visuals. The
existing fixed-step projectile system remains authoritative: its terminal terrain-impact path
applies the crater, records the impact, clears flight, and advances exactly once in that order;
its non-impact path clears flight and advances without invented impact state. The explosion and
mesh refresh remain derived presentation and never gate turn progression.

## Technical Context

**Language/Version**: Rust edition 2024 on the repository's current stable toolchain

**Primary Dependencies**: Bevy 0.18.1, using its existing desktop application, input, fixed-time,
rendering, and UI facilities

**Storage**: N/A; turn and aim state are authoritative in-memory gameplay state for the running
match

**Testing**: `cargo test --workspace`; deterministic unit tests for the pure turn/aim model,
focused resolution helpers, player-specific launch derivation, and existing projectile/terrain
domain tests

**Target Platform**: Desktop development platforms supported by the existing Bevy application

**Project Type**: Single-crate desktop game application

**Performance Goals**: Preserve the existing interactive rendering and deterministic 120 Hz
projectile simulation; player selection, aim lookup, and turn transition are constant-size work
for exactly two players

**Constraints**: Exactly two fixed existing players; one active projectile; only Ready accepts
aim/fire input; terrain impact/deformation must precede advancement; out-of-bounds/lifetime
termination advances without impact; no dependencies, crates, movement, damage, elimination,
match orchestration, or generic turn/action framework

**Scale/Scope**: Two retained aim states, a two-state turn loop, one shared keyboard surface, two
tank visual firing representations, one HUD update, existing projectile/impact/deformation flow,
and focused documentation/roadmap updates

## Constitution Check

*GATE: Passed before Phase 0 research. Re-checked after Phase 1 design: passed.*

| Principle | Design response | Status |
|-----------|-----------------|--------|
| Fun over realism; simple, emergent systems | Alternating a single learnable fire action lets each player observe terrain changes and correct the retained prior shot without adding simulation rules. | Pass |
| Code coherence; purposeful boundaries | A compact pure `turn.rs` owns only current player, phase, and two fixed player aims. Existing `aiming`, tank, projectile, and battlefield concepts are reused; no generic framework or crate is introduced. | Pass |
| Deterministic, testable simulation | Turn transitions occur in the existing fixed-step terminal-resolution path. Pure domain operations and resolution helpers provide renderer-independent deterministic coverage. | Pass |
| Presentation must not own game | HUD, tank transforms, mesh refresh, marker, and boom observe gameplay state. Crater application and advancement happen before presentation, and boom lifetime cannot gate a turn. | Pass |
| Playable progress and scope discipline | Delivers the first complete two-player aim → fire → resolve → alternate loop while deliberately deferring movement, damage, winners, and final hot-seat polish. | Pass |
| Roadmap and quality discipline | The implementation will update only demonstrated turn-flow, local-player, current-player, and HUD items; surviving-player advancement and match completion stay open. Standard Cargo quality gates and docs are included. | Pass |
| Dependencies earn their place | Existing Bevy and standard Rust state are sufficient. No dependency change is needed. | Pass |

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260905-225321-minimal-turn-loop/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/turn-controls.md
└── tasks.md                 # Created later by $speckit-tasks
```

### Source Code (repository root)

```text
crates/azimuth-game/
├── src/
│   ├── main.rs          # Bevy integration: resources, input, launch, fixed resolution, HUD, visuals
│   ├── turn.rs          # New pure two-player current-turn and retained-aim gameplay state
│   ├── aiming.rs        # Existing aim validation, adjustments, and canonical shot construction
│   ├── tank.rs          # Existing player IDs, tanks, and player-specific firing representations
│   ├── projectile.rs    # Existing deterministic fixed-step trajectory and terminal outcomes
│   ├── battlefield.rs   # Existing authoritative mutable terrain and crater operation
│   └── world.rs         # Existing engine-independent positions and vectors
└── Cargo.toml           # Unchanged dependency set

docs/
├── projectile-model.md  # Update Player One-only wording to current-player turn semantics
├── world-conventions.md # Retain shared firing-origin convention; update only if turn ownership needs clarity
├── roadmap.md           # Check only demonstrated minimal-turn, player, and HUD items at completion
└── specs/20260905-225321-minimal-turn-loop/
    ├── plan.md
    ├── research.md
    ├── data-model.md
    ├── contracts/turn-controls.md
    ├── quickstart.md
    └── tasks.md          # Created later by $speckit-tasks
```

**Structure Decision**: Keep the existing single application crate. Add `turn.rs` because the
feature needs a compact engine-independent ownership boundary that can prove turn invariants
without renderer or scheduler tests. It is not a generic turn engine: it represents only two fixed
players, Ready/Resolving, and their two existing aiming states. `main.rs` remains the direct Bevy
integration boundary; the existing domain modules retain their current responsibilities.

## Complexity Tracking

No constitution violations require justification.
