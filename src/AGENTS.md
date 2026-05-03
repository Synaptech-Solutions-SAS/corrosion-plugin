# SOURCE TREE GUIDE

## OVERVIEW
`src/` contains the live plugin implementation: host integration, DSP, voices, presets, offline tools, and CLI helpers.

## WHERE TO LOOK
| Task | Location | Notes |
|------|----------|-------|
| Host/plugin entry | `lib.rs` | exports, process loop, editor, state helpers |
| Parameter surface | `params.rs` | user-facing params + formatting |
| DSP algorithms | `dsp/` | objects, transforms, resonator math |
| Voice lifecycle | `voice/` | per-note state and polyphony |
| Preset persistence | `presets/` | JSON shape and param reconstruction |
| Offline/reference renderers | `offline/`, `bin/` | non-real-time helpers |

## CONVENTIONS
- Keep real-time code separate from file I/O / serde / tooling.
- If a task touches DSP from Gate 3 onward, read the matching `docs/new-detailed-specs/*.md` first.
- Preserve user-facing names from `docs/full-feature-surface.md` and `docs/sound-direction-brief.md`.
- Prefer small, explicit modules over hidden helper chains in performance-critical code.

## ANTI-PATTERNS
- Don’t add non-audio-safe work to `voice/`, `dsp/`, or the sample loop in `lib.rs`.
- Don’t change parameter naming in code without aligning docs and presets.
- Don’t mix offline/testing shortcuts into runtime modules unless clearly gated.

## NOTES
- `lib.rs` is the integration seam; `dsp/` and `voice/` are the hotspot directories with their own local rules.
