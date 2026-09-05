# Data Model: First Boom — Visible Projectile Explosion

This feature has no persistent storage or gameplay data. It adds temporary renderer-owned state
that observes the existing terrain-impact result.

## Existing input

| Value | Meaning | Rule |
| --- | --- | --- |
| latest terrain impact | Resolved simulation result with world position | Sole explosion origin; absent after non-impact termination or new launch. |
| impact marker | Existing persistent diagnostic primitive | Continues independently of explosion lifetime. |

## Explosion presentation

| Value | Meaning | Rule |
| --- | --- | --- |
| origin | World position copied from terrain impact | Never recalculated by presentation. |
| elapsed time | Visual time since spawn | Starts at zero and advances only in presentation updates. |
| duration | Configured visual lifetime | Positive, tuneable, presentation-only. |
| scale | Current visible size | Starts small, grows to maximum, and has no gameplay radius meaning. |

## Local lifecycle state

| Value | Meaning | Rule |
| --- | --- | --- |
| consumed current impact | Whether current latest result spawned its one boom | Set at spawn; reset when new launch clears latest impact. |
| active explosion | Tagged temporary primitive | Exists from spawn to duration expiry, then despawns. |

## State transitions

```text
No latest impact --terrain impact--> Latest impact, marker, one explosion
Active explosion --visual time advances--> Active explosion with larger scale
Active explosion --duration complete--> No explosion; marker remains
Latest impact --new launch clears result--> No latest impact; consumption resets
No latest impact --next terrain impact--> Latest impact, marker, new explosion
```

Out-of-bounds termination has no terrain impact and cannot enter this lifecycle.
