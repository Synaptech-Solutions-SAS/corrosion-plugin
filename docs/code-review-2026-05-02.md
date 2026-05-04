# Corrosion Code Review

Date: 2026-05-02

## Overall Verdict

The codebase does not fully satisfy the active plans/docs yet.

- Gate 0 appears functionally complete, but its evidence documents are stale and still reference deleted files.
- Gate 1 should remain `OPEN`, which matches `README.md` and `.sisyphus/evidence/gate-1-summary.md`, but the implementation still has blocking correctness issues beyond the documented REAPER runtime blocker.
- Gate 2 is not fulfilled. The repository contains a few Gate 2-shaped pieces (`8` voices, `Pipe`/`Plate`/`Tank` object selection), but the required parameter surface, preset/content work, DAW verification, and real-time constraints are not met.

## Primary Baseline Used

- `docs/corrosion_plugin_prd_and_specs.md`
- `docs/IMPLEMENTATION_PLAN.md`
- `docs/EXECUTION_TRACKER.md`
- `README.md`
- `.sisyphus/plans/corrosion-roadmap.md`
- `.sisyphus/evidence/gate-0-summary.md`
- `.sisyphus/evidence/gate-1-summary.md`

## Blocking Findings

### 1. Only one MIDI event is processed per audio buffer

- Severity: Critical
- File: `src/lib.rs:75`
- Evidence: `process()` calls `context.next_event()` once before the sample loop, stores it in `next_event`, and never advances to later events.
- Impact: Any additional `NoteOn`/`NoteOff` events in the same buffer are silently dropped. Chords, fast passages, and dense MIDI input will behave incorrectly.
- Plan impact: This undermines Gate 1's "MIDI note handling" and "playable from MIDI" goals.

### 2. Voice output is always divided by 8, even when fewer voices are active

- Severity: High
- File: `src/voice/manager.rs:63`
- Evidence: `process_sample()` sums all voices and returns `sum / MAX_VOICES as f32`.
- Impact: A single active note is attenuated by a factor of `8`, so normal use is much quieter than intended.
- Plan impact: Weakens the core audible-playable plugin requirement in Gate 1 and the polyphony behavior expected in Gate 2.

### 3. `note_off()` immediately makes a voice stealable, so natural decay is not preserved under reuse

- Severity: High
- Files: `src/voice/mod.rs:56`, `src/voice/mod.rs:96`, `src/voice/manager.rs:27`
- Evidence: `Voice::note_off()` sets `active = false` immediately. `VoiceManager::note_on()` first reuses any inactive voice.
- Impact: The resonator can keep ringing only until another note claims that slot. In practice, a new note can cut off the old note's tail early.
- Plan impact: This only partially satisfies Gate 1's requirement that note-off allow natural decay.

### 4. The host-visible `Gain` parameter does nothing

- Severity: High
- Files: `src/params.rs:5`, `src/lib.rs:69`
- Evidence: `gain` is defined in `CorrosionParams`, but `process()` never reads or applies it.
- Impact: Hosts expose and may automate a parameter that has no audio effect.
- Plan impact: Violates the documented expectation that exposed parameters are meaningful and automatable.

### 5. MIDI pitch is tested but not used in the plugin audio path

- Severity: High
- Files: `src/voice/mod.rs:7`, `src/voice/mod.rs:40`
- Evidence: `midi_to_hz()` exists and is tested, but `note_on()` rebuilds the resonator with default profile settings and no note-derived retuning.
- Impact: Different MIDI notes do not retune the resonator, so pitch behavior does not match the plans/docs.
- Plan impact: Fails the PRD instrument-mode expectation that pitch is derived from MIDI note, and does not satisfy the Gate 1 checklist item for MIDI note to base-frequency conversion in any observable plugin behavior.

## Major Findings

### 6. The real-time path ignores the planned physical parameters

- Severity: Major
- Files: `src/voice/mod.rs:48`, `src/params.rs:4`
- Evidence: The plugin path always rebuilds voices with `SizeScale::default()`, `RustAmount::default()`, and `DamageAmount::default()`. Only `Object` is selectable, and `Gain` is unused.
- Impact: The repository contains the offline DSP transforms, but the live plugin does not expose or use the Gate 2 parameter surface (`Object`, `Size`, `Rust`, `Damage`, `Drive`, `Output`).
- Plan impact: Confirms Gate 2 is not implemented even though some supporting DSP primitives exist.

### 7. The note-on path still allocates heap memory

