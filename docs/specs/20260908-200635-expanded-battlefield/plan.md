# Implementation Plan: Expanded Battlefield — Mountains, Valleys, Terrain Colour and World Dressing

**Branch**: `20260908-200635-expanded-battlefield` | **Date**: 2026-09-08 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `docs/specs/20260908-200635-expanded-battlefield/spec.md`

## Summary

Replace the 40-by-40 authored development arena with one 120-by-120 deterministic Azimuth battlefield. Keep the current mutable triangle-height terrain as the only authoritative world surface, but generate it from a captured match seed using distinct macro geography and local variation. Add current-height vertex colours, a flat visual-only water plane, sparse visual-only building blocks, deterministic distributed 2–8 player starts, and focused projectile/camera limit changes. Preserve all existing ballistics, wind, weapons, crater, support, movement, turn, HUD, and match semantics.

The design uses a 64-cell-per-side terrain (65 by 65 vertices, 1.875 units per cell). This keeps the map large without matching area growth with excessive mesh density: the Basic Shell crater spans about 4.3 cells in diameter and the HE crater about 6.4 cells, while the full terrain remains only 4,225 vertices and 24,576 indices.

## Technical Context

**Language/Version**: Rust edition 2024

**Primary Dependencies**: Bevy 0.18.1; existing Rust standard library only for deterministic seed mixing

**Storage**: In-memory match resources; no persistence or external storage

**Testing**: `cargo test` unit/regression coverage in source modules; `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`

**Target Platform**: Desktop application supported by Bevy’s current local development setup

**Project Type**: Single Rust desktop game application

**Performance Goals**: Maintain a responsive playable local match while rendering one 4,225-vertex terrain, one water plane, and sparse cuboid dressing; terrain refresh after an existing crater must remain visually prompt and preserve fixed-step gameplay.

**Constraints**: One 120-by-120 map; at most 65 by 65 terrain vertices; existing 1/120-second simulation; no new dependency, generic world-object system, water physics, collision for dressing, AI, or map-selection UI; current 1-unit movement / 0.75 elevation-change rule and existing weapon identities remain unchanged.

**Scale/Scope**: One deterministic battlefield definition, five representative recorded-seed validation cases, 2–8 human starts, one visual water table, and sparse primitive buildings. Current source ownership remains in `battlefield.rs`, `tank.rs`, `projectile.rs`, and `main.rs`.

## Constitution Check

### Pre-design gate — PASS

- **Fun over realism / emergent gameplay**: Macro terrain changes existing ballistic paths and movement context without special mountain, water, or building rules.
- **Coherence / purposeful boundaries**: Extend the current single height field and direct Bevy scene code. Do not add a biome engine, object model, physics layer, or dependency.
- **Determinism / testability**: Capture one match seed and derive labelled independent sub-seeds; keep all terrain/start tests pure and renderer-independent where possible.
- **Presentation does not own the game**: Water and buildings have explicit visual-only data and scene entities; they do not enter terrain, tank, projectile, combat, or turn logic.
- **Playable progress / scope / roadmap**: Preserve the existing complete human artillery loop and defer fair-start analysis, authoritative structures, water gameplay, and AI.
- **Quality and documentation**: Update world/projectile documentation and roadmap only after implementation/manual evidence; run build, tests, formatter, and Clippy.

No violations require a complexity exception.

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260908-200635-expanded-battlefield/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── battlefield-generation.md
└── tasks.md                 # Created later by $speckit-tasks
```

### Source Code (repository root)

```text
crates/azimuth-game/
├── src/
│   ├── battlefield.rs       # dimensions, generated authoritative heights, colour/water helpers
│   ├── tank.rs              # deterministic suitable-spawn selection and validation
│   ├── projectile.rs        # battlefield-aware useful-volume limits and tests
│   ├── main.rs              # captured seed, scene dressing/water, mesh colours, camera integration
│   ├── match_setup.rs       # existing 2–8 setup remains unchanged
│   ├── combat.rs            # existing blast regression coverage
│   ├── turn.rs
│   ├── weapon.rs
│   └── world.rs
├── Cargo.toml
docs/
├── world-conventions.md
├── projectile-model.md
└── roadmap.md
```

**Structure Decision**: Retain the existing small, coherent module layout. Terrain domain rules live in `battlefield.rs`; spawn validation belongs alongside tank placement in `tank.rs`; only Bevy-facing visual entities and resource wiring belong in `main.rs`.

## Implementation Approach

1. Establish the battlefield domain contract first: replace old dimensions/grid constants, add a captured `BattlefieldSeed`, define labelled seed derivation, water table, height-colour mapping, and a fixed generated-terrain constructor. Preserve `BattlefieldTerrain` as the mutable height vector driving interpolation, craters, collision, and mesh positions.
2. Implement bounded macro shapes (dominant side mountain, elongated ridge, broad bowl/valley) plus low-amplitude local variation. Sample all parameters from the terrain-specific seed in bounded regions so every seed retains useful dry flatter areas. Do not introduce generic noise or biome abstractions.
3. Replace fixed tank locations with deterministic candidate selection using the start-specific seed: select distributed sector candidates, reject invalid terrain, then build tanks at accepted locations. Generate dressing only after starts are final so the presentation path cannot change starts or wind.
4. Extend the existing mesh creation/refresh path to add per-vertex current-height colour. Spawn a static water plane and simple cuboid building scene entities from visual dressing records only. Leave all authoritative queries and collision paths untouched.
5. Make map-sized simulation/camera bounds explicit, then prove current weapons still work before considering any global limit tune. Update documentation and roadmap only after automated and manual validation.

## Constitution Check

### Post-design gate — PASS

The detailed model and contract preserve one authoritative mutable terrain surface, isolate deterministic streams, and place only presentation state in renderer-facing records. The design creates no new crate, external dependency, generic object hierarchy, physics system, or speculative abstraction. The only new data is narrowly required for the 120-unit battlefield, starts, water, colour, and dressing; remaining fairness, collision, and water behaviour are explicitly deferred.

## Complexity Tracking

No constitution violation or complexity exception is present.
