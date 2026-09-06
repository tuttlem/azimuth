# Research: Tank Support, Gravity and Terrain Settling

## Decision: use the existing tank base as one support point

**Rationale**: `TankPose.position` is already the authoritative tank base. Both deliberate
movement and firing derive their behavior from it, and the current tank placeholder exposes no
footprint or terrain-normal orientation. Comparing that point to the current terrain height is the
smallest general rule that handles direct, edge, and overlapping craters without crater-specific
logic.

**Alternatives considered**:

- Multi-point track/wheel footprint: no current representation or observed gameplay need; it adds
  contact rules before the game has use for them.
- Move every tank after any blast: incorrectly makes proximity to an explosion, rather than loss
  of support, authoritative.
- Separate movement terrain: would diverge from the terrain mesh, projectile collision, and
  existing movement-height query.

## Decision: store only supported/falling state and vertical velocity on `Tank`

**Rationale**: A `Supported` or `Falling { vertical_velocity }` state is enough to explain and
test the only new transient behavior. It preserves the existing pose as the visible and gameplay
source of truth, while avoiding a general body, velocity-vector, or physics-component system.

**Alternatives considered**:

- A global physics resource: separates a tank's own authoritative motion from its identity and
  complicates two-tank lookup.
- Full 3D velocity, forces, torque, or contacts: not needed for vertical settling.

## Decision: use current terrain and 0.05 world-unit support tolerance

**Rationale**: After deformation, a living base at or within 0.05 units above the current surface
is supported and snaps exactly to it, removing harmless numeric drift. A surface more than 0.05
units below starts a fall. A terrain rise to or above the base also snaps it upward to prevent
embedding. Each falling tick queries the current terrain again before contact.

**Alternatives considered**:

- Exact floating-point equality: too brittle for repeated fixed-step/contact operations.
- Crater radius/depth checks: couples tank behavior to one terrain weapon and fails for future
  changes.

## Decision: reuse the existing fixed cadence and gravity

**Rationale**: Projectiles already advance at a documented 1/120-second fixed cadence using a
validated, configurable downward gravity. Tank descent follows the same constant-acceleration
motion convention, so equal terrain, gravity, and step counts produce equal tank results. Expose
only the needed gravity magnitude rather than duplicating a second hard-coded tank gravity.

**Alternatives considered**:

- Render-frame delta: would make authoritative final tank positions frame-rate dependent.
- A new tank gravity value: breaks the battlefield-gravity relationship and adds needless tuning.
- Sharing a broad projectile/body abstraction: conflates a compact contact-bound tank operation
  with projectile lifetime and terrain-crossing rules.

## Decision: retain `ResolvingFire` until settling completes

**Rationale**: The existing phase already rejects aim, movement, and fire, and its completion
method already prevents duplicate handoff. At impact: apply the existing one-time explosion damage
→ deform terrain → reconcile living tanks → either complete immediately if all stable or leave the
turn resolving. A chained fixed settling system advances living falling tanks and completes the
existing resolution only when none remain falling.

**Alternatives considered**:

- A new turn phase: duplicates the action lock already supplied by `ResolvingFire`.
- Complete the turn and animate later: permits the next player to act against stale tank positions.
- Let boom/camera completion gate handoff: violates the presentation-only invariant.

## Decision: no fall damage and no settling for eliminated tanks

**Rationale**: Explosion damage remains its existing single pre-deformation consequence. A living
tank falling into a crater already changes later movement and launch origin; extra fall damage
would need unproven tuning. Eliminated tanks are visually hidden/inert and have no need for wreck
physics, so support evaluation skips them and match evaluation can finish promptly.

**Alternatives considered**:

- Damage based on fall distance: adds an unvalidated survival rule and double consequence.
- Wreck settling: adds presentation/physics scope without changing the finished duel.

## Decision: zero gravity leaves an unsupported tank explicitly falling and the shot unresolved

**Rationale**: Zero gravity is an already-valid battlefield parameter. With no downward force, an
unsupported tank cannot honestly settle on lower terrain. Keeping its falling state with zero
velocity is deterministic, bounded to one fixed update per tick, and does not invent a teleport or
fall-damage rule. The normal positive development gravity remains fully playable; a future
environment feature must choose a player-facing zero-gravity resolution policy.

**Alternatives considered**:

- Teleport immediately to terrain: contradicts the stated gravity relationship.
- Force normal turn completion while retaining a floating tank: violates the resolution boundary.

## Decision: preserve existing horizontal orientation

**Rationale**: The tank representation only supports horizontal body and turret directions. A
vertical descent preserving X/Z and those directions is a stable, legible initial resting result.
Deliberate movement continues to set body facing; terrain-normal tilt and slope sliding remain
future work.

**Alternatives considered**:

- Derive pitch/roll from terrain: introduces visual orientation behavior not supported by the
  current tank model or required for grounding.
- Slide down steep crater walls: changes tactical movement and requires a separate slope-motion
  design.
