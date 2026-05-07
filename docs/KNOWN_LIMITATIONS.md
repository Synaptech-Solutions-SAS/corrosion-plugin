# Known Limitations

This document lists current gaps, risks, and unimplemented features in Corrosion. It is intended for developers, testers, and power users evaluating the plugin for production use.

## 1. Quality Modes

**Status:** Not implemented  
**Impact:** CPU cost is fixed at the highest-fidelity setting for every preset.

The master prompt specifies four quality tiers (Eco, Normal, High, Render) that should scale mode count, oversampling, and space complexity. Currently the resonator always uses its full mode table and the oversampled clipper always runs at 4x. On lower-end CPUs this may limit polyphony.

**Workaround:** Reduce polyphony by playing fewer simultaneous notes.

## 2. Aliasing Characterization

**Status:** Not measured  
**Impact:** Unknown whether nonlinear stages (drive, clipper) produce audible aliasing at standard sample rates.

The Lorenz drive and oversampled clipper have bounded-output tests, but no spectral regression tests exist to quantify aliasing energy relative to the fundamental.

**Workaround:** The oversampled clipper (4x) mitigates aliasing at the final stage; upstream drive stages are not oversampled.

## 3. Idle CPU Usage

**Status:** Not profiled  
**Impact:** Unknown baseline CPU cost when no notes are playing.

The post-processing chain still processes per-sample even with zero input. While this is cheap, it has not been measured against a target idle budget.

## 4. Windows Bundle Validation

**Status:** No CI coverage  
**Impact:** `bundle-win.sh` is untested in automation.

The CI workflow only validates the Linux GNU bundle. Cross-compiled Windows artifacts are built by script but not smoke-tested in a Windows environment.

## 5. DAW-Specific Compatibility

**Status:** Manual only  
**Impact:** Host quirks (parameter automation timing, preset recall, UI resizing) may vary across DAWs.

Validation currently relies on `pluginval` and `clap-validator`. No REAPER, Bitwig, Ardour, Ableton, or FL Studio smoke tests are automated.

## 6. Preset Migration Depth

**Status:** Basic fallback only  
**Impact:** Loading a future schema version (v4+) will silently use defaults for unknown fields.

Current sanitization clamps values to valid ranges but does not implement explicit migration rules (e.g., remapping old parameter IDs, warning about deprecated fields).

## 7. Post-Processing Block Optimization

**Status:** Per-sample only  
**Impact:** Slightly higher CPU usage than necessary for global effects.

The `PostProcessingChain` processes one stereo sample per call. Block processing could improve cache locality and reduce function-call overhead, but the current per-sample approach keeps the code simple and predictable.

## 8. GUI Testing

**Status:** Headless environments skip GUI tests  
**Impact:** `pluginval --skip-gui-tests` is required in CI; custom widget behavior is only tested via unit tests in `gui/editor.rs`.

The industrial egui interface (knobs, faders, scaling) has regression tests for drag math and layout counting, but no automated visual regression or interaction tests.

## 9. No SIMD / Vectorization

**Status:** Scalar code throughout  
**Impact:** Modal resonator updates and exciter loops run sequentially.

The modal bank updates each mode independently. A structure-of-arrays layout with SIMD could significantly improve voice throughput, but this would be a major refactor of `SecondOrderMode` and `ModalResonator`.

## 10. Parameter Smoothing

**Status:** Host-dependent  
**Impact:** Rapid automation of filter cutoff or drive may produce zipper noise.

NIH-plug provides parameter smoothing, but the plugin does not implement additional per-parameter smoothing inside the DSP for parameters with audible step artifacts.
