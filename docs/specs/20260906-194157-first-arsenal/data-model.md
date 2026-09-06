# Data Model: First Arsenal

## WeaponId

Stable ordinary weapon identity. Initial values are Basic Shell and High Explosive. It is used for
catalogue lookup, inventory, selection, HUD mapping, tests, and the fired-shot snapshot; display
text and collection position are never identity.

## WeaponDefinition

| Field | Meaning | Rule |
|---|---|---|
| identity | Stable ordinary weapon identity | Unique in the catalogue |
| display name | Player-facing name | Presentation only; never an inventory key |
| availability rule | Unlimited or starting finite quantity | Basic is unlimited; HE starts at 2 |
| projectile profile | Current supported projectile configuration | Both initial rounds preserve existing ballistic flight |
| impact profile | Damage radius, maximum damage, crater, visual scale | Copied into a fired shot |

Initial profiles:

| Weapon | Availability | Damage radius / maximum | Crater radius / depth |
|---|---:|---:|---:|
| Basic Shell | Unlimited | 6.0 / 40 | 4.0 / 1.8 |
| High Explosive | 2 | 8.0 / 60 | 6.0 / 3.0 |

## PlayerWeaponLoadout

One authoritative player's weapon state: selected `WeaponId` and per-identity availability. It is
created with Basic Shell selected. Selection is accepted only for an available weapon during the
existing choosing phase. Spending is accepted only as part of a valid firing commitment.

**Transitions**:

```text
initial -> Basic selected, Basic unlimited, HE 2
choosing + 1 -> Basic selected
choosing + 2 + HE available -> HE selected
valid HE fire -> HE remaining decreases once
HE reaches 0 -> Basic selected
invalid selection/fire -> unchanged
```

## PlayerWeaponLoadouts

Two fixed `PlayerWeaponLoadout` values associated with the existing two stable players. A player
lookup returns only that player’s loadout. No operation on Player One's selection or ammunition may
mutate Player Two's loadout.

## FiredShot

Immutable authoritative record created at fire commitment:

| Field | Meaning |
|---|---|
| weapon identity | The selected conventional weapon at commitment |
| projectile | Existing position, velocity, and elapsed-step state |
| projectile profile | Values used by current flight, if any |
| impact profile | Copied damage, crater, and visual consequence values |

During flight the existing deterministic projectile is advanced. Terrain impact reads only the
captured impact profile, then executes common damage, crater, support, settling, survivor, and turn
completion. Player selection and inventory cannot alter it.

## HUD weapon view

Read-only derived view of the active player’s selected definition and availability. It displays a
name and either `UNLIMITED` or a finite remaining count. It is present while a player can make a
shot choice, and selection controls are shown only during choosing.
