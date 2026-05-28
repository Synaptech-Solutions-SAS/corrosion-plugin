# Corrosion — Work Backlog (missing, partial, and broken)

A single prioritized list of what needs solving, derived from the 2026-05 code
review (`docs/code-review.md`) and the as-built architecture
(`docs/ARCHITECTURE.md`). Scope excludes `src/presets/`.

Status key: **[BUG]** broken behavior · **[PARTIAL]** wired but incomplete ·
**[MISSING]** specced but absent · **[CHORE]** cleanup/quality · **[VERIFY]** not
yet read in review, confirm before acting.

Each item notes the relevant file(s) and an acceptance check.

---

## P0 — Correctness defects (fix first)

- [x] **[BUG] Oversampled clipper does not oversample.** *(fixed 2026-05-27)*
  `src/dsp/post_processing/oversampled_clipper.rs`. `process()` zero-order-holds a
  single input (`os_samples.fill(input)`) and averages identical clipped copies, so
  1×/4×/8×/16× are mathematically identical. No interpolation, no anti-aliasing,
  `upsample_state`/`downsample_state` unused.
  *Fix:* real upsampling (polyphase, or at least linear interp across prev/next
  input) → clip at high rate → low-pass decimate.
  *Accept:* Eco vs Render differ at the clipper; `analyze_post_chain_aliasing`
  shows lower `alias_ratio_db` at higher factors; re-base `quality_mode_changes_oversample_factor`
  to exercise the factor (not the Eco bypass).

- [x] **[BUG] FactoryReverb comb delays mutate cumulatively.** *(fixed 2026-05-27)*
  `src/dsp/post_processing/space.rs::update_delays` re-scales `comb_delays` from
  their own prior values, and `lib.rs` calls it every sample → delays rail to the
  clamp bounds; `factory_size` is effectively dead.
  *Fix:* keep immutable base delays and recompute `comb_delays = base · size_scale`
  each update (mirror `SpringReverb::update_delay`).
  *Accept:* sweeping `factory_size` changes the tail length monotonically; add a test.

- [x] **[BUG/PERF] Post-chain parameters set per sample.** *(fixed 2026-05-27)*
  `src/lib.rs::process` calls ~10 `post_chain.set_*` setters inside the sample loop;
  several recompute coefficients (`tan()`, FEM freqs, reverb delays) at audio rate.
  Root cause of the FactoryReverb bug and wasted CPU.
  *Fix:* apply setters once per buffer (or only on change); keep coefficient
  recompute out of the per-sample loop.
  *Accept:* `post_processing_chain` benchmark improves; FactoryReverb fix holds.

---

## Approved change — Algorithmic resonator engine (decided 2026-05, IMPLEMENTED 2026-05-27)

Decision (with sign-off): promote the per-object **algorithmic** resonator path to
the **only** path, remove the `complex_algo` toggle, and expose a curated set of
per-object "character" parameters. Profile tables (`src/dsp/profiles/`) are kept as
`mode_count` / budget / test metadata only — they stop driving sound. The default
timbre of every object will change (accepted). See `docs/ARCHITECTURE.md` §5 and
`docs/detailed-specs/resonator-algorithms.md` for the design.

*Implemented 2026-05-27: algorithmic path is now the sole resonator engine,
`complex_algo` is gone, all 14 character params are exposed and threaded, Chain/Tank
pitch-track the note, and the Cable/Sheet dynamic hooks run per sample (verified
allocation-free by `tests/no_alloc.rs`). All checks below pass.*

- [x] **[CHORE] Remove `complex_algo`.** Dropped the param + `complex_algo_param`
  (`src/params.rs`), the `VoiceControls` field + the `if controls.complex_algo != 0`
  branch (`src/voice/mod.rs`), the snapshot (`src/lib.rs`), and the GUI
  toggle (`src/gui/editor.rs`). Resonator construction is now unconditionally
  `with_algorithm_controls_and_note`.

