# Presentation Contracts

## Scoreboard Projection Contract

**Inputs**: ordered configured players; current tanks keyed by stable player owner; current turn and
match state.

**Output**: an ordered collection of player-summary entries.

| Requirement | Contract |
|---|---|
| Cardinality | Output length exactly equals the configured-player count, which is valid only from 2 through 8. |
| Order | Output order exactly matches `MatchConfiguration.players`; tank storage order cannot change presentation order. |
| Identity/name | Each output identity is the configured stable ID and each name is that configured player’s display name. |
| Condition | Health and eliminated state exactly reflect the matching current tank. |
| Active state | Exactly the current in-progress player is active; resolving and completed matches produce no active entry. |
| Authority | Projection is pure/read-only and has no API that mutates configuration, tanks, turn, terrain, weapons, or input. |

## Visual World Contract

| Requirement | Contract |
|---|---|
| Input | The horizon may read the existing authoritative terrain only to form its initial visual seam and may read the captured terrain seed to colour/elevate its exterior. |
| Output | It provides render-only geometry/appearance data and static background/cloud presentation. |
| Separation | It does not expose an authoritative terrain-height, collision, movement, spawning, projectile, damage, or crater operation. |
| Bounds | `HALF_EXTENT` and all existing authoritative in-bounds decisions remain unchanged; outside queries still reject/return absent under the existing contract. |
| Updates | Authoritative terrain mesh refresh follows deformation as today. Horizon does not receive live crater/deformation updates. |
| Camera | Existing controls and intents remain the contract; any adjustment is a narrow visibility safeguard, not a new control mode. |

## Validation Contract

- Automated checks assert scoreboard 2/3/8 cardinality, configured order/names, live health,
  eliminated state, and active-state rules without rendering pixels.
- Automated checks assert horizon descriptor initialization is finite/valid, joins its initial inner
  perimeter to sampled authoritative edge data, contains no interior presentation vertices except
  the shared boundary, and leaves the authoritative bounds/query contract unchanged.
- Manual checks cover the visual qualities a pure test cannot prove: 2/4/8 readability, simple sky
  and clouds, no normal-view void/floating board, non-distracting seam, camera safety, and healthy
  responsiveness.
