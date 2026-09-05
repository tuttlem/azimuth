# Research: Minimal 3D Battlefield and Camera

## Decision: Use a bounded target-centred orbit camera

Use one small camera-state component for the single development camera. Hold right mouse and drag to orbit; use the scroll wheel to change distance; use WASD or arrow keys to pan the orbit target over world X/Z. Clamp target, pitch, and distance to useful battlefield-centred ranges.

**Rationale**: A target-centred orbit keeps the scene understandable, makes every requested inspection operation available, and gives direct places to enforce the initial battlefield bounds. Bevy's camera-orbit example uses accumulated mouse motion for yaw/pitch, clamps pitch away from vertical, and positions the camera relative to its target. Its input APIs support held keyboard and mouse state, while wheel messages provide simple distance adjustment. The controller belongs in a normal per-frame update rather than a fixed simulation schedule.

**Alternatives considered**:

- Bevy's first-party free camera controller: suitable for free scene inspection, but it has no natural target-centred/bounded battlefield semantics and would make the requested constraints less explicit.
- A cinematic, stateful, interpolated, or projectile-following camera: rejected because no current feature requires it.
- A third-party camera-controller dependency: rejected because direct input handling is small, clear, and uses facilities already supplied by Bevy.

**Sources**:

- [Bevy camera orbit example](https://bevy.org/examples/camera/camera-orbit/)
- [Bevy free-camera release notes](https://bevy.org/news/bevy-0-18/#first-party-camera-controllers)
- [Bevy free-camera API](https://docs.rs/bevy/0.18.0/bevy/camera_controller/free_camera/struct.FreeCamera.html)
- [Bevy wheel-input example](https://bevy.org/examples-webgpu/3d-rendering/visibility-range/)
- [Bevy fixed-timestep guidance](https://bevy.org/examples/movement/physics-in-fixed-timestep/)

## Decision: Use one static generated indexed grid mesh

Create one compact, deterministic indexed grid mesh at startup, covering the documented 40 by 40 unit battlefield. Give vertices modest authored height variation and calculate smooth normals for lighting. The mesh remains static for this feature.

**Rationale**: A small grid plainly shows slopes and relief while avoiding a terrain API, procedural-generation system, asset pipeline, or runtime mesh mutation. It stays understandable and leaves a clean future decision point for gameplay terrain and deformation.

**Alternatives considered**:

- A flat plane plus stacked primitives: rejected because it communicates height less clearly and does not provide an obvious continuous ground surface for later work.
- A reusable/procedural terrain generator: rejected because this feature only needs one readable placeholder landscape.
- Runtime-mutable terrain assets: rejected because deformation is explicitly out of scope.

**Source**: [Bevy 0.18.1 generated custom mesh example](https://raw.githubusercontent.com/bevyengine/bevy/v0.18.1/examples/3d/generate_custom_mesh.rs)

## Decision: Use only origin axes as debug visualisation

Draw Bevy's built-in axes gizmo at the origin during scene updates.

**Rationale**: The axes immediately establish origin and orientation without creating geometry, state, assets, or a debug-rendering framework. The terrain mesh already makes the battlefield extent visually apparent.

**Alternative considered**: Boundary grids and wireframes are useful only if a concrete inspection problem appears; a general debug visualisation layer is premature infrastructure.

**Source**: [Bevy 0.18.1 axes gizmo example](https://raw.githubusercontent.com/bevyengine/bevy/v0.18.1/examples/gizmos/axes.rs)

## Decision: Document only minimal conventions

Record Y-up, origin-centred placement, abstract units, and the initial 40 by 40 unit horizontal bounds in `docs/world-conventions.md`. Explicitly defer azimuth, elevation-angle, launch-vector, spawn-position, and out-of-bounds conventions.

**Rationale**: These facts are required for a coherent visible world and bounded camera now; the deferred items affect gameplay and must be chosen with the feature that needs them.
