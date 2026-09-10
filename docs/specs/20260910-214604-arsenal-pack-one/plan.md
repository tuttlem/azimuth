# Implementation Plan: Arsenal Pack #1

**Branch**: `20260910-214604-arsenal-pack-one` | **Date**: 2026-09-10 | **Spec**: [spec.md](spec.md)

## Summary

Extend committed-shot authority with earned deterministic carrier deployment, bounded terrain-contact rolling, and bounded deferred penetration. Existing ballistic motion, wind, terrain, explosion, deformation, settling, and handoff remain shared; presentation observes only.

## Technical Context

Rust stable; existing Bevy desktop game; deterministic in-memory state; Cargo tests/manual native review/fmt/Clippy. No dependencies, generic graph, rigid-body, or underground engine. Existing AI remains Basic Shell.

## Constitution Check

**PASS before and after design.** Four concrete weapons earn small deployment/contact vocabulary, compose existing systems, preserve deterministic order, and need no framework, dependency, crate, or exception.

## Project Structure

`crates/azimuth-game/src/{weapon,projectile,battlefield,main,ai}.rs`; `docs/{projectile-model,roadmap}.md`; feature documentation beneath this directory.

## Implementation Sequence

1. Generalise MIRV-only bounded children into behavior/state while preserving conventional shots.
2. Add dense Cluster and wide Net descending patterns and ordered resolution.
3. Add minimal deterministic terrain-slope sampling and bounded Roller movement.
4. Add Bunker Buster bounded impact-direction penetration and one internal impact.
5. Extend HUD/visual/camera/feedback, tests, documentation, and roadmap acceptance.
