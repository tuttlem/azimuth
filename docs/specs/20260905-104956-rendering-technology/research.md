# Research: Rendering Technology Selection

## Decision: Select Bevy 0.18.1

**Rationale**: Bevy is the smallest integrated Rust-native choice that covers Azimuth's immediate
desktop lifecycle, 3D rendering, perspective camera, input, assets, UI, audio compatibility,
runtime mesh work, and debug-rendering needs. It allows the project to reach a visible 3D proof
without owning the graphics and application infrastructure itself. The proof can use Bevy directly
in `azimuth-game`; later domain simulation can remain plain Rust until a real integration boundary
is needed.

Bevy's current 0.18.1 release supports Rust 1.89.0 or newer, below the project's current stable
toolchain. Its standard plugins provide desktop window/event-loop ownership; its 3D examples use
perspective cameras, primitive meshes, materials, and lights without external assets. Mesh assets
can be modified through the main-world mesh asset collection, making generated and replaced terrain
geometry practical for later scoped work. Terrain deformation remains future work because mesh
updates will still need deliberate normal, index, bounds, collision, and update-cost design.

**Trade-offs**: Bevy has substantial dependency and compile costs, an ECS learning surface, and
regular breaking releases. It must not dictate the future game-domain model or become a reason to
invent gameplay components now. These costs are justified by avoiding greater application-owned
infrastructure during the project's most important visible-progress phase.

**Primary references**:

- [Bevy 0.18.1 release](https://github.com/bevyengine/bevy/releases/tag/v0.18.1)
- [Bevy package metadata and features](https://raw.githubusercontent.com/bevyengine/bevy/v0.18.1/Cargo.toml)
- [Bevy 3D scene example](https://github.com/bevyengine/bevy/blob/main/examples/3d/3d_scene.rs)
- [Bevy window integration](https://github.com/bevyengine/bevy/blob/main/crates/bevy_winit/src/lib.rs)
- [Mutable mesh example](https://github.com/bevyengine/bevy/blob/main/examples/asset/alter_mesh.rs)
- [Bevy Linux dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)

## Alternative considered: `wgpu` plus `winit`

**Rationale**: This is a credible lower-level Rust stack. `winit` supplies cross-platform desktop
windows, lifecycle, close/resize/redraw, and keyboard/mouse events; `wgpu` supplies a portable GPU
API and can support perspective scenes, dynamic geometry, and runtime vertex/index uploads.

**Why not selected now**: `winit` does not render. Combining it with `wgpu` would require Azimuth to
own surface and resize handling, depth setup, camera/projection math, buffers, shaders, render
pipelines, materials, resource lifecycles, asset loading, UI integration, audio integration,
particles, debug drawing, and ongoing graphics maintenance. It offers lower framework convention
but a much higher amount of non-game work, conflicting with playable progress over infrastructure.

**Reconsider when**: Revisit a lower-level renderer only if a concrete later requirement is blocked
by Bevy or if the project deliberately changes its goals to own rendering-engine technology.

**Primary references**:

- [winit documentation](https://docs.rs/winit/latest/winit/)
- [wgpu documentation](https://wgpu.rs/doc/wgpu/)
- [wgpu queue uploads](https://wgpu.rs/doc/wgpu/struct.Queue.html)
- [wgpu best practices](https://wgpu.rs/doc/src/wgpu/documentation/best_practices/dos_and_donts.rs.html)

## Capability comparison

| Need | Bevy | `wgpu` + `winit` |
|---|---|---|
| Window, lifecycle, close, input | Integrated desktop plugins | `winit` supplies it; application owns integration |
| Perspective 3D, meshes, lights | Direct built-in scene facilities | Application builds pipeline, depth, camera, shaders, and meshes |
| UI, assets, audio compatibility | Integrated subsystems and examples | Compose and maintain separate integrations |
| Dynamic terrain mesh updates | Mutable mesh assets; later deformation remains deliberate | Possible via buffer uploads; topology/buffer management is application work |
| Effects and debug drawing | Existing facilities and examples | Application owns buffers, shaders, pipelines, and utilities |
| Domain/presentation boundary | Keep direct presentation code in application crate | Naturally direct, but with much more infrastructure |
| Complexity cost | Engine/ECS/dependency/compile cost | Less engine convention; substantially more application boilerplate and maintenance |

## Decision: Do not survey more candidates

**Rationale**: Bevy and the lower-level stack provide the meaningful decision boundary: an integrated
engine versus ownership of a graphics/application stack. A broader survey would not improve this
feature's decision enough to justify delaying the first visible proof.
