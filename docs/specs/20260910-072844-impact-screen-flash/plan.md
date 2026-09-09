# Implementation Plan: Impact Screen Flash

**Branch**: `20260910-072844-impact-screen-flash` | **Date**: 2026-09-10 | **Spec**: [spec.md](spec.md)

## Summary

Reuse the existing once-per-terrain-impact presentation consumption and impact visual scale to create one short, bounded full-screen white overlay pulse. It fades independently of simulation and does not react per damaged tank.

## Technical Context

**Language/Version**: Rust edition 2024
**Primary Dependencies**: Bevy 0.18.1 UI and current presentation systems
**Storage**: In-memory presentation state only
**Testing**: Module-local unit tests and manual visual review
**Target Platform**: Native desktop
**Project Type**: Single game application crate
**Performance Goals**: One UI overlay and lightweight per-frame fade only
**Constraints**: One pulse per impact; rapid bounded fade; no strobing or gameplay authority
**Scale/Scope**: One active projectile and one latest terrain impact

## Constitution Check

### Pre-design gate — PASS

| Principle | Plan response |
|---|---|
| Fun over realism | A brief exaggerated accent reinforces impact weight. |
| Coherence | Reuse the existing visual-impact boundary and HUD/UI facilities. |
| Determinism | Pulse reads impact context only. |
| Presentation boundary | No simulation or turn system reads flash state. |
| Scope discipline | No accessibility UI, post-processing framework, or camera change. |

## Project Structure

```text
docs/specs/20260910-072844-impact-screen-flash/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/impact-screen-flash.md

crates/azimuth-game/src/main.rs  # overlay, pulse lifecycle, tests
```

**Structure Decision**: Keep the overlay and its pure profile helpers beside the existing explosion visual and tactical HUD in `main.rs`.

## Post-design Constitution Check

**PASS.** One bounded UI pulse observes the existing resolved impact and cannot alter authoritative play.

## Complexity Tracking

No exception is required.
