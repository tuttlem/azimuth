# Implementation Plan: First Projectile and Deterministic Ballistic Arc

**Branch**: `feature/projectile-ballistics` | **Date**: 2026-09-05 | **Spec**: [spec.md](./spec.md)

## Summary

Add Azimuth's first deterministic ballistic flight without adding collision or gameplay systems.
Extract the already shared world position into a small engine-independent world-value module; add a
pure projectile module that validates a shot, derives its launch vector, and advances it with a
1/120-second constant-acceleration update. The existing application will launch one fixed Player
One shot on Space, advance only the authoritative domain state in Bevy's fixed schedule, and copy
that state to one simple rendered sphere. Documentation will establish the angular and lifetime
conventions; focused unit tests will protect the mathematical model.

## Technical Context

**Language/Version**: Rust stable, edition 2024

**Primary Dependencies**: Bevy 0.18.1; no additions

**Storage**: N/A

**Testing**: Standard Rust unit tests via `cargo test --workspace`; manual desktop validation using
the existing development camera

**Target Platform**: Existing supported desktop platforms; Linux/Vulkan is currently verified

**Project Type**: Rust Cargo-workspace desktop application

**Performance Goals**: One projectile advances at 120 fixed simulation steps per simulated second
and remains visually smooth in the existing development scene

**Constraints**: Domain physics must contain no Bevy types; rendering must only observe that state.
Use the specified 1/120-second kinematic update, one active projectile, no collision, no terrain
queries, no new crate, and no new dependency.

**Scale/Scope**: One fixed development shot from Player One, one gravity value, one visible sphere,
one generous flight-volume/lifetime rule, and focused deterministic domain tests

## Constitution Check

| Principle | Plan compliance |
| --- | --- |
| Fun over realism | Uses a simple configurable constant gravity value selected for a readable arc, not an Earth simulation. |
| Simple systems | Position, velocity, gravity, and fixed steps are the only simulation concepts; no collision, weapons, or environmental model is introduced. |
| Code coherence | A small world-value module and a small pure projectile module have immediate shared-domain and testability purposes. |
| Purposeful workspace boundaries | The existing application crate remains sufficient; internal modules, not new crates, are the narrowest useful boundaries. |
| Determinism and testability | Fixed-step kinematics and parameter validation are pure domain behaviour with focused unit coverage. |
| Presentation boundary | Bevy handles Space input, fixed-schedule invocation, and the sphere transform; it does not calculate trajectory state. |
| Playable progress | Space produces the first visible artillery arc while avoiding infrastructure and unfinished gameplay systems. |
| Scope discipline | Terrain impact, aiming, UI, turns, weapons, effects, drag, wind, and multiple shots remain explicitly absent. |
| Quality and dependencies | No dependency is added; workspace validation, manual verification, documentation, and accurate roadmap updates are completion gates. |

**Gate result (pre-design)**: Pass. The extra world-value module is justified by a concrete current
need: the tank firing origin and projectile state now share a renderer-independent position type.

## Research Summary

See [research.md](./research.md). Bevy's fixed schedule can run zero or many times between visual
updates, so launch input belongs in `Update` while only advancement belongs in `FixedUpdate`. The
existing primitives and transform patterns are sufficient for a single tagged projectile sphere.

## Design

### Domain model and conventions

1. Add a focused `world.rs` module containing the existing three-scalar world position and a
   three-scalar world vector. Move the tank's world-position definition there; keep tank-specific
   horizontal position and direction in `tank.rs`. Do not add vector trait machinery or a math
   crate.
2. Add a pure `projectile.rs` module. It owns shot parameters, gravity, projectile position and
   velocity, elapsed flight time, simulation limits, fixed-step constant, validation, angle
   conversion, tank-direction-to-azimuth conversion, and one fixed-step advancement operation.
3. Validate finite values, elevation in 0 through 90 degrees, positive speed, non-negative gravity,
   and a non-zero finite tank horizontal direction. Normalize any finite azimuth to 0 through less
   than 360 degrees.
