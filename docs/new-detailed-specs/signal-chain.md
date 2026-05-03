Rust combined with `nih_plug` is arguably the absolute best modern stack for a synthesizer of this magnitude. `nih_plug` handles all the boilerplate of VST3/CLAP formats effortlessly, letting you focus purely on the DSP architecture, while Rust’s strict memory safety and fearless concurrency will keep those massive FDTD physics simulations from crashing your DAW.

To help you translate the physics into a structured Rust crate, here is the complete macro-architecture signal path. I have formatted this as an ASCII/Markdown flowchart, which is universally readable and serves as an excellent reference comment block for your `lib.rs` or `processor.rs` file.

```text
========================================================================================
                         INDUSTRIAL SYNTH: SIGNAL FLOW ARCHITECTURE
========================================================================================

                                 ┌─────────────────┐
                                 │   MIDI INPUT    │
                                 └────────┬────────┘
                                          │
       ┌──────────────────────────────────┼──────────────────────────────────┐
       │                                  │                                  │
┌──────▼──────┐                    ┌──────▼──────┐                    ┌──────▼──────┐
│  PITCH/KEY  │                    │     GATE    │                    │   VELOCITY  │
└──────┬──────┘                    └──────┬──────┘                    └──────┬──────┘
       │                                  │                                  │
       │                           ┌──────▼──────┐                    ┌──────▼──────┐
       │                           │ 6-STAGE MSEG│                    │ VELOCITY    │
       │                           │ (Looping)   │                    │ MACRO MATRIX│
       │                           └──────┬──────┘                    └──────┬──────┘
       │                                  │                                  │
       └─────────────────────────┐        └──────────┐           ┌───────────┘
                                 │                   │           │
                                 ▼                   ▼           ▼
                         [ MODULATION BUS: Control Rate (GUI/Parameters) ]
                                 │                   │           │
   ┌─────────────────────────────┴───────────────────┴───────────┴─────────────────────┐
   │                                                                                   │
   │                              EXCITER BLOCK                                        │
   │                                                                                   │
   │  Algorithm Selector:                                                              │
   │  [ ] Impact (Hertzian)   [ ] Scrape (Stribeck)   [ ] Specialty (Fluid/Noise)      │
   │                                                                                   │
   │  Internal State: Exciter Position (xh), Exciter Velocity (vh), Mass (mh)          │
   └─────────────────────────────────────┬───────────────────────▲─────────────────────┘
                                Force (F)│                       │ Velocity (vm)
                                         │                       │ Displacement (xm)
   ┌─────────────────────────────────────▼───────────────────────┴─────────────────────┐
   │                              INTERACTION BUS                                      │
   │                                                                                   │
   │  1. Takes Target Pitch and calculates Base Frequencies.                           │
   │  2. Modulates 'Strike Position' (LFO/Env) to calculate Spatial Coefficients (cn). │
   │  3. Distributes Force to Modal Bank scaled by Strike Position.                    │
   │  4. Feeds combined Modal Velocity/Displacement back to Exciter.                   │
   └─────────────────────────────────────┬───────────────────────▲─────────────────────┘
                                Force (F)│                       │
                                         │                       │
   ┌─────────────────────────────────────▼───────────────────────┴─────────────────────┐
   │                              RESONATOR BLOCK                                      │
   │                                                                                   │
   │  [ TRANSFORMATIONS LAYER (Modulates Mode Array Math prior to processing) ]        │
   │  ► Size  ► Rust  ► Damage/Rattle  ► Thickness  ► Heat  ► Sludge                   │
   │                                                                                   │
   │  [ MODAL BANK (Audio Rate) ]                                                      │
   │  Array of N damped harmonic oscillators (Pipe, Plate, Beam, Cog, etc.)            │
   │  Equation: y_n''(t) + 2*d_n*y_n'(t) + ω_n^2*y_n(t) = F_in_n(t)                    │
   │                                                                                   │
   │  (Output is the summed velocities of all N modes)                                 │
   └─────────────────────────────────────┬─────────────────────────────────────────────┘
                                         │ Audio Signal (Monophonic/Raw)
   ┌─────────────────────────────────────▼─────────────────────────────────────────────┐
   │                            POST-PROCESSING BLOCK                                  │
   │                                                                                   │
   │  1. LADDER FILTER: Wave Digital Filter (WDF) / 4-Pole non-linear.                 │
   │          │                                                                        │
   │  2. DRIVE STAGE: Asymmetric Saturation -> Lorenz Chaotic Wavefolder.              │
   │          │                                                                        │
   │  3. BODY RESONATOR: FEM Chassis (adds fixed low-mid 'wood/metal' mass).           │
   │          │                                                                        │
   │  4. STEREO SPREAD: HRTF / Mode Panning (Splits Mono into L/R based on freq).      │
   └─────────────────────────────────────┬─────────────────────────────────────────────┘
                                         │ Audio Signal (Stereo L/R)
   ┌─────────────────────────────────────▼─────────────────────────────────────────────┐
   │                            SPACE & OUTPUT BLOCK                                   │
   │                                                                                   │
   │  1. SPACE MODULE (Selectable):                                                    │
   │     [ ] FDTD Factory Reverb (3D Wave mesh w/ Machinery Diffraction)               │
   │     [ ] FDTD Spring Reverb (Helical 1D PDE solver)                                │
   │     [ ] Doppler Echo (Spatial Multi-path Delay)                                   │
   │          │                                                                        │
   │  2. OVERSAMPLING: Upsample 16x.                                                   │
   │          │                                                                        │
   │  3. ANALOG CLIPPER: True-peak diode soft clipper.                                 │
   │          │                                                                        │
   │  4. DECIMATION: Downsample 16x.                                                   │
   │          │                                                                        │
   │  5. LIMITER: Hard knee at -0.3 dBFS.                                              │
   └─────────────────────────────────────┬─────────────────────────────────────────────┘
                                         │
                                 ┌───────▼───────┐
                                 │   AUDIO OUT   │
                                 └───────────────┘
```

### Implementation Notes for `nih_plug`
When setting this up in Rust, pay attention to these structural boundaries:

1.  **Parameters & GUI Thread:** Everything in the `[MODULATION BUS]` layer should be registered using `nih_plug`'s `Params` trait (e.g., `FloatParam`, `IntParam`).
2.  **Audio Thread (`process` function):** The Exciter, Interaction Bus, and Resonator must sit tightly in a `for sample in buffer.iter_samples()` loop. Because the interaction is bidirectional, you calculate these three blocks on a strict per-sample basis.
3.  **Block Processing:** The effects in the Post-Processing and Space blocks (Filter, Reverb, Clipper) can be calculated using vectorized block processing after the raw synth voices are summed, which will drastically save CPU on polyphonic patches.
