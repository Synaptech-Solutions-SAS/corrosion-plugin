# Corrosion Architecture

This document describes the internal structure, signal flow, and engineering contracts of the Corrosion industrial physical-modeling synthesizer.

## 1. Signal Chain

The audio path follows a strict per-sample loop inside the host callback:

```text
MIDI Note Event
      │
      ▼
┌─────────────┐
│ VoiceManager│  ← Polyphony, stealing, per-note state
│   (8 voices)│
└──────┬──────┘
       │
       ▼
┌─────────────┐     ┌─────────────┐
│   Exciter   │────▶│ Interaction │  ← Force, displacement, velocity
│  (16 types) │     │    Bus      │
└─────────────┘     └──────┬──────┘
                           │
                           ▼
┌─────────────────────────────────────┐
│         ModalResonator              │  ← 9 object profiles, mode banks
│  (Pipe, Plate, Tank, Chain, etc.)   │
└─────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────┐
│      PostProcessingChain            │  ← Filter, drive, body, spread,
│  (WDF → Drive → Body → HRTF → Space│    space, oversampled clipper
│   → Clipper → Limiter)              │
└─────────────────────────────────────┘
                           │
                           ▼
                    Stereo Output
```

### Per-Sample Voice Loop

Inside `Voice::process_sample_stereo()`:

1. **Envelope** — Compute force envelope (OneShot, ADSR, or MSEG)
2. **Exciter** — Generate force from exciter model (displacement/velocity feedback)
3. **Resonator** — Feed force into modal bank, get stereo output
4. **Damage Rattle** — Add amplitude-dependent noise if damage > 0
5. **Highpass Boost** — Velocity-sensitive spectral shaping
6. **Clamp & Denormal Flush** — Output bounds and subnormal protection
7. **Peak Hold & Tail Tracking** — Voice-manager stealing data

## 2. Module Responsibilities

### `src/lib.rs` — Plugin Host Integration

- Implements `nih_plug::Plugin`, `ClapPlugin`, `Vst3Plugin`
- Bridges host MIDI → `VoiceManager`
- Bridges host parameter snapshots → `VoiceControls`
- Owns the global `PostProcessingChain`
- Applies final output gain and hard limiter
- **Thread:** Single audio thread only

### `src/params.rs` — Parameter Surface

- Defines 70+ `FloatParam` / `IntParam` fields with ranges, defaults, and skew
- `ExciterType` and `Object` enums map integers to named variants
- Host automation, serialization, and UI all read from this single source of truth
- Parameter IDs are stable for preset compatibility

### `src/voice/` — Polyphony and Per-Note Execution

#### `voice/mod.rs`
- `Voice` struct: one resonator + 16 exciter slots + envelope state
- `VoiceControls`: snapshot of all parameters copied at note-on
- `midi_to_hz()`: standard 440 Hz A4 tuning
- Envelope dispatch: `OneShotEnvelopePhase`, `AdsrPhase`, `MSEG`
- Real-time safety: no allocation, no locks, bounded loops

#### `voice/manager.rs`
- `VoiceManager`: fixed array of 8 `Voice` instances
- Stealing: prefer oldest voice with lowest `peak_hold`; non-finite peaks are treated as zero
- Sample-accurate event timing via `process_pending_events()`

### `src/dsp/` — Pure DSP Primitives

| Sub-module | Purpose |
|---|---|
| `exciters/` | 16 exciter algorithms (impact, scrape, specialty) |
| `resonators/` | Modal resonator core + 9 object profiles |
| `interaction/` | Bidirectional bus, spatial coefficients, coupling |
| `envelopes/` | MSEG with looping, velocity response, curve tension |
| `post_processing/` | WDF filter, Lorenz drive, body, HRTF spread, space, clipper |
| `profile/` | Modal mode tables for each object family |
| `transforms/` | Size, rust, damage, thickness, heat, sludge transformations |
| `utils/` | Mode-count estimation, offline limits, safe wrappers |

### `src/gui/editor.rs` — Industrial egui Interface

- Custom knobs, faders, screws, metal panels
- Parameter binding via NIH-plug gesture lifecycle
- UI scaling (50%–150%) via `EguiState`
- **Thread:** GUI thread only; never touches audio state directly

### `src/presets/mod.rs` — State Persistence

- `Preset` struct: versioned JSON schema with top-level + extended parameters
- `Preset::sanitized()` loads values through live parameter ranges (clamps hostile input)
- `Preset::from_params()` / `into_params()` roundtrip through `CorrosionParams`

### `src/offline/` — Batch Rendering

- Deterministic offline renderer for QA and comparison suites
- WAV output + JSON manifest
- Used by `src/bin/render.rs` CLI and integration tests

## 3. Real-Time Safety Contract

The audio callback (`Plugin::process()`) guarantees:

- **No heap allocation** — Verified by `tests/no_alloc.rs`
- **No locks or blocking** — Voice array is fixed; no mutexes
- **No file I/O or logging** — All I/O happens in GUI or offline contexts
- **No dynamic dispatch in hot path** — Exciter dispatch is a `match` on integer, not trait objects
- **No unbounded loops** — Modal banks have fixed maximum sizes; MSEG stages are bounded
- **Denormal protection** — `DENORMAL_FLUSH` added to voice output; resonator coefficients clamped
- **NaN/Inf guards** — Every voice output is checked with `is_finite()` before leaving the voice boundary

