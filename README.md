# Corrosion

Corrosion is an industrial physical-modeling instrument built as a VST3 and CLAP plugin. MIDI notes trigger modal resonators and post-processing that are tuned for damaged metal, friction, weight, and mechanical tension rather than synth-style oscillator framing.

Current package version: `0.1.0`.

## What It Ships

- VST3 and CLAP plugin entrypoints via NIH-plug
- Stereo instrument layout with no audio input
- MIDI Basic note handling
- 8-voice polyphony with deterministic stealing
- A large parameter surface for exciter, object, transformation, and post-processing control
- An egui editor with UI scaling
- Offline renderers for comparison suites and preset batch renders

## Current Sound Surface

### Exciters

The current exciter set contains 16 types:
`Bow`, `HandStrike`, `FeltMallet`, `HardMallet`, `Drumstick`, `WireBrush`, `MetalPipe`, `MetalChain`, `StiffPoint`, `HeavyGrinding`, `CorrugatedDrag`, `TensionRise`, `PneumaticJet`, `ElectromagneticHum`, `TensionSnap`, and `ParticleRain`.

### Objects

The current object set contains 9 modal profiles:
`Pipe`, `Plate`, `Tank`, `Chain`, `IBeam`, `TautCable`, `CoilSpring`, `SheetMetal`, and `IndustrialCog`.

### Main Control Groups

- `Exciter`, `Object`
- `Size`, `Rust`, `Damage`
- `Drive`, `Output`, `Width`, `Body`
- exciter-specific envelopes and interaction controls
- UI scaling for the editor

## Repository Layout

```text
.
├── Cargo.toml
├── bundle.sh
├── bundle-win.sh
├── docs/
├── scripts/
├── src/
│   ├── lib.rs
│   ├── params.rs
│   ├── offline/
│   ├── presets/
│   ├── voice/
│   ├── dsp/
│   └── bin/
├── tests/
└── .sisyphus/
```

## Build And Run

```bash
# Test
cargo test --workspace --no-default-features

# Build Linux
cargo build --target x86_64-unknown-linux-gnu

# Bundle Linux plugins
./bundle.sh

# Cross-compile Windows bundles
./bundle-win.sh

# Offline debug renderer
cargo run --target x86_64-unknown-linux-gnu --bin render -- --suite all

# Render all presets to WAV + manifest
cargo run --target x86_64-unknown-linux-gnu --bin render_presets
```

## Validation

If you have the tools installed, the local validation flow is:

```bash
pluginval --strictness-level 5 --validate target/bundled/Corrosion.vst3 --skip-gui-tests
clap-validator validate target/bundled/Corrosion.clap/Corrosion.clap --only-failed
```

## Docs

Active design and implementation references:

- `docs/full-feature-surface.md`
- `docs/sound-direction-brief.md`
- `docs/interface-design-specification.md`
- `docs/new-detailed-specs/`

Legacy roadmap/evidence notes still live under `.sisyphus/`, but the docs above are the current reference for the shipped surface.
