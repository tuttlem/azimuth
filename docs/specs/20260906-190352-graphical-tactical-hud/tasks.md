---

description: "Dependency-ordered implementation tasks for Graphical Tactical HUD"
---

# Tasks: Graphical Tactical HUD

**Input**: Design documents from `/docs/specs/20260906-190352-graphical-tactical-hud/`

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md),
[data-model.md](data-model.md), [HUD contract](contracts/tactical-hud.md), and
[quickstart.md](quickstart.md)

**Tests**: Required. Test the pure read-only state-to-view mapping before native UI rendering;
avoid pixel, screenshot, font-metric, and camera-timing tests.

**Organization**: The existing single HUD and its formatter are replaced through one shared
foundational presentation model, then each user story adds an independently testable visual group.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Different-file work with no incomplete-task dependency.
- **[US#]**: User-story traceability label; omitted for shared foundation and final polish.

## Phase 1: Setup

**Purpose**: No project initialization or dependency task is required. The existing Bevy UI,
workspace quality commands, player colours, and temporary HUD seam are ready.

---

## Phase 2: Foundational (Blocking Prerequisite)

**Purpose**: Define the sole read-only presentation transformation before rendering panels.

- [X] T001 Add failing pure HUD-view tests for phase/match precedence, both player health ratios,
  elimination, aim values, movement visibility/rejection, contextual hints, calm wind, and opposite
  X/Z wind vectors in `crates/azimuth-game/src/main.rs`.
- [X] T002 Implement the small read-only tactical HUD view, player condition, action/hint, and
  normalized wind-plot mapping in `crates/azimuth-game/src/main.rs`; keep it independent of UI
  entities and do not mutate gameplay state.
- [X] T003 Add failing native-UI synchronization tests or focused helpers for health-fill sizing,
  movement/control visibility, and result-state suppression in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: One testable presentation model maps all existing gameplay states without a second
authoritative state or renderer-specific gameplay dependency.

---

## Phase 3: User Story 1 - Read the Current Turn at a Glance (Priority: P1) 🎯 MVP

**Goal**: Replace the debug-like text dump with a compact graphical player/match frame, two health
bars, active identity, action state, and clear winner/draw result.

**Independent Test**: Start/advance a local match, damage/eliminate a tank, and finish winner/draw;
verify player/match presentation derived from the view model and inspect the graphical frame.

### Implementation for User Story 1

- [X] T004 [US1] Replace the legacy `AimingHud` text entity with a non-interactive, anchored
  retained tactical-frame root and stable top-left player/match plus health-panel entities in
  `crates/azimuth-game/src/main.rs`.
- [X] T005 [US1] Add active-player colour/border treatment, two exact health labels, proportional
  health fills, eliminated `OUT` presentation, and match-result panel binding in
  `crates/azimuth-game/src/main.rs`.
- [X] T006 [US1] Replace `sync_aiming_hud` with a read-only tactical-frame synchronizer in
  `crates/azimuth-game/src/main.rs`; it must update only renderer-facing UI fields and leave
  turn/tank/projectile/camera state untouched.
- [X] T007 [US1] Add regression coverage for active-player, health/elimination, winner/draw, and
  read-only presentation invariants in `crates/azimuth-game/src/main.rs`.

**Checkpoint**: A player can independently read the current turn, both conditions, and match result
from a graphical frame; no legacy text dump remains.

---

## Phase 4: User Story 2 - Aim and Compensate Deliberately (Priority: P1)

**Goal**: Add exact aiming values and an ASCII-safe graphical world-axis wind plot without
obscuring the battlefield.

**Independent Test**: Change each active aim value and feed cardinal, diagonal, opposite, and calm
wind values into the view model; inspect exact values and plot direction/neutrality.

### Implementation for User Story 2

- [X] T008 [US2] Add failing mapping tests for azimuth/elevation/power precision and graphical
  world-axis wind plot position, opposition, and calm neutrality in
  `crates/azimuth-game/src/main.rs`.
- [X] T009 [US2] Add the retained top-right shot/environment panel with precise ASCII-safe aim and
  wind-strength fields in `crates/azimuth-game/src/main.rs`.
- [X] T010 [US2] Add the geometric wind plot using labelled `+X`/`+Z` axes and a normalized marker
  derived from the authoritative wind vector in `crates/azimuth-game/src/main.rs`; do not use
  unsupported Unicode direction glyphs or camera-relative direction.
- [X] T011 [US2] Bind the shot/environment panel to the existing read-only HUD view in
  `crates/azimuth-game/src/main.rs` and verify it cannot change aiming or wind.

**Checkpoint**: The active player can read precise aim and a graphical, camera-independent wind
direction/strength from the frame.

---

## Phase 5: User Story 3 - Move Without Losing Tactical Context (Priority: P2)

**Goal**: Show movement allowance and concise controls only in relevant phases.

**Independent Test**: Select movement, spend/reject/forfeit steps, return to choosing, and resolve a
shot; inspect that movement/control sections and warnings map exactly to the authoritative phase.

### Implementation for User Story 3

- [X] T012 [US3] Add failing view/synchronization tests for choosing, moving with remaining steps,
  movement bounds/slope feedback, resolving, and finished contextual-hint visibility in
  `crates/azimuth-game/src/main.rs`.
- [X] T013 [US3] Add a small lower-corner movement/contextual-control panel and bind movement
  allowance plus compact rejection text in `crates/azimuth-game/src/main.rs`.
- [X] T014 [US3] Implement phase-specific concise ASCII-safe hints from the existing actual input
  scheme in `crates/azimuth-game/src/main.rs`, hiding action controls while resolving or finished.

**Checkpoint**: Movement is supported by clear allowance/context, while firing and resolving states
are not covered by a persistent keyboard manual.

---

## Phase 6: User Story 4 - Keep the Battlefield Primary (Priority: P2)

**Goal**: Ensure the retained frame has sensible anchored layout and remains independent from
camera/gameplay timing.

**Independent Test**: Resize a running normal desktop window and alternate move/fire turns while
camera transitions and projectile resolution continue; visual groups remain usable and central play
space remains clear.

### Implementation for User Story 4

- [X] T015 [US4] Add focused layout/helper tests for bounded health-fill and wind-marker placement
  plus visibility changes that do not require exact pixels in `crates/azimuth-game/src/main.rs`.
- [X] T016 [US4] Tune anchored root/panel layout, contrast, spacing, and visibility in
  `crates/azimuth-game/src/main.rs` to keep corners readable across ordinary window sizes while
  leaving the central battlefield free.
- [X] T017 [US4] Verify synchronization remains an Update-time, read-only observer separate from
  fixed projectile/settling and camera systems; document the timing boundary in
  `crates/azimuth-game/src/main.rs`.

**Checkpoint**: The graphical frame is a stable tactical aid, not a dashboard or gameplay gate.

---

## Phase 7: Polish and Cross-Cutting Validation

- [X] T018 [P] Update graphical HUD, wind-axis convention, and concise displayed-control
  documentation in `README.md` and `docs/world-conventions.md`.
- [X] T019 Update only earned Graphical Tactical HUD, graphical wind-indicator, and usability
  checkboxes in `docs/roadmap.md`.
- [ ] T020 Run `cargo check --workspace --all-targets`, `cargo test --workspace`,
  `cargo fmt --all -- --check`, and `cargo clippy --workspace --all-targets --all-features -- -D
  warnings`; perform the manual graphical-frame, wind, match-result, and resize validation in
  `docs/specs/20260906-190352-graphical-tactical-hud/quickstart.md`.

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1**: No setup work is required.
- **Phase 2**: T001–T003 define the shared read-only mapping and block rendering work.
- **US1 (Phase 3)**: Depends on Phase 2 and establishes the graphical frame MVP.
- **US2 (Phase 4)**: Depends on Phase 2 and integrates into the US1 frame.
- **US3 (Phase 5)**: Depends on Phase 2 and integrates into the same retained frame after US1.
- **US4 (Phase 6)**: Depends on the complete graphical frame from US1–US3.
- **Polish (Phase 7)**: Follows all feature work.

### User Story Dependencies

- **US1 (P1)**: Delivers the independently valuable player/match and health frame.
- **US2 (P1)**: Requires shared view mapping and frame structure; adds aim/wind skill feedback.
- **US3 (P2)**: Requires shared view mapping and frame structure; adds phase-specific movement/help.
- **US4 (P2)**: Validates layout and presentation boundaries after all visual groups exist.

### Parallel Opportunities

- T018 can proceed independently once displayed strings/conventions are final.
- Test authoring is sequential within `main.rs`, which is intentionally the sole existing HUD seam.
- Documentation files in T018 may be edited in parallel after the implementation behaviour is fixed.

## Parallel Example: Documentation

```text
Task: "Update HUD/control documentation in README.md"
Task: "Update world-axis wind convention in docs/world-conventions.md"
```

## Implementation Strategy

### MVP First (US1)

1. Complete T001–T003 to prove the read-only presentation model.
2. Complete T004–T007 to replace the temporary text block with player/match and health panels.
3. Validate turn changes, damage/elimination, winner/draw, and the absence of duplicate legacy UI.

### Incremental Delivery

1. US1 establishes the coherent graphical frame.
2. US2 adds precision aiming and graphical world-axis wind feedback.
3. US3 replaces the persistent control dump with movement/contextual information.
4. US4 verifies the frame remains a battlefield-first, presentation-only aid.
5. Polish documents and validates only earned roadmap progress.

## Notes

- Every task uses an exact path and the required checklist format.
- Do not add UI-owned game state, external assets, glyph arrows, menus, animation, or dependencies.
- Mark each completed task `[X]`; leave T020 open until manual validation is actually reported.
