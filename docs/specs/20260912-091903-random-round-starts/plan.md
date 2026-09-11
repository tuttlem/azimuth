# Implementation Plan: Randomized Round Starts

**Branch**: `master` | **Date**: 2026-09-12 | **Spec**: [spec.md](spec.md)

## Summary

Make every round creation use a distinct, reproducible round seed rather than reusing the process-start seed. Reuse the existing generated-terrain and seeded-start selector, but make the round-world factory retry a bounded sequence of derived candidate seeds when a terrain cannot host all players. Route both a new local game and a shop-to-next-round transition through one round-start boundary that refreshes the complete battlefield and its presentation while retaining session-owned player resources. Add pure regression coverage for seed derivation, valid starts, candidate retry, and reset ownership.

## Technical Context

**Language/Version**: Rust edition 2024

**Primary Dependencies**: Bevy 0.18.1; Rust standard library only for deterministic seed mixing

**Storage**: In-memory resources only; no persistence or external storage

**Testing**: `cargo test`; `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`

**Target Platform**: Desktop local graphical game on supported Bevy platforms

**Project Type**: Single desktop game application in one Cargo workspace crate

**Performance Goals**: Preserve responsive transition into the first turn; bounded generation must not create unbounded work or a visible stalled round start.

**Constraints**: Support the current 2–8 participants and 120-by-120 battlefield; retain current movement, weapons, damage, deformation, turns, and session economy; add no dependency, persistence, map-selection UI, or general random-number framework.

**Scale/Scope**: One existing `azimuth-game` crate; focused changes to the round generator, session-transition wiring, visual cleanup/recreation, and source-level regression tests.

## Constitution Check

### Pre-design gate — PASS

- **Fun over realism / emergent gameplay**: A new terrain and spread of starts makes ordinary movement, aiming, projectiles, wind, and terrain deformation produce a fresh tactical situation without special-case gameplay rules.
- **Coherence / purposeful boundaries**: Keep deterministic terrain in `battlefield.rs`, valid tank starts in `tank.rs`, session ownership in `session.rs`, and Bevy resource/entity wiring in `main.rs`. Do not introduce a map framework or random-service abstraction.
- **Determinism / testability**: Preserve the existing labelled seed streams. Record/reuse a session root and derive a specific round seed from it plus the round number; keep generation and retries pure and testable away from rendering.
- **Presentation does not own the game**: Recreate visual terrain-adjacent entities from the newly authoritative terrain, but leave gameplay terrain, starts, and cleanup independent of scene entities.
- **Playable progress / scope**: The work repairs one concrete continuing-round behavior and preserves the current combat loop. Tactical spawn fairness beyond valid, dry, gentle, separated starts remains deferred.
- **Quality / dependencies**: No dependencies are needed. Add focused regression tests and run formatting, Clippy, and the workspace suite before completion.

### Post-design gate — PASS

The design keeps the existing direct module ownership and labelled deterministic streams. The bounded candidate loop is only the minimum needed to turn the existing panic-on-unsuitable-terrain path into the required valid-round guarantee. No constitution exception or complexity tracking is required.

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260912-091903-random-round-starts/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── round-generation.md
└── tasks.md                 # Generated later by $speckit-tasks
```

### Source Code (repository root)
```text
crates/azimuth-game/
├── src/
│   ├── battlefield.rs       # deterministic terrain and terrain validity helpers
│   ├── tank.rs              # deterministic valid, separated start selection
│   ├── session.rs           # persistent session and per-round accounting ownership
│   └── main.rs              # round seed/root, factory, transition cleanup, Bevy scene rebuild
└── Cargo.toml
```

**Structure Decision**: Retain the existing single-crate module layout. This is lifecycle integration work, not a new architecture boundary.

## Complexity Tracking

No constitution violations require tracking.

## Implementation Approach

1. Define the lifecycle seed contract: choose a fresh root when a new local session begins; deterministically derive a round seed from that root and the one-based round number. Keep the existing labelled terrain, starts, dressing, wind, and AI substreams derived solely from the selected round seed.
2. Change the pure round-world factory to try a small named bounded sequence of candidate terrain/start seeds. A candidate succeeds only when the existing start selector yields every requested player on valid, separated terrain; failure advances deterministically to the next candidate rather than panicking.
3. Create one explicit round-start operation for initial setup and transition. It refreshes terrain, tanks, dressing, wind, turn state, initial aiming, selected weapon state, AI cursor, round accounting, and combat/presentation resources while copying only session-owned identity, controller, appearance, cash, wins, and ammunition.
4. Despawn/rebuild terrain-associated scene entities from the new authoritative world, including the horizon, buildings, tanks, HUD, projectile/impact/effect entities, and reset their matching resources before active play resumes.
5. Add pure/regression tests for seed reproducibility and round variation, 2–8 valid starts across a representative seed range, bounded retry behavior, round cleanup, preserved session values, and normal first-turn controls. Finish with workspace checks and a graphical multi-round smoke run.
