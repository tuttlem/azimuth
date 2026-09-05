<!--
Sync Impact Report
- Version change: 1.0.0 -> 1.1.0
- Modified principles: VIII. Scope Discipline; IX. Roadmap Discipline; X. Timestamped
  Specifications; Development Workflow and Quality Gates.
- Added sections: none.
- Removed sections: none.
- Follow-up TODOs: none.
-->

# Azimuth Constitution

## Core Principles

### I. Fun Over Realism

Azimuth is a game first. Physics, simulation, environmental modelling, weapons, movement,
terrain, and related systems MUST primarily serve enjoyable, understandable gameplay. Physical
realism is valuable only when it improves the player's experience; a simpler or deliberately
exaggerated model MUST be preferred when it produces clearer, more satisfying, or more
interesting play. The goal is internally consistent, learnable physics rather than a reproduction
of real-world Earth physics, so players can build useful intuition about the world.

### II. Emergent Gameplay Through Simple Systems

The game MUST prefer a small set of understandable, composable systems over bespoke gameplay
rules. Projectile motion, gravity, wind, atmosphere, terrain, terrain deformation, movement, and
weapons SHOULD combine to produce emergent play. New simulation complexity MUST create
meaningful gameplay possibilities and MUST NOT exist solely for technical sophistication; prefer
environmental parameter changes and composition of existing mechanics to special cases.

### III. Code Coherence Over Cleverness

Code MUST optimise for readability, understandability, maintainability, and ease of modification.
Prefer straightforward, explicit Rust. Abstractions MUST earn their existence through real project
requirements: do not introduce speculative abstractions, unjustified trait hierarchies, generics
that only remove small duplication, macros where ordinary Rust is clear, conventional game
patterns without need, excessive module or crate fragmentation, or premature generalisation. A
developer SHOULD be able to understand a subsystem without architectural archaeology.

### IV. Workspace With Purposeful Boundaries

Azimuth MUST remain a Rust Cargo workspace. Crates SHOULD exist only when they create a concrete
architectural, dependency, testing, ownership, or reuse boundary; they MUST NOT merely organise
source files or anticipate hypothetical requirements. Prefer a small set of coherent crates, and
extract, combine, split, or remove boundaries when experience demonstrates that it improves
coherence. Individual specifications MAY evolve the workspace without this constitution fixing
crate names or topology.

### V. Deterministic and Testable Simulation

Simulation and game-domain behaviour SHOULD be deterministic wherever practical. Randomness that
affects simulation MUST be controllable through explicit seeds or an equivalent reproducible
mechanism. Physics and simulation SHOULD be testable independently of rendering, audio, input,
and other presentation concerns. Tests MUST target observable behaviour and invariants, remain
readable, and add regression coverage for important gameplay bugs where practical.

### VI. Presentation Must Not Own the Game

Core gameplay and simulation rules SHOULD remain separable from presentation technology where
practical. A selected engine, renderer, ECS, UI framework, audio system, or equivalent MUST NOT
unnecessarily dictate the core domain representation. Engine-specific types SHOULD NOT leak into
engine-independent simulation when a simple boundary prevents it. This principle does not require
elaborate abstraction layers: a direct, understandable integration boundary is preferred.

### VII. Playable Progress Over Infrastructure

Azimuth MUST become playable early and remain playable throughout development. Work SHOULD favour
vertical slices with visible or interactive gameplay over prolonged infrastructure construction,
and infrastructure MUST answer an actual need. After the foundation exists, work MUST prioritise
the smallest satisfying artillery interaction: aim -> fire -> projectile arcs through the world ->
impact -> visible result. The project MUST NOT become a physics engine, general-purpose game
engine, architecture experiment, or technology demonstration at the expense of becoming a game.

### VIII. Scope Discipline

Ideas are encouraged; implementation scope is constrained. Interesting future work SHOULD be
recorded in the roadmap rather than implemented immediately, and roadmap placement MUST NOT be
treated as an implementation commitment. Each specification MUST select coherent, bounded work
and MUST NOT opportunistically implement unrelated roadmap items. Work discovered during
implementation SHOULD be recorded for later unless it is required to satisfy the active
specification correctly.

### IX. Roadmap Discipline

`docs/roadmap.md` MUST be the long-lived project backlog. It MUST use Markdown checkboxes for
actionable work, group related work understandably, capture known features, experiments, and
ideas, and distinguish near-term work from speculation where useful. Specifications that satisfy
roadmap acceptance criteria MUST update the corresponding items. Newly discovered ideas SHOULD
normally be added to the roadmap instead of silently expanding the active specification.

### X. Timestamped Specifications

