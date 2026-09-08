# Battlefield Generation Contract

This is an internal game-domain and presentation boundary, not a network or public API.

## Inputs

| Input | Required behaviour |
|---|---|
| Captured match seed | Identical seed and relevant configuration reproduce terrain, starts, dressing, and wind through independent labelled derivations. |
| Player identities | Valid list of 2–8 configured human player identities receives one start each. |
| Current terrain | The mutable authoritative surface used for queries, craters, starts, collision, and support. |

## Authoritative Outputs

| Output | Contract |
|---|---|
| Generated terrain | 120-by-120 bounds, 64 cells per side, finite current heights, macro/local geography, and one consistent height query/mesh/collision source. |
| Start list | Exactly one grounded start per requested player; each is in bounds, dry, locally suitable, and at least 18 horizontal units from every other start. |
| Battlefield projectile limits | Fixed-step limits that allow valid in-bounds terrain collision on the expanded map and retain distinct out-of-bounds termination. |

## Presentation Outputs

| Output | Contract |
|---|---|
| Terrain vertex colour | Bounded current-height colour mapping, regenerated with mesh positions after terrain mutation. |
| Water plane | Flat blue plane at water-table elevation; not queried by gameplay. |
| Building records/entities | Sparse dry, grounded, start-clear primitive blocks; never supplied to authoritative game systems. |

## Invariants

- Rendering schedule, frame rate, water visibility, and dressing placement must not mutate or change authoritative terrain, starts, wind, projectile paths, tank support, damage, or turns.
- A crater changes only current terrain. It may change the derived mesh colours and look flooded below water, but it never triggers water physics or building interaction.
- Buildings and water are never terrain, tanks, targets, colliders, obstacles, cover, damage recipients, or line-of-fire inputs.
- The contract makes no claim of tactical fairness, reachable every-opponent geometry, or collision with buildings.

## Failure and Diagnostics

- A malformed or out-of-bounds terrain query retains existing safe/rejected semantics.
- Bounded deterministic start selection must expose a clear development failure if it cannot produce enough valid starts; it must not fall back to an underwater, steep, overlapping, or out-of-bounds placement.
- A dressing candidate that fails any placement rule is omitted; visual dressing failure cannot invalidate a match.
