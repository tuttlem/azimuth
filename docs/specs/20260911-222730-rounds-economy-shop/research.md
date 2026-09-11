# Research: Continuing Game Loop

## Decision: Session state is independent of round tanks

Use a session domain containing stable configured players, cash, finite loadouts, wins, round number, phase, and current accounting. Keep terrain, tanks, health, turns, projectiles, effects, and result as disposable round state.

**Rationale**: MatchConfiguration already has stable PlayerId/name/controller/colour; existing setup recreates the combat world.

**Alternatives considered**: Wallets on tanks were rejected because tanks are recreated. A terminology-only rename was rejected as churn.

## Decision: Account from applied damage and firing owner

Combat reports target, actual health removed, and newly-eliminated transition. Fired shots retain their owner. Reward only opponent applied damage.

**Rationale**: Existing damage saturation prevents overkill but attempted-damage values cannot support correct rewards. This naturally handles splash and multi-projectile weapons.

## Decision: Reuse existing setup rebuild for fresh rounds

Extract the setup Enter path to replace terrain, tanks, dressing, wind, turns, projectile/effect state, visuals, and HUD; seed every round through isolated labelled derivations and restore session loadouts.

## Decision: One phase resource and separate overlays

Use Setup, Playing, Celebrating, Accounting, Shopping, and Transition. Combat input and battle AI require Playing. Result camera uses surviving winner location; draws have no focus.

## Decision: Central prices and atomic purchase

Limited weapon price data lives with catalogue/economy configuration. Validated purchase deducts integer cash and adds one round; Basic Shell has no price.

## Decision: Bounded deterministic AI shopping

Use a labelled shop seed separate from terrain, starts, and tactical AI. AI makes a capped affordable valid purchase sequence or buys nothing and immediately completes.

