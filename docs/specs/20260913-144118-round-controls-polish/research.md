# Research: Round, Controls and Battlefield Polish

## Decision: Replace multiline accounting text with fixed compact rows

**Rationale**: The current session overlay builds six text lines per player, so seven/eight entries exceed desktop height. Spawn title, headers, up to eight compact presentation-only rows, and an Enter hint once; synchronise from `GameSession.players` and `GameSession.earnings`. Preserve `RoundEarnings::total()` and accounting-to-shop flow.

**Alternatives considered**: Shrinking one text block (poor grouping), scrolling/pagination (violates simultaneous visibility), and two-column cards (poor comparison/long-name behaviour) were rejected.

## Decision: Change the canonical price table only

**Rationale**: `weapon::weapon_price` drives shop display, affordability, and `GameSession::purchase`. Change nine non-Nuke entries to $300, $250, $600, $550, $400, $700, $350, $400, and $350; retain Nuke at $15,000 and Basic Shell without price.

**Alternatives considered**: A multiplier layer is needless indirection; session/UI changes risk display/charge divergence.

## Decision: Make move mode fieldless and explicit-completion only

**Rationale**: Replace `TurnPhase::Moving { remaining_steps }` with `TurnPhase::Moving`. Accepted steps retain move mode; explicit completion is the sole handoff. Keep in-bounds validation/terrain-height grounding in `Tank::step_on_terrain`, but remove elevation comparison/slope rejection.

**Alternatives considered**: High allowance remains finite; vector movement adds unrequested pointer/pathfinding/collision scope; nonblocking slope feedback preserves obsolete complexity.

## Decision: Accept Space/Enter without accidental firing

**Rationale**: `update_movement_input` recognises both keys. Scheduling/input guarding must prevent Space completion from launching the next active player's weapon in the same frame. Camera-relative cardinal arrow semantics stay unchanged.

**Alternatives considered**: Enter alone misses requested convenience; confirmation UI adds friction.

## Decision: Map Tab to the existing setup transition

**Rationale**: `update_match_setup` already invokes `MatchConfiguration::set_controller`, preserving identity/visual identity and established name/validation behaviour. Change only the trigger/guidance from Control-C to Tab.

**Alternatives considered**: New setup widget is redundant; another modifier chord retains the reported problem.

## Decision: Brighten existing wind-arrow material

**Rationale**: Direction/camera logic works. Improve shaft/head contrast with high-brightness/emissive or unlit material, optionally tune dedicated light; retain geometry, viewport, rotation, and wind simulation.

**Alternatives considered**: Changing wind meaning risks gameplay; moving/enlarging/duplicating indicator causes unrelated HUD churn.

## Decision: Blend existing visual horizon rather than expand terrain

**Rationale**: `VisualHorizon` already provides a render-only skirt, but inner palette/material differs from active terrain. Match its inner ring to terrain surface colour, attenuate colours outward, and use a consistent terrain-compatible material on initial/later rounds. Combat terrain, collision, deformation, and bounds remain fixed.

**Alternatives considered**: Larger horizon hides but does not fix seam; terrain expansion changes gameplay/performance; fog/shaders are disproportionate.
