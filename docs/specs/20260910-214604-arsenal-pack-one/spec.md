# Feature Specification: Arsenal Pack #1 — Cluster Bomb, Bomb Net, Roller and Bunker Buster

**Feature Branch**: `20260910-214604-arsenal-pack-one`
**Created**: 2026-09-10
**Status**: Draft

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Saturate a Known Region (Priority: P1)

As a player, I can use Cluster Bomb to convert a descending carrier into a dense local barrage, so I can saturate a roughly known region rather than use MIRV's longer spread.

**Independent Test**: Fire at a ridge or nearby tanks; observe one carrier, deterministic late deployment, 8–12 small wind-affected children, and completion after the final consequence.

### User Story 2 - Cover a Broad Region (Priority: P1)

As a player, I can use Bomb Net to deploy a visibly broad 4×4-like pattern of modest falling bombs, so approximate aim can make a large valley unsafe.

**Independent Test**: Fire across open terrain and verify a deterministic two-axis pattern, independent child impacts, materially wider coverage than Cluster, and one-round consumption.

### User Story 3 - Let Terrain Deliver an Attack (Priority: P1)

As a player, I can land Roller on terrain and let carried landing momentum and slope move it before detonation, so I can attack beyond direct ballistic reach.

**Independent Test**: Land it on downhill, uphill, and cross-slope terrain; verify current-surface following, slope-influenced path, bounded movement, and deterministic final explosion.

### User Story 4 - Attack Through Terrain (Priority: P1)

As a player, I can use Bunker Buster to bury a projectile before it detonates, so protected positions receive a visibly deeper, narrower terrain strike than surface HE.

**Independent Test**: Compare a representative HE hit with a Bunker Buster strike; verify normal flight, no immediate surface blast, bounded velocity-directed penetration, internal detonation, and normal final consequences.

### Edge Cases

- Child ordering is deterministic and later children query terrain after earlier crater changes.
- Roller neither floats nor tunnels and completes on a bounded stop/time/distance rule.
- Bunker Buster penetration remains finite and valid without an underground simulation.
- Presentation may coalesce rapid impacts but never omits authority or delays handoff.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Add limited Cluster Bomb (2), Bomb Net (1), Roller (2), and Bunker Buster (2) to existing selection, HUD, inventory, and shared firing paths.
- **FR-002**: Cluster and Bomb Net MUST use normal deterministic carrier ballistics until descent deployment; Cluster creates 8–12 concentrated children, Bomb Net a readable broad two-axis 4×4-like pattern.
- **FR-003**: Deployed bombs MUST inherit carrier motion plus deterministic offsets, use ordinary gravity/wind/collision/impact independently, and have modest profiles; Net coverage MUST be wider/lower-density than Cluster.
- **FR-004**: Roller MUST use normal initial flight, then deterministic terrain-surface rolling influenced by landing momentum and local two-axis slope, ending with a bounded understandable detonation rule.
- **FR-005**: Bunker Buster MUST use normal initial flight, replace surface impact with bounded impact-direction penetration, then resolve one normal internal explosion with deformation deeper/narrower than HE.
- **FR-006**: Shots MUST remain resolving through every child, rolling/penetration phase, deformation, damage, elimination, and settling; advance exactly once only when complete.
- **FR-007**: Deployment, patterns, child flight, rolling, penetration, impact ordering, terrain, damage, and handoff MUST reproduce for equal authority inputs.
- **FR-008**: Human camera MUST show carrier then Cluster/Net aggregate region, Roller plus terrain path, and Bunker Buster impact/deep result; AI remains tactical-wide.
- **FR-009**: Rapid multi-impact pulse/audio MUST remain bounded; presentation never changes authoritative results.
- **FR-010**: AI remains autonomous with its current supported behavior; AI weapon strategy is excluded.
- **FR-011**: Tests MUST cover inventory, deterministic Cluster/Net patterns and width difference, independent children/complete-shot resolution, Roller slope behavior/termination, Bunker Buster penetration/deformation, and Basic/HE/Heavy/MIRV regression.
- **FR-012**: Do not add scripting, behavior graphs, rigid-body physics, general underground simulation, fuse controls, hazards, bouncing/tunnelling, shop/progression, or future weapon families.

### Key Entities

- **Carrier deployment**: deterministic carrier-to-pattern transition.
- **Terrain-contact state**: minimal explicit Roller or Bunker Buster post-contact state.
- **Rolling projectile**: bounded terrain follower with momentum and slope influence.
- **Penetrating projectile**: bounded deferred-detonation strike along impact direction.

## Success Criteria *(mandatory)*

- **SC-001**: Twenty repeats of each representative weapon fixture produce identical authoritative outcomes.
- **SC-002**: Ten Cluster/Net shots visibly demonstrate dense versus broad coverage, with Net's representative footprint larger.
- **SC-003**: Ten Roller reviews visibly show downhill, uphill, and cross-slope influence.
- **SC-004**: Ten Bunker Buster comparisons show delayed internal detonation and deeper/narrower terrain than comparable HE.
- **SC-005**: Existing weapon regressions pass and no shot hands off before all behavior and settling resolve.

## Assumptions

- MIRV's demonstrated multi-projectile ownership may be extended only as these weapons require.
- Existing terrain queries can support a first deterministic slope estimate; no physics engine is needed.
- Exact counts, spacing, roll budgets, and penetration depth are playtest tuning, while tactical identities are mandatory.

## Roadmap Alignment

This feature may complete Cluster Bomb, Bomb Net, deep/narrow penetrator, Rolling Bomb, and slope-affected terrain following only after delivery and acceptance. Milestone G's distinct-weapons and meaningful-choice boxes require play evidence. Configurable split timing, bouncing, tunnelling, terrain-building, hazards, and future arsenal candidates remain open.

## Architecture Review

- **Do weapons reuse authoritative physics and complete-shot resolution where appropriate?** It must be yes.
- **Do Cluster and Net naturally extend multi-projectile behavior?** It must be yes.
- **Do Roller and Bunker Buster remain bounded behaviors rather than general engines?** It must be yes.
- **Are results deterministic and abstractions earned?** It must be yes.
