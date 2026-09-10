# Implementation Plan: MIRV — First Multi-Projectile Weapon

**Branch**: `20260910-181415-mirv-weapon` | **Date**: 2026-09-10 | **Spec**: [spec.md](spec.md)

## Summary

Add a two-round MIRV which fires one ordinary carrier, replaces it at the first authoritative apex crossing with five fixed-order children, and completes only after the complete barrage and settling. Extend committed-shot ownership, not projectile physics: every child uses existing fixed-step gravity, captured wind, swept collision, impact, crater, damage, and settling. A read-only shot snapshot and ordered presentation events let camera, pulse, and audio handle a barrage safely.

## Technical Context

**Language/Version**: Rust stable. **Dependencies**: Bevy/Cargo workspace. **Storage**: in-memory deterministic match state. **Testing**: `cargo test -p azimuth-game`, manual desktop match review, fmt and Clippy. **Platform**: existing native desktop game. **Scope**: one weapon, at most five children, 2–8 players, no new dependencies. **Constraints**: 120 Hz deterministic integration; stable terrain-mutation order; presentation has no authority; no configurable split/count or AI strategy.

## Constitution Check

### Pre-design: PASS

MIRV composes existing simple systems into a distinct coverage weapon. One shot-owned ordered active-projectile collection is earned by the demonstrated single-projectile limit; a graph/scripting system is not. Stable order preserves reproducibility and tests. Presentation consumes read-only data/events and cannot delay simulation. The scope is a playable Arsenal slice with no new dependency.

### Post-design: PASS

The design adds only the required active-shot collection and presentation event stream, because the current single projectile/latest impact cannot represent a multi-impact barrage. No exception or complexity tracking is needed.

## Project Structure

```text
crates/azimuth-game/src/
├── weapon.rs      # MIRV definition, ammo, committed shot ownership
├── projectile.rs  # existing reusable fixed-step primitive
├── main.rs        # advance/resolution, HUD, visuals, camera, audio, pulse
├── ai.rs          # Basic-Shell AI regression
└── turn.rs        # unchanged turn state, exercised by resolution tests
docs/
├── projectile-model.md
└── roadmap.md
docs/specs/20260910-181415-mirv-weapon/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/mirv-shot-presentation.md
```

**Structure Decision**: Retain the current single game crate and direct module boundaries; MIRV does not justify a crate or generic weapon framework.

## Implementation Sequence

1. Add MIRV inventory/definition and a committed shot with ordered active projectile records; conventional shots have one record.
2. Advance records in stable order. A normal carrier that survives a step with non-positive vertical velocity replaces itself with five children; collision/bounds on that step takes precedence. Children first advance next fixed tick.
3. Resolve each impact through explosion → crater → support reconciliation, remove only that record, and complete only when no records and no tanks are settling.
4. Update Digit-4 selection/HUD, visual mapping, and carrier/spread/aggregate camera snapshots.
5. Drain ordered resolved-impact presentation events: create every visual, coalesce pulse response, and bound close child audio without changing authority.
6. Add deterministic/regression tests, update projectile documentation and delivered roadmap boxes, then complete manual acceptance and quality checks.
