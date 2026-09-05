# Data Model: Rendering Technology Selection

This feature creates no game-domain data model, persistent state, or gameplay entities. The only
runtime objects are presentation-local proof-scene objects owned by the selected technology.

| Presentation object | Responsibility | Boundary and validation |
|---|---|---|
| Application | Owns the desktop lifecycle and normal close behaviour. | Lives in `azimuth-game`; is not a game-domain application model. |
| Fixed camera | Views the proof scene with perspective projection. | Has no player or gameplay-control meaning. |
| Ground visual | Makes orientation and depth visible. | Is a flat rendered primitive, not battlefield terrain. |
| Geometry visual | Provides a second depth-visible object. | Is not a tank, projectile, or gameplay entity. |
| Light | Makes the proof scene readable where required. | Has no environmental or gameplay meaning. |

There are no relationships, validation rules, state transitions, external contracts, or persisted
records beyond the application lifecycle. Future terrain, physics, coordinates, entities, and
simulation state remain separate features.
