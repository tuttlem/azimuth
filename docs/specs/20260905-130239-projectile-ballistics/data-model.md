# Data Model: First Projectile and Deterministic Ballistic Arc

The model has no persistence and no external service. It introduces only the concrete values needed
to describe, simulate, and visualise one in-flight projectile.

## Shared world values

| Entity | Fields | Rules |
| --- | --- | --- |
| World position | `x`, `y`, `z` | Abstract game-space point; Y is upward. Used by the existing tank firing origin and projectile position. |
| World vector | `x`, `y`, `z` | Abstract game-space displacement, velocity, or acceleration; it is not a presentation vector. |

## Shot parameters

| Field | Meaning | Validation |
| --- | --- | --- |
| launch position | Initial world point | All components are finite; obtained from the selected tank firing origin for the development shot. |
| azimuth degrees | Horizontal direction | Finite; normalized to 0 through less than 360. Zero is negative Z and positive rotation is clockwise from above. |
| elevation degrees | Upward launch angle | Finite and inclusive from 0 through 90. |
| launch speed | Initial speed | Finite and greater than zero. |

**Derived value**: launch velocity is the documented unit direction multiplied by launch speed.

## Gravity

| Field | Meaning | Validation |
| --- | --- | --- |
| downward acceleration | Positive magnitude of Y-down acceleration | Finite and non-negative. Zero is valid. |

Gravity derives the per-step acceleration vector `(0, -magnitude, 0)`; it contains no Earth
meaning and no direction variation.

## Projectile flight

| Field | Meaning | Rules |
| --- | --- | --- |
| position | Current world point | Starts at shot launch position and changes only through a fixed domain step. |
| velocity | Current world velocity | Starts as derived launch velocity; X/Z remain unchanged by gravity-only flight. |
| elapsed time | Simulated flight duration | Starts at zero and increases by exactly 1/120 second per advancement. |

An application-level optional holder represents whether the single projectile is active. Absence
means no in-flight projectile; a separate active boolean is unnecessary.

## Simulation limits

| Field | Value | Meaning |
| --- | --- | --- |
| horizontal extent | 60 units on X and Z | Flight ends when either horizontal coordinate is outside this range. |
| vertical minimum | -30 units | Flight ends below this value. |
| vertical maximum | 100 units | Flight ends above this value. |
| maximum duration | 20 simulated seconds | Flight ends at or after this duration. |

## State transitions

```text
No flight --Space with valid fixed shot--> In flight
In flight --fixed step within limits--> In flight
In flight --fixed step beyond a limit--> No flight
In flight --Space--> In flight (ignored request; state unchanged)
```

Terrain is not part of any transition. A projectile may cross the terrain surface and remain in
flight until a simulation-volume condition ends it.

## Relationships

- Player One's existing tank supplies the development shot's firing origin and turret direction.
- The turret's horizontal direction maps to the documented azimuth before the fixed elevation and
  speed are applied.
- The pure projectile flight is authoritative; the one rendered sphere copies its position but
  does not own velocity or advance itself.
