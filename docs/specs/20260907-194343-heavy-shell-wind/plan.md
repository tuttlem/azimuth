# Implementation Plan: Heavy Shell Wind Resistance

**Branch**: `feature/heavy-shell-wind-resistance` | **Date**: 2026-09-07 | **Spec**: [spec.md](spec.md)

## Summary

Add Heavy Shell as the third conventional weapon: two rounds per player, Basic Shell-class impact,
and a captured 0.40 horizontal-wind response. Evolve the currently empty projectile profile by one
validated, generic wind-response value. Copy that value into the launched projectile so shared
fixed-step flight scales wind while preserving gravity, collision, impact, and deterministic
resolution. Extend the existing explicit inventory, direct selection, HUD, and regression tests;
do not add mass, drag, special firing paths, or weapon-name logic in simulation.

## Technical Context

**Language/Version**: Rust edition 2024, stable toolchain

**Primary Dependencies**: Bevy 0.18.1 for desktop presentation and input; existing local game-domain modules

**Storage**: In-memory authoritative match resources; no persistence or external data files

**Testing**: Rust unit tests in source modules; `cargo test --workspace`; manual local two-player validation

**Target Platform**: Local desktop game supported by the current Bevy application

**Project Type**: Single Rust desktop-game workspace member

**Performance Goals**: Preserve the existing 120 Hz fixed authoritative projectile step and responsive local presentation; this one scalar multiply adds no material per-step allocation or new frame dependency

**Constraints**: Wind stays a constant horizontal match acceleration; gravity remains independent and vertical; flight must use immutable committed state; camera/HUD/render timing must not influence simulation; no new dependency or physical mass/drag model

**Scale/Scope**: One game crate, three conventional weapons, two local players, one active projectile, and focused unit/manual regression coverage

## Constitution Check

### Pre-design gate — PASS

| Principle | Plan response |
|---|---|
| Fun over realism | Uses a visible 0.40 gameplay wind-response multiplier rather than mass or aerodynamics. |
| Simple composable systems | Adds one real per-projectile scalar to the existing wind/gravity composition; no Heavy-Shell simulation branch. |
| Coherence and purposeful boundaries | Keeps the weapon definition, fired-shot, projectile, and existing resolver boundaries; no crate, trait hierarchy, dependency, or parallel firing path. |
| Determinism and testability | Captures response at commitment and tests exact fixed-step comparisons, zero-wind invariance, reversal, strength, gravity, and repeat traces. |
| Presentation does not own the game | HUD only reports selected availability; presentation/camera never selects or modifies flight values. |
| Playable progress and scope discipline | Delivers one bounded conventional weapon and records camera work separately on the roadmap. It excludes all future weapon and physics ideas. |
| Quality | Plans focused tests, documentation updates, formatting, Clippy, build, and manual two-player matches before roadmap completion claims. |

No constitutional violation or exception is needed.

## Research Findings

See [research.md](research.md). All technical-context questions are resolved from the current
repository. The chosen boundary is `WeaponDefinition → ProjectileProfile → Projectile → shared
simulation`; `FiredShot` remains the immutable authority containing that projectile and copied
impact profile.

## Design

See [data-model.md](data-model.md) and [conventional-weapons.md](contracts/conventional-weapons.md).

### Implementation sequence

1. Add a small validated `WindResponse` gameplay value in the projectile domain, with normal
   response and a finite non-negative constructor/accessor. Store it on `Projectile`, set it at
   launch, and scale only `Wind::horizontal_acceleration()` before combining it with gravity.
2. Replace the empty `ProjectileProfile` with its `WindResponse`; provide normal and 0.40 Heavy
   Shell profile constants. Add `WeaponId::HeavyShell`, `HEAVY_SHELL_STARTING_ROUNDS`, display
   metadata, and a Basic Shell-class impact profile in the central catalogue.
3. Extend the current explicit per-player loadout entries and generic availability/commit matching
   to include Heavy Shell. Keep Basic Shell unlimited and the existing final-round fallback.
4. Construct the projectile from the committed definition's profile at the launch boundary, then
   construct the existing `FiredShot`. This order makes wind response immutable before flight.
5. Extend the normal direct selector and its concise HUD control text so all three conventional
   weapons are reachable. Keep selected-name/ammunition rendering generic through the catalogue.
6. Add focused module tests: profile validation and wind maths; Heavy Shell catalogue/loadout/
   snapshot behavior; normal selector and HUD text; common flight, impact, settling, victory, and
   baseline/HE regression. Update player/projectile documentation and roadmap items only where
   manual evidence meets their acceptance criteria.

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260907-194343-heavy-shell-wind/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── conventional-weapons.md
└── tasks.md                 # Created later by $speckit-tasks
```

### Source Code

```text
crates/azimuth-game/
├── src/
│   ├── main.rs              # launch boundary, normal selection, HUD, game-level tests
│   ├── projectile.rs        # wind response, fixed-step flight, trajectory tests
│   └── weapon.rs            # catalogue, profile, per-player loadout, fired-shot tests
docs/
├── projectile-model.md      # player/developer ballistic model and wind-response semantics
├── roadmap.md               # completed items and recorded play findings only
└── specs/20260907-194343-heavy-shell-wind/
```

**Structure Decision**: Retain the existing single game crate and module boundaries. The feature
requires no new crate, runtime content format, persistence, network interface, or external API.

## Post-design Constitution Check — PASS

The data model introduces only the real wind-response variation required by Heavy Shell. The
contract preserves immutable shot authority and shared deterministic resolution. The quickstart
separates automated proof from manual balance evidence. No design adds a dependency, generic weapon
scripting model, physical mass semantics, or presentation-owned gameplay state.

## Complexity Tracking

No constitutional exceptions or complexity justifications are required.
