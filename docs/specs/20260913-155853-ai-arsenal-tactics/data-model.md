# Data Model: AI Arsenal Tactics

## AI difficulty

| Field | Meaning | Validation / lifecycle |
|---|---|---|
| level | Easy, Normal, or Hard | Exactly one valid value; defaults Normal; retained through controller changes and session/round copies. |
| owner | Configured player identity | Each AI has its own level. Human slots retain but do not apply/display it. |

## Tactical context

| Field | Meaning | Validation / lifecycle |
|---|---|---|
| actor / target | Current AI and a living opponent | Target is never actor; stable nearest-target tie-break. |
| distance / grouping | Horizontal separation and nearby opponents | Derived only from visible living tank positions. |
| wind | Current public battlefield wind | Informs judgement but reveals no future trajectory. |
| loadout | Actor's ammunition | Limited proposal must be available; Basic remains fallback. |

Lifecycle: constructed for one pure proposal; it cannot mutate tanks, cash, ammunition, or turn state.

## AI firing decision

| Field | Meaning | Validation / lifecycle |
|---|---|---|
| target | Intended living opponent | Valid at decision time. |
| weapon | Proposed weapon | Available/usable or Basic fallback. |
| aim | Azimuth, elevation, power | Existing bounds apply; difficulty affects error and ranking only. |

The shared firing boundary validates selection and consumes ammunition only after a successful shot.

## AI shopping plan

| Field | Meaning | Validation / lifecycle |
|---|---|---|
| candidates | Ordered limited-weapon purchases | No Basic/Nuke; at most 1/2/3 for Easy/Normal/Hard. |
| cash / loadout snapshot | Acting AI state | Favour affordable missing roles; authoritative purchase rechecks state. |

Each candidate goes through the active-shopper purchase operation. Rejection leaves state unchanged; the shopper advances once after the plan. Other participants are never part of this plan.
