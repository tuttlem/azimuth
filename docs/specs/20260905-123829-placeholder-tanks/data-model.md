# Data Model: Placeholder Tanks and Player Entities

The model has no persistent storage and no external interface. It defines only the two initial
game pieces and the values required to place and render them.

## Player identity

| Field | Meaning | Rules |
| --- | --- | --- |
| identifier | Stable owner identity | Exactly two distinct values exist for this feature. It is independent of colour or render entity. |

## Tank pose

| Field | Meaning | Rules |
| --- | --- | --- |
| horizontal position | Initial X/Z spawn location | Fixed, distinct, and inside the current battlefield bounds. |
| world position | Terrain-resolved X/Y/Z location | Y is produced by the current terrain-height calculation at the horizontal position. |
| body direction | Horizontal direction the body faces | Non-zero and used to orient the visible body. |
| turret direction | Horizontal direction the barrel faces | Non-zero and used for the visible barrel and derived firing origin. |

## Tank

| Field | Meaning | Rules |
| --- | --- | --- |
| owner | Player identity | One initial tank per player. |
| pose | Position and directions | Concrete domain data; rendering derives transforms from it. |
| firing origin | Derived point, not stored state | Above the terrain-resolved position and forward of turret direction. |

## Battlefield placement dependency

| Value | Meaning | Rules |
| --- | --- | --- |
| horizontal bounds | Current visible battlefield extent | Initial tank X/Z values must remain within it. |
| terrain height | Current deterministic visual elevation | Used for mesh construction and initial tank Y placement. |

## Relationships

- Each tank has exactly one player identity and one pose.
- Each pose has one derived firing origin.
- Each initial pose resolves its vertical position through the same terrain-height calculation used
  by the visible battlefield mesh.
- Presentation converts tank pose values into primitive transforms but does not own the domain data.
