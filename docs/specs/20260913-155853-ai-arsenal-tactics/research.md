# Research: AI Arsenal Tactics

## Difficulty ownership and setup control

**Decision**: Store Easy, Normal, or Hard in every player configuration, defaulting to Normal. Display and edit it only for a selected AI slot. `D` cycles difficulty for that slot and its value survives a Human/AI toggle.

**Rationale**: Player configuration already survives match creation and round transitions. Contextual `D` avoids existing count, slot, controller-toggle, and Human name-editing controls.

**Alternatives considered**: A global difficulty breaks mixed matches; runtime-only state complicates retention; arrows, Tab, and number keys already have setup roles.

## Tactical decision policy

**Decision**: Extend the pure firing decision with difficulty, acting loadout, and current wind. Derive nearest living target, distance, target grouping, wind, and legal weapons from public state, then use a fixed ranking and stable tie-break.

**Rationale**: The existing decision already returns a weapon but hard-codes Basic Shell. Loadout prevents depleted selections; wind supports Heavy Shell; grouping supports MIRV/Cluster. A small explicit policy is readable and does not require a trajectory solver.

**Difficulty policy**: Easy has higher bounded aim error and simple range choices; Normal uses range, grouping, and wind; Hard uses the same visible data with lower error and more reliable ranking. No level receives hidden state, extra actions, cash, or ammunition.

**Initial weapon roles**: Basic is universal fallback; HE is close splash; Heavy is wind-resistant/direct; MIRV/Cluster are for grouping; Roller is close terrain-following. Bunker Buster, Dirt Bomb, Curve Ball, and Bouncer remain legal but need not be primary automatic choices. Nuke is never automatic in this slice.

**Alternatives considered**: Global weapon tactical metadata is premature catalogue abstraction; exhaustive shot simulation is expensive and out of scope; runtime hard-coding is not inventory-aware or directly testable.

## AI shopping policy

**Decision**: Produce a pure bounded shopping plan from difficulty, cash, and loadout. Submit every proposal through `GameSession::purchase`, then complete the existing active shopper once. Shopping uses no random draws.

**Rationale**: The existing purchase operation atomically checks phase, shopper, price, cash, and inventory. Keeping policy outside it protects that boundary and makes fallbacks testable.

**Policy bounds**: At most 1/2/3 candidates for Easy/Normal/Hard; limited affordable weapons only; favour missing useful roles; never Basic Shell or Nuke. An unaffordable candidate must not prevent later affordable candidates.

**Alternatives considered**: The fixed HE-then-Roller loop stops prematurely and is not varied; direct mutation bypasses transaction safety; sharing the tactical random stream adds needless call-order coupling.

## Determinism and integration

**Decision**: Keep the existing labelled tactical seed solely for firing. Keep shopping state-derived. Apply tactical proposals via existing loadout selection/shared firing and purchases via existing session purchase.

**Rationale**: This prevents UI or shopping call order from perturbing reproducible tactical behaviour and keeps presentation out of game-rule ownership.
