# Research: First Duel — Damage, Elimination and Victory

## Decision: put health directly on `Tank`

**Rationale**: A tank already owns its player identity and authoritative base position. A shared
`MAX_HEALTH` plus current whole health and an `is_eliminated` predicate is the smallest coherent
survival representation. Zero health derives elimination, avoiding a second state that can drift.

**Alternatives considered**:

- Separate per-player health resource: duplicates tank ownership and complicates blast lookup.
- Armour/component systems: no first-duel gameplay need.

## Decision: use a pure `combat.rs` blast resolver

**Rationale**: Full 3D world distance and `ceil(40 * (1 - d / 6))` are domain rules that can be
tested without Bevy. The resolver first computes both damage amounts from the same immutable tank
array, then applies them. This naturally supports self-damage and mutual elimination.

**Alternatives considered**:

- Visual explosion scale: presentation timing/scale cannot define authority.
- Damage each tank while iterating mutably: makes same-event guarantees less explicit.

## Decision: add a compact outcome to existing turn state

**Rationale**: `InProgress`, `Winner(PlayerId)`, and `Draw` are sufficient to gate actions and
stop handoff. Existing turn methods receive survivor information and select a living player rather
than blindly calling `PlayerId::other`; no generic scheduler is needed for two players.

**Alternatives considered**:

- Generic game-mode/match engine: disproportional to one local duel.
- Infer match end from visibility: renderer state is not authority.

## Decision: resolve combat before handoff in terrain-impact resolution

**Rationale**: The fixed-update resolver already owns impact, crater, and `complete_resolution`.
It will calculate/apply damage, deform terrain, determine result, then hand off or finish. An
out-of-bounds projectile remains a non-damaging normal handoff.

**Alternatives considered**:

- Apply damage from the boom system: visual lifetime would incorrectly control gameplay.
- Wait for camera: violates the established presentation boundary.

## Decision: hide eliminated placeholder roots and show text health/result

**Rationale**: Existing root visuals are already identifiable by player. Hiding an eliminated root
is clear and avoids destruction animations. One existing HUD can report both health totals,
elimination, and winner/draw without a health-bar system.

**Alternatives considered**:

- Wrecks, debris, fire, or animation: out of scope.
- Renderer-based health as authority: conflicts with deterministic domain state.
