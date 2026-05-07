# Production Hardening Log

## Phase 0 — Repository Truth Restoration

### Goal
Bring the repository's build targets, renderer entrypoints, and top-level documentation back into alignment with the actual codebase before deeper production-hardening work.

### Changes
- Restored the missing `render` CLI at `src/bin/render.rs` using the existing deterministic offline renderer in `src/offline/mod.rs`.
- Updated `README.md` so the docs section references files that actually exist in `docs/`.
- Updated `README.md` build/validation guidance to distinguish the local default musl target from the explicit gnu bundle flow used by `bundle.sh`.

### Why
- `Cargo.toml` declared a `render` binary that did not exist, breaking trust in the repo's build contract.
- `README.md` pointed to missing documentation files, making the current reference section unreliable.
- The repository default target and the documented bundle target were both valid, but not explained together, which made build expectations ambiguous.

### Verification
- `cargo test --workspace --no-default-features`
- `cargo test --lib --target x86_64-unknown-linux-gnu`
- `cargo run --target x86_64-unknown-linux-gnu --bin render -- --suite family --out-dir /tmp/corrosion-phase0-render`
- `python3 scripts/check_wav.py /tmp/corrosion-phase0-render/pipe_comparison.wav`

### Results
- `cargo test --workspace --no-default-features` passed.
- `cargo test --lib --target x86_64-unknown-linux-gnu` passed.
- `cargo run --target x86_64-unknown-linux-gnu --bin render -- --suite family --out-dir /tmp/corrosion-phase0-render` succeeded and produced:
  - `family_comparison_manifest.txt`
  - `pipe_comparison.wav`
  - `plate_comparison.wav`
  - `tank_comparison.wav`
  - matching summary files for each render
- `python3 scripts/check_wav.py /tmp/corrosion-phase0-render/pipe_comparison.wav` reported `peak=0.702759 rms=0.304205 nan_count=0 frames=48000 sr=48000`.

### Evidence To Capture Next
- Preserve the output manifest from the offline renderer run for later release QA comparison.

### Remaining Phase 0 Risks
- CI/workflow automation is still absent.
- Benchmark harness is still absent.
- The default target mismatch is now documented, but not yet structurally reconciled.

## Phase 1 — Repeatable Automation Gates

### Goal
Add a realistic repository automation lane for compile, lint, test, offline-render smoke, and Linux bundle validation without pretending that host-specific validators are universally available.

### Changes
- Added GitHub Actions workflow at `.github/workflows/ci.yml`.
- Added local verification mirror script at `scripts/verify-local.sh`.
- Updated `README.md` validation guidance to include the CI-equivalent manual flow and the local one-shot verification script.

### Why
- The repository had no CI/workflow automation despite already having a meaningful QA surface.
- The DAW smoke scripts and external validators are useful, but they are environment-dependent and should not block the first required gate.
- A local script reduces drift between developer verification and CI verification.

### Planned Verification
- `bash scripts/verify-local.sh`
- Review `.github/workflows/ci.yml` for target/tool alignment with the commands already proven in this environment.

### Remaining Phase 1 Risks
- External validators (`pluginval`, `clap-validator`) are still manual/optional.
- Windows bundle validation is still outside the required CI lane.

