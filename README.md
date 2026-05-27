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
# Test using the repo's default local target from .cargo/config.toml
cargo test --workspace --no-default-features

# Build the Linux bundle target used by bundle.sh
cargo build --target x86_64-unknown-linux-gnu

# Bundle Linux plugins
./bundle.sh

# Cross-compile Windows bundles
./bundle-win.sh

# Offline debug renderer
cargo run --target x86_64-unknown-linux-gnu --bin render -- --suite all

# Render all presets to WAV + manifest
cargo run --target x86_64-unknown-linux-gnu --bin render_presets

# Run the benchmark harness on the supported native no-GUI lane
cargo bench --no-default-features --target x86_64-unknown-linux-gnu --bench performance
```

The repository defaults to `x86_64-unknown-linux-musl` for unqualified local cargo commands through `.cargo/config.toml`. The bundle scripts and the documented plugin packaging flow use the explicit `x86_64-unknown-linux-gnu` target shown above.

## Validation

If you have the tools installed, the local validation flow is:

```bash
# Full local verification lane that mirrors CI
./scripts/verify-local.sh

# Or run the major steps manually
cargo fmt --check
cargo clippy --workspace --all-targets --no-default-features -- -D warnings
cargo test --workspace --no-default-features
cargo test --lib --target x86_64-unknown-linux-gnu
cargo run --target x86_64-unknown-linux-gnu --bin render -- --suite family --out-dir /tmp/corrosion-verify-render
python3 scripts/check_wav.py /tmp/corrosion-verify-render/pipe_comparison.wav
./bundle.sh release

# Optional host/plugin validators when installed locally
./scripts/validate-plugins.sh

pluginval --strictness-level 5 --validate target/bundled/Corrosion.vst3 --skip-gui-tests
clap-validator validate target/bundled/Corrosion.clap/Corrosion.clap --only-failed
```

The repository CI currently automates the formatter, clippy, the no-default-features workspace tests, the native Linux library lane, the offline renderer smoke test, WAV validation, and the Linux bundle script. DAW smoke scripts under `tests/daw/` and external plugin validators remain optional/manual because they depend on tools that are not guaranteed to exist in every environment.

Benchmark coverage is local/manual for now. Use `cargo bench --no-default-features --target x86_64-unknown-linux-gnu --bench performance` to measure the current hot-path baseline for voice rendering, stochastic excitation, post-processing, and offline rendering.

## Docs

Active design and implementation references:

- `docs/ARCHITECTURE.md` — as-built architecture, signal chain, and status of claimed work
- `docs/code-review.md` — critical code review and doc-vs-code conformance matrix
- `docs/corrosion_plugin_prd_and_specs.md` — product requirements (see §0 for current-state reconciliation)
- `docs/master-prompt.md` — production-hardening agent prompt (see "Current state" banner)
- `docs/detailed-specs/` — DSP design notes (each file carries an implementation-status banner)

Legacy roadmap/evidence notes still live under `.sisyphus/`, but the docs above are the current reference for the shipped surface.
