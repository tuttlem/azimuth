# Research: Randomized Round Starts

## Round seed lifecycle

**Decision**: Retain an immutable root seed for the active local session and derive a distinct round seed from that root plus the one-based round number. A newly started local session receives a fresh root; replay/debugging can reproduce a round from its root and number.

**Rationale**: `main.rs` currently selects one wall-clock `MatchSeed` at startup, but transition calls `generate_match_world(match_seed.0, &ids)` again. The existing factory is pure and already derives labelled terrain, starts, dressing, wind, and AI streams, so changing only its round-level input fixes repeated worlds without coupling gameplay randomness to UI or cosmetics.

**Alternatives considered**: Sampling wall-clock randomness at every transition would be unreproducible; mutating one shared random cursor would let unrelated consumers perturb worlds; rebuilding terrain/start algorithms is unnecessary because both already accept deterministic seeds.

## Valid round generation

**Decision**: Make the round-world factory return a valid generated world only after a bounded deterministic candidate sequence finds terrain with valid starts for every configured player.

**Rationale**: `BattlefieldTerrain::generated` is deterministic and `initial_tanks_for_players_seeded` selects dry, gentle, in-bounds, separated starts, but the selector currently panics if candidates are insufficient. A bounded retry is small, reproducible, and satisfies the playable-round guarantee.

**Alternatives considered**: Keeping the panic violates the guarantee; accepting fewer, overlapping, or wet starts violates spawn invariants; tactical balance analysis or a map-selection system exceeds scope.

## Round boundary ownership

**Decision**: Use one explicit round-start boundary for initial setup and shopping transition, resetting all combat/presentation-only resources and entities there while copying only session-owned player resources forward.

**Rationale**: Transition already replaces terrain, tanks, turn, dressing, and wind, but leaves flight, latest impact, effect state, temporary input feedback, and related entities intact. Centralizing reset prevents stale markers, effects, selection, aiming, and result state.

**Alternatives considered**: Waiting for later sync systems can leave old effects visible; a full app restart loses cash, wins, ammo, identity, and controller configuration.

## Terrain-adjacent presentation

**Decision**: Recreate the horizon as well as tanks and dressing when a round terrain changes.

**Rationale**: The horizon is created only at startup from the old terrain seed; leaving it creates a visible mismatch. It remains presentation-only and has no gameplay role.

**Alternatives considered**: Keeping the old horizon contradicts a fresh world; making it authoritative or deformable violates the presentation boundary.
