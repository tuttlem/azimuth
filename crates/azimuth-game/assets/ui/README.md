# Azimuth UI assets

The weapon icons in `weapons/` are original project-created artwork generated for Azimuth's UI
polish feature. They have transparent backgrounds and are intentionally a single consistent
arcade-artillery illustration set. `WeaponId` metadata in `src/weapon.rs` is the authoritative
presentation lookup shared by the battlefield inventory and shop; do not add unrelated stock art
or duplicate per-screen icon mappings.

`../audio/ui-purchase.ogg` is an original short 1.9 kHz mechanically restrained purchase cue,
procedurally rendered for the project; it contains no third-party recording.
