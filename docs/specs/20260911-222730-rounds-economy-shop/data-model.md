# Data Model

## GameSession

Players keyed by PlayerId, each holding stable PlayerConfiguration identity, non-negative cash, finite loadout, and wins. Session also owns positive round number, phase, and RoundAccounting.

## Round State

Existing terrain, tanks/health, turns, aiming, projectiles, result, and temporary presentation stay transient and are replaced for a next round.

## AppliedDamage

Authoritative target PlayerId, actual health removed, and newly-eliminated flag. Rewards require target different from shot owner.

## RoundAccounting

Per player: opponent damage reward, elimination bonus, winner bonus, total earnings, resulting cash. Finalisation is idempotent; draws receive no winner bonus.

## Economy and Shop

Damage rate $10 per applied opponent health; elimination $250; winner $500. Each limited weapon has one price. A purchase is exactly one round or no state change. ShopProgress holds configured-order active human/readiness; AI completes immediately.

## State Transitions

Setup to Playing to Celebrating to Accounting to Shopping to Transition to Playing. END GAME from accounting/shop returns to Setup and discards session. Only Playing accepts combat actions.

