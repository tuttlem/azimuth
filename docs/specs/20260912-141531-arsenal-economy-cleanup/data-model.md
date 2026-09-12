# Data Model: Arsenal and Economy Cleanup

Weapon catalogue has identity, ammunition, price, terrain, and health-damage behavior; Bomb Net is absent and Bunker Buster is terrain-only.

Round earnings contains `damage_income` and `placement_income`; total is their sum. Damage income is actual opponent health removed × $10.

Placement record derives player ordinal and award from survivor/elimination state. Same-resolution eliminations use configured-player order; draws have no first place.
