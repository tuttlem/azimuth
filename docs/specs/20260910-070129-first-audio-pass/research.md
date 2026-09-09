# Research: First Audio Pass

## Decision: Reuse existing engine audio support

The current Bevy 0.18.1 dependency already provides one-shot, looping, positional playback, listener support, gain, pitch, and automatic cleanup.

**Rationale**: Native support provides coherent world cues without a dependency or generic audio layer.

**Alternatives considered**: A third-party mixer or manual acoustics adds infrastructure with no current gameplay value.

## Decision: Map existing authoritative facts to small presentation cues

Successful shared launch requests fire and starts flight; resolved terrain impact requests explosion and stops flight; newly observed elimination may request one restrained accent; wind controls ambience. Each cue is consumed once by presentation only.

**Rationale**: These shared Human/AI boundaries already exist. Audio neither predicts collision nor creates a rules path.

## Decision: Use shared families with profile-led scale variation

Basic and Heavy share a launch family, with Heavy optionally deeper; High Explosive uses its existing larger impact profile for a stronger boom.

**Rationale**: Existing definition data provides stable identity without bespoke libraries or controller/name rules.

## Decision: Keep flight and ambience sparse

One quiet flight loop exists only during the one active projectile; one quiet wind loop maps bounded match wind strength to bounded gain.

**Rationale**: Preserve report → anticipation → boom rather than continuous noise.

## Decision: Use a tiny documented asset set

Use only appropriately licensed repository assets with provenance in `assets/audio/README.md`; generated placeholders are labelled. Asset/playback failure remains presentation-only.
