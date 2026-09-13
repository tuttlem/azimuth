---

description: "Executable task list for Azimuth UI Polish"
---

# Tasks: Azimuth UI Polish — A Finished Visual Language for HUD, Shop and Game Flow

**Input**: Design documents from `/docs/specs/20260912-231438-ui-polish/`

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [ui-presentation.md](contracts/ui-presentation.md), [quickstart.md](quickstart.md)

**Tests**: Add focused Rust tests for authoritative presentation mappings and run the existing workspace quality suite. Rendering quality is validated manually; no screenshot/pixel tests are required.

**Organization**: Work is grouped by user story. Shared theme and identity coverage are intentionally completed first, then the HUD and shop are independently deliverable P1 increments.

## Phase 1: Setup

**Purpose**: Establish the source-owned asset location and inspect the existing UI before changing it.

- [X] T001 Create `crates/azimuth-game/assets/ui/weapons/` and add an asset provenance/readme note at `crates/azimuth-game/assets/ui/README.md` documenting that the feature's artwork must be original/project-created and transparent-background.
- [X] T002 Inspect and preserve existing session, shop, weapon-selection, setup, accounting, result, and audio boundaries in `crates/azimuth-game/src/main.rs`, `crates/azimuth-game/src/session.rs`, and `crates/azimuth-game/src/weapon.rs`; do not move game-state authority into presentation code.

---

## Phase 2: Foundational Presentation Infrastructure

**Purpose**: Create the shared visual/identity seam needed by all screens.

**⚠️ CRITICAL**: Complete this phase before HUD, shop, and flow restyling.

- [X] T003 Define focused shared Azimuth UI theme constants and small reusable panel/button/card construction helpers in `crates/azimuth-game/src/main.rs` for surfaces, semantic colours, type hierarchy, borders, spacing, and control states without creating a general UI framework.
- [X] T004 Replace Unicode glyph-only `WeaponPresentation` metadata with stable compact-name, shared icon-key/path, and concise optional description metadata in `crates/azimuth-game/src/weapon.rs`, keeping it engine-neutral and keyed solely by `WeaponId`.
- [X] T005 Create one coherent original transparent icon asset for each actual current weapon—Basic Shell, HE, Heavy Shell, MIRV, Cluster Bomb, Roller, Bunker Buster, Dirt Bomb, Curve Ball, Bouncer, and Nuke—in `crates/azimuth-game/assets/ui/weapons/`; do not add Bomb Net or text/emoji artwork.
- [X] T006 Add asset loading and a shared `WeaponId` → icon-handle resolver in `crates/azimuth-game/src/main.rs` that loads each icon once and reports missing required identity/assets clearly during development.
- [X] T007 Add focused metadata/asset-coverage regression tests in `crates/azimuth-game/src/weapon.rs` proving every `ACTIVE_WEAPONS` identity has valid shared presentation metadata, every `SHOP_WEAPONS` identity resolves it, Basic Shell is not purchasable, and removed weapons cannot enter the active catalogue.
- [X] T008 Add shared Bevy button interaction styling/synchronisation in `crates/azimuth-game/src/main.rs` for normal, hovered, pressed, selected, and disabled states, ensuring disabled controls do not invoke an existing domain action.

**Checkpoint**: A shared theme, source-owned icon set, and safe shared identity/control-state mapping exist before any screen is restyled.

---

## Phase 3: User Story 1 - Read the Battlefield at a Glance (Priority: P1) 🎯 MVP

**Goal**: Deliver a compact graphical battlefield HUD whose weapon inventory, player/action information, health, aiming, wind, and player status can be scanned without obscuring the battle.

**Independent Test**: Run 2-, 4-, and 8-player rounds at 1280×720, 1920×1080, and a wider desktop layout; identify active player, health, aim, wind, selected weapon, and ammunition, then choose an available weapon by mouse.

### Tests for User Story 1

- [X] T009 [US1] Add presentation-state tests in `crates/azimuth-game/src/main.rs` for limited, exhausted, selected, and unlimited Basic Shell weapon-slot mapping, including the shared icon identity used by the strip.
- [X] T010 [US1] Add focused player-status mapping tests in `crates/azimuth-game/src/main.rs` covering active and eliminated presentation for 2–8 configured players without changing tank/session rules.

### Implementation for User Story 1

