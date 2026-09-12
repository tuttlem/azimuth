# Feature Specification: Arsenal and Economy Cleanup

**Feature Branch**: `20260912-141531-arsenal-economy-cleanup`

**Created**: 2026-09-12

**Status**: Draft

**Input**: Remove Bomb Net, turn Bunker Buster into massive zero-damage terraforming, and make round earnings actual opponent health removed times $10 plus podium bonuses.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Use a focused arsenal (Priority: P1)

A player chooses weapons from a concise, coherent arsenal where Cluster Bomb owns the multi-bomblet area-saturation role and Bomb Net no longer appears anywhere playable.

**Why this priority**: Removing a redundant weapon makes weapon selection, inventories, shopping, AI choices, and the HUD easier to understand immediately.

**Independent Test**: Start a match, inspect the weapon bar and shop, and exercise AI/human selection; Bomb Net is absent while Cluster Bomb remains available and functional.

**Acceptance Scenarios**:

1. **Given** a new match or weapon shop, **When** a player or AI views available weapons, **Then** Bomb Net is absent from catalogues, inventory, selection, purchase, and display.
2. **Given** multi-projectile weapons remain, **When** Cluster Bomb is used, **Then** its established broad bomblet coverage remains available without a Bomb Net compatibility path.

---

### User Story 2 - Reshape the battlefield without hurting tanks (Priority: P1)

A player can buy and fire Bunker Buster to make a dramatic deep, directional excavation that changes future movement and firing lanes but never removes health or earns damage income.

**Why this priority**: This gives Bunker Buster a distinct positional purpose instead of competing with damaging explosives.

**Independent Test**: Fire Bunker Buster directly at, beside, and below one or more tanks; compare terrain before/after and confirm substantially deeper/larger directional terrain removal, unchanged health, stable settling, and zero earnings.

**Acceptance Scenarios**:

1. **Given** terrain containing a ridge or hill, **When** Bunker Buster resolves, **Then** it creates a conspicuously large, deep, penetrating terrain excavation rather than a scaled-up spherical blast.
2. **Given** one or more tanks within the impact or deformation region, **When** Bunker Buster resolves, **Then** every affected tank retains its exact prior health while normal terrain-settling behavior may change its position.
3. **Given** a direct intersection with a tank, **When** Bunker Buster resolves into terrain, **Then** no direct or splash health damage is applied.

---

### User Story 3 - Understand every dollar earned (Priority: P1)

At round completion, players see earnings composed only of actual opponent health removed at $10 per health point and their final placement bonus.

**Why this priority**: A simple accounting rule lets players understand the reward for tactical damage and finishing position without hidden bonus categories.

**Independent Test**: Resolve controlled damage, overkill, self-damage, and multi-player finishes; compare wallet and accounting display to `actual opponent health removed × $10 + placement bonus`.

**Acceptance Scenarios**:

1. **Given** an opponent with 17 health receives 80 nominal damage, **When** health is reduced to zero, **Then** the attacker earns exactly $170 for that target.
2. **Given** a completed four-player round, **When** accounting is shown, **Then** first, second, third, and fourth receive placement bonuses of $5,000, $2,500, $1,000, and $0 respectively, in addition to their actual damage income.
3. **Given** self-damage, terrain-only deformation, or a Bunker Buster impact, **When** it resolves, **Then** it earns $0 damage income.

### Edge Cases

