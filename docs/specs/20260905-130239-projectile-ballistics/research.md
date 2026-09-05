# Research: First Projectile and Deterministic Ballistic Arc

## Decision: Keep projectile physics in a small pure domain module

**Rationale**: The first flight needs the existing tank firing origin and a new velocity value, and
its mathematics needs fast deterministic unit tests. A concrete shared `world.rs` for scalar
position/vector values and a concrete `projectile.rs` for flight state provide that boundary
without a crate, trait, math library, or presentation abstraction.

**Alternatives considered**:

- Put Bevy vectors and trajectory calculations directly in `main.rs`: rejected because tests would
  become presentation-coupled and tank/projectile values would have no clear shared domain form.
- Create an `azimuth-math` or physics crate: rejected because one position/vector pair and one
  flight model do not yet establish a dependency, ownership, reuse, or testing boundary requiring a
  crate.
- Add a physics engine: rejected because constant gravity flight is a few explicit calculations and
  collision is intentionally outside this feature.

## Decision: Use an exact fixed 1/120-second constant-acceleration update

**Rationale**: The specification requires reproducible flight independent of render rate. The
update `p += v * dt + 0.5 * a * dt²; v += a * dt` directly expresses constant gravity, preserves
horizontal velocity, has simple analytical tests, and leaves a clear per-step boundary for future
terrain collision.

**Alternatives considered**:

- Variable render-frame updates: rejected because frame rate would define results and make
  regression behaviour unreliable.
- Semi-implicit Euler: rejected because the specified kinematic update is equally simple and more
  directly matches the constant-acceleration model.
- A general physics scheduler: rejected because a single fixed projectile update has no present
  need for one.

## Decision: Configure the application's fixed schedule at 120 Hz

**Rationale**: Bevy 0.18.1's `FixedUpdate` runs zero, one, or multiple times between visual
updates and supplies fixed elapsed time inside that schedule. Explicitly configuring 120 Hz keeps
simulation progression reproducible while giving smooth-enough movement for one visible sphere.

**Alternatives considered**:

- Run flight from the normal update schedule using frame delta: rejected because it violates the
  fixed-step requirement.
- Handle `Space` in the fixed schedule: rejected because an input edge can otherwise be observed
  unpredictably when fixed ticks run zero or multiple times per rendered frame.
- Add interpolation: rejected because 120 Hz is sufficient for this proof and interpolation would
  add state not needed for the feature.

## Decision: Launch one fixed development shot from Player One with Space

**Rationale**: The existing Player One firing origin and turret direction are the appropriate
handoff point. `Space` makes the vertical slice immediately observable without creating aiming or
turn systems. The planned 45-degree, 18-unit-per-second shot with default gravity 8 produces a
readable arc in the current abstract-scale scene.

**Alternatives considered**:

- Adjustable angle or power controls: rejected because the dedicated aiming feature owns that
  player interaction.
- Multiple simultaneous projectiles or a firing queue: rejected because this is one first-flight
  proof, not a weapon architecture.
- Automatic repeat fire: rejected because it obscures reproducibility and makes inspection harder.

## Decision: Use one tagged primitive sphere as a pure visual observer

**Rationale**: Existing scene code already uses primitive meshes and direct transforms. One bright
sphere spawned at launch, synchronised from domain state, and removed when the domain flight ends
is immediately understandable and needs no asset or effects system.

**Alternatives considered**:

- A trajectory trace or generic diagnostics service: deferred unless a specific inspection need
  appears during implementation; it is not necessary to prove a visible arc.
- Visual-side motion: rejected because simulation must remain authoritative.
- GPU/screenshot tests: rejected because they add fragile infrastructure with little value for this
  first rendering proof.

## Decision: End only on simple simulation-volume limits, not terrain

**Rationale**: X/Z extent 60, Y range -30 through 100, and a 20-second lifetime prevent an
indefinite active object while allowing the shot to visibly pass through the intentionally
non-solid terrain. This makes the next collision feature's boundary unambiguous.

**Alternatives considered**:

- Stop at terrain height: rejected because it would prematurely implement projectile impact.
- Keep projectiles indefinitely: rejected because it leaves unnecessary active simulation state.
- Use only a time limit: rejected because clearly departed projectiles should be removed promptly.
