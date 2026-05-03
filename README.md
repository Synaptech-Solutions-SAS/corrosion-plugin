# Corrosion

Corrosion is an industrial physical-modeling instrument built as a VST3 and CLAP plugin. The current implementation turns MIDI notes into struck metal resonances using curated modal profiles for pipe, plate, and tank objects. The repository also contains an offline renderer that produces deterministic WAV evidence for DSP behavior and gate-by-gate verification artifacts under `.sisyphus/evidence/`.

**Current Status: Gate 1 CLOSED ✅ | Ready for Gate 2 (MVP 0.1.0)**

The core Rust crate builds a NIH-plug plugin shell, exports VST3 and CLAP entrypoints, preserves the offline renderer, and includes DSP/voice tests for the current physical-modeling path.

---

## Current Status

### Gate 1 Complete (CLOSED)

- ✅ NIH-plug integration with a `Corrosion` plugin type
- ✅ Linux VST3 bundle generation at `target/bundled/Corrosion.vst3/`
- ✅ Linux CLAP bundle generation at `target/bundled/Corrosion.clap/`
- ✅ Windows VST3/CLAP cross-compile via `bundle-win.sh`
- ✅ VST3 export via `nih_export_vst3!(Corrosion)`
- ✅ CLAP export via `nih_export_clap!(Corrosion)`
- ✅ Instrument-style audio layout: no main input, stereo output
- ✅ MIDI input handling for `NoteOn` and `NoteOff`
- ✅ 8-slot fixed voice manager with deterministic stealing behavior
- ✅ Hit exciter routed into pipe/plate/tank modal resonators
- ✅ Hard output safety: finite guard, denormal guard, and clamp to `[-1.0, 1.0]`
- ✅ `Gain` and `Object` parameters using NIH-plug's `Params` derive
- ✅ Offline deterministic rendering for family, rust, and damage evidence
- ✅ Linux VST3 validation with pluginval strictness 5 - **PASSED**
- ✅ Linux CLAP validation with `clap-validator` - **PASSED**
- ✅ REAPER smoke test - **PASSED**
- ✅ All 51 tests passing
- ✅ Git tag `gate-1-complete` created

### Blockers Resolved

All previously blocked items are now resolved:
- ✅ REAPER runtime libraries installed and working
- ✅ Windows cross-compile toolchain installed (mingw-w64)
- ✅ All validation tools installed (pluginval, clap-validator)
- ✅ Rustup with musl and windows-gnu targets

### Gate 2 TODO (MVP 0.1.0)

- [ ] Expand parameter surface: Size, Rust, Damage, Drive, Output
- [ ] Apply MIDI note frequency scaling to modal profiles
- [ ] Create 20+ factory presets
- [ ] Implement generic editor (NIH-plug egui or default)
- [ ] Add hard safety limiter / soft clipper
- [ ] Real-time parameter change handling (no allocation)

---

## What Corrosion Does

Corrosion models struck industrial objects instead of using oscillator/filter/amp synth framing. A MIDI note triggers a short excitation impulse. That impulse drives a bank of second-order modal resonators. The modal profile controls the perceived object:

- `Pipe`: tubular ring, clearer fundamental, moderate sustain.
- `Plate`: flatter metallic object, more inharmonic spread.
- `Tank`: lower, boomier cavity-like metal profile.

The offline renderer and DSP tests verify perceptual direction with metrics rather than subjective listening only:

- `brightness_proxy`
- `roughness_proxy`
- `late_to_early_energy_ratio`
- peak/RMS/checksum values

Gate 0 established that pipe, plate, and tank are measurably distinct; rust darkens and shortens; damage roughens/destabilizes; and rendered output is deterministic and finite.

---

## Quick Start

```bash
# Run all tests
cargo test --workspace

# Build Linux bundles
./bundle.sh

# Build Windows bundles (cross-compile)
./bundle-win.sh

# Run offline renderer
cargo run --release --bin render

# Validate plugins
pluginval --strictness-level 5 --validate target/bundled/Corrosion.vst3 --skip-gui-tests
clap-validator validate target/bundled/Corrosion.clap/Corrosion.clap --only-failed
```

---

## Repository Layout

```text
.
├── Cargo.toml
├── bundle.sh              # Linux VST3/CLAP bundles
├── bundle-win.sh          # Windows VST3/CLAP bundles (cross-compile)
├── .cargo/config.toml     # Multi-target Rust configuration
├── .github/workflows/     # CI/CD for macOS builds
├── docs/
│   ├── IMPLEMENTATION_PLAN.md
│   └── plans/corrosion.md
├── output/                # Offline renderer output
├── scripts/
│   ├── check_clicks.py
│   └── check_wav.py
├── src/
│   ├── lib.rs             # Plugin entry point
│   ├── params.rs          # Host parameters
│   ├── bin/render.rs      # CLI offline renderer
│   ├── dsp/               # DSP modules
│   │   ├── mod.rs
│   │   ├── budget.rs
│   │   ├── excitation.rs
│   │   ├── profile.rs
│   │   ├── resonator.rs
│   │   └── transforms.rs
│   ├── offline/mod.rs     # Offline rendering
│   └── voice/             # Voice management
│       ├── mod.rs
│       └── manager.rs
├── tests/
│   └── daw/
│       └── run-reaper.sh  # DAW smoke test
└── .sisyphus/
    ├── evidence/          # Verification artifacts
    ├── notepads/          # Development notes
    └── plans/             # Roadmap
```

