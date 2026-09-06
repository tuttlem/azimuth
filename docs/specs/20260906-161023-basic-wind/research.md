# Research: Basic Wind — First Environmental Gameplay

## Decision: represent wind as validated horizontal acceleration

**Rationale**: The projectile already uses one explicit constant acceleration for gravity. A finite
horizontal world vector with zero Y component is the smallest direct extension: it is tuneable,
deterministic, and means exactly “the direction wind pushes a projectile toward.” The default
`(+1.5, 0, 0)` gives about 3 units of +X drift after two seconds and 6.75 after three seconds,
which is visible against the current battlefield without exceeding gravity's 8 units/s².

**Alternatives considered**:

- Drag toward a wind velocity: needs new coefficient/tuning semantics and obscures first-use
  compensation.
- Force, mass, or atmosphere model: has no first-wind gameplay benefit.
- Screen-relative wind: changes meaning with the presentation camera and is not authoritative.

## Decision: extend the sole fixed projectile advance path

**Rationale**: Add wind acceleration to gravity before the existing constant-acceleration position
and velocity update. The existing swept segment and deterministic bisection collision then consume
the wind-altered candidate directly. Zero wind exactly reproduces the existing gravity-only vector.

**Alternatives considered**:

- A post-step sideways offset: would not update velocity correctly and creates an inconsistent
  collision path.
- A second projectile mode: risks behavioral divergence and duplicate tests.

## Decision: use one default match resource, separate from gravity

**Rationale**: The application already owns one `BattlefieldGravity` resource. A parallel compact
`BattlefieldWind` resource makes the constant condition explicit and passes it only to projectile
simulation. Keeping it separate preserves the player-facing distinction and prevents wind from
affecting tank settling, which deliberately consumes gravity only.

**Alternatives considered**:

- Store wind on each projectile: contradicts the one shared battlefield condition and complicates
  current launch state without supporting changing wind.
- General environment/force resource: anticipates presets and systems the game does not need.

## Decision: display camera-independent “toward” wording

**Rationale**: The existing world conventions define X/Z and azimuth but no permanent screen
right. The ordinary HUD line `Wind: toward +X, 1.5 units/s²` directly tells both players what the
same authoritative vector means. It remains visible in choosing, movement, and resolving-fire
states, without displaying a prediction or modifying aim.

**Alternatives considered**:

- A world-space visual/particle/flag: presentation cost without better first-use clarity.
- A compass label without axis definition: less precise than the documented coordinate convention.

## Decision: keep vertical wind and balance conclusions deferred

**Rationale**: Wind Y would overlap gravity and complicate trajectory intuition. Constant +X at
1.5 units/s² is a visible starting condition, not proof of final strength ranges or compensation
quality. Manual windy duels determine whether those evaluation roadmap items are earned.

**Alternatives considered**:

- Complete vertical-wind decision now: no play evidence requires it.
- Mark balance work complete from a numeric default: would be dishonest roadmap progress.
