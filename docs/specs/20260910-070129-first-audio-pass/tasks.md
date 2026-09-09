---

description: "Actionable implementation tasks for the first presentation-only audio pass"
---

# Tasks: First Audio Pass

**Input**: Design documents in `docs/specs/20260910-070129-first-audio-pass/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/audio-presentation.md, quickstart.md

**Tests**: Required by the feature specification. Add module-local tests for cue selection, lifecycle, profile scale, wind bounds, and non-authority; do not require an audio device or waveform playback.

**Organization**: All runtime work belongs in `crates/azimuth-game/src/main.rs`, so it is intentionally serial. Asset/provenance work may proceed after the cue contract is stable.

## Phase 1: Setup

**Purpose**: Establish the exact presentation boundaries and clean baseline.

- [x] T001 Inspect the current launch, projectile, impact, elimination, camera listener, wind, asset, and update ordering boundaries in crates/azimuth-game/src/main.rs
- [x] T002 Run and record the focused baseline with cargo test -p azimuth-game from the workspace root
- [x] T003 Verify native audio support is available through the existing Bevy dependency and record the no-new-dependency decision in docs/specs/20260910-070129-first-audio-pass/research.md

---

## Phase 2: Foundational Audio Presentation Boundary

**Purpose**: Add the small non-authoritative cue model shared by every story.

**CRITICAL**: Complete this phase before implementing any audible story.

- [ ] T004 Add a finite presentation-only audio cue/state model, central gain/pitch tuning, and pure selection helpers in crates/azimuth-game/src/main.rs
- [ ] T005 Add unit coverage for cue identity, successful versus rejected launch selection, Human/AI equality, and no display-name/controller-specific physics branch in crates/azimuth-game/src/main.rs
- [x] T006 Add engine-native audio asset handles, active gameplay-camera listener integration, safe playback spawning, and automatic cleanup in crates/azimuth-game/src/main.rs
- [ ] T007 Add a documented small asset manifest and provenance/licence record in crates/azimuth-game/assets/audio/README.md

**Checkpoint**: A cue can be selected and safely ignored or presented without mutating simulation state.

---

## Phase 3: User Story 1 - Hear a Satisfying Shot (Priority: P1) 🎯 MVP

**Goal**: Every successful Human or AI weapon launch receives one immediate, positional firing cue.

**Independent Test**: Launch Basic, High Explosive, and Heavy Shell as Human and AI; each successful shared launch requests one appropriate cue, while blocked or unavailable fire requests none.

### Tests for User Story 1

- [ ] T008 [US1] Add failing tests for one successful fire cue, rejected fire silence, Human/AI equality, and definition-led Heavy variation in crates/azimuth-game/src/main.rs

### Implementation for User Story 1

- [x] T009 [US1] Extend the existing shared successful fire_current_player boundary to emit exactly one captured firing presentation request in crates/azimuth-game/src/main.rs
- [ ] T010 [US1] Map existing weapon definitions and captured projectile profiles to a shared fire family with optional bounded Heavy variation in crates/azimuth-game/src/weapon.rs
- [ ] T011 [US1] Consume firing requests as safe positional one-shot playback at the firing tank/weapon position in crates/azimuth-game/src/main.rs

**Checkpoint**: Firing alone is independently useful and does not affect valid/invalid launch behaviour.

---

## Phase 4: User Story 2 - Follow the Shell by Ear (Priority: P1)

**Goal**: The sole active projectile has a restrained shared flight cue that starts after launch and reliably ends with flight.

**Independent Test**: Human and AI short, long, high-arc, immediate-impact, and out-of-bounds shots produce one bounded flight lifecycle with no surviving loop after the projectile ends.

### Tests for User Story 2

- [ ] T012 [US2] Add failing tests for shared Human/AI flight selection, one-active-flight bounds, immediate-impact handling, and out-of-bounds cleanup in crates/azimuth-game/src/main.rs

### Implementation for User Story 2

- [ ] T013 [US2] Derive the active flight audio state only from read-only ProjectileFlight lifecycle and retain its world position in crates/azimuth-game/src/main.rs
- [ ] T014 [US2] Start, update, and stop one restrained positional flight loop without consulting shot-camera mode in crates/azimuth-game/src/main.rs

**Checkpoint**: Flight enhances anticipation in either camera mode and cannot invent an impact or alter flight.

---

## Phase 5: User Story 3 - Feel the Impact and Consequences (Priority: P1)

**Goal**: Each resolved terrain impact gives one scaled explosion cue and, only where useful, a controlled destruction accent.

**Independent Test**: Terrain miss, direct hit, High Explosive, multi-hit, ordinary damage, elimination, and final-result variants produce correct bounded requests with identical authoritative results when audio is absent.

### Tests for User Story 3

- [ ] T015 [US3] Add failing tests for once-per-impact selection, captured impact-profile scale, impact flight-stop ordering, new-elimination detection, and ordinary-damage silence in crates/azimuth-game/src/main.rs
- [ ] T016 [US3] Extend deterministic resolution fixtures to prove audio request evaluation leaves projectile, terrain, tanks, turn, survivors, and winner/draw unchanged in crates/azimuth-game/src/main.rs

### Implementation for User Story 3

- [ ] T017 [US3] Capture resolved impact presentation context at resolve_projectile_advance, using the existing FiredShot impact profile and before/after tank state only as read-only inputs in crates/azimuth-game/src/main.rs
- [ ] T018 [US3] Consume each new terrain-impact context as one positional explosion cue with bounded profile-led scale in crates/azimuth-game/src/main.rs
- [ ] T019 [US3] Add one optional bounded destruction accent for newly eliminated tanks and ensure multi-elimination cannot create an uncontrolled cue wall in crates/azimuth-game/src/main.rs

**Checkpoint**: Consequence audio is authoritative-event-driven, scaled by existing profile data, and safe with multiple targets.

---

## Phase 6: User Story 4 - Hear a Living Battlefield (Priority: P2)

**Goal**: Existing match wind supplies quiet, bounded ambience under the core events.

**Independent Test**: Weak and strong deterministic wind strengths map to distinct bounded ambience settings without mutating wind or gameplay.

### Tests for User Story 4

- [ ] T020 [US4] Add failing pure tests for bounded weak/strong wind ambience mapping, stable zero/safe fallback, and non-mutation of Wind in crates/azimuth-game/src/main.rs

### Implementation for User Story 4

- [ ] T021 [US4] Add one presentation-only looping wind ambience controller that observes BattlefieldWind and remains below gameplay cue tuning in crates/azimuth-game/src/main.rs

**Checkpoint**: The battlefield is no longer silent, while quiet periods and gameplay-event hierarchy remain intact.

---

## Phase 7: Polish and Cross-Cutting Validation

**Purpose**: Prove asset safety, documentation accuracy, quality, manual acceptance, and only demonstrated roadmap progress.

- [ ] T022 [P] Document current audio behaviour, asset provenance, safe failure behaviour, and validation commands in README.md and crates/azimuth-game/assets/audio/README.md
- [ ] T023 Audit all audio paths to ensure camera affects listener perspective only, requests use no match RNG, and playback cannot gate fixed-update or authoritative resolution in crates/azimuth-game/src/main.rs
- [ ] T024 Run the automated commands and complete the manual matrix in docs/specs/20260910-070129-first-audio-pass/quickstart.md
- [ ] T025 Update only manually demonstrated Audio and Milestone H roadmap checkboxes; retain undelivered terrain/debris, UI, music, and advanced-audio items in docs/roadmap.md
- [ ] T026 Run cargo fmt --all -- --check, cargo test --workspace, cargo check --workspace --all-targets, and cargo clippy --workspace --all-targets --all-features -- -D warnings from the workspace root

---

## Dependencies & Execution Order

### Phase Dependencies

- Phase 1 has no prerequisites.
- Phase 2 blocks all user stories.
- US1 establishes the shared launch cue, then US2 consumes its active flight lifecycle.
- US3 depends on the shared cue model and may follow US1 independently of US2, but is sequenced after it because all runtime work shares one source file.
- US4 depends only on Phase 2 but is sequenced after impact work to tune ambience beneath validated gameplay cues.
- Phase 7 follows all delivered stories.

### User Story Dependencies

- **US1 (P1)**: Depends on Phase 2; minimum viable audio pass.
- **US2 (P1)**: Depends on Phase 2 and shares the launch lifecycle established by US1.
- **US3 (P1)**: Depends on Phase 2 and existing resolved-impact flow; reuses shared cue consumption.
- **US4 (P2)**: Depends on Phase 2 and can be developed independently, but shares `main.rs`.

### Parallel Opportunities

- T007 asset provenance can run in parallel with T004–T006 after the cue names are agreed.
- T022 documentation can run in parallel with T024 manual review after T023 is complete.
- A reviewer can perform the manual matrix while another runs T026 after T024 begins.

## Parallel Example: Final Validation

```text
Task: "Document audio behaviour and provenance in README.md and crates/azimuth-game/assets/audio/README.md"
Task: "Run manual matrix in docs/specs/20260910-070129-first-audio-pass/quickstart.md"
```

## Implementation Strategy

### MVP First

1. Complete T001–T007.
2. Complete US1 (T008–T011).
3. Validate a successful Human and AI firing cue before adding continuous sound.

### Incremental Delivery

1. Launch cue establishes audible cause.
2. Flight cue establishes anticipation.
3. Impact and optional destruction establish payoff.
4. Wind ambience supplies quiet environmental identity.
5. Manual acceptance decides completed roadmap items.

## Format Validation

All 26 tasks use the required checkbox, sequential ID, story label in story phases, and exact file-path format.
