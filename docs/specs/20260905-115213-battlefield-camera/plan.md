# Implementation Plan: Minimal 3D Battlefield and Camera

**Branch**: `feature/battlefield-camera` | **Date**: 2026-09-05 | **Spec**: [spec.md](./spec.md)

## Summary

Evolve the Bevy proof into a development-inspectable battlefield: one static, deterministic 40 by
40 unit terrain mesh with visible relief; origin axes; and a bounded target-centred orbit camera.
Keep the work in the existing executable, use no new dependencies, and document only minimum world
conventions.

## Technical Context

**Language/Version**: Rust stable, edition 2024

**Primary Dependencies**: Bevy 0.18.1 (already selected and present)

**Storage**: N/A

**Testing**: Standard Rust unit tests via `cargo test --workspace`; manual desktop verification for
rendering and input behaviour

**Target Platform**: Supported desktop environments capable of running the existing Bevy app;
Linux is currently verified

**Project Type**: Rust Cargo-workspace desktop application

**Performance Goals**: Responsive development-camera interaction and one compact static terrain
mesh needing no runtime mutation after startup

**Constraints**: No new dependencies, crates, external assets, terrain framework, gameplay model,
or generic camera/debug abstractions. Preserve existing workspace quality commands. Camera target
and distance remain bounded to the approximately 40 by 40 unit battlefield.

**Scale/Scope**: One executable crate, one static placeholder battlefield, one development camera,
one lightweight orientation aid, and one world-conventions document

## Constitution Check

| Principle | Plan compliance |
| --- | --- |
| Fun over realism | Relief is deliberately authored for spatial readability, not realism. |
| Simple composable systems | A static mesh and direct camera controls supply only immediate visual context. |
| Code coherence | Keep the feature direct in the existing application; no premature crate/module/trait framework. |
| Purposeful workspace boundaries | No new workspace member has a demonstrated boundary. |
| Deterministic and testable simulation | The deterministic height calculation is unit-tested where valuable; visual/input behaviour is manually verified. |
| Presentation boundary | This is presentation/input work; it establishes minimal world conventions but no simulation abstraction. |
| Playable progress | It makes next visible gameplay slices easier without building infrastructure. |
| Scope and roadmap discipline | No tanks, projectiles, collisions, deformation, or final camera work; only genuinely met roadmap items are checked after implementation. |
| Quality and dependencies | Existing build, test, format, and Clippy checks remain required; no dependency is added. |

**Gate result (pre-design)**: Pass. The small `BattlefieldCamera` state component is justified by
the mutable state required to operate this one camera. It is not a general camera framework.

## Research Summary

See [research.md](./research.md). Use a target-centred orbit camera because it naturally enforces
bounds and provides rotation, pan, and distance. Use a static indexed grid because it makes slope
and height unmistakable with no terrain framework.

## Design

### Battlefield rendering

1. Replace the proof plane and cubes with one static indexed grid covering X/Z from -20 to 20,
   using an authored deterministic height function and smooth normals.
2. Build it once at startup, give it a simple material and lighting, and introduce no mesh
   mutation, collision, height queries, chunking, or asset loading.
3. Draw only origin axes with Bevy gizmos. This scene-local aid does not become reusable debug
   infrastructure.

### Development camera

1. Add one camera-state component holding target, yaw, pitch, distance, and fixed limits.
2. In a per-frame update system, pan target X/Z with WASD or arrow keys; orbit with right-mouse
   drag; and adjust distance with mouse wheel.
3. Clamp target to battlefield X/Z extents, pitch away from vertical, and distance to useful
   near/far limits. Recompute the transform directly; add no interpolation, controller states, or
   projectile tracking.
4. Document controls in README: right-mouse drag to orbit, mouse wheel to zoom, and WASD or arrow
   keys to pan.

### World conventions

Create `docs/world-conventions.md` with: Y is up; `(0, 0, 0)` is battlefield centre; units are
abstract game-space units; and the initial battlefield spans approximately 40 units in each
horizontal direction. State that azimuth, angular semantics, projectile launch placement, and
out-of-bounds rules remain undecided.

### Verification and roadmap

1. Add direct unit tests only for deterministic terrain-height and/or bounds calculations where
   meaningful.
2. Manually run [quickstart.md](./quickstart.md) to prove relief, camera controls, origin axes,
   and normal close behaviour.
3. Run workspace check, test, formatting, and Clippy commands.
4. Update README, `docs/roadmap.md`, and task completion state only after verification passes.

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260905-115213-battlefield-camera/
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
crates/azimuth-game/
├── Cargo.toml                # Unchanged dependency set
└── src/
    └── main.rs               # Scene setup, static terrain, bounded camera, tests

docs/
├── roadmap.md                # Updated only for fulfilled items
├── world-conventions.md      # New minimal shared conventions
└── specs/20260905-115213-battlefield-camera/
```

**Structure Decision**: Keep this bounded feature in `main.rs`. Terrain mesh, camera state, and
scene setup are cohesive parts of the sole executable and do not yet justify separate modules or a
new crate. Reconsider only when real game-domain or reuse boundaries appear.

## Complexity Tracking

No constitution violations require justification.

## Post-Design Constitution Check

Pass. The design uses direct Bevy facilities already in the application, adds no dependency or
future-facing framework, confines itself to rendering/input context, and preserves later decisions
about gameplay terrain and projectile semantics.