Specifications MUST live beneath `docs/specs/` and use timestamp identifiers, not sequential
feature numbers. The directory format is `YYYYMMDD-HHMMSS-feature-name`, using the project's
local development time; for example, `docs/specs/20260905-091846-workspace-setup/`. SpecKit
workflow, feature-creation tooling, templates, and project guidance MUST preserve this
convention. Specifications MUST be independently understandable, explicitly name meaningful
dependencies, and MUST NOT rely on chronological order as an implicit dependency mechanism.

### XI. Quality Is Part of Completion

Completed work MUST leave the repository healthy unless its specification documents a justified
exception. The workspace MUST compile; relevant tests, formatting, and Clippy MUST pass without
unjustified warnings; affected documentation MUST be accurate; and completed roadmap work MUST
be updated. Warnings, failing tests, TODO implementations, disabled checks, or commented-out
code MUST NOT be used merely to make work appear complete.

### XII. Dependencies Must Earn Their Place

Dependencies SHOULD be added deliberately. Before adding one, contributors MUST consider whether
it solves a meaningful problem better than a small, clear local implementation; this MUST NOT be
used to justify reimplementing mature libraries unnecessarily. Established libraries are preferred
for substantial solved problems, but trivial conveniences do not justify dependencies. Large
architectural dependencies, including game engines, ECS frameworks, physics engines, and rendering
frameworks, MUST be deliberate project decisions. Dependencies that no longer provide value SHOULD
be removed.

### XIII. Comments Explain Intent

Code SHOULD explain mechanics through clear naming and structure. Comments MUST explain intent:
why behaviour exists, non-obvious gameplay decisions, mathematical reasoning, important
invariants, deliberate approximations, or surprising constraints; they MUST NOT merely translate
obvious code into English. Physics code SHOULD document its gameplay model and assumptions enough
for another developer to understand the intended behaviour without reconstructing the mathematics.

### XIV. Refactoring Is Normal

Architecture is not sacred. Refactoring SHOULD occur when it materially improves coherence,
readability, testability, or the ability to implement gameplay. Contributors MUST NOT preserve a
poor abstraction merely because an earlier specification created it. A specification MAY include
focused refactoring needed for its feature; unrelated large-scale cleanup SHOULD become separate
roadmap work.

### XV. The Game Must Remain Fun to Build

Azimuth is a recreational project. Technical decisions SHOULD preserve experimentation, visible
progress, and enjoyment of development. Between otherwise reasonable choices, prefer the approach
that keeps the project understandable and makes gameplay experimentation easier. The project MUST
leave room for ridiculous weapons, strange worlds, surprising interactions, and ideas discovered
through play; it MUST NOT optimise the fun out of the project.

## Project Constraints

Azimuth is a turn-based 3D artillery game in the spirit of classic artillery games such as
Scorched Earth. Its battlefield includes positional movement, deformable terrain, multi-axis
environmental effects, and varied conditions. These describe the project context, not a mandate
for any particular feature, engine, renderer, ECS, physics model, or crate topology. A proposed
technical choice MUST be judged against the core principles, especially learnable gameplay,
purposeful boundaries, and deliberate dependencies.

## Development Workflow and Quality Gates

Specifications MUST define bounded, independently understandable work and document meaningful
dependencies explicitly. Before creating a feature specification, contributors MUST create and
switch to a dedicated feature branch. Specification, planning, tasking, implementation, and
feature-specific documentation work MUST occur only on that feature branch until the feature is
integrated. Before considering a specification complete, contributors MUST review it and its
implementation for compliance with this constitution, update the roadmap items whose acceptance
criteria were satisfied, and record newly discovered nonessential work in `docs/roadmap.md`.
Quality checks MUST be proportionate to the change but may not be bypassed merely for convenience.
Exceptions require an explicit, justified statement in the relevant specification.

## Governance

This constitution governs all Azimuth specifications and implementation work and supersedes
conflicting project practices. A proposed implementation that conflicts with it MUST change, unless
the project's principles themselves have genuinely changed and the constitution is intentionally
amended and documented. Compliance review is required when creating specifications, reviewing
implementation plans, and declaring work complete.

Amendments MUST state the reason and update the Sync Impact Report. Governance changes use semantic
versioning: MAJOR for backward-incompatible removals or redefinitions of principles, MINOR for new
principles or materially expanded guidance, and PATCH for clarifications, wording, or other
non-semantic refinements. When principles conflict, apply them in this order: fun and playable
gameplay; code coherence and maintainability; correctness and reproducibility; scope discipline;
architectural elegance; then realism and technical sophistication. The simplest interpretation
consistent with these principles prevails; the constitution guides engineering judgement rather
than creating bureaucracy.

**Version**: 1.1.0 | **Ratified**: 2026-09-05 | **Last Amended**: 2026-09-05
