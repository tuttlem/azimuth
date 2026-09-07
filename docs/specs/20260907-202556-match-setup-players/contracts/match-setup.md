# Match Setup Contract

This internal contract defines the boundary between Match Setup presentation and authoritative match creation.

| Operation | Input | Result | Required behaviour |
|---|---|---|---|
| Default configuration | None | Two-slot configuration | Player 1 and Player 2 are Human with unique IDs/visuals and are start-valid. |
| Set count | Requested count | Updated configuration/error | Accept 2–8 only. Growing creates default Human slots; shrinking removes excess prospective slots. |
| Edit name | ID, text | Updated configuration/error | Trim, require non-empty, max 20 render-safe characters; preserve ID/colour. |
| Set controller | ID, Human/AI | Updated configuration | Preserve ID/colour. Human → AI assigns captured pool name; AI → Human has valid editable name. |
| Reroll AI name (optional) | ID | Updated configuration | AI only; changes only display name; does not affect identity, other slots, or gameplay RNG. |
| Validate/start | Configuration | Runtime input/errors | Reject invalid configuration and any AI slot with explicit AI-not-yet-available reason. |

For every validated entry, runtime creation establishes matching ID/name/controller/visual identity plus deterministic supported spawn/orientation, health/alive state, default aim/movement, standard Basic Shell/High Explosive/Heavy Shell loadout, and slot-ordered turn participation.

Presentation shows Match Setup at launch, shows slot colour/name/controller, and enables Start only for valid all-human configuration. Tactical HUD lists collection-derived player status and resolves winner ID to configured name. Camera may animate to active tank but cannot delay or decide turns.
