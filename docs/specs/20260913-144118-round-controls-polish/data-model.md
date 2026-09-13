# Data Model: Round, Controls and Battlefield Polish

## Authoritative concepts

### Shop price list

| Field | Rule |
|---|---|
| Weapon identity | Existing stable limited `WeaponId` values are unchanged. |
| Non-Nuke price | Each of nine purchasable non-Nuke prices equals prior price ÷ 10. |
| Nuke price | Fixed at $15,000 and more than twenty times ordinary maximum. |
| Basic Shell price | Absent; it remains unlimited/unpurchasable. |

The one `weapon_price` lookup drives products, rendered price/affordability, and atomic charging.

### Movement turn

| Field | Rule |
|---|---|
| Turn phase | `Choosing`, fieldless `Moving`, fire resolution, or finished. |
| Current player | Existing stable identity; only this tank moves. |
| Requested step | Existing one-unit camera-relative cardinal direction. |
| Accepted destination | In bounds; vertical position is current terrain height. |
| Rejection | Bounds only; preserves tank and move mode. |
| Completion | Space or Enter requests one normal handoff. |

Transition: `Choosing → Moving` on Move; `Moving → Moving` per accepted step; `Moving → Choosing` for next survivor only on explicit finish. Fire/aim remain unavailable while moving.

### Setup controller selection

Existing selected-slot index receives Tab and invokes existing `set_controller`, retaining identity/visual identity and established name/validation rules.

## Presentation-only concepts

### Accounting row

| Field | Source |
|---|---|
| Player identity/name/colour | Matching session player configuration |
| Damage and placement income | Matching `RoundEarnings` |
| Total | Existing `RoundEarnings::total()` |
| Wallet | Matching session player cash |
| Visibility | Row index below player count and phase is Accounting |

Eight fixed row entities own no cash, earnings, phase, or identity state.

### Tactical wind cue and battlefield surround

Arrow retains direction/geometry/camera semantics and gains contrast-only materials/light. The render-only horizon matches terrain edge colour and blends toward scenery; it cannot participate in collision, terrain queries, spawning, movement, projectiles, or deformation.
