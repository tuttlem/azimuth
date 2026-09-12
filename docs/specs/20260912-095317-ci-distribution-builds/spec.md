# Feature Specification: Continuous Integration and Cross-Platform Distribution Builds

**Feature Branch**: `20260912-095317-ci-distribution-builds`  
**Created**: 2026-09-12  
**Status**: Draft  
**Input**: Automate quality validation and downloadable native Windows, Linux, and macOS builds for every push to master.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Trust a master change (Priority: P1)

When a change reaches `master`, maintainers can see one clear automated result showing whether Azimuth meets the same health standards expected locally before distribution builds begin.

**Why this priority**: A distributable build is valuable only when it comes from a healthy revision.

**Independent Test**: Push a valid change to `master` and inspect its automation run; deliberately introduce a formatting, lint, test, or build failure in a test change and verify the run visibly fails without treating the revision as fully distributable.

**Acceptance Scenarios**:

1. **Given** a push to `master`, **When** automation begins, **Then** it validates formatting, warning-free linting, deterministic tests, and workspace compilation using the repository's checked-in toolchain policy.
2. **Given** any required quality check fails, **When** the run completes, **Then** the failure is visible and no platform distribution job is considered successful for that revision.
3. **Given** a pull request targeting `master`, **When** validation is enabled, **Then** it provides repository-health feedback without requiring costly distribution artifacts.

---

### User Story 2 - Download the correct native game build (Priority: P1)

After a healthy `master` push, a player can open the completed automation run and download a clearly named build for Windows, Linux, or the native architecture produced by the selected macOS runner.

**Why this priority**: The feature's central value is giving collaborators runnable builds without source checkout or Rust installation.

**Independent Test**: Complete a healthy `master` run, inspect the artifacts, and confirm exactly one clearly identified native package is available for each required platform.

**Acceptance Scenarios**:

1. **Given** quality validation passes on a `master` push, **When** distribution builds complete, **Then** separate artifacts exist for Windows, Linux, and macOS with truthful platform and architecture names.
2. **Given** a platform package is downloaded and unpacked, **When** the recipient opens it, **Then** it contains the native Azimuth executable and only runtime-required supporting files plus optional minimal build attribution.
3. **Given** a required platform build or package step fails, **When** the automation run completes, **Then** the run visibly reports the platform failure rather than silently presenting a complete distribution result.

---

### User Story 3 - Run an unpacked build (Priority: P1)

People on each supported platform can unpack the corresponding artifact and launch Azimuth without cloning the repository, installing Rust, copying resources manually, or changing hidden working-directory settings.

**Why this priority**: A binary is not a usable distribution if its sounds or other runtime resources are missing.

**Independent Test**: Download each available platform artifact onto its target operating system, unpack it, launch the executable from the package, start a round, and observe loaded audio and normal graphical gameplay.

**Acceptance Scenarios**:

1. **Given** an unpacked Windows package, **When** its executable is launched, **Then** the game can locate all packaged runtime resources and start normally.
2. **Given** an unpacked Linux package on a compatible graphical Linux system, **When** its executable is launched with normal executable permission, **Then** the game can locate all packaged runtime resources and start normally.
3. **Given** an unpacked macOS package on a compatible system, **When** its executable is launched subject to ordinary unsigned-binary operating-system behavior, **Then** the package layout and binary architecture are clear and all runtime resources are present.

### Edge Cases

