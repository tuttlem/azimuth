# Implementation Plan: Arsenal Pack #2

**Branch**: `20260910-224611-arsenal-pack-two` | **Date**: 2026-09-10 | **Spec**: [spec.md](spec.md)

## Summary

Add twelve-weapon click-first selection and four deliberately distinct deterministic behaviours:
terrain addition, launch-relative lateral curve, bounded surface-normal ricochet, and a large
ordinary-impact consequence. Reuse fixed-step flight, current terrain, shared damage/settling, and
shot resolution; add only the terrain/shot vocabulary directly demonstrated by these weapons.

## Technical Context

**Language/Version**: Rust stable

**Primary Dependencies**: Existing Bevy 0.18 desktop application; no new dependency

**Storage**: In-memory deterministic match state

**Testing**: `cargo test -p azimuth-game`, formatting, Clippy

**Target Platform**: Linux desktop now; existing desktop game portability remains unchanged

**Project Type**: Desktop game

**Performance Goals**: Preserve the current fixed-step match responsiveness; each deformation scans the existing bounded terrain grid

**Constraints**: No homing, generic scripting, physics engine, underground simulation, or AI strategy; presentation cannot own simulation

**Scale/Scope**: Twelve weapon boxes in one desktop bottom strip; four new bounded weapon behaviours

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**PASS before research and after design.** The plan preserves deterministic fixed-step authority,
adds no dependencies, retains presentation as an observer, and uses simple composable mechanics.
`Mound`, launch-relative curve acceleration, surface normal, and bounded bounce state each answer a
concrete new weapon need; no generic weapon graph is introduced.

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260910-224611-arsenal-pack-two/
├── plan.md              # This file ($speckit-plan command output)
├── research.md          # Phase 0 output ($speckit-plan command)
├── data-model.md        # Phase 1 output ($speckit-plan command)
├── quickstart.md        # Phase 1 output ($speckit-plan command)
├── contracts/           # Phase 1 output ($speckit-plan command)
└── tasks.md             # Phase 2 output ($speckit-tasks command - NOT created by $speckit-plan)
```

### Source Code (repository root)
```text
crates/azimuth-game/src/
├── weapon.rs       # catalogue, loadout, committed-shot configuration
├── projectile.rs   # fixed-step flight plus narrow optional acceleration
├── battlefield.rs  # current terrain, mound/crater, exact surface normal
├── combat.rs       # shared radial damage
├── tank.rs         # support and settling
├── ai.rs           # unchanged Basic Shell AI safety
└── main.rs         # input/UI, authoritative shot resolution, presentation

docs/
├── projectile-model.md
├── roadmap.md
└── specs/20260910-224611-arsenal-pack-two/
```

**Structure Decision**: Retain the existing compact game crate. The feature crosses the current
weapon/projectile/terrain/shot boundary but does not justify a new crate or framework.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| None | — | — |
