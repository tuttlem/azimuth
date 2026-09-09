# Research: First AI Controller

## Decision: Use a pure firing-first decision policy

**Rationale**: The current game already has a complete ordinary firing lifecycle and the feature
accepts firing-only AI. Deferring movement avoids creating navigation, terrain path evaluation, or
a second action implementation. The policy returns a target, valid aim, and normal weapon choice;
it never owns projectile, combat, terrain, or turn resolution.

**Alternatives considered**:
- Add occasional movement now: rejected because legal-direction choice and ending movement expand
  dispatch and recovery paths without being required for a first playable AI.
- Build a generic action/event framework: rejected because the existing game has a small fixed
  move-or-fire surface and only needs one shared launch boundary.
- Create AI-specific game entities: rejected because controller type is a decision source, not
  gameplay identity.

## Decision: Target the nearest living non-self tank with stable identity tie-breaking

**Rationale**: The existing tank collection holds current positions and elimination state. Nearest
targeting is understandable, needs no hidden strategic evaluation, handles 1–7 opponents, and can
be made reproducible by stable player identity tie-breaking.

**Alternatives considered**:
- Random living target: valid but less immediately understandable and adds random-stream use for a
  choice that does not need it.
- Threat, terrain, or weapon evaluation: rejected as explicitly out of scope.
- Exact target centre plus exact solution: rejected because it would make the first AI superhuman.

## Decision: Generate rough geometry-based aim with bounded seeded error and ignore wind

**Rationale**: Existing azimuth conversion supplies the coordinate convention. Horizontal
separation can map monotonically to a clamped launch-speed heuristic, while a preferred mid-range
elevation plus bounded elevation, azimuth, and power perturbations produces recognisably aimed but
fallible shots. Ignoring wind preserves the roadmap's future wind-compensation capability and lets
ordinary wind create entertaining misses.

**Alternatives considered**:
- Exact ballistic equation or simulated trajectory search: rejected by feature boundary.
- Wind compensation: deferred; the baseline must be deterministic but need not correct wind.
- Fully random aim/power: rejected because it does not generally look target-directed.

## Decision: Use a dedicated match-derived AI decision seed and advance it only when a decision is committed

**Rationale**: The existing labeled seed derivation isolates terrain, starts, dressing, and wind.
A similarly labeled AI seed plus a small decision cursor makes error repeatable for the same match
history. It keeps setup AI-name selection and visual presentation from affecting authoritative
choices. Stable ID and living tank state form the decision input; display names do not.

**Alternatives considered**:
- System or thread randomness: rejected because it breaks reproducibility.
- Reuse terrain/wind/dressing streams: rejected because unrelated changes could alter AI behavior.
- Derive only from current player: rejected because repeated turns could repeat identical errors.

## Decision: Extract a shared current-player firing operation and gate source-specific input

**Rationale**: The current launch system validates keyboard input, commits selected inventory,
begins fire, constructs the ordinary projectile, and resets impact state in one place. Split only
the input detection from a shared legal firing operation. Human and AI then converge before weapon
commitment and projectile construction, while each source is prevented from modifying the other
controller's turn.

**Alternatives considered**:
- Have AI synthesize keyboard events: rejected because it couples decisions to presentation input.
- Duplicate the launch sequence for AI: rejected because inventory and physics could diverge.
- Refactor all gameplay into a generic command architecture: rejected as unnecessary abstraction.

## Decision: Use a presentation-only short readiness state, with pure tests calling decisions directly

**Rationale**: A small readable pause lets the active AI name and camera focus register before a
shot. It is not authoritative: action validity is rechecked on execution, and deterministic unit
tests exercise the pure policy and dispatch state without waiting for elapsed time.

**Alternatives considered**:
- Fire immediately on the same update: technically valid but less readable in all-AI matches.
- Make authoritative turns depend on a cinematic timer: rejected because presentation must not own
  gameplay and test behavior should not depend on wall-clock timing.