- [x] **[MISSING] Expose 14 curated per-object character params** and thread them
  `CorrosionParams` → `VoiceControls` → `generate_algorithm_modes` (replacing the
  `::default()` generator construction in `src/dsp/resonators/core.rs:178`):

  | Object | UI name | id | range | default | generator field |
  |---|---|---|---:|---:|---|
  | Pipe | Pipe Diameter | `pipe_diameter` | 0–1 | 0.5 | `tube_diameter` |
  | Plate | Plate Aspect | `plate_aspect` | 0.1–4.0 | 1.0 | `aspect_ratio` |
  | Plate | Plate Stiffness | `plate_stiffness` | 0.25–3.0 | 1.0 | `metal_stiffness` * |
  | Tank | Tank Volume | `tank_volume` | 0–1 | 0.5 | `tank_volume` |
  | Tank | Cavity Mix | `tank_cavity_mix` | 0–1 | 0.6 | `cavity_mix` |
  | Chain | Link Mass | `chain_link_mass` | 0.1–1.0 | 0.5 | `link_mass` |
  | Chain | Instability | `chain_instability` | 0–1 | 0.3 | `instability` |
  | IBeam | Shear Density | `beam_shear` | 0–1 | 0.5 | `shear_density` |
  | TautCable | Braid Stiffness | `cable_braid` | 0–1 | 0.3 | `braid_stiffness` |
  | TautCable | Tension Drop | `cable_tension_drop` | 0–1 | 0.4 | `tension_drop` |
  | CoilSpring | Dispersion Chirp | `spring_dispersion` | 0–1 | 0.5 | `dispersion_chirp` |
  | CoilSpring | Spring Slosh | `spring_slosh` | 0–1 | 0.3 | `spring_slosh` |
  | SheetMetal | Metal Thinness | `sheet_thinness` | 0–1 | 0.4 | `metal_thinness` |
  | IndustrialCog | Tooth Dissonance | `cog_dissonance` | 0–1 | 0.1 | `tooth_dissonance` |

  \* `metal_stiffness` is already the Metal Pipe **exciter** param id, so the Plate
  control ships under id `plate_stiffness`. Dropped as redundant with global
  Size/Damping/Brightness: `sustain_time`, `wall_thickness`, `friction_decay`,
  `chain_length`, `beam_mass`, `rigidity_damping`, `cable_tension`, `coil_length`,
  `sheet_size`, `edge_damping`, `blade_radius`, `blade_thickness`.

- [x] **[BUG] Fix algorithmic pitch tracking.** `ChainResonator::generate_modes`
  now anchors its GOE cluster to `fundamental_hz` (heavier `link_mass` shifts it
  lower) and `TankResonator`'s cavity mode follows the note as a deep sub-octave
  (`basic.rs`); both track the MIDI note.

- [x] **[MISSING] Wire the dynamic hooks** into the per-sample resonator loop
  (`ModalResonator::process_sample[_stereo]`): the cable tension-drop
  (amplitude→pitch "boing") and sheet-metal buckling warp now run every sample.
  The resonator retains the relevant algorithm instance (`Dynamics` enum) plus a
  smoothed amplitude / low-frequency estimate, recomputing affected mode
  coefficients from each mode's immutable base frequency (no cumulative drift).
  Verified allocation-free by `tests/no_alloc.rs`.

- [x] **[CHORE] Preset migration.** A legacy `complex_algo` field is ignored on
  load (serde drops the unknown key); the 14 new fields carry serde defaults so
  existing presets still open (`src/presets/mod.rs`). Covered by
  `legacy_complex_algo_field_is_ignored_on_load`.

- [x] **[CHORE] Profiles → metadata only.** `*_MODAL_PROFILE_MODES` now feed only
  `mode_count` (via `ModalProfile::from_id(..).mode_count()` in
  `generate_algorithm_modes`); they no longer drive sound generation.

