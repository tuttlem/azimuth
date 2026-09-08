# Data Model: Expanded Battlefield

## Authority Boundary

| Entity | Authority | Purpose |
|---|---|---|
| Battlefield definition | Gameplay | One map’s dimensions, water table, grid resolution, and captured seed. |
| Battlefield terrain | Gameplay | Mutable vertex heights; the sole source of terrain query, collision, deformation, support, and visible ground shape. |
| Start placement | Gameplay | Deterministic valid tank positions and directions for configured player identities. |
| Elevation colour | Presentation derived from gameplay terrain | Vertex colour calculated from the terrain’s current height; never independently stored as simulation state. |
| Water surface | Presentation | One flat plane at the water table; no physics or collision. |
| World dressing | Presentation | Sparse building records/entities; no authority, collision, damage, or cover. |

## Battlefield Definition

| Field | Rules |
|---|---|
| Horizontal half extent | Fixed at 60 units; valid X/Z are inclusive -60 through +60. |
| Terrain cells per side | Fixed at 64; vertices per side are 65 and total vertices are 4,225. |
| Water-table elevation | Central constant at 0; a dry candidate requires terrain height strictly greater than 0. |
| Captured master seed | Finite `u64` captured once for a match; never changed by rendering or action timing. |
| Derived stream label | Fixed domain label for terrain, starts, dressing, or wind. Identical master seed plus label gives identical sub-seed. |

## Battlefield Terrain

| Field / relationship | Rules |
|---|---|
| Current vertex heights | Finite values generated once from the terrain sub-seed, then mutated only by authoritative craters. |
| Height query | Uses current triangle interpolation, including bounds/corner handling. |
| Macro contribution | Broad bounded mountain, ridge, and bowl/valley terms establish strategic geography. |
| Local contribution | Bounded low-amplitude variation adds irregularity without replacing macro shape. |
| Mesh data | Positions and colours are recreated from the same current heights whenever terrain changes. |
| Crater relationship | Existing crater lowering composes on current heights; it changes future query, collision, support, and derived colour. |

## Spawn Candidate and Placement

| Field | Validation / transition |
|---|---|
| Horizontal position | Must stay within a safe in-bounds margin and have finite terrain query values. |
| Dryness | Height must be strictly above water table. |
| Local slope | Each relevant cardinal one-unit neighbour must be in bounds and differ by no more than the existing 0.75 movement elevation threshold. |
| Separation | Must be at least 18 horizontal units from every accepted start. |
| Distribution | Candidate traversal covers broad map sectors before remaining slots; no tactical fairness score is represented. |
| Orientation | Deterministically faces the map centre or selected opponent-facing direction; does not affect validity. |
| State transition | Candidate → rejected for failed rule, or accepted → grounded `Tank` with position Y from the authoritative height. |

## Elevation Colour

| Region | Visual intent | Rule |
|---|---|---|
| At/below water table | Submerged terrain | Valid bounded ground colour, visually covered by water plane. |
| Lowland | Dark/rich green | Begins immediately above water. |
| Middle | Grass green | Blends through mid-elevation range. |
| Upper | Brown/grey rock | Blends above grass. |
| Peak | Snow-like | Blends at highest elevations. |

The mapping is a pure bounded function of current absolute height. It must interpolate stops continuously and not depend on seed, dressing, or prior mesh state.

## World Dressing Placement

| Field | Rules |
|---|---|
| Placement seed | Derived solely from dressing stream. |
| Footprint, height, orientation | Small deterministic primitive-block variation. |
| Terrain grounding | Base Y derives from authoritative height at its X/Z. |
| Validity | In bounds, dry, locally suitable slope, and outside start exclusion distance. |
| Cardinality | Sparse individual blocks or small groups; zero is valid for a seed with no suitable sites. |
| Authority | Presentation only: excluded from all game-domain collision, movement, damage, terrain, and turn data. |

## Match Construction Flow

```text
captured master seed
  ├─ terrain stream  → generated mutable authoritative terrain
  ├─ start stream    → valid distributed tank starts
  ├─ dressing stream → visual-only building placements after starts
  └─ wind stream     → existing match-constant wind

terrain impact → crater mutates terrain → query/collision/support/mesh-colour update
```

All four branches are independent; only terrain and starts participate in authoritative match state.
