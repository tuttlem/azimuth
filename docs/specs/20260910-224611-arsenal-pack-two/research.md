# Research: Arsenal Pack #2

## Terrain addition and Nuke

**Decision**: Add a validated mound deformation beside the existing crater deformation and route
Dirt Bomb through it; give Nuke a large ordinary impact profile.

**Rationale**: The authoritative terrain is already one bounded mutable height field. Its rendered
mesh, height query, projectile collision, tank support, and later deformation all consume the same
surface. A radial upward counterpart to the crater is deterministic and automatically composes with
those systems. Nuke needs no separate damage, terrain, or winner model: a large bounded profile
uses the current radial damage, crater, reconciliation, settling, and survivor flow.

**Alternatives considered**: Soil conservation, terrain-body physics, special burial death, and a
separate nuclear simulation were rejected as unearned. Current tank support raises a tank whose base
terrain rises, so Dirt Bomb can surround, obstruct, and create steep terrain around a tank but does
not promise literal body-volume entombment; clearance/trapped mechanics remain future work.

## Curve Ball

**Decision**: Capture a left/right choice at fire commit and apply one fixed lateral acceleration
relative to the original horizontal launch orientation for the shot's normal flight.

**Rationale**: The existing fixed-step integrator already composes gravity and wind. A narrow extra
acceleration parameter preserves all current trajectories when zero, keeps curve independent of
tanks, and makes left/right fixtures naturally opposite and repeatable.

**Alternatives considered**: A world-axis curve would be unintuitive; deriving side from changing
velocity would let wind alter intended handedness; target steering and a solver violate the feature.

## Bouncer

**Decision**: Add an exact current-terrain surface-normal query and reflect the retained impact
velocity with a fixed energy loss and three-bounce budget; offset the projectile slightly outward
after each bounce.

**Rationale**: Terrain collision already refines contact deterministically and retains candidate
velocity. The piecewise terrain triangle normal makes real 3D mountain/valley bank shots possible.
The outward offset prevents same-surface recontact; a bounded counter prevents endless ricochet.

**Alternatives considered**: World-up bounce and 2D slope bounce would lose terrain orientation;
scripted trajectories would not be genuine bank shots; a full rigid-body engine is unnecessary.

## Strip, Curve control, camera, feedback, and AI

**Decision**: Extend the stable click-first strip to twelve compact boxes, and show two mouse-click
curve-direction controls only while Curve Ball is selected. Keep existing keys as optional legacy
shortcuts rather than assigning more. Let Human tracking observe current projectile velocity; widen
only Nuke's presentation aftermath; reuse low-key feedback for Bouncer intermediate contacts.

**Rationale**: The strip already projects authoritative per-player availability and selection. A
captured curve choice prevents later UI changes affecting a fired shot. Existing AI always chooses
Basic Shell, so no strategy work is necessary; any future Curve AI inherits a safe default.

**Alternatives considered**: Keyboard-only curve controls conflict with the click-first direction;
pages/categories are premature at twelve desktop slots; full impact flashes for every bounce would
miscommunicate final detonation and harm comfort.
