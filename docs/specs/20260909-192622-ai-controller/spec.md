# Feature Specification: First AI Controller — A Computer Player That Can Complete a Turn

**Feature Branch**: `20260909-192622-ai-controller`  
**Created**: 2026-09-09  
**Status**: Draft  
**Input**: User description: "First AI Controller — A Computer Player That Can Complete a Turn"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Play Against a Computer Opponent (Priority: P1)

As a player, I can configure one or more existing player slots as AI and start the match, then watch each AI complete an ordinary legal turn without anyone operating the keyboard for it.

**Why this priority**: AI-configured matches are deliberately blocked today. Removing that seam is the smallest useful solo-play and spectator-play experience.

**Independent Test**: Configure one Human and one AI, complete the Human turn, and verify the AI becomes active, makes a visible firing decision, launches an ordinary shot, and hands off only after normal resolution.

**Acceptance Scenarios**:

1. **Given** a valid two-to-eight-player configuration containing one or more AI slots, **When** Start Match is requested, **Then** the match starts with configured identities, names, colours, inventories, and turn order; it is not rejected merely because a slot is AI.
2. **Given** an in-progress AI turn with a living opponent, **When** any brief readability pause has elapsed, **Then** the AI chooses a living non-self target, sets valid ordinary aim and an available weapon, and fires through the normal match action path.
3. **Given** the resulting shot, **When** it flies and resolves, **Then** wind, gravity, terrain, damage, deformation, settling, elimination, victory, and turn advancement behave exactly as for a Human shot.
4. **Given** an AI-controlled active player, **When** a user presses tactical aim, weapon, movement, or fire controls, **Then** those controls do not alter that AI player's turn.

---

### User Story 2 - Watch Consecutive Computer Turns (Priority: P1)

As a player in a mixed or all-AI local match, I can watch consecutive surviving AI players take their turns and eventually reach the normal winner or draw result without a Human turn being required between them.

**Why this priority**: Consecutive computer turns and all-AI play prove that the controller is a real participant in the existing match rather than an assist attached to Human input.

**Independent Test**: Start a deterministic four-player all-AI match, advance enough ordinary shot-resolution cycles to observe several consecutive handoffs, and verify no turn waits for keyboard input or selects an eliminated player.

**Acceptance Scenarios**:

1. **Given** multiple consecutive AI players, **When** one AI shot fully resolves, **Then** the next living configured player acts according to its controller type, including AI-to-AI handoffs.
2. **Given** an eliminated Human or AI player, **When** later turns advance through that slot, **Then** the eliminated player is skipped and cannot make, receive, or retain a queued action.
3. **Given** eight AI slots, **When** the match starts, **Then** it progresses autonomously with ordinary projectile and impact presentation rather than deadlocking for input or instantly collapsing the whole match into an unobservable result.
4. **Given** one survivor or no survivors after ordinary resolution, **When** the match ends, **Then** the normal configured-name winner or draw presentation appears and no AI acts again.

---

### User Story 3 - Recognise Imperfect Artillery Decisions (Priority: P2)

As an observer, I can understand that an AI is trying to shoot another player because its aim and power broadly correspond to target direction and distance, while its imperfect shots remain surprising and entertaining.

**Why this priority**: The first controller should feel like another player, not a perfect solver or a random scripted launcher.

**Independent Test**: Use seeded two-, four-, and eight-player arrangements with representative near, far, uphill, and downhill opponents; inspect targets and firing values for valid, generally target-directed, boundedly imperfect decisions.

**Acceptance Scenarios**:

1. **Given** one or more living opponents, **When** an AI chooses a target, **Then** it chooses exactly one living opponent and never deliberately chooses itself or an eliminated player.
2. **Given** nearer and farther representative targets, **When** the AI chooses shots, **Then** azimuth broadly points toward the target and farther targets generally receive longer shots than nearer targets, within intentionally imperfect bounded variation.
3. **Given** ordinary wind and uneven terrain, **When** an AI fires, **Then** it may miss, strike terrain, splash itself, or hit a target; it does not use an exact guaranteed-hit solution or advanced terrain planning.
4. **Given** the normal HUD and camera, **When** an AI turn begins and fires, **Then** the active configured name, selected weapon, and current aim remain coherent and use ordinary presentation.

---

### User Story 4 - Preserve Ordinary Weapons and Human Play (Priority: P2)

