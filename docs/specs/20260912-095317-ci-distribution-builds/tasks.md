---

description: "Actionable implementation tasks for continuous integration and cross-platform builds"
---

# Tasks: Continuous Integration and Cross-Platform Distribution Builds

**Input**: Design documents from `/docs/specs/20260912-095317-ci-distribution-builds/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/ci-workflow.md`, and `quickstart.md`

**Tests**: The feature requires CI to execute the existing workspace health commands. It does not require new game-domain test code or graphical hosted-runner tests.

**Organization**: Tasks are grouped by user story so quality validation, platform artifacts, and unpacked runtime behavior have distinct completion checks.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish the single readable workflow entry point.

- [X] T001 Create `.github/workflows/ci-build.yml` with the `CI / Build` name, `master` push and pull-request triggers, and minimal read-only workflow permissions.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish shared native-build setup before implementing the quality gate and packages.

**⚠️ CRITICAL**: Complete this phase before user-story implementation; later tasks extend the same workflow file.

- [X] T002 Configure checked-in stable Rust toolchain setup and maintained Cargo dependency/build caching in `.github/workflows/ci-build.yml`, ensuring cache identity includes runner OS, toolchain, and `Cargo.lock`.
- [X] T003 Add the documented minimal Bevy 0.18.1 Ubuntu development-package provisioning (`g++`, `pkg-config`, X11, ALSA, udev, and XKB dependencies) to `.github/workflows/ci-build.yml`.

**Checkpoint**: The workflow has a repeatable toolchain/cache/base-native-dependency setup.

---

## Phase 3: User Story 1 - Trust a master change (Priority: P1) 🎯 MVP

**Goal**: Every push and eligible pull request gets a visible health result matching the local repository policy, and no package job can begin after a failed health gate.

**Independent Test**: Inspect a `master` or pull-request workflow run and confirm format, warning-denied Clippy, tests, workspace check, and workspace build execute in the quality job. A controlled failure must make that job red and leave dependent package jobs unstarted.

