# Research: Graphical Tactical HUD

## Decision: derive one pure view from existing gameplay state

**Rationale**: The existing HUD formatter already reads the complete presentation input tuple:
turn/match, two tanks, wind, and movement feedback. A pure derived view keeps the one-way
gameplay-to-presentation boundary, gives match-over precedence one explicit home, and can be
tested without UI entities or pixels.

**Alternatives considered**:

- HUD-owned health, aim, movement, or wind copies: rejected because they can become stale.
- Direct scattered UI reads: rejected because each panel would duplicate mapping rules.
- A reactive framework/event bus: rejected as needless infrastructure for one retained frame.

## Decision: retain one native UI tree with dynamic field markers

**Rationale**: Replace the one legacy text entity with one full-screen non-interactive root and a
small stable child tree. Markers identify title, health labels/fills, aim values, wind plot,
movement, controls, and result fields. A read-only sync system changes text, sizing, colour, and
visibility rather than recreating the HUD each update.

**Alternatives considered**:

- Rebuild every frame: needless entity churn.
- Add a widget library: no current benefit justifies another dependency.

## Decision: compact corner groups preserve the battlefield

**Rationale**: Top-left player/match and health, top-right shot/environment, and a small lower
contextual-hint group leave the centre open for tanks, arcs, and impacts. Ordinary anchored layout
and modest panel bounds support normal resizing without a responsive-UI framework.

## Decision: health bars retain exact numbers

**Rationale**: A proportional fill makes relative health instantly comparable; the existing numeric
value remains vital to tactical judgement. Existing player colours and a muted ASCII `OUT` state
need no new assets or effects.

## Decision: wind uses a geometric X/Z plot, not glyph arrows

**Rationale**: Unicode arrows rendered as boxes in the current font. A small top-down plot uses
two lines, ASCII `+X`/`+Z` labels, and a positioned marker from the normalized authoritative wind:
wind pushes **toward** the marker; +X is right and +Z is up. Strength remains numeric and calm is a
neutral origin marker. The plot is camera-independent.

## Decision: contextual hints are phase-specific

**Rationale**: Choosing shows compact action/aim controls, moving shows move/end controls, and
resolving or finished shows no dominant action hint. This replaces the persistent control dump and
cannot invent an unavailable action.

## Decision: static HUD updates only

**Rationale**: The tactical frame has no animation clock or callbacks. It observes existing game
and camera state, preserving independent input, fixed simulation, and presentation timing.
