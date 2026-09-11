# Game Flow Contract

## Authoritative operations

- apply_damage(owner, impact) returns actual applied results.
- finalise_round(result) awards accounting once; winner bonus applies only to a winner.
- purchase(player, weapon) validates a priced limited weapon and funds, deducting price and adding one round atomically.
- complete_shop(player) advances sequential human shopping; AI completion is immediate and bounded.
- start_next_round preserves session fields and replaces every round field.
- end_game discards session and returns to setup.

## UI contract

Result, accounting, and shop screens project current phase/state. Buttons request these operations; they never mutate cash or ammunition directly. Combat controls are accepted only in Playing.

