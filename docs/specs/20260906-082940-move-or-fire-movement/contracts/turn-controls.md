# Turn and Movement Controls Contract

This is the observable desktop local-player control contract, not a network or library API.

| State | Display | Accepted controls | Required result |
|-------|---------|-------------------|-----------------|
| Choosing | Current player, aim, move/fire choice | Q/E, R/F, T/G, Shift, M, Space | Aim changes only current player. M starts six-step movement; Space starts established shot resolution. |
| Moving (1–6) | Current player and remaining steps | I/J/K/L, Enter | Valid request changes only active tank, grounds it, faces body in requested direction, and consumes one. Enter forfeits rest and hands off. |
| Moving (0) | Movement exhausted | None | Immediately hand off once. |
| Resolving Fire | Current shot is resolving | No action controls | Selection, aim, movement, and fire are ignored until terminal resolution hands off. |

| Key | X/Z offset | Acceptance |
|-----|------------|------------|
| I | `(0, -1)` | In bounds and endpoint elevation difference ≤ 0.75. |
| J | `(-1, 0)` | In bounds and endpoint elevation difference ≤ 0.75. |
| K | `(0, +1)` | In bounds and endpoint elevation difference ≤ 0.75. |
| L | `(+1, 0)` | In bounds and endpoint elevation difference ≤ 0.75. |

At most one direction is selected from a frame; opposing or ambiguous simultaneous inputs make no request. Rejections identify bounds, slope, or allowance and preserve pose, facing, and budget. Camera WASD/arrows, right-drag orbit, and scroll zoom remain presentation-only.
