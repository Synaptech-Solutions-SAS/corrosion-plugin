# Corrosion Plugin - Complete Interface Documentation & Design Specification

## Plugin Overview

**Corrosion** is an industrial physical-modeling synthesizer built as a VST3 and CLAP plugin. It turns MIDI gestures into struck, scraped, bent, snapped, groaning, and resonating industrial objects through modal synthesis, interaction modeling, and a post-processing chain that pushes the sound into factory-scale space.

This document is a **product/design handoff spec**, not just a mirror of the current implementation. It combines:

- the currently surfaced controls from `src/params.rs` and `src/gui/editor.rs`
- the authoritative user-facing controls defined in `docs/new-detailed-specs/`
- a full interface design brief for a graphic designer AI

When the current implementation and the long-term spec diverge, both are called out explicitly.

---

## Source of Truth and Status Legend

### Primary Sources

- `docs/new-detailed-specs/exciter-algorithms.md`
- `docs/new-detailed-specs/resonator-algorithms.md`
- `docs/new-detailed-specs/exciter-resonator-interaction.md`
- `docs/new-detailed-specs/transformation-algorithms.md`
- `docs/new-detailed-specs/post-processing.md`
- `src/params.rs`
- `src/gui/editor.rs`

### Status Legend

- **[Surfaced Now]** — currently implemented as a user-facing parameter/control
- **[Spec Defined]** — explicitly defined in the detailed design docs as a user-facing control
- **[Spec Defined, Not Surfaced Yet]** — intended control exists in the spec but is not currently exposed in the implementation/UI

This matters because the design brief should cover the **full intended interface**, not just the subset that is already wired today.

---

## Visual Theme: Industrial Factory Aesthetic

### Core Concept

The interface should feel like standing on a **steel factory floor** beside dangerous, heavy, aging machinery. The UI must communicate **mass, abrasion, heat, instability, corrosion, and mechanical force**. It should never read as a clean consumer-grade virtual synth.

### Color Palette

#### Primary Colors

| Color Name | Hex | RGB | Usage |
|---|---:|---:|---|
| Steel Dark | `#1C1E22` | 28, 30, 34 | Main background, panel fill |
| Steel Mid | `#2D3036` | 45, 48, 54 | Secondary surfaces, graph backgrounds |
| Steel Light | `#444850` | 68, 72, 80 | Inactive controls, slider troughs |
| Steel Highlight | `#5F646E` | 95, 100, 110 | Borders, edges, tick marks |

#### Accent Colors

| Color Name | Hex | RGB | Usage |
|---|---:|---:|---|
| Rust Mid | `#A04628` | 160, 70, 40 | Corrosion, aged metal, scrape/MSEG emphasis |
| Rust Bright | `#E66E3C` | 230, 110, 60 | Logo, active accent, cutoff marker |
| Hazard Yellow | `#DCB428` | 220, 180, 40 | Warnings, macro damage, tolerance/instability hints |
| Warning Orange | `#C87828` | 200, 120, 40 | Exciter emphasis, drive, unsafe energy |
| Danger Red | `#B43C32` | 180, 60, 50 | Damage, destructive force, clipping risk |
| Cool Blue | `#5082AA` | 80, 130, 170 | Technical controls: filter, damping, stereo positioning |

#### Text Colors

| Color Name | Hex | RGB | Usage |
|---|---:|---:|---|
| Text Primary | `#DCDAD7` | 220, 218, 215 | Main labels, values |
| Text Secondary | `#A09E9B` | 160, 158, 155 | Support labels, descriptions |

### Visual Motifs

- brushed steel panels with subtle directional grain
- rust bloom around seams, bolts, and high-friction edges
- hazard stripe accents on destructive or high-risk controls
- industrial graph-paper / oscilloscope styling for visual feedback areas
- control group frames that feel like machine modules bolted to a chassis
- circular metal knobs with arc indicators, not glossy synth knobs
- labels that feel like equipment silkscreen or stamped control plates

### Emotional Tone

- **heavy**
- **dangerous**
- **mechanical**
- **aged but engineered**
- **precise, not luxurious**
- **utility-first, but visually striking**

---

## Layout Structure

### Current Implemented Layout

The current editor is a three-zone layout:

1. **Top-left macro row**
2. **Main center columns**
   - Exciter column
   - Resonator column
3. **Right post-processing column**

