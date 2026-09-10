# MIRV Research and Decisions

## Shot ownership

**Decision**: Replace the one-projectile `FiredShot` shape with one shot owning an ordered small collection of active projectile records.

**Rationale**: Current fixed advance makes `None` after a single outcome. A collection directly expresses a firing turn that continues after individual children resolve; conventional shots remain a one-record case.

**Alternatives considered**: A parallel MIRV resource duplicates authority; a programmable projectile graph is premature; a global unowned list loses turn/inventory membership.

## Split timing and spread

**Decision**: Advance the carrier normally, then split a surviving carrier once when its post-step vertical velocity is non-positive. Five indexed children copy its state and receive fixed authored forward/right-axis impulses; they advance next fixed step.

**Rationale**: This is a visible apex rule, keeps existing ballistics authoritative, gives deterministic 3D coverage, and lets impact/bounds win if they occur on the transition step.

**Alternatives considered**: Configurable time/height and predicted flight fraction add unneeded controls/estimation; random or enormous scatter weakens reproducibility and aiming.

## Resolution order

**Decision**: Process active records strictly by stable child index against mutable terrain; gate handoff on both empty shot and no settling tank.

**Rationale**: Later same-step children observe earlier craters. Current per-impact completion must be guarded because it would otherwise finish a turn while siblings fly.

## Presentation

**Decision**: Expose read-only carrier/child/aggregate-region snapshots and an ordered resolved-impact event stream. Coalesce pulses and attenuate/cap close child impact audio.

**Rationale**: Current presentation observes one projectile and one latest-impact slot, so it can lose multiple fixed-step impacts and restart flashes. Events preserve each presentation consequence without giving it authority; aggregate framing prevents child-to-child snaps.

## AI

**Decision**: Retain the AI's Basic-Shell decision and test it remains safe with MIRV inventory.

**Rationale**: Shared firing remains functional; MIRV tactical selection is explicitly deferred.
