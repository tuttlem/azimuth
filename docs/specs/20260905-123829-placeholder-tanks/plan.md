# Implementation Plan: Placeholder Tanks and Player Entities

**Branch**: `feature/placeholder-tanks` | **Date**: 2026-09-05 | **Spec**: [spec.md](./spec.md)

## Summary

Introduce two deterministic player-owned tank pieces on the existing battlefield. Extract the
current battlefield dimensions and height calculation into one small engine-independent module;
add one concrete tank-domain module containing player identity, terrain-resolved pose, direction,
and derived firing origin; then render each tank as simple parented primitives at the presentation
boundary. No dependency, new crate, projectile, aiming control, collision, movement, or turn state
is added.

## Technical Context

**Language/Version**: Rust stable, edition 2024

**Primary Dependencies**: Bevy 0.18.1; no additions

**Storage**: N/A

**Testing**: Standard Rust unit tests via `cargo test --workspace`; manual desktop inspection using
the existing camera

**Target Platform**: Existing supported desktop platforms; Linux/Vulkan is currently verified

**Project Type**: Rust Cargo-workspace desktop application

**Performance Goals**: Two static primitive-composed tanks with no runtime simulation or asset
loading cost beyond the existing scene

**Constraints**: Preserve direct Bevy use at the presentation boundary. Domain types must not use
engine types. No generic entity/terrain/weapon framework, external assets, random spawning, or
camera behaviour changes.

**Scale/Scope**: Two fixed initial players and tanks; one limited terrain-height query; one derived
firing origin per tank; one simple parent/child presentation hierarchy per tank

## Constitution Check

| Principle | Plan compliance |
| --- | --- |
| Fun over realism | Tanks are readable stylised placeholders, not vehicle simulations. |
| Simple systems | Fixed player poses and a derived firing origin are sufficient for the next projectile slice. |
| Code coherence | Two small focused modules separate concrete domain facts from rendering without a framework. |
| Purposeful workspace boundaries | Internal modules provide an immediate readability/testability boundary; no crate is justified. |
| Determinism and testability | Spawn placement, terrain elevation, bounds, and firing origins are pure deterministic domain calculations. |
| Presentation boundary | Bevy types appear only when domain positions/directions become rendered primitives. |
| Playable progress | The scene obtains recognisable opposing pieces while deferring all gameplay mechanics. |
| Scope discipline | No aiming, launch, projectile, damage, movement, turns, weapons, AI, or player configuration. |
| Quality and dependencies | No dependency added; normal workspace checks and visual inspection remain completion gates. |

**Gate result (pre-design)**: Pass. `battlefield.rs` and `tank.rs` are focused, concrete modules
with demonstrated current responsibilities; they are not generic subsystems or an abstraction layer.

## Research Summary

See [research.md](./research.md). Use Bevy parent-child transforms for the visible body, turret,
and barrel while deriving firing origins from tank-domain data rather than renderer state. This keeps
the future projectile input explicit and prevents a firing reference from becoming stale.

## Design

### Domain and terrain placement

1. Move the existing battlefield half extent, terrain-height calculation, and bounds check into
   `battlefield.rs`. Keep this limited to the current deterministic visual terrain, not a generic
   terrain API.
2. Add `tank.rs` with explicit concrete player identity, horizontal position/direction, terrain
   resolved world position, tank pose, and tank types using ordinary numeric fields rather than
   presentation types.
3. Define exactly two fixed initial tanks at separated opposing spawn locations. Resolve each Y
   position through the current terrain-height calculation and validate horizontal bounds.
4. Derive each firing origin from its current pose: a small positive vertical offset plus an offset
   along turret direction. Do not store a redundant firing-origin value, launch anything, or define
   azimuth semantics.

### Placeholder presentation

1. Have `main.rs` request the two initial tanks and convert their domain values to Bevy transforms
   only while spawning the scene.
2. Spawn one parent entity per tank at its terrain-resolved world position and body direction,
   explicitly retaining inherited visibility. Add child primitive geometry for a body, turret, and
   forward barrel using local transforms.
3. Reuse meshes and assign one simple distinct material colour per player. Keep tank body upright;
   terrain contact comes from the resolved base elevation rather than slope-alignment machinery.
4. If needed for immediate inspection, draw a small firing-origin marker or direction line with
   the existing local gizmo facility. Do not create a reusable diagnostics layer.

### Verification and documentation

1. Add focused unit tests for fixed spawn uniqueness and bounds, terrain-resolved height, and
   derived firing origin being above and ahead of turret direction.
2. Retain and adapt existing battlefield tests after their focused refactor.
3. Update README status only if required to remain accurate; the existing camera controls and world
   conventions remain valid.
4. Manually verify both distinct tanks, terrain contact, orientation, firing reference, existing
   camera, origin axes, and clean close using [quickstart.md](./quickstart.md).
5. Run workspace check, tests, format check, and Clippy. Update only satisfied tank/player and
   near-term roadmap items after those checks and manual verification pass.

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260905-123829-placeholder-tanks/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── checklists/
│   └── requirements.md
└── tasks.md                    # Created by $speckit-tasks
```

### Source Code (repository root)

```text
crates/azimuth-game/src/
├── main.rs                     # Bevy scene, camera, terrain mesh, and tank presentation
├── battlefield.rs              # Current terrain bounds and deterministic height query
└── tank.rs                     # Concrete player/tank pose and derived firing origin

docs/
├── roadmap.md                  # Updated only for fulfilled tank/player items
└── world-conventions.md         # Remains the source for limited current world conventions
```

**Structure Decision**: Extract only the current domain facts from `main.rs`. `battlefield.rs`
owns the already-existing terrain facts and `tank.rs` owns the first concrete game-piece state;
`main.rs` continues to own direct Bevy presentation and input. No new crate, trait, or interface is
introduced.

## Complexity Tracking

No constitution violations require justification.

## Post-Design Constitution Check

Pass. The design has a direct, demonstrable domain/presentation boundary, stays deterministic and
testable, and adds only the concrete state immediately needed for two visible tanks and the next
projectile specification.