### Intended Expanded Layout

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ CORROSION                                                      UI SCALE      │
├──────────────────────────────────────────────────────────────────────────────┤
│ MACROS: Mass | Corrosion | Violence | Damage                                 │
├───────────────────────────────────────┬──────────────────────────────────────┤
│ EXCITER COLUMN                        │ RESONATOR COLUMN                     │
│ - exciter selector                    │ - object selector                    │
│ - shared family controls              │ - shared resonator controls          │
│ - contextual per-exciter panel        │ - contextual per-object panel        │
│ - envelope / gesture controls         │ - transformation controls            │
│ - interaction-bus controls            │ - dynamic strike-position controls   │
├───────────────────────────────────────┴──────────────────────────────────────┤
│ POST-PROCESSING COLUMN                                                     │
│ - filter graph                                                               │
│ - drive                                                                      │
│ - body                                                                       │
│ - stereo spread                                                              │
│ - space mode / echo / reverb                                                 │
│ - output clipper                                                             │
└──────────────────────────────────────────────────────────────────────────────┘
```

### UI Organization Rules

- shared controls should remain visible and stable
- per-exciter and per-resonator controls should swap contextually when the selection changes
- interaction and modulation controls should not be buried; they are central to the instrument identity
- post-processing should read like a master chassis/effects rack rather than just miscellaneous FX

---

## Complete User-Facing Control Inventory

## 1. Global Shared Controls

These are high-level controls that affect the whole instrument or broad sections of it.

| Control | Status | Current Range / UI | Role |
|---|---|---|---|
| `exciter` | [Surfaced Now] | Int 0–16, dropdown | Selects the active exciter algorithm |
| `object` | [Surfaced Now] | Int 0–8, dropdown | Selects the active resonator/object |
| `output` | [Surfaced Now] | Float, 0 to +40 dB gain domain | Final output gain |
| `width` | [Surfaced Now] | Float -2.0 to 3.0 | Stereo width / polarity spread |
| `ui_scale` | [Surfaced Now] | 50%–150% | Editor scaling |

---

## 2. Macro Controls

These are top-row identity controls. In the current implementation they drive secondary parameters, but visually they should feel like primary instrument-defining inputs.

| Control | Status | Current Range | Maps / Intent |
|---|---|---:|---|
| `mass` | [Surfaced Now] | 0.0–2.0 | Macro for object size and object-family selection |
| `corrosion` | [Surfaced Now] | 0.0–2.0 | Macro for rust amount and body coloration |
| `violence` | [Surfaced Now] | 0.0–2.0 | Macro for drive / aggression |
| `damage_macro` | [Surfaced Now] | 0.0–2.0 | Macro for structural damage intensity |

### Macro Personality Notes

- **Mass** should feel like selecting the physical scale and heft of the machine part.
- **Corrosion** should feel like age, oxidation, grime, and compromised surface behavior.
- **Violence** should feel like force, impact energy, motor strain, and unsafe intensity.
- **Damage** should feel like cracks, split modes, loose rattles, and failing metal.

---

## 3. Shared Exciter Controls and Envelope Controls

These are the currently surfaced controls that apply generically across exciter families.

### Shared Exciter Performance Controls

| Control | Status | Current Range | Meaning |
|---|---|---:|---|
| `exciter_pressure` | [Surfaced Now] | 0.0–1.0 | Generic force/pressure macro |
| `exciter_speed` | [Surfaced Now] | 0.0–1.0 | Generic speed/velocity macro |
| `exciter_roughness` | [Surfaced Now] | 0.0–1.0 | Generic roughness/grit macro |

### Hit Envelope Controls

| Control | Status | Current Range | Meaning |
|---|---|---:|---|
| `env_attack` | [Surfaced Now] | 0.001–2.0 s | Attack time for hit/specialty envelope |
| `env_release` | [Surfaced Now] | 0.01–5.0 s | Release time for hit/specialty envelope |

### Specialty ADSR Controls

| Control | Status | Current Range | Meaning |
|---|---|---:|---|
| `env_attack` | [Surfaced Now] | 0.001–2.0 s | Attack time |
| `env_decay` | [Surfaced Now] | 0.01–5.0 s | Decay time |
| `env_sustain` | [Surfaced Now] | 0.0–1.0 | Sustain level |
| `env_release` | [Surfaced Now] | 0.01–5.0 s | Release time |

### Scrape / MSEG Controls

| Control | Status | Current Range | Meaning |
|---|---|---:|---|
| `mseg_onset` | [Surfaced Now] | 0.001–1.0 s | Contact transient |
| `mseg_attack` | [Surfaced Now] | 0.001–2.0 s | Force build-up |
| `mseg_hold` | [Surfaced Now] | 0.0–2.0 s | Hold at peak |
| `mseg_decay` | [Surfaced Now] | 0.01–5.0 s | Fall toward sustain |
| `mseg_sustain` | [Surfaced Now] | 0.0–1.0 | Sustained friction level |
| `mseg_release` | [Surfaced Now] | 0.01–5.0 s | Release time |

### Spec-Defined MSEG Depth Controls Not Yet Surfaced

| Control | Status | Meaning |
|---|---|---|
| `c_onset` | [Spec Defined, Not Surfaced Yet] | Curve tension into onset |
| `c_attack` | [Spec Defined, Not Surfaced Yet] | Curve tension of attack |
| `c_decay` | [Spec Defined, Not Surfaced Yet] | Curve tension of decay |
| `c_release` | [Spec Defined, Not Surfaced Yet] | Curve tension of release |
| `env_amount` | [Spec Defined, Not Surfaced Yet] | Overall modulation amount |
| `velocity_to_peak` | [Spec Defined, Not Surfaced Yet] | Velocity scaling into MSEG peak |
| `loop_mode` | [Spec Defined, Not Surfaced Yet] | Off / Forward / Ping-Pong |
| `loop_start_stage` | [Spec Defined, Not Surfaced Yet] | Loop start node |
| `loop_end_stage` | [Spec Defined, Not Surfaced Yet] | Loop end node |
| `sync_rate` | [Spec Defined, Not Surfaced Yet] | Tempo-synced timing |
| `global_time_scale` | [Spec Defined, Not Surfaced Yet] | Global envelope duration scale |
| `velocity_to_level` | [Spec Defined, Not Surfaced Yet] | Velocity scaling into level |
| `velocity_to_time` | [Spec Defined, Not Surfaced Yet] | Velocity scaling into timing |
| `curve_tension` | [Spec Defined, Not Surfaced Yet] | Global macro over curve behavior |

---

## 4. Per-Exciter Controls

This section corrects the prior omission. The detailed specs define **unique user-facing controls per exciter**. The current implementation mostly exposes generic pressure/speed/roughness, but the intended UI should support the controls below as contextual sub-panels.

### 4.1 Hit Category

#### Hit

| Control | Status | Meaning |
|---|---|---|
| `exciter_pressure` | [Surfaced Now] | Generic hit-force macro |
| `exciter_speed` | [Surfaced Now] | Generic hit-speed macro |
| `exciter_roughness` | [Surfaced Now] | Generic impact texture macro |

This is the simplified fallback hit model, not one of the richer named algorithm panels below.

#### Hand Strike

| Control | Status | Meaning |
|---|---|---|
| `hand_mass` | [Spec Defined, Not Surfaced Yet] | Overall force multiplier |
| `flesh_stiffness` | [Spec Defined, Not Surfaced Yet] | Initial transient stiffness |
| `flesh_damping` | [Spec Defined, Not Surfaced Yet] | High-frequency absorption |
| `mute_decay` | [Spec Defined, Not Surfaced Yet] | Return-to-rest speed; slap vs palm-mute behavior |

#### Felt Mallet

| Control | Status | Meaning |
|---|---|---|
| `mallet_mass` | [Spec Defined, Not Surfaced Yet] | Overall momentum |
| `felt_softness` | [Spec Defined, Not Surfaced Yet] | Low-velocity stiffness / thud softness |
| `core_hardness` | [Spec Defined, Not Surfaced Yet] | High-velocity hardness multiplier |
| `compression_curve` | [Spec Defined, Not Surfaced Yet] | Felt-to-core transition abruptness |

#### Hard Mallet

| Control | Status | Meaning |
|---|---|---|
| `mallet_mass` | [Spec Defined, Not Surfaced Yet] | High-default low-end driving mass |
| `material_stiffness` | [Spec Defined, Not Surfaced Yet] | Strike brightness |
| `impact_damping` | [Spec Defined, Not Surfaced Yet] | Bounce suppression / single-strike cleanliness |

#### Drumstick

| Control | Status | Meaning |
|---|---|---|
| `stick_mass` | [Spec Defined, Not Surfaced Yet] | Stick mass |
| `tip_stiffness` | [Spec Defined, Not Surfaced Yet] | Ping / bite of the tip |
| `restitution_bounciness` | [Spec Defined, Not Surfaced Yet] | Retained energy after rebound |
| `micro_bounce_limit` | [Spec Defined, Not Surfaced Yet] | Maximum bounce count |

#### Wire Brush

| Control | Status | Meaning |
|---|---|---|
| `wire_density` | [Spec Defined, Not Surfaced Yet] | Number of impulses |
| `spread_duration` | [Spec Defined, Not Surfaced Yet] | Cluster time spread |
| `wire_stiffness` | [Spec Defined, Not Surfaced Yet] | High-pass tilt of the cluster |
| `amplitude_randomization` | [Spec Defined, Not Surfaced Yet] | Variance in impulse amplitudes |

#### Metal Pipe

| Control | Status | Meaning |
|---|---|---|
| `pipe_mass` | [Spec Defined, Not Surfaced Yet] | Contact force multiplier |
| `metal_stiffness` | [Spec Defined, Not Surfaced Yet] | Very bright rigid contact stiffness |
| `pipe_pitch` | [Spec Defined, Not Surfaced Yet] | Pitch of the exciter’s internal pipe modes |
| `pipe_ring_decay` | [Spec Defined, Not Surfaced Yet] | Exciter-pipe ringing duration |

#### Metal Chain

| Control | Status | Meaning |
|---|---|---|
| `link_count` | [Spec Defined, Not Surfaced Yet] | Number of distinct link impacts |
| `chain_mass` | [Spec Defined, Not Surfaced Yet] | Weight per link |
| `drop_envelope_spread` | [Spec Defined, Not Surfaced Yet] | Time between first and last impact |
| `internal_rattle` | [Spec Defined, Not Surfaced Yet] | Gain of injected link-grind noise |
| `rattle_color` | [Spec Defined, Not Surfaced Yet] | Spectral color of rattle noise |

### 4.2 Scrape Category

#### Scrape

| Control | Status | Meaning |
|---|---|---|
| `exciter_pressure` | [Surfaced Now] | Generic scrape pressure macro |
| `exciter_speed` | [Surfaced Now] | Generic scrape speed macro |
| `exciter_roughness` | [Surfaced Now] | Generic scrape roughness macro |

This is the simplified shared scrape control surface, distinct from the richer named scrape models below.

#### The Bow

| Control | Status | Meaning |
|---|---|---|
| `bow_pressure` | [Spec Defined, Not Surfaced Yet] | Overall excitation force |
| `bow_speed` | [Spec Defined, Not Surfaced Yet] | Primary driving speed |
| `rosin_grip` | [Spec Defined, Not Surfaced Yet] | Static friction bite |
| `slip_curve` | [Spec Defined, Not Surfaced Yet] | Smoothness of grip-to-slip transition |

#### Stiff Point Scrape

| Control | Status | Meaning |
|---|---|---|
| `scrape_speed` | [Spec Defined, Not Surfaced Yet] | Driving speed |
| `point_pressure` | [Spec Defined, Not Surfaced Yet] | Snap threshold pressure |
| `chatter_pitch` | [Spec Defined, Not Surfaced Yet] | Frequency of squeak/chatter |
| `chatter_damping` | [Spec Defined, Not Surfaced Yet] | Decay of chatter micro-snaps |

#### Heavy Grinding

| Control | Status | Meaning |
|---|---|---|
| `grind_speed` | [Spec Defined, Not Surfaced Yet] | Tearing-noise amplitude driver |
| `grind_pressure` | [Spec Defined, Not Surfaced Yet] | Baseline dragging force |
| `surface_grit` | [Spec Defined, Not Surfaced Yet] | Friction vs tearing ratio |
| `grit_color` | [Spec Defined, Not Surfaced Yet] | Spectral color of the grind noise |

#### Corrugated Drag

| Control | Status | Meaning |
|---|---|---|
| `drag_speed` | [Spec Defined, Not Surfaced Yet] | Bump-rate driver |
| `ridge_spacing` | [Spec Defined, Not Surfaced Yet] | Distance between ridges |
| `ridge_depth` | [Spec Defined, Not Surfaced Yet] | Depth of fall between ridges |
| `exciter_mass` | [Spec Defined, Not Surfaced Yet] | Punch/weight of ridge impacts |

#### Tension Rise

| Control | Status | Meaning |
|---|---|---|
| `pull_speed` | [Spec Defined, Not Surfaced Yet] | Tension accumulation rate |
| `break_threshold` | [Spec Defined, Not Surfaced Yet] | Slip threshold |
| `slip_stochasticity` | [Spec Defined, Not Surfaced Yet] | Random threshold jitter |
| `creak_sharpness` | [Spec Defined, Not Surfaced Yet] | Dull-thud vs sharp-crack tone |

### 4.3 Specialty Category

#### Pneumatic Jet

| Control | Status | Meaning |
|---|---|---|
| `air_pressure` | [Spec Defined, Not Surfaced Yet] | Jet speed / overall intensity |
| `nozzle_width` | [Spec Defined, Not Surfaced Yet] | Narrow whistle vs wide roar |
| `turbulence_chaos` | [Spec Defined, Not Surfaced Yet] | Nonlinear choking/overload behavior |

#### Electromagnetic Hum

| Control | Status | Meaning |
|---|---|---|
| `mains_frequency` | [Spec Defined, Not Surfaced Yet] | Base 50/60 Hz hum frequency |
| `coil_proximity` | [Spec Defined, Not Surfaced Yet] | Magnetic-field proximity / main drive |
| `voltage_sag` | [Spec Defined, Not Surfaced Yet] | Odd-harmonic failing-transformer distortion |

#### Tension Snap

| Control | Status | Meaning |
|---|---|---|
| `pull_distance` | [Spec Defined, Not Surfaced Yet] | Pull excursion before snap |
| `hook_stiffness` | [Spec Defined, Not Surfaced Yet] | Hook stiffness / build-up feel |
| `snap_force` | [Spec Defined, Not Surfaced Yet] | Required breaking force |

#### Particle Rain

| Control | Status | Meaning |
|---|---|---|
| `flow_rate` | [Spec Defined, Not Surfaced Yet] | Density of particle stream |
| `particle_mass` | [Spec Defined, Not Surfaced Yet] | Light sand vs heavy gravel mass |
| `mass_variance` | [Spec Defined, Not Surfaced Yet] | Organic per-hit randomization |

### Exact Shared Exciter Parameter Names Across Spec Panels

- `mallet_mass` appears in both **Felt Mallet** and **Hard Mallet**.
- All other per-exciter parameter names are unique in the current detailed spec.

---

## 5. Shared Resonator Controls

These are the currently surfaced, generic resonator controls.

| Control | Status | Current Range | Meaning |
|---|---|---:|---|
| `res_damping` | [Surfaced Now] | 0.0–1.0 | Global damping/decay character |
| `res_brightness` | [Surfaced Now] | 0.0–1.0 | Global high-frequency emphasis |

These are useful as macros, but they do **not** replace the per-object parameter sets below.

---

## 6. Per-Resonator Controls

### Pipe

| Control | Status | Meaning |
|---|---|---|
| `pipe_length` | [Spec Defined, Not Surfaced Yet] | Core fundamental pitch |
| `tube_diameter` | [Spec Defined, Not Surfaced Yet] | Inharmonicity / bell-vs-chime stiffness |
| `sustain_time` | [Spec Defined, Not Surfaced Yet] | Global ringing time |

### Plate

| Control | Status | Meaning |
|---|---|---|
| `plate_size` | [Spec Defined, Not Surfaced Yet] | Overall modal cluster scaling |
| `aspect_ratio` | [Spec Defined, Not Surfaced Yet] | Inharmonic flavor / rectangular geometry |
| `metal_stiffness` | [Spec Defined, Not Surfaced Yet] | Brightness and modal spacing |

### Tank

| Control | Status | Meaning |
|---|---|---|
| `tank_volume` | [Spec Defined, Not Surfaced Yet] | Depth of cavity boom |
| `wall_thickness` | [Spec Defined, Not Surfaced Yet] | Sustain vs dullness |
| `cavity_mix` | [Spec Defined, Not Surfaced Yet] | Air mode vs shell mode balance |

### Chain

| Control | Status | Meaning |
|---|---|---|
| `link_mass` | [Spec Defined, Not Surfaced Yet] | Base frequency region of the modal cluster |
| `chain_length` | [Spec Defined, Not Surfaced Yet] | Total number of active modes |
| `instability` | [Spec Defined, Not Surfaced Yet] | Chaotic coupling strength |
| `friction_decay` | [Spec Defined, Not Surfaced Yet] | Short, choppy chain decay control |

Note: the source doc appears text-corrupted around `instability`, but it is clearly intended as the chaotic coupling control for the chain object.

### I-Beam

| Control | Status | Meaning |
|---|---|---|
| `beam_mass` | [Spec Defined, Not Surfaced Yet] | Fundamental/sub-bass positioning |
| `shear_density` | [Spec Defined, Not Surfaced Yet] | High-mode compression |
| `rigidity_damping` | [Spec Defined, Not Surfaced Yet] | Fast high-frequency decay / thud emphasis |

### Taut Cable

| Control | Status | Meaning |
|---|---|---|
| `cable_tension` | [Spec Defined, Not Surfaced Yet] | Base tuning |
| `braid_stiffness` | [Spec Defined, Not Surfaced Yet] | Sharpness of upper harmonics |
| `tension_drop` | [Spec Defined, Not Surfaced Yet] | Downward pitch envelope after strike |

### Heavy Coil Spring

| Control | Status | Meaning |
|---|---|---|
| `coil_length` | [Spec Defined, Not Surfaced Yet] | Lowest resonant thud |
| `dispersion_chirp` | [Spec Defined, Not Surfaced Yet] | Severity of the “pew” transient |
| `spring_slosh` | [Spec Defined, Not Surfaced Yet] | Chaotic detune / metallic reverberation |

### Sheet Metal

| Control | Status | Meaning |
|---|---|---|
| `sheet_size` | [Spec Defined, Not Surfaced Yet] | Overall frequency footprint |
| `metal_thinness` | [Spec Defined, Not Surfaced Yet] | Buckling intensity |
| `edge_damping` | [Spec Defined, Not Surfaced Yet] | Free edge wash vs choked crash |

### Industrial Cog

| Control | Status | Meaning |
|---|---|---|
| `blade_radius` | [Spec Defined, Not Surfaced Yet] | Overall pitch/tuning |
| `tooth_dissonance` | [Spec Defined, Not Surfaced Yet] | Mode-splitting imperfection |
| `blade_thickness` | [Spec Defined, Not Surfaced Yet] | High-vs-mid energy emphasis |
| `friction_decay` | [Spec Defined, Not Surfaced Yet] | Shortens decay to avoid bell-like ringing |

### Exact Shared Resonator Parameter Names Across Spec Panels

- No exact duplicates were defined across the resonator/object parameter sets.

---

## 7. Interaction-Bus Controls

These are core physical-modeling controls from `exciter-resonator-interaction.md`. They are easy to miss in a generic synth UI, but they are central to Corrosion’s identity.

| Control | Status | Meaning |
|---|---|---|
| `strike_position` | [Spec Defined, Not Surfaced Yet] | Base physical contact position on the object |
| `coupling_stiffness` | [Spec Defined, Not Surfaced Yet] | 0% = feed-forward shortcut, 100% = fully coupled physical interaction |
| `position_wander` | [Spec Defined, Not Surfaced Yet] | Slow random/LFO movement of the strike point |
| `position_envelope` | [Spec Defined, Not Surfaced Yet] | Envelope-driven strike-position sweep |
| `fundamental_anchor` | [Spec Defined, Not Surfaced Yet] | Prevents the fundamental from disappearing at modal nodes |

### Design Note

These controls deserve either:

- a dedicated **Interaction** subsection under the Exciter/Resonator columns, or
- a compact **Physical Coupling** module placed between those two columns.

---

## 8. Transformation Controls

### Currently Surfaced Transformations

| Control | Status | Current Range | Meaning |
|---|---|---:|---|
| `size` | [Surfaced Now] | 0.05–10.0 | Macro-geometry / global pitch and decay scaling |
| `rust` | [Surfaced Now] | 0.0–5.0 | Surface oxidation / high-frequency decay |
| `damage` | [Surfaced Now] | 0.0–10.0 | Structural compromise / split modes + rattle |

### Additional Spec-Defined Transformations Not Yet Surfaced

| Control | Status | Meaning |
|---|---|---|
| `thickness` | [Spec Defined, Not Surfaced Yet] | Material gauge / stiffness spacing |
| `heat` | [Spec Defined, Not Surfaced Yet] | Thermal expansion, pitch wander, softened attacks |
| `sludge` | [Spec Defined, Not Surfaced Yet] | Mass loading, muffling, viscous damping |

### Velocity Macro Matrix Controls

The transformation spec also implies a higher-level velocity mapping layer.

| Control | Status | Meaning |
|---|---|---|
| `velocity_to_force` | [Spec Defined, Not Surfaced Yet] | Nonlinear velocity scaling of output force |
| `velocity_to_brightness` | [Spec Defined, Not Surfaced Yet] | Velocity scaling into exciter stiffness / brightness |
| `velocity_to_damage_rattle` | [Spec Defined, Not Surfaced Yet] | Velocity scaling into rattle threshold behavior |
| `velocity_to_excitation_decay` | [Spec Defined, Not Surfaced Yet] | Velocity scaling of strike decay behavior |

---

## 9. Post-Processing Controls

This section combines what is currently exposed and what the long-form post-processing spec defines.

### 9.1 WDF Filter

| Control | Status | Current Range | Meaning |
|---|---|---:|---|
| `filter_cutoff` | [Surfaced Now] | 20–20000 Hz | Filter cutoff |
| `filter_resonance` | [Surfaced Now] | 0.0–1.0 | Cutoff-region emphasis / self-oscillation tendency |
| `component_tolerance` | [Surfaced Now] | 0.0–1.0 | Micro-variation in virtual analog components |

### Filter Visual Feedback

The current UI already includes a compact filter graph. The intended design should preserve and elevate it:

- logarithmic frequency axis from **20 Hz** to **20 kHz**
- visible cutoff marker
- visible resonance peak marker
- visible component-tolerance instability hint
- graph styling that feels like industrial measurement equipment, not a soft consumer EQ widget

### 9.2 Drive

| Control | Status | Current Range | Meaning |
|---|---|---:|---|
| `drive_amount` | [Surfaced Now] | 0.0–5.0 | Input drive into Lorenz/tube stage |
| `bias_starvation` | [Surfaced Now] | 0.0–1.0 | Sputtering / unstable power behavior |
| `chaos_depth` | [Surfaced Now] | 0.0–1.0 | Bifurcation depth of the Lorenz-style chaos stage |

### 9.3 Stereo Spread / Body

| Control | Status | Current Range | Meaning |
|---|---|---:|---|
| `spread_width` | [Surfaced Now] | 0.0–1.0 | Spatial spread of emitted modes |
| `listener_proximity` | [Surfaced Now] | 0.0–1.0 | Listener distance |
| `chassis_material` | [Surfaced Now] | 0.0–1.0 | Body/chassis material morph |
| `chassis_volume` | [Surfaced Now] | 0.0–1.0 | Body/chassis size or density scale |

### 9.4 Space / Reverb / Echo

#### Currently Surfaced

| Control | Status | Current Range | Meaning |
|---|---|---:|---|
| `space_mode` | [Surfaced Now] | Off / Factory / Spring | Selects the active space model |
| `space_amount` | [Surfaced Now] | 0.0–1.0 | Wet amount |
| `factory_size` | [Surfaced Now] | 0.0–1.0 | Factory reverb scale |
| `machinery_clutter` | [Surfaced Now] | 0.0–1.0 | Obstacle density / diffraction complexity |
| `wall_impedance` | [Surfaced Now] | 0.0–1.0 | Wall reflectivity |
| `spring_tension` | [Surfaced Now] | 0.0–1.0 | Spring wave speed |
| `wire_stiffness` | [Surfaced Now] | 0.0–1.0 | Spring dispersion |
| `spring_tank_size` | [Surfaced Now] | 0.0–1.0 | Spring length / delay scale |

#### Additional Spec-Defined Echo Controls Not Yet Surfaced

| Control | Status | Meaning |
|---|---|---|
| `delay_time` | [Spec Defined, Not Surfaced Yet] | Base reflection time for factory echo |
| `machinery_movement` | [Spec Defined, Not Surfaced Yet] | Doppler/position modulation of moving reflectors |
| `high_frequency_damping` | [Spec Defined, Not Surfaced Yet] | Air-loss darkening of echoes |

### 9.5 Output Stage

| Control | Status | Current Range | Meaning |
|---|---|---:|---|
| `analog_ceiling` | [Surfaced Now] | 0.5–1.0 | Maximum peak output |
| `diode_softness` | [Surfaced Now] | 0.0–1.0 | Softness of clipper knee |
| `output` | [Surfaced Now] | gain domain | Final gain |
| `width` | [Surfaced Now] | -2.0–3.0 | Stereo width / inversion spread |

---

## 10. Current Implementation vs Full Intended Product Surface

### Currently Surfaced in Code and UI

Based on `src/params.rs` and `src/gui/editor.rs`, the currently exposed UI surface includes:

- macro row: `mass`, `corrosion`, `violence`, `damage_macro`
- selectors: `exciter`, `object`, `space_mode`
- shared exciter controls: `exciter_pressure`, `exciter_speed`, `exciter_roughness`
- shared resonator controls: `res_damping`, `res_brightness`
- basic transformations: `size`, `rust`, `damage`
- envelope controls: AR, ADSR, basic 6-stage MSEG timings/levels
- post: filter, drive, body, spread, factory/spring space, clipper, output
- filter graph visualization

### Not Yet Surfaced but Explicitly Required by the Spec

The following control families are still missing from the actual surfaced UI:

- contextual per-exciter parameter panels
- contextual per-resonator parameter panels
- interaction-bus controls
- dynamic strike-position modulation controls
- extended MSEG modulation controls
- transformation additions: `thickness`, `heat`, `sludge`
- velocity macro matrix controls
- factory echo controls

### Product Design Recommendation

The designer should treat the current UI as **phase-one scaffolding**, not the finished control surface. The finished interface should support contextual deep-edit panels for both exciter and resonator models.

---

## 11. Interface Module Recommendations

### Exciter Module

Should be split into three layers:

1. **Exciter selector**
2. **Shared gesture controls**
3. **Contextual model-specific parameter panel**

### Resonator Module

Should be split into three layers:

1. **Object selector**
2. **Shared resonator shaping**
3. **Contextual object-specific parameter panel**

### Interaction Module

Should exist as its own visible subsystem, because it is the center of the instrument’s identity. It should expose:

- strike position
- coupling stiffness
- dynamic position modulation
- fundamental anchoring

### Post-Processing Module

Should feel like a physically separate master rack:

- filter subsection with graph
- drive subsection
- body/spread subsection
- space subsection
- output protection subsection

---

## 12. Graphic Designer AI Prompt

### Context

You are designing the user interface for **Corrosion**, an industrial physical-modeling synthesizer plugin. This is not a glossy EDM synth and not a generic subtractive workstation. It is an instrument for metallic strikes, scrapes, groans, rattles, snapping wires, industrial hum, chaotic modal resonance, and large post-processing space.

Your task is to design a high-fidelity plugin interface mockup that treats the instrument like a **dangerous machine panel** in a steel factory.

### Core Art Direction

The interface should feel like:

- worn industrial equipment
- brushed steel housing
- bolted panel modules
- rust on seams and corners
- warning-painted controls
- technical measurement displays
- analog machinery with digital precision overlays

It should feel **heavy**, **dangerous**, **mechanical**, **aged**, and **purpose-built**.

### Color System

Use these exact anchor colors:

- Steel Dark `#1C1E22`
- Steel Mid `#2D3036`
- Steel Light `#444850`
- Steel Highlight `#5F646E`
- Rust Mid `#A04628`
- Rust Bright `#E66E3C`
- Hazard Yellow `#DCB428`
- Warning Orange `#C87828`
- Danger Red `#B43C32`
- Cool Blue `#5082AA`
- Text Primary `#DCDAD7`
- Text Secondary `#A09E9B`