As a Human player, I retain the current move-or-fire, aiming, selection, inventory, and projectile experience, while an AI uses the same legal weapon and firing rules rather than special effects or hidden ammunition.

**Why this priority**: A shared authoritative action path prevents controller-specific gameplay drift and protects established local multiplayer.

**Independent Test**: Run an all-Human regression match and mixed matches where AI choices include available conventional weapons; verify the same inventory changes and projectile consequences as ordinary player actions.

**Acceptance Scenarios**:

1. **Given** an AI has an unavailable limited weapon selected or proposed, **When** it chooses an action, **Then** it uses another available ordinary weapon and completes a valid turn rather than panicking, stalling, or bypassing availability.
2. **Given** an AI uses a limited ordinary weapon, **When** it fires, **Then** ammunition and fallback selection follow the same rules as for a Human player.
3. **Given** an all-Human match, **When** players aim, select weapons, move, fire, resolve shots, and win, **Then** their existing controls and outcomes remain unchanged.

### Edge Cases

- A target selected during planning is eliminated or the match ends before action: discard the stale decision and take no invalid action.
- Exactly one living opponent remains: select that opponent; if one total survivor remains, normal victory ends the match before another decision.
- An AI's preferred weapon has no usable ammunition: select an available ordinary fallback.
- A proposed aim reaches a gameplay limit: retain a valid bounded ordinary aim.
- Every supported 2–8 player mixture, including eight Humans and eight AIs, starts correctly; display-name edits and setup-only AI-name rerolls do not alter authoritative AI choices.
- Consecutive AI turns, resolving shots, settling, elimination, and match end cannot leave a controller waiting indefinitely for tactical input.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Match validation and start MUST accept every valid two-to-eight-player mixture of Human and AI controller types, including all-AI matches; controller type remains independent of display name, identity, colour, inventory, and tank state.
- **FR-002**: When an in-progress choosing turn belongs to an AI-configured living player, the game MUST autonomously request and execute one legal ordinary action. This first feature may use firing only and MUST explicitly leave tactical movement for later if it is not implemented.
- **FR-003**: Human tactical input MUST remain enabled only for Human-configured choosing turns. During AI choosing turns it MUST not alter aim, weapon selection, movement, or firing.
- **FR-004**: AI and Human decisions MUST converge before authoritative gameplay mutation. AI MUST use the same legal aim, weapon-selection, ammunition, firing, projectile, wind, gravity, collision, damage, terrain-deformation, settling, elimination, and resolution rules as Humans; no controller-specific tank, projectile, physics, terrain, damage, or collision model is allowed.
- **FR-005**: An AI firing decision MUST select a living non-self opponent from the player collection, ignore eliminated players, and safely discard or replace a later-invalid decision.
- **FR-006**: AI firing values MUST be valid under existing aim limits and broadly use shooter-to-target geometry: azimuth generally points toward the selected opponent, elevation is plausible artillery range, and power generally increases with horizontal separation.
- **FR-007**: AI aim MUST include bounded deliberate uncertainty. The first controller MUST NOT use an exact analytic or exhaustive ballistic solution, trajectory search, terrain-path analysis, or sophisticated wind compensation. It may ignore wind under the documented baseline policy.
- **FR-008**: AI weapon choice MUST use ordinary inventory and availability semantics. It may favour Basic Shell and may select other available conventional weapons, but MUST never consume unavailable ammunition or create AI-only weapon behaviour.
- **FR-009**: Gameplay-affecting AI randomness MUST be deterministic and isolated from setup names and presentation. Identical match seed, configuration identities, and preceding authoritative state MUST produce identical choices; display-name-only changes MUST not change them.
- **FR-010**: A small readable AI turn delay is permitted, but it MUST not change authoritative simulation or turn outcomes, and automated tests MUST not depend on wall-clock waiting.
- **FR-011**: Existing camera, scoreboard, active-player indication, aim, weapon, wind, projectile, terrain, and result presentation MUST remain coherent through AI turns. No separate AI camera or thought/debug HUD is required.
- **FR-012**: A failed or stale preferred AI action MUST recover with a currently valid ordinary action or safely end only through existing match rules; it MUST not panic, stall, or act after elimination or match completion.
- **FR-013**: Human-to-AI, AI-to-Human, and AI-to-AI handoffs MUST follow existing ordered living-player progression. Eliminated Human and AI players MUST be skipped during firing and movement handoffs alike.
- **FR-014**: Automated coverage MUST prove dispatch/input isolation; target eligibility; valid target-directed bounded-error aim; normal inventory/projectile lifecycle; deterministic choices; representative 2-, 4-, and 8-player configurations; consecutive AI turns; elimination skipping; all-AI progress; and all-Human regression behavior.
- **FR-015**: Completion MUST update only roadmap items demonstrated by implementation and manual evidence. Future AI skill, memory, wind-compensation, movement, weapon-quality, and personality work MUST remain open.

