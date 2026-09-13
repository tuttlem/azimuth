# Feature Specification: AI Arsenal Tactics

**Feature Branch**: `20260913-155853-ai-arsenal-tactics`  
**Created**: 2026-09-13  
**Status**: Draft  
**Input**: User description: "Improve AI intelligence, add a setup difficulty level, make AI aware of other weapons, and let it shop at the end of a round."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Set an opponent's challenge level (Priority: P1)

When configuring a local match, a player can set each AI opponent to Easy, Normal, or Hard and can see that setting before the match begins.

**Why this priority**: Players need a clear, approachable way to choose a solo or mixed-match challenge without altering the rules or giving opponents hidden advantages.

**Independent Test**: Configure each supported AI slot at each difficulty, start a match, and observe that the chosen difficulty is retained for that opponent's turns and between-round decisions.

**Acceptance Scenarios**:

1. **Given** a match setup slot is controlled by AI, **When** the player changes its difficulty, **Then** the slot visibly cycles through Easy, Normal, and Hard and retains the selected value until the match ends or setup is changed.
2. **Given** a Human-controlled setup slot, **When** setup is displayed, **Then** AI difficulty does not imply that the human is controlled by AI or interfere with human naming and controller selection.
3. **Given** a match containing AI opponents at different difficulties, **When** the match begins, **Then** each opponent follows its own selected difficulty rather than a shared global setting.

---

### User Story 2 - Face AI that uses its arsenal sensibly (Priority: P1)

During an AI turn, the opponent chooses an available weapon that suits the tactical situation instead of always using the Basic Shell.

**Why this priority**: The expanded weapon catalogue and shop are much more engaging when AI opponents participate in those choices and produce varied, recognizable attacks.

**Independent Test**: Give an AI a controlled mix of ammunition and place opponents at close, long, grouped, and wind-exposed positions; verify that the chosen weapon is available, is consumed only when fired, and varies appropriately across situations.

**Acceptance Scenarios**:

1. **Given** an AI has one or more limited weapons with ammunition, **When** it takes a firing turn, **Then** it may choose a suitable owned weapon rather than being restricted to Basic Shell.
2. **Given** an AI lacks ammunition for a limited weapon, **When** it chooses an action, **Then** it never selects or consumes that weapon and falls back to an available legal weapon.
3. **Given** comparable targets at different ranges, wind conditions, or groupings, **When** a Normal or Hard AI makes decisions, **Then** its weapon choices demonstrably differ across at least some suitable situations rather than always selecting a single weapon type.
4. **Given** an AI at any difficulty, **When** it fires, **Then** it uses the same public battlefield state, weapon inventory, ammunition, turn restrictions, and projectile rules available to human players.

---

### User Story 3 - Meet opponents whose judgement matches difficulty (Priority: P2)

Players experience Easy opponents as forgiving, Normal opponents as competent, and Hard opponents as more consistent and tactically aware without making them omniscient.

**Why this priority**: A difficulty label is useful only when its effect is noticeable, fair, and dependable in actual games.

**Independent Test**: Run repeatable scenarios with the same participants, terrain, inventory, and initial conditions at each difficulty; compare weapon selection and aiming quality over a representative sequence of AI turns.

**Acceptance Scenarios**:

1. **Given** identical repeatable battlefield conditions, **When** Easy and Hard opponents act over a representative sequence, **Then** Hard produces fewer grossly unsuitable weapon choices and more consistent shots than Easy.
2. **Given** a Normal AI, **When** it assesses a practical shot, **Then** it makes sensible range- and wind-aware choices without requiring perfect hits or exhaustive trajectory prediction.
3. **Given** any AI difficulty, **When** its decision cannot produce a legal shot, **Then** it safely uses an available fallback action without stalling the match or corrupting turn state.

---

### User Story 4 - Watch AI participate in the between-round shop (Priority: P2)

At the end of a round, AI opponents spend their own earned cash on useful ammunition before the next round begins.

**Why this priority**: AI shopping completes the economy loop, allows its tactical weapon choices to matter in later rounds, and makes a session feel like a coherent contest rather than a human-only shop.

**Independent Test**: Complete rounds with AI wallets at zero, below a product price, and sufficient for multiple products; inspect the next round's cash and ammunition for each AI.

**Acceptance Scenarios**:

1. **Given** an AI reaches its shop turn with enough cash for one or more products, **When** the shop phase resolves, **Then** it buys only offered purchasable weapons that fit within its own wallet and receives the corresponding ammunition.
2. **Given** an AI has insufficient cash for every product, **When** its shop turn resolves, **Then** it makes no invalid purchase, retains its cash, and the session continues.
3. **Given** successive rounds and sufficient funds, **When** an AI shops, **Then** its purchases create a usable and varied future inventory rather than permanently relying only on Basic Shell or accumulating an unusable product.
4. **Given** AI and human shoppers in the same round transition, **When** AI shopping completes, **Then** it neither changes human cash or ammunition nor blocks the existing human shopping flow.

### Edge Cases

