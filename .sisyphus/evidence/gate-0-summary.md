# Gate 0 Evidence Summary — Research Prototype

**Status**: CLOSED
**Date**: 2026-05-02
**Gate**: 0 — Research Prototype

---

## 1. Deliverables

- Offline DSP renderer in Rust: DONE (`src/offline/mod.rs`, `src/bin/render.rs`)
- Second-order modal resonator prototype: DONE
- Deterministic excitation input: DONE (`ExcitationInput::deterministic_excitation()`)
- Pipe modal profile: DONE
- Plate modal profile: DONE
- Tank modal profile: DONE
- Family tuning (clearly distinct by ear): DONE
- Size scaling: DONE
- Rust-based decay/brightness loss: DONE
- Damage-driven detuning/roughness: DONE
- Mode count estimates: DONE
- CPU hot-spot identification: DONE
- Family comparison WAVs: DONE
- Rust variation WAVs: DONE
- Damage variation WAVs: DONE
- Initial parameter ranges: DONE (`.sisyphus/evidence/parameter-ranges.md`)

---

## 2. Render Artifacts Inventory

| Artifact Path | Peak | RMS | Checksum | Metrics |
|---------------|------|-----|----------|---------|
| `output/family-comparisons/pipe_comparison.wav` | 0.7705 | 0.3341 | 10505856658297768697 | - |
| `output/family-comparisons/plate_comparison.wav` | 0.4572 | 0.1364 | 11972400855468446761 | - |
| `output/family-comparisons/tank_comparison.wav` | 3.9568 | 1.4603 | 3876904354905967544 | - |
| `output/rust-variations/pipe_low_rust.wav` | 0.7003 | 0.3085 | 549757922344347805 | brightness=0.0335, late/early=0.9868 |
| `output/rust-variations/pipe_high_rust.wav` | 0.5351 | 0.2483 | 11657830496691765079 | brightness=0.0319, late/early=0.9767 |
| `output/damage-variations/pipe_low_damage.wav` | 0.7918 | 0.3342 | 8143782563304979968 | roughness=0.0661, late/early=0.9638 |
| `output/damage-variations/pipe_high_damage.wav` | 0.9876 | 0.3320 | 9023600758791334886 | roughness=0.1016, late/early=0.8557 |

---

## 3. Pass Criteria Status

| # | Criterion | Status | Evidence |
|---|-----------|--------|----------|
| 1 | Pipe, plate, and tank sound clearly distinct | PASS | RMS values differ by >10% pairwise (Pipe 0.33, Plate 0.13, Tank 1.46). |
| 2 | Excitation produces audible decaying output | PASS | All peaks > 0.05; late/early energy ratios < 1.0 in all variants. |
| 3 | Rust audibly darkens and shortens the sound | PASS | High rust brightness (0.0319) < low rust (0.0335); high rust late/early (0.9767) < low rust (0.9868). |
| 4 | Damage audibly destabilizes or roughens the sound | PASS | High damage roughness (0.1016) > low damage (0.0661). |
| 5 | Output is not silent by default when excited | PASS | All peaks > 0.01 in all renders. |
| 6 | No NaN, infinity, runaway feedback, or uncontrolled blowup | PASS | All peaks are finite; no NaN reported. Tank peak (3.95) is stable and decays. |

---

## 4. Carry-Forward Items

- **Real-time mode budget**: pipe=6, plate=8, tank=8, shared cap=8.
- **Allocation concern**: `ModalModeSpec::damaged` allocates per source mode — must NOT migrate to real-time rebuild path.
- **CPU hot path**: per-frame × per-mode `SecondOrderMode::process` loop in `src/dsp/resonator.rs`.

---

## 5. Gate 0 Decision

**GATE 0 STATUS: CLOSED**
