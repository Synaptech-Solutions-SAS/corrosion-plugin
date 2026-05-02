# Corrosion Gate 0 — Initial Parameter Ranges

> Frozen: 2026-05-02. Source: src/renderer.rs + docs/notepads/corrosion/decisions.md.
> Status: FROZEN FOR PLUGIN WORK (may adjust after MVP listening tests where noted).

## SizeScale

- min: 0.25
- max: None
- default: 1.0
- Perceptual Direction: size↑ ⇒ lower fundamental, longer decay, stronger low-mode gain

Source: `src/renderer.rs:781` (SizeScale struct)

Notes: Minimum factor is enforced in the constructor. No explicit maximum is set in the current prototype.

## RustAmount

- min: 0.0
- max: 1.0
- default: 0.0
- Perceptual Direction: rust↑ ⇒ darker highs, shorter tails

Source: `src/renderer.rs:812` (RustAmount struct)

Notes: Clamped to [0.0, 1.0] in the constructor.

## DamageAmount

- min: 0.0
- max: 1.0
- default: 0.0
- Perceptual Direction: damage↑ ⇒ rougher/beating behavior, detuning, companion modes

Source: `src/renderer.rs:842` (DamageAmount struct)

Notes: Clamped to [0.0, 1.0] in the constructor.

## Modal Mode Counts (Real-Time Budget)

- Pipe: min: 6 (safe), max: 12 (offline peak)
- Plate: min: 8 (safe), max: 16 (offline peak)
- Tank: min: 8 (safe), max: 16 (offline peak)
- Shared: default: 8 (safe cap)

Source: `src/renderer.rs:713` (RealtimeModeCountEstimate), `docs/notepads/corrosion/decisions.md:13`

## Decay Seconds (per family)

- Pipe: min: 0.62, max: 2.05
- Plate: min: 0.31, max: 0.94
- Tank: min: 0.72, max: 2.90

Source: `src/renderer.rs:871` (Pipe), `src/renderer.rs:881` (Plate), `src/renderer.rs:893` (Tank)

## Base Frequency Range

- Pipe: min: 220.0, max: 1327.0
- Plate: min: 286.0, max: 2860.0
- Tank: min: 96.0, max: 1002.0

Source: `src/renderer.rs:871` (Pipe), `src/renderer.rs:881` (Plate), `src/renderer.rs:893` (Tank)

## RenderConfig Defaults

- sample_rate: default: 48,000
- duration_seconds: default: 1.0 (48,000 frames)
- excitation_amplitude: default: 1.0

Source: `src/renderer.rs:7` (RenderConfig)
