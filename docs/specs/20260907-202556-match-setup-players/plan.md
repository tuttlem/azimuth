# Implementation Plan: Match Setup — Named 2–8 Player Matches and Controller Slots

**Branch**: feature/match-setup-players | **Date**: 2026-09-07 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from docs/specs/20260907-202556-match-setup-players/spec.md

## Summary

Replace direct startup into a fixed two-player duel with a compact Match Setup screen. The screen edits an engine-independent MatchConfiguration containing two through eight named slots, stable player identities, controller type, and visual identity. A validated all-human configuration creates the existing game through collection-based player, tank, aim, weapon, turn, combat, settling, camera, and HUD paths. AI is represented faithfully in configuration, given an isolated setup-only generated name, and blocks match start until an AI controller exists.

The focused architectural evolution replaces the current two-element/other-player representations with ordered, identity-addressed collections. This preserves ordinary projectile, weapon, terrain, movement, and impact behaviour while making every configured player a first-class participant.

## Technical Context

**Language/Version**: Rust, edition 2024

**Primary Dependencies**: Bevy 0.18.1

**Storage**: In-memory Bevy resources and entities; no persistence or external storage

**Testing**: Rust tests through cargo test; manual desktop play for 2-, 3-, 4-, and 8-human matches

**Target Platform**: Local desktop application supported by the existing Bevy build

**Project Type**: Rust Cargo workspace with one Bevy desktop-game crate

**Performance Goals**: Preserve the deterministic fixed-step projectile/terrain loop and responsive tactical HUD while handling at most eight tanks and player-status rows

**Constraints**: 2–8 local slots; AI configuration is visible but cannot start; no AI decisions, networking, persistence, menu framework, procedural spawns, or new gameplay physics; presentation does not own authoritative match configuration or runtime state

**Scale/Scope**: One game crate; one pre-match screen; maximum eight configured local players, with all existing weapon, terrain, combat, camera, and turn interactions retained

## Constitution Check

| Gate | Pre-design status | Planned evidence |
|---|---|---|
| Fun and simple emergent systems | Pass | Adds understandable local-player setup and preserves existing projectile, wind, terrain, and weapon rules. |
| Code coherence and boundaries | Pass | Adds only a configuration domain and collection-based state demanded by 2–8 play; no controller hierarchy or menu framework. |
| Deterministic and testable simulation | Pass | Stable identities, slot order, spawn layout, turn rotation, effects, and victory are testable. Setup-name randomness is isolated. |
| Presentation must not own the game | Pass | Bevy widgets edit domain configuration; validation and match creation remain engine-independent where practical. |
| Playable progress and scope | Pass | Delivers local human multiplayer. AI visibly blocks launch and unrelated setup choices remain deferred. |
| Roadmap, timestamp, and quality discipline | Pass | Uses timestamped spec; implementation will update only demonstrated roadmap items and run format, tests, Clippy, and build. |

**Post-design re-check**: Pass. The research and data model retain these small explicit boundaries; no constitution exception is required.

## Project Structure

### Documentation (this feature)

~~~text
docs/specs/20260907-202556-match-setup-players/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── match-setup.md
├── checklists/
│   └── requirements.md
└── tasks.md                    # Created by $speckit-tasks
~~~

### Source Code

~~~text
Cargo.toml
crates/azimuth-game/
├── Cargo.toml
└── src/
    ├── main.rs                 # App state, setup UI, ECS integration, dynamic HUD/camera
    ├── match_setup.rs          # New configuration, validation, names, setup-only helpers
    ├── tank.rs                 # Player identity, tank state, deterministic multi-player spawns
    ├── turn.rs                 # Living-player rotation and per-player aim/turn state
    ├── weapon.rs               # Independent loadouts for all player identities
    ├── combat.rs               # Explosion effects across all tanks
    ├── aiming.rs               # Existing human action interpretation
    ├── projectile.rs           # Existing deterministic flight/impact behaviour
    ├── battlefield.rs          # Existing terrain and deformation support
    └── world.rs                # Existing world conventions
docs/
└── roadmap.md                  # Completion-only evidence and deferred discoveries
~~~

**Structure Decision**: Keep one purposeful game crate. Add one compact match_setup domain module because pre-match configuration, validation, controller types, and setup-only naming must remain usable without Bevy widgets. Refactor existing domain modules in place from two-player arrays to identity-addressed collections; do not add crates, a generic menu framework, or a parallel AI player model.

## Implementation Approach

1. Establish MatchConfiguration and validated PlayerConfiguration slots before changing UI. Give each slot a stable PlayerId, explicit ControllerType, display name, and one of eight curated visual identities. Keep name generation setup-local and testable.
2. Convert gameplay to ordered player collections: tanks/spawns, turn and aim lookup, independent weapon loadouts, combat damage, support settling, survivor calculation, and result state. Preserve the current common projectile and impact pipeline.
3. Add a Bevy Match Setup app state and compact tactical-style screen. It edits/presents configuration, displays validation, and hands a valid all-human configuration to match creation. AI slots visibly block Start.
4. Initialise and render all configured humans. Generate deterministic supported fixed spawn positions, focus the active tank, derive player HUD rows/result text from the collection, and preserve the active player's ordinary input path.
5. Test configuration and runtime boundaries, representative 2/3/4/8-player regressions, effects/turn/win cases, quality checks, and manual local matches.

## Complexity Tracking

No constitution violations require justification.
