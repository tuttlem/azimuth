# Research: Heavy Shell Wind Resistance

## Decision: Model wind resistance as a validated projectile wind-response value

**Rationale**: The current wind model is an explicit constant horizontal acceleration, not air
velocity, drag, or mass. A finite non-negative scalar that multiplies that existing acceleration
expresses the requested tactical difference directly. Normal shells use `1.0`; Heavy Shell begins
at `0.40`, which yields a clear proportional comparison while remaining affected by strong wind.

**Alternatives considered**:

- Projectile mass: rejected because current wind is not a force model and the roadmap reserves
  mass for gameplay that genuinely needs mass semantics.
- Drag/aerodynamics: rejected as unrelated atmosphere work and a larger simulation change.
- A Heavy-Shell conditional in flight: rejected because it couples common physics to weapon
  identity and violates the feature's architectural acceptance.
- Immunity (`0.0`): rejected for Heavy Shell because players should still compensate for wind.

## Decision: Capture wind response in `Projectile` at the fire commitment boundary

**Rationale**: `FiredShot` already owns an immutable `Projectile` and copied impact profile while
the resolver deliberately avoids mutable selection/inventory/catalogue reads. Placing the response
on the projectile makes common `advance_with_terrain` consume its own authoritative flight state
and preserves the existing snapshot invariant.

**Alternatives considered**:

- Read the selected weapon during each step: rejected because selection can change and is not
  flight authority.
- Read the catalogue during each step: rejected because catalogue data is definition-time data,
  not necessary runtime dependence.
- Add a second profile field beside `Projectile` on `FiredShot`: rejected because projectile
  simulation should not need an external weapon wrapper to know its flight characteristic.

## Decision: Retain the current explicit conventional loadout representation

**Rationale**: The existing loadout has two named availability entries and clear matching logic.
Adding one third entry is a small, local extension; a map, dynamic registry, data files, or a
universal inventory abstraction would be speculative for three fixed weapons.

**Alternatives considered**:

- Dynamic map or external catalogue: rejected because it introduces unnecessary generality,
  configuration, and new failure modes.
- Sentinel ammunition count for Basic Shell: rejected because explicit unlimited availability is
  an established invariant.

## Decision: Use the established normal direct-selector pattern for Heavy Shell

**Rationale**: The present selection input maps `1` and `2` to the first two conventional weapons
and HUD controls describe those choices. Extending that same surface to the third weapon satisfies
the feature without a Heavy-Shell mode or a HUD redesign.

**Alternatives considered**:

- A Heavy-Shell-only key/mode: rejected because it would be a special control path.
- Automatic wind-based selection: rejected because it removes the intended player decision.

## Decision: Keep Heavy Shell's initial impact equal to Basic Shell

**Rationale**: Basic Shell is already the ordinary 6-unit / 40-damage / 4.0-radius, 1.8-depth
crater baseline. Reusing it isolates wind resistance as Heavy Shell's learnable role, while
High Explosive retains its larger blast and terrain role.

**Alternatives considered**:

- A larger Heavy blast or crater: rejected because it would blur the High Explosive role.
- A special direct-hit, penetration, or terrain rule: rejected as explicitly future work.

## Decision: Prove behaviour with differential deterministic tests plus manual matches

**Rationale**: A same-launch comparison isolates wind response more reliably than an absolute
coordinate. Four direction cases, zero-wind equivalence, stronger-versus-weaker wind, unchanged
gravity, and repeat traces prove the generic physics. Manual matches establish whether the effect
is readable and tactically useful rather than merely numeric.

**Alternatives considered**:

- Pixel/camera assertions: rejected because presentation is not authoritative gameplay.
- Only one absolute impact coordinate: rejected because it does not establish proportional or
  direction-independent response.
