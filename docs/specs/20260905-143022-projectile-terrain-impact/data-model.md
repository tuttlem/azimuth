# Data Model: Projectile Terrain Impact Detection

The feature has no persistent storage or external contract. It extends the in-memory domain with
one authoritative static terrain surface and the smallest observable flight outcome needed by
presentation and later gameplay.

## Authoritative terrain surface

| Value | Meaning | Rules |
| --- | --- | --- |
| horizontal bounds | Static X/Z terrain domain | Inclusive -20 to +20 on each axis; outside it, no terrain intersection is evaluated. |
| grid resolution | Shared mesh/query layout | 20 cells per side with mesh-consistent cell diagonals. |
| vertex sample | Deterministic authored elevation | Retains the current non-flat relief values. |
| local terrain height | Surface Y at in-bounds X/Z | Piecewise planar mesh-triangle interpolation. |

The mesh consumes the same samples and triangles; tank grounding and projectile collision consume
the same local height.

## Projectile flight

| Field | Meaning | Rules |
| --- | --- | --- |
| position | Current world position | Starts at firing origin; uses current kinematics until refined to impact surface. |
| velocity | Current world velocity | Uses existing gravity-only update; no bounce, impulse, or post-impact flight. |
| elapsed steps | Completed fixed steps | Increases once per advance, including an impact step. |
| active flight holder | Optional current projectile | Present only while advancement outcome is Active. |

## Terrain impact result

| Field | Meaning | Rules |
| --- | --- | --- |
| position | Resolved world-space contact | Produced only by an above-to-on/below crossing; within 0.01 world units of authoritative surface. |

Its existence indicates terrain impact. It deliberately has no normal, material, explosion,
weapon, damage, energy, or generic collision data.

## Advancement outcome

| Outcome | Meaning | Application transition |
| --- | --- | --- |
| Active | Candidate is within useful limits and does not cross terrain | Retain projectile; sync visual. |
| Terrain impact | Travelled segment crosses terrain | Store result; remove flight/visual; show marker. |
| Non-impact termination | Limits/lifetime end flight without terrain contact | Remove flight/visual; no result or marker. |

Terrain impact takes precedence for a valid in-bounds crossing. Outside terrain's horizontal
domain, existing useful-volume and lifetime rules remain authoritative.

## Presentation state

| Value | Meaning | Rules |
| --- | --- | --- |
| latest impact | Optional stored terrain result | Set only by Terrain impact; cleared at next development launch. |
| projectile visual | Tagged sphere observing active flight | Never advances or decides collision. |
| impact marker | Tagged primitive observing latest impact | Uses stored position only; removable without domain change. |

## State transitions

```text
No flight, no latest impact --Space--> Active flight, no latest impact
Active flight --step above terrain and within limits--> Active flight
Active flight --segment crosses terrain--> No flight, latest terrain impact
Active flight --step outside limits without terrain crossing--> No flight, no terrain impact
No flight, latest terrain impact --Space--> Active flight, no latest impact
```
