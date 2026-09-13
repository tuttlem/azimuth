# Internal UI Presentation Contract

This is an internal contract between existing Azimuth domain/session state and Bevy presentation. It is not a network or public API.

## Weapon identity and artwork

1. `weapon::ACTIVE_WEAPONS` is the complete playable weapon source for the battlefield inventory.
2. `weapon::SHOP_WEAPONS` is the complete purchasable limited-ammunition source for the shop.
3. Every active identity resolves shared presentation metadata: readable compact name and one project-owned weapon icon asset.
4. Shop cards and battlefield slots must resolve icons from that same `WeaponId` metadata; they may use different display sizes but not duplicate identities.
5. Basic Shell has a HUD icon and renders `Unlimited`; it has no purchasable shop action. Bomb Net and any other removed identity must not be rendered.

## State projection

| Presented value | Authoritative source |
|---|---|
| Weapon availability/selected state | Current player loadout and existing selection state |
| Shop cash, price, affordability, owned ammunition | `GameSession`, weapon catalogue price, and active shopper loadout |
| Player colour/name/controller/elimination/health | Player configuration and live tank/session state |
| Aim and wind | Existing aiming and world/wind resources |
| Winner, accounting, and flow availability | Existing match/session/accounting resources |

Presentation cannot maintain a second copy of these values.

## Interaction behaviour

- Pressing an enabled weapon slot invokes the existing selection path only when the battlefield controls are active.
- Pressing an enabled Buy control invokes the existing atomic session purchase once. A failed/disabled purchase changes neither UI-owned nor domain state; the next synchronisation reflects authoritative cash and inventory.
- Pressing Done invokes the existing active-shop completion path once for the active human shopper.
- UI controls expose normal, hovered, pressed, selected where applicable, and disabled treatment. Disabled controls remain visibly informative but never request an action.
- UI audio is emitted only for meaningful successful actions (and optional clearly unavailable feedback), never as an authoritative state signal.

## Asset contract

- Weapon icon files live beneath `crates/azimuth-game/assets/ui/weapons/` and are source-owned/original or otherwise documented with clear licence provenance.
- Assets are loaded once through the normal Bevy asset server and referenced by presentation code.
- Development/test validation covers full active/shop metadata coverage. Missing paths/handles must make failures diagnosable rather than silently rendering blank controls.
