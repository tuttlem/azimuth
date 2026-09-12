# Data Model: Between-Round Shop — Weapon Purchasing and Visual Inventory

## Shop Item

| Field | Meaning | Validation |
|---|---|---|
| identity | Stable item identity | Maps to exactly one active product |
| category | Product grouping | `Weapons` is active; `Armour` is reserved only |
| display identity | Shared full name, compact name, and visual recipe | Exists for every selectable current weapon |
| price | Cost of one purchase unit | Positive for purchasable items |
| purchase effect | Authoritative state change | Must be atomic or make no change |

For this feature, every active Shop Item has the weapon-ammunition effect. The model intentionally permits a later Armour item to use a different effect without introducing a generic inventory system.

## Shop Turn

| Field | Meaning |
|---|---|
| participant order | Configured participant order, retained for the entire session |
| active participant | Player currently allowed to buy or finish |
| completed participants | Players whose between-round processing is complete |
| controller | Determines human interaction or automatic AI resolution |

### State transitions

```text
Accounting acknowledged
    → Shopping(active first configured participant)
    → human purchases zero or more items → Done → next participant
    → AI bounded purchase pass → next participant
    → all participants complete → Transition → fresh round
```

No player outcome, wallet value, or inventory count removes a participant from this sequence. A Shop Turn becomes immutable after Done.

## Session Player and Inventory

`SessionPlayer` remains the owner of cash and `PlayerWeaponLoadout`. A successful purchase changes only the active owner's cash and matching limited availability. Fresh round creation copies this session inventory back to battlefield weapon state and resets only selection; it does not reset owned limited ammunition.

## Weapon Presentation Identity

| Attribute | Use |
|---|---|
| stable weapon identity | Links shop product, loadout, and strip card |
| full name | Shop card readability |
| compact name | Battlefield strip readability |
| visual recipe | Small recognisable Azimuth-specific card image |
| availability | Owned count or unlimited state |

Presentation is a projection of stable weapon identity. It owns no cash, ammunition, selection, or combat behavior.
