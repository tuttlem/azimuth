# Research: Azimuth UI Polish

## Decision: Use original, transparent weapon-icon assets keyed by stable weapon identity

**Rationale**: The current `WeaponPresentation` lookup provides a compact name and Unicode glyph for each `WeaponId`; both the shop and battlefield already consume it. Replace the glyph concept with a stable icon asset key/path (while keeping Bevy handles in presentation code) and create one coherent source-owned icon for every item in `ACTIVE_WEAPONS`. This supplies the requested visual quality without making weapon simulation depend on Bevy image types.

**Alternatives considered**:

- Keep Unicode symbols: rejected because they are explicitly placeholder-like and vary by font/platform.
- Use unrelated web/stock icons: rejected because provenance and visual consistency would be weak.
- Generate icons every frame from UI primitives: rejected because it is harder to tune artistically and creates unnecessary work.
- Build an icon-atlas pipeline: rejected as disproportionate for eleven static icons.

## Decision: Add a compact shared theme and local reusable UI helpers

**Rationale**: `main.rs` currently repeats literal colours, borders, font sizes, padding, and button construction across setup, shop, flow overlays, and HUD. A focused theme module/section and a few surface/button/card helper constructors will establish deliberate panel, accent, typography, spacing, and interaction rules without creating CSS-in-Rust. Screen-specific geometry remains local where it aids readability.

**Alternatives considered**:

- Restyle every screen independently: rejected because it recreates inconsistency.
- Adopt a generic component/design-system framework: rejected as unearned infrastructure.
- Centralise every Bevy `Node` property: rejected because it obscures layout intent and creates a faux stylesheet.

## Decision: Reuse Bevy `Interaction` for button feedback and update styles only on changes

**Rationale**: Existing shop and weapon selection buttons already use `Interaction` and `Changed<Interaction>`. Extend that path to apply normal, hover, pressed, selected, and disabled treatment consistently, and trigger a small press/purchase sound only for meaningful actions. Synchronise affordability, selected weapon, and exhausted state from authoritative resources; do not simulate UI-owned state or rebuild controls each frame.

**Alternatives considered**:

- Add a general animation state machine: rejected as outside the needed bounded pulse/highlight feedback.
- Play hover sounds continuously: rejected as noisy and contrary to the requested restraint.
- Let buttons mutate cash/loadouts directly: rejected by the presentation boundary.

## Decision: Make the current fixed HUD/shop layouts responsive through bounded flex layouts

**Rationale**: The current shop grid already uses `FlexWrap`, but fixed card/overlay sizes and the 768px weapon bar need a composition pass. Use Bevy's flex layout, bounded widths, compact cards/slots, and intentional gaps; add the smallest overflow behaviour only if the eleven current weapons do not remain legible at 1280×720. Verify representative 16:9 and wider desktop layouts manually rather than introducing a web-style responsive abstraction.

**Alternatives considered**:

- Shrink all cards/text until they always fit: rejected because usability is the goal.
- Add a weapon wheel or complex navigation: rejected as a gameplay/UI redesign with no present need.
- Support all possible displays/settings now: rejected as out of scope.

## Decision: Keep existing screens and flow state; improve their presentation in place

**Rationale**: Match setup, result/accounting overlay, shop overlay, and tactical HUD already correspond to the desired session lifecycle. Restyling them with shared panels, hierarchy, player accents, numeric formatting, clear calls to action, and winner celebration gives continuity with minimal risk. Winner presentation continues to leave the battlefield visible, and accounting continues to reflect existing award calculations.

**Alternatives considered**:

- Replace flow with a new screen/navigation system: rejected because it would risk round transition correctness and duplicate session ownership.
- Alter economy or AI behaviour to make the screens more interesting: rejected as unrelated gameplay work.

## Decision: Add only a restrained UI-audio extension to the existing crate-local audio collection

**Rationale**: Audio already loads through `AudioAssets` from `crates/azimuth-game/assets/audio`. A small original or clearly licensed press/purchase cue can be loaded once with the existing assets and emitted on successful actions. Artillery audio remains dominant.

**Alternatives considered**:

- Add a sound-theme engine or hover loops: rejected as unnecessary infrastructure/noise.
- Require a unique sound for every widget: rejected because it does not materially improve comprehension.
