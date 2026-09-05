# Implementation Plan: Project Foundation

**Branch**: `feature/project-foundation` | **Date**: 2026-09-05 | **Spec**:
[spec.md](./spec.md)

**Input**: Feature specification from `docs/specs/20260905-101749-project-foundation/spec.md`

## Summary

Establish Azimuth as a minimal virtual Cargo workspace with one `azimuth-game` executable member,
no runtime dependencies, a checked-in stable-toolchain policy, standard Cargo validation commands,
focused repository hygiene, and concise developer documentation. The foundation deliberately
defers CI and every rendering, engine, simulation, and gameplay decision.

## Technical Context

**Language/Version**: Rust stable channel, selected through a checked-in toolchain declaration;
no nightly or unstable features.

**Primary Dependencies**: None. The standard library and Cargo provide all foundation behaviour.

**Storage**: N/A; this feature does not persist application data.

**Testing**: Standard Cargo unit tests colocated with the code they verify. Integration tests and
shared test support are deferred until a real external interface or duplication warrants them.

**Target Platform**: Developer machines supported by the stable Rust toolchain; no game platform
or renderer target is selected.

**Project Type**: Virtual Rust Cargo workspace with one `azimuth-game` executable member that
meaningfully identifies the project foundation without implementing gameplay.

**Performance Goals**: Validation feedback should use the fast workspace check command for normal
development; no runtime or gameplay performance goal is in scope.

**Constraints**: No runtime dependencies; no engine, renderer, ECS, physics, gameplay, terrain,
audio, networking, persistence, task runner, custom build script, or CI implementation. Use
standard formatting without custom rules and direct Cargo commands without wrappers.

**Scale/Scope**: One `azimuth-game` package and source directory, root workspace metadata and
hygiene files, the root README, and roadmap updates for completed foundation work. CI roadmap
items remain open.

## Constitution Check

*GATE: Passed before Phase 0 research; re-checked after Phase 1 design.*

| Principle | Plan response | Result |
|---|---|---|
| Fun over realism / emergent systems | No gameplay or simulation is introduced. | Pass |
| Code coherence / purposeful workspace boundaries | `azimuth-game` is a concrete application boundary; no future-domain member or abstraction is added. | Pass |
| Determinism and presentation boundary | Future simulation test guidance is documented; no presentation technology is selected. | Pass |
| Playable progress / scope discipline | This bounded foundation unblocks the next deliberate technology feature and defers unrelated work. | Pass |
| Roadmap and timestamped specifications | Work remains under the active timestamped directory; implementation will complete applicable roadmap items only. | Pass |
| Quality and dependencies | Standard Cargo health gates, stable Rust, and zero dependencies satisfy the quality and dependency principles. | Pass |

**Post-design review**: Passed. Research confirms that a virtual workspace with an `azimuth-game`
member, a checked-in stable toolchain declaration, and direct Cargo commands meet the requirements
without speculative dependencies or future-system boundaries.

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260905-101749-project-foundation/
├── checklists/
│   └── requirements.md
├── data-model.md
├── plan.md
├── quickstart.md
├── research.md
└── spec.md
```

No `contracts/` directory is created: this foundation exposes no external API, protocol, CLI
schema, or service interface.

### Source Code (repository root)

```text
.
├── .gitignore
├── Cargo.toml
├── README.md
├── crates/
│   └── azimuth-game/
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
├── rust-toolchain.toml
└── docs/
    ├── roadmap.md
    └── specs/
        └── 20260905-101749-project-foundation/
```

**Structure Decision**: Use a virtual workspace with `crates/azimuth-game` as the sole initial
member. The game is a concrete application boundary, so its code does not live directly in the
repository root. This leaves the root for workspace and project metadata and makes later members
such as `azimuth-math` or a physics crate natural additions only when their real dependency,
testability, reuse, or ownership boundary is demonstrated.

## Implementation Approach

1. Create the root Cargo manifest as a virtual workspace using Cargo's current workspace resolver,
   with `crates/azimuth-game` as its sole member. Add no dependencies, workspace dependency table,
   lint table, aliases, or build script.
2. Add one small `azimuth-game` executable target and a readable unit test that demonstrate normal
   compilation and test discovery without modelling any game system.
3. Add a checked-in stable Rust toolchain declaration that includes the formatter and linter
   components. Do not pin a dated compiler release or enable nightly.
4. Add a focused `.gitignore` for Cargo build output and common local editor/operating-system
   artifacts. Do not add generated build output to version control.
5. Write the root README with project orientation, current foundation status, stable-toolchain
   policy, direct validation commands, documentation locations, deterministic-test guidance,
   regression-test guidance, and the required feature-branch workflow.
6. Mark only the completed Project Foundation and near-term repository-foundation roadmap items as
   done. Record that CI was evaluated and deliberately deferred, leaving its automation checkboxes
   unchecked.
7. Validate the final repository using the commands in [quickstart.md](./quickstart.md).

## Complexity Tracking

No constitutional violations or complexity justifications are required.