### Global Layout

Design the interface as:

1. **Header strip** with brand and scale options
2. **Macro strip** across the upper-left portion
3. **Exciter column** on the left-center
4. **Resonator column** on the center-right
5. **Post-processing master column** on the far right

The layout must support both **currently implemented controls** and **future contextual sub-panels** for the deeper spec-defined controls.

### Macro Row Requirements

Create four large industrial knobs:

- **Mass**
- **Corrosion**
- **Violence**
- **Damage**

Each knob should have:

- machined circular metal body
- heavy arc indicator
- subtle grime/wear
- value readout embedded in the center
- strong active glow when adjusted

The four knobs should each have a distinct personality:

- Mass = steel / gravity / load-bearing
- Corrosion = rust / oxidation / age
- Violence = unsafe force / impact / red-orange hazard
- Damage = split metal / broken structure / yellow-black warning language

### Exciter Column Requirements

The exciter column should support both shared and contextual controls.

Always-visible controls:

- exciter selector
- shared pressure/speed/roughness controls
- envelope section that changes by family

Contextual sub-panel area beneath the selector must be designed to swap between parameter sets such as:

- Hand Strike: `hand_mass`, `flesh_stiffness`, `flesh_damping`, `mute_decay`
- Felt Mallet: `mallet_mass`, `felt_softness`, `core_hardness`, `compression_curve`
- Drumstick: `stick_mass`, `tip_stiffness`, `restitution_bounciness`, `micro_bounce_limit`
- Wire Brush: `wire_density`, `spread_duration`, `wire_stiffness`, `amplitude_randomization`
- Metal Pipe: `pipe_mass`, `metal_stiffness`, `pipe_pitch`, `pipe_ring_decay`
- Metal Chain: `link_count`, `chain_mass`, `drop_envelope_spread`, `internal_rattle`, `rattle_color`
- Bow: `bow_pressure`, `bow_speed`, `rosin_grip`, `slip_curve`
- Stiff Point: `scrape_speed`, `point_pressure`, `chatter_pitch`, `chatter_damping`
- Heavy Grinding: `grind_speed`, `grind_pressure`, `surface_grit`, `grit_color`
- Corrugated Drag: `drag_speed`, `ridge_spacing`, `ridge_depth`, `exciter_mass`
- Tension Rise: `pull_speed`, `break_threshold`, `slip_stochasticity`, `creak_sharpness`
- Pneumatic Jet: `air_pressure`, `nozzle_width`, `turbulence_chaos`
- Electromagnetic Hum: `mains_frequency`, `coil_proximity`, `voltage_sag`
- Tension Snap: `pull_distance`, `hook_stiffness`, `snap_force`
- Particle Rain: `flow_rate`, `particle_mass`, `mass_variance`