- The application currently loads required OGG audio at runtime; generated terrain, meshes, materials, effects, and default UI font do not require separately shipped source assets.
- A platform package must not accidentally include source, tests, target output, specifications, Cargo metadata, or developer-only files.
- Automated tests must remain headless and must not require a graphical display merely to validate the game domain.
- A cache miss must rebuild successfully; a cache hit must never bypass a required validation or cause stale output to be distributed.
- A macOS artifact must state its actual runner-native architecture and must not claim universal support.
- Normal unsigned-macOS/Gatekeeper behavior and host graphics/audio driver requirements are documented limitations, not reasons to suppress build failure.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Repository automation MUST run for every push to `master` and use one understandable configuration located beneath `.github/workflows/`.
- **FR-002**: Automation MUST apply the repository's existing local quality policy: formatting verification, warning-denied linting across workspace targets/features, workspace tests, and compilation/build validation.
- **FR-003**: Automation MUST honour the checked-in stable toolchain and requested formatter/linter components rather than independently selecting a conflicting toolchain version.
- **FR-004**: Distribution builds MUST begin only after required quality validation succeeds for the same revision.
- **FR-005**: A successful `master` revision MUST produce optimized native build artifacts for Windows, Linux, and macOS using supported hosted native environments.
- **FR-006**: Every platform job MUST verify that the expected native executable was built and that its staged package contains every runtime-required resource before upload.
- **FR-007**: Each completed distribution artifact MUST have a clear, truthful platform-and-architecture name and must contain a self-contained runnable directory, optionally including minimal source/build attribution.
- **FR-008**: Packaging MUST preserve the runtime resource layout required by Azimuth, including its shipped audio files, so users can run the unpacked game without source checkout or Rust tooling.
- **FR-009**: Automation MUST use ordinary dependency/build caching keyed sufficiently to prevent stale dependencies or outputs from determining correctness.
- **FR-010**: Linux build provisioning MUST include only the native development libraries required by the currently enabled game dependency set.
- **FR-011**: Pull requests targeting `master` SHOULD receive quality validation; distribution artifacts MAY be reserved for pushes to `master` to avoid unnecessary cost.
- **FR-012**: The automation configuration MUST use minimal permissions, require no secrets, and avoid deployment, release, signing, installer, updater, or telemetry infrastructure.
- **FR-013**: README and roadmap documentation MUST state that CI now validates `master`, describe where to find platform artifacts, record known platform caveats, and replace the obsolete CI-deferral rationale.
- **FR-014**: The existing game-domain tests MUST remain runnable without launching the graphical application or requiring a virtual display.

### Key Entities

- **Quality Run**: The visible result for one source revision's repository-health checks.
- **Distribution Build**: The validated optimized native output for one operating system and architecture.
- **Runtime Package**: The unpackable executable plus only the resources required by the game at launch.
- **Build Attribution**: Optional small metadata identifying the source revision and target platform that produced a package.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of pushes to `master` start a visible quality-validation run automatically.
- **SC-002**: A healthy `master` run completes all four required quality outcomes—formatting, warning-denied linting, tests, and compilation—before platform package jobs begin.
- **SC-003**: Every healthy `master` run provides exactly three downloadable native artifacts: one Windows, one Linux, and one macOS artifact, each truthfully named for its target architecture.
- **SC-004**: For each supported package manually tested on its target operating system, unpacking and launching requires no Rust installation, source checkout, or manual asset copy, and the game reaches a playable round with audio resources available.
- **SC-005**: Package validation rejects any platform artifact missing its executable or required runtime resources before upload.
- **SC-006**: The workflow contains no signing, deployment credentials, release publication, installer, tag, or version-bump steps.
- **SC-007**: A deliberately introduced formatting, lint, test, compile, or package failure makes the relevant workflow run visibly fail in 100% of validation trials.

## Assumptions

- Native hosted runners are sufficient for the initial builds; the macOS artifact is for the runner's actual native architecture, not a universal binary.
- The package needs the executable plus the existing `crates/azimuth-game/assets` runtime asset tree. Current rendering, terrain, effects, UI font, and visual dressing are code/default-generated and need no extra packaged source files.
- Windows, Linux, and macOS users provide ordinary operating-system graphics, windowing, and audio support; dependency bundling, installers, code signing, and notarization are out of scope.
- GitHub Actions run metadata supplies baseline traceability; a small package build-information file is optional.
- Existing release-mode defaults are adequate for this feature; no profile tuning is needed merely for distribution.

## Dependencies

- Existing workspace manifest, lockfile, stable toolchain policy, local quality commands, `azimuth-game` executable target, runtime asset-loading behavior, README, and roadmap.
- GitHub-hosted automation runners and their standard artifact/cache capabilities.

## Out of Scope

- GitHub Releases, tags, versioning automation, deployment services, installers, app bundles/disk images, universal macOS binaries, package-manager distribution, code signing, notarization, updates, telemetry, Docker/WASM/mobile/server builds, graphical hosted-runner smoke tests, and new game-domain functionality.
