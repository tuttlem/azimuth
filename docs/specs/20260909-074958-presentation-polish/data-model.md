# Data Model: Presentation Polish

## Authority Boundary

| Entity | Authority | Purpose |
|---|---|---|
| `MatchConfiguration.players` | Match configuration | Ordered stable player IDs, configured display names, and visual identities. |
| `Tank` collection | Gameplay | Current health, elimination, pose, and ownership. |
| `TurnState` | Gameplay | Current active player, phase, and match completion state. |
| Scoreboard entry/view | Presentation derived from gameplay | Read-only per-player summary for the retained HUD rows. |
| `BattlefieldTerrain` | Gameplay | The sole bounded mutable terrain surface for all terrain queries and deformation. |
| Visual horizon landscape | Presentation derived at scene setup | Immutable lower-detail exterior mesh data; no game-domain query or mutation path. |
| Sky/cloud presentation | Presentation | Clear colour and static visual entities only. |

## Scoreboard Entry

| Field | Source | Rules |
|---|---|---|
| Player ID | Configured player | Stable identity key and UI row tag; never derived from name or index alone. |
| Display name | Configured player | Uses configured value, in configuration order. |
| Visual identity | Configured player/player ID mapping | Retains existing eight-player visual language. |
| Health | Matching tank | Current authoritative health, including zero. |
| Eliminated | Matching tank | True exactly when existing tank elimination says so; row remains present. |
| Active | Turn state | True only when match is in progress and this ID is current; false in resolving or finished states. |

**Validation**:

- One entry exists for each configured player and no other player.
- Entries are iterated exactly in configuration order for every supported count (2–8).
- Every entry resolves a matching tank by stable owner ID; an unmatched configured player is an
  invariant failure rather than a silently fabricated condition.
- Exactly one active entry exists only for an in-progress selectable turn; none exists while
  resolving or finished.

## Scoreboard Lifecycle

```text
Match configuration changed before start
  → accepted match start
  → create one retained HUD row per configured player
  → each HUD sync projects configuration + tanks + turn
  → row text/fill/active styling updates

Tank health reaches zero
  → existing tank elimination
  → same ordered row remains with OUT/eliminated styling

Turn/match phase changes
  → active flag moves or becomes absent; no gameplay write occurs
```

## Visual Horizon Landscape

| Field | Rules |
|---|---|
| Inner perimeter | Samples the current authoritative terrain edge at scene/match initialization so the first exterior ring shares edge elevation and compatible height colour. |
| Outer extent | Extends from the existing ±60 playable square to a named fixed presentation-only distance of roughly ±180–220, adequate for normal views and inside the current far clip. |
| Detail | Uses a small fixed grid/ring substantially lower density than the 64-cell-per-side authoritative terrain. |
| Elevation/colour | Uses the captured terrain seed’s existing height rule outside the welded boundary and the existing elevation palette; far detail may be softened by inexpensive haze. |
| Mutability | Immutable presentation data after construction; never receives craters or tank/support updates. |
| Authority | Has no `height`, bounds, collision, spawn, movement, projectile, or deformation interface. |

## Sky and Clouds

| Entity | Rules |
|---|---|
| Sky clear colour | Static blue scene background visible where no opaque geometry is drawn. |
| Cloud cluster | Small fixed group of simple white/translucent presentation shapes placed high and distant from normal gameplay views. |
| Haze | Optional restrained far-distance visual softening; never alters simulation visibility, collision, terrain query, or controls. |

## Scene Lifecycle

```text
accepted match terrain + player configuration
  ├─ authoritative terrain → current battlefield mesh, terrain queries, collision, craters
  ├─ terrain edge → immutable visual horizon mesh
  ├─ scene setup → sky clear colour + static cloud clusters
  └─ configured players + tanks + turn → read-only scoreboard entries → HUD rows

terrain impact → authoritative crater + current battlefield-mesh refresh only
```

The horizon branch is never an input to the authoritative branch. Its exterior seed use is read-only
visual repetition of existing terrain rules, not a gameplay terrain-generation change.
