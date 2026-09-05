# Quickstart: Validate the Project Foundation

## Prerequisites

- Clone the repository and enter its root directory.
- Use Rust's stable toolchain. The checked-in toolchain policy supplies the formatter and linter
  components; if a local Rust installation predates the policy, update it before continuing.

## Validation

Run each command from the repository root.

```sh
cargo check --workspace --all-targets
cargo build --workspace
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Expected outcomes:

- The check and build commands complete successfully for the complete workspace.
- The test command reports all tests passing.
- The formatting command reports no files requiring changes.
- The lint command completes without warnings.

## Manual Documentation Review

1. Open the root `README.md`.
2. Confirm it introduces Azimuth, describes the foundation's current status and gameplay direction,
   and documents the commands above.
3. Follow its links to `docs/roadmap.md` and `docs/specs/`.
4. Confirm the roadmap marks completed foundation work and retains the explicitly deferred CI items
   as open work.

## Scope Review

Inspect the root workspace manifest and `crates/azimuth-game` source directory. The completed
foundation must have no runtime dependencies and no engine, renderer, ECS, physics, gameplay,
terrain, audio, networking, or other future-system implementation. There is no external interface
contract for this feature.
