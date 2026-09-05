# Implementation Plan: First Crater — Terrain Deformation

**Branch**: `20260905-183525-first-crater-terrain-deformation` | **Date**: 2026-09-05 | **Spec**: [spec.md](./spec.md)

## Summary

Evolve the fixed triangle-grid battlefield into one mutable, engine-independent terrain value that
owns sampled vertex heights. Apply a deterministic radial lowering profile to those heights from
each authoritative projectile impact, regenerate the compact rendered mesh, and pass the same
terrain query to later swept projectile steps. The existing boom remains presentation only; crater
radius and depth are explicit gameplay parameters.

## Technical Context

**Language/Version**: Rust stable, edition 2024

**Primary Dependencies**: Bevy 0.18.1 already present; no additions

**Storage**: One in-memory bounded terrain grid and a renderer-owned mesh derived from it

**Testing**: Rust unit tests for terrain domain and projectile interaction; manual desktop validation

**Target Platform**: Existing supported desktop environments with graphics support

**Project Type**: Rust Cargo-workspace desktop application with one executable crate

**Performance Goals**: Regenerate one compact 20-by-20-cell battlefield mesh per impact; no
per-frame terrain processing while terrain is unchanged

**Constraints**: One authoritative terrain state; deterministic deformation and collision; no
physics engine, terrain engine, chunking, GPU deformation, damage, or tank repositioning

**Scale/Scope**: One default crater profile, bounded grid deformation, mesh refresh, and direct
integration with the existing single-projectile impact path

## Constitution Check

| Principle | Plan compliance |
| --- | --- |
| Fun over realism | A simple exaggerated bowl-shaped crater prioritises readable terrain change. |
| Simple composable systems | One grid, one radial lowering function, and existing impact data; no terrain framework. |
| Code coherence | Focused refactoring replaces static height generation with mutable terrain. |
| Deterministic and testable simulation | Terrain mutation and queries are engine-independent deterministic functions. |
| Presentation must not own the game | Mesh recreation observes terrain; simulation impact changes terrain, not the boom. |
| Playable progress | Completes fire → impact → boom → permanent changed ground → next shot. |
| Scope and roadmap discipline | Excludes tank response, damage, deposition, particles, and terrain optimisation systems. |
| Quality and dependencies | Existing quality gates remain required; no dependency or crate is added. |

**Gate result (pre-design)**: Pass.

## Design

### Authoritative mutable terrain

1. Replace static terrain-height generation with a concrete `BattlefieldTerrain` domain value that
   stores current heights for the existing bounded grid. Its constructor samples the current
   authored non-flat relief once, preserving the initial battlefield.
2. Keep triangle interpolation inside this type. Its in-bounds and optional height queries use the
   same current vertex data used to generate mesh positions, so rendering, tank grounding at
   startup, and projectile collision share one surface.
3. Keep mesh indices and horizontal grid layout fixed. This feature changes only elevations; it
   adds no terrain chunks, collision meshes, or second height representation.

### Deterministic crater application

1. Add a small `Crater` configuration with explicit positive finite radius and depth, plus one
   readable default. It is gameplay data and has no relationship to visual explosion scale.
2. For every valid grid vertex within the crater radius, subtract a smooth radial bowl lowering
   from its current height. Use a zero-at-edge falloff, so the centre gets maximum lowering and
   the boundary joins unchanged terrain.
3. Iterate grid vertices in fixed stored order. Clipping follows from visiting only valid vertices;
   overlapping impacts subtract from current heights and compose deterministically.

### Impact, collision, and rendering integration

1. Construct terrain before initial tanks and retain it as one application resource. Update tank
   initial grounding and projectile terrain queries to read it rather than module globals.
2. When fixed-step projectile simulation returns its existing terrain impact, apply the default
   crater immediately from that resolved point before ending flight. Out-of-bounds termination has
   no impact and cannot deform terrain.
3. Tag the battlefield visual and, only after terrain changes, refresh its mesh from current terrain
   positions. The compact mesh is regenerated once per crater; rendering neither calculates nor
   owns deformation.

### Verification and documentation

1. Add terrain-domain tests for profile, radius/depth, exterior preservation, slopes, overlap,
   edge clipping, finiteness, query/rendered-vertex agreement, and repeatability.
2. Add projectile tests using mutable terrain for travel through removed ground and impact on the
   new crater surface. Avoid renderer assertions.
3. Manually fire the impact shot repeatedly, inspect the boom and crater, and confirm later shots
   interact with lowered ground. Update README, world/projectile documentation, and only completed
   deformation roadmap items.

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260905-183525-first-crater-terrain-deformation/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── checklists/
│   └── requirements.md
└── tasks.md                  # Created by $speckit-tasks
```

### Source Code (repository root)

```text
crates/azimuth-game/src/
├── battlefield.rs        # Mutable terrain domain, crater profile, queries, mesh position data
├── projectile.rs         # Swept flight generalized only to query current terrain
├── tank.rs               # Initial grounding from constructed authoritative terrain
├── main.rs               # Terrain resource, impact-to-crater integration, mesh refresh
└── world.rs              # Shared engine-independent world values

docs/
├── projectile-model.md
├── world-conventions.md
├── roadmap.md
└── README.md
```

**Structure Decision**: Keep mutable terrain in the existing `battlefield.rs` domain module and
keep renderer refresh in `main.rs`. Gameplay terrain stays independent of Bevy; the only
engine-native work is rebuilding its derived mesh. No external interface is introduced, so no
contracts directory is needed.

## Complexity Tracking

No constitution violations or exceptions require justification.

## Post-Design Constitution Check

Pass. The design has one direct mutable terrain value, derives all relevant consumers from it, and
uses a fixed deterministic profile and compact mesh refresh without speculative infrastructure.