### Key Entities

- **Controller designation**: The configured Human or AI decision source for a stable player identity; it does not create a separate gameplay player.
- **AI decision**: A reproducible, short-lived proposal for a current living AI player's ordinary action, target, aim, and eligible weapon; it is invalid once authoritative state no longer fits.
- **Authoritative action**: The existing legal move or fire operation that validates current turn state, applies inventory rules, and begins normal simulation regardless of decision source.
- **AI decision seed**: Match-derived authoritative state used only for reproducible AI choices, kept separate from setup naming and presentation variation.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All seven supported participant counts accept tested Human/AI mixtures, including 1 Human plus 7 AI, 4 Humans plus 4 AI, 8 Humans, and 8 AI, without AI-unavailable rejection.
- **SC-002**: In deterministic automated two-, four-, and eight-player scenarios, 100% of AI decisions select one living non-self opponent and produce valid aim and available-weapon values.
- **SC-003**: In deterministic mixed-controller traces, 100% of tested Human-to-AI, AI-to-Human, AI-to-AI, and eliminated-player handoffs select the next living configured player; no active AI waits for Human tactical input.
- **SC-004**: Identical match seed, stable configuration, and preceding authoritative state yield identical AI choices in 100% of repeated automated runs, while an AI display-name-only change yields the same choice.
- **SC-005**: In representative near/far target tests every azimuth is valid and target-directed; median far-target power exceeds median near-target power, and at least one bounded-error case differs from ideal direction or range.
- **SC-006**: Manual one-Human-versus-one-AI and eight-AI matches show ordinary projectile flights, impacts, and turn progression; the all-AI match completes at least one turn for each initial living participant without tactical input.
- **SC-007**: Existing all-Human coverage for aiming, selection, movement, firing, projectile simulation, wind, damage, terrain deformation, settling, elimination, and victory remains passing.

## Assumptions

- The baseline controller is firing-first; movement is deferred unless it can reuse ordinary movement without navigation or tactical-planning scope.
- The baseline ignores wind rather than solving or finely compensating for it.
- A small visible AI pause may aid readability, but all-AI matches continue automatically and tests do not rely on real elapsed time.
- Existing seeded terrain, starts, wind, tank/turn state, conventional weapons, projectile lifecycle, camera, HUD, and configuration remain the feature's authority and dependencies.

## Out of Scope

- Difficulty levels, perfect ballistic solving, trajectory search, shot memory, bracketing, previous-impact learning, sophisticated wind compensation, terrain understanding, cover/threat analysis, and tactical movement strategy.
- Personality, team/coordinated AI, opponent modelling, predictive movement, machine learning, behavior-tree/utility/planning frameworks, pathfinding, navigation meshes, external AI services, or LLM-driven opponents.
- AI-specific weapons, projectiles, damage, terrain, physics, collision, rendering, camera, or a separate player model; new weapons; HUD redesign; unrelated environment, networking, or rules.

## Roadmap Alignment

- This advances **AI Opponents / Basic AI**: target selection, azimuth, elevation, firing power, and firing. Mark weapon choice only if normal multi-weapon selection is delivered; mark movement only if genuinely implemented and playtested.
- Update **Match Setup** from AI-unavailable to AI-available and mark **Milestone H — AI opponent exists** only after a complete ordinary AI match is manually demonstrated.
- Retain future AI work and record nonessential discoveries in `docs/roadmap.md` rather than expanding this feature.

## Constitution Compliance

- The controller is a narrow decision source over existing simple systems, preserving one authoritative match model and avoiding speculative AI architecture.
- AI randomness is reproducible and separated from presentation and setup naming.
- The feature favours an immediately playable, entertaining imperfect opponent over realism or optimal calculation, while preserving gameplay/presentation boundaries and quality gates.

