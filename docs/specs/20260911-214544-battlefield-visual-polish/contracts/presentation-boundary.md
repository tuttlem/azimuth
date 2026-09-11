# Presentation Boundary Contract

## Purpose

Define the one-way relationship between deterministic artillery gameplay and this visual-polish
feature. This is an internal application contract, not a network/public API.

## Authoritative Inputs Observed by Presentation

| Source | Presentation may read | Presentation must not change |
|---|---|---|
| Tank firing representation | Pose, turret direction, canonical muzzle | Position, support, health, owner, aim, launch calculation |
| Match configuration | Ordered player IDs and established colours | Identity, controller, configuration, turn order |
| Terrain | Current mesh positions, normals, height-derived colour | Query, height, crater/mound, collision/bounds |
| Impact result | Resolved position and visual scale | Damage, deformation, support, resolution |
| Wind | Read-only direction/strength for smoke flavour | Wind resource/projectile response |

## Tank Contract

1. One cosmetic assignment exists per configured player at match creation.
2. Assignment derives separately from authoritative RNG and does not become a `Tank` field.
3. All recipes retain visual root, turret, barrel, and muzzle roles.
4. Canonical firing-representation muzzle is the projectile launch point for every recipe; reshaped
   visual barrels never change it.
5. Player colour remains visible on armour; shared dark material is for tracks, barrel, and
   mechanical detail.

## Impact and Smoke Contract

1. Each resolved terrain impact may append one renderer-owned request only after authoritative
   damage/deformation/support work completes.
2. Consumption may create, animate, cap, fade, and remove presentation entities only.
3. Requests/effects are never inputs to collision, damage, terrain, visibility, AI, wind, or turns.
4. Split shots may append one request per child; cosmetic budgets never limit child resolution.
5. Temporary entities have finite life and are removed at expiry and match-scene reset.

## Disablement Contract

Disabling tank assignment, enriched materials, particles, or smoke must leave the same gameplay
seed and inputs with identical trajectory, impact, damage, terrain, settling, AI, weapon, and turn
outcomes. Tests compare authoritative state rather than pixels.
