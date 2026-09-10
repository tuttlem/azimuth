# Arsenal Pack #1 Data Model

| Entity | State | Invariant |
|---|---|---|
| Deployment carrier | projectile, behavior, pattern | Replaces itself once on deterministic descent. |
| Child pattern | ordered offsets/copied state | Stable order controls terrain and presentation order. |
| Roller | surface position, momentum, budget | Current terrain, bounded, one final impact. |
| Penetrator | contact, impact direction, depth | Finite displacement then one internal impact. |
| Fired shot | active behavior | Completes only when inactive and tanks settled. |
