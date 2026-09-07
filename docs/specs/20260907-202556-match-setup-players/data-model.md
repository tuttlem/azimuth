# Data Model: Match Setup — Named 2–8 Player Matches and Controller Slots

## Configuration boundary

MatchConfiguration is authoritative only before a match begins. It contains ordered PlayerConfiguration entries and is validated before it creates running state. It is not a Bevy widget resource disguised as domain data.

| Entity | Core fields/invariants | Lifecycle |
|---|---|---|
| PlayerId | Stable, unique internal identity; never derived from name, colour, turn, or collection position | Allocated for setup slot and retained into runtime |
| ControllerType | Explicit Human or Ai | Stored in configuration; AI blocks start until next feature |
| PlayerVisualIdentity | One of eight curated colour identities | Assigned per slot and retained through edits |
| PlayerConfiguration | ID, display name, controller type, visual identity | Mutable during setup; input to runtime creation |
| MatchConfiguration | Ordered 2–8 configurations; slot order is initial turn order | Default at launch; validated at Start |
| MatchConfigurationError | Invalid count, identity, name, visual/controller value, or unavailable AI reason | Presented by UI, owned by domain validation |
| AI-name helper | Curated pool; setup-local source; prefers unused names | Runs only on AI change/reroll, captures string in slot |

### Configuration invariants

- Count is inclusive 2 through 8; shrinking removes excess prospective slots.
- Human names are trimmed, non-empty, safely renderable, and at most 20 characters. Duplicates are legal.
- Human to AI preserves ID/colour and assigns a pool name. AI to Human preserves ID/colour and supplies a valid editable name.
- AI configuration cannot start and is never silently converted.
- Setup-name randomness never consumes gameplay simulation randomness.

## Running match state

The exact Bevy resource/component split follows existing integration, but relationships are identity-keyed collections rather than two-player fields/arrays.

| Entity | Core fields/invariants | Relationship |
|---|---|---|
| Participant collection | Ordered IDs and human-facing metadata | Spawn, turn order, HUD, survivors, result names |
| Tank collection | One supported tank per configured human, with health/position/orientation/movement/alive state | Combat, terrain deformation, and settling iterate all tanks |
| Per-player aim | Azimuth, elevation, power per ID | Active input writes only active ID |
| Player weapon loadouts | Standard independent inventory and selection per ID | Initialised for every participant; fired snapshots unchanged |
| Turn state | Ordered IDs, active ID, existing phase/action state | Finds next surviving ID; no other() |
| Match result | In progress, winning ID, or draw | Computed only after all shot consequences settle |

### Transitions

~~~text
Launch → default configuration → setup edit/validate → valid all-human configuration
      → player/tank/aim/loadout/turn initialisation → move or fire
      → projectile/impact → all damage + terrain + all settling
      → next living player | winner | draw
~~~

### Runtime invariants

- Each player has exactly one independent tank, aim, loadout, movement, and health state. Only ordinary shared shot consequences may affect others.
- Existing projectile and explosion profiles keep their common snapshot-driven pipeline; collision/effects iterate all tanks.
- Damage, terrain deformation, and settling all finish before next turn, victory, or draw.
- Camera/HUD derive state and never gate authoritative resolution.

## Ownership map

| Concern | Authoritative owner | Presentation role |
|---|---|---|
| Count, identity, name, controller, visual | Configuration validation | Edit/display values and errors |
| AI name selection | Setup helper plus slot | Controller choice and optional reroll |
| Start availability | Configuration validation | Disabled/enabled button and message |
| Spawn, health, aim, inventory, turn, elimination | Running collections | Render/translate legal active-human input |
| Camera/status layout | Derived active/running state | Animate/show only |
