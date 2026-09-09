# Quickstart: Validate Impact Screen Flash

Run `cargo test -p azimuth-game` and `cargo run --package azimuth-game` from the repository root.

| Scenario | Expected result |
|---|---|
| Basic terrain impact | One noticeable, rapid restrained white pulse |
| High Explosive impact | Modestly stronger/longer pulse, still comfortable |
| Multi-tank splash | One pulse only |
| Consecutive AI impacts | No strobing or persistent wash |
| Out of bounds | No pulse |

Confirm impact, crater, damage, settling, winner/draw, and next turn remain unchanged. Update the exaggerated-presentation roadmap item only after manual acceptance.
