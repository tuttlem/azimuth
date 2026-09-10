# Feature Specification: Arsenal Pack #2 — Dirt Bomb, Curve Ball, Bouncer and Nuke

**Feature Branch**: `20260910-224611-arsenal-pack-two`
**Created**: 2026-09-10
**Status**: Draft

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Build and Bury with Dirt (Priority: P1)

As a player, I can fire Dirt Bomb to build a substantial mound at its normal ballistic impact point, so I can create cover, fill craters, alter routes, or comically bury a tank instead of merely removing terrain.

**Why this priority**: It creates the first inverse terrain weapon and immediately expands tactical terrain play.

**Independent Test**: Fire at flat ground, a crater, a slope, and beside a tank; verify a visible deterministic mound, no meaningful blast damage, and coherent tank support/settling.

**Acceptance Scenarios**:

1. **Given** an available Dirt Bomb, **When** it strikes open terrain, **Then** the impacted region becomes materially higher and nearby unaffected terrain does not change.
2. **Given** a tank beside an impact, **When** dirt is deposited around it, **Then** the tank remains a valid game entity and its position/support responds through the ordinary terrain rules.

---

### User Story 2 - Bend a Learned Shot (Priority: P1)

As a player, I can select Curve Ball and choose left or right curvature from the weapon UI, so I can deliberately bend a shot around a ridge without target seeking.

**Why this priority**: Curving around terrain is a distinct learned aiming skill, not a damage variation.

**Independent Test**: Fire equal left and right Curve Balls in calm and crosswind conditions; verify opposite lateral paths, continued gravity/wind response, deterministic repeats, and no target-dependent steering.

**Acceptance Scenarios**:

1. **Given** Curve Ball is selected, **When** the player chooses left or right in the visible control, **Then** the shot bends in that direction relative to its launch direction.
2. **Given** a ridge blocks a direct path, **When** a player correctly aims a Curve Ball, **Then** it can clear around the obstruction while an equivalent ordinary path remains blocked.

---

### User Story 3 - Make a Bank Shot (Priority: P1)

As a player, I can fire Bouncer and use terrain faces as bounded ricochets before its final detonation, so terrain orientation becomes part of a bank-shot solution.

**Why this priority**: It makes collision geometry an intentional delivery mechanism.

**Independent Test**: Fire at flat, sloped, and valley terrain; verify the first contact bounces rather than explodes, each bounce loses energy, the number is bounded, and the final contact resolves normally.

**Acceptance Scenarios**:

1. **Given** Bouncer has bounce budget remaining, **When** it contacts terrain, **Then** it visibly changes direction and remains airborne without an ordinary explosion.
2. **Given** a Bouncer has exhausted its bounded bounce budget, **When** it next contacts terrain, **Then** it detonates once and completes the shot normally.

---

### User Story 4 - Spend a Nuke (Priority: P1)

As a player, I can spend one Nuke on an ordinary ballistic shot to devastate a wide region, so I can make a high-consequence decision that visibly reshapes the battlefield.

**Why this priority**: It provides the first intentionally absurd, match-altering arsenal choice.

**Independent Test**: Fire at a populated region; verify one-round consumption, substantially broader damage/deformation than HE, possible multi-player/self elimination, stable winner/draw handling, and a readable large aftermath.

**Acceptance Scenarios**:

1. **Given** a player has one Nuke, **When** it impacts terrain, **Then** it creates a clearly exceptional blast region and crater while still applying ordinary radial self-damage.
2. **Given** several tanks are inside that region, **When** the Nuke resolves, **Then** every affected tank is evaluated by the existing elimination and winner/draw rules.

### Edge Cases

