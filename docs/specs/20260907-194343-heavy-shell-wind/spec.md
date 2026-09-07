# Feature Specification: Heavy Shell Wind Resistance

**Feature Branch**: `feature/heavy-shell-wind-resistance`

**Created**: 2026-09-07

**Status**: Draft
**Input**: Add Heavy Shell, a limited conventional projectile whose ordinary ballistic flight is noticeably less affected by horizontal wind.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Keep a Shot on Line in Strong Wind (Priority: P1)

As a player facing a strong crosswind, I can select Heavy Shell and fire it with the familiar aim controls, so I can spend limited ammunition for a more predictable trajectory rather than attempting to compensate for as much drift with a normal shell.

**Why this priority**: This is Heavy Shell's defining tactical value. Without a visible, learnable reduction in wind drift, it is only a third label in the weapon selector.

**Independent Test**: From identical launch position, azimuth, elevation, power, gravity, wind, terrain, and fixed-step duration, compare a Basic Shell and a Heavy Shell. The Heavy Shell has a smaller horizontal displacement attributable to wind, but still moves in the wind direction.

**Acceptance Scenarios**:

1. **Given** an in-progress choosing turn and Heavy Shell ammunition remaining, **When** the active player selects Heavy Shell and fires, **Then** the projectile follows the existing ordinary azimuth/elevation/power ballistic arc and the displayed Heavy Shell count decreases by exactly one.
2. **Given** equal ordinary and Heavy Shell launches in a noticeable crosswind, **When** both advance for the same fixed simulation duration, **Then** Heavy Shell is displaced less across the intended line than Basic Shell while both are displaced toward the wind.
3. **Given** zero wind and otherwise identical launch inputs, **When** Basic Shell and Heavy Shell advance through the same fixed steps, **Then** their position and velocity traces are equivalent within the project's established numeric tolerance.
4. **Given** equal-strength reversed winds, **When** a Heavy Shell is fired under each condition, **Then** its horizontal influence reverses direction and remains smaller in magnitude than the corresponding Basic Shell influence.

---

### User Story 2 - Use a Third Conventional Weapon Naturally (Priority: P1)

As a player, I see Heavy Shell in the existing weapon-selection and tactical HUD flow, so I can understand its remaining rounds and use it without learning a special control scheme.

**Why this priority**: The feature validates the First Arsenal framework: a conventional weapon must join normal selection, availability, commitment, HUD, and exhaustion behaviour rather than create a parallel firing path.

**Independent Test**: Give both players their default loadouts, select Heavy Shell through the normal selector, fire it, exhaust it, and verify the existing generic selection and fallback rules apply independently to each player.

**Acceptance Scenarios**:

1. **Given** a new two-player match, **When** either player reaches a choosing turn, **Then** that player has unlimited Basic Shell, two High Explosive rounds, and two Heavy Shell rounds independently of the other player.
2. **Given** an available Heavy Shell, **When** the active player selects it without firing, **Then** no ammunition, aim, movement, terrain, health, turn, or flight state changes.
3. **Given** Heavy Shell is selected, **When** the player fires it, **Then** exactly one of only that player's Heavy Shell rounds is committed and the HUD reports the new truthful remaining count on a later choosing turn.
4. **Given** the final selected Heavy Shell round is committed, **When** the player next has a choosing turn, **Then** generic availability behaviour has returned selection to available Basic Shell and prevents selecting exhausted Heavy Shell.
5. **Given** a player is moving, a projectile is resolving, a tank is settling, or the match is finished, **When** a Heavy Shell selection input is attempted, **Then** it is rejected without state mutation.

---

### User Story 3 - Preserve the Ordinary Impact Game (Priority: P2)

As a player, I can use Heavy Shell without it unexpectedly becoming a stronger explosive, special terrain tool, or special direct-hit rule, so the meaningful choice remains wind resistance rather than hidden mechanics.

**Why this priority**: Heavy Shell should complement the unlimited general-purpose round and larger High Explosive blast without replacing either role.

**Independent Test**: Resolve a Heavy Shell terrain impact at a controlled location and verify it uses the ordinary collision, damage, crater, tank-settling, elimination, victory, and turn-completion sequence with the Basic Shell-class impact profile.

**Acceptance Scenarios**:

1. **Given** a Heavy Shell intersects terrain, **When** the swept ordinary terrain collision resolves, **Then** it produces one ordinary terrain impact and no bounce, penetration, split, guidance, or extra projectile.
2. **Given** equal controlled terrain impacts by Basic Shell and Heavy Shell, **When** their consequences resolve, **Then** Heavy Shell uses the established Basic Shell impact values unless playtesting documents a restrained, deliberately different value, and it does not use High Explosive's larger blast or crater role.
3. **Given** a Heavy Shell impact deforms terrain beneath a living tank, **When** authoritative consequence resolution completes, **Then** existing damage, support reconciliation, gravity-driven settling, elimination/draw/winner evaluation, and turn handoff remain correct.

---

### User Story 4 - Extend Projectile Behaviour Without Weapon-Name Logic (Priority: P2)

As a developer, I can express Heavy Shell's wind resistance as a supported projectile characteristic captured at firing time, so common projectile simulation need not know the weapon is named Heavy Shell.

**Why this priority**: This is the bounded architectural proof that real weapon behaviour can evolve the weapon framework without speculative mass, drag, or a second firing system.

**Independent Test**: Inspect a committed Heavy Shell shot and advance it through common flight alongside Basic Shell; verify its authoritative projectile state contains its configured wind response, the simulation applies that generic value, and later selection/catalogue changes cannot alter flight.

**Acceptance Scenarios**:

1. **Given** each catalogue definition, **When** its projectile profile is read, **Then** Basic Shell and High Explosive specify normal wind response and Heavy Shell specifies a substantially reduced, positive response.
2. **Given** a committed Heavy Shell shot, **When** the player changes selection or its inventory changes afterward, **Then** the in-flight projectile retains the Heavy Shell wind response and impact profile captured at commitment.
3. **Given** any conventional projectile with a configured wind response, **When** common fixed-step simulation advances it, **Then** gravity is applied unchanged and horizontal wind is scaled only by that generic projectile characteristic, with no condition based on Heavy Shell identity or display name.
4. **Given** strong valid wind, **When** Heavy Shell advances, **Then** stronger wind causes more Heavy Shell displacement than weaker wind; it is resistant rather than immune.

### Edge Cases

