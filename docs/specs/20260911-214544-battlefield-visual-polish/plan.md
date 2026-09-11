# Implementation Plan: Battlefield Visual Polish — Tank Variety, Materials, Explosion Particles and Smoke

**Branch**: `20260911-214544-battlefield-visual-polish` | **Date**: 2026-09-11 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/docs/specs/20260911-214544-battlefield-visual-polish/spec.md`

## Summary

Deliver five primitive tank silhouettes with player-coloured painted-metal materials, richer
height-derived terrain and water surfaces, and bounded profile-scaled impact debris and smoke.
Keep `Tank`, terrain, weapons, simulation seeds, collision, and turn logic authoritative and
unchanged. Use existing primitive mesh/material paths, a separately derived cosmetic assignment
stream, and renderer-owned requests emitted after terrain impacts resolve.

## Technical Context

**Language/Version**: Rust edition 2024; project toolchain pinned in `rust-toolchain.toml`

**Primary Dependencies**: Bevy 0.18.1 standard 3D mesh, material, ECS, time, and asset facilities; no new dependency

**Storage**: In-memory match/presentation resources and existing small audio assets; no persistent storage or external visual assets

**Testing**: `cargo test --workspace`; `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets --all-features -- -D warnings`; manual graphical matches

**Target Platform**: Existing desktop graphical targets supported by the current Bevy configuration

**Project Type**: Single desktop game application in one Cargo workspace crate

**Performance Goals**: Remain responsive in manual 2-, 4-, and 8-player matches. Cap active debris/hot particles at 128, active smoke puffs at 32, an impact at 32 particles/eight puffs, and pending requests at 64.

**Constraints**: Presentation must not mutate gameplay or authoritative RNG; five models selected from the required 4–6; no external packs, custom shader, general VFX system, visibility mechanic, water physics, or rendering rewrite; finite effects reuse shared mesh/material handles.

**Scale/Scope**: One game crate, 2–8 players, five tank recipes, the existing 12-weapon arsenal, and effects for all resolved terrain impacts. Existing explosion visual scale controls cosmetic magnitude.

## Constitution Check

### Pre-Research Gate

| Principle | Plan response | Status |
|---|---|---|
| I. Fun Over Realism | Use readable exaggerated primitive silhouettes and bursts; do not simulate tracks, fluids, or debris. | Pass |
| III. Code Coherence | Extend existing presentation seams with small explicit types; create no generic asset, particle, or rendering framework. | Pass |
| V. Deterministic and Testable Simulation | Preserve domain/fixed-step state; isolate cosmetic derivation and test enabled/disabled equivalence. | Pass |
| VI. Presentation Must Not Own the Game | Observe canonical tank firing and resolved impacts one-way; presentation never feeds simulation. | Pass |
| VII/VIII. Playable Progress and Scope | Implement one cohesive visual slice from current primitives/materials; record nonessential discoveries later. | Pass |
| IX/X. Roadmap and Specs | Work remains on the timestamped feature branch and checks only demonstrated roadmap outcomes. | Pass |
| XI/XII. Quality and Dependencies | Add lifecycle/boundary coverage, run normal quality gates, and add no dependency. | Pass |

### Post-Design Re-check

Phase 0 and 1 retain every gate: recipes sit outside `Tank`, terrain treatment derives from the
current mutable surface, effects are finite capped renderer entities, and no crate or dependency is
added. No constitution exception or complexity justification is required.

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260911-214544-battlefield-visual-polish/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── presentation-boundary.md
└── tasks.md                         # Created later by $speckit-tasks
```

### Source Code (repository root)

```text
crates/azimuth-game/
├── Cargo.toml                        # Existing Bevy 0.18.1 dependency
├── assets/                           # No external visual asset required
└── src/
    ├── main.rs                       # Scene resources, visual recipes, materials, effects, tests
    ├── battlefield.rs                # Authoritative terrain and height-derived colours
    ├── tank.rs                       # Unchanged authoritative Tank/firing representation
    ├── weapon.rs                     # Existing impact-profile magnitude source
    ├── projectile.rs                 # Existing read-only wind data
    └── {combat,aiming,turn,world,ai,match_setup}.rs
```

**Structure Decision**: Retain the established single-crate design. Make narrowly scoped changes in
the existing presentation composition root and preserve `battlefield.rs`, `tank.rs`, and `weapon.rs`
as authority. A new module is not warranted for five recipes and one bounded effect path.

## Implementation Approach

1. Add a five-value presentation-only tank-model enum, match-local assignments, and a pure
   assignment helper. Use a `tank-visuals` derivation of the match seed, unique-first assignment,
   then repeat; do not change `Tank` or consume terrain/start/wind/AI streams.
2. Replace the single tank mesh bundle with reusable primitive parts and five recipe layouts. Keep
   existing visual root/turret/barrel/muzzle roles, add armour/track/mechanical material roles, and
   align visual barrels to the canonical `TankFiringRepresentation` muzzle marker.
3. Upgrade ordinary materials: moderate-metallic player armour, darker shared mechanical parts,
   opaque lower-roughness water, and modest directional-light tuning only when visual review needs
   it. Prefer scalar materials; add a tiny generated neutral tile and terrain UVs only if needed.
4. Preserve terrain mutation/query paths. Enrich only render colour/UV presentation derived from
   current height and world position, letting the existing mesh refresh keep craters and Dirt Bomb
   mounds coherent. Horizon stays non-authoritative.
5. Add a capped renderer-owned queue of position/scale/serial requests at both terrain-impact
   resolution paths (normal and split-child). Retain `LatestTerrainImpact` for current marker,
   flash, audio, and camera; the queue preserves all same-tick barrage effects.
6. Drain requests in Update. Reuse primitive meshes/fixed materials for hot/dirt particles and
   translucent smoke puffs; use pure cosmetic hashing, profile scale, optional read-only wind
   drift, finite lifetime, fade/scale, and the declared cap policy.
7. Test assignments, muzzle transforms, terrain refresh, profile monotonicity, split recording,
   caps, lifecycle/reset cleanup, and authoritative enabled/disabled equivalence. Run quality and
   graphical checks, then update only proven roadmap work.

## Complexity Tracking

No constitution violations require justification.
