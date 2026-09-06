# Feature Specification: First Duel — Damage, Elimination and Victory

**Feature Branch**: `20260906-102046-first-duel-damage`

**Created**: 2026-09-06

**Status**: Draft

**Input**: User description: "Create the next Azimuth feature specification for First Duel — Damage, Elimination and Victory."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Hurt Tanks With Accurate Shots (Priority: P1)

Two local players can use the existing artillery loop to inflict understandable splash damage on
either tank, including themselves, when a terrain impact explodes nearby.

**Why this priority**: A readable consequence for a well-placed shot is the minimum missing piece
between the current artillery sandbox and an actual duel.

**Independent Test**: Resolve terrain impacts at fixed distances from one or both tank positions
and verify the resulting health changes use the documented distance rule, remain bounded, and are
identical for identical scenarios.

**Acceptance Scenarios**:

1. **Given** both tanks begin at full health, **When** an impact occurs outside the 6-unit damage
   radius, **Then** neither tank's health changes.
2. **Given** an impact occurs within the radius, **When** it is closer to a tank than a second
   otherwise equivalent impact, **Then** it applies greater or equal damage to that tank.
3. **Given** both tanks are inside one impact radius, **When** the explosion resolves, **Then**
   both are evaluated from the same impact and each receives its independently calculated damage.
4. **Given** the firing tank lies within its own impact radius, **When** its projectile resolves,
   **Then** it receives the same distance-based damage rule as every other tank.

---

### User Story 2 - Eliminate a Player and Continue Only With Survivors (Priority: P1)

When damage reduces a tank to zero health, players can clearly see that it is out, and an ongoing
match never grants that eliminated player another move-or-fire turn.

**Why this priority**: Damage has no tactical meaning unless zero health changes who may act.

**Independent Test**: Apply lethal damage to one tank while the other remains alive; verify the
eliminated player cannot aim, move, or fire, their visible tank is clearly inert, and the surviving
player remains the only possible current player until match completion.

**Acceptance Scenarios**:

1. **Given** a tank has health above zero, **When** accumulated damage reaches or exceeds its
   remaining health, **Then** health becomes exactly zero and the tank is eliminated.
2. **Given** one player is eliminated and the other survives, **When** a shot has fully resolved
   but the match has not otherwise finished, **Then** ordinary turn selection chooses only a
   surviving player.
3. **Given** a player is eliminated, **When** aim, move, or fire input is requested for that
   player, **Then** it produces no gameplay action.
4. **Given** a tank is eliminated, **When** the battlefield is viewed, **Then** it is clearly
   shown as out of the duel without requiring destruction animation or physics.

---

### User Story 3 - Finish and Announce the Duel (Priority: P2)

When an impact leaves exactly one survivor, the duel ends immediately after authoritative damage
resolution and clearly announces the winner. When one impact eliminates every remaining tank, the
duel ends as a draw.

**Why this priority**: A clear ending makes the two-player loop a complete playable match rather
than an endless target practice session.

**Independent Test**: Resolve lethal single-survivor and mutual-elimination impacts and verify the
stored result, visible result text, and rejection of all ordinary actions after completion.

**Acceptance Scenarios**:

1. **Given** an impact eliminates one tank while exactly one remains alive, **When** all explosion
   gameplay consequences resolve, **Then** the match is finished and names the surviving player as
   winner before any subsequent turn begins.
2. **Given** the same impact reduces all remaining tanks to zero health, **When** it resolves,
   **Then** the match finishes as a draw rather than selecting an arbitrary winner.
3. **Given** a match is finished, **When** the user attempts aiming, movement, or firing, **Then**
   no ordinary turn action or projectile launch occurs and the result remains visible.

### Edge Cases

- Damage at or beyond the 6-unit radius is zero; any nonzero distance strictly inside the radius
  produces the documented reduced whole-number damage.
- Health never becomes negative, and applying damage to an already eliminated tank changes neither
  health nor match result.
- Every tank's damage for one explosion is calculated before health/elimination mutations decide
  the outcome, so one tank's resolution cannot affect another tank's eligibility for that blast.
- A projectile that leaves the existing useful simulation volume without terrain impact causes no
  explosion damage and retains the current no-impact handoff behavior while the match continues.
- A terrain impact that ends the match does not wait for the boom visual or camera transition.
- Existing terrain change beneath a stationary tank remains outside scope; blast distance uses the
  current authoritative tank base position without tank settling or cover/occlusion rules.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Each of the two starting tanks MUST have 100 maximum health and begin every new
  application run at 100 current health. Health MUST be clamped to the inclusive range 0–100.
- **FR-002**: A terrain impact MUST create one authoritative explosion gameplay result centred on
  the resolved impact position, independent of visual explosion scale, duration, camera state, or
  renderer geometry.
- **FR-003**: The explosion gameplay result MUST use a shared, documented 6-world-unit damage
  radius and maximum damage of 40 health. These are explicit tuneable gameplay values, not tank
  classes or weapon variants.