- Dirt deposited near a boundary or a tank must remain finite, bounded to the playable terrain, and use the same current surface as collision and support.
- Curve choice must have a safe visible default for a newly selected weapon and must never read tank locations.
- Bouncer must not tunnel through a steep surface, bounce indefinitely, or create full-impact presentation on intermediate contacts.
- Nuke deformation near battlefield edges must remain deterministic and valid; simultaneous eliminations may produce a draw, including the firing player.
- All twelve weapon boxes must remain selectable by mouse without requiring an additional unique keyboard shortcut.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Add limited Dirt Bomb, Curve Ball, Bouncer, and Nuke to each player's existing inventory and clickable bottom weapon strip; Nuke starts at one round and the other starting quantities are small playtest quantities.
- **FR-002**: The strip MUST display only currently available weapons, identity, selected state, and remaining rounds; all four new weapons MUST be mouse-selectable through the same authoritative selection state as existing weapons.
- **FR-003**: Dirt Bomb MUST follow ordinary launch, gravity, wind, and terrain collision before replacing the usual crater with a substantial deterministic radial terrain addition; it MUST do little or no conventional damage.
- **FR-004**: Dirt Bomb terrain addition MUST share the authoritative current terrain surface used by rendering, later collision, tank support, and settling; it MUST not add special burial deaths or excavation controls.
- **FR-005**: Curve Ball MUST use ordinary launch, gravity, wind, collision, and impact consequences plus a deterministic player-selected left/right lateral bend relative to launch orientation.
- **FR-006**: Curve Ball controls MUST be visible and mouse-coherent when Curve Ball is selected, have an understandable default, and must not inspect tanks, target positions, or an aiming solver.
- **FR-007**: Bouncer MUST use ordinary initial flight and, before a bounded final detonation, reflect from the current terrain surface with reduced energy; surface orientation MUST materially affect its continued 3D trajectory.
- **FR-008**: Bouncer intermediate contacts MUST use restrained bounce presentation, retain gravity and wind during subsequent flight, and never generate ordinary explosion damage, deformation, or a full screen pulse before final detonation.
- **FR-009**: Nuke MUST use ordinary ballistic delivery and one limited round, then compose the existing radial damage, deformation, settling, elimination, and winner/draw rules at an unmistakably larger scale than existing weapons.
- **FR-010**: Nuke presentation MUST communicate one large event through wider aftermath framing, an appropriately enlarged visual/audio response, and a comfortable single impact pulse; presentation MUST not alter authoritative resolution.
- **FR-011**: Every new weapon MUST remain deterministic for equal authoritative inputs and retain the complete-shot lifecycle through impact, deformation, tank consequences, and handoff.
- **FR-012**: Human and AI shot cameras MUST remain controller-aware: Human shots preserve readable terrain context for curve/bounce and impact scale; AI remains tactical-wide and autonomous with safe defaults.
- **FR-013**: Tests MUST cover deterministic terrain addition, opposite Curve Ball displacement with gravity/wind, bounded terrain-normal Bouncer reflection, Nuke multi-target/self-damage and deformation, inventory, full-shot resolution, and all eight prior weapon regressions.
- **FR-014**: This feature MUST NOT add homing, generic projectile scripting/behaviour graphs, a rigid-body engine, persistent hazards, shops/progression, AI weapon strategy, Death Sphere, or other future weapon families.

### Key Entities

- **Terrain addition profile**: a bounded mound consequence with no strong conventional blast.
- **Curve choice**: the selected Curve Ball handedness captured with a committed shot.
- **Bounce state**: remaining ricochet budget and energy carried by one projectile.
- **Nuke impact profile**: limited high-consequence radial damage, deformation, and presentation scale.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Twenty repeated fixtures for each new weapon produce identical authoritative terrain, trajectory, damage, and handoff results.
- **SC-002**: Dirt Bomb raises terrain materially at every representative impact while leaving terrain outside its defined affected region unchanged.
- **SC-003**: Equal left/right Curve Ball fixtures finish with opposite meaningful lateral displacement and retain non-zero wind influence.
- **SC-004**: Bouncer fixtures demonstrate at least one continued post-contact trajectory for each of three distinct terrain orientations and always terminate within the configured bounce limit.
- **SC-005**: A representative Nuke affects a materially wider region and creates a visibly larger lasting terrain change than HE while preserving valid survivor, winner, and draw results.
- **SC-006**: In manual matches, players can explain a distinct reason to choose each of the twelve available weapons; no new weapon depends on a keyboard-only selection path.

## Assumptions

- Existing current-surface deformation, radial damage, fixed-step simulation, settling, cameras, and click-first weapon strip are authoritative integration points.
- Curve Ball has a fixed, learnable bend strength and left/right choice; no strength selector is needed for this first version.
- Bouncer begins with a small fixed maximum of three bounces; restitution and Nuke/Dirt Bomb scale are playtest tuning values.
- Dirt Bomb may leave tanks buried or movement-constrained through existing terrain relationships, but does not require new escape rules.
- Death Sphere is recorded as the only immediate future weapon follow-up; all other future weapon ideas remain roadmap items.

## Roadmap Alignment

This feature may complete terrain-building Dirt Bomb, bouncing projectile, Curve Ball, a large-scale destructive weapon, and Milestone G's ridiculous weapon only after implementation and play acceptance. Death Sphere, hazards, homing, and all unselected families remain open.

## Architecture Review

- **Does Dirt Bomb add terrain without corrupting current terrain authority?** It must be yes.
- **Can Curve Ball bend in 3D without homing?** It must be yes.
- **Does Bouncer use surface orientation plus normal projectile flight rather than scripted paths?** It must be yes.
- **Does Nuke gain scale by composing existing consequences?** It must be yes.
- **Does the strip scale through clickable boxes rather than one-key-per-weapon?** It must be yes.
- **Are deterministic behaviour and earned abstractions preserved?** It must be yes.
