# Implementation Plan: First AI Controller — A Computer Player That Can Complete a Turn

**Branch**: `20260909-192622-ai-controller` | **Date**: 2026-09-09 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `docs/specs/20260909-192622-ai-controller/spec.md`

## Summary

Make configured AI slots start-valid and give an active AI one deterministic, imperfect,
firing-first decision per choosing turn. A small pure decision module selects the nearest living
non-self target, derives rough target-directed aim with bounded seeded error, ignores wind, and
uses the normal available Basic Shell. A narrow shared current-player launch operation receives
both Human and AI requests; it retains existing inventory commitment, turn transition, projectile
creation, and all later authoritative resolution. Human tactical systems are gated by the active
configured controller, while an AI dispatcher runs only for a valid active AI turn.

## Technical Context

**Language/Version**: Rust edition 2024

**Primary Dependencies**: Bevy 0.18.1 for the desktop scene, input, resources, and rendering; no new dependency

**Storage**: In-memory match resources only; no persistence or network state

**Testing**: `cargo test -p azimuth-game`; focused pure decision, turn, configuration, and shared-action tests; graphical manual acceptance

**Target Platform**: Linux desktop local game, with existing cross-platform desktop rendering assumptions

**Project Type**: Single-crate desktop game in a Cargo workspace

**Performance Goals**: Preserve responsive normal play; inspect only the current 2–8 player collection and create at most one decision/action per valid AI choosing turn

**Constraints**: Deterministic authoritative AI choices from a dedicated match-derived seed; one authoritative player/tank/projectile/terrain model; no exact ballistic solver, terrain search, AI navigation, new external dependency, or wall-clock-dependent automated behavior test

**Scale/Scope**: 2–8 configured participants, any Human/AI mix including all-AI; baseline firing only, Basic Shell selection, nearest-living target, rough geometry-based aim, bounded error, ignored wind

## Constitution Check

**Pre-design gate: PASS**

- Fun and emergent simple systems: an imperfect geometry-guided shot remains exposed to current wind, terrain, projectile, crater, settling, and combat systems; it is deliberately not optimal.
- Coherence and purposeful boundaries: add one small pure AI-decision module and one shared current-player action operation; do not create an AI-specific player, projectile, or framework.
- Determinism/testability: derive a dedicated authoritative AI seed from the existing match seed, preserve stable identity ordering, and test decisions independently of rendering/input.
- Presentation boundary: controller dispatch reads configuration and requests ordinary actions; HUD/camera remain observers. A presentation delay may not mutate or resolve gameplay.
- Scope/roadmap/quality: firing-first scope defers movement, memory, advanced weapon evaluation, wind compensation, personalities, and planning. Existing quality gates remain required.

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260909-192622-ai-controller/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── ai-controller.md
└── tasks.md                 # Created later by $speckit-tasks
```

### Source Code (repository root)

```text
crates/azimuth-game/
├── src/
│   ├── main.rs              # Match resources, input/AI dispatch, shared action, scene/HUD
│   ├── ai.rs                # New pure deterministic decision policy and tests
│   ├── match_setup.rs       # Start validation and configured controller data
│   ├── turn.rs              # Current-player aim and ordered authoritative turn state
│   ├── weapon.rs            # Existing selection and commitment rules
│   ├── tank.rs              # Existing player-owned tank state
│   ├── projectile.rs        # Existing launch, wind, collision, and flight behavior
│   └── battlefield.rs       # Existing authoritative terrain
└── Cargo.toml
docs/
└── roadmap.md               # Update only demonstrated AI availability/capabilities
```

**Structure Decision**: Keep the pure policy in one new sibling module because it needs no
rendering types and is independently testable. Keep controller dispatch and the extracted shared
launch operation beside existing input systems in `main.rs`, where the authoritative resource
boundary is visible.

## Complexity Tracking

No constitution violations or complexity exceptions are required.

## Post-Design Constitution Check

**PASS** — research and data design retain one player collection, one turn state, one weapon
inventory, one projectile lifecycle, and one terrain authority. The only new state is a small
authoritative AI decision cursor and optional presentation-only readiness state. No new crate,
dependency, generic command bus, or AI framework is justified.