- [X] T004 [US1] Implement the Linux `quality` job in `.github/workflows/ci-build.yml` to run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace`, `cargo check --workspace --all-targets`, and `cargo build --workspace`.
- [X] T005 [US1] Add the `quality` job dependency and pull-request-versus-master-push conditions to `.github/workflows/ci-build.yml` so distribution work can only follow a successful master-push gate.

**Checkpoint**: User Story 1 is independently complete when a PR receives only the complete quality policy and a failed quality command visibly blocks all distribution work.

---

## Phase 4: User Story 2 - Download the correct native game build (Priority: P1)

**Goal**: A healthy `master` push provides one clearly and truthfully named native artifact for Windows, Linux, and macOS.

**Independent Test**: After a healthy master run, the Actions page shows exactly `azimuth-windows-x86_64`, `azimuth-linux-x86_64`, and `azimuth-macos-arm64`; each platform build has used a release executable and a failed build/upload turns its job red.

- [X] T006 [US2] Define the Windows x86_64, Ubuntu x86_64, and `macos-14` arm64 native build matrix—including runner, artifact name, executable suffix, and macOS architecture assertion—in `.github/workflows/ci-build.yml`.
- [X] T007 [US2] Implement release compilation of package `azimuth-game` and explicit expected-executable checks for every matrix entry in `.github/workflows/ci-build.yml`.
- [X] T008 [US2] Upload each verified staged platform directory with the matrix artifact name via a maintained artifact action in `.github/workflows/ci-build.yml`.

**Checkpoint**: User Story 2 is independently complete when a healthy master revision produces three distinct, truthfully named native artifact downloads and any release-build/upload failure is visible.

---

## Phase 5: User Story 3 - Run an unpacked build (Priority: P1)

**Goal**: Each artifact is a clean runnable directory whose executable can find every runtime resource without a source checkout or Rust installation.

**Independent Test**: Download and unpack each artifact on its target OS. Confirm package root contains only the native executable, sibling `assets/`, and optional `BUILD.txt`; start a round and fire a weapon without missing-resource errors. If macOS execution is unavailable, record the limitation.

- [X] T009 [US3] Stage a clean per-platform package directory in `.github/workflows/ci-build.yml` by copying only the release executable and `crates/azimuth-game/assets` as sibling `assets/`, excluding source, `target`, tests, specs, and Cargo metadata.
- [X] T010 [US3] Add pre-upload package validation and optional commit/platform/architecture `BUILD.txt` generation in `.github/workflows/ci-build.yml`; verify the executable plus `assets/audio/fire.ogg`, `impact.ogg`, and `turret-dink.ogg` exist.
- [X] T011 [US3] Perform and record the target-OS manual unpack/launch acceptance procedure in `docs/specs/20260912-095317-ci-distribution-builds/quickstart.md`, including Linux executable permission handling and macOS unsigned/Gatekeeper limitations.

**Checkpoint**: User Story 3 is independently complete when the downloaded package layout passes CI checks and target-OS acceptance confirms assets, audio, and playable gameplay load from the unpacked directory.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Align public documentation, roadmap status, and final repository health with the completed workflow.

- [X] T012 [P] Update CI status, master artifact discovery, validation commands, and platform caveats in `README.md`; remove the obsolete CI-deferral statement and misleading stale status claims relevant to the feature.
- [X] T013 [P] Mark Automation / CI justification, build validation, tests, formatting, and Clippy tasks complete and replace the deferral note in `docs/roadmap.md`; add a compact downloadable-build note.
- [X] T014 Reconcile the implemented workflow against the trigger, quality, artifact, and failure requirements in `docs/specs/20260912-095317-ci-distribution-builds/contracts/ci-workflow.md`.
- [X] T015 Run the documented formatter, Clippy, test, check, and build commands from `README.md` at repository root; inspect `git diff --check` and confirm the final workflow only contains scoped CI/package changes.

**Checkpoint**: Documentation accurately describes current automation, roadmap items reflect completion, and local quality remains healthy.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Starts immediately.
- **Foundational (Phase 2)**: Depends on T001; blocks workflow implementation because all later tasks reuse its toolchain and Linux setup.
- **US1 (Phase 3)**: Depends on T002–T003. It must complete before platform builds because `quality` is their gate.
- **US2 (Phase 4)**: Depends on T004–T005; provides release executable and artifact mechanics.
- **US3 (Phase 5)**: Depends on T006–T008; adds the runtime package contents that artifact upload publishes.
- **Polish (Phase 6)**: Depends on all three stories; T012 and T013 may run in parallel, then T014–T015 close out validation.

### User Story Dependencies

- **US1 (P1)**: Requires foundational runner setup only. It is the MVP and gate for all artifact work.
- **US2 (P1)**: Requires US1 because packages must not run for failed quality revisions.
- **US3 (P1)**: Requires US2 because it validates/stages the artifact directory produced by the matrix.

### Parallel Opportunities

- T012 and T013 modify distinct documentation files and may run in parallel after workflow behavior is known.
- Manual Windows, Linux, and macOS portions of T011 may be carried out in parallel by people with the relevant operating systems after T009–T010 land.

## Parallel Example: Documentation and Manual Acceptance

```text
Task: "Update CI status and artifact guidance in README.md"
Task: "Update completed Automation / CI work in docs/roadmap.md"

Task: "Manually unpack and launch azimuth-windows-x86_64 on Windows"
Task: "Manually unpack and launch azimuth-linux-x86_64 on Linux"
Task: "Manually inspect or launch azimuth-macos-arm64 on macOS"
```

## Implementation Strategy

### MVP First: User Story 1

1. Complete T001–T003 to establish one workflow and shared runner setup.
2. Complete T004–T005 and validate that quality runs on both a pull request and master push.
3. Stop at the Phase 3 checkpoint: the repository now has a dependable visible quality gate.

### Incremental Delivery

1. Add the gated native release matrix and uploads (T006–T008).
2. Add clean runtime staging and structural package checks (T009–T010).
3. Perform target-OS acceptance and align the README/roadmap (T011–T015).

Every increment preserves the same central rule: a package is available only from a revision that passed the local-equivalent health policy.
