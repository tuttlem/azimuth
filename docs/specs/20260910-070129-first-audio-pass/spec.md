# Feature Specification: First Audio Pass — Fire, Flight, Impact and Battlefield Ambience

**Feature Branch**: `20260910-070129-first-audio-pass`

**Created**: 2026-09-10

**Status**: Draft

**Input**: User description: "First Audio Pass — Fire, Flight, Impact and Battlefield Ambience"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Hear a Satisfying Shot (Priority: P1)

As a player, I hear an immediate, weighty report when either a Human or AI successfully fires a weapon, so the start of every artillery shot feels physical and intentional.

**Why this priority**: The firing report begins the core aim → fire → observe loop and establishes the feature's value even before flight, impact, or ambience is added.

**Independent Test**: Start a match with Human and AI players; launch each available weapon from both controller types and confirm exactly one physical firing cue follows every successful launch, with none for rejected or unavailable fire attempts.

**Acceptance Scenarios**:

1. **Given** a Human player successfully commits a weapon, **When** its projectile launches, **Then** a firing cue occurs from that weapon's firing location and not merely from the input.
2. **Given** an AI successfully launches the same weapon, **When** its projectile launches, **Then** it receives the same physical firing treatment as the Human launch.
3. **Given** a player cannot fire because the match, turn, movement, ammunition, or shot state rejects the action, **When** they attempt it, **Then** no successful-shot firing cue occurs.
4. **Given** a Heavy Shell is launched, **When** modest weapon variation is available, **Then** its firing report may sound heavier without changing gameplay behaviour; otherwise it uses the shared firing family.

---

### User Story 2 - Follow the Shell by Ear (Priority: P1)

As a match observer, I hear a restrained travelling-shell cue during useful portions of a shot, so long arcs retain anticipation without becoming a continuous nuisance.

**Why this priority**: Human follow shots and AI tactical-wide shots both benefit from a quiet, shared physical flight cue that makes the battlefield more readable and less empty.

**Independent Test**: Fire short, long, and high-arc shots from both controller types, including a shot crossing much of the battlefield, and verify that flight is perceptible but subordinate to launch and impact.

**Acceptance Scenarios**:

1. **Given** any successfully launched projectile is in useful flight, **When** a travelling cue improves anticipation, **Then** it is audible without depending on whether the shooter is Human or AI.
2. **Given** a Human shot uses its follow presentation, **When** the projectile climbs and descends, **Then** flight audio complements the camera without becoming an aggressive constant loop.
3. **Given** an AI shot uses its tactical presentation, **When** the shell crosses the battlefield, **Then** the same physical flight treatment remains understandable from the wider view.
4. **Given** a flight ends immediately or leaves the battlefield, **When** no useful flight remains, **Then** any travelling cue stops safely without inventing an impact event.

---

### User Story 3 - Feel the Impact and Consequences (Priority: P1)

As a player, I hear a substantial explosion when an authoritative terrain impact resolves, so the visual blast, crater, damage, elimination, and shared impact camera have audible weight.

**Why this priority**: A satisfying impact is the payoff for every artillery decision and the largest current gap in the game's presentation identity.

**Independent Test**: Produce terrain misses, direct and splash hits, High Explosive impacts, self-hits, eliminations, and match-ending shots; verify one appropriate explosion cue per resolved impact, unchanged gameplay results, and no uncontrolled wall of sound.

**Acceptance Scenarios**:

1. **Given** a projectile resolves a terrain impact, **When** explosion and crater consequences occur, **Then** one weighty explosion cue corresponds to that resolved impact.
2. **Given** a High Explosive impact has a larger existing impact profile, **When** it is heard, **Then** it is clearly more substantial than the baseline where a simple scale variation is available.
3. **Given** an explosion eliminates one or more tanks, **When** an additional destruction cue materially improves readability, **Then** it is a controlled accent rather than a duplicate explosion for every eliminated tank.
4. **Given** ordinary damage does not eliminate a tank, **When** the explosion resolves, **Then** no tank-destruction cue is falsely presented.
5. **Given** sound is unavailable, muted, delayed, or an asset cannot play, **When** an impact resolves, **Then** damage, deformation, settling, elimination, result, and handoff continue normally.

---

### User Story 4 - Hear a Living Battlefield (Priority: P2)

