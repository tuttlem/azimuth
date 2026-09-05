# Implementation Plan: Rendering Technology Selection

**Branch**: `feature/rendering-technology` | **Date**: 2026-09-05 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `docs/specs/20260905-104956-rendering-technology/spec.md`

## Summary

Select Bevy 0.18.1 as Azimuth's initial desktop rendering and application technology. Integrate it
directly into the existing `azimuth-game` crate and replace its foundation message with a minimal
3D proof: a Bevy-managed window, fixed perspective camera, ground plane, simple depth-visible
geometry, and basic light. Record why Bevy is preferable to a `wgpu`/`winit` stack for current
needs, while keeping gameplay and game-domain architecture out of scope.

## Technical Context

**Language/Version**: Rust stable; Bevy 0.18.1 requires Rust 1.89.0 or newer, which the project's
stable-toolchain policy currently satisfies.

**Primary Dependencies**: `bevy` 0.18.1 using its standard default feature set. No separate
windowing, renderer, UI, audio, asset, math, or effect dependency is added.

**Storage**: N/A; the proof has no persisted data or external assets.

**Testing**: Existing standard Cargo unit testing remains. No automated graphics, screenshot, GPU,
or headless-rendering test infrastructure is introduced.

**Target Platform**: Supported desktop development environments with the graphics drivers and
system libraries required by Bevy; no web, mobile, console, VR, or network platform is selected.

**Project Type**: Desktop graphical application within the existing `azimuth-game` workspace member.

**Performance Goals**: A minimal scene must open and remain responsive for manual inspection; no
frame-rate, scale, terrain, or gameplay performance target is in scope.

**Constraints**: Keep direct Bevy usage in the application crate. Do not add a renderer abstraction,
new workspace member, custom ECS, scene graph, controller, asset pipeline, gameplay component,
external art asset, permanent coordinate convention, or input binding beyond normal window close.

**Scale/Scope**: One decision record, one dependency addition, one minimal scene in the existing
application entry point, a concise README update, and accurate roadmap bookkeeping.

## Constitution Check

*GATE: Passed before Phase 0 research; re-checked after Phase 1 design.*

| Principle | Plan response | Result |
|---|---|---|
| Fun / playable progress | Bevy gets Azimuth to a visible 3D proof without building engine plumbing. | Pass |
| Simple, composable systems | No simulation or gameplay system is introduced; the proof stays small. | Pass |
| Code coherence | Direct Bevy code in one application entry point is clearer than an unearned wrapper. | Pass |
| Purposeful boundaries | The existing `azimuth-game` application boundary is sufficient; no crate is added. | Pass |
| Presentation boundary | Only presentation proof types live in the application crate; no game-domain model is manufactured. | Pass |
| Quality and dependencies | Bevy is a deliberate substantial dependency with a documented rationale; existing quality gates remain required. | Pass |
| Scope and roadmap discipline | Only technology decision, window proof, plane, and accurate foundation bookkeeping are completed. | Pass |

**Post-design review**: Passed. Bevy's integrated window lifecycle, 3D rendering, input, assets,
UI/audio compatibility, mutable meshes, and debug facilities satisfy current needs with less
non-game infrastructure than the lower-level alternative. Its dependency/compile cost, ECS model,
and API churn are documented trade-offs, not ignored.

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260905-104956-rendering-technology/
├── checklists/
│   └── requirements.md
├── data-model.md
├── plan.md
├── quickstart.md
├── research.md
└── spec.md
```

No `contracts/` directory is created: this desktop proof exposes no external API or protocol.

### Source Code (repository root)

```text
.
├── Cargo.toml
├── README.md
├── docs/
│   ├── adr/
│   │   └── 0001-initial-rendering-technology.md
│   └── roadmap.md
└── crates/
    └── azimuth-game/
        ├── Cargo.toml
        └── src/
            └── main.rs
```

**Structure Decision**: Retain the virtual workspace and its sole `azimuth-game` member. Bevy is
presentation infrastructure for the existing desktop application, not grounds to add a renderer or
engine crate. Later independent simulation or math concepts may establish their own boundaries
when actual requirements justify them.

## Implementation Approach

1. Add the selected Bevy dependency to `crates/azimuth-game/Cargo.toml` and update the lockfile;
   add nothing outside Bevy's requirements.
2. Replace the foundation executable body in `crates/azimuth-game/src/main.rs` with direct Bevy
   app setup, a close-on-window-request lifecycle system, and a startup scene system.
3. Build the proof from framework-provided primitive meshes and materials: a flat visual plane,
   simple elevated geometry, a fixed perspective camera, and light. Treat all scene transforms as
   presentation-local and do not name them as game coordinate semantics.
4. Add `docs/adr/0001-initial-rendering-technology.md` as one lightweight decision record. It compares Bevy with
   `wgpu` plus `winit`, records the selected version and trade-offs, and limits the decision's
   scope without introducing an ADR framework.
5. Update `README.md` with the run command and a link to the decision record; update only the
   roadmap items actually satisfied, including the already-complete Milestone A summary details.
6. Verify build, tests, formatting, Clippy, and the manual desktop scene validation in
   [quickstart.md](./quickstart.md).

## Complexity Tracking

No constitutional violation is introduced. Bevy's substantial dependency and compile cost are
accepted because it replaces a larger amount of otherwise project-owned windowing, rendering,
input, asset, UI, audio, and debug infrastructure needed to reach playable progress.
