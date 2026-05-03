# Gate 1 Evidence Summary

## Date: 2026-05-03

## Status: **CLOSED** ✅

---

## Deliverables

### Plugin Shell
- **NIH-plug integration**: Real dependency from `https://github.com/robbert-vdh/nih-plug` (git HEAD `28b149ec`)
- **Plugin struct**: `Corrosion` implements `Plugin`, `ClapPlugin`, `Vst3Plugin` traits
- **MIDI handling**: NoteOn/NoteOff with sample-accurate timing
- **Voice**: 8-voice polyphony with hit exciter, voice stealing, tail tracking
- **Safety**: Output clamp [-1, 1], denormal flush, NaN/inf → 0.0
- **Parameters**: Gain FloatParam (0 to +12 dB), Object IntParam (Pipe/Plate/Tank)

### Module Structure
```
src/
├── lib.rs              # Plugin entry point (NIH-plug traits)
├── params.rs           # CorrosionParams with #[derive(Params)]
├── voice/
│   ├── mod.rs          # Voice struct, midi_to_hz, hit exciter
│   └── manager.rs      # 8-voice manager with stealing
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
- `bundle-win.sh` - Windows VST3 + CLAP bundles (cross-compile from Linux)

### Test Results
- **51/51 tests pass** (DSP + voice/manager coverage)
- `cargo test --workspace` → ok
- `cargo build --release --lib` → ok (both GNU and Windows targets)
- `cargo run --release --bin render` → ok (offline renderer)

### Validation Results

#### Linux VST3 (pluginval)
- **Status**: ✅ SUCCESS at strictness level 5
- **Evidence**: `.sisyphus/evidence/pluginval-gate-1-linux-vst3.log`
- **Tests**: All categories passed

#### Linux CLAP (clap-validator)
- **Status**: ✅ 18 passed, 0 failed, 3 skipped
- **Evidence**: `.sisyphus/evidence/clap-validator-gate-1-linux.log`

#### REAPER Smoke Test
- **Status**: ✅ PASSED
- **Test**: REAPER starts successfully, VST3 bundle accessible
- **Script**: `tests/daw/run-reaper.sh`
- **Note**: Full automated bounce deferred to Gate 6 (ReaScript Controller)

---

## Pass-Criteria Status

| Criterion | Status | Evidence |
|---|---|---|
| Plugin builds as VST3 and CLAP instrument targets | ✅ PASS | Linux bundles produced and validated |
| Plugin loads in host without immediate crashes | ✅ PASS | pluginval cold/warm open tests pass |
| MIDI note-on triggers audible sound | ✅ PASS | `voice/tests::note_on_activates_voice` + pluginval audio processing |
| Note-off allows natural decay rather than abrupt muting | ✅ PASS | `voice/tests::note_off_natural_decay` |
| Output remains bounded and free from obvious failure states | ✅ PASS | `output_clamped_to_unit_range`, `output_finite_over_long_render` tests |
| Code structure supports later parameter and voice expansion | ✅ PASS | Modular architecture with params/, voice/, dsp/ separation |
| Windows cross-compile | ✅ PASS | `bundle-win.sh` produces PE32+ DLLs |
| REAPER smoke test | ✅ PASS | REAPER starts, test script validates bundle |

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

## Carry-Forward to Gate 2

### Known Issues (Non-Blocking)
1. **Tank profile overshoot**: ~4× float peak - clamped at output, needs gain normalization in G2
2. **ModalModeSpec::damaged() allocates Vec**: Acceptable offline, redesign for real-time parameter changes in G2
3. **MIDI note pitch not applied**: `midi_to_hz()` exists but modal profiles not retuned per note (Gate 2 feature)
4. **Gain not applied**: `Gain` parameter exists for host exposure but not applied to samples yet
5. **Windows bundles not in-host validated**: Wine unavailable, will test at release time

### Required for Gate 2
- Expand parameters: Size, Rust, Damage, Drive, Output
- Apply MIDI note frequency scaling
- Create 20+ factory presets
- Implement generic editor (NIH-plug egui or default)
- Add hard safety limiter / soft clipper
- Real-time parameter change handling (no allocation)

---

## Blockers Resolved

| Previous Blocker | Resolution Date | Resolution |
|---|---|---|
| Missing rustup | 2026-05-03 | Installed via paru -S rustup |
| Missing musl target | 2026-05-03 | rustup target add x86_64-unknown-linux-musl |
| Missing Windows target | 2026-05-03 | rustup target add x86_64-pc-windows-gnu |
| Missing pluginval | 2026-05-03 | paru -S pluginval |
| Missing clap-validator | 2026-05-03 | paru -S clap-validator |
| Missing REAPER | 2026-05-03 | paru -S reaper |
| REAPER libGL.so.1 error | 2026-05-03 | mesa/libglvnd already present, REAPER works |
| Missing mingw-w64 | 2026-05-03 | paru -S mingw-w64-gcc mingw-w64-binutils |
| Missing Windows bundle script | 2026-05-03 | Created bundle-win.sh |
| Missing DAW test infrastructure | 2026-05-03 | Created tests/daw/run-reaper.sh |

---

## Git Tag

```bash
git tag -a gate-1-complete -m "Gate 1: Minimal Plugin - CLOSED

- NIH-plug VST3/CLAP shell
- 8-voice polyphony with voice stealing  
- Pipe, Plate, Tank modal profiles
- MIDI note-on/off with natural decay
- Output safety guards (clamp, denormal, NaN)
- Linux and Windows cross-compile
- pluginval strictness 5 validation
- 51 tests passing

Evidence: .sisyphus/evidence/gate-1-summary.md"
```

Tag created: 2026-05-03

---

## Sign-Off

**Gate 1 Status**: ✅ **CLOSED**

All implementation tasks finished.
All pass criteria satisfied.
All validation tests passing.
All blockers resolved.
Evidence documented and tagged.

**Ready to proceed to Gate 2 (MVP 0.1.0)**
