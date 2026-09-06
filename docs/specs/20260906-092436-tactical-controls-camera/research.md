# Research: Tactical Controls and Camera Flow

## Decision: arrows plus `-`/`=` with a local six-key repeat tracker

**Rationale**: Arrows make the azimuth/elevation mapping obvious, and adjacent `-`/`=` provides a
reachable power pair without colliding with movement or fire. Bevy exposes pressed/released state
but no feature-specific repeat cadence. A six-entry local tracker is smaller than configurable
input infrastructure: it applies once immediately, waits 300 ms, then accumulates one adjustment
per 100 ms interval. Shift is sampled for each adjustment and preserves current coarse values.

**Alternatives considered**:

- Operating-system repeat: platform configuration cannot guarantee the requested timing.
- One adjustment every render frame: materially frame-rate dependent.
- Generic binding/repeat framework: it has no second consumer or configuration need.

## Decision: opposing keys for one axis produce no adjustment and reset that pair's repeat state

**Rationale**: Both direction inputs are ambiguous. Suspending the pair prevents an old held key
from resuming with a delayed surprise after conflict resolution. Separate axes remain independent.

**Alternatives considered**:

- Arbitrary priority: invisible and accident-prone.
- Apply both: needless bounded state churn for a net-zero result.

## Decision: repeat timing remains outside fixed simulation

**Rationale**: It only translates eligible Update-time player input into the existing bounded
`TurnState::apply_current_aim`. It is cleared outside `Choosing` and has no path to terrain,
movement, projectile, or resolution methods.

**Alternatives considered**:

- FixedUpdate repeat: falsely couples input comfort to ballistic cadence.

## Decision: extend `BattlefieldCamera` with presentation intent and desired pose

**Rationale**: The existing controller owns target, yaw, pitch, and distance. `ActivePlayer` or
`WatchingShot` intent plus a desired bounded pose directly replaces stale transitions; current
pose smoothly interpolates. No queue, director, or parallel state machine is needed.

**Alternatives considered**:

- Timeline/event queue: requires cancellation and creates an unnecessary presentation framework.
- Direct snaps: fails readable handoffs in normal use.
- Full projectile follow: deferred and potentially disorienting.

## Decision: active view follows barrel azimuth; shot view is a bounded battlefield framing

**Rationale**: The player-facing view stays aligned with the active barrel's horizontal firing
direction instead of a stale tank body direction left by movement. The camera retains any mouse
orbit offset while applying aim-yaw changes smoothly. A fixed wide view keeps arc/terrain/impact
legible without per-projectile tracking. Constants remain direct and tuneable.

**Alternatives considered**:

- Tank-body view: can reveal stale movement facing rather than the current player's firing frame.
- Obstruction-aware camera navigation: explicitly out of scope.

## Decision: retain right-mouse orbit/wheel zoom, remove keyboard panning

**Rationale**: This frees arrows for tactical control while retaining non-conflicting useful
inspection controls. These operations alter presentation only.

## Decision: test helpers/state reactions rather than pixels

**Rationale**: Key mapping, repeat cadence, intent selection, target bounds, and independent
turn resolution are stable behavior. Pixel or exact-coordinate tests are brittle and do not prove
the presentation-authority boundary.