### Results
- `bash scripts/verify-local.sh` passed end to end.
- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets --no-default-features -- -D warnings` passed after targeted lint cleanup.
- `cargo test --workspace --no-default-features` passed.
- `cargo test --lib --target x86_64-unknown-linux-gnu` passed.
- `cargo run --target x86_64-unknown-linux-gnu --bin render -- --suite family --out-dir /tmp/corrosion-local-verify-render` succeeded.
- `python3 scripts/check_wav.py /tmp/corrosion-local-verify-render/pipe_comparison.wav` reported `peak=0.702759 rms=0.304205 nan_count=0 frames=48000 sr=48000`.
- `./bundle.sh release` succeeded and produced Linux VST3 and CLAP bundles under `target/bundled/`.

## Phase 3 — Benchmark Harness

### Goal
Make the documented `cargo bench` workflow real so performance work can be measured before any optimization/refactor claims.

### Changes
- Added Criterion as a development benchmark dependency.
- Added `benches/performance.rs` with focused benchmarks for:
  - single-voice manager rendering
  - eight-voice manager rendering
  - dense wire-brush excitation
  - full post-processing chain
  - deterministic offline resonator rendering
- Updated `README.md` to document the benchmark entrypoint.

### Why
- The repo and hardening brief both expected benchmark coverage, but no harness existed.
- These cases map directly onto the currently visible hot paths and release-relevant workloads.

### Planned Verification
- `cargo bench --no-default-features --target x86_64-unknown-linux-gnu --bench performance`

### Remaining Phase 3 Risks
- No baseline history or regression thresholds exist yet.
- Benchmarks are local/manual and not part of CI.

### Results
- `cargo check --no-default-features --target x86_64-unknown-linux-gnu --bench performance` passed.
- `cargo bench --no-default-features --target x86_64-unknown-linux-gnu --bench performance -- --noplot --sample-size 10` ran successfully.
- The benchmark harness produced initial timings for:
  - `voice_manager_single_voice`
  - `voice_manager_eight_voices`
  - `wire_brush_dense_cluster`
  - `post_processing_chain`
  - `offline_renderer_pipe_family`

## Phase 4 — First Measured Hot-Path Optimization

### Goal
Use the new benchmark harness to make one evidence-backed hot-path improvement without broad architecture churn.

### Changes
- Optimized `src/dsp/exciters/wire_brush.rs` by replacing the full per-sample scan of the impulse list with a monotonic cursor over the already time-sorted impulse schedule.
- Added `dense_cluster_stays_finite_and_finishes` to prove the dense stochastic cluster remains finite and settles correctly.

### Why
- The earlier review identified `WireBrush` as an O(N)-per-sample candidate.
- Its trigger stage already builds a sorted impulse schedule, so a cursor yields the same event order with much less per-sample work.

### Verification
- `cargo test --workspace --no-default-features`
- `cargo test --lib --target x86_64-unknown-linux-gnu`
- `cargo bench --no-default-features --target x86_64-unknown-linux-gnu --bench performance -- --noplot --sample-size 10`
- `bash scripts/verify-local.sh`

### Results
- `dense_cluster_stays_finite_and_finishes` is now part of the unit test suite.
- The local verification lane passed after the optimization.
- `wire_brush_dense_cluster` improved from millisecond-scale baseline measurements to microsecond-scale measurements in the benchmark harness.

### Remaining Phase 4 Risks
- No benchmark history is persisted yet, so current results are informative but not tracked over time.
- `MetalChain` still has a similar per-sample scan pattern and remains a future optimization candidate.

## Phase 5 — State And Validator Hardening

### Goal
Harden preset/state restoration against invalid external values and make host/plugin validator runs executable from the repository.

### Changes
- `Preset::load()` now sanitizes loaded values through the live parameter range surface instead of trusting raw JSON values.
- Added `invalid_preset_values_are_sanitized_on_load` to cover out-of-range top-level and extended preset fields.
- Added `exciter_and_object_matrix_stays_finite` to expand the automation/parameter-surface stress coverage across all exciters and objects.
- Added `scripts/validate-plugins.sh` as a concrete wrapper for `pluginval` and `clap-validator` with explicit skip behavior when those tools are absent.
- Updated `README.md` to include the validator wrapper in the documented local validation flow.

### Why
- The repo previously covered happy-path preset roundtrips, but not hostile or corrupted external state.
- The automation stress surface was too narrow for a plugin with a large control matrix.
- Host-facing validator commands existed only as documentation, not as an executable release step.

### Verification
- `cargo test --test preset_roundtrip --target x86_64-unknown-linux-gnu`
- `cargo test --test automation_stress --target x86_64-unknown-linux-gnu`
- `cargo fmt && bash scripts/verify-local.sh`
- `bash scripts/validate-plugins.sh`

### Results
- `preset_roundtrip` passed 4/4, including invalid-state sanitization coverage.
- `automation_stress` passed 3/3, including the new exciter/object matrix finite-output test.
- The full local verification lane passed after the state hardening changes.
- `pluginval` completed successfully against the bundled VST3.
- `clap-validator` completed successfully with 21 tests run, 18 passed, 0 failed, and 3 skipped.

### Remaining Phase 5 Risks
- The validator wrapper is still local/manual rather than part of a reproducible CI release lane.
- Preset sanitization currently normalizes ranges but does not yet implement a richer explicit migration/reporting layer for unknown future schema versions.

## Phase 6 — Documentation Suite And Quality Modes

### Goal
Close the remaining documentation gaps identified in the master prompt self-review and implement CPU/quality tradeoff modes.

### Changes
- Added `docs/ARCHITECTURE.md` with signal chain diagram, module map, real-time safety contract, voice lifecycle, parameter flow, and file organization.
- Added `docs/KNOWN_LIMITATIONS.md` documenting: missing quality modes (now resolved), unmeasured aliasing, unprofiled idle CPU, no Windows CI, no DAW-specific smoke tests, basic preset migration, per-sample post-processing, skipped GUI tests, no SIMD, and host-dependent parameter smoothing.
- Added `CHANGELOG.md` tracking all production-hardening phases from UI rewrite through Phase 6.
- Added `QualityMode` enum (Eco/Normal/High/Render) to `src/params.rs` with `quality_mode_param()` helper.
- Added `PostQualityMode` enum to `src/dsp/post_processing/post_chain.rs` with quality-aware stage bypass.
- Modified `PostProcessingChain::process()` to skip body, spread, and space in Eco mode.
- Made `OversampledClipper` oversampling factor configurable (1×/4×/8×/16×) and mapped it to Eco/Normal/High/Render.
- Added quality mode selector to the industrial egui header (`src/gui/editor.rs`).
- Added `quality_mode` to `Preset` with `#[serde(default = "default_quality_mode")]` so old presets default to Normal.
- Added idle-CPU benchmarks (`voice_manager_idle`, `post_processing_chain_idle`) to `benches/performance.rs`.
- Added `post_processing_chain_eco` benchmark for quality-mode comparison.
- Added `eco_mode_bypasses_stages` and `quality_mode_changes_oversample_factor` tests to `post_chain.rs`.
- Added `quality_mode_preset_roundtrip` test to `tests/preset_roundtrip.rs`.

