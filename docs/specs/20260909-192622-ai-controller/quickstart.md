# Quickstart: Validate the First AI Controller

## Prerequisites

- Run from the repository root on branch `20260909-192622-ai-controller`.
- Use the existing desktop-capable Azimuth environment.
- Review [AI controller contract](contracts/ai-controller.md) and [data model](data-model.md) for
  expected ownership and state transitions.

## Automated validation

1. Run focused AI, match setup, turn, weapon, and shared-action tests:
   ```sh
   cargo test -p azimuth-game
   ```
2. Run required repository quality checks:
   ```sh
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo build --workspace
   ```
3. Confirm deterministic decision tests cover equal seed/state repetition and display-name-only
   changes, plus 2-, 4-, and 8-player Human/AI mixtures.
4. Confirm shared-launch tests show both decision sources reach the ordinary projectile lifecycle
   and normal limited-ammunition behavior.

## Manual acceptance

1. Run the game:
   ```sh
   cargo run -p azimuth-game
   ```
2. Configure Michael as Human and Crater Kate as AI. Start the match; verify setup no longer blocks
   AI. Complete Michael's turn and do not use tactical input during Kate's turn. Verify her active
   name, plausible target-directed aim, ordinary shot, ordinary wind/terrain interaction, and
   normal handoff.
3. Configure Michael plus four AI players. Verify consecutive AI turns, correct full scoreboard,
   skipped eliminated players, understandable camera transitions, and eventual ordinary result.
4. Configure all eight players as AI. Start and observe without tactical input. Verify every initial
   participant gets an ordinary visible turn, AI-to-AI handoff continues after impacts/settling, and
   the match does not wait for a Human. Stop only after enough turns to assess plausible misses and
   occasional useful shots; do not claim completion until a match reaches a normal winner or draw.
5. Run an all-Human match as a regression: aiming, selection, movement, fire, projectile behavior,
   terrain deformation, settling, elimination, and victory remain unchanged.

## Expected qualitative outcome

AI shots broadly face a living opponent and scale roughly with distance, but wind and terrain cause
misses. The AI must feel like another imperfect player using Azimuth's ordinary rules, not a
scripted or perfect shot generator.

