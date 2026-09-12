# Research: Between-Round Shop — Weapon Purchasing and Visual Inventory

## Decision: extend the existing session Shop phase rather than create a second flow

**Rationale**: The current lifecycle already has `Celebrating → Accounting → Shopping → Transition`, and fresh-round creation is guarded by `Transition`. Completing that seam ensures the battlefield does not recreate early and keeps between-round behavior in session state.

**Alternatives considered**: A separate shop screen that creates rounds directly would duplicate the transition owner; a battlefield-owned overlay would make session preparation depend on discarded round state.

## Decision: use one authoritative weapon-backed Shop Item catalogue

**Rationale**: Weapon IDs, prices, ammunition rules, and `GameSession::purchase` already define the legal current transaction. A compact item records category, stable identity, presentation identity, price, and effect while delegating actual purchase to session authority. This allows a later Armour item to use another effect without making every item a weapon.

**Alternatives considered**: Hand-written UI product lists drift from the weapon catalogue; a trait/plugin commerce framework exceeds the one known future category; adding Armour now would decide out-of-scope rules.

## Decision: retain `GameSession::purchase` as the atomic weapon transaction

**Rationale**: It checks price and cash, adds ammunition, and deducts cash only after the ammunition operation succeeds. Shop UI and AI invoke it rather than changing wallet and loadout separately.

**Alternatives considered**: UI-side mutations would risk partial updates; a new parallel weapon purchase function would create competing authority.

## Decision: process shop turns in configured order and auto-resolve AI turns

**Rationale**: Configured order is stable for all 2–8 player counts, gives humans a single clear wallet context, and includes eliminated or cashless players. AI can make at most a small fixed number of deterministic/seeded affordable purchases, then mark ready immediately.

**Alternatives considered**: Showing all wallets at once harms readability; skipping zero-cash or eliminated players violates the loop; sophisticated economic AI is unrelated scope.

## Decision: use a shared presentation lookup with procedural/project-owned icon shapes

**Rationale**: Existing UI is constructed from simple built-in shapes, colors, text, and buttons and the repository has no icon asset library. A presentation lookup keyed by `WeaponId` can provide full and compact names plus a simple visual recipe for both contexts without leaking UI types into combat rules or adding external licensing obligations.

**Alternatives considered**: External stock icons create provenance/style concerns; separate shop/HUD icon functions drift; icon-only cards harm learnability; a heavyweight texture pipeline is not justified.

## Decision: use a compact wrapping grid before adding navigation

**Rationale**: The current catalogue has ten purchasable cards and the battlefield strip eleven cards. Existing UI supports fixed-size button layouts; compact cards with a wrapping/grid arrangement should satisfy the current window and preserves a simple later path to scrolling only if manual validation proves it necessary.

**Alternatives considered**: Weapon wheels and elaborate category navigation add unrelated interaction complexity; a spreadsheet list conflicts with the visual shop goal.
