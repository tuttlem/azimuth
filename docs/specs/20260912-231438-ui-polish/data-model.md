# Presentation Data Model

This feature adds view-model concepts only. Existing `GameSession`, loadouts, cash, weapon prices, aiming, wind, match state, and player configuration remain authoritative.

## AzimuthUiTheme

Shared presentation constants used by HUD and overlays.

| Field group | Purpose |
|---|---|
| Surface colours | Base, elevated, inset, and translucent overlay panels. |
| Semantic colours | Primary accent, player accent application, active/success, warning/damage, muted/disabled text and borders. |
| Typography scale | Display/title, section heading, body, compact label, and numeric-emphasis sizes/colours. |
| Layout tokens | Standard panel/card/button padding, gaps, border thickness, slot/card dimensions, and button heights. |
| Interaction styles | Normal, hovered, pressed, selected, and disabled visual treatment. |

Validation: values are presentation-only; semantic states must retain readable contrast and selected/disabled must differ by more than text colour.

## WeaponPresentationIdentity

Engine-neutral metadata indexed by `WeaponId` and resolved in Bevy UI into the associated loaded image.

| Field | Source/meaning |
|---|---|
| `weapon` | Stable `WeaponId` identity. |
| `compact_name` | Existing short readable HUD label. |
| `icon_key` / asset path | One project-owned icon under `assets/ui/weapons/`; no Bevy `Handle` leaks into domain metadata. |
| optional short description | Concise personality/help text for shop hover/detail presentation, if implemented. |

Relationships and rules:

- Every member of `ACTIVE_WEAPONS` resolves exactly one non-empty icon identity.
- Every member of `SHOP_WEAPONS` reuses its `ACTIVE_WEAPONS` icon identity.
- `BasicShell` remains in the HUD identity map but not in purchasable shop products and renders availability as unlimited.
- Removed/non-playable identities, including Bomb Net, have no active icon/card mapping.
- Missing identity/asset is reported clearly during development rather than yielding an empty interactive control.

## UiControlVisualState

Derived visual state, not stored gameplay state.

| State | Inputs | Result |
|---|---|---|
| Normal | Interactive, not selected | Standard surface/button treatment. |
| Hovered | Bevy `Interaction::Hovered` | Clear but restrained elevation/accent response. |
| Pressed | Bevy `Interaction::Pressed` | Immediate depress/flash feedback; domain action is requested once. |
| Selected | Authoritative selected `WeaponId` | Accent border/background/marker in addition to readable text. |
| Disabled | Zero limited ammo or unaffordable shop item | Muted/desaturated treatment, legible label/price/count, and no requested action. |

## ScreenPresentation

Read-only projection for existing screens.

| Screen | Reads | Action boundary |
|---|---|---|
| Tactical HUD | turn, tanks, aim, wind, selected weapon, loadout | Existing weapon/aim/move/fire requests. |
| Shop | active shopper, cash, catalogue, loadout, session phase | Existing `purchase` and `complete_active_shopper`. |
| Setup | match configuration and setup gate | Existing setup configuration/start requests. |
| Winner / accounting | match result, accounting resource, session phase | Existing continuation request only. |

No screen stores a duplicate wallet, inventory, winner, health, price, or phase. UI synchronisation always re-reads the authoritative value after an action.
