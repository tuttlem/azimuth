# Feature Specification: First Arsenal — Extensible Weapon Framework, Selection and High Explosive

**Feature Branch**: `20260906-194157-first-arsenal`  
**Created**: 2026-09-06  
**Status**: Draft  
**Input**: User description: "Create the next Azimuth feature specification for First Arsenal — Extensible Weapon Framework, Selection and High Explosive."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Choose a Round Before Firing (Priority: P1)

As the active player, I can see and choose the round I will fire, so I can decide whether to use a
limited larger explosion or retain it for a better opportunity.

**Why this priority**: A visible, truthful choice is the gameplay value of the first arsenal; a
larger blast without selection or ammunition would not create a tactical decision.

**Independent Test**: Begin a choosing turn, select Basic Shell and High Explosive in turn, and
confirm the HUD shows the active player's selected weapon and truthful availability before firing.

**Acceptance Scenarios**:

1. **Given** a living active player is choosing an action, **When** they press `1` or `2`, **Then**
   the available Basic Shell or High Explosive respectively becomes that player's selected weapon
   without changing aim, movement allowance, turn ownership, or ammunition.
2. **Given** the active player selects a weapon, **When** the graphical tactical HUD is shown,
   **Then** it identifies the selected weapon and its ammunition using an unambiguous unlimited or
   remaining-round presentation.
3. **Given** a player has no High Explosive rounds remaining, **When** they attempt to select or
   fire it, **Then** no unavailable High Explosive shot is committed and the HUD remains truthful.
4. **Given** control passes between players, **When** each player becomes active, **Then** each
   sees their own retained selection and inventory, not the other player's state.

---

### User Story 2 - Spend High Explosive for a Bigger Consequence (Priority: P1)

As a player, I can fire High Explosive to create a visibly and tactically larger blast and crater,
so spending scarce ammunition is a meaningful choice.

**Why this priority**: The second weapon proves the arsenal is more than a renamed copy of the
existing shell and combines damage, terrain, and support systems already central to Azimuth.

**Independent Test**: Fire Basic Shell and High Explosive at comparable terrain locations and
confirm their recorded consequences use their respective blast and crater profiles while normal
wind, gravity, collision, damage, settling, and handoff still occur.

**Acceptance Scenarios**:

1. **Given** Basic Shell is selected, **When** it impacts terrain, **Then** it retains the current
   familiar 6-unit damage radius, maximum 40 damage, and 4.0-radius/1.8-depth crater.
2. **Given** High Explosive is selected, **When** it impacts terrain, **Then** it uses the common
   ballistic flight and impact sequence but produces a distinct 8-unit damage radius, maximum 60
   damage, and 6.0-radius/3.0-depth crater.
3. **Given** either weapon impacts beneath a living tank, **When** its crater removes enough
   support, **Then** existing terrain support and gravity settling resolve normally without a
   weapon-specific tank-displacement rule.
4. **Given** either weapon is fired in the current battlefield condition, **When** it is in flight,
   **Then** its trajectory continues to use the existing player aim, gravity, wind, collision, and
   deterministic timestep model.

---

### User Story 3 - Trust a Fired Weapon to Stay the Same (Priority: P1)

As a player, I can trust that the selected round shown when I fire is the round that resolves,
even if later state changes, so weapon results are understandable and reproducible.

**Why this priority**: Capturing a committed shot separates authoritative simulation from mutable
player selection and is the essential correctness boundary for future conventional weapons.

**Independent Test**: Commit a High Explosive shot, change or inspect player selection after shot
creation in a domain test, then resolve impact and verify it still applies High Explosive values
exactly once.

**Acceptance Scenarios**:

1. **Given** a valid selected weapon and a fire command, **When** the shot is committed, **Then**
   the authoritative in-flight shot records the weapon identity and all current projectile and
   impact values needed to resolve it.
2. **Given** High Explosive is committed, **When** it is later resolved, **Then** exactly one HE
   round has been spent and the impact uses the captured HE profile rather than any later player
   selection or inventory state.
3. **Given** firing cannot be committed because the game is resolving, finished, or the weapon is
   unavailable, **When** the player requests fire, **Then** no ammunition is spent and no shot is
   created.
4. **Given** an impact completes, **When** damage, terrain deformation, settling, elimination, and
   victory are evaluated, **Then** all resulting authoritative consequences complete before the
   next turn begins as they do for the existing shell.

---

### User Story 4 - Grow Ordinary Weapons Locally (Priority: P2)

As a developer, I can add a further conventional explosive round by defining it centrally and
making it available, so future arsenal work does not require scattered weapon-name rules.

**Why this priority**: This is the bounded architectural value required by an arsenal, while still
being proved by the two real weapons rather than speculative exotic-mechanic infrastructure.

**Independent Test**: Add a test-only third conventional definition with a distinct supported
profile and verify it can be selected, captured, and resolved through the same ordinary shot path
without a weapon-name condition in firing, flight, damage, deformation, or HUD mapping.

**Acceptance Scenarios**:

1. **Given** a stable ordinary weapon identity, **When** its central definition provides a display
   name, ammunition rule, and supported projectile and impact profiles, **Then** ordinary gameplay
   can obtain all required values from that definition.
2. **Given** a third conventional explosive definition, **When** its values differ only in existing
   supported parameters, **Then** it resolves through the same fire, flight, and impact flow as
   Basic Shell and High Explosive.
3. **Given** a future weapon needs a behaviour not represented by conventional explosive data,
   **When** that concrete weapon is proposed, **Then** the project may add the smallest explicit
   extension for that proven behaviour rather than predicting a universal weapon scripting model.

### Edge Cases

- Basic Shell is deliberately unlimited; its availability must not be represented by an arbitrary
  large number or decremented on fire.
- High Explosive begins with two rounds for each player. One player's use, exhaustion, or selection
  never changes the other player's inventory or selected weapon.
- Selection is accepted only during an in-progress choosing phase; moving, an active flight,
  settling, and a finished match reject it without mutation.
- If the final selected HE round is committed, selection falls back to an available Basic Shell
  before that player's next choosing turn; the committed shot remains HE.
- A no-impact shot still consumes its committed limited round once, but produces no invented blast
  or crater, matching existing out-of-bounds resolution behaviour.
- Simultaneous blast damage, self-damage, elimination, draw, and tank settling retain the existing
  authoritative order; weapon identity cannot cause presentation timing to influence results.
- Weapon display names are presentation metadata and cannot be used as inventory keys or firing
  decisions.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The game MUST establish stable explicit identities for ordinary weapons. Identity
  MUST be independent of display text, player inventory position, and HUD ordering.
- **FR-002**: The game MUST have one authoritative, centrally discoverable catalogue of current
  conventional weapon definitions. A definition MUST supply its identity, player-facing name,
  ammunition rule, supported projectile profile, impact profile, and any minimal presentation
  information required to distinguish its impact.
- **FR-003**: The catalogue MUST contain Basic Shell and High Explosive with unique identities.
  Basic Shell MUST preserve the existing ballistic, 6-unit/40-damage, 4.0-radius/1.8-depth crater
  behaviour. High Explosive MUST use an 8-unit/60-damage, 6.0-radius/3.0-depth crater profile.
  These are initial tuneable values, not final balance commitments.
- **FR-004**: Both initial weapons MUST use the existing ballistic flight model and therefore the
  current azimuth, elevation, power, gravity, horizontal wind, terrain collision, fixed simulation
  timestep, and projectile limits. This feature MUST NOT add projectile mass, drag, guidance,
  bounces, fuses, multiple projectiles, or weapon-specific wind resistance.
- **FR-005**: Each player MUST receive an independent authoritative weapon inventory containing
  unlimited Basic Shell and two High Explosive rounds. Unlimited availability MUST be represented
  explicitly rather than by a sentinel quantity.
- **FR-006**: Each player MUST retain an independent selected weapon. The default selection MUST
  be Basic Shell. Only the living active player in an in-progress choosing phase may select an
  available weapon; `1` selects Basic Shell and `2` selects High Explosive.
- **FR-007**: Selecting a weapon MUST not consume ammunition or change aim, movement, health,
  terrain, turn ownership, or current flight. An unavailable weapon MUST not become a deceptive
  selected firing state.
- **FR-008**: A successful fire command MUST validate the active selected weapon, capture the
  weapon identity and the required projectile/impact values into immutable authoritative fired-shot
  state, consume exactly one limited round if applicable, and then begin the existing resolving
  firing turn. Rejected fire commands MUST consume no ammunition and create no shot.
- **FR-009**: An in-flight shot MUST resolve from its captured profile, not mutable player
  selection, inventory, catalogue presentation, HUD, camera, or render timing. A committed HE shot
  MUST remain HE through flight and impact.
- **FR-010**: The common ordinary impact pipeline MUST use the captured impact profile to determine
  radial damage, crater radius/depth, and a proportionate existing-style explosion presentation.
  Gameplay blast radius MUST remain independent from visual scale.
- **FR-011**: Weapon-specific damage and crater consequences MUST resolve at the actual
  wind/gravity-influenced terrain impact and reuse existing deterministic damage, deformation,
  support, settling, elimination, victory, and turn-completion rules.
- **FR-012**: When a selected limited weapon becomes unavailable after committing its final round,
  the affected player MUST safely fall back to Basic Shell for a later choosing turn. The other
  player’s selection and inventory MUST remain unchanged.
- **FR-013**: The graphical tactical HUD MUST show the active player’s selected weapon and truthful
  ammunition while firing selection is relevant, using a clear unlimited form for Basic Shell and a
  remaining-round form for High Explosive. Concise selection controls MUST appear only where they
  are currently usable.
- **FR-014**: Ordinary conventional weapons that differ only in values already supported by the
  catalogue MUST use the common selection, fired-shot, flight, impact, and HUD path without
  weapon-name-specific conditional branches scattered across gameplay or presentation systems.
