# Conventional Weapon Contract: Heavy Shell Extension

This is an internal gameplay contract, not a network or public API.

## Catalogue and loadouts

- `WeaponId::HeavyShell` is a stable identity distinct from Basic Shell and High Explosive.
- The central Heavy Shell definition has display name `HEAVY SHELL`, limited availability of two
  rounds per player, a positive `0.40` wind response, and the ordinary Basic Shell-class impact
  profile.
- Every player loadout is independent. Selecting an available weapon never consumes ammunition.
- Basic Shell remains explicitly unlimited; High Explosive and Heavy Shell are each finite.

## Commitment and snapshot

```text
active choosing player + selected available weapon + fire
  → capture definition projectile profile into Projectile
  → copy projectile and impact profile into FiredShot
  → consume one finite selected round, if applicable
  → resolving fire phase
```

A rejected fire creates no shot and spends no ammunition. A final finite round returns future
selection to Basic Shell only after that round is captured. Later selection, availability, HUD, or
catalogue presentation cannot change the flight response or impact of a committed shot.

## Shared flight and impact

```text
Projectile(position, velocity, captured wind response)
  + gravity + match horizontal wind
  → shared fixed-step terrain collision
  → captured ordinary impact profile
  → damage, crater, support/settling, victory, handoff
```

Flight scales match horizontal wind by the projectile's captured response. It never branches on
Heavy Shell identity and does not scale gravity. Heavy Shell can land elsewhere due to reduced
drift, but uses the same collision and ordinary impact sequence as the other conventional weapons.

## Presentation boundary

The normal weapon selector and tactical HUD expose selection and truthful availability. They do
not offer predicted impacts, corrected aim, or automatic compensation. Camera, HUD, projectile
visual, and explosion presentation observe authoritative state only.
