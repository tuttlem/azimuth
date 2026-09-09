# AI Controller Internal Contract

This contract defines the narrow boundary between controller-specific decision sources and the
existing authoritative match actions.

| Operation | Preconditions | Request / result | Required behavior |
|---|---|---|---|
| Controller dispatch | Setup started; match in progress; current player in choosing phase | Current configured controller type | Human enables only Human tactical input; AI suppresses tactical Human input and becomes eligible for AI dispatch. |
| Derive AI firing decision | Current player is living AI; no active flight; at least one living non-self tank | Player ID, target ID, aim, requested normal weapon | Select only a living opponent. Use stable nearest-target selection, valid rough geometry aim, bounded match-seeded error, and ignore wind. No mutation occurs here. |
| Apply AI intent | Derived decision is still current and valid | Set existing current aim and select ordinary available weapon | Use existing aim and inventory validation. Discard stale/invalid intent; never act for eliminated or finished state. |
| Fire current player | Current player is choosing with an available selected weapon | Ordinary launch result / no result | One shared operation commits normal inventory, begins normal fire resolution, builds the normal projectile, and resets ordinary impact presentation. Human and AI requests meet here. |
| Resolve shot | Existing resolving flight | Existing impact/turn outcome | Existing wind, gravity, collision, damage, deformation, settling, survivor, winner, draw, HUD, and camera behavior applies unchanged. |
| Reset at match start | Valid configuration starts | New AI decision state | Initialise the dedicated match-derived AI seed/cursor. Setup AI name generation must not affect it. |

## Invariants

- There is no AI-specific tank, projectile, weapon, damage, terrain, physics, or resolution path.
- The controller designation is read from configured player data by stable ID, never display name.
- A Human input event cannot mutate an AI turn; an AI dispatcher cannot act in Human, resolving,
  finished, stale, or eliminated state.
- The baseline AI requests Basic Shell through ordinary selection/commitment. Future multi-weapon
  policy may reuse this contract without bypassing inventory.
- Determinism tests call decision derivation directly; presentation timing is not a test input.

