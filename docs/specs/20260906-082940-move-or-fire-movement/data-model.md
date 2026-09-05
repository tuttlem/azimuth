# Data Model: Move or Fire — Basic Tactical Movement

## Tanks

The existing two-tank collection becomes mutable authoritative match state. Each fixed player owns exactly one tank; only the current player can receive a changed pose.

| Field | Meaning | Invariant |
|-------|---------|-----------|
| Owner | Player One or Player Two | Exactly one tank per player. |
| Position | X/Z plus terrain-grounded Y | Every accepted step sets Y to current terrain height at destination. |
| Body forward | Normalized horizontal direction | Changes only after an accepted step. |
| Retained aim | Existing azimuth, elevation, and power | Independent from body direction; movement never resets it. |

Stationary tanks retain their existing pose after later deformation. Movement compares current terrain heights at both horizontal endpoints, rather than stale tank Y, and grounds every accepted destination.

## Movement request and result

| Item | Values | Invariant |
|------|--------|-----------|
| Direction | I: -Z; J: -X; K: +Z; L: +X | A request is one world-unit step; no diagonal exists. |
| Allowance | Integer 0–6 | Starts at six only when movement is selected; each accepted step costs one. |
| Maximum slope | 0.75 vertical units / one horizontal unit | Shared gameplay limit, not a tank statistic. |
| Accepted result | New grounded pose and one fewer step | Updates body forward and preserves turret/aim. |
| Rejected result | Bounds, slope, or no allowance | Preserves pose and allowance; provides feedback. |

## Turn action state

| State | Data | Allowed input | Transition |
|-------|------|---------------|------------|
| Choosing | Current player and two retained aims | Aim, M, Space | M begins Moving with six; Space begins Resolving Fire. |
| Moving | Current player, aims, remaining steps | I/J/K/L, Enter | Valid step decrements; Enter or zero hand off to the other player's Choosing state. |
| Resolving Fire | Current player and aims | None | Existing terminal projectile path deforms terrain before handoff. |

The state is authoritative; UI, projectiles, booms, and disabled controls only project it. A handoff alternates the two fixed players exactly once.

## Derived presentation

| Projection | Reads | Rule |
|------------|-------|------|
| Tank root | Current mutable position | Translate root to authoritative pose. |
| Tank body | Body forward | Orient body child only; leave turret independent. |
| Turret/barrel/muzzle | Tank position plus retained player aim | Continue existing canonical firing representation. |
| HUD | Action state, aim, remaining steps, latest result | Show player, controls, allowance, and concise rejection feedback. |
| Projectile/terrain/boom | Existing launch/terminal state | Unchanged except launch uses current tank pose. |