---

## Prerequisites

### Arch Linux (Current Development Environment)

```bash
# Core Rust toolchain
paru -S rustup
rustup default stable
rustup target add x86_64-unknown-linux-musl
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu

# Build tools
paru -S gcc mingw-w64-gcc mingw-w64-binutils

# Validation tools
paru -S pluginval clap-validator reaper

# Python for helper scripts
paru -S python3
```

### Other Linux Distributions

**Ubuntu/Debian:**
```bash
sudo apt install -y rustup gcc mingw-w64 python3
# pluginval and clap-validator: download from GitHub releases
```

**Fedora/AlmaLinux/RHEL:**
```bash
sudo dnf install -y rustup gcc mingw64-gcc python3
# Enable CRB repository for mingw on AlmaLinux 9
```

---

## How To Build And Run

### Run All Tests

```bash
cargo test --workspace
```

Expected: **51 tests pass**.

### Build Linux Bundles

```bash
./bundle.sh
```

Outputs:
- `target/bundled/Corrosion.vst3/Contents/x86_64-linux/Corrosion.so`
- `target/bundled/Corrosion.clap/Corrosion.clap`

### Build Windows Bundles (Cross-Compile)

```bash
./bundle-win.sh
```

Outputs:
- `target/bundled-win/Corrosion.vst3/Contents/x86_64-win/Corrosion.vst3`
- `target/bundled-win/Corrosion.clap/Corrosion.clap`

Verify PE format:
```bash
file target/bundled-win/Corrosion.vst3/Contents/x86_64-win/Corrosion.vst3
# Should show: PE32+ executable (DLL) x86-64
```

### Run The Offline Renderer

```bash
cargo run --release --bin render
```

Output goes to `output/damage-variations/`.

Validate a generated WAV:
```bash
python3 scripts/check_wav.py output/damage-variations/pipe_high_damage.wav
```

### Validate VST3 With Pluginval

```bash
pluginval --strictness-level 5 \
  --validate target/bundled/Corrosion.vst3 \
  --skip-gui-tests
```

Expected: **SUCCESS**

Evidence: `.sisyphus/evidence/pluginval-gate-1-linux-vst3.log`

### Validate CLAP With Clap Validator

```bash
clap-validator validate \
  target/bundled/Corrosion.clap/Corrosion.clap \
  --only-failed
```

Expected: **18 passed, 0 failed, 3 skipped**

Evidence: `.sisyphus/evidence/clap-validator-gate-1-linux.log`

### REAPER Smoke Test

```bash
./tests/daw/run-reaper.sh
```

Validates REAPER can start and access the plugin bundle.

---

## Module Guide

### `src/lib.rs`

Plugin entrypoint. Wires together NIH-plug, parameters, voice manager, and exports.

Responsibilities:
- Defines `Corrosion` plugin struct
- Implements `Plugin`, `ClapPlugin`, `Vst3Plugin` traits
- Exports CLAP and VST3 entrypoints
- Sets stereo instrument output layout
- Pulls MIDI events from process context
- Converts object parameter into modal profile
- Sends note events to `VoiceManager`
- Writes mixed voice output

### `src/params.rs`

Host-visible parameters:
- `Gain`: linear gain from `0.0` to `+12 dB` equivalent
- `Object`: integer selecting `Pipe`, `Plate`, or `Tank`

Gate 2 will expand this to include Size, Rust, Damage, Drive, and Output.

### `src/voice/mod.rs`

Single voice implementation:
- `midi_to_hz(note)`: MIDI note to frequency conversion
- `Voice`: activity, note, velocity, resonator state, peak hold, tail tracking
- `note_on()`: arms voice with chosen profile
- `note_off()`: marks inactive while allowing resonator tail
- `process_sample()`: excitation + resonator + safety guards

**Note**: MIDI note-to-frequency conversion exists and is tested, but modal profiles are not yet retuned per MIDI note. This is planned for Gate 2.

### `src/voice/manager.rs`

8-voice polyphony management:
- Fixed `[Voice; 8]` array
- First inactive slot allocation
- Quietest voice stealing with deterministic ties
- Frame counter for age tracking
- Summed output scaled by voice count

### `src/dsp/resonator.rs`

Modal resonator core:
- `PlaceholderResonator`: modal resonator wrapper
- `ResonatorCore`: trait for offline renderer and voices
- `SecondOrderMode`: per-mode state
- `ResonatorCoefficients`: frequency/decay/sample_rate → coefficients

This is the hottest DSP path in the project.

### `src/dsp/profile.rs`

