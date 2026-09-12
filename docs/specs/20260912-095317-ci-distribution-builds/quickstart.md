# Quickstart: Validate CI and Downloadable Builds

## Prerequisites

- GitHub Actions enabled after merging this branch into `master`.
- A target OS with ordinary graphical, graphics, and audio support for manual launch acceptance.
- Rust stable plus checked-in components only for local workflow validation.

## Local pre-push validation

Run the same policy CI enforces from the repository root:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo check --workspace --all-targets
cargo build --workspace
```

See [contracts/ci-workflow.md](contracts/ci-workflow.md) for the workflow and package contracts.

## Automated acceptance

1. Push a validated revision to `master` and open `CI / Build` in Actions.
2. Confirm the Linux quality job completes each command successfully.
3. Confirm all dependent package jobs succeed and expose `azimuth-windows-x86_64`, `azimuth-linux-x86_64`, and `azimuth-macos-arm64`.
4. Inspect/download each artifact. Confirm `assets/audio/fire.ogg`, `impact.ogg`, and `turret-dink.ogg` are present beside its executable.
5. On a disposable test revision, introduce one formatting, Clippy, test, build, or staging failure. The relevant job must fail; a quality failure cannot report dependent distribution output.

## Manual runtime acceptance

### Windows

Extract `azimuth-windows-x86_64`; confirm `azimuth-game.exe` and sibling `assets/`, launch it, start a round, fire a weapon, and verify audio/visuals load without missing-resource errors.

### Linux

Extract `azimuth-linux-x86_64`. If needed, run `chmod +x azimuth-game`, then `./azimuth-game`. Start a round and verify audio/rendering. Normal host graphics/audio driver support is not bundled.

### macOS

Extract `azimuth-macos-arm64` on compatible Apple silicon. Confirm executable, `assets/`, optional `BUILD.txt`, and arm64 architecture. Launch where local policy permits. Normal unsigned-binary Gatekeeper behavior is expected; signing/notarization is excluded. If manual execution is unavailable, record that limitation rather than claiming acceptance.

## Acceptance Record

The repository-local quality commands and YAML parsing were verified while implementing this
feature. The first GitHub-hosted Windows, Linux, and macOS workflow run—and manual launch of its
downloaded artifacts—must be recorded after this branch is merged or pushed to `master`. They
cannot be truthfully completed from this Linux development workspace before the `master` trigger
exists. In particular, macOS runtime acceptance remains pending access to compatible Apple
silicon hardware, and unsigned-binary Gatekeeper behavior must be noted if it prevents launch.
