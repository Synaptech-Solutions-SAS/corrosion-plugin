> **Implementation status (2026-05).** This file originally described an
> "Unbound Fidelity Tier" of research-grade DSP. The **shipped** post chain is a
> set of lightweight, real-time-safe *approximations* of those targets. Each
> section below states what is **Implemented** today and keeps the original
> **Design target** for reference. Verified against
> `src/dsp/post_processing/` and `src/lib.rs`. See `docs/ARCHITECTURE.md` §9 and
> `docs/code-review.md` for the conformance matrix and known defects.
>
> Chain order (`PostProcessingChain::process`): fold→mono → WDF filter → Lorenz
> drive → **[Eco: bypass body/spread/space]** → FEM body → re-stereo → HRTF
> spread → Space → oversampled clipper. The whole chain runs **per sample**
> today, and `lib.rs` currently sets every stage's parameters once per sample
> (a known inefficiency — see code review Finding 3).

---

### `4. Post / Space / Output Processing`

### 1. Multimode Ladder Filter
**Implemented (`wdf_filter.rs`):** A 4-pole ladder approximation. Per-stage
trapezoidal-style update with a `tanh`-like soft "transistor" clip; resonance via
a feedback path with a `1/(1+res·3.5)` compensation gain; `component_tolerance`
injects a tiny hash-noise jitter. It is **not** a Newton-Raphson WDF circuit
solver. Parameters: `filter_cutoff` (20 Hz–20 kHz, clamped), `filter_resonance`
(0–1), `component_tolerance` (0–1).

**Design target:** Wave Digital Filter / RK4 transistor-ladder circuit simulation
with parasitic capacitance, thermal noise, and true electrical self-oscillation.

### 2. Drive (Soft Saturation to Fuzz Chaos)
**Implemented (`lorenz_drive.rs`):** A genuine Lorenz attractor is integrated
(σ=10, ρ=28, β=8/3, clamped state) but is used only as a **mild multiplicative
gain modulator** (`1 + chaos·tanh(x·0.02)·0.5`) on top of a soft tube curve.
`bias_starvation` gates the signal via a crude threshold on the attractor's z
state. Output is dry/wet mixed and clamped. Parameters: `drive_amount` (0–5),
`bias_starvation` (0–1), `chaos_depth` (0–1).

**Design target:** Component-level tube/diode cascade coupled into a bifurcating
Lorenz system that drives the audio into true analog chaos.

### 3. Stereo Spread
**Implemented (`hrtf_spread.rs`):** Up to ~1 ms interaural delay + a one-pole
low-pass/high-pass split shaped by `listener_proximity`, plus width-based
crossfeed. There is **no HRIR convolution and no azimuth/elevation model**.
Parameters: `spread_width` (0–1), `listener_proximity` (0–1).

**Design target:** Per-mode 3D radiation pattern convolved with measured HRIR
filters (`h_{L,n}(t,θ,φ)`).

### 4. Body Resonator
**Implemented (`fem_body.rs`):** A bank of **8 fixed second-order modes**
(220–1400 Hz base) whose frequencies shift with `chassis_material` and
`chassis_volume`. It is a small modal coloration stage, **not** a finite-element
mesh. Parameters: `chassis_material` (0–1), `chassis_volume` (0–1). Note:
`lib.rs` feeds `chassis_volume.max(body_amount)` so the global `Body` macro can
drive it.

**Design target:** Pre-computed 3D FEM mesh of a wooden/metallic body with
thousands of interconnected nodes.

---

### `Space Modes`

### 5. Factory Reverb
**Implemented (`space.rs::FactoryReverb`):** A Schroeder-style reverb — 4 comb
filters + 2 allpass diffusers, with `wall_impedance` setting feedback and
`machinery_clutter` modulating it. It is **not** an FDTD wave solver.
> ⚠️ **Known bug:** `update_delays` re-scales `comb_delays` cumulatively and is
> called every sample, so `factory_size` rails the delays to their clamp bounds.
> See code review Finding 2.

**Design target:** 3D Finite-Difference Time-Domain acoustic wave solver with
diffraction around injected boundary nodes.

### 6. Spring Reverb
**Implemented (`space.rs::SpringReverb`):** A single delay line with a one-pole
dispersion filter and tension-controlled feedback. **Not** a helical PDE.
Parameters: `spring_tension`, `wire_stiffness`, `spring_tank_size`.

**Design target:** FDTD stiff-string PDE capturing longitudinal/transverse/
torsional waves.

### 7. Factory Echo
**Implemented (`space.rs::FactoryEcho`):** A modulated stereo delay;
`machinery_movement` modulates the read position for a gentle Doppler/detune,
`high_frequency_damping` rolls off the wet path. This is a reasonable match to the
design intent. Parameters: `delay_time`, `machinery_movement`, `high_frequency_damping`.

**Design target:** Doppler-shifted spatial multi-path delay off moving objects.

### 8. Output Block (Clipper + Limiter)
**Implemented (`oversampled_clipper.rs` + `lib.rs`):** A soft diode-style clip
with adjustable `analog_ceiling` (default 0.9661 ≈ −0.3 dBFS) and `diode_softness`.
The final output limiter (`apply_output_limiter`) hard-clamps to ±0.9661.
> ❌ **Known defect:** the "oversampling" loop zero-order-holds a single input and
> averages identical clipped copies, so **1×/4×/8×/16× all produce identical
> output** — there is no real oversampling or anti-aliasing. The quality-mode
> factor is therefore inaudible at the clipper. See code review Finding 1.

**Design target:** 16× polyphase upsample → analog diode clip → linear-phase
anti-aliasing decimation, guaranteeing a true-peak ceiling.

---

### Quality modes

`QualityMode` (`Eco`/`Normal`/`High`/`Render`) maps to `PostQualityMode`:

| Mode | Requested oversample | Chain |
|---|---|---|
| Eco | 1× | bypasses FEM body, HRTF spread, Space |
| Normal (default) | 4× | full |
| High | 8× | full |
| Render | 16× | full |

The Eco stage-bypass is real and audible; the oversample factor is currently not
(see defect above).
