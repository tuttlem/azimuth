# Data Model: Project Foundation

This feature introduces no game-domain data model, persistence, or runtime state. The following
repository-level artifacts are the only entities whose relationships matter to implementation.

| Entity | Responsibility | Relationship and validation |
|---|---|---|
| Workspace manifest | Defines the virtual root and the complete Cargo workspace. | Includes `azimuth-game` as its sole member and contains no runtime dependencies. Workspace-wide commands must select it successfully. |
| Toolchain policy | Declares the supported stable Rust channel and quality-tool availability. | Applies to all workspace commands; must not request nightly or unstable features. |
| `azimuth-game` executable | Provides the smallest meaningful runnable project target. | Belongs to the `azimuth-game` member; must not model gameplay, rendering, or future systems. |
| Unit test | Demonstrates standard test discovery and an observable foundation invariant. | Lives with the code it verifies; must be readable and require no shared support. |
| Root README | Orients contributors and documents validation and workflow. | References the roadmap and timestamped specification location; must match actual commands and policy. |
| Roadmap | Records completed foundation work and deferred CI work. | Is updated after implementation satisfies acceptance criteria; unimplemented automation remains open. |

There are no state transitions, external contracts, or persisted records in scope.
