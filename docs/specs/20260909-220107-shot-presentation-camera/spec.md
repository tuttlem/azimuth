# Feature Specification: Shot Presentation Camera

**Feature Branch**: `20260909-220107-shot-presentation-camera`

**Created**: 2026-09-09

**Status**: Draft

**Input**: User description: "Controller-aware shot presentation: Human follow cam, AI tactical view, shared impact framing."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Learn From a Human Shot (Priority: P1)

As a Human player, I can watch my fired shell from a comfortable broadcast-style follow view so I can understand its arc, wind drift, terrain clearance, range, and landing position before taking a later shot.

**Why this priority**: Feedback from a player's own shot is central to learnable artillery aiming.

**Independent Test**: Start a Human turn and fire short, medium, long, and high-arc shots across uneven terrain; each provides readable flight and landing feedback without changing its physical outcome.

**Acceptance Scenarios**:

1. **Given** a Human player fires, **When** the shell launches, **Then** the view leaves aiming smoothly and acquires the shell from a trailing, offset, non-first-person angle that retains nearby terrain.
2. **Given** a Human shell climbs, **When** it approaches and passes its highest observed flight region, **Then** the view widens smoothly to reveal shot shape, surrounding terrain, and likely destination where practical.
3. **Given** a Human shell descends, **When** its impact area becomes apparent, **Then** the view keeps the shell readable while favouring the landing area without snapping, aggressive roll, or tight missile-style attachment.
4. **Given** wind shifts a shell sideways or it clears or strikes a ridge, **When** flight is observed, **Then** the composition provides enough reference to distinguish drift and terrain relationship where practical.

---

### User Story 2 - Understand an AI Shot Tactically (Priority: P1)

As a person watching an AI turn, I see a broad tactical view of the firing tank, shell route, nearby battlefield, and likely danger area so I can understand what the AI did without a self-indulgent projectile chase.

**Why this priority**: AI turns should preserve battlefield awareness, including in all-AI spectator matches.

**Independent Test**: Run a match containing AI players, several consecutive AI turns, and an all-AI match; every AI shot uses broad tactical presentation rather than Human follow treatment.

**Acceptance Scenarios**:

1. **Given** an AI turn begins, **When** it becomes active, **Then** normal active-player presentation establishes the firing tank without a prolonged separate introduction.
2. **Given** an AI fires, **When** the shell is in flight, **Then** a generally wide view shows direction, surrounding terrain, and likely impact region while keeping shooter, shell, relevant tanks, and a Human tank visible where reasonably achievable.
3. **Given** several AI players act in succession, **When** their turns resolve, **Then** transitions remain efficient and readable rather than replaying excessive cinematic motion.
4. **Given** an all-AI match, **When** AI shots occur, **Then** the tactical view remains useful spectator composition and does not depend on a Human tank.

---

### User Story 3 - See Shared Impact Consequences (Priority: P1)

As a match observer, I see the consequence of every meaningful shot in a shared impact view, regardless of who fired, so explosions, crater formation, tank damage, elimination, and settling are understandable before the next turn or result.

**Why this priority**: Consequences matter more than controller-specific flight treatment once a projectile lands.

**Independent Test**: Cause terrain misses, direct and splash hits, self-hits, multi-tank impacts, crater-induced settling, eliminations, and match-ending impacts from both Human and AI turns; each converges to a common aftermath composition.

**Acceptance Scenarios**:

1. **Given** either controller's projectile impacts terrain, **When** explosion and crater effects occur, **Then** the view frames impact point, changed terrain, and nearby tanks where practical and remains long enough for crater formation to be apparent.
2. **Given** an impact damages, eliminates, or causes tanks to settle, **When** aftermath resolves, **Then** the affected area is prioritised so the outcome is legible where practical, while scoreboard and match context remain available.
3. **Given** a shot ends the match, **When** outcome is resolved, **Then** final impact and aftermath appear before the existing winner/draw presentation, and no new-player view is selected.
4. **Given** a shot has no affected tanks or leaves the expected visible region, **When** it resolves, **Then** presentation degrades to a safe bounded view without invalid, underground, or stage-edge-focused framing.

---

### User Story 4 - Preserve Authoritative Play (Priority: P1)

As a player, I receive improved shot presentation without camera state changing deterministic result, timing, or match progression.

**Why this priority**: Camera behaviour is presentation only; artillery correctness must be independent of how a shot is shown.

**Independent Test**: Compare identical shots with alternate presentation modes or presentation disabled and verify identical impact, damage, terrain, settling, elimination, victory, and turn-resolution results.

**Acceptance Scenarios**:

