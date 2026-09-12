# CI Workflow Contract

## Trigger contract

| Event | Branch | Required outcome |
|---|---|---|
| Push | `master` | Quality validation, then three native artifacts on success. |
| Pull request | Target `master` | Quality validation only. |

## Quality contract

The quality job uses the checked-in Rust policy and succeeds only when all of these pass:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo check --workspace --all-targets
cargo build --workspace
```

No distribution job may begin without a successful quality job for the same revision.

## Artifact contract

| Artifact | Runner/architecture | Package-root content |
|---|---|---|
| `azimuth-windows-x86_64` | Windows x86_64 | `azimuth-game.exe`, `assets/`, optional `BUILD.txt` |
| `azimuth-linux-x86_64` | Linux x86_64 | executable `azimuth-game`, `assets/`, optional `BUILD.txt` |
| `azimuth-macos-arm64` | macOS arm64 | executable `azimuth-game`, `assets/`, optional `BUILD.txt` |

`assets/` is the full `crates/azimuth-game/assets` copy, including OGG files used by current AssetServer paths. Upload requires executable and audio checks.

## Failure contract

Failure of quality, release build, architecture assertion, staging, required-file validation, or upload fails the relevant job. The workflow uses no secrets and creates no releases, tags, installers, signing, or deployment output.
