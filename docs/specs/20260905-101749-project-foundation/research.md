# Research: Project Foundation

## Decision: Use a virtual Cargo workspace with one `azimuth-game` executable member

**Rationale**: A virtual workspace keeps project metadata at the repository root while
`crates/azimuth-game` provides a concrete home for the game application. This is a purposeful
boundary: it keeps game code out of the workspace root and enables later sibling crates such as
`azimuth-math` or physics only when they earn their own dependency, testability, reuse, or
ownership boundary. The executable provides a meaningful compile and run target without
representing future gameplay.

**Alternatives considered**:

- A root-package workspace was rejected because it puts game code directly in the repository root
  and obscures the application boundary the project wants to preserve.
- Multiple future-domain crates were rejected because they would violate the purposeful-boundary
  and scope-discipline principles.
- No workspace was rejected because the feature explicitly requires a Cargo workspace.

**References**: [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html),
[cargo check](https://doc.rust-lang.org/cargo/commands/cargo-check.html).

## Decision: Check in a stable-channel toolchain declaration with formatter and linter components

**Rationale**: A project toolchain file can be committed and selects the stable channel for every
developer who enters the repository. Declaring the standard formatter and linter components makes
the documented quality commands available without nightly, a dated version pin, or an undocumented
local setup requirement.

**Alternatives considered**:

- Relying on each developer's default toolchain was rejected because it leaves the project policy
  and quality-tool availability implicit.
- Pinning a specific compiler release was rejected because the requested policy is current stable
  and a pinned release adds maintenance without a demonstrated compatibility need.
- Nightly was rejected because no unstable feature has been justified.

**Reference**: [Rustup toolchain files](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file).

## Decision: Use direct Cargo commands for all local repository-health checks

**Rationale**: Direct, workspace-wide Cargo commands are readable, familiar to Rust developers,
and meet the foundation validation requirements without wrappers. The lint command treats warnings
as failures so they are resolved intentionally rather than hidden.

| Check | Command | Purpose |
|---|---|---|
| Fast compilation validation | `cargo check --workspace --all-targets` | Check all current targets without final code generation. |
| Full build | `cargo build --workspace` | Produce the workspace's build artifacts when a full build is wanted. |
| Tests | `cargo test --workspace` | Run the workspace's unit, integration, and documentation tests. |
| Formatting | `cargo fmt --all -- --check` | Report formatting differences without changing files. |
| Linting | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | Run all lints and fail on warnings. |

**Alternatives considered**:

- A Makefile, Justfile, scripts, or Cargo aliases were rejected because they only wrap a small set
  of already concise commands.
- `cargo build` as the only routine compilation check was rejected because `cargo check` provides
  faster feedback; the full build remains documented separately.
- Broad lint allowances were rejected because they conceal issues and violate the specification.

**References**: [cargo check](https://doc.rust-lang.org/cargo/commands/cargo-check.html),
[cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html),
[cargo fmt](https://doc.rust-lang.org/cargo/commands/cargo-fmt.html),
[cargo clippy](https://doc.rust-lang.org/cargo/commands/cargo-clippy.html).

## Decision: Use standard colocated unit tests; defer integration support and CI

**Rationale**: Standard unit tests are sufficient to demonstrate the initial workspace. Integration
tests, shared test helpers, mocks, and CI have no concrete need before an external interface or
repeated collaboration workflow exists. Future simulation tests will use deterministic inputs and
control randomness; meaningful gameplay defects will gain readable regression tests where practical.

**Alternatives considered**:

- Establishing integration-test infrastructure now was rejected because there is no external
  behaviour to exercise and no duplication to remove.
- Adding CI now was rejected because no hosting or automation need has been established. The
  roadmap keeps CI open for deliberate later adoption.

**Reference**: [cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html).