The design must make it obvious that each exciter is a different **physical machine behavior**, not just a preset variation.

### Envelope / Gesture Area

The envelope section should visually switch between:

- **AR** for hits
- **6-stage MSEG** for scrapes
- **ADSR** for specialty exciters

The UI should also reserve space for future deep MSEG controls:

- loop mode
- per-stage curve tension
- velocity scaling
- global time scaling

This should feel more like a **gesture shaper** than a generic synth envelope.

### Resonator Column Requirements

Always-visible controls:

- object selector
- shared damping / brightness
- transformation block

Contextual per-object panel area must support:

- Pipe: `pipe_length`, `tube_diameter`, `sustain_time`
- Plate: `plate_size`, `aspect_ratio`, `metal_stiffness`
- Tank: `tank_volume`, `wall_thickness`, `cavity_mix`
- Chain: `link_mass`, `chain_length`, `instability`, `friction_decay`
- I-Beam: `beam_mass`, `shear_density`, `rigidity_damping`
- Taut Cable: `cable_tension`, `braid_stiffness`, `tension_drop`
- Heavy Coil Spring: `coil_length`, `dispersion_chirp`, `spring_slosh`
- Sheet Metal: `sheet_size`, `metal_thinness`, `edge_damping`
- Industrial Cog: `blade_radius`, `tooth_dissonance`, `blade_thickness`, `friction_decay`

