# Research: Minimal Turn Loop

## Decision: Use a compact pure two-player turn model

Create a focused gameplay model with a current existing `PlayerId`, a two-value phase
(`Ready` or `Resolving`), and exactly two retained `AimingState` values selected directly by player
identity. It starts as Player One/Ready, switches to the other player only from Resolving, and
retains each player's aim across the other player's turn.

**Rationale**: Existing `PlayerId::{One, Two}`, `AimingState`, and fixed two-tank setup already
express the necessary identities and aim validation. A pure model makes starting order, input
ownership, retained aims, and exact alternation straightforward to test independently of Bevy,
rendering, or input timing.

**Alternatives considered**:

- Infer readiness only from whether `ProjectileFlight` is present: rejected because it conflates
  projectile lifetime with explicit gameplay phase and does not make current-player ownership
  readable or independently testable.
- Keep one `ActiveAiming` value and copy/reset it on player change: rejected because it obscures
  ownership and risks overwriting the other player's retained settings.
- Use a collection supporting arbitrary players, a player manager, or generic turn actions:
  rejected because the feature has exactly two fixed participants and one action.
- Keep all state as unrelated resources in `main.rs`: viable but rejected because cross-resource
  invariants would need scheduler-coupled tests; one compact pure domain model earns its boundary.

## Decision: Complete turns synchronously in fixed-step terminal resolution

The existing `FixedUpdate` projectile advancement remains the sole authority for action completion.
For a terrain impact it will apply the crater, record the impact, clear the active projectile, and
then complete the turn in that order in the same gameplay path. For an out-of-bounds or lifetime
termination it will clear the projectile and complete the turn without impact state or terrain
change. An active projectile leaves the phase Resolving.

**Rationale**: Current terrain deformation already occurs at the terminal fixed step, before
`ProjectileFlight` is cleared. Coupling the player switch directly to that work ensures the next
player cannot act against terrain whose authoritative change is pending, and makes progression
independent of rendering frame timing.

**Alternatives considered**:

- Advance in an Update system after mesh or HUD refresh: rejected because presentation scheduling
  would become part of gameplay authority.
- Wait for the temporary explosion to expire: rejected because the boom is explicitly cosmetic and
  uses presentation time.
- Emit a generic completion event or use a chained generic workflow: rejected because one
  synchronous terminal outcome has no demonstrated need for indirection.

## Decision: Reuse one local input surface and current-player tank lookup

The established Q/E, R/F, T/G, Shift, and Space controls remain unchanged. Input reads only the
turn model's current player and Ready phase, changes that player's retained aim, and fires from
that player's existing tank firing representation. Both tanks receive an owner-associated visual
representation so the selected/current tank's turret, barrel, and muzzle follow the corresponding
retained aim; visuals never determine ownership.

**Rationale**: This preserves existing learnable controls and directly supports local hot-seat
play without duplicating Player One/Player Two input paths. The existing tank owner and firing
representation provide the required identity-to-launch mapping.

**Alternatives considered**:

- Separate per-player keyboard handlers: rejected because it duplicates controls and does not
  model hot-seat ownership.
- Let tank colour, active visual tags, or last projectile infer current player: rejected because
  presentation state is not authoritative gameplay state.
- Add a current-player camera focus: deferred; the existing manual battlefield camera is adequate.

## Decision: Test pure transitions and focused terminal-resolution ordering

Unit tests will exercise the pure turn model for initial state, current-player-only adjustments,
fire-to-resolution transition, rejected input during resolution, retained aims, and deterministic
alternation. Focused gameplay helpers will exercise `Active`, terrain-impact, and out-of-bounds
projectile outcomes, including crater-before-advance ordering and player-specific launch origins.
Manual validation will cover HUD and visuals without asserting renderer internals.

**Rationale**: Current unit tests already cover pure aim, projectile, terrain, and small input
helpers. Extending that style yields meaningful authoritative coverage without brittle scene,
text, or frame tests.

**Alternatives considered**:

- Screenshot, UI-text, or frame-delay tests: rejected because they would test presentation timing
  rather than authoritative turn behaviour.
- Full Bevy-world schedule tests for every invariant: rejected because pure domain tests express
  the rules more directly; a few focused integration helpers cover resolution ordering.
