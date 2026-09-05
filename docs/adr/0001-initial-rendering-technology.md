# ADR 0001: Initial Rendering Technology

**Status**: Accepted

## Context

Azimuth needs desktop windowing, real-time 3D rendering, a perspective camera, input, assets, UI,
audio compatibility, mutable mesh support, and debugging facilities. The project prioritises
visible gameplay progress over owning engine infrastructure.

## Decision

Azimuth selects **Bevy 0.18.1** for its initial desktop rendering and application lifecycle. Use
Bevy directly in `azimuth-game`; do not create a renderer abstraction.

## Alternatives Considered

`wgpu` with `winit` would require Azimuth to own rendering pipelines, shaders, depth and resize
handling, camera math, asset/UI/audio integration, debug drawing, and their maintenance. That is
the wrong trade-off for the current project phase.

## Consequences

Bevy has meaningful compile/dependency cost, an ECS learning surface, and regular API changes.
Future simulation code should remain independent where practical. Mutable mesh assets make later
terrain work practical, but deformation, collision, coordinate conventions, and gameplay entities
remain future work.
