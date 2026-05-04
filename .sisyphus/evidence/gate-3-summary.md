# Gate 3 Evidence Summary

**Date**: 2026-05-03
**Version**: 0.2.0
**Status**: CLOSED

## Pass Criteria Review

| Criterion | Status | Evidence |
|-----------|--------|----------|
| 1. Scrape exciter core | PASS | `src/dsp/exciters/scrape.rs`, `tests/scrape_exciter.rs` |
| 2. Scrape flavors (3 presets) | PASS | `presets/factory/scrape_*.corrosion-preset` |
| 3. Chain object profile | PASS | `src/dsp/profile.rs`, `tests/chain_distinct.rs` |
| 4. Stereo modal spread + Width | PASS | `src/dsp/resonator.rs`, `tests/stereo_width.rs` |
| 5. Lightweight body resonator | PASS | `src/dsp/body.rs`, `tests/body.rs` |
| 6. Roughness/rattle character | PASS | `src/voice/mod.rs`, `tests/damage_rattle.rs` |
| 7. Saturation character (Drive) | PASS | `src/lib.rs`, `tests/drive_dynamics.rs` |
| 8. Velocity expressiveness | PASS | `src/voice/mod.rs`, `tests/velocity_expressivity.rs` |
| 9. 20 additional presets | PASS | `presets/factory/*.corrosion-preset` (43 files) |
| 10. Automation stress test | PASS | `tests/automation_stress.rs` |
| 11. Regression vs Gate 2 | PASS | `tests/plugin_metrics.rs`, all Gate 2 tests pass |
| 12. Gate 3 summary | PASS | This file |

## Evidence Files

- `task-G3-1-scrape.log` — scrape exciter test results
- `task-G3-2-flavors.log` — scrape flavor preset validation
- `task-G3-3-chain.log` — chain distinctiveness test
- `task-G3-4-width.log` — stereo width test
- `task-G3-5-body.log` — body resonator test
- `task-G3-6-rattle.log` — damage rattle test
- `task-G3-7-drive.log` — drive dynamics test
- `task-G3-8-velocity.log` — velocity expressivity test
- `task-G3-9-presets.log` — preset count and coverage
- `task-G3-10-stress.log` — automation stress test
- `task-G3-11-regression.log` — Gate 2 regression suite

## Test Results

- Unit tests: 53 passed
- Integration tests: 30 passed
  - automation_stress: 2
  - body: 3
  - chain_distinct: 1
  - damage_rattle: 3
  - drive_dynamics: 3
  - limiter: 2
  - no_alloc: 1
  - plugin_metrics: 4
  - preset_roundtrip: 1
  - scrape_exciter: 2
  - stereo_width: 2
  - velocity_brightness: 4
  - velocity_expressivity: 2
- **Total: 83 tests passed**

## Preset Inventory

Total factory presets: 43

| Category | Count | Examples |
|----------|-------|----------|
| Scrape-focused | 5 | `scrape_metal`, `scrape_bowed_steel`, `scrape_brake_squeal`, `scrape_tension_rise`, `brake_drag` |
| Chain-focused | 5 | `chain_gang`, `chain_rattle`, `industrial_chain`, `link_clank`, `loose_links` |
| Drone-focused | 5 | `drone_pipe`, `deep_hum`, `eternal_ring`, `void_resonance`, `long_sustained_tone` |
| Transition-focused | 5 | `rise_impact`, `drop_clang`, `cinematic_hit`, `release_crash`, `tension_build` |
| Hit/Clang | 12 | `clang_*`, `short_*`, `bass_*` |
| Boom/Low | 6 | `boom_*`, `bass_depth_charge`, `bass_subterranean` |

## Build Artifacts

- Linux VST3: `target/bundled/Corrosion.vst3`
- Linux CLAP: `target/bundled/Corrosion.clap`
- Windows VST3: `target/bundled-win/Corrosion.vst3`
- Windows CLAP: `target/bundled-win/Corrosion.clap`

## Notes

- Scrape exciter uses stick-slip friction model with pressure/speed/roughness parameters.
- Chain profile has 10 modes with high inharmonicity and shorter individual decays.
- Stereo width uses deterministic per-mode panning based on frequency and mode index.
- Body resonator is a 4-mode broad resonance bank (220Hz, 380Hz, 550Hz, 720Hz).
- Drive uses 3-stage asymmetric waveshaper (linear / soft-clip / exponential compression).
- Damage rattle is signal-dependent high-pass noise scaled by damage amount.
- Velocity modulates excitation decay rate and damage rattle depth.

## GATE 3 STATUS: CLOSED