- Wind response is finite, non-negative, and valid only for a projectile that can be simulated; invalid configuration is rejected rather than producing undefined flight.
- A response of zero, if the generic domain permits one, would mean no wind contribution but is not Heavy Shell's configured behaviour; Heavy Shell must use a positive reduced response.
- Reduced response applies equally to both horizontal axes, including crosswind, aligned wind, and every reversed direction; it must never introduce a vertical wind component.
- Heavy Shell uses the same gravity acceleration as Basic Shell and High Explosive. Its name must not imply a changed gravity trajectory.
- A Heavy Shell still consumes its already committed limited round if it leaves the simulation volume without terrain impact, matching existing limited-weapon semantics.
- If a wind-altered Heavy Shell impact lands on a different hill, crater, tank, or out-of-bounds path, the resulting ordinary gameplay consequence is determined solely by that resolved trajectory.
- The default ammo quantity and wind response are initial balance values. If manual matches show Heavy Shell is not a meaningful but non-dominant strong-wind choice, tuning may adjust only those documented conventional values without expanding weapon mechanics.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The game MUST establish `Heavy Shell` as a stable explicit conventional-weapon identity, independent of display text, selector position, inventory storage, or visual presentation.
- **FR-002**: The authoritative conventional weapon catalogue MUST contain Heavy Shell with display name `HEAVY SHELL`, two starting rounds per player, a projectile profile, and an ordinary impact profile. Basic Shell and High Explosive identities and definitions MUST remain available and correct.
- **FR-003**: The projectile profile MUST gain the smallest explicit projectile-level wind-response characteristic required by Heavy Shell. It MUST represent a multiplicative response to the match's existing horizontal wind acceleration, not physical mass, drag, density, area, lift, spin, terminal velocity, or penetration.
- **FR-004**: Basic Shell and High Explosive MUST retain normal wind response. Heavy Shell MUST use a positive response substantially below normal; the initial configured target is 40% of ordinary wind acceleration, subject to documented manual-play tuning evidence.
- **FR-005**: A committed fired shot MUST capture the selected definition's projectile wind response into immutable authoritative flight state before simulation begins. In-flight resolution MUST not depend on mutable selection, inventory, catalogue lookup, HUD, camera, rendering, or visual-effect timing.
- **FR-006**: The common fixed-step projectile simulation MUST combine unchanged established gravity with horizontal wind scaled by the projectile's captured generic wind-response characteristic. It MUST contain no condition keyed to Heavy Shell identity, display name, selector key, or inventory position.
- **FR-007**: With zero wind and otherwise identical launch configuration, Basic Shell and Heavy Shell MUST have equivalent position and velocity traces within established numeric tolerance. Reduced wind response MUST affect neither gravity nor vertical motion.
- **FR-008**: For every valid horizontal wind direction, equal launch inputs, and equal fixed-step duration, Heavy Shell MUST experience a smaller horizontal wind contribution than Basic Shell. It MUST still experience non-zero wind influence in strong valid wind, and reversing wind MUST reverse its influence.
- **FR-009**: Heavy Shell MUST use the existing azimuth, elevation, firing-power, launch-origin, deterministic timestep, terrain collision, out-of-bounds, and ordinary ballistic rules. It MUST NOT add guidance, thrust, steering, stages, bouncing, rolling, splitting, trajectory prediction, or automatic wind compensation.
- **FR-010**: Each player MUST receive an independent Heavy Shell availability entry of two rounds through the existing inventory mechanism. Selecting it MUST consume no ammunition; a legal firing commitment MUST consume exactly one round for only the firing player; rejected actions MUST consume none.
- **FR-011**: Heavy Shell MUST participate in the existing generic weapon cycle and normal selected-weapon persistence, availability, and exhaustion fallback behaviour. It MUST add no Heavy-Shell-specific control surface; the normal selector must make all three conventional weapons reachable.
- **FR-012**: The existing tactical HUD MUST show the active player's selected `HEAVY SHELL` name and truthful remaining count using its current selected-weapon/inventory presentation. The HUD must not redesign weapon presentation, expose corrected aim, or calculate a predicted impact.
- **FR-013**: Heavy Shell MUST resolve terrain impacts through the ordinary shared collision and impact pipeline. Its initial impact profile MUST match Basic Shell's 6-unit damage radius, 40 maximum damage, 4.0-radius/1.8-depth crater, and restrained normal visual scale unless manual balance testing justifies a documented small deviation.
- **FR-014**: Heavy Shell impacts MUST preserve ordinary radial damage, terrain deformation, support reconciliation, tank settling, elimination/winner/draw determination, and turn completion. It MUST not acquire High Explosive's larger-blast role, a direct-hit rule, or a special terrain rule.
- **FR-015**: Given identical weapon profile, launch inputs, terrain, gravity, wind, and fixed-step sequence, Heavy Shell trajectory and impact results MUST be deterministic and independent of presentation state.
- **FR-016**: Automated coverage MUST verify catalogue identity/metadata/profile values; independent inventories; selection non-consumption; exact firing consumption; generic exhaustion; shot snapshots; ordinary impact consequences; and Basic Shell/High Explosive regression behaviour.
- **FR-017**: Automated projectile comparisons MUST verify Heavy Shell's smaller crosswind displacement, proportional response in reversed crosswind and aligned/reversed aligned wind, stronger-versus-weaker Heavy Shell wind displacement, non-immunity, zero-wind equivalence, unchanged gravity, deterministic repeat traces, and ordinary terrain collision.
- **FR-018**: Completion MUST include manual two-player matches in calm/low, noticeable, and strong valid winds to assess whether Heavy Shell is visibly less wind-sensitive and sometimes preferred as a limited tactical choice. The workspace build, relevant tests, formatting, linting, documentation, and only genuinely satisfied roadmap checkboxes MUST be healthy.

### Key Entities

