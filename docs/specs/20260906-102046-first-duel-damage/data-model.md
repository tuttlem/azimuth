# Data Model: First Duel — Damage, Elimination and Victory

## Tank survival state

| Field | Rule |
|---|---|
| maximum health | Shared constant: 100. |
| current health | Whole amount in 0–100; begins at 100 and saturates at zero. |
| eliminated | Derived as current health equals zero. |

The existing tank owner and authoritative world-space base position remain unchanged. A tank is
never removed from gameplay identity when eliminated.

## Explosion gameplay configuration and result

| Field | Rule |
|---|---|
| centre | Existing authoritative terrain-impact world position. |
| damage radius | Shared tuneable 6.0 world units. |
| maximum damage | Shared tuneable 40 health. |
| tank distance | Euclidean 3D distance from centre to tank base. |
| damage | Zero at/after radius; otherwise `ceil(maximum * (1 - distance / radius))`. |

One resolution captures both damage amounts from pre-damage tank positions before mutating either
tank. It then applies both clamped health changes and derives survivors.

## Match and turn state

| State | Meaning | Allowed ordinary actions |
|---|---|---|
| In progress | Two surviving tanks; normal choosing/moving/resolving turn phase applies. | Current living player only. |
| Winner(PlayerId) | Exactly one survivor after explosion resolution. | None. |
| Draw | No survivor after the same explosion resolution. | None. |

**Transitions**:

1. Application startup: both health values 100; match in progress; Player One choosing.
2. Terrain impact: calculate all damages → apply health → deform terrain → count survivors.
3. Two survivors: complete resolution and choose the next survivor.
4. One survivor: finish with that winner; do not advance to a new turn.
5. No survivors: finish draw; do not advance to a new turn.
6. Non-impact projectile termination: no damage; complete normal survivor-aware handoff.

## Presentation projections

| Presentation | Source | Authority |
|---|---|---|
| HUD health/elimination/result | tank health and match outcome | Read-only |
| Hidden eliminated tank root | tank elimination | Read-only |
| Camera/boom | existing turn, flight, and impact state | Read-only |

```text
terrain impact -> combat damage resolution -> tank health/elimination
              -> terrain crater -> match/survivor result -> turn handoff or finish
              -> HUD/tank visibility/camera/boom presentation
```
