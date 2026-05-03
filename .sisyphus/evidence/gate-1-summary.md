# Gate 1 Evidence Summary

## Date: 2026-05-03

## Status: **OPEN / BLOCKED**

Gate 1 cannot be closed until the REAPER host smoke test runs. The VST3 and CLAP bundles are built and pass pluginval/clap-validator, but the scripted host bounce evidence (G1-9) is blocked on missing `libGL.so.1`.

---

## Deliverables

### Plugin Shell
- **NIH-plug integration**: Real dependency from `https://github.com/robbert-vdh/nih-plug` (git HEAD `28b149ec`)
- **Plugin struct**: `Corrosion` implements `Plugin`, `ClapPlugin`, `Vst3Plugin` traits
- **MIDI handling**: NoteOn/NoteOff with sample-accurate timing
- **Voice**: 8-slot fixed voice manager with voice stealing, tail tracking
- **Safety**: Output clamp [-1, 1], denormal flush, NaN/inf → 0.0
- **Parameters**: Gain FloatParam (0 to +12 dB), Object IntParam (Pipe/Plate/Tank)

### Object Routing
The `Object` parameter routes the hit exciter into modal resonators:
- **Pipe**: tubular ring, clearer fundamental, moderate sustain
- **Plate**: flatter metallic object, more inharmonic spread
- **Tank**: lower, boomier cavity-like metal profile

The Object enum is applied in the audio path via `Object::to_profile()`.

### Module Structure
```
src/
├── lib.rs              # Plugin entry point (NIH-plug traits)
├── params.rs           # CorrosionParams with #[derive(Params)]
├── voice/
│   ├── mod.rs          # Voice struct, midi_to_hz, hit exciter
│   └── manager.rs      # 8-slot VoiceManager with deterministic stealing
├── dsp/
│   ├── mod.rs          # Re-exports + tests
│   ├── resonator.rs    # PlaceholderResonator, SecondOrderMode
│   ├── profile.rs      # ModalProfile, ModalModeSpec, 3 profiles
│   ├── transforms.rs   # SizeScale, RustAmount, DamageAmount
│   ├── excitation.rs   # ExcitationInput
│   └── budget.rs       # RealtimeModeCountEstimate
├── offline/
│   └── mod.rs          # OfflineRenderer, WAV writer, metrics
└── bin/
    └── render.rs       # Command-line offline renderer
```

### Build Artifacts

#### Linux (x86_64-linux-gnu)
- **VST3**: `target/bundled/Corrosion.vst3/Contents/x86_64-linux/Corrosion.so` (ELF 64-bit, 1.6MB)
- **CLAP**: `target/bundled/Corrosion.clap/Corrosion.clap` (ELF 64-bit, 1.6MB)

#### Windows (x86_64-pc-windows-gnu)
- **VST3**: `target/bundled-win/Corrosion.vst3/Contents/x86_64-win/Corrosion.vst3` (PE32+ DLL, 3.8MB)
- **CLAP**: `target/bundled-win/Corrosion.clap/Corrosion.clap` (PE32+ DLL, 3.8MB)

### Build Scripts
- `bundle.sh` - Linux VST3 + CLAP bundles
- `bundle-win.sh` - Windows VST3 + CLAP bundles

### Test Results
- **51/51 tests pass** (DSP + voice/manager coverage)
- `cargo test --workspace` → ok
- `cargo build --release --lib` → ok (both GNU and Windows targets)
- `cargo run --release --bin render` → ok (offline renderer)

### Validation Results

#### Linux VST3 (pluginval)
- **Status**: ✅ SUCCESS at strictness level 5
- **Evidence**: `.sisyphus/evidence/pluginval-gate-1-linux-vst3.log`
- **Tests**: All audio processing, automation, state, parameters passed

#### Linux CLAP (clap-validator)
- **Status**: ✅ 18 passed, 0 failed, 3 skipped
- **Evidence**: `.sisyphus/evidence/clap-validator-gate-1-linux.log`

#### REAPER Smoke Test
- **Status**: ⛔ **BLOCKED**
- **Blocker**: `libGL.so.1: cannot open shared object file`
- **Evidence**: `.sisyphus/evidence/task-G1-9-blocked.md`
- **Note**: REAPER is installed at `/usr/local/bin/reaper` but cannot start without Mesa/GL libraries