### Why
- The master prompt Release Candidate Checklist requires architecture docs, known limitations, and a changelog.
- Benchmarks revealed the voice manager iterates all 8 slots even when idle, and post-processing is a meaningful CPU share; quality modes provide a real escape hatch for low-end systems.
- Mode counts are already modest (6–12 base modes), so mode-count scaling would be negligible; post-processing bypass and oversampling reduction are the high-leverage targets.

### Verification
- `cargo test --workspace --no-default-features`
- `cargo test --lib --target x86_64-unknown-linux-gnu`
- `cargo bench --no-default-features --target x86_64-unknown-linux-gnu --bench performance -- --noplot --sample-size 10`
- `cargo fmt && bash scripts/verify-local.sh`

### Results
- `post_processing_chain_eco` measured ~1.04 ms vs `post_processing_chain` ~2.57 ms for 4096 samples, a ~59% speedup.
- `voice_manager_idle` measured ~11.4 ms for 4096 samples, confirming the manager iterates all voice slots regardless of activity.
- Full local verification passed with 113 lib tests and 104 workspace tests.
- Linux release bundles built successfully.

### Remaining Phase 6 Risks
- Aliasing characterization is still unmeasured; the oversampled clipper mitigates it but no spectral regression tests exist.
- Idle CPU overhead is now benchmarked but not yet optimized (voice manager still scans all slots).
- Quality modes affect only post-processing; resonator mode count remains fixed at full fidelity.
- GUI tests are still skipped in headless validator runs.

