# Implementation Plan: Tactical Controls and Camera Flow

**Branch**: `20260906-092436-tactical-controls-camera` | **Date**: 2026-09-06 | **Spec**: [spec.md](spec.md)

## Summary

Replace press-only Q/E, R/F, and T/G aiming with arrows for azimuth/elevation and `-`/`=` for
power. Add immediate plus held-key repeat (300 ms delay; 100 ms interval); retain Shift coarse
adjustment. Retire keyboard camera pan but keep mouse orbit and wheel zoom.

Extend the existing `BattlefieldCamera` with a presentation-local intent and desired pose. It
observes turn, tank, and projectile-flight state to seek a bounded active-player or wide-shot
view. It uses render time only; fixed projectile simulation and turn authority do not read it.

## Technical Context

**Language/Version**: Rust edition 2024, stable toolchain (`rust-toolchain.toml`)

**Primary Dependencies**: Bevy 0.18.1 (desktop app, ECS, input, rendering, UI)

**Storage**: In-memory Bevy resources/components; no persistence or external service

**Testing**: Rust unit tests with `cargo test --workspace`; manual desktop validation via
`cargo run --package azimuth-game`

**Target Platform**: Desktop Linux development; portable Bevy desktop application

**Project Type**: Single Rust-workspace desktop game application

**Performance Goals**: Smooth bounded presentation during normal play; preserve 120 Hz fixed
projectile simulation

**Constraints**: Camera/repeat timing must not alter turn, terrain, projectile, movement, or fixed
simulation. No dependencies, configurable bindings, camera engine, or vehicle/follow camera.

**Scale/Scope**: One game crate; direct edits primarily in `crates/azimuth-game/src/main.rs`,
plus controls documentation and truthful roadmap updates

## Constitution Check

| Principle | Design response | Status |
|---|---|---|
| I. Fun Over Realism | Familiar controls and readable views take priority over camera realism. | Pass |
| III. Code Coherence | Direct mappings, a tiny repeat resource, and pose interpolation in the existing camera. | Pass |
| V. Determinism/Testability | Turn/projectile/movement logic stays unchanged; pure helpers receive unit coverage. | Pass |
| VI. Presentation Must Not Own the Game | Camera reads game resources; no authoritative system reads camera/repeat presentation state. | Pass |
| VII/VIII. Progress and Scope | Bounded control/presentation slice; no generic binding or cinematic framework. | Pass |
| IX/X. Roadmap and Specs | Timestamped artifacts; only demonstrated roadmap outcomes are checked. | Pass |
| XI/XII. Quality/Dependencies | Existing Cargo checks and no new dependency. | Pass |

**Post-design re-check**: Pass. The design has no authority path from presentation state to
gameplay state.

## Project Structure

```text
crates/azimuth-game/src/
├── main.rs          # Bevy input, camera presentation, HUD, visual integration, tests
├── aiming.rs        # Existing bounded per-player aim values and adjustments
├── tank.rs          # Existing tank poses/body orientation
├── turn.rs          # Existing authoritative turn state
├── projectile.rs    # Existing fixed-step projectile simulation
└── battlefield.rs   # Existing authoritative terrain

docs/specs/20260906-092436-tactical-controls-camera/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/tactical-controls.md
```

**Structure Decision**: Retain the existing single game crate. Input translation and camera
presentation are Bevy integration concerns already located in `main.rs`; no new domain boundary
is justified.

## Implementation Sequence

1. Replace mappings with arrows and `-`/`=`, update mapping tests, and remove keyboard camera pan
   plus its unused pan constant.
2. Add a small repeat resource/helper. Reset it on release, conflicting pair, or ineligible turn;
   apply immediate/repeated current-player adjustments only while choosing.
3. Add presentation intent and desired pose to the existing camera. Compute an active-player view
   from tank pose/current barrel azimuth and a bounded wide-shot view. Interpolate via render `Time`.
4. Preserve normal action/fixed-update order: camera observes changes but never delays launch,
   resolution, or handoff.
5. Update HUD, README, world/control documentation, and only fulfilled roadmap items.
6. Add focused mapping/repeat/intent/authority-boundary tests; run full checks and quickstart.

## Complexity Tracking

No constitution violations or complexity exceptions require justification.