- [x] **[CHORE] Tests & docs.** Replaced `complex_algo_toggle_changes_resonator_output`
  with `character_param_changes_resonator_output`, added `chain_resonator_tracks_pitch`
  and `taut_cable_dynamic_hook_changes_output` (`src/voice/mod.rs`); updated
  `tests/preset_roundtrip.rs` (character-param roundtrip + legacy-field migration);
  reconciled docs (`ARCHITECTURE.md` §5, `resonator-algorithms.md`).

> **Interaction with P1 "held-note automation":** these new params are snapshotted
> at note-on like every other voice control, so they will not update during a
> sustained note until that separate limitation is addressed. The Cable/Sheet
> dynamic hooks above *do* update per sample because they are driven by the
> resonator's own running amplitude, not by parameter automation.

---

## P1 — Partially implemented (finish or formally scope down)

- [ ] **[PARTIAL] Quality modes are shallow.**
  Eco bypasses body/spread/space via a fixed `0.3/0.7` mix; oversample factor is
  broken (see P0); resonator mode count is **not** scaled by quality (fixed ~6–12
  modes per profile). Decide: scale mode count per quality, or document mode count
  as intentionally fixed. `src/params.rs`, `src/dsp/post_processing/post_chain.rs`,
  `src/dsp/resonators/core.rs`.

- [ ] **[PARTIAL] Aliasing measurement is a proxy, with no budget.**
  `offline::analyze_post_chain_aliasing` reports residual energy but there is no
  asserted threshold and no oversampled-reference comparison. Tie to the clipper
  fix, then add a regression test that fails if `alias_ratio_db` regresses.

- [ ] **[PARTIAL] Held-note automation has no effect.**
  `VoiceControls` is snapshotted at note-on (`src/lib.rs`, `src/voice/mod.rs`), so
  changing exciter/resonator/transform parameters during a sustained note does
  nothing until the next note. Fine for impacts, limiting for drones/friction and
  for the PRD's "automate all major parameters" goal.
  *Fix:* push a subset of controls (damping, brightness, strike position, drive)
  to live per-sample/per-block updates for sustaining voices.

- [ ] **[PARTIAL] No parameter smoothing.**
  Neither NIH-plug smoothing nor internal smoothing is applied to audibly-stepping
  params (filter cutoff, drive, strike position). Rapid automation can zipper.
  *Fix:* add smoothing on the post-chain controls and strike position.

- [ ] **[PARTIAL] MSEG is not a routable modulation source.**
  `src/dsp/envelopes/mod.rs` MSEG only drives the force envelope for friction
  voices. The specs/master-prompt describe a fully-exposed, routable MSEG + mod
  matrix. Currently the only "modulation" is `position_wander`.

- [ ] **[PARTIAL] `sync_rate` parameter appears unused.**
  `src/params.rs` defines `sync_rate`, but no host-tempo/transport sync path was
  found. Either wire MSEG/echo to host BPM or remove the control. **[VERIFY]**

- [ ] **[PARTIAL] Post/space DSP are approximations of the detailed specs.**
  WDF (4-pole ladder, not a circuit solver), FEM body (8 modes), HRTF (delay+filter,
  no HRIR), Factory/Spring reverbs (Schroeder/single-delay, not FDTD/PDE). Either
  upgrade toward `detailed-specs/post-processing.md` (large effort, watch CPU) or
  keep the now-corrected spec banners as the contract. Decide intent.

---

## P2 — Missing features (specced in PRD/master-prompt, not present)

- [ ] **[MISSING] MIDI expression beyond note on/off.**
  `src/lib.rs::handle_note_event` only handles `NoteOn`/`NoteOff`. No channel
  pressure, poly pressure, mod wheel, or pitch bend (PRD §13.4–13.5).
- [ ] **[MISSING] Macro controls** (Mass, Corrosion, Violence, Damage, etc.; PRD §12).
  Only the `Body` macro partially maps; no macro layer exists.
