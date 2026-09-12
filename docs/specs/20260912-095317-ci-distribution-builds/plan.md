# Implementation Plan: Continuous Integration and Cross-Platform Distribution Builds

**Branch**: `20260912-095317-ci-distribution-builds` | **Date**: 2026-09-12 | **Spec**: [spec.md](./spec.md)

## Summary

Add one readable GitHub Actions workflow that validates every push and pull request targeting `master` on Ubuntu. Only after a successful `master` push quality gate, it builds optimized native packages on Windows, Ubuntu, and Apple-silicon macOS runners. Each job stages the release executable with the existing crate `assets/` directory, verifies contents, writes concise attribution, and uploads a platform artifact. README and roadmap documentation will describe the policy and its intentional limits.

## Technical Context

**Language/Version**: Rust edition 2024; `rust-toolchain.toml` selects stable with `rustfmt` and `clippy`

**Primary Dependencies**: Cargo workspace; Bevy 0.18.1; GitHub Actions checkout, Rust setup, Cargo cache, and artifact-upload actions

**Storage**: N/A; GitHub Actions artifacts are temporary build downloads

**Testing**: Existing format, warning-denied Clippy, workspace tests, all-target check, and workspace build commands; workflow package-layout checks; manual target-OS launch acceptance

**Target Platform**: Hosted Windows x86_64, Ubuntu x86_64, and macOS arm64; downloaded native desktop packages

**Project Type**: Rust desktop game with repository automation

**Performance Goals**: Optimized release binaries; a single Linux quality gate avoids tripled Bevy validation while caching accelerates native builds

**Constraints**: Master push trigger; no secrets, releases, tags, signing, installers, or automated graphical launch; packages contain only executable, runtime assets, and optional small metadata; existing tests remain headless

**Scale/Scope**: One workflow YAML file, three native packages, documentation updates, and no game-domain changes unless runtime asset-path validation requires one

## Constitution Check

*Pre-research gate: PASS. Post-design gate: PASS.*

| Principle | Assessment |
|---|---|
| III. Code Coherence | One explicit quality job plus a small build matrix is simpler than coordinated workflows. |
| V. Deterministic and Testable Simulation | Existing domain tests run without a window or virtual display. |
| VI. Presentation Must Not Own the Game | Platform setup and packaging stay in CI; game code changes only if package validation proves an asset-root need. |
| VII. Playable Progress | The workflow meets a real downloadable-build need without release-engineering expansion. |
| VIII. Scope Discipline | Signing, installers, releases, deployment, universal binaries, and graphical smoke infrastructure remain excluded. |
| IX. Roadmap Discipline | The Automation / CI completion items and deferral note will be updated. |
| X. Timestamped Specifications | Artifacts remain on the dedicated timestamped feature branch. |
| XI. Quality Is Part of Completion | The documented formatter, Clippy, tests, check, and build commands are the gate. |
| XII. Dependencies Must Earn Their Place | Maintained Actions only; no new Cargo dependencies. |

No constitutional violation requires justification.

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260912-095317-ci-distribution-builds/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── ci-workflow.md
└── tasks.md                 # Created later by $speckit-tasks
```

### Source Code (repository root)

```text
.github/workflows/ci-build.yml    # New quality gate and master package matrix
crates/azimuth-game/assets/audio/ # Runtime OGG files copied into every package
crates/azimuth-game/src/main.rs   # Existing AssetServer paths; no planned change
README.md                         # CI and artifact discovery/caveats
docs/roadmap.md                   # Automation / CI completion and distribution note
```

**Structure Decision**: One internal workflow owns runner setup, validation, staging, and upload. The existing game crate remains executable and asset owner; no new crate, script framework, or runtime API is needed.

## Implementation Approach

1. Create `.github/workflows/ci-build.yml` named `CI / Build`, with minimal read-only permissions and `push`/`pull_request` triggers scoped to `master`.
2. Add Linux `quality`: check out revision, install `rust-toolchain.toml` policy, restore a Cargo cache keyed by OS/toolchain/`Cargo.lock`, install only current Bevy Linux development packages (including Wayland development files required by enabled Bevy features), then run the five documented health commands.
3. Add `build` requiring `quality`, restricted to pushes, with explicit `windows-latest`/x86_64, `ubuntu-latest`/x86_64, and `macos-14`/arm64 entries.
4. Release-build `azimuth-game`; stage only `target/release/azimuth-game[.exe]`, `crates/azimuth-game/assets`, and optional `BUILD.txt` in an artifact-named directory. Verify executable and required audio paths before upload.
5. Upload directories as `azimuth-windows-x86_64`, `azimuth-linux-x86_64`, and `azimuth-macos-arm64`. Actions artifacts are the download container—no independent archives.
6. Update README and roadmap with validation policy, artifact location, Linux executable-permission/host-library caveats, and unsigned macOS/Gatekeeper limitation.

## Complexity Tracking

Not applicable.