- [X] T011 [US1] Rebuild battlefield weapon slots in `crates/azimuth-game/src/main.rs` to render the shared weapon image, compact text label, and authoritative ammunition/Unlimited marker with comfortable mouse hit targets.
- [X] T012 [US1] Update weapon-slot synchronisation and selection input in `crates/azimuth-game/src/main.rs` so selected, exhausted, and unavailable states use the shared interaction styling while preserving existing mouse selection and battle-control gating.
- [X] T013 [US1] Restyle the tactical HUD composition in `crates/azimuth-game/src/main.rs` with themed panels and hierarchy for active player/action, graphical health, movement, 2–8 player list, exact aiming instrumentation/numbers, and existing wind viewport/readout.
- [X] T014 [US1] Tune HUD/weapon-bar flex sizing and overflow behaviour in `crates/azimuth-game/src/main.rs` for the current eleven-weapon arsenal at the representative desktop resolutions, adding only the smallest needed scaling behaviour.
- [ ] T015 [US1] Manually validate the User Story 1 scenarios from `docs/specs/20260912-231438-ui-polish/quickstart.md` and correct HUD overlap, clipping, unreadable contrast, or mouse-hit-target regressions in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: The battle bar is a recognisable visual inventory and the battlefield remains the focus.

---

## Phase 4: User Story 2 - Browse a Beautiful Artillery Shop (Priority: P1)

**Goal**: Turn the functional between-round shop into a browsable graphical weapon catalogue while retaining existing atomic purchase/session rules.

**Independent Test**: Complete a round, open shop with cash sufficient for some but not all items, purchase a weapon, and confirm shared artwork, price, ownership, affordability, wallet, count, feedback, and Done behaviour are immediately clear.

### Tests for User Story 2

- [X] T016 [US2] Add shop presentation mapping tests in `crates/azimuth-game/src/main.rs` for shared `WeaponId` icon resolution, authoritative owned-ammo display, affordable/unaffordable states, and a visible but disabled Nuke purchase control.

### Implementation for User Story 2

- [X] T017 [US2] Rebuild `spawn_shop_overlay` and shop card components in `crates/azimuth-game/src/main.rs` as a responsive themed product grid with large shared icon artwork, name, optional concise weapon personality text, price, owned ammunition, Buy, shopper identity/colour, cash, round context, and Done hierarchy.
- [X] T018 [US2] Update `sync_shop_overlay` and `handle_shop_input` in `crates/azimuth-game/src/main.rs` so card/button appearance derives only from authoritative wallet/loadout/session state, unaffordable products remain readable but non-actionable, and successful purchases give bounded visual feedback.
- [X] T019 [US2] Add one restrained source-owned or clearly licensed purchase/primary-action UI cue under `crates/azimuth-game/assets/audio/` and load/play it through the existing `AudioAssets` path in `crates/azimuth-game/src/main.rs` only after successful user actions.
- [X] T020 [US2] Tune the shop catalogue layout in `crates/azimuth-game/src/main.rs` at 1280×720, 1920×1080, and a wider desktop layout so all current products remain browsable without clipping, tiny text, or a spreadsheet-like presentation.
- [ ] T021 [US2] Manually validate shop entry, purchase, unaffordability, Human Done, AI progression, and next-round inventory persistence using `docs/specs/20260912-231438-ui-polish/quickstart.md`; correct presentation-only defects in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: The shop feels like browsing artillery products while all existing purchase and transition rules remain authoritative and intact.

---

## Phase 5: User Story 3 - Experience One Coherent Match Flow (Priority: P2)

**Goal**: Apply the shared visual language across setup, winner, accounting, and continuation presentation so the complete game flow feels like one product.

**Independent Test**: Run Setup → Round → Winner → Accounting → Shop → Round 2 and verify shared panels, typography, spacing, player accents, numerical hierarchy, button feedback, and mouse-native continuation throughout.

### Tests for User Story 3

- [X] T022 [US3] Add focused formatting/view-state tests in `crates/azimuth-game/src/main.rs` for accounting damage/placement/total/wallet hierarchy and winner/continuation text derived from existing result and accounting state.

### Implementation for User Story 3

