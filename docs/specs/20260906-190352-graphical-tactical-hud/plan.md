# Implementation Plan: Graphical Tactical HUD

**Branch**: `20260906-190352-graphical-tactical-hud` | **Date**: 2026-09-06 | **Spec**: [spec.md](spec.md)

## Summary

Replace the temporary single text dump with a compact retained graphical tactical frame. A pure
read-only view derived from the current turn/match, tanks, wind, and movement feedback drives
native panels: player/match and health at top-left, shot/environment at top-right, and small
phase-relevant movement/control information at a lower corner. Health uses fill bars plus exact
numbers. Wind uses a geometric ASCII-labelled world X/Z plot with a positioned direction marker.

## Technical Context

**Language/Version**: Rust edition 2024, stable toolchain

**Primary Dependencies**: Bevy 0.18.1 native UI; no new dependency

**Storage**: In-memory authoritative game resources; no persistence

**Testing**: Rust unit tests via `cargo test --workspace`; manual desktop validation

**Target Platform**: Desktop Linux development environment; portable desktop game

**Project Type**: Single-workspace desktop game

**Performance Goals**: Synchronize one small retained UI tree without extra simulation work or
per-frame gameplay entity creation

**Constraints**: Read-only presentation; no gameplay/camera timing gate; no external assets;
ASCII-safe wind labels; central battlefield remains clear; normal desktop resize remains usable

**Scale/Scope**: One local two-player match, one match-constant wind, three compact HUD groups,
and current keyboard contexts

## Constitution Check

*GATE: Pass before Phase 0 research and after Phase 1 design.*

| Principle | Design response | Status |
|---|---|---|
| Fun/readability | Compact visual cues improve tactical reading without instrumentation clutter. | Pass |
| Simple coherent systems | One derived view and retained UI tree; no widget framework or duplicated state. | Pass |
| Deterministic/testable simulation | Pure mapping tests; HUD never enters simulation. | Pass |
| Presentation boundary | UI reads existing state only; it cannot gate input, fixed updates, or camera. | Pass |
| Scope/dependencies | Bounded frame, no assets/dependencies, menus, animations, or input redesign. | Pass |

**Post-design re-check**: Pass. The design preserves the existing one-way gameplay-to-presentation
dependency.

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260906-190352-graphical-tactical-hud/
├── plan.md              # This file ($speckit-plan command output)
├── research.md          # Phase 0 output ($speckit-plan command)
├── data-model.md        # Phase 1 output ($speckit-plan command)
├── quickstart.md        # Phase 1 output ($speckit-plan command)
├── contracts/           # Phase 1 output ($speckit-plan command)
└── tasks.md             # Phase 2 output ($speckit-tasks command - NOT created by $speckit-plan)
```

### Source Code (repository root)
```text
crates/azimuth-game/src/
├── main.rs          # HUD view mapping, frame spawn, retained display synchronization
├── turn.rs          # Existing read-only turn/match source
├── tank.rs          # Existing read-only health/elimination/movement source
├── projectile.rs    # Existing read-only wind source
├── aiming.rs        # Existing read-only aiming source
└── world.rs         # Existing world-axis conventions

docs/
├── roadmap.md
├── world-conventions.md
└── specs/20260906-190352-graphical-tactical-hud/
    ├── plan.md
    ├── research.md
    ├── data-model.md
    ├── contracts/tactical-hud.md
    └── quickstart.md
```

**Structure Decision**: Keep the existing single game crate. The temporary HUD, authoritative
resources, input, and camera boundary already live in `main.rs`; a small local HUD view and native
UI tree are sufficient. No crate, dependency, or general UI abstraction is justified.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No constitution violations or complexity exceptions require justification.
