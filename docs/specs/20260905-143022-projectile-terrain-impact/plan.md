# Implementation Plan: Projectile Terrain Impact Detection

**Branch**: `20260905-143022-projectile-terrain-impact` | **Date**: 2026-09-05 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `docs/specs/20260905-143022-projectile-terrain-impact/spec.md`

## Summary

Make the static battlefield's rendered triangle surface authoritative for terrain rendering,
height queries, tank grounding, and projectile impact. Preserve the fixed ballistic advance, test
its travelled segment for an above-to-below terrain crossing, refine the contact deterministically,
and return a small terrain-impact result that ends flight. Presentation only observes the result:
it removes the projectile visual and displays one lightweight marker.

## Technical Context

**Language/Version**: Rust stable, edition 2024

**Primary Dependencies**: Bevy 0.18.1, already present; no additions

**Storage**: N/A; static terrain and transient in-memory flight/impact state

**Testing**: Rust unit tests via `cargo test --workspace`; manual desktop validation

**Target Platform**: Existing supported desktop environments with graphics support

**Project Type**: Rust Cargo-workspace desktop application with one executable crate

**Performance Goals**: One projectile at 120 Hz, with a fixed bounded number of terrain-height
evaluations per flight step

**Constraints**: Preserve pre-impact ballistic calculation; no new dependency, physics engine,
generic collider, event bus, or render-owned simulation; retain distinct out-of-bounds/lifetime
termination; impact location within 0.01 world units of visible terrain

**Scale/Scope**: One 40-by-40 unit static grid, one active projectile, one latest impact, and one
presentation-only marker

## Constitution Check

| Principle | Plan compliance |
| --- | --- |
| Fun over realism | Direct surface crossing creates readable ground hits without full collision realism. |
| Simple composable systems | Static terrain height, existing ballistics, one result, and one marker; no framework. |
| Code coherence | `battlefield.rs` owns surface rules; `projectile.rs` owns pure flight advancement. |
| Purposeful workspace boundaries | Existing single-crate separation remains sufficient; no crate is added. |
| Deterministic and testable simulation | Pure values plus fixed-count refinement permit renderer-independent tests. |
| Presentation must not own the game | Bevy observes impact state only; it neither detects nor resolves collision. |
| Playable progress | Completes launch → flight → impact marker. |
| Scope and roadmap discipline | Excludes explosions, damage, deformation, non-terrain collision, and aiming. |
| Quality and dependencies | Existing quality commands remain required; no dependency is proposed. |

**Gate result (pre-design)**: Pass. The focused terrain refactor is needed to eliminate the
current smooth height-query versus triangulated rendered-surface mismatch.

## Design

### Authoritative terrain surface

1. Retain the bounded 20-cells-per-side generated grid and its current visual relief. Move shared
   grid-resolution and sampling knowledge into `battlefield.rs` so the domain, not `main.rs`, owns
   the surface definition.
2. Retain the existing continuous relief expression only to sample vertex elevations. Make
   `terrain_height(x, z)` interpolate the same piecewise planar triangles and cell diagonals
   emitted by the mesh. Queries, tank grounding, collision, and visible ground then agree.
3. Keep `is_within_bounds` as terrain's horizontal domain. Build the mesh from battlefield helpers.
4. Test bounds, determinism, non-flat relief, vertex agreement, and within-cell interpolation.

### Deterministic swept impact

1. Keep the existing fixed constant-acceleration candidate position/velocity calculation and
   elapsed-step increment unchanged.
2. Evaluate signed vertical separation from terrain at the prior and candidate positions. A prior
   point above and candidate point on/below the in-bounds terrain is a crossing; paths outside the
   terrain domain retain existing useful-volume/lifetime behaviour.
3. Refine a crossing along that same segment using 24 documented fixed bisection
   iterations. At each midpoint, interpolate X/Z, evaluate authoritative height, and retain the
   above-side or below-side interval. The fixed count is chosen to bound the residual interval
   below 0.01 world units for the relevant simulation segment.
4. Set position to the refined surface point and retain candidate velocity/elapsed step. Flight
   ends immediately; collision does not add bounce, impulse, or a post-impact ballistic state.
5. Replace the current boolean advance result with a small concrete outcome: `Active`,
   `TerrainImpact { position }`, or `OutOfBounds`. This is a direct step result, not a state
   machine or collision hierarchy.
6. Keep the entry point pure through a terrain-height function/value boundary, enabling synthetic
   flat and sloped terrain tests without Bevy types.

### Application lifecycle and marker

1. Retain `ProjectileFlight(Option<Projectile>)`. Add one optional latest-impact resource and
   clear it when the next development shot launches.
2. Fixed update advances against `battlefield::terrain_height`: retain Active; record and clear
   flight on TerrainImpact; clear flight without impact on OutOfBounds.
3. Existing projectile visual sync remains an observer. Add one tagged small primitive marker from
   the stored impact position, replace it on each impact, and remove it on the next launch. No
   effects, history, event system, or diagnostics service is added.
4. Preserve Space launch, current gravity, camera, tanks, axes, and one-flight behaviour.

### Verification and documentation

1. Add domain tests for above-terrain continuation, flat analytical descent, high-speed crossing,
   non-flat and synthetic slope, surface proximity, repeats, gravity variation, endpoint contact,
   and non-impact useful-volume termination.
2. Run check, test, format, and Clippy. Manually inspect shots from multiple terrain elevations and
   angles, marker placement, lack of through-ground flight, and out-of-bounds termination.
3. Update projectile model, world conventions, and README. After verification, check only the
   impact, terrain-intersection, impact-marker, and (if truly satisfied) terrain-representation
   roadmap entries. Do not mark explosion or later-loop work complete.

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260905-143022-projectile-terrain-impact/
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
├── battlefield.rs        # Authoritative triangulated terrain surface and bounds
├── projectile.rs         # Pure fixed-step swept impact and outcome types
├── tank.rs               # Reuses local terrain height
├── world.rs              # Existing engine-independent position/vector values
└── main.rs               # Mesh consumer, flight lifecycle, visual and impact marker

docs/
├── projectile-model.md
├── roadmap.md
└── world-conventions.md
```

**Structure Decision**: Keep this work in the existing executable crate. `battlefield.rs` and
`projectile.rs` are already the demonstrated domain boundary, while direct scene integration stays
in `main.rs`. No external interface is introduced, so no `contracts/` directory is warranted.

## Complexity Tracking

No constitution violations or exceptions require justification.

## Post-Design Constitution Check

Pass. The design makes one existing static terrain surface authoritative, uses fixed-cost direct
deterministic calculations, retains presentation-independent simulation, and adds one disposable
visual observer. No dependencies, generic abstractions, or scope-expanding systems are introduced.
