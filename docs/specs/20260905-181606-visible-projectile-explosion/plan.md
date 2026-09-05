# Implementation Plan: First Boom — Visible Projectile Explosion

**Branch**: `20260905-181606-visible-projectile-explosion` | **Date**: 2026-09-05 | **Spec**: [spec.md](./spec.md)

## Summary

Add one bright, expanding primitive that is spawned once from the existing latest terrain-impact
position, grows over a short explicit visual duration, and despawns at expiry. Retain the existing
impact marker unchanged. The effect stays entirely in `main.rs` presentation code and consumes the
existing result directly; gameplay/simulation modules remain untouched.

## Technical Context

**Language/Version**: Rust stable, edition 2024

**Primary Dependencies**: Bevy 0.18.1 already present; no additions

**Storage**: N/A; short-lived in-memory presentation entity/component state

**Testing**: Rust unit tests for meaningful pure lifetime/scale values; manual desktop validation

**Target Platform**: Existing supported desktop environments with graphics support

**Project Type**: Rust Cargo-workspace desktop application with one executable crate

**Performance Goals**: One active short-lived effect in the single-shot workflow; bounded per-frame
component updates and no obsolete entities after expiry

**Constraints**: Consume only existing terrain impact; no physics, terrain, tank, marker change,
dependency, particle/animation/VFX framework, or gameplay radius

**Scale/Scope**: One effect mesh/material, one effect component, a small consumed-impact tracker,
and documentation/roadmap updates

## Constitution Check

| Principle | Plan compliance |
| --- | --- |
| Fun over realism | A bright exaggerated expanding boom prioritises impact readability. |
| Simple composable systems | One primitive, one elapsed lifetime, and existing impact state; no VFX framework. |
| Code coherence | Presentation code remains direct and local to the current Bevy scene. |
| Deterministic and testable simulation | Simulation is unchanged; pure timing/scale behaviour receives proportionate tests. |
| Presentation must not own the game | Existing simulation impact is the only origin; presentation neither detects nor changes impact. |
| Playable progress | Adds the first satisfying visible consequence to the arc-impact loop. |
| Scope and roadmap discipline | Excludes damage, deformation, sound, smoke, camera response, and gameplay explosion mechanics. |
| Quality and dependencies | Existing validation gates remain required; no dependency is added. |

**Gate result (pre-design)**: Pass.

## Design

### Explosion presentation and lifecycle

1. Add presentation-only constants for visual duration, initial scale, maximum scale, bright
   material, and a small vertical offset to avoid terrain z-fighting. They have no gameplay meaning.
2. Add one tagged explosion component holding elapsed visual time. Normal updates advance elapsed
   time from frame delta, set scale from an explicit initial-to-maximum curve, and despawn at expiry.
   Use the existing primitive mesh/material path; no particles, fade framework, or opacity change
   is needed.
3. Add small pure helpers for normalised lifetime progress and scale, with direct tests for start,
   intermediate progress, terminal progress, and expiry.

### Consume impact exactly once

1. Retain `LatestTerrainImpact` and its marker system unchanged. Add one local presentation
   resource recording whether the current latest impact has already produced an explosion.
2. When a latest impact first appears, spawn one explosion at its exact world position and mark it
   consumed. When a new launch clears latest impact, reset consumption. A later identical impact
   can then create a new boom, while the persistent marker result cannot respawn one every frame.
3. Do not add collision checks, events, gameplay radius, normals, material semantics, tank effects,
   or terrain changes. Out-of-bounds termination has no result and cannot create an explosion.

### Verification and documentation

1. Add focused unit tests for lifetime/scale and one-shot consumption/reset helpers where they are
   deterministic. Avoid screenshots and renderer-internal tests.
2. Manually fire the existing impact shot, inspect boom and marker coexistence, wait for cleanup,
   then fire again and confirm one new boom with no stale entity.
3. Update README and projectile-model documentation. Check only the near-term visible
   impact/explosion and explosion-presentation roadmap entries after verification.

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260905-181606-visible-projectile-explosion/
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
├── main.rs               # Existing scene plus local explosion presentation lifecycle
├── projectile.rs         # Unchanged authoritative impact simulation
├── battlefield.rs        # Unchanged terrain surface
├── tank.rs               # Unchanged tank data
└── world.rs              # Unchanged engine-independent values

docs/
├── projectile-model.md
├── roadmap.md
└── README.md
```

**Structure Decision**: Keep the effect in `main.rs`. It is an engine-native temporary response to
one already-local presentation resource, and no demonstrated reuse boundary warrants a module,
crate, generic framework, or external contract.

## Complexity Tracking

No constitution violations or exceptions require justification.

## Post-Design Constitution Check

Pass. The effect is a small presentation observer with direct cleanup. It preserves authoritative
simulation and the diagnostic marker while adding no speculative abstraction or dependency.