- [X] T023 [US3] Restyle match setup and its player rows/identity, Human/AI indication, keyboard guidance, and start hierarchy in `crates/azimuth-game/src/main.rs` using the shared theme without changing setup rules.
- [X] T024 [US3] Replace the raw session-flow overlay treatment in `crates/azimuth-game/src/main.rs` with themed winner celebration and accounting presentation that preserves the battlefield/winning tank, exposes round/player identity, and makes damage, placement, total, wallet, and next action easy to scan.
- [X] T025 [US3] Apply the shared button/panel/typography/interaction treatment to all existing setup, result, accounting, shop Done, and battlefield UI controls in `crates/azimuth-game/src/main.rs`, retaining useful keyboard shortcuts and clear mouse feedback.
- [ ] T026 [US3] Manually validate the complete flow at representative resolutions using `docs/specs/20260912-231438-ui-polish/quickstart.md` and correct inconsistent colours, font scales, margins, dead space, or raw/debug-like surfaces in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: Setup, battle, result, accounting, shop, and next-round continuation are recognisably the same game.

---

## Phase 6: Polish & Cross-Cutting Completion

**Purpose**: Verify quality, performance restraint, documentation, and the constitutional boundary across all stories.

- [X] T027 Review `crates/azimuth-game/src/main.rs` for unnecessary per-frame UI-tree reconstruction, repeated asset loads, or duplicated presentation identities; retain state-driven updates and simplify only where it improves coherence.
- [X] T028 Update only implementation-verified UI, mouse, scalability, clutter, battlefield-focus, sound, visual-direction, and polished-flow items in `docs/roadmap.md`; record discovered out-of-scope work rather than implementing it.
- [X] T029 Run `cargo fmt --all -- --check`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo check --workspace`, and `cargo build --workspace` from the repository root; fix regressions in the affected files.
- [ ] T030 Complete and record the full manual acceptance matrix from `docs/specs/20260912-231438-ui-polish/quickstart.md` for 2-, 4-, and 8-player sessions, mixed Human/AI shopping, all weapon states, 1280×720, 1920×1080, and a wider desktop layout.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1**: Starts immediately.
- **Phase 2**: Depends on T001–T002 and blocks restyled UI work.
- **US1 (Phase 3)**: Depends on Phase 2. It is the suggested MVP.
- **US2 (Phase 4)**: Depends on Phase 2 and can be implemented in parallel with US1 after the shared `main.rs` foundation is stabilised; integrate its shared interaction/audio changes carefully because both stories edit `main.rs`.
- **US3 (Phase 5)**: Depends on Phase 2 and benefits from the theme already exercised by US1/US2; execute after those P1 increments for the lowest merge/conflict risk.
- **Phase 6**: Depends on all desired stories.

### User Story Completion Order

```text
Setup → Shared theme / icon identity
                  ├── US1: Battlefield HUD MVP
                  └── US2: Shop catalogue
                           ↓
                  US3: Coherent setup/result/accounting flow
                           ↓
                  Cross-cutting validation and roadmap update
```

### Parallel Opportunities

- T005's individual source-owned icon files can be produced in parallel once the common art direction and filenames are agreed.
- T007 metadata tests and T008 interaction-style implementation can proceed in parallel after T004, because they primarily exercise different concerns in `weapon.rs` and `main.rs`.
- After Phase 2, US1 and US2 are independently testable, although their edits to `main.rs` should be sequenced or carefully integrated.

## Parallel Example: Foundational Artwork and Coverage

```text
Task: "Create Basic Shell through Roller icon assets in crates/azimuth-game/assets/ui/weapons/"
Task: "Create Bunker Buster through Nuke icon assets in crates/azimuth-game/assets/ui/weapons/"
Task: "Add metadata/asset-coverage regression tests in crates/azimuth-game/src/weapon.rs"
Task: "Add shared Bevy button interaction styling in crates/azimuth-game/src/main.rs"
```

## Implementation Strategy

### MVP First

1. Complete setup and shared theme/icon work (T001–T008).
2. Deliver the graphical battlefield HUD (T009–T015).
3. Validate US1 independently before touching broader flow presentation.

### Incremental Delivery

1. Add the beautiful shop as the second P1 increment (T016–T021), retaining existing purchases/AI/round flow.
2. Apply the established language to setup, winner, and accounting (T022–T026).
3. Finish with quality gates, resolution/manual acceptance, and honest roadmap updates (T027–T030).

### Format Validation

All 30 implementation tasks use the required checkbox, sequential ID, optional parallel marker, user-story label where applicable, action, and exact repository path format.
