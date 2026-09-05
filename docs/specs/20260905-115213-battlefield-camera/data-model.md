# Data Model: Minimal 3D Battlefield and Camera

This feature has no persistent data model or external interface. These are the small runtime concepts and documented values needed by the executable.

## Battlefield

| Field | Meaning | Rules |
| --- | --- | --- |
| centre | Shared world-space reference point | Fixed at `(0, 0, 0)` for this scene. |
| horizontal extent | Visible X/Z play area | Approximately 40 units by 40 units, centred on origin. |
| height function | Deterministic visual elevation at a horizontal location | Used only to create the static mesh; not gameplay state. |
| mesh | Rendered ground surface | Built once at startup; no mutation, collision, or terrain queries. |

## Development camera

| Field | Meaning | Validation / transition |
| --- | --- | --- |
| target | Point the camera orbits and pans around | Starts near origin; X/Z remains within configured inspection area. |
| yaw | Horizontal orbit orientation | Changes only from documented orbit input; no roll. |
| pitch | Vertical orbit orientation | Changes only from documented input and remains short of vertical singularities. |
| distance | Target-to-camera distance | Changes from scroll input and remains within configured near/far range. |
| transform | Rendered camera pose | Recomputed from target, yaw, pitch, and distance after input. |

## World conventions

| Convention | Current meaning | Explicitly not decided |
| --- | --- | --- |
| vertical | Y is elevation; positive Y is up | Gravity magnitude or direction as gameplay behaviour. |
| origin | `(0, 0, 0)` is battlefield centre | Projectile spawn/launch position. |
| unit | One abstract game-space unit | Physical/metre equivalence. |
| bounds | Initial horizontal battlefield is approximately 40 by 40 units | Projectile out-of-bounds outcome. |
| angles | Not established here | Azimuth zero/direction, elevation-angle convention, and launch-vector conversion. |

## Relationships

- The camera target is constrained by the battlefield's horizontal extent.
- The terrain mesh is positioned according to the documented origin and extent.
- The origin axes visualise the same convention; they do not own or alter it.
