# Research: Placeholder Tanks and Player Entities

## Decision: Use concrete engine-independent player and tank data

Place the first game-piece concepts in a small `tank.rs` module using ordinary numeric fields for
player identity, horizontal position, world position, horizontal directions, pose, and tank. Keep
the types concrete and limited to two fixed initial tanks.

**Rationale**: The feature has a present need for domain data that a later projectile system can
read without depending on renderer objects. Plain concrete values make deterministic placement,
bounds validation, and firing-origin tests straightforward. This is the smallest useful boundary;
it does not require a library crate, traits, an entity framework, or a generic player/vehicle model.

**Alternatives considered**:

- Store player/tank information only in rendering entities: rejected because a later simulation
  would have to depend on presentation state for its pose and firing origin.
- Create a new reusable simulation crate: rejected because two initial pieces provide neither
  reuse nor ownership evidence for a workspace boundary.
- Add generic actor, vehicle, weapon, or spawn abstractions: rejected as premature generalisation.

## Decision: Extract the existing terrain facts into a small battlefield module

Move the current fixed horizontal extent, terrain-height calculation, and simple in-bounds test to
`battlefield.rs`. Tank placement uses this current height function directly.

**Rationale**: The terrain calculation now has two concrete consumers—mesh construction and tank
placement. Keeping it in one engine-independent location prevents visual geometry and player
placement from disagreeing, while remaining far smaller than a terrain-query framework.

**Alternatives considered**:

- Duplicate the height calculation in tank placement: rejected because visual and domain terrain
  would diverge.
- Introduce a terrain trait or generic query service: rejected because one current deterministic
  function satisfies the only requirement.
- Implement collision or slope alignment: rejected because placement, not physics or movement, is
  the scope and an upright body is explicitly acceptable.

## Decision: Derive firing origin from pose

Compute the firing origin every time from the terrain-resolved tank position, turret direction, a
small forward offset, and a positive vertical offset. Do not store it separately.

**Rationale**: A derived origin cannot become stale if a later feature changes a tank pose or turret
direction. It gives the next projectile feature a clear input without deciding aiming controls,
angle conventions, projectile type, or launch behaviour.

**Alternatives considered**:

- Store a world-space firing point during spawn: rejected because it would no longer follow future
  pose changes automatically.
- Omit a firing origin: rejected because the feature's clear handoff to deterministic projectile
  launch would be weaker.

## Decision: Render one parent tank with primitive child parts

At the Bevy boundary, use one parent entity for each tank pose and local child primitives for body,
turret, and barrel. Give each player's parts one distinct simple material colour. Use an explicit
visibility component on transform-only parents when needed for child visibility inheritance.

**Rationale**: Parent-child transforms keep the visible barrel direction aligned with the concrete
tank pose and make a readable silhouette with no external assets. The domain remains authoritative
for position, direction, and firing origin; Bevy transforms only present those values.

**Alternatives considered**:

- Separate unrelated world-space primitives: rejected because keeping parts together is clearer
  and would otherwise make later pose updates error-prone.
- Final artwork or an asset pipeline: rejected because simple readability is all this feature needs.
- A renderer-independent scene graph: rejected as an abstraction with no present requirement.

**Sources**:

- [Bevy parenting example](https://bevy.org/examples/3d-rendering/parenting/)
- [Bevy visibility inheritance guidance](https://bevy.org/learn/errors/b0004/)
- [Bevy Transform API](https://docs.rs/bevy/0.18.1/bevy/prelude/struct.Transform.html)
- [Bevy 0.18 migration guide](https://bevy.org/learn/migration-guides/0-17-to-0-18/)