- Bunker Buster direct contact, adjacent impact, underground effect, and terrain settling must all preserve zero health damage; no fall damage is introduced.
- Damage income uses health actually removed after clamping at zero, not nominal blast damage, child-projectile count, or explosion count.
- Simultaneous eliminations use the current authoritative configured-player resolution order for deterministic placement assignment; a true all-eliminated draw receives no invented first-place bonus.
- Players below third retain damage income but get no placement award.
- No save-game or inventory migration is required because session state is current-executable only.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The game MUST remove Bomb Net completely from the active weapon catalogue, IDs, inventories, initial loadouts, shop, prices, HUD, keyboard/click selection, AI handling, tests, and current documentation.
- **FR-002**: The game MUST retain Cluster Bomb as the existing multi-bomblet area-saturation option without introducing a Bomb Net replacement or unnecessary Cluster Bomb redesign.
- **FR-003**: Bunker Buster MUST remain purchasable at a price reflecting its major positional value and MUST create a dramatically larger/deeper terrain effect than ordinary crater weapons on the current battlefield.
- **FR-004**: Bunker Buster terrain change MUST preserve a deep, directional, penetrating excavation identity and use the existing terrain/deformation rules safely.
- **FR-005**: Bunker Buster MUST apply exactly zero direct and splash health damage to every tank, including its shooter, regardless of collision, proximity, deformation, or settling outcome.
- **FR-006**: Large Bunker Buster deformation MUST preserve valid terrain, bounds, normals, settling, and later projectile collision without invalid numeric state or unbounded processing.
- **FR-007**: The presentation and camera response for Bunker Buster MUST make the terrain transformation legible without falsely communicating player health damage.
- **FR-008**: Damage income MUST equal $10 multiplied by actual health removed from opponents; self-damage, overkill, terrain effects, and event/projectile counts MUST not increase it.
- **FR-009**: Final placement awards MUST be exactly $5,000 for first, $2,500 for second, $1,000 for third, and $0 below third.
- **FR-010**: Placement MUST be determined from existing round/elimination state with deterministic configured-player ordering for eliminations in one authoritative resolution; a draw receives no fabricated winner or first-place award.
- **FR-011**: The game MUST remove elimination, winner, survival, participation, hit, terrain, and any other reward sources beyond damage income and placement awards.
- **FR-012**: The accounting display and wallet/shop balance MUST show and use only damage income and placement income, with totals that reconcile exactly.
- **FR-013**: Focused regression tests MUST cover removal, zero Bunker Buster damage, materially larger stable terrain deformation, actual-damage income, podium awards, overkill, multi-target damage, and deterministic simultaneous/draw outcomes.
- **FR-014**: README and roadmap references to the active arsenal and reward model MUST reflect this cleanup without rewriting unrelated project history.

### Key Entities

- **Weapon Catalogue**: The currently selectable and purchasable weapons, their identity, inventory rules, price, and impact behavior.
- **Bunker Buster Resolution**: A terrain-first shot outcome with major directional deformation and no health-damage component.
- **Round Earnings**: Per-player money from actual opponent health removed and final placement only.
- **Placement**: Deterministic final first/second/third ranking derived from round survival and elimination state.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Bomb Net appears in zero active catalogue, loadout, shop, HUD, AI, or selection paths, while Cluster Bomb remains usable.
- **SC-002**: In direct-hit, nearby, deformation-region, multi-tank, and self-impact trials, Bunker Buster changes each tank's health by exactly 0.
- **SC-003**: Representative Bunker Buster terrain trials remove materially more depth and affected terrain than Basic Shell and High Explosive while preserving valid subsequent terrain queries and projectile collision.
- **SC-004**: Controlled damage rewards exactly match $10 per actual opponent health point for 1, 10, 37, and 100 removed health, including 17-health overkill yielding $170.
- **SC-005**: Two-, three-, and four-to-eight-player completion tests award only the exact $5,000/$2,500/$1,000 podium values and $0 below third, with no elimination or separate winner additions.
- **SC-006**: The accounting view lets a player reconcile every round total using one equation: actual opponent health removed × $10 plus placement bonus.

## Assumptions

- Existing configured-player ordering is a sufficiently stable deterministic tie-break for simultaneous eliminations because the current authoritative resolver already processes that order.
- Bunker Buster price will receive one bounded sanity adjustment; comprehensive economy balancing remains outside scope.
- Existing terrain settlement remains positional only and causes no health damage.
- No persistence migration, team rules, tournament standings, new weapons, upgrades, or general economy system are required.

## Out of Scope

- Replacement weapons, Cluster Bomb redesign, fall/crushing damage, kill/accuracy/survival/participation rewards, team or tournament scoring, dynamic pricing, persistent profiles, terrain restoration, and broad shop rebalancing.