- **FR-015**: The feature MUST document the extension rule: do not generalise a weapon behaviour
  until a real weapon needs it. Future specialised weapons may earn a narrow explicit extension,
  but this feature MUST NOT introduce generic effect graphs, scripting, plugin loading, dynamic
  content packages, item/RPG inventories, or a speculative behaviour hierarchy.
- **FR-016**: Tests MUST cover catalogue identity and lookup, independent inventories/selections,
  unlimited and limited ammunition, selection and rejected-action non-consumption, shot snapshots,
  weapon-specific impact profiles, deterministic consequences, final-round fallback, and a third
  conventional test definition using the common path. Existing projectile, wind, gravity, damage,
  deformation, settling, victory, and turn-resolution coverage MUST remain healthy.
- **FR-017**: The feature MUST update player and architecture documentation plus only roadmap items
  whose acceptance criteria it genuinely satisfies. The workspace build, relevant tests,
  formatting, and lint checks MUST pass without unjustified warnings.

### Key Entities

- **Weapon identity**: Stable gameplay name for an ordinary weapon, used by catalogue lookup,
  inventory, selection, fired-shot recording, HUD mapping, and tests.
- **Weapon definition**: Central description of one conventional weapon’s player-facing metadata,
  availability rule, supported projectile profile, impact profile, and minimal presentation scale.
- **Weapon inventory**: One player’s authoritative quantity state keyed by weapon identity; it can
  explicitly represent an unlimited available weapon or a finite remaining quantity.
- **Selected weapon**: One player’s retained identity for the conventional round they will fire
  when a legal firing action is committed.
- **Fired shot**: Immutable committed shot state containing the selected weapon identity and the
  values required for its flight and impact to resolve independently of later player state.
- **Impact profile**: The conventional explosion’s damage radius, maximum damage, crater shape, and
  minimal visual scale, evaluated by the shared authoritative consequence sequence.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In manual local play, both players can select Basic Shell or available High Explosive
  in one keypress, and the HUD shows the selected name and truthful ammunition before firing.
- **SC-002**: In automated scenarios, 100% of Basic Shell impacts retain the existing 6-unit,
  40-damage, 4.0-radius/1.8-depth values, while 100% of HE impacts use 8-unit, 60-damage,
  6.0-radius/3.0-depth values at their resolved impact point.
- **SC-003**: In automated inventory traces, each committed HE shot decreases only its firing
  player’s HE quantity by exactly one; selecting or rejected firing decreases no quantity; Basic
  Shell remains available after every tested shot.
- **SC-004**: In automated snapshot scenarios, changing player selection or inventory after a shot
  is committed changes none of that shot’s recorded identity, flight, blast, damage, or crater
  consequence.
- **SC-005**: A test-only third conventional definition with different supported values reaches the
  common ordinary impact result without adding a weapon-name-specific branch to the core fire,
  projectile, damage, deformation, or HUD flow.
- **SC-006**: A full two-player duel remains playable through victory or draw with either weapon;
  projectile flight, deformation, tank settling, and handoff complete before the next turn, and
  none wait for camera or HUD presentation.
- **SC-007**: Required workspace checks complete without warnings, and manual validation confirms
  that High Explosive is visibly and tactically distinguishable from the dependable Basic Shell.

## Assumptions

- The current two-player local duel, move-or-fire rule, player aiming state, current terrain and
  damage models, graphical HUD, fixed projectile timestep, constant match wind, and
  presentation-only camera remain the authoritative starting state.
- `1` and `2` are unused by current tactical controls and are concise, non-conflicting direct
  selectors for Basic Shell and High Explosive. No configurable bindings are introduced.
- Basic Shell has unlimited ammunition and HE starts at two rounds per player because this provides
  repeated testing without making the larger blast an unlimited default answer.
- The initial HE values deliberately favour a clearly larger area and crater, not final balance;
  further tuning belongs to playtesting rather than this specification expanding into an arsenal.
- A conventional weapon presently needs no projectile properties beyond the existing launch
  parameters, while its impact needs blast, crater, and simple visual scale data.
- This feature depends on the existing projectile, combat, terrain, tank support, turn, and HUD
  behaviour, and on roadmap sections for projectile configuration, aiming, weapons, explosions,
  human-vs-human play, and HUD.

## Out of Scope

- Any weapon beyond Basic Shell and High Explosive, including heavy, precision, cluster, bouncing,
  rolling, tunnelling, terrain-building, delayed, guided, remote-detonated, or persistent weapons.
- Projectile mass, drag, weapon-specific wind sensitivity, atmosphere, direct tank collision,
  armour, shields, damage types, critical hits, status effects, or terrain blast occlusion.
- Shops, purchases, economy, pickups, random loadouts, configurable inventories, match setup,
  multiplayer/networking, persistence, runtime mods, external weapon packs, or scripting.
- A general item system, universal weapon traits, behaviour hierarchy, effect graph, plugin
  framework, weapon-specific sound, elaborate VFX, or final weapon balancing.
