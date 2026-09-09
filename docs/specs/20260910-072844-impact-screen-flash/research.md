# Research: Impact Screen Flash

## Decision: Reuse the existing once-per-impact presentation boundary

The current latest terrain impact and consumed explosion presentation already distinguish one resolved terrain impact from its persistent diagnostic state.

**Rationale**: It prevents duplicate flashes from multi-tank damage, elimination, camera movement, or repeated frames.

**Alternatives considered**: Damage- or elimination-driven flashing would incorrectly repeat one physical explosion.

## Decision: Derive a bounded pulse profile from impact visual scale

Baseline scale produces restrained opacity/duration; the existing larger High Explosive scale produces modestly stronger/longer presentation.

**Rationale**: Existing captured profile data avoids weapon-name rules.

## Decision: Use one UI overlay and a rapid fade

The overlay is created once and receives short-lived presentation state after impact.

**Rationale**: It is simpler and safer than post-processing or multiple layered flashes.
