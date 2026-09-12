# Research: Continuous Integration and Cross-Platform Distribution Builds

## Decision: one workflow, Linux quality gate, master-only distribution matrix

**Rationale**: A single workflow makes the path from revision to artifact legible. Formatting, linting, tests, checking, and workspace build run once on Ubuntu, applying the local policy without tripling Bevy compilation. `needs: quality` prevents distribution after a failed gate. Pull requests get health feedback; packages are reserved for `master` pushes.

**Alternatives considered**: Full quality on every platform was rejected as costly without a platform-specific test requirement. Separate quality/release workflows obscure their dependency. PR packages spend native-build capacity on non-distribution revisions.

## Decision: checked-in Rust policy and maintained Actions

**Rationale**: `rust-toolchain.toml` remains authoritative. Setup supplies stable and its `rustfmt`/`clippy` components without pinning a numerical compiler version. Maintained checkout, Rust setup, Cargo-cache, and artifact-upload actions minimize custom YAML. Caching incorporates OS, `Cargo.lock`, and toolchain identity while Cargo commands still execute.

**Alternatives considered**: Preinstalled runner Rust makes component availability implicit. Hand-rolled caching adds maintenance. A numerical Rust pin can drift from the checked-in policy.

## Decision: native Windows x86_64, Ubuntu x86_64, and macOS arm64

**Rationale**: Target runners avoid cross-compilation complexity around Bevy native dependencies. `macos-14` is selected for Apple silicon, with architecture verified before applying an arm64 label. An explicit matrix makes names, runners, and executable suffixes visible.

**Alternatives considered**: Linux cross-compilation is less dependable for native desktop packages. Universal macOS output is expressly out of scope. A generic macOS runner label might change architecture and make an artifact name misleading.

## Decision: only current Bevy 0.18.1 Ubuntu development dependencies

**Rationale**: With Bevy default native windowing/audio features, Ubuntu compilation needs `g++`, `pkg-config`, `libx11-dev`, `libasound2-dev`, `libudev-dev`, and `libxkbcommon-x11-0`. CI compiles but does not launch a graphical game.

**Alternatives considered**: Broad desktop meta-packages obscure requirements. Virtual-display launching is beyond the compile/package goal and no clean headless launch mechanism exists.

## Decision: stage executable plus complete crate asset tree

**Rationale**: Azimuth loads `audio/fire.ogg`, `audio/impact.ogg`, and `audio/turret-dink.ogg` through Bevy `AssetServer`. Copying `crates/azimuth-game/assets` as sibling `assets/` preserves these paths. Terrain, meshes, materials, effects, and UI font are code/default-generated. `BUILD.txt` gives attribution without versioning infrastructure.

**Alternatives considered**: Uploading `target/release` directly can include non-runtime files. Embedding audio changes application behavior solely for CI. Installers, disk images, and bundles are outside scope.

## Decision: structural CI checks plus manual target-OS launch

**Rationale**: Each job verifies staged executable and audio files before upload. Hosted runners need not launch graphical Bevy; `quickstart.md` defines end-to-end unpack/launch acceptance.

**Alternatives considered**: Upload success alone cannot prove an artifact is complete. Screenshot/virtual-display smoke tests are outside scope.
