# Research: Move or Fire — Basic Tactical Movement

## Movement model

**Decision**: Use one-world-unit, cardinal, discrete movement requests with six accepted steps per movement action.

**Rationale**: The game already has keyboard `just_pressed` gameplay controls, a small bounded terrain surface, and deterministic simulation. One request maps to player intent, gives a legible allowance, avoids frame-rate-dependent authority, and remains tactical rather than vehicle driving.

**Alternatives considered**: Continuous movement would need elapsed-time authority and risk driving-like interaction. A tile/grid would add a second spatial representation over the continuous piecewise-planar terrain. Pathfinding is unnecessary for a player-chosen short sequence.

## Terrain slope and deformation

**Decision**: Use the absolute height difference between the current terrain surface at movement start and destination, accepting at most 0.75 rise/run.

**Rationale**: `BattlefieldTerrain` already owns bounded interpolation, craters, collision, and mesh positions. Endpoint queries give a simple local slope that naturally reacts to craters and grounds an accepted destination on the same surface.

**Alternatives considered**: Renderer normals/collision would make presentation own gameplay; authored terrain would ignore craters; traction, sliding, and slope-cost systems exceed the feature.

## Action authority

**Decision**: Evolve the existing phase to `Choosing`, `Moving { remaining }`, and `ResolvingFire`.

**Rationale**: These are the only observable action states required. They authorize aim only while choosing, fire only from choosing, movement only while moving, and handoff only after movement completion or fired-shot resolution.

**Alternatives considered**: A generic action/ability framework would obscure three fixed states. Inferring state from visuals or disabled controls would violate authoritative exclusivity. Advancing after every movement step would remove the multi-step tactical choice.

## Orientation, aim, and launch

**Decision**: An accepted step changes only body direction. Retained azimuth, elevation, and power continue to drive turret/barrel direction and launch from the new pose.

**Rationale**: Existing tank body/turret concepts and aim-derived firing representation already support this split. Position changes the firing solution without deleting player knowledge.

**Alternatives considered**: Resetting or rotating aim arbitrarily penalizes bracketing. Rotating the whole visual parent would wrongly couple an independently aimed turret. Hull pitch/roll is vehicle presentation out of scope.

## Input and presentation

**Decision**: Use M, I/J/K/L, and Enter as direct keyboard adapters; hold transient movement rejection feedback separately for HUD display; tag tank root/body visuals for pose projection.

**Rationale**: The keys do not conflict with camera WASD/arrows, aiming Q/E/R/F/G/T, or Space. Each press issues one ordered request; HUD and transforms observe rather than decide gameplay.

**Alternatives considered**: Camera key reuse creates competing intent. An action bar or path preview is unnecessary. Camera-system movement couples gameplay to render timing.

## Validation approach

**Decision**: Test terrain, tank, and turn invariants in pure unit tests; retain only focused application helpers for key selection and launch/resolution integration; manually validate the playable loop.

**Rationale**: The constitution requires deterministic gameplay independent of renderer, UI, and frame timing.

**Alternatives considered**: Keyboard-event, exact-HUD-text, and render-driven tests are brittle and do not prove authority.
