# Feature Specification: Impact Screen Flash

**Feature Branch**: `20260910-072844-impact-screen-flash`

**Created**: 2026-09-10

**Status**: Draft

**Input**: User description: "Add a brief full-screen white impact pulse when a projectile explosion occurs."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Feel an Explosion's Force (Priority: P1)

As a player, I see one very brief white impact pulse when a projectile explosion resolves, so the explosion, sound, crater, damage, and impact framing feel more forceful.

**Why this priority**: It is a compact, readable payoff for the game's central artillery event.

**Independent Test**: Fire Basic, Heavy, and High Explosive shells into terrain and confirm each resolved explosion creates one quick pulse that has fully faded before aftermath awareness is lost.

**Acceptance Scenarios**:

1. **Given** a projectile resolves one terrain explosion, **When** its presentation begins, **Then** one full-screen white pulse appears and rapidly fades to normal.
2. **Given** High Explosive uses a larger existing impact profile, **When** it explodes, **Then** its pulse is modestly stronger and/or longer than baseline without weapon-name-specific rules.
3. **Given** one explosion damages or eliminates several tanks, **When** its consequences resolve, **Then** it creates only one pulse.
4. **Given** an out-of-bounds projectile has no terrain explosion, **When** it resolves, **Then** no impact pulse occurs.

---

### User Story 2 - Keep the Battle Comfortable and Authoritative (Priority: P1)

As a player, I retain awareness of the impact aftermath and can watch consecutive AI shots comfortably, while the flash remains entirely presentation-only.

**Why this priority**: Visual force is useful only when it does not become strobing, obscure the result, or influence gameplay.

**Independent Test**: Play consecutive Human and AI shots, including multiplayer splash impacts, and compare authoritative outcomes with pulse observation enabled or bypassed.

**Acceptance Scenarios**:

1. **Given** repeated shots occur, **When** each impact resolves, **Then** pulses remain single, short, and comfortable rather than persisting or strobing.
2. **Given** camera, audio, damage, terrain, settling, or turn resolution changes timing, **When** an impact occurs, **Then** the pulse neither gates nor changes any of them.
3. **Given** a flash cannot be shown, **When** an explosion resolves, **Then** the existing explosion and authoritative outcome continue normally.

### Edge Cases

- Immediate impact produces one safe pulse.
- High scale values remain bounded and fade quickly.
- A new impact after the previous pulse has faded creates a new pulse; a persistent prior impact does not.
- The flash must not create per-tank or per-elimination repetition.

## Requirements *(mandatory)*

- **FR-001**: The game MUST present at most one full-screen white pulse for each resolved terrain explosion.
- **FR-002**: The pulse MUST start from the existing resolved impact presentation boundary, not input, camera mode, predicted collision, damage, or tank elimination.
- **FR-003**: The pulse MUST fade rapidly and use bounded intensity and duration suitable for consecutive shots.
- **FR-004**: Existing impact-profile scale MUST be the sole source for modest pulse variation; controller type and weapon display name MUST NOT select it.
- **FR-005**: The pulse MUST remain entirely presentational and MUST NOT alter projectile physics, damage, terrain, camera timing, turn progression, AI, or audio.
- **FR-006**: Automated coverage MUST verify once-per-impact selection, bounded scale/fade, High Explosive variation, out-of-bounds silence, multi-tank non-repetition, and authoritative independence.

### Key Entities

- **Impact flash**: A short-lived full-screen presentation pulse owned by the already-resolved terrain impact.
- **Pulse profile**: Bounded opacity and duration derived from the existing impact scale.
- **Pulse consumption state**: Presentation-only state ensuring the persistent latest impact produces one pulse.

## Success Criteria *(mandatory)*

- **SC-001**: In 20 Basic, Heavy, and High Explosive terrain impacts, 20 of 20 produce one pulse and 0 produce duplicate pulses from splash or elimination.
- **SC-002**: In 20 consecutive Human/AI impacts, every pulse has faded before the next normal impact aftermath must be read, with no persistent white overlay or repeated strobing.
- **SC-003**: High Explosive pulses are visibly stronger or longer than baseline in 10 of 10 manual comparisons while remaining comfortable.
- **SC-004**: With pulse presentation bypassed, representative impact position, damage, terrain, settling, survivors, and match result remain identical.

## Assumptions

- The existing impact visual uses a once-per-impact presentation boundary and exposes a bounded explosion visual scale.
- The current HUD can host a temporary full-screen presentation overlay without replacing match context.
- Accessibility settings are out of scope; restraint is supplied through conservative default bounds.

## Roadmap Alignment

This may complete “Deliberately exaggerated presentation where appropriate” only after manual acceptance confirms a restrained, useful result. It does not complete terrain debris, smoke, direct impact behaviour, or any gameplay item.