4. Use the spec's X/Z and angular convention directly: 0 degrees points toward negative Z;
   positive angles rotate clockwise from above; elevation rises from level. Derive a unit direction
   with the specified sine/cosine formula, then scale it by launch speed.
5. Use the mandated exact constant-acceleration fixed-step update. Increment elapsed flight time
   each step and end a flight only when it exceeds X/Z 60, Y -30/100, or 20 simulated seconds. Do
   not ask the terrain about a projectile or encode impact behaviour.

### Application integration and presentation

1. Configure Bevy's fixed time at 120 Hz once during application setup. Keep camera input on the
   normal update schedule; fixed ticks may occur zero, one, or many times per rendered frame.
2. Store at most one `Option<Projectile>` in a small application resource. This resource is only a
   container for the pure domain object, not a second physics representation.
3. In `Update`, handle `Space` as an edge-triggered development launch request. If no projectile is
   active, derive Player One's azimuth from its turret direction, use its existing firing origin,
   a 45-degree elevation, speed 18 abstract units per second, and default gravity 8 abstract units
   per second squared. If a shot is active, ignore the request.
4. In `FixedUpdate`, advance the active projectile through the pure domain operation and remove it
   from the resource once it reaches the documented termination rule. Never use variable render
   delta for flight motion.
5. On launch, spawn one bright primitive sphere tagged solely as the projectile visual. In normal
   updates, copy the authoritative position into its transform; when no active state remains,
   despawn that visual. The visual starts at the firing origin and owns no trajectory calculation.
6. Retain the battlefield, tanks, origin axes, and existing camera controls unchanged. Do not add a
   trace unless it is required during implementation to make a specific behaviour understandable;
   any such aid must remain local and non-authoritative.

### Verification and documentation

1. Add pure domain tests for cardinal azimuths, level and vertical elevation, speed scaling,
   tank-direction azimuth, exact repeated trajectories, zero gravity, horizontal invariance,
   vertical acceleration, apex/descent, stronger gravity, lifetime limits, and terrain-independent
   termination behaviour.
2. Keep Bevy testing to the existing small camera-bound test; do not introduce GPU, screenshot, or
   renderer-internal tests.
3. Expand `docs/world-conventions.md` with the X/Z, azimuth, elevation, firing-origin, and
   simulation-volume definitions. Add concise `docs/projectile-model.md` documenting the fixed
   timestep, constant-gravity equations, default development values, and intentional lack of
   terrain impact.
4. Update README status and controls to describe the Space development launch and link to the
   projectile model. Update only demonstrated coordinate, projectile, gravity, rendering, and
   Milestone B roadmap checkboxes after all checks and manual verification pass.
5. Follow [quickstart.md](./quickstart.md) for full workspace validation, visual launch testing,
   gravity variation verification, and scope review.

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260905-130239-projectile-ballistics/
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
├── main.rs                     # Bevy scene, input, fixed scheduling, and projectile visual sync
├── battlefield.rs              # Existing terrain bounds and visual height query
├── tank.rs                     # Existing player/tank pose and derived firing origin
├── world.rs                    # Shared engine-independent world position and vector values
└── projectile.rs               # Pure launch, gravity, fixed-step flight, limits, and tests

docs/
├── roadmap.md                  # Updated only for fulfilled projectile-slice items
├── world-conventions.md         # Extended coordinate and angular agreement
└── projectile-model.md          # Concise deterministic ballistic model reference
```

**Structure Decision**: Keep Bevy direct in `main.rs`. `world.rs` earns its existence as the
concrete shared coordinate value used by the existing tank and new projectile; `projectile.rs`
earns its existence as independently testable gameplay physics. Neither is a crate, framework, or
engine abstraction.

## Complexity Tracking

No constitution violations require justification.

## Post-Design Constitution Check

Pass. The design adds no dependencies, architecture layer, generic hierarchy, or simulation system
beyond one concrete deterministic flight. It protects a direct simulation/presentation boundary,
keeps the visible vertical slice small, and intentionally defers impact and aiming work.