## 4. Voice Lifecycle

```text
Idle ──note_on()──▶ Active ──note_off()──▶ Tail ──peak<threshold──▶ Idle
                      │                        │   (0.1s decay)
                      │                        │
                      ▼                        ▼
               Steal if polyphony        Natural resonator decay
               exhausted                + envelope release
```

- **Note-on**: Parameters snapshotted → resonator rebuilt → exciter triggered → envelope initialized
- **Note-off**: `active` flag cleared → envelope enters release → exciter may release
- **Tail**: Resonator decays naturally; voice deactivates after `TAIL_DEACTIVATE_FRAMES` below `TAIL_ENERGY_THRESHOLD`
- **Steal**: Manager selects voice with lowest `peak_hold`; if all peaks are non-finite, they sort as zero

## 5. Parameter Flow

```text
Host DAW
   │
   ▼
nih_plug Params trait
   │
   ▼
CorrosionParams (Arc)
   │
   ├──▶ GUI thread (read-only display)
   │
   └──▶ Audio thread (read-only per-sample)
            │
            ▼
      VoiceControls snapshot at note-on
            │
            ▼
      Voice::process_sample_stereo()
```

All parameter values are copied into `VoiceControls` at note-on so the audio thread never reads shared mutable parameter state after that point.

## 6. Resonator Model

Each `ModalResonator` maintains a bank of `SecondOrderMode` structures:

- **Frequency**: Derived from profile table, scaled by `SizeScale`, detuned by `DamageAmount`
- **Damping**: Modified by `RustAmount`, `SludgeAmount`, `HeatAmount`, and `ResDamping`
- **Gain**: Normalized per-profile, boosted by `ResBrightness`, shaped by strike position
- **Transformation macros**: Applied at note-on; coefficients remain fixed during the note

Profiles are curated for distinct physical geometries:
- **Pipe** — Cylindrical longitudinal modes
- **Plate** — 2D surface, flatter metallic ring
- **Tank** — Volumetric, boomier cavity modes
- **Chain** — Linked segments, roughest profile
- **IBeam**, **TautCable**, **CoilSpring**, **SheetMetal**, **IndustrialCog**

## 7. Post-Processing Chain

Global stereo effects applied after voice mixing:

1. **WDF Ladder Filter** — Resonant lowpass with component tolerance
2. **Lorenz Drive** — Chaotic saturation with bias starvation
3. **Fem Body Resonator** — Cabinet/chassis simulation
4. **HRTF Spread** — Stereo width via interaural time difference
5. **Space** — Factory reverb, spring reverb, or echo (selectable)
6. **Oversampled Clipper** — 4x oversampled diode clipper with adjustable ceiling
7. **Hard Limiter** — Final `[-0.9661, 0.9661]` clamp

## 8. Build and Test Infrastructure

| Component | Purpose |
|---|---|
| `.github/workflows/ci.yml` | CI lane: fmt, clippy, tests, render smoke, bundle |
| `scripts/verify-local.sh` | Local one-shot verification mirror of CI |
| `scripts/validate-plugins.sh` | Optional `pluginval` + `clap-validator` wrapper |
| `benches/performance.rs` | Criterion harness for voice/exciter/post/offline paths |
| `tests/` | Integration tests: automation stress, preset roundtrip, body, drive, etc. |
| `src/bin/render.rs` | Offline CLI for deterministic QA renders |

## 9. Known Architectural Decisions

- **Per-sample processing**: The entire voice chain is per-sample rather than block-based to preserve the bidirectional exciter↔resonator feedback loop. Post-processing could be block-optimized in the future but is currently per-sample for simplicity.
- **Static exciter dispatch**: Each voice owns all 16 exciter structs to avoid allocation and virtual dispatch. This increases voice memory but guarantees real-time safety.
- **No quality modes yet**: The resonator always uses its full mode count. Eco/Normal/High/Render modes are planned but not yet implemented.
- **MSEG only for friction family**: Impact exciters use OneShot; specialty uses ADSR; friction uses MSEG. This is by design to match physical behavior, not a limitation.

## 10. File Map

```text
src/
  lib.rs              Plugin trait impl, MIDI routing, output limiter
  params.rs           70+ parameter definitions, ExciterType/Object enums
  voice/
    mod.rs            Voice struct, envelopes, exciter dispatch
    manager.rs        VoiceManager, stealing, polyphony
  dsp/
    mod.rs            DSP re-exports
    exciters/         16 exciter algorithms
    resonators/       Modal bank, second-order modes, 9 profiles
    interaction/      Bidirectional bus, spatial coefficients
    envelopes/        MSEG, ADSR, OneShot
    post_processing/  Filter, drive, body, spread, space, clipper
    profile/          Modal mode tables
    transforms/       Size, rust, damage, thickness, heat, sludge
    utils/            Mode-count estimation, offline helpers
  gui/
    editor.rs         Industrial egui interface (feature-gated)
  presets/
    mod.rs            Preset save/load, sanitization, roundtrip
  offline/
    mod.rs            Offline renderer, WAV writer, comparison suites
  bin/
    render.rs         CLI entry point for offline rendering
    render_presets.rs Preset batch renderer
```