1. **Given** a projectile is in flight or aftermath is shown, **When** camera movement is slow, interrupted, or unavailable, **Then** projectile integration, wind, collision, damage, terrain deformation, settling, elimination, victory, and turn advancement continue under existing authoritative rules.
2. **Given** a firing player's display name changes while controller type remains the same, **When** it fires, **Then** presentation remains unchanged; changing controller type changes the flight mode.
3. **Given** impact causes settling, **When** authoritative settling completes, **Then** next turn or final result resolves by match rules rather than a camera waypoint or aftermath timer.

### Edge Cases

- Immediate terrain impact, very low arc, and self-hit retain a safe shot-to-impact transition without requiring prolonged tracking.
- Extremely high arcs and shots near or beyond visible battlefield use bounded framing and readable fallback rather than unsupported stage edges or extreme clipping.
- An unexpected multi-tank hit gives impact consequences priority over retaining the preceding Human or AI flight composition.
- A terrain miss with no affected tank still makes crater location and its relation to terrain understandable, especially for Human learning.
- A final impact eliminating all remaining tanks presents draw aftermath and never selects another player.
- Two-, four-, and eight-player matches may have several nearby tanks; no composition must fit every tank perfectly.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST select Human follow-shot presentation when the firing player's configured controller type is Human, and AI tactical-shot presentation when it is AI.
- **FR-002**: Mode selection MUST use firing-player controller classification, not display name, input history, shot accuracy, projectile characteristics, or existence of a Human-controlled survivor.
- **FR-003**: Human follow-shot presentation MUST smoothly acquire projectile after launch from an offset, trailing perspective that shows projectile and useful terrain without rigid attachment, roll matching, or aggressive rotation.
- **FR-004**: Human follow-shot presentation MUST widen around observed ascent-to-descent transition, using current motion or an equally reliable observed flight signal, to show broader terrain and likely destination where practical.
- **FR-005**: Human follow-shot presentation MUST progressively favour approaching impact area during descent while preserving projectile visibility where practical.
- **FR-006**: AI tactical-shot presentation MUST remain generally broader than Human follow view and prioritise shell direction, shooter location, surrounding terrain, relevant tanks, and likely impact region over close projectile tracking.
- **FR-007**: AI tactical-shot presentation MUST remain readable and efficient across consecutive AI turns and all-AI matches, without requiring a Human-controlled tank as a framing target.
- **FR-008**: Both flight modes MUST converge on one shared impact-presentation phase for terrain impacts, prioritising explosion, crater, terrain feature, and affected tanks where practical.
- **FR-009**: Shared impact presentation MUST allow explosion and terrain deformation to become visible and SHOULD retain settling, damage, and elimination consequences in view where practical before normal next-player presentation or match result.
- **FR-010**: When a shot produces a winner or draw, presentation MUST preserve final aftermath then show existing match result without a new active-player view.
- **FR-011**: Camera presentation state MUST be small, explicit, and understandable, covering normal player framing, controller-specific flight framing, common impact framing, and return to next player or result.
- **FR-012**: Camera presentation MAY observe projectile position, velocity, elapsed flight progress, active/impact state, terrain, tank state, battlefield bounds, and match state, but MUST NOT mutate or gate projectile physics, wind, collision, explosion, damage, terrain deformation, tank settling, elimination, winner resolution, or authoritative turn advancement.
- **FR-013**: Camera movement and framing MUST use smooth bounded transitions and deliberate zoom changes; it MUST avoid obvious underground views, persistent terrain clipping, unnecessary stage-edge exposure, and extreme clipping artefacts.
- **FR-014**: Existing scoreboard, active-player identity, and essential match context MUST remain useful throughout shot and impact presentation; this feature MUST NOT introduce a separate HUD system.
- **FR-015**: Meaningful tuning values for follow offset, smoothing, apex widening, tactical breadth, impact composition, and aftermath duration MUST be centralised.
- **FR-016**: The feature MUST provide focused automated coverage for controller-aware mode selection, Human and AI state progressions, apex widening, common impact handling, final-result handling, controller/display-name independence, and presentation-independent authoritative results.
- **FR-017**: The feature MUST NOT add projectile-physics changes, aim assistance, shot memory or bracketing, trajectory prediction, slow motion, replay, kill/death cameras, spectator redesign, audio, AI targeting changes, weapons, or a general cinematic framework.
- **FR-018**: Design-intent comments MUST explain Human feedback versus AI awareness, apex widening, shared impact consequences, simulation independence, and safety fallback behaviour.

### Key Entities

