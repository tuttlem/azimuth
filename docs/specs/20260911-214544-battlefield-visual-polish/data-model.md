# Data Model: Battlefield Visual Polish

## Authority Boundary

Existing `Tank`, terrain, weapon impact profile, projectile, wind, and turn state remain
authoritative. This data is renderer-owned and must never be read by gameplay resolution.

## Cosmetic Tank Model

| Field | Meaning | Validation |
|---|---|---|
| model kind | Classic, Heavy, Low Profile, Compact, or Angular | Exactly one of five fixed recipes. |
| hull recipe | Primitive parts/local transforms for armour and tracks | Readable hull and two track-side forms. |
| turret recipe | Primitive parts/local transform under root | Retains existing turret ownership/aim role. |
| barrel recipe | Primitive parts/local transform under turret | Visually meets canonical muzzle marker. |

Relationship: one model is assigned to each configured `PlayerId`; players may share a model only
after all models have been used where possible.

## Tank Visual Assignment

| Field | Meaning | Validation |
|---|---|---|
| player ID | Existing stable gameplay identity | Exactly one assignment per configured player. |
| model kind | Cosmetic model for that player | Valid recipe; no gameplay fields. |
| cosmetic derivation | Isolated deterministic assignment input | Independent of terrain, starts, wind, and AI streams. |

Lifecycle: create at initial scene/match creation, replace on match restart, discard with old
visual scene. It never alters the `Tanks` resource.

## Tank Presentation Assets

| Asset group | Meaning | Validation |
|---|---|---|
| shared primitive meshes | Reusable hull, track, turret, barrel, and muzzle-compatible forms | Handles reused across instances. |
| player armour materials | Painted player-colour material set | One visual identity per existing player colour. |
| shared mechanical materials | Dark tracks, barrel, and fittings | Reused; no unnecessary per-part allocation. |

## Battlefield Surface Treatment

| Field | Meaning | Validation |
|---|---|---|
| height-derived base colour | Existing live terrain-height presentation | Recomputed after crater/mound refresh; does not change height. |
| subtle mottle/UV data | Optional visual-only world-scale variation | Deterministic and unused by gameplay. |
| water material | Opaque blue, lower-roughness appearance | Separate from terrain authority; no water gameplay state. |

Current authoritative terrain supplies positions, normals, and base colour; the treatment changes
only how that existing surface is drawn.

## Impact Presentation Request

| Field | Meaning | Validation |
|---|---|---|
| impact position | Resolved terrain-impact location copied for display | Finite; captured after resolution. |
| visual scale | Existing impact profile magnitude | Positive finite scale drives bounded budget. |
| cosmetic serial | Presentation-local sequence value | Used only for stable cosmetic variation. |

Lifecycle: append after every ordinary/split-child terrain impact; drain in presentation Update;
retain no more than 64 pending requests. A cap may reduce cosmetic density but never suppresses the
underlying impact.

## Impact Particle

| Field | Meaning | Validation |
|---|---|---|
| kind | Hot fragment or dirt/debris appearance | Fixed shared material palette. |
| visual velocity | Outward/upward renderer-only motion | Finite; never a projectile/collider. |
| age and lifetime | Finite visual lifecycle | Despawn at or after lifetime. |
| scale/fade | Current visual size/opacity | Non-negative and trends to invisible. |

Limits: at most 32 spawned for an impact and 128 active in the scene.

## Smoke Puff

| Field | Meaning | Validation |
|---|---|---|
| drift/rise | Cosmetic velocity, optional read-only wind | Finite; no wind write-back. |
| age and lifetime | Temporary aftermath lifecycle | Despawn at or after a few seconds. |
| initial/current scale | Expanding plume appearance | Nuke produces a greater bounded plume. |
| translucency/fade | Readable non-opaque presentation | Trends to invisible before despawn. |

Limits: at most eight puffs for an impact and 32 active in the scene.
