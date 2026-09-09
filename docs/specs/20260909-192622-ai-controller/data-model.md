# Data Model: First AI Controller

## Authoritative state

| Entity | Core fields / invariants | Lifecycle |
|---|---|---|
| Controller designation | Existing Human or AI value on each configured stable player | Captured by setup; read during every current choosing turn |
| AI decision seed/cursor | Match-derived seed and monotonically advancing decision position | Reset when a configured match starts; advances only for a committed AI decision |
| AI firing decision | Acting player ID, living target ID, valid aim, normal weapon selection | Purely derived for the current AI choosing state; discarded if no longer current/valid |
| Current-player action | Existing legal launch request and its ordinary result | Validates choosing state, selected availability, active player, then commits weapon and begins fire |
| Tank collection | Existing ordered player-owned pose, health, and elimination state | Sole source for AI target eligibility and geometry |
| Turn state | Existing active ID, per-player aim, phase, and result | Receives chosen AI aim through the same legal aim state used by Human firing |
| Weapon loadouts | Existing independent selected weapon and availability per player | Receives ordinary selection and commitment; Basic Shell is always the baseline valid choice |

## Relationships and validation

- Configuration is the source of the active player's controller designation; display name is never
  a controller or randomness input.
- A decision may exist only for an in-progress, choosing, living AI player with no active flight.
- Target candidates are living tanks whose owner differs from the acting player. Choose the nearest
  horizontal candidate; equal distances resolve by stable configured order/identity.
- Decision aim is constructed through existing aim validation and remains within current elevation
  and power limits. Error is bounded and seeded; wind is deliberately ignored.
- A requested AI weapon must be normally available. Baseline policy selects Basic Shell; shared
  commitment still validates it, consumes according to normal rules, and captures its ordinary
  profile.
- Before action execution, revalidate setup-started, controller type, active player, in-progress
  choosing phase, no active flight, actor alive, and target eligibility. Invalid/stale decisions
  are discarded and replaced on the next valid dispatch opportunity.
- AI choice state is gameplay-affecting and match-seeded. HUD, camera, cloud, name-reroll, and
  other presentation/setup variation do not advance or read its stream.

## State transitions

```text
Match setup (Human/AI slots valid)
  → start: initialise tanks, turn, loadouts, dedicated AI decision state
  → Human current choosing turn: accept only Human tactical input
  → AI current choosing turn: presentation readiness → derive pure AI decision
  → revalidate → set ordinary current aim/select weapon → shared fire operation
  → ordinary resolving flight / impact / deformation / settling
  → next living configured player | winner | draw
```

An AI decision has no transition that directly damages, spawns, resolves, advances, or declares a
winner. Those transitions remain owned by existing ordinary gameplay.

## Presentation-only state

| Entity | Purpose | Restriction |
|---|---|---|
| AI turn readiness | Briefly marks that an active AI may act after its readable turn-start interval | May delay issuing the request but cannot change the decision, shot simulation, survivor state, or turn result |
| Existing HUD/camera | Shows active name, aim, weapon, wind, shot, and result | Reads existing configuration and running state; does not own AI action validity |

