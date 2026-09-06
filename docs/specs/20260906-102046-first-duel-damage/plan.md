# Implementation Plan: First Duel — Damage, Elimination and Victory

**Branch**: `20260906-102046-first-duel-damage` | **Date**: 2026-09-06 | **Spec**: [spec.md](spec.md)

## Summary

Turn the existing two-tank artillery loop into a finishable local duel. Add 100 health directly to
each domain tank and a small pure combat module for a 6-unit, 40-maximum, 3D linear-falloff blast.
At terrain impact, calculate damage for both pre-mutation tank positions, apply it, deform terrain,
derive survivors/match result, then either hand off to the next survivor or finish as winner/draw.

Extend the existing turn state only with an explicit match outcome and survivor-aware advancement.
Hide eliminated placeholder tanks and extend the HUD with both health values and the final result.
Camera and boom systems continue to observe state and must never control damage, turn handoff, or
match completion.

## Technical Context

**Language/Version**: Rust edition 2024 on the stable toolchain

**Primary Dependencies**: Bevy 0.18.1 for the desktop application, input, rendering, and HUD

**Storage**: In-memory domain values and Bevy resources; no persistence or external service

**Testing**: Rust unit tests via `cargo test --workspace`; manual desktop validation via
`cargo run --package azimuth-game`

**Target Platform**: Desktop Linux development environment; portable Bevy desktop application

**Project Type**: Single Rust-workspace desktop game application

**Performance Goals**: Preserve the existing 120 Hz fixed projectile simulation; resolve a
two-tank blast and match outcome in the same authoritative impact step

**Constraints**: Damage/match state must be deterministic and renderer-independent. No direct tank
collision, terrain occlusion, physics impulse, generic combat/match framework, new dependencies,
or presentation-driven timing.

**Scale/Scope**: Two local human players, one shared health/damage profile, one single-elimination
duel per application run

## Constitution Check

| Principle | Design response | Status |
|---|---|---|
| I. Fun Over Realism | Fixed health and visible distance falloff favor readable artillery consequences. | Pass |
| II/III. Simple, Coherent Systems | One damage formula, tank health field, and compact match state; no combat hierarchy. | Pass |
| V. Deterministic/Testable Simulation | Pure combat/turn rules use world positions and integer health, with focused unit tests. | Pass |
| VI. Presentation Must Not Own the Game | Impact resolution updates combat before visual systems; camera/boom only read state. | Pass |
| VII/VIII. Playable Progress and Scope | Bounded vertical slice reaches the first duel without weapons, AI, restart, or polish systems. | Pass |
| IX/X. Roadmap and Specs | Timestamped artifacts and demonstrated roadmap updates only. | Pass |
| XI/XII. Quality and Dependencies | Existing Cargo quality gates; no dependency addition. | Pass |

**Post-design re-check**: Pass. No presentation type participates in health, damage, survivor, or
match-result mutation.

## Project Structure

```text
crates/azimuth-game/src/
├── combat.rs       # Pure blast falloff and same-event damage resolution
├── tank.rs         # Tank identity, pose, health, and elimination predicate
├── turn.rs         # Existing action phases plus match outcome/survivor advancement
├── main.rs         # Bevy resource wiring, impact integration, HUD, and hidden eliminated visual
├── projectile.rs   # Existing fixed-step flight; no direct tank collision added
├── battlefield.rs  # Existing authoritative terrain and deformation
└── world.rs        # World-position operations shared by combat

README.md
docs/projectile-model.md
docs/world-conventions.md
docs/roadmap.md
docs/specs/20260906-102046-first-duel-damage/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/first-duel.md
```

**Structure Decision**: Add one `combat.rs` domain module because falloff and same-event damage
need renderer-independent tests and should not live in Bevy integration. Retain the existing game
crate; no new crate or abstraction is justified.

## Implementation Sequence

1. Add tank health/elimination and pure combat configuration, distance/falloff, and same-event
   damage resolution with tests before Bevy integration.
2. Evolve `TurnState` with the small in-progress/winner/draw state and survivor-aware action gates
   and handoff; update turn tests without adding a scheduler.
3. Integrate blast resolution into the fixed terrain-impact path: calculate damage, mutate tanks,
   apply crater, finalize match/turn, and preserve out-of-bounds shot behavior.
4. Make eliminated tank roots hidden, expand the HUD, and ensure camera observes finished state
   without affecting gameplay.
5. Update gameplay documentation and only validated roadmap boxes; run quality and manual duel
   acceptance.

## Complexity Tracking

No constitution violations or complexity exceptions require justification.