- **Shot presentation mode**: Observer-facing Human follow or AI tactical selection derived from the firing player's controller classification.
- **Camera presentation state**: Short-lived observer state for normal player framing, controller-specific flight coverage, shared impact aftermath, and return to player or result.
- **Observed flight state**: Read-only projectile information used to keep presentation grounded in physical motion, including ascent/descent indication.
- **Impact context**: Read-only aftermath information centred on impact position, altered terrain, nearby or affected tanks, settling, and final-match status.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In short, medium, long, and high-arc Human shots, observers can identify shell, lateral wind drift, and whether it landed short or long in all 4 of 4 scripted manual-review cases.
- **SC-002**: In controller-selection tests covering Human and AI shots, including renamed players, 100% choose the mode associated with controller type and none choose from display name.
- **SC-003**: In Human and AI impact variants covering terrain miss, direct/near hit, multi-tank splash, self-damage, settling, elimination, and match-ending shot, 100% enter the same impact phase before next-player or result presentation.
- **SC-004**: Across 2-, 4-, and 8-player matches plus an all-AI match, every completed AI shot retains broad battlefield-readable composition and returns to correct next survivor or final result without invalid camera state.
- **SC-005**: For representative deterministic shots, enabling, disabling, or varying presentation produces identical authoritative impact position, damage, terrain, settling, elimination, and match outcome.
- **SC-006**: During manual terrain review across mountains, ridges, valleys, water regions, and distant impacts, no recurring obvious underground camera position, severe terrain clipping, or uncontrolled zoom oscillation occurs in 20 consecutive shots.

## Assumptions

- Existing controller classification in match configuration is the authoritative presentation selector; changing it follows existing match-setup rules.
- Current projectile state exposes sufficient read-only position, velocity, and flight-progress information for practical apex detection; landing prediction is unnecessary.
- Existing active-player presentation, HUD, battlefield bounds, terrain/horizon presentation, impact effects, tank settling, and winner/draw presentation are extended rather than replaced.
- “Where practical” permits bounded fallback composition when terrain, safety, extreme shot geometry, or many players make simultaneous inclusion impossible; impact consequence takes priority over showing every entity.
- Authoritative match resolution may finish before presentation completes its brief aftermath hold; presentation observes it and never delays or alters it.
- Roadmap alignment is limited to projectile tracking, controller-aware Human golf-style and AI tactical presentation, apex widening, impact framing, terrain/affected-player visibility, and smooth return to the next player. No unrelated camera item is claimed complete.

## Dependencies

- Existing configured 2–8 player matches, including Human and AI controller classifications.
- Existing normal active-player camera, wide shot presentation, smooth transitions, zoom limits, battlefield bounds, terrain/horizon presentation, and tactical HUD.
- Existing deterministic projectile lifecycle, impact/explosion flow, terrain deformation, tank settling, elimination, winner/draw resolution, and regression tests.

## Manual Acceptance

- **Human learning shots**: In a Human-controlled match, review short, medium, long, and high-arc shots. Confirm smooth departure from aiming; easy projectile location; comfortable offset tracking; visible local terrain; widening near apex; readable wind displacement and descent; clear crater/impact; understandable settling; and correct next-turn return. The practical result must let the player identify “short/long” and “downwind/upwind.”
- **AI awareness shots**: In a match with AI, confirm an AI flight is tactical-wide rather than Human-follow; shell direction, geography, shooter, and likely danger region remain understandable; a Human tank is retained where reasonable; and the common impact view takes priority for meaningful hits.
- **Consecutive and all-AI turns**: Run Human → AI → AI → AI → Human and a separate all-AI match. Confirm each shooter is established efficiently, no transition feels repeatedly theatrical, impacts remain understandable, control returns cleanly, and no framing depends on a Human tank.
- **Terrain safety**: Fire across mountains, ridges, valleys, water regions, and distant areas. Confirm Human tracking makes clearance readable, apex widening reveals geography, impact framing works at varied elevations, and the camera does not regularly clip through terrain or spoil the horizon/background.
- **Impact variants**: Observe clean terrain miss, direct/near hit, multi-player splash, self-damage, elimination, crater-caused settling, and final match-ending impact. Confirm each is visually legible before normal continuation or final result.
- **Regression review**: Run the focused automated coverage and compare deterministic fixtures with alternate or absent presentation. Confirm impact position, damage, terrain result, settling, elimination, winner/draw, and turn outcome are unchanged.

## Architecture Review

- **Does camera behaviour observe the simulation rather than own it?** Yes. Presentation reads authoritative flight and aftermath only; it has no authority over simulation or turn progression.
- **Does controller type select presentation without creating separate gameplay paths?** Yes. Classification chooses observer presentation only; firing and resolution use the same gameplay flow.
- **Do Human and AI shot modes converge cleanly into common impact framing?** Yes. Distinct flight coverage joins a single consequence-first impact phase.
- **Could camera behaviour be changed or disabled without altering authoritative shot result?** Yes. Camera state is an optional observer of independently resolved gameplay.
