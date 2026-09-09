# Shot Presentation Camera Contract

## Purpose

This internal game-presentation contract defines observable shot coverage. It is not a network or public API.

## Inputs

| Input | Contract |
|---|---|
| Firing configuration | Supplies stable firing player and Human/AI controller at launch. Display name is not a selector. |
| Active projectile | Supplies read-only position, velocity, and terminal state. |
| Authoritative aftermath | Supplies terrain impact, altered terrain, tank pose/health/elimination/settling, and match result. |
| Existing camera safeguards | Supply target bounds, pitch/distance limits, and interpolation. |

## Required behavior

| Condition | Presentation |
|---|---|
| Human launch | Human offset follow, not projectile-mounted coverage |
| Human ascent becomes descent | Widen once without exact frame or predicted landing point |
| AI launch | Broad tactical coverage, not Human follow |
| Either terrain impact | Same impact view, prioritising impact/crater/nearby affected tanks |
| Settling, elimination, winner, draw | Brief readable aftermath while authoritative resolution continues independently |
| Continuing match after hold | Normal next-survivor view |
| Final match after hold | Result view; never next player |
| Out-of-bounds or untrackable flight | Bounded safe fallback based on authoritative result |

## Invariants

- Camera selection uses controller type only, never name, input, target calculation, or accuracy.
- Presentation only reads projectile, terrain, tank, and turn state; it never mutates or delays their resolution.
- Human and AI have distinct flight coverage and one common impact coverage.
- Scoreboard, active-player identity, and match context remain useful; no HUD becomes authoritative.
- Coverage never exposes AI target choice, intended miss, solution, or predicted trajectory.
