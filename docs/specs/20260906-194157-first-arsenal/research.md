# Research: First Arsenal

## Decision: use one compile-time conventional weapon catalogue

**Decision**: Define stable conventional weapon identities and their definitions in one small,
obvious weapons domain boundary. Each definition supplies display metadata, an explicit unlimited
or finite availability rule, the currently supported projectile profile, impact profile, and small
visual scale.

**Rationale**: The current round is implicit: firing creates a bare projectile while impact uses
global 6-unit/40-damage and default-crater values. A central definition makes Basic Shell explicit,
lets High Explosive vary values through the same path, and gives a future ordinary shell one place
to be added.

**Alternatives considered**:

- Weapon-name conditionals in firing and impact: rejected because values would spread through
  unrelated systems and fail the ordinary-third-weapon criterion.
- A runtime registry, external data, plugins, or scripting: rejected because two static weapons do
  not warrant content infrastructure.
- A universal weapon behaviour hierarchy: rejected because no current weapon needs exceptional
  flight or impact behaviour.

## Decision: keep player weapon state outside tanks and turns

**Decision**: Keep one compact per-player conventional loadout resource parallel to existing tanks:
selected identity plus inventory keyed by stable identity. Tanks remain physical game pieces and
turn state remains aim/phase ownership.

**Rationale**: Current `Tank` owns pose, health, and support; `TurnState` owns independent aim and
turn phase. A loadout is neither physics nor phase state, but needs independent player ownership.
The existing two-player fixed representation permits direct, readable player lookup without a
generic RPG inventory.

**Alternatives considered**:

- Add weapons to `Tank`: rejected because it mixes physical simulation with player arsenal state.
- Add weapons to `TurnState`: rejected because phase/aim transition code should not own inventory.
- Generic item inventory: rejected because only conventional ammunition is required.

## Decision: select with 1/2 only while choosing

**Decision**: `1` selects Basic Shell and `2` selects available High Explosive only during an
in-progress choosing phase. The HUD shows selection and concise selectors only then.

**Rationale**: Existing arrows, `-`/`=`, Shift, M, Enter, and Space are occupied. `1`/`2` do not
conflict. Choosing is already the only phase that accepts aim or begins fire, so it preserves the
move-or-fire rule and avoids an extra mode.

**Alternatives considered**:

- Cycling keys: rejected because direct two-weapon selection is clearer and avoids exhausted-cycle
  policy complexity.
- Selection during movement or resolving: rejected because it makes a turn action ambiguous and
  could change a pending shot representation.

## Decision: snapshot a fired conventional shot

**Decision**: Replace the bare active projectile state with a fired-shot snapshot containing the
weapon identity, existing projectile, and copied impact/presentation profile. Validate availability
before beginning fire; immediately after snapshot commitment consume one finite round and fall back
to Basic Shell if that was the final HE round.

**Rationale**: Existing fixed update moves `Projectile`, then hard-codes combat and crater values.
Capturing profile data means the resolver never reads mutable selected weapon/inventory. It also
makes an out-of-bounds committed HE correctly consume exactly once while producing no impact.

**Alternatives considered**:

- Look up active player selection during impact: rejected because later selection/state could alter
  the in-flight shot.
- Store only identity and re-read the catalogue: rejected because copied data is the clearest
  immutable authority boundary and protects later catalogue evolution.

## Decision: parameterise the existing common impact consequence

**Decision**: Pass captured impact data to existing radial damage and crater application. Preserve
Basic Shell's 6/40 and 4.0/1.8 values; configure HE as 8/60 and 6.0/3.0 with two rounds per player.

**Rationale**: Current combat already has pure deterministic three-dimensional linear falloff and
terrain already accepts arbitrary validated craters. Parameterisation reuses these systems while
HE visibly changes the game.

**Alternatives considered**:

- Separate HE explosion/deformation systems: rejected because it duplicates exactly the ordinary
  behaviour that should remain common.
- Infer gameplay blast from visual scale: rejected because existing rules deliberately separate
  presentation from authority.

## Decision: extend the existing pure tactical HUD view

**Decision**: Add selected weapon/ammunition presentation to the existing derived tactical HUD view
and shot/environment panel. Use ASCII-safe `UNLIMITED` rather than a glyph; add weapon controls to
the choosing hint only.

**Rationale**: The HUD already has one read-only state-to-view seam and an available aim/wind panel.
This avoids duplicate weapon state and the prior font issue with special characters.

## Framework evolution rule

Ordinary weapons remain definition data as long as existing flight and impact parameters express
their difference. Do not generalise behaviour until a real weapon requires it. A concrete future
MIRV, bouncer, roller, penetrator, terrain-builder, or teleporter may justify the smallest explicit
new behaviour at that time; this feature does not predict a universal scripting model.