---

## Pass-Criteria Status

| Criterion | Status | Evidence |
|---|---|---|
| Plugin builds as VST3 and CLAP instrument targets | ✅ PASS | Linux bundles produced and validated |
| Plugin loads in host without immediate crashes | ✅ PASS | pluginval cold/warm open tests pass |
| MIDI note-on triggers audible sound | ✅ PASS | `voice/tests::note_on_activates_voice` + pluginval audio processing |
| Note-off allows natural decay rather than abrupt muting | ✅ PASS | `voice/tests::note_off_natural_decay` |
| Output remains bounded and free from obvious failure states | ✅ PASS | `output_clamped_to_unit_range`, `output_finite_over_long_render` tests |
| The code structure supports later parameter and voice expansion | ✅ PASS | Modular architecture with params/, voice/, dsp/ separation |
| Windows cross-compile | ✅ PASS | `bundle-win.sh` produces PE32+ DLLs |
| REAPER smoke test | ⛔ **BLOCKED** | `libGL.so.1` missing - see `.sisyphus/evidence/task-G1-9-blocked.md` |

---

## Quality Gates Verified

### Real-Time Safety
- ✅ No heap allocation in audio thread (assert_no_alloc feature enabled)
- ✅ No file I/O in sample rendering
- ✅ No logging in sample rendering
- ✅ No mutex locking in sample rendering
- ✅ No JSON parsing in sample rendering
- ✅ Output bounded to [-1.0, 1.0]
- ✅ Denormal and NaN/inf guards in place

### Cross-Platform
- ✅ Linux x86_64 VST3
- ✅ Linux x86_64 CLAP
- ✅ Windows x86_64 VST3 (cross-compiled)
- ✅ Windows x86_64 CLAP (cross-compiled)

---

## Current Blockers

| Blocker | Status | Details |
|---|---|---|
| REAPER `libGL.so.1` | ⛔ **ACTIVE** | Mesa/libglvnd packages needed for REAPER to start |
| Windows bundle validation | ⛔ **BLOCKED** | Wine unavailable for in-host validation |

Gate 1 remains **OPEN** until the REAPER scripted bounce can run.

---

## Carry-Forward to Gate 2

### Known Issues (Non-Blocking)
1. **Tank profile overshoot**: ~4× float peak - clamped at output, needs gain normalization
2. **ModalModeSpec::damaged() allocates Vec**: Acceptable offline, must be redesigned for real-time parameter changes
3. **MIDI note pitch not applied**: `midi_to_hz()` exists but modal profiles not retuned per note (Gate 2 feature)
4. **Gain not applied**: `Gain` parameter exists for host exposure but is not applied to samples yet
5. **Windows bundles not validated in-host**: Wine unavailable, will validate at release testing

### Required for Gate 2
- Expand parameter surface: Size, Rust, Damage, Drive, Output
- Apply MIDI note frequency scaling to modal profiles
- 20+ factory presets
- Generic editor (NIH-plug default or custom egui)
- Hard safety limiter / soft clipper

---

## Resolved Blockers

| Previous Blocker | Resolution Date | Resolution |
|---|---|---|
| Windows cross-compile tools | 2026-05-03 | mingw-w64-gcc and rustup targets installed |
| pluginval not installed | 2026-05-03 | `paru -S pluginval` installed |
| clap-validator not installed | 2026-05-03 | `paru -S clap-validator` installed |
| rustup not available | 2026-05-03 | `paru -S rustup` installed, musl + windows targets added |

---

## Evidence Logs

- `.sisyphus/evidence/pluginval-gate-1-linux-vst3.log`
- `.sisyphus/evidence/clap-validator-gate-1-linux.log`
- `.sisyphus/evidence/task-G1-9-blocked.md`
- `.sisyphus/evidence/task-4-gate1-*.log` (ongoing)

---

## Sign-Off

**Gate 1 Status**: ⛔ **OPEN / BLOCKED**

REAPER host smoke test is blocked on `libGL.so.1`. All other pass criteria satisfied. Gate 1 will close once the scripted host bounce evidence can be produced.