As a player, I hear subtle wind-led battlefield ambience between major events, so the world no longer feels silent while launch, flight, and impact retain dramatic contrast.

**Why this priority**: Wind is already a stable, visible battlefield condition and provides the smallest coherent foundation for environmental identity without adding music or a broad soundscape.

**Independent Test**: Compare representative weak and strong match wind conditions through several Human and AI turns and verify that ambience is audible but subordinate, bounded, and does not alter wind or gameplay.

**Acceptance Scenarios**:

1. **Given** a match is active, **When** its wind condition is weak, **Then** battlefield ambience is quiet or nearly quiet.
2. **Given** a match is active, **When** its wind condition is stronger, **Then** ambience becomes perceptibly stronger while remaining below major gameplay cues.
3. **Given** a long projectile trajectory, **When** the firing report fades and flight treatment is restrained, **Then** quieter ambience preserves anticipation before impact.
4. **Given** ambience cannot play, **When** the match continues, **Then** wind, projectile motion, terrain, AI decisions, and turn progression remain unchanged.

### Edge Cases

- A failed or blocked firing attempt must never produce the same cue as a successful launch.
- Immediate terrain impact and out-of-bounds flight must stop or bypass flight treatment safely.
- Multiple affected or eliminated tanks must not create clipping, repeated detonation cues, or an uncontrolled wall of sound.
- Distant, nearby, and rapidly reframed events must retain basic positional coherence where available, without acoustic simulation.
- Missing, malformed, muted, delayed, or unavailable presentation assets must not make a match unusable or cause a simulation failure.
- Audio variation, if any, must not consume or perturb authoritative match randomness.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The game MUST request a firing cue only after a weapon has successfully launched a projectile through the shared authoritative firing path.
- **FR-002**: Human and AI launches of the same weapon MUST request the same physical firing and flight treatment; controller type and player display name MUST NOT select different physics sounds.
- **FR-003**: The game MUST provide a clear firing cue for Basic Shell, High Explosive, and Heavy Shell launches, with a straightforward definition-led path for modest weapon-specific variation.
- **FR-004**: The game MUST provide restrained projectile-flight treatment when it improves active-projectile readability and anticipation, and MUST stop it when flight ends.
- **FR-005**: The game MUST request an explosion cue from the resolved terrain-impact consequence, not from predicted collision, camera state, input, or audio completion.
- **FR-006**: The game MUST allow the existing impact profile to influence the apparent scale of an explosion cue through a small, understandable mapping.
- **FR-007**: If tank-destruction treatment is included, it MUST be requested only for newly eliminated tanks and MUST remain a restrained accent to the impact cue.
- **FR-008**: The game MUST provide low-level battlefield ambience whose bounded intensity responds to existing match wind strength without mutating wind or environment state.
- **FR-009**: World events that have meaningful battlefield locations SHOULD use basic positional coherence and listener-relative distance treatment when naturally supported; advanced acoustics are not required.
- **FR-010**: The game MUST keep explosion and weapon fire stronger than projectile flight, destruction accents secondary to their triggering explosion, and ambience quieter than gameplay events.
- **FR-011**: All audio presentation MUST remain optional: muted, missing, delayed, or unavailable cues MUST NOT block or alter projectile trajectory, damage, terrain deformation, settling, elimination, winner/draw resolution, turn progression, AI decisions, or input validity.
- **FR-012**: Automated coverage MUST verify successful and rejected fire cue selection, shared Human/AI physical event selection, impact scale selection, optional elimination selection, wind ambience bounds, and simulation independence without requiring audible playback.
- **FR-013**: The feature MUST document provenance and licence for every bundled sound asset and clearly identify any generated or placeholder asset.
- **FR-014**: The feature MUST retain useful silence and anticipation between major events rather than filling a match with loud continuous sound.
- **FR-015**: The feature MUST NOT add music, an audio settings screen, detailed mixer, advanced acoustics, voice work, soundtrack management, a generic audio framework, terrain occlusion, environmental reverb, or a production sound library.

### Key Entities