- **FR-004**: Damage MUST use full three-dimensional world-space distance from impact centre to a
  tank's authoritative base position. For distance `d` strictly inside the radius `r`, damage MUST
  equal `ceil(40 * (1 - d / r))`; at or beyond `r`, damage MUST be zero. This is the initial
  linear falloff model.
- **FR-005**: An impact MUST evaluate all tanks using the same impact centre and their pre-damage
  positions before applying any resulting health changes. The rule MUST include the firing tank;
  no shooter immunity, terrain occlusion, direct-hit collision, impulse, or cover reduction is
  introduced.
- **FR-006**: Damage, health changes, elimination state, terrain deformation, survivor selection,
  and match result MUST resolve authoritatively before turn advancement. Camera and boom timing
  MUST never delay or decide any of those outcomes.
- **FR-007**: A zero-health tank MUST be marked eliminated, shown with a simple clearly inert
  battlefield presentation, and excluded from all future aiming, moving, firing, and ordinary
  turn selection.
- **FR-008**: The game MUST represent the match explicitly as either in progress or finished with
  `Player One wins`, `Player Two wins`, or `Draw`. It MUST determine the result from survivors
  after every damaging explosion: one survivor wins; no survivors draw.
- **FR-009**: When a match is finished, it MUST stop turn advancement and reject all ordinary
  aiming, movement, and firing actions while keeping the final result visibly available. Restarting
  the application is sufficient to begin another duel.
- **FR-010**: While a match is in progress, advancement after movement or a non-lethal shot MUST
  select the next surviving player rather than blindly alternating to an eliminated player. The
  current two-player representation MUST remain simple; no generic multiplayer scheduler is
  required.
- **FR-011**: The minimal HUD MUST show both players' current and maximum health, identify an
  eliminated player, and prominently show the winner or draw when finished. It MUST retain current
  move-or-fire/action feedback for a living active player.
- **FR-012**: Existing move-or-fire rules, retained per-player aim, terrain collision/deformation,
  fixed projectile simulation, keyboard controls, and presentation-only camera behavior MUST
  remain intact for every in-progress duel.
- **FR-013**: Automated tests MUST cover starting health, radius boundary, linear falloff,
  clamping, self-damage, multi-tank/same-event evaluation, deterministic results, elimination,
  next-survivor selection, winner/draw determination, finished-match input rejection, and the
  invariant that camera/visual timing cannot change damage or match outcomes. Tests MUST target
  gameplay rules rather than renderer entities or HUD text where practical.
- **FR-014**: The feature MUST document health, damage values, distance/falloff, self-damage,
  elimination, and victory behavior; it MUST update only roadmap items proven complete. Workspace
  build, relevant tests, formatting, and linting MUST pass without unjustified warnings.

### Key Entities

- **Tank survival state**: A tank's shared maximum health, current clamped health, and eliminated
  status derived when health reaches zero.
- **Explosion gameplay result**: The authoritative terrain-impact centre plus shared radius and
  maximum-damage values used to calculate one blast's tank damage.
- **Damage resolution**: The deterministic set of all tank damage amounts calculated from one
  explosion before health states are updated.
- **Match state**: The authoritative in-progress or finished result: Player One win, Player Two
  win, or draw.
- **Survivor selection**: The direct two-player decision that chooses the next living player only
  while an in-progress match has survivors.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In automated fixed-scenario tests, 100% of blast damage values match the documented
  6-unit, 40-maximum linear-falloff rule and leave health within 0–100.
- **SC-002**: In manual play, a player can distinguish both health totals, an eliminated tank, and
  the final winner or draw from the normal game view without developer inspection.
- **SC-003**: In a local duel, two or three sufficiently accurate centre-near impacts can eliminate
  a full-health tank, while impacts outside the radius cause no damage and near-edge impacts cause
  visibly smaller damage.
- **SC-004**: In automated lethal and mutual-elimination scenarios, 100% of outcomes stop ordinary
  actions after resolution and report the same winner or draw for identical initial state, tank
  positions, and impact sequence.
- **SC-005**: Existing quality commands complete successfully and manual validation confirms at
  least one full local duel from opening turn to a clear winner or draw.

## Assumptions

- The existing terrain-impact point is the first authoritative explosion centre. Projectiles do
  not yet collide directly with tank geometry, so a terrain impact near a tank supplies the first
  useful direct-hit-like outcome.
- The shared 100 health, 6-unit radius, and 40 maximum damage are initial development tuning:
  centre-near shots need roughly three impacts to eliminate a full-health tank, while near misses
  are useful but not automatically lethal.
- Damage is a whole health amount using the specified ceiling rule so every strictly in-radius
  impact has a visible consequence; values are expected to be revisited after real duels.
- The eliminated-tank presentation may hide or visibly disable the existing placeholder; wrecks,
  animation, sound, smoke, and terrain-driven settling remain later work.
- A draw is the deterministic outcome when an explosion eliminates all remaining tanks. No restart,
  rematch, match setup, AI, weapons, armour, wind, or additional match modes are part of this
  feature.
- This feature depends on the existing two-player tanks, impact/deformation resolution, turn
  phases, HUD, and presentation-only camera. It advances the first playable duel without adding a
  general combat or match framework.
