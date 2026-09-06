# Data Model: Tactical Controls and Camera Flow

## Existing authoritative entities (read/reused only)

| Entity | Relevant fields | Feature role |
|---|---|---|
| `TurnState` | `current_player`, `phase`, retained aims | Authorizes aim in `Choosing`; camera reads active player only. |
| `AimingState` | azimuth, elevation, launch speed | Receives existing bounded fine/coarse `AimAdjustment`s. |
| `Tanks` / `Tank` | owner, position | Supplies active-player camera framing; remains movement authority. |
| `ProjectileFlight` | `Option<Projectile>` | Selects wider shot presentation. |

No camera/repeat field may be read by projectile, terrain, movement, or turn resolution.

## Tactical input repeat state

| Field | Meaning | Lifecycle |
|---|---|---|
| logical adjustment/key | One of six fixed mapped adjustments | Left/Right, Up/Down, `-`/`=`. |
| held/repeat elapsed time | Time since an eligible key began holding | Reset on release, opposing-key conflict, or phase other than `Choosing`. |
| whole repeat intervals | 100 ms intervals elapsed after first 300 ms | Each is applied through existing bounded aim API. |

1. Initial eligible press applies one adjustment and starts delay.
2. Eligible hold applies each elapsed repeat interval, using current Shift coarse state.
3. Release/conflict/movement/resolving/flight clears repeat state.

## Camera presentation state

| Field | Meaning | Lifecycle |
|---|---|---|
| `CameraPresentationIntent` | `ActivePlayer(PlayerId)` or `WatchingShot` | Derived from read-only turn, tanks, and flight state. |
| desired camera pose | target, yaw, pitch, distance | Recomputed/replaced when intent changes; active yaw follows retained barrel azimuth while preserving manual orbit offset. |
| current camera pose | existing `BattlefieldCamera` values | Interpolates to desired pose using render time; mouse orbit/zoom remain local. |

1. Startup/new choosing turn selects `ActivePlayer(current_player)`.
2. Projectile launch selects `WatchingShot` without waiting.
3. Flight conclusion plus normal turn handoff replaces it with `ActivePlayer(new_player)`.
4. No transition is queued and no gameplay code observes transition completion.

## Relationships

```text
ButtonInput + render Time -> repeat tracker -> TurnState::apply_current_aim
TurnState + Tanks + ProjectileFlight --read only--> camera intent -> desired pose
desired pose + render Time -> BattlefieldCamera / Camera3d Transform

TurnState + ProjectileFlight -> FixedUpdate projectile/terrain resolution
             (no camera or repeat dependency)
```