- [ ] **[MISSING] Sequencer + per-step locks** (PRD §18) — no sequencer module.
- [ ] **[MISSING] Percussion Kit mode** (note→object mapping; PRD §9.2).
- [ ] **[MISSING] Explicit Drone mode** (PRD §9.3). Friction voices sustain via MSEG
  loop, but there is no dedicated sustained/drone mode.
- [ ] **[MISSING] Effect-mode variant** (audio-in excites the resonator; PRD §9.4).
- [ ] **[MISSING] Output options:** lookahead limiter / output meter / selectable
  soft-clip modes (PRD §20.3 future).
- [ ] **[MISSING] Preset schema migration** (versioned remap of old IDs; currently
  sanitize-and-clamp only). *(Presets module excluded from this review — flagged for awareness.)*

---

## P3 — Code quality / cleanup chores

- [ ] **[CHORE] Remove dead code.**
  `Voice::process_sample_stereo` `exciter_type == 0` branch (unreachable; ids are 1–16);
  `OversampledClipper::{upsample_state, downsample_state}` (unused);
  `ModalResonator::{last_output, current_output}` (written, never read);
  unused interaction helpers (`distribute_force_to_modes`, `update_resonator_state`,
  `distribute_force`, `per_mode_forces`, `mode_coefficient_2d`, `mode_coefficient_circular`).
- [ ] **[CHORE] Fix `src/lib.rs` docstring "17 different exciter types" → 16.**
- [ ] **[CHORE] Fix `src/params.rs` docstring "70+ parameters" → 127.**
- [ ] **[CHORE] Declare `render_presets` in `Cargo.toml`** (currently only auto-discovered).
- [ ] **[CHORE] Rename/reword the "WDF" filter** to reflect that it is a ladder
  approximation, not a Newton-Raphson WDF circuit.
- [ ] **[CHORE] Bound `midi_to_hz`** input (cosmetic robustness).
- [ ] **[CHORE] Extract DSP magic numbers** (drive thresholds in `lib.rs::apply_drive`,
  body/reverb constants) into named constants where it aids readability.

---

## P4 — Testing & release infrastructure

- [ ] **[MISSING] Aliasing/spectral regression thresholds** (harness exists; no asserted budget — pair with P0 clipper fix).
- [ ] **[MISSING] Windows CI lane / bundle smoke** (CI is Linux-only; `bundle-win.sh` untested in automation).
- [ ] **[MISSING] DAW smoke automation** (REAPER/Bitwig/Ardour/Live/FL) — currently manual.
- [ ] **[MISSING] GUI interaction/visual regression tests** (headless only today).
- [ ] **[CHORE] Persist benchmark baselines** so Criterion runs can flag regressions.
- [ ] **[CHORE] `assert_process_allocs` is disabled on Windows** (`Cargo.toml` target cfg) — document or enable.

---

## Needs verification before estimating (not read in this review)

- [ ] **[VERIFY] `src/dsp/envelopes/mod.rs`** — MSEG loop / ping-pong correctness
  (params docstring calls PingPong "partial").
- [ ] **[VERIFY] `src/gui/editor.rs`** — quality-mode selector wiring, knob drag,
  preset apply path.
- [ ] **[VERIFY] `src/randomizer/mod.rs`** — that randomization respects the safe
  ranges / constraints in PRD §17.3.
- [ ] **[VERIFY] Per-exciter DSP** for all 16 models and the algorithmic resonators
  (`coil_spring`, `ibeam`, `industrial_cog`, `sheet_metal`, `taut_cable`) — review
  trusted dispatch + docstrings, not every formula.

---

## Suggested order of attack

1. P0 #1–#3 (correctness; small, localized, unblocks quality modes + aliasing).
2. P1 held-note automation + smoothing (biggest playability/expressiveness win).
3. P3 cleanup (cheap, reduces confusion).
4. Decide the P1 "approximation vs upgrade" question for post DSP — it determines
   whether the detailed specs are a contract or a wishlist.
5. P2 features by product priority (MIDI expression and macros likely first).
