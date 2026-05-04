## 2026-05-03 - G2-9 Preset Format Decisions

- Use `Object` as the serde-backed enum with `PascalCase` names so `.corrosion-preset` stores the human-readable object name directly.
- Keep `Preset` as the canonical external file shape and rebuild `CorrosionParams` from it on load instead of mutating NIH-plug params in place.
- Mirror the default parameter ranges in preset reconstruction so load/save stays aligned with the UI and host-facing values.

## 2026-05-03 - G2-11 Limiter Test Strategy

- Test the limiter math through a small public helper instead of trying to spin up a full plugin host inside the integration test.
- Keep the plugin-side clamp in `Corrosion::process()` as the authoritative safety gate; the integration test only guards the threshold and clamp behavior.

## 2026-05-03 - Gate 2 Closure Documentation

- Use a dedicated `.sisyphus/evidence/gate-2-summary.md` closure file as the canonical pass-criteria record for Gate 2.
- Keep the gate closure tag (`gate-2-complete`) aligned with the evidence summary so future audits can jump from the tag to the closure artifact immediately.

## 2026-05-03 - Post-Gate-2 Triage Order

- Fix functional playback bugs before any DSP retuning; the current first priority is correct multi-note event consumption in the plugin process loop.
- Use human-readable object names everywhere the host/UI renders the object parameter, with the formatter defined once and reused by preset reconstruction.
- Defer physical-model retuning until the exciter expansion starts, then evaluate resonator changes against user-provided sonic references instead of tuning against the current presets alone.

## 2026-05-03 - Detailed Spec Authority Decision

- `docs/new-detailed-specs/` is the authoritative algorithm-definition layer for future exciter, resonator, transformation, interaction, and post-processing work.
- If the roadmap and the detailed specs disagree about timing/scope, the roadmap wins; if they disagree about mathematical behavior or named model identity, the detailed specs win.
- From Gate 3 onward, every DSP-facing task must cite the exact spec file(s) it implements and must preserve the named algorithm family unless a staged approximation is documented explicitly in evidence.
- `docs/full-feature-surface.md` and `docs/sound-direction-brief.md` are now treated as the bridge between roadmap sequencing and detailed-spec implementation, and must stay aligned with both.

## 2026-05-03 - Chain Object Wiring Decision

- Add Chain as a fourth user-facing object option (`Object::Chain`) and map it directly to `ModalProfileId::Chain` everywhere the object enum is translated to DSP profiles, presets, or preset-render tooling.
- Keep Chain as a true modal profile with a static mode table; do not introduce dynamic coupling yet, even though the detailed spec describes it, because this task only closes the static bank wiring.
