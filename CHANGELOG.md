# Changelog

All notable changes to Corrosion during the production-hardening effort are documented in this file.

## [Unreleased] — Production Hardening

### UI — Industrial egui Rewrite

- Complete egui editor overhaul with industrial design tokens (brushed metal, dark panels, hazard stripes)
- Custom knob widget with radial arc, tick marks, and value readout
- Custom fader widget with channel-style gain staging
- ADSR and MSEG envelope visualizers
- Metal panel sections with screw decorations
- UI scaling support (50%, 75%, 100%, 125%, 150%)
- Header layout with title, logo placeholder, and output meter
- Parameter grouping by physical function (Exciter, Object, Transform, Post, Space)

### Fixed

- **Knob drag instability** — Replaced unstable drag tracking with `response.id`-based state and `knob_drag_value()` helper. Added regression tests for monotonicity and clamping.
- **README broken links** — Updated doc references to point to existing files in `docs/`.
- **Missing render CLI** — Restored `src/bin/render.rs` for deterministic offline QA.

### Infrastructure

- Added `.github/workflows/ci.yml` with fmt, clippy, tests, render smoke, WAV validation, and Linux bundle steps.
- Added `scripts/verify-local.sh` as a one-shot local verification mirror of CI.
- Added `scripts/validate-plugins.sh` wrapper for `pluginval` and `clap-validator` with graceful skip behavior.

### Code Quality

- Fixed all strict clippy blockers (`-D warnings`) across workspace.
- Added targeted `#[allow(clippy::too_many_arguments)]` with explanatory comments on voice and resonator APIs.
- Eliminated manual clamp fallbacks in favor of `clamp()`.
- Fixed floating-point precision lints and loop patterns.

### Testing

- Added `tests/automation_stress.rs` with rapid parameter sweep coverage.
- Added `tests/preset_roundtrip.rs` with invalid-value sanitization test.
- Added `tests/no_alloc.rs` to prove real-time safety.
- Added `voice::manager::tests::non_finite_peak_hold_is_preferred_for_stealing` to guard against NaN panic during voice steal.
- Added `dsp::exciters::wire_brush::tests::dense_cluster_stays_finite_and_finishes` for stochastic exciter validation.
- Expanded automation stress to full 16×9 exciter/object matrix (`exciter_and_object_matrix_stays_finite`).

### Benchmarks

- Added `benches/performance.rs` Criterion harness with 5 benchmarks:
  - `voice_manager_single_voice`
  - `voice_manager_eight_voices`
  - `wire_brush_dense_cluster`
  - `post_processing_chain`
  - `offline_renderer_pipe_family`

### Performance

- **WireBrush cursor optimization** — Replaced O(N) per-sample impulse scan with monotonic cursor over pre-sorted schedule. Improved `wire_brush_dense_cluster` from millisecond-scale to microsecond-scale.

### State Hardening

- `Preset::sanitized()` now loads values through live parameter ranges instead of trusting raw JSON.
- Invalid out-of-range preset fields are clamped on load.
- Host state save/load (`get_state` / `load_state`) uses the same preset path.

### Validation

- `pluginval` passes at strictness level 5 against the bundled VST3.
- `clap-validator` passes with 18/21 tests passed, 0 failed, 3 skipped.

### Quality Modes

- Added `QualityMode` parameter (Eco/Normal/High/Render) controlling post-processing fidelity.
- Eco mode bypasses body resonance, stereo spread, and space effects; uses 1× clipper oversampling.
- Normal mode uses 4× oversampling and full chain (new default).
- High mode uses 8× oversampling; Render mode uses 16× oversampling.
- Preset backward compatibility: missing `quality_mode` defaults to Normal.

### Documentation

- Added `docs/production-hardening-log.md` — phase-by-phase evidence with verification commands and results.
- Added `docs/ARCHITECTURE.md` — signal chain, module map, real-time contract, and file organization.
- Added `docs/KNOWN_LIMITATIONS.md` — honest assessment of current gaps.
- Added `CHANGELOG.md` — this file.

## [0.1.0] — Initial Release

- VST3 and CLAP plugin entrypoints via NIH-plug
- 8-voice polyphony with deterministic stealing
- 16 exciter types (impact, scrape, specialty)
- 9 resonator objects (pipe, plate, tank, chain, etc.)
- Modal synthesis engine with size/rust/damage/thickness/heat/sludge transforms
- Post-processing: WDF filter, drive, body, stereo spread, space, oversampled clipper
- Basic egui editor
- Offline renderer for comparison suites
- Preset save/load (JSON)
