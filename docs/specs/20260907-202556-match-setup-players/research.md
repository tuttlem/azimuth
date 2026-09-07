# Research: Match Setup — Named 2–8 Player Matches and Controller Slots

## Separate configuration from running player state

**Decision**: Add a small engine-independent setup module containing prospective configuration, validation, controller type, stable identity, visual identity, and AI-name helpers. Create mutable tanks, aim, inventory, movement, survival, and turn state only after Start accepts a valid all-human configuration.

**Why**: Bevy widgets cannot be the source of truth. Configuration is explicit match input; runtime state is authoritative after play begins.

**Rejected alternatives**: UI-only storage is renderer-coupled and hard to test. Putting configuration on tanks fails because tanks do not exist before a match.

## Generalise identity and participants, not merely UI

**Decision**: Evolve PlayerId beyond One/Two and remove other() as a domain operation. Use stable identity independently of display name and an ordered participant collection for rotation, lookup, HUD, effects, and victory.

**Why**: Existing enums, two-element tank/loadout arrays, and toggled turns encode a duel. A collection is the smallest real model for 2–8 players and multiple eliminated-player skips.

**Rejected alternatives**: Eight enum variants/fixed arrays preserve coupling. Names cannot be keys because they are editable and duplicate. A generic ECS/controller framework is not needed.

## AI is configuration, not a player subtype

**Decision**: Each slot stores ControllerType::Human or ControllerType::Ai. Preserve that model for the next feature, but validation blocks match creation if an AI exists.

**Why**: A future AI should submit ordinary actions for ordinary state. Special AI tanks/inventories, silent Human conversion, or skipped AI turns would be misleading.

## Isolate setup-name randomness

**Decision**: A curated pool of at least sixteen names is selected with a setup-local random source, preferring unused configuration names. The chosen string is captured in its slot; optional reroll uses the same path.

**Why**: Names are configuration/presentation, not simulation. This prevents UI interaction from changing wind, projectile, terrain, or other deterministic gameplay sequences.

**Rejected alternatives**: Rendering-time generation makes names unstable; gameplay RNG contaminates deterministic outcomes; procedural names add needless complexity.

## Use curated deterministic spawns

**Decision**: Define a fixed supported, in-bounds, non-overlapping spawn/facing layout for up to eight identities, allocated in slot order. Adjust the development arena only if necessary to validate eight tanks.

**Why**: It is reproducible and testable without claiming final fair randomised placement has been solved.

## Derive all presentation from collections

**Decision**: Setup renders one row per configuration entry. HUD renders one compact status row per participant; active identity drives emphasis and camera targeting.

**Why**: It retains a clean duel, scales to eight without bespoke panels, and eliminates Player One/Two text paths. A generic responsive UI framework is unnecessary.
