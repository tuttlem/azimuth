# Research: Basic Player Aiming Controls

## Authoritative aiming state

**Decision**: Add a small engine-independent `AimingState` with explicit limits and adjustment operations. It owns normalized azimuth, bounded elevation, and bounded launch velocity; it builds existing `ShotParameters` from a supplied muzzle. Initial values are Player One's existing turret azimuth, 45 degrees elevation, and 18 units/s velocity.

**Rationale**: Aim is retained gameplay state and needs deterministic validation. These defaults retain the current visible development shot while making it playable.

**Alternatives considered**: HUD text or tank transforms as state would let presentation own gameplay. Generic weapon/input/aiming abstractions are unsupported by one tank and six controls. Held-key acceleration adds state where fixed presses plus Shift meet the precision/coarse need.

## Canonical angle conversion

**Decision**: Preserve `ShotParameters::launch_direction()` as the sole azimuth/elevation conversion. Add only a focused pure reuse helper there if needed so launch and barrel representation get the same normalized world direction.

**Rationale**: Existing projectile code validates and implements the documented convention, avoiding sign, axis, normalization, and elevation drift.

**Alternatives considered**: Duplicate trigonometry in rendering/tank code risks mismatch. Mutating `TankPose` with live aim would blur fixed placement and authoritative aim.

## Full barrel direction and muzzle

**Decision**: Derive Player One's firing representation from fixed tank placement and canonical launch direction. The pivot remains one unit above the tank base; the muzzle is 2.1 units along the full direction. Turret yaw uses its X/Z projection, barrel uses full direction, and marker sits at the muzzle.

**Rationale**: At zero elevation this equals the existing level muzzle relationship. At positive elevation the visual barrel and gameplay origin have one exact barrel-end point.

**Alternatives considered**: Keeping a level-only origin while rotating barrel geometry visibly disagrees. Rendering offsets make presentation authoritative. An articulation system is outside scope.

## Direct input and flight lock

**Decision**: Handle Q/E, R/F, T/G, and Space only while `ProjectileFlight` is empty. Shift selects coarse 5-degree or 2.5-unit steps; otherwise use 1 degree or 0.5 units. Opposite keys cancel.

**Rationale**: Keys avoid existing WASD/arrow camera panning; current `ButtonInput<KeyCode>` and `Option<Projectile>` suffice.

**Alternatives considered**: The I inspection shot is a parallel launch path; changing aim during flight is ambiguous; queueing shots violates the single-projectile boundary.

## Corner text HUD

**Decision**: Spawn one absolute-positioned Bevy UI `Text` with an `AimingHud` marker and update its string from aim state. It shows Player One, values/units, controls, and lock state.

**Rationale**: Bevy 0.18.1 default plugins and the existing 3D camera support this derived presentation without a dependency or UI hierarchy.

## Test strategy

**Decision**: Add unit coverage for normalization, bounds, fine/coarse changes, canonical launch conditions, muzzle geometry, and blocked input; run existing Cargo gates and the manual quickstart.

**Rationale**: These are deterministic gameplay rules. Screenshot tests add little value while existing projectile, impact, boom, and crater tests cover the unchanged downstream path.
