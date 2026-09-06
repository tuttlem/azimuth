# Implementation Plan: First Arsenal — Extensible Weapon Framework, Selection and High Explosive

**Branch**: `20260906-194157-first-arsenal` | **Date**: 2026-09-06 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `docs/specs/20260906-194157-first-arsenal/spec.md`

## Summary

Establish ordinary weapons as central definitions, retain one compact independent loadout per
player, and snapshot the selected conventional weapon into a fired shot. Basic Shell preserves the
current projectile/impact result. High Explosive uses the same deterministic flight but a larger
captured blast and crater profile with two rounds per player. The current common impact flow is
parameterised rather than duplicated; HUD observes selected weapon and availability read-only.

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: Rust edition declared by the existing Cargo workspace

**Primary Dependencies**: Bevy 0.18; existing workspace only; no new dependencies

**Storage**: In-memory match resources only; no persistence

**Testing**: `cargo test --workspace`, pure domain tests colocated with current modules

**Target Platform**: Existing desktop local-game target

**Project Type**: Desktop game application in one Cargo workspace crate

**Performance Goals**: Preserve current responsive local play and deterministic 120 Hz fixed
projectile/settling simulation; two players and two normal weapons add no material per-step cost

**Constraints**: Weapon definitions and shot consequences are authoritative and deterministic;
HUD/camera remain observers; only existing conventional ballistic behaviour is in scope

**Scale/Scope**: Two local players; Basic Shell unlimited, High Explosive two rounds each; one
test-only third ordinary definition proves the extension seam

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle / gate | Plan response | Status |
|---|---|---|
| Fun and emergent simple systems | HE changes existing blast, crater, support, movement, and line-of-fire consequences without new physics. | Pass |
| Coherence over cleverness | One small `weapon` domain module supplies data; no generic effects/items/trait hierarchy. | Pass |
| Determinism and testing | Loadout mutations and captured shot profiles are domain state; tests remain renderer-independent. | Pass |
| Presentation does not own gameplay | HUD only derives selected weapon and availability; impact never reads HUD/camera. | Pass |
| Playable vertical slice | Two real selectable weapons are delivered with a direct HUD/control loop. | Pass |
| Scope and roadmap discipline | No exotic weapons, shops, drag, or mass; deferred ideas remain roadmap work. | Pass |
| Dependencies and quality | No dependency changes; workspace checks and documentation updates are required. | Pass |

**Post-design re-check**: Pass. The design adds only the necessary conventional-weapon boundary.

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260906-194157-first-arsenal/
├── plan.md              # This file ($speckit-plan command output)
├── research.md          # Phase 0 output ($speckit-plan command)
├── data-model.md        # Phase 1 output ($speckit-plan command)
├── quickstart.md        # Phase 1 output ($speckit-plan command)
├── contracts/           # Phase 1 output ($speckit-plan command)
└── tasks.md             # Phase 2 output ($speckit-tasks command - NOT created by $speckit-plan)
```

### Source Code (repository root)
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
crates/azimuth-game/src/
├── main.rs          # Bevy resource wiring, input, retained HUD, shot/impact integration
├── weapon.rs        # Catalogue, conventional profiles, player loadouts, fired-shot snapshot
├── projectile.rs    # Existing deterministic ballistic motion
├── combat.rs        # Parameterised shared radial-damage consequence
├── battlefield.rs   # Existing crater application and terrain queries
├── tank.rs          # Existing health, support, and settling
└── turn.rs          # Existing turn/action authority
```

**Structure Decision**: Add only `weapon.rs`. It is a demonstrated domain boundary: definitions,
loadouts, and captured conventional-shot data belong together, while projectile kinematics, combat
falloff, terrain, tank physics, turn authority, and Bevy presentation retain their existing homes.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No constitution violations require justification.