Curated modal profiles:
- `ModalProfileId`: `Pipe`, `Plate`, `Tank`
- `ModalProfile`: bank of mode specs
- `ModalModeSpec`: frequency_hz, decay_seconds, gain

Profiles are curated, not algorithmically generated, to maintain industrial sound identity.

### `src/dsp/transforms.rs`

Physical behavior transforms:
- `SizeScale`: larger objects → lower pitch, longer decay
- `RustAmount`: darkens and shortens
- `DamageAmount`: detunes, roughens, expands modes

Frozen parameter ranges in `.sisyphus/evidence/parameter-ranges.md`.

### `src/dsp/excitation.rs`

Deterministic excitation input for offline renderer.

### `src/dsp/budget.rs`

Real-time mode-count estimates:
- Pipe: 6 modes
- Plate: 8 modes  
- Tank: 8 modes
- Shared cap: 8 modes per voice

### `src/offline/mod.rs`

Non-plugin offline rendering:
- `OfflineRenderer`: deterministic renders
- PCM WAV writing
- Render summaries (peak, RMS, checksum, first samples)
- Behavior metrics (brightness, roughness, energy ratios)

This module does file I/O and allocation (not real-time safe).

### `src/bin/render.rs`

Command-line offline renderer. Renders damage variations to `output/damage-variations/`.

### `scripts/check_wav.py`

Validates WAV files: length, finite samples, non-silent, peak threshold.

### `scripts/check_clicks.py`

Click/silence heuristics for rendered WAVs.

---

## Platform Support

| Platform | VST3 | CLAP | Build Method |
|----------|------|------|--------------|
| Linux x86_64 | ✅ Native | ✅ Native | `./bundle.sh` |
| Windows x86_64 | ✅ Cross-compile | ✅ Cross-compile | `./bundle-win.sh` |
| macOS x86_64/ARM | ⚠️ CI only | ⚠️ CI only | GitHub Actions |

**Note**: macOS builds are produced via GitHub Actions (free) since cross-compiling from Linux requires macOS SDK (Apple license restriction).

---

## Roadmap

See `.sisyphus/plans/corrosion-roadmap.md` for full details.

### Gate 0 ✅ CLOSED
Offline DSP prototype with pipe/plate/tank profiles, size/rust/damage transforms, evidence renders.

### Gate 1 ✅ CLOSED
Minimal NIH-plug VST3/CLAP shell, 8-voice polyphony, MIDI handling, output safety, validation clean.

### Gate 2 🔄 IN PROGRESS (MVP 0.1.0)
- [ ] Expand parameters (Size, Rust, Damage, Drive, Output)
- [ ] MIDI note frequency scaling
- [ ] 20+ factory presets
- [ ] Generic editor
- [ ] Hard safety limiter

### Gate 3 (Industrial Character 0.2.0)
Scrape exciter, chain object, stereo modal spread, body resonator, 40+ presets.

### Gate 4 (Product UX 0.3.0)
Custom GUI, macros, randomizer, preset browser.

### Gate 5 (Sequenced Instrument)
16/32-step sequencer, per-step locks, host sync.

### Gate 6 (Version 1.0)
100+ presets, full docs, release packaging.

---

## Real-Time Safety

Hot path requirements (process callback, per-sample loops):
- ❌ No file I/O
- ❌ No logging
- ❌ No mutex/RwLock
- ❌ No JSON parsing
- ❌ No blocking work
- ❌ No heap allocation or vector resizing

Current exceptions (acceptable for setup only):
- Resonator/profile construction
- Excitation setup

These will be redesigned for Gate 2 real-time parameter changes.

---

## Current Limitations

- `Gain` parameter exists but is not applied in `process()` yet
- MIDI note pitch not used to retune modal profiles (function exists, not wired)
- Realtime plugin uses default size/rust/damage values
- No GUI yet
- No preset bank yet
- No sequencer yet
- Tank profile has ~4× peak overshoot (clamped at output, needs gain normalization in Gate 2)
- `ModalModeSpec::damaged()` allocates `Vec` (acceptable offline, must redesign for Gate 2 RT)

---

## Evidence And Project State

Important evidence files:
- `.sisyphus/evidence/gate-0-summary.md` - Gate 0 closure
- `.sisyphus/evidence/gate-1-summary.md` - Gate 1 closure (CLOSED)
- `.sisyphus/evidence/parameter-ranges.md` - Frozen parameter ranges
- `.sisyphus/evidence/pluginval-gate-1-linux-vst3.log` - VST3 validation
- `.sisyphus/evidence/clap-validator-gate-1-linux.log` - CLAP validation

Important notepads:
- `.sisyphus/notepads/corrosion-roadmap/learnings.md`
- `.sisyphus/notepads/corrosion-roadmap/issues.md`

---

## License

See `Cargo.toml` and individual file headers. VST3 bindings are GPLv3; NIH-plug framework is ISC.

---

## Contributing

This is a personal project following the Sisyphus planning methodology. See `.sisyphus/plans/` for roadmap details.
