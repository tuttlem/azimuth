# Research: First Boom — Visible Projectile Explosion

## Decision: Use one bright expanding primitive

Spawn one luminous warm-coloured primitive at the existing impact position and scale it rapidly
over a short duration before removal.

**Rationale**: The scene already creates primitive meshes and materials directly. A bright
expanding sphere is unmistakable, cheap, easy to tune, and needs no particle infrastructure.

**Alternatives considered**:

- Particle/debris system: rejected because one first boom does not demonstrate a reusable need.
- Smoke, sound, camera shake, or physical blast simulation: rejected as later work.
- Static impact mesh: rejected because temporary change and cleanup are required.

## Decision: Track lifetime in a local presentation component

Each spawned effect holds elapsed visual time. Normal presentation updates add frame delta,
calculate scale from configured duration, and despawn when the duration completes.

**Rationale**: This is the smallest direct lifecycle for renderer-owned state. It is not gameplay
and has no requirement to use the projectile's deterministic fixed schedule.

**Alternatives considered**:

- Generic animation framework: rejected because one scale curve does not justify one.
- Fixed-update lifetime: rejected because presentation does not own simulation authority.
- Never remove the entity: rejected because repeated shots would accumulate stale state.

## Decision: Consume persistent latest impact once per shot

The current latest terrain impact persists so its marker can remain. A small presentation tracker
records whether it has already spawned an explosion and resets when the next launch clears it.

**Rationale**: This prevents a boom each frame and permits a later identical impact position to
create another boom, without changing simulation or adding an event bus.

**Alternatives considered**:

- Remove impact result at explosion spawn: rejected because it removes the useful marker.
- Compare positions to infer a new impact: rejected because repeated shots may land identically.
- Add a generic event stream: rejected because the single consumer needs direct local state.
