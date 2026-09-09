# Research: Shot Presentation Camera

## Decision: Capture controller-aware presentation at shared launch

Capture firing player identity and configured Human/AI classification when the existing shared firing function commits a shot. Do not infer it later from display name, keyboard input, AI decision, projectile behavior, or surviving Humans.

**Rationale**: Both controllers already use the same firing path. A presentation-only record is the smallest way to select coverage without creating gameplay paths.

**Alternatives considered**: Input/name inference fails renamed and all-AI cases; putting mode on projectile/weapon makes presentation authoritative; splitting firing paths risks divergent gameplay.

## Decision: Use a small explicit observational state

Use normal player view, Human follow, AI tactical, shared impact, and next/result outcomes. Human follow has an apex-observed marker that changes once when observed vertical velocity changes from positive to non-positive. AI never gains the close-follow phase.

**Rationale**: Current code has only active-player and generic watching-shot intent. Explicit transitions are understandable and unit-testable without a cinematic engine.

**Alternatives considered**: Scattered conditions obscure impact hold/final result; analytic landing prediction is unnecessary because velocity provides apex signal; a tween/timeline framework duplicates existing interpolation.

## Decision: Observe authoritative lifecycle and retain an independent impact snapshot

Observe live projectile state, latest terrain impact, tanks, and turn/match state. Terrain impact selects common impact framing. Keep a presentation snapshot long enough for the existing boom/crater and settling aftermath. Out-of-bounds uses safe normal/result fallback; winner/draw never selects a player view.

**Rationale**: Fixed update currently owns flight, crater, damage, settling, and immediate handoff/result. Camera must work even if removed.

**Alternatives considered**: Waiting for a camera timer before resolution violates separation. Using the explosion lifetime as authority is incorrect because it is transient. A new event system is not warranted unless direct observation proves insufficient.

## Decision: Prevent only a subsequent input/AI launch during brief aftermath

While a presentation impact hold is active, keep existing authoritative state resolving/ready as it already is, but prevent a new Human action or automatic AI launch until the consequence has been seen. This is required because an immediately following AI shot otherwise clears the retained latest-impact presentation.

**Rationale**: It makes consecutive AI turns readable while leaving every resolved shot result, settling process, handoff, winner/draw decision, and physics schedule independent.

**Alternatives considered**: Allow a new shot immediately and overwrite impact state; rejected because it defeats shared aftermath. Delay TurnState completion; rejected because camera would own authoritative progression.

## Decision: Derive bounded compositions from current state

Human coverage targets an offset behind/above observed travel direction, widens at apex, and shifts attention toward descent/impact. AI coverage derives a broad shooter-projectile-forward region and includes a Human tank only opportunistically. Impact coverage centres the impact and nearby living or settling tanks. Reuse existing target clamp, distance/pitch limits, smoothing, and zero-roll camera transform.

**Rationale**: It meets Human learning and AI awareness with physical observations only, without exposing AI intent or prediction.

**Alternatives considered**: Rigid projectile camera is disorienting; static overhead AI view loses route context; guaranteeing every tank in frame is impossible on varied 2–8-player geometry.

## Decision: Test decisions, not pixels

Add pure/unit-testable selector, transition, apex, pose-bounds, and safe-fallback tests near existing camera tests. Compare/retain deterministic resolution fixtures so presentation cannot mutate authoritative results.

**Rationale**: Observable state decisions provide robust coverage without fragile rendering tests.

**Alternatives considered**: Pixel tests are brittle; full schedule-only tests hide important state decisions.