These controls should feel like **engineering dimensions**, not abstract synth macros.

### Interaction-Bus Module

Include a dedicated module or bridge zone between exciter and resonator for:

- `strike_position`
- `coupling_stiffness`
- `position_wander`
- `position_envelope`
- `fundamental_anchor`

This section should visually communicate that the exciter and resonator are physically coupled.

Good visual motifs:

- pipe or cable links between panels
- moving strike marker over a simplified object diagram
- node/antinode visualization

### Transformation Module

Do not stop at Size / Rust / Damage. Design the transformation zone so it can support:

- `size`
- `rust`
- `damage`
- `thickness`
- `heat`
- `sludge`

These should read like **material-state controls**.

Suggested visual metaphors:

- thickness = gauge / plate thickness indicator
- heat = glow / thermal bloom / warble accent
- sludge = viscous smear / oil-darkened contamination

### Post-Processing Column Requirements

This column should feel like a **master processing rack**.

#### WDF Filter Section

Must include a visible graph with:

- log frequency range 20 Hz to 20 kHz
- response curve
- resonance peak marker
- cutoff marker
- tolerance instability artifacts

Include controls for:

- `filter_cutoff`
- `filter_resonance`
- `component_tolerance`

#### Lorenz Drive Section

Include controls for:

- `drive_amount`
- `bias_starvation`
- `chaos_depth`

