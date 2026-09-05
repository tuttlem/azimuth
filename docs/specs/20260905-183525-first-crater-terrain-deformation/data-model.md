# Data Model: First Crater — Terrain Deformation

## Mutable authoritative terrain

| Value | Meaning | Rule |
| --- | --- | --- |
| bounds | Existing horizontal battlefield extent | Never changes; all grid access remains within it. |
| grid heights | Current elevation at each fixed terrain-grid vertex | Sole mutable terrain data; finite at all times. |
| triangle topology | Existing fixed vertex ordering and triangle indices | Never changes in this feature. |
| local height | Interpolated value on one current grid triangle | Used by mesh positions, tank grounding, and projectile collision. |

The grid starts with the current authored non-flat relief. There is no preserved original-height
copy for gameplay; every crater applies to current heights.

## Crater configuration

| Field | Meaning | Validation |
| --- | --- | --- |
| centre | Existing resolved terrain-impact world position | Must be an in-bounds impact supplied by simulation. |
| radius | Horizontal extent of a crater | Finite and greater than zero. |
| depth | Maximum lowering at the centre | Finite and greater than zero. |

Radius and depth are gameplay deformation values independent of explosion presentation.

## Crater application

For each current grid vertex, calculate horizontal distance `d` from the crater centre. If `d <
radius`, subtract `depth * (1 - d² / radius²)²` from its current elevation. Otherwise retain its
current elevation. Fixed iteration and direct subtraction make impacts deterministic and compositional.

## State transitions

```text
Initial terrain --resolved terrain impact--> Current terrain with crater
Current terrain --later resolved terrain impact--> Further-deformed current terrain
Current terrain with crater --mesh refresh--> Visible surface matching current terrain
Current terrain with crater --later projectile step--> Collision against current terrain
```

Out-of-bounds termination has no impact and cannot enter this sequence. Tanks retain their poses.
