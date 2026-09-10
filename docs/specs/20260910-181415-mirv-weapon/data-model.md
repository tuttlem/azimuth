# MIRV Data Model

| Entity | State | Invariants |
|---|---|---|
| Weapon definition | ID, name, ammo, behavior, profiles | MIRV has two rounds; child profile is lower than HE. |
| Fired shot | committed weapon; ordered active projectile records | Owns all carrier/children until zero active records. Conventional weapons own one. |
| Active projectile | projectile state, copied impact profile, role | Role is conventional, carrier, or stable child index; no graph behavior. |
| Carrier | ordinary active projectile, split pending | On surviving first post-step `velocity.y <= 0`, remove it and create exactly five children. |
| Child | ordinary active projectile, index | Copies split state plus fixed impulse; independently advances, collides, and terminates. |
| Impact presentation event | ordered ID, position, scale, hit indication | Emitted after authoritative terrain impact; observers only. |
| Shot snapshot | mode, carrier/children positions, split/aggregate region | Read-only camera/visual input. |

```text
commit → [carrier] → normal ascent → apex split → [child 0..4]
→ ordered independent termination → empty shot → settle → handoff/result

pre-split terrain/bounds termination → normal unsplit resolution
```

- A carrier is active or replaced, never both. Child order is terrain and presentation order.
- A child cannot remove siblings. Inventory is consumed only at commit.
- Turn completion requires zero active records and zero settling living tanks.
- Presentation may coalesce feedback, never outcomes.
