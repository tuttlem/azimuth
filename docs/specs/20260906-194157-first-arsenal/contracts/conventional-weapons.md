# Conventional Weapon Gameplay Contract

This is an internal gameplay contract, not a network or external API.

## Catalogue and loadouts

- `WeaponId` is stable and independent of label/order.
- A catalogue definition is the sole source for an ordinary weapon's metadata and supported
  ordinary projectile/impact values.
- Basic Shell is explicitly unlimited and High Explosive begins at two per player.
- Player inventories and selected weapons are independent.
- Selecting a weapon is legal only in an in-progress choosing turn and never spends ammunition.

## Fire commitment

```text
active player + selected available weapon + fire
  -> captured FiredShot
  -> consume one finite round, if applicable
  -> resolving fire phase
```

If any precondition fails, this transition creates no shot and spends no ammunition. A final HE
round causes future selection to fall back to Basic Shell only after the HE shot was captured.

## Ordinary shot resolution

```text
FiredShot projectile + fixed gravity/wind + terrain
  -> terrain impact
  -> captured impact profile applies shared radial damage and crater
  -> support reconciliation / deterministic settling
  -> survivor and match resolution
  -> next turn, if applicable
```

The resolver must not read mutable player selection, inventory, HUD, camera, or visual-effect
timing. A no-impact termination has no blast/crater but still resolves the already committed shot.

## Extension boundary

A new conventional explosive with only supported value differences is added centrally as a
definition and availability entry. A new mechanic earns a narrow extension only after a real
weapon demonstrates that the ordinary shot contract cannot express it.