The visual styling should suggest unstable analog circuitry, starving voltage, and nonlinear chaos.

#### Body / Spread Section

Include controls for:

- `chassis_material`
- `chassis_volume`
- `spread_width`
- `listener_proximity`

This should feel like positioning a resonating machine body in front of a listener.

#### Space Section

Include current and planned modes:

- Off
- Factory Reverb
- Spring Reverb
- Factory Echo

Controls to design for:

- `space_mode`
- `space_amount`
- `factory_size`
- `machinery_clutter`
- `wall_impedance`
- `spring_tension`
- `wire_stiffness`
- `spring_tank_size`
- `delay_time`
- `machinery_movement`
- `high_frequency_damping`

#### Output Section

Include controls for:

- `analog_ceiling`
- `diode_softness`
- `output`
- `width`

This area should feel like the final safety/output stage of dangerous industrial equipment.

### Texture and Material Requirements

Use:

- brushed steel grain on major panel surfaces
- rust deposits around worn edges
- bolts or rivets at major corners
- subtle caution-strip accents on dangerous modules
- engraved, stamped, or silkscreened control labels
- occasional heat staining or oil-darkened regions where appropriate

Avoid:

- glossy plastic synth aesthetics
- neon cyberpunk styling
- soft toy-like rounding
- generic DAW plugin minimalism

### Typography

- headers: bold industrial sans-serif or slightly condensed machinery-style lettering
- labels: clear technical sans-serif
- values: tabular numerals / mono-friendly digits
- warnings: stencil-inspired or equipment-plate style treatment

### Interaction States

- hover: slight brightness lift or subtle steel edge glow
- active drag: stronger illumination and accent color intensification
- disabled / unsurfaced / future panel slots: visibly muted, but structurally present where needed

### Final Output Request

Produce a high-fidelity interface mockup at approximately **1000–1100 px width** and **700–800 px height** showing:

- macro row
- exciter column
- resonator column
- interaction-bus region
- post-processing column
- filter graph
- one example contextual exciter panel
- one example contextual resonator panel

The final design should look like a real product for a premium industrial sound-design instrument: **mechanically credible, visually dangerous, highly legible, and unmistakably metallic**.