- **Audio presentation request**: A non-authoritative request to present a recognised gameplay consequence such as launch, flight, impact, destruction, or ambience.
- **Weapon audio identity**: The small definition-owned description that permits a weapon's launch and impact cues to share or modestly vary a sound family.
- **Active flight presentation**: The short-lived, read-only representation of a currently travelling projectile's audible treatment.
- **Impact audio context**: The resolved impact location and existing impact scale used to request one explosion cue and any justified destruction accent.
- **Wind ambience profile**: A bounded, presentation-only relationship between existing match wind strength and background battlefield intensity.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 20 representative successful launches spanning Human and AI players and all three current weapons, 20 of 20 produce exactly one firing presentation request; 0 of 20 rejected firing attempts produce one.
- **SC-002**: In 12 representative short, long, and high-arc shots across Human and AI turns, all active flights have either one bounded flight treatment or an explicitly safe silent fallback, and no treatment survives after the projectile ends.
- **SC-003**: In terrain-miss, direct-hit, High Explosive, multi-hit, self-hit, elimination, and final-result impact variants, 100% of resolved impacts produce one corresponding explosion presentation request while authoritative outcomes remain identical with audio disabled.
- **SC-004**: In manual review of near and far shots, observers can distinguish at least 8 of 10 firing or explosion events as nearer versus farther than the active view when positional treatment is available.
- **SC-005**: In weak versus strong representative wind conditions, reviewers can distinguish ambience level in 8 of 10 comparisons, while no reviewer identifies ambience as louder than associated firing or explosion events.
- **SC-006**: Across a complete 2-, 4-, or 8-player match containing Human and AI turns, reviewers report materially more satisfying launch and impact feedback, with no recurring clipping, painful repetition, or loss of useful quiet.

## Assumptions

- The current match has one active projectile and exposes successful launch, resolved terrain impact, elimination, camera/listener, and wind state that presentation can observe.
- A small curated set of appropriately licensed sound assets is sufficient for this first pass; bespoke sounds for every weapon and environment are deferred.
- Existing weapon and impact definitions are the natural source of any audio-scale or family variation; a weapon display name is not an audio rule.
- Basic positional treatment should be used only where it fits naturally within the current game technology; no physical-acoustics model is needed.
- Tank destruction and brief terrain/debris accents are optional refinements: include them only if existing event boundaries make them small and clearly beneficial.
- UI audio is secondary and remains out of scope unless one significant setup or weapon-selection cue is trivial and does not distract from gameplay audio.

## Dependencies

- Existing shared successful firing path used by Human and AI turns.
- Existing weapon definitions and captured projectile/impact profiles.
- Existing projectile lifecycle, resolved terrain impact, explosion visual, tank elimination, and presentation-only shot camera.
- Existing match wind state, active gameplay camera, HUD, and presentation asset conventions.
- Appropriately licensed, documented audio assets that can be safely unavailable at runtime.

## Roadmap Alignment

This feature directly advances weapon firing sound, projectile flight sound, explosion sound, environmental ambience, wind ambience, slightly exaggerated arcade-like effects, and dramatic silence/anticipation. It may also complete tank destruction sound only if the delivered event boundary and manual review justify it. Terrain/debris sound, UI sounds, music, unusual-weapon identity, and advanced environment audio remain separate work unless explicitly delivered.

Manual acceptance evidence is required before marking the Milestone H item “Audio gives shots and impacts satisfying weight” complete. The preceding shot-camera feature's current implementation and acceptance evidence must also be reconciled with any still-open camera roadmap checkbox before audio implementation completion.

## Manual Acceptance

- Fire Basic Shell, High Explosive, and Heavy Shell as a Human; confirm immediate weight, useful anticipation, satisfying impact, and no misleading weapon implication.
- Run several AI turns and an all-AI sequence; confirm equivalent physical cues work with the tactical-wide view and remain meaningful at distance.
- Compare nearby, distant, crossing, and fast-reframed shots; confirm placement and distance feel coherent rather than broken.
- Compare weak and strong wind matches; confirm ambience is distinguishable but unobtrusive.
- Play a complete multiplayer match with headphones or speakers; confirm repeated cues remain pleasant, impacts rewarding, and quiet intervals useful.
- Repeat relevant deterministic checks with audio unavailable; confirm gameplay results are unchanged.

## Architecture Review

- Could all audio be disabled without changing gameplay? **It must be yes.**
- Do Human and AI shots converge on the same physical sound events? **It must be yes.**
- Does audio observe authoritative events rather than recreate their rules? **It must be yes.**
- Is any audio variation isolated from gameplay determinism? **It must be yes.**
