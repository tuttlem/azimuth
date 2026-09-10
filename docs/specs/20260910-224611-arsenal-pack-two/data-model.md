# Arsenal Pack #2 Data Model

| Entity | Captured / current state | Invariant |
|---|---|---|
| Weapon definition | identity, limited ammunition, projectile and impact profile | Definition values are never selected from UI after fire commit. |
| Player loadout | selected weapon, per-weapon availability, Curve Ball direction | Clicks update only this authoritative player state; unavailable weapons are hidden. |
| Fired shot | weapon, projectile, impact profile, curve direction, bounded contact/bounce state | One launch consumes one round and owns all behaviour until resolved. |
| Mound | radius and positive height | Applies a finite deterministic upward radial deformation within terrain bounds. |
| Surface normal | current triangle normal at an in-bounds horizontal point | Comes from the same current terrain triangle used by collision/querying. |
| Bouncer state | remaining bounces and reduced velocity | Each bounce remains airborne; final later impact is the only damaging impact. |
| Nuke impact | large normal radial impact profile | Uses existing damage, deformation, settling, elimination, and winner/draw consequences. |

## State transitions

```text
choosing Curve Ball direction → commit shot → captured direction → normal curved flight → impact → resolve
normal Bouncer flight → terrain contact → bounce (0..3) → normal flight → final contact → resolve
normal Dirt/Nuke flight → terrain contact → mound or crater/damage → reconcile support → settle → handoff
```

All transitions are fixed-step authoritative state. Camera, UI, audio, and pulse presentation
observe them and never change their order or outcome.
