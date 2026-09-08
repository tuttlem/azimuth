# Implementation Plan: Presentation Polish — Full Player Scoreboard and Immersive Battlefield Background

**Branch**: `20260909-074958-presentation-polish` | **Date**: 2026-09-09 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/docs/specs/20260909-074958-presentation-polish/spec.md`

## Summary

Generalise the existing top-left player panel from two fixed entries to retained rows generated
from the ordered `MatchConfiguration.players` collection. A pure scoreboard projection joins each
configured player to their tank and current turn, then the HUD renders that read-only result with
the established colours, health bars, active border treatment, and compact count-aware spacing.

Replace the empty world backdrop with a clear blue sky, static simple cloud clusters, and one
low-detail `HorizonLandscape` mesh. The mesh welds to the authoritative terrain boundary and uses
the captured terrain seed outside it for visual continuity, has no terrain-query or collision API,
and is never passed to gameplay. Retain existing controls but cap orbit at the smallest safe
downward/near-horizontal maximum so ordinary views cannot put the camera below the terrain.

## Technical Context

**Language/Version**: Rust edition 2024

**Primary Dependencies**: Bevy 0.18.1 (existing engine/rendering/UI dependency only)

**Storage**: N/A; match, terrain, and presentation data are in-memory

**Testing**: `cargo test`; pure unit tests colocated with the existing Rust modules; manual graphical acceptance

**Target Platform**: Current desktop platforms supported by Bevy’s default plugins

**Project Type**: Single-crate desktop 3D game in a Cargo workspace

**Performance Goals**: Preserve responsive ordinary 2-, 4-, and 8-player play; add one low-detail static horizon mesh and a small fixed cloud set without increasing authoritative terrain density

**Constraints**: 2–8 scoreboard rows; player configuration is canonical ordering/name metadata; `BattlefieldTerrain` remains the only mutable gameplay terrain; no new dependency, external asset pipeline, renderer framework, outer-world collision, or pixel tests

**Scale/Scope**: One existing game crate, one HUD panel, one static visual horizon/background setup, and focused pure-state/boundary tests

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Pre-research gate: PASS**

- Fun and playable progress: improves the readability and atmosphere of the currently playable
  local match; no new game-system detour.
- Simple composable systems: uses one collection projection, static simple presentation geometry,
  and existing scene/UI facilities rather than generic HUD or environment infrastructure.
- Coherence and purposeful boundaries: changes remain in the existing crate; pure gameplay types
  remain free of engine-specific types; only presentation consumes meshes/materials/UI entities.
- Determinism and testability: no new gameplay randomness or authority is introduced; pure
  scoreboard and horizon descriptors are testable without rendering, while manual checks cover
  visual acceptance.
- Presentation must not own the game: configured players, tanks, turns, and the existing bounded
  battlefield stay authoritative. Horizon/sky/clouds have no gameplay APIs or mutation path.
- Scope/roadmap/quality: excludes AI and the listed atmosphere, weather, streaming, physics, and
  UI redesign work; retains the next AI controller feature; requires focused tests plus workspace
  format, test, lint, and build checks.

**Post-design gate: PASS**

The selected design adds no crate, dependency, generic framework, second terrain authority, or
simulation state. The explicit read-only scoreboard projection and presentation-only horizon
descriptor give the required test seams while preserving the project’s direct architecture.

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260909-074958-presentation-polish/
├── plan.md              # This file ($speckit-plan command output)
├── research.md          # Phase 0 output ($speckit-plan command)
├── data-model.md        # Phase 1 output ($speckit-plan command)
├── quickstart.md        # Phase 1 output ($speckit-plan command)
├── contracts/           # Phase 1 output ($speckit-plan command)
└── tasks.md             # Phase 2 output ($speckit-tasks command - NOT created by $speckit-plan)
```

### Source Code (repository root)
```text
crates/azimuth-game/
├── src/
│   ├── main.rs           # Existing Bevy scene, camera, HUD, scoreboard sync, pure view tests
│   ├── battlefield.rs    # Authoritative terrain plus pure visual-horizon descriptor/mesh data
│   ├── match_setup.rs    # Ordered configured player metadata (unchanged authority)
│   ├── tank.rs           # PlayerId, Tank health/elimination/position (unchanged authority)
│   └── turn.rs           # Turn/match state (unchanged authority)
└── Cargo.toml
```

**Structure Decision**: Keep the existing single game crate and its direct module boundaries.
`battlefield.rs` owns engine-independent visual-horizon data because it can sample the
authoritative surface without exposing or changing gameplay queries; `main.rs` converts that data
to existing render/UI entities and contains the pure scoreboard projection alongside its current
HUD view. No new crate or cross-cutting framework is warranted.

## Complexity Tracking

No constitution violations or complexity exceptions require tracking.