- Severity: Major
- Files: `src/dsp/profile.rs:60`, `src/dsp/resonator.rs:70`
- Evidence: `ModalModeSpec::damaged()` returns `Vec<Self>`, and `ModalResonator` stores a `Vec<SecondOrderMode>` built during voice setup.
- Impact: Voice creation on note-on is not allocation-free.
- Plan impact: This is explicitly incompatible with the Gate 2 / cross-gate real-time safety target of no allocation in the live render path.

### 8. Documentation and evidence drift is significant

- Severity: Major
- Files: `.sisyphus/evidence/gate-0-summary.md:11`, `.sisyphus/evidence/parameter-ranges.md:3`
- Evidence: Both files cite deleted paths like `src/renderer.rs` and `src/main.rs`, while the current implementation lives in `src/dsp/`, `src/offline/mod.rs`, and `src/bin/render.rs`.
- Impact: The audit trail is no longer trustworthy without manual reinterpretation.
- Plan impact: Weakens the gate evidence requirement in `docs/IMPLEMENTATION_PLAN.md` because the evidence no longer cleanly matches the codebase.

### 9. Gate 1 evidence understates the current implementation and overstates closure confidence

- Severity: Major
- File: `.sisyphus/evidence/gate-1-summary.md:10`
- Evidence: The summary still describes a single-voice pipe-only path, but the code already contains `MAX_VOICES = 8` and an `Object` parameter with `Pipe`, `Plate`, and `Tank`.
- Impact: The report is stale in both directions: it misses implemented scope and it does not mention the current correctness bugs in the expanded path.

## Moderate Findings

### 10. The project currently mixes Gate 1 and Gate 2 scope without meeting either cleanly

- Severity: Moderate
- Files: `docs/IMPLEMENTATION_PLAN.md:197`, `docs/IMPLEMENTATION_PLAN.md:286`, `src/voice/manager.rs:3`, `src/params.rs:11`
- Evidence: Gate 1 says polyphony and plate/tank are out of scope, but the implementation already includes both. At the same time, key Gate 2 deliverables are still absent.
- Impact: The codebase is harder to reason about because roadmap stage and implementation stage diverge.

### 11. Verification cannot currently be reproduced in this environment

- Severity: Moderate
- Evidence from direct checks:
  - `rustup toolchain list` returned `no installed toolchains`
  - `cargo test --workspace` failed immediately because no default toolchain is configured
  - `cargo run --release --bin render` failed for the same reason
  - `./bundle.sh` failed for the same reason
  - `lsp_diagnostics` could not run because `rust-analyzer` is not installed
- Impact: I could inspect the code and prior evidence, but I could not freshly validate the claimed green state.

## Fulfillment Against Plans And Docs

### Gate 0

- Judgment: Substantively complete, documentation stale
- Why: The DSP/offline modules and artifact directories align with the Gate 0 outcome, but the evidence docs still reference deleted source files.

### Gate 1

- Judgment: Still open
- Why:
  - The documented REAPER smoke-test blocker is still real.
  - MIDI handling is incorrect for multi-event buffers.
  - Natural decay semantics are only partial because note-off makes voices immediately reusable.
  - The plugin exposes a `Gain` parameter that does not work.

### Gate 2

- Judgment: Not fulfilled
- Missing or unsupported:
  - Full parameter surface (`Size`, `Rust`, `Damage`, `Drive`, `Output`)
  - Meaningful velocity-to-physical-behavior mapping
  - No-allocation note/render path
  - Presets and preset workflow
  - REAPER-based DAW verification
  - Stable, clearly correct 8-voice behavior

## Documentation Consistency Notes

- `README.md` is the clearest current status summary.
- `docs/IMPLEMENTATION_PLAN.md` is the best acceptance baseline.
- `docs/EXECUTION_TRACKER.md` and `docs/plans/corrosion.md` still carry many stale unchecked items and should not be treated as live completion evidence without reconciliation.
- `.sisyphus/evidence/*` needs a pass to update file references after the module split from `src/renderer.rs`.

## Recommended Next Actions

1. Fix `src/lib.rs` MIDI event iteration so all events in the buffer are processed in time order.
2. Fix voice lifecycle semantics so `note_off` preserves the decay tail without making the slot immediately reusable.
3. Remove or apply `Gain`; do not expose dead parameters.
4. Wire MIDI pitch into the resonator path so note input changes the produced pitch.
5. Reconcile the docs/evidence set with the current file layout before treating the gate evidence as authoritative.
6. Restore a working Rust toolchain in the environment and re-run the advertised verification commands.