- An AI whose preferred weapon is depleted, unavailable, or unaffordable must choose a legal alternative without losing its turn.
- Basic Shell remains the universal legal fallback and is not bought as a shop item.
- An AI with no living target, a terminal match state, or a non-action presentation state must not make an invalid tactical decision.
- Difficulty changes in setup apply only to the selected AI slot; switching a slot to Human must preserve the existing controller and identity expectations without exposing irrelevant controls.
- AI purchases must never create negative cash, duplicate a purchase, exceed shop stock rules if such rules exist, or alter another participant's inventory.
- Decisions must remain reproducible for the same match configuration and random seed.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Match setup MUST provide an Easy, Normal, and Hard difficulty setting for every AI-controlled player and visibly associate each setting with its AI slot.
- **FR-002**: Setup MUST give each newly created or newly converted AI slot a documented default difficulty of Normal and MUST retain the chosen difficulty through match start and round transitions.
- **FR-003**: AI difficulty controls MUST be accessible by keyboard alongside existing setup controls and MUST not conflict with player-count selection, Human/AI toggling, name editing, or match start.
- **FR-004**: Every AI firing decision MUST select only a weapon that the acting player currently owns, has ammunition for when limited, and may legally fire in the current turn state.
- **FR-005**: AI weapon choice MUST consider at least target distance, target grouping, current wind conditions, and the distinct gameplay role of available weapons; it MUST use Basic Shell as a legal fallback rather than a fixed default for every decision.
- **FR-006**: AI difficulty MUST affect the reliability and tactical quality of aiming and weapon selection in a consistently ordered way: Easy is forgiving, Normal is competent, and Hard is more consistent and situationally aware than Normal.
- **FR-007**: AI MUST obey the same observable gameplay information, ammunition, cash, weapon effects, action timing, combat bounds, and turn restrictions as a human player; difficulty MUST NOT grant hidden state, free items, altered projectile physics, or extra actions.
- **FR-008**: During each AI shop turn, the AI MUST decide whether and what to purchase from the currently offered purchasable weapons using only its own cash and inventory.
- **FR-009**: An AI shop purchase MUST follow the same price, affordability, ammunition, and cash rules as a human purchase and MUST never reduce the AI's wallet below zero.
- **FR-010**: AI shopping MUST produce a bounded, promptly completed decision, preserve the sequential between-round flow, and leave the next participant or next-round transition unblocked.
- **FR-011**: Difficulty-aware AI behaviour and AI shopping MUST be reproducible for the same match configuration and seed, without setup or presentation actions perturbing game decisions.
- **FR-012**: Automated coverage MUST validate all difficulties, legal/depleted-inventory fallbacks, varied tactical weapon choices, fair shopping, insufficient-funds behaviour, and deterministic repeated decisions.
- **FR-013**: Before completion, setup guidance, relevant roadmap status, and user-facing shop/AI information MUST be reviewed; workspace build, relevant tests, formatting, and lint checks MUST pass without unjustified warnings.

### Key Entities

- **AI difficulty**: The per-opponent setting—Easy, Normal, or Hard—that controls the quality and consistency of the opponent's decisions while preserving fair game rules.
- **Tactical decision**: An AI's legal choice of target, weapon, and firing parameters during its turn.
- **AI inventory**: The acting opponent's currently usable Basic Shell and limited-weapon ammunition, which constrains weapon selection.
- **AI shopping decision**: A bounded between-round choice to retain cash or buy one or more affordable offered weapons for the acting AI participant.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In setup checks covering all supported player slots, 100% of AI slots can be set to Easy, Normal, or Hard and preserve the selected setting into the match; Human slots remain unaffected.
- **SC-002**: Across controlled close-range, long-range, grouped-target, and strong-wind trials, AI selects only legal owned weapons in 100% of turns and chooses at least two distinct limited weapon types when suitable ammunition is supplied.
- **SC-003**: Across 30 identical seeded decision opportunities per difficulty, Hard produces fewer unsuitable-weapon or gross-aim-error outcomes than Easy, and Normal falls between them; no difficulty receives information or resources unavailable to a human.
- **SC-004**: In controlled AI shopping trials at zero, insufficient, and multi-purchase funding levels, 100% of purchases preserve non-negative cash, correctly add ammunition, and leave other participants' wallets and inventories unchanged.
- **SC-005**: Repeating a representative mixed Human/AI match setup with the same seed produces the same AI tactical and shopping decisions in 100% of runs.

## Assumptions

- Difficulty is configured independently for each AI setup slot, with Normal as the default; players may mix difficulty levels in one local match.
- Difficulty changes decision quality and bounded randomness, not the shared game rules or information available to the opponent.
- The initial tactical slice covers weapon choice and firing decisions, not AI-controlled movement, pathfinding, personalities, teams, or a perfect ballistic solver.
- AI shopping uses the existing sequential shop, existing product catalogue, and existing cash/ammunition rules; no separate AI currency, discount, stockpile cap, or shop product category is introduced.
- "Better" AI means more appropriate, more consistent decisions that remain visibly fallible and enjoyable, rather than optimal or unbeatable play.

## Out of Scope

- AI tactical movement, pathfinding, collision avoidance, terrain navigation, or moving instead of firing.
- Perfect trajectory prediction, full terrain simulation search, hidden-state reading, aim assistance beyond the AI's ordinary visible battlefield information, or difficulty-specific combat rules.
- New weapons, weapon balance changes, shop categories, shop stock limits, armour, environment presets, or new match modes.
- Online multiplayer, replays, controller support, mouse setup controls, or persistent player-wide difficulty preferences.

## Roadmap Alignment

- This feature addresses the open AI requirements to choose weapons and become more capable, and makes the existing AI shopping milestone demonstrably useful.
- On completion, update only the verified AI, setup, shop, and complete-match roadmap items. Record AI movement, personalities, advanced wind/environment reasoning, and further economy tuning as later work.

## Constitution Compliance

- The feature builds on the playable artillery loop with a bounded, visible improvement to solo and mixed local matches rather than a general-purpose AI framework.
- AI remains subject to the same simple, learnable systems as a human. Its deterministic, seed-controlled decisions are testable independently of presentation and do not grant hidden advantages.
- The scope prioritises enjoyable, varied weapon use and a complete round-to-round loop while deferring speculative tactical movement and optimal simulation.
