# Data Model: Heavy Shell Wind Resistance

## `WindResponse`

| Field / operation | Meaning | Rules |
|---|---|---|
| response factor | Fraction of match horizontal wind acceleration accepted by one projectile | Finite and non-negative; normal is `1.0`; Heavy Shell is initially `0.40`. |
| validate / construct | Creates a response value | Reject NaN, infinity, and negative values. |
| read value | Supplies the fixed-step wind multiplier | Does not modify gravity, launch speed, vertical motion, or terrain collision. |

`WindResponse` is a concrete gameplay parameter, not kilograms or an aerodynamic model. A zero
value is valid for the generic domain but is not assigned to Heavy Shell.

## `ProjectileProfile`

| Field | Meaning | Rules |
|---|---|---|
| `wind_response` | Flight characteristic supplied by a conventional weapon definition | Basic Shell and High Explosive use normal response; Heavy Shell uses the positive reduced response. |

Relationship: one `WeaponDefinition` owns one `ProjectileProfile`.

## `WeaponDefinition` and `WeaponId`

| Entity | New state | Rules |
|---|---|---|
| `WeaponId` | Adds stable `HeavyShell` identity | Never inferred from display label, selector key, or storage order. |
| Heavy Shell definition | `HEAVY SHELL`; limited two-round rule; 0.40 projectile response; Basic Shell-class impact profile | Central catalogue is the source of these values. |
| Existing definitions | Basic Shell: unlimited/normal response; High Explosive: two rounds/normal response | Existing impact and flight identities remain unchanged. |

## `PlayerWeaponLoadout`

| State | Initial value | Transition |
|---|---|---|
| selected weapon | Basic Shell | Selecting any available conventional weapon changes only selection. |
| Basic Shell availability | unlimited | Never decremented. |
| High Explosive availability | 2 per player | Legal fire decrements only the firing player's entry. |
| Heavy Shell availability | 2 per player | Legal fire decrements only the firing player's entry. |

On successful commitment, the selected finite weapon is decremented exactly once. If that leaves
it unavailable, selection falls back to Basic Shell for a subsequent choosing turn. Invalid
selection/fire has no mutation. Each player owns a separate loadout.

## `Projectile` and `FiredShot`

| Entity | New / retained authoritative state | Transition |
|---|---|---|
| `Projectile` | Position, velocity, elapsed fixed steps, captured `WindResponse` | Each fixed step adds gravity plus scaled horizontal wind, calculates swept terrain collision, and updates state deterministically. |
| `FiredShot` | Weapon identity, projectile, copied impact profile | Created after legal commitment; remains independent of later selection, availability, catalogue presentation, HUD, camera, or effects. |

Authoritative flow:

```text
selected WeaponDefinition
  → ProjectileProfile.wind_response
  → launched Projectile.wind_response
  → FiredShot
  → shared fixed-step terrain flight
  → copied ordinary impact profile
```

## Wind and gravity invariant

For one step, the existing gravity vector is unchanged. The horizontal wind vector is multiplied
by the projectile response before it is combined with gravity. Therefore zero wind has no
response-dependent result, and every horizontal wind direction is scaled proportionally without a
vertical component.