- **Heavy Shell**: A limited conventional weapon whose player-facing identity is a stable, wind-resistant ballistic round.
- **Projectile wind response**: A captured per-projectile multiplier that scales only the authoritative match's horizontal wind contribution; normal shells use normal response and Heavy Shell uses a reduced positive response.
- **Weapon definition**: The central conventional-weapon record supplying stable identity, display metadata, ammunition rule, projectile profile, and impact profile.
- **Player weapon loadout**: One player's independent authoritative selection and availability for Basic Shell, High Explosive, and Heavy Shell.
- **Fired shot**: Immutable committed authority containing weapon identity, projectile flight state including wind response, and copied impact profile.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In automated same-launch, same-gravity, same-wind, same-duration traces, Heavy Shell's horizontal displacement caused by a 1.5-units/s² crosswind is 40% of Basic Shell's displacement within established numeric tolerance, and is greater than zero.
- **SC-002**: In automated traces for positive/negative crosswind and positive/negative aligned wind, Heavy Shell's wind-induced horizontal displacement has the same direction as each wind vector and a magnitude smaller than Basic Shell's corresponding displacement in all four cases.
- **SC-003**: In 100% of zero-wind comparison traces with otherwise identical ballistics, Basic Shell and Heavy Shell position and velocity states are equivalent within the established numeric tolerance; their vertical velocity and displacement also remain equivalent under non-zero horizontal wind.
- **SC-004**: In automated per-player inventory traces, both players begin with exactly two Heavy Shell rounds; selection consumes zero; each legal Heavy Shell shot consumes exactly one own round; rejected actions consume zero; and a final-round commitment leaves the next choosing selection on available Basic Shell.
- **SC-005**: In automated controlled terrain-impact scenarios, Heavy Shell produces exactly one ordinary impact and preserves damage, crater deformation, tank settling, elimination/winner/draw resolution, and turn handoff through the same consequence sequence as the other conventional weapons.
- **SC-006**: In manual two-player play, all three weapons are selectable via the normal UI, Heavy Shell ammunition is legible, and players can visibly distinguish reduced Heavy Shell crosswind drift from Basic Shell during strong-wind shots without aim assistance.
- **SC-007**: After several manual matches with strong valid wind, at least one documented player decision selects Heavy Shell specifically to reduce expected drift while retaining Basic Shell for conservation and High Explosive for blast area; if this does not occur, balance findings are recorded rather than declaring tactical-choice roadmap work complete.

## Assumptions

- The current authoritative starting point is the First Arsenal framework: unlimited Basic Shell, two High Explosive rounds, immutable conventional fired shots, a fixed 1/120-second simulation step, 8.0 units/s² gravity, and a match-constant horizontal wind sampled from 0.75 through 1.75 units/s².
- The supplied acceptance example `HEAVY SHELL ×2` is adopted as the initial two-round loadout. Limited availability is preferred before changing familiar firing-power/range semantics.
- Heavy Shell initially matches Basic Shell's launch-power relationship and impact profile. Its sole intentional mechanical distinction is a 0.40 wind-response multiplier; this is an initial tuneable gameplay value, not a physical mass claim.
- The current direct weapon selectors use `1` and `2`; the implementation may extend that established normal selection scheme (for example, a third selector or a cycle) so all three weapons remain naturally reachable without a Heavy-Shell-specific control mode.
- This feature depends on the existing weapon catalogue/loadout/fired-shot boundary, projectile simulation, wind and gravity model, aiming controls, tactical HUD, terrain/deformation, combat, settling, turn, and victory systems.

## Out of Scope

- Physically simulated projectile mass, drag, atmosphere, density, aerodynamic area, terminal velocity, lift, spin, or any other speculative projectile physics.
- Variable, vertical, layered, or weather-driven wind.
- Guidance, thrust, steering, trajectory prediction, recommended aim, automatic compensation, or precision aiming assistance.
- New explosion mechanics, large-radius explosive behaviour, direct-hit rules, penetration, bouncing, rolling, splitting, MIRV, cluster, mines, napalm, terrain building, or crater-focused behaviour.
- Armour, shields, economy, purchases, weapon shops, configurable starting inventories, external weapon files, modding, weapon-specific audio, or elaborate weapon-specific effects.
- Small Precise Projectile, Large-Radius Explosive, MIRV, Crater-Focused Weapon, or any other future weapon.

## Roadmap Alignment

- On completion, review **Weapons / Initial Weapons** and check **Heavy projectile with reduced wind sensitivity** only after the limited Heavy Shell is playable and visibly wind-resistant.
- Preserve the unchecked **configurable projectile mass** and **projectile drag** items: this feature uses an explicit gameplay wind-response multiplier, not either system.
- Use this feature as further evidence for the already completed **Support weapon-specific projectile behaviour without premature abstraction** item; it does not broaden that item into a universal weapon framework.
- Record strong-wind tuning and compensation discoveries under **Wind Gameplay**. Check wind-choice or Arsenal milestone items only after manual matches demonstrate distinct useful Basic Shell, High Explosive, and Heavy Shell decisions.
- Potential later arsenal candidates remain roadmap work: Small Precise Projectile, Large-Radius Explosive, MIRV, and Crater-Focused Weapon.

## Constitution Compliance

- The feature prioritises a simple, learnable tactical decision over a realistic mass or aerodynamics simulation.
- It extends one real, demonstrated projectile need with a narrow profile value and shared deterministic simulation, avoiding speculative weapon abstractions and weapon-name branches.
- It preserves the playable vertical slice and presentation-independent authoritative consequences; nonessential balance discoveries are recorded in the roadmap rather than silently expanding scope.