## Phase 7 — Interaction Stability, Idle Rendering, Aliasing Harness, And Preset Recall

### Goal
Resolve the remaining user-facing control instability, eliminate wasted idle voice work, add an executable aliasing measurement path, and make the factory preset bank fully recall its stored state.

### Changes
- Simplified knob drag interaction in `src/gui/editor.rs` to use a fixed pointer anchor plus live display value instead of cumulative `drag_delta()` replay.
- Added `KnobDragState` so knob motion is derived from the initial pointer Y position and starting value for the entire drag gesture.
- Added a `rendering` lifecycle flag to `src/voice/mod.rs` so untouched and fully-decayed voices stop rendering entirely while note-off tails still ring out until silence.
- Updated `src/voice/manager.rs` to process only voices that still need rendering.
- Added `note_off_tail_still_renders_after_slot_becomes_inactive` to guard the tail behavior during the idle-path optimization.
- Added `AliasingReport` and `analyze_post_chain_aliasing()` to `src/offline/mod.rs` using a deterministic nonlinear post-chain stress signal and DFT-based residual-energy report.
- Extended `src/bin/render.rs` with `--suite aliasing`, which writes `aliasing_report.txt` as an executable QA artifact.
- Added `aliasing_report_is_finite_and_populated` to lock down the new aliasing harness.
- Fixed `apply_preset()` in `src/gui/editor.rs` so the factory preset loader applies the full sanitized preset surface, including extended controls and `quality_mode`, rather than only the top-level fields.

### Why
- The knob wobble was caused by the only control in the editor still using cumulative drag-delta replay instead of the file’s otherwise stable pointer-anchored interaction style.
- Idle benchmarking showed the manager was paying almost the full per-sample cost of all eight slots even when the pool was empty.
- Aliasing was the last major DSP quality question still documented as unmeasured.
- The factory preset bank existed on disk, but the GUI loader was not actually recalling most of each preset’s stored sound-design state.

### Verification
- `cargo test --lib --target x86_64-unknown-linux-gnu knob_drag_value -- --nocapture`
- `cargo test --lib --target x86_64-unknown-linux-gnu note_off_tail_still_renders_after_slot_becomes_inactive -- --nocapture`
- `cargo bench --no-default-features --target x86_64-unknown-linux-gnu --bench performance -- voice_manager_idle --noplot --sample-size 10`
- `cargo test --lib --target x86_64-unknown-linux-gnu aliasing_report_is_finite_and_populated -- --nocapture`
- `cargo run --target x86_64-unknown-linux-gnu --bin render -- --suite aliasing --out-dir /tmp/corrosion-aliasing-check --frame-count 2048`
- `cargo test --lib --target x86_64-unknown-linux-gnu factory_preset_loader_finds_factory_bank -- --nocapture`
- `cargo run --target x86_64-unknown-linux-gnu --bin render_presets -- --contains anchored_tank_moan --limit 1 --out-dir /tmp/corrosion-preset-bank-check`
- `bash scripts/verify-local.sh`

### Results
- Knob helper tests still pass, and the live knob path now uses anchored pointer motion instead of cumulative delta replay.
- `voice_manager_idle` improved from millisecond-scale runtime to `~91.8 µs` for 4096 samples, a `~99.19%` improvement.
- The aliasing CLI produced a concrete report at `/tmp/corrosion-aliasing-check/aliasing_report.txt` with `input_frequency_hz=9000`, `harmonic_bins=[384, 768]`, and `alias_ratio_db=-69.236900330`.
- The preset bank render path successfully rendered `Anchored Tank Moan` and wrote `/tmp/corrosion-preset-bank-check/preset_render_manifest.txt`.
- Full local verification passed after all four requested changes.

### Remaining Phase 7 Risks
- The aliasing report is a deterministic residual-energy proxy for the nonlinear post chain, not a full perceptual or oversampled-reference aliasing analysis.
- Direct interactive GUI drag verification still depends on headless egui/unit coverage plus user-side host confirmation because the repo has no full GUI event harness.
