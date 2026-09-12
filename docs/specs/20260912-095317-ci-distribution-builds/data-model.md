# Data Model: Continuous Integration and Cross-Platform Distribution Builds

This feature has no application-persistent data model. Its operational entities are workflow inputs and package outputs.

## Quality Run

| Field | Description | Validation |
|---|---|---|
| revision | Commit checked out by workflow | All checks use the same checkout. |
| trigger | Push or PR targeting `master` | Only configured branch events invoke workflow. |
| status | Pass/fail quality gate | Formatter, Clippy, test, check, and build must succeed. |

A successful quality run is the prerequisite for distribution builds on pushes. A PR quality run has no distribution builds.

## Distribution Build

| Field | Description | Validation |
|---|---|---|
| platform | Windows, Linux, or macOS | Exactly one matrix entry per target. |
| architecture | x86_64 or arm64 | macOS runner architecture is checked before label use. |
| runner | Hosted native runner | Must match matrix platform/architecture. |
| release executable | Cargo output for `azimuth-game` | Must exist before staging. |
| status | Pass/fail package build | Build, staging, validation, and upload must succeed. |

A master-push quality run creates exactly three distribution builds; each produces one runtime package and artifact.

## Runtime Package

| Field | Description | Validation |
|---|---|---|
| package directory | `azimuth-{platform}-{architecture}` | New/clean; no source or Cargo metadata. |
| executable | `.exe` on Windows; no suffix elsewhere | Exists in package root; Unix executable bit preserved. |
| assets directory | Copy of crate assets | Sibling `assets/` includes required audio. |
| attribution | Optional `BUILD.txt` | If present: commit, platform, architecture, Actions origin. |

State transition: `release-built` → `staged` → `layout-verified` → `uploaded`. A failure stops the platform job.
