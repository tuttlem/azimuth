# Data Model: Shot Presentation Camera

The feature adds presentation-owned in-memory state only. Existing projectile, terrain, tank, weapon, and turn data remain authoritative read-only inputs.

## Shot Presentation

| Field | Meaning | Validation |
|---|---|---|
| firing player | Stable player that committed this shot | Captured at shared launch; must be configured |
| controller type | Human or AI selection for this shot | Captured from player configuration; never from name/input |
| phase | Current presentation phase | Uses only valid transitions below |
| last projectile | Read-only position/velocity snapshot | Updated only from live finite projectile state |
| apex observed | Human flight crossed from rising to descending | Starts false, becomes true at most once |
| impact context | Optional impact point and presentation hold | Exists only after authoritative terrain impact |

### Phase transitions

    PlayerView(active player)
      -> HumanShotFollow        (Human launches)
      -> AiTacticalShot         (AI launches)
    HumanShotFollow
      -> widened HumanShotFollow (vertical velocity becomes non-positive)
      -> ImpactView             (terrain impact)
      -> PlayerView/ResultView  (out of bounds)
    AiTacticalShot
      -> ImpactView             (terrain impact)
      -> PlayerView/ResultView  (out of bounds)
    ImpactView
      -> PlayerView(next survivor) (hold ends; match continues)
      -> ResultView                (hold ends; winner/draw)

None of these transitions changes authoritative turn state.

## Camera intent

| Intent | Inputs | Outcome |
|---|---|---|
| Player view | Active player, retained aim, tank pose | Existing active-player framing |
| Human shot follow | Presentation context, projectile, terrain/tanks | Offset follow, apex widening, descent emphasis |
| AI tactical shot | Presentation context, shooter, projectile, terrain/tanks | Broad battlefield awareness |
| Impact view | Impact context, terrain, nearby/affected tanks | Explosion, crater, and aftermath framing |
| Result view | Winner/draw and last impact where available | Result presentation; never next player |

## Impact Context

| Field | Meaning | Validation |
|---|---|---|
| impact position | Authoritative terrain-impact point | Finite; target is clamped to bounds |
| nearby/affected tanks | Read-only pose, health, eliminated/settling state | Empty set is valid |
| match status | Current authoritative in-progress/winner/draw state | Final means no PlayerView transition |
| hold progress | Presentation elapsed aftermath time | Nonnegative; cannot gate resolution |

## Invariants

- Exactly one ShotPresentation may track an active shot because gameplay allows one projectile.
- Firing player/controller are immutable after capture.
- Human and AI phases converge to one ImpactView.
- ImpactView may survive after flight is removed, but cannot recreate/mutate projectile.
- Existing HUD remains separately derived and read-only.
- Named tuning constants cover Human offset/look-ahead/apex breadth, AI breadth, impact breadth/hold, and existing safety limits.
