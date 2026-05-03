# VOICE / POLYPHONY GUIDE

## OVERVIEW
`src/voice/` is the real-time execution boundary for note lifecycle, excitation delivery, voice stealing, and tail handling.

## WHERE TO LOOK
| Task | Location | Notes |
|------|----------|-------|
| Per-note state / excitation | `mod.rs` | note_on/off, excitation shaping, denormal handling |
| Polyphony / stealing | `manager.rs` | 8-voice pool, oldest/quietest selection |

## CONVENTIONS
- Assume this directory is audio-thread sensitive by default.
- Keep note-event handling deterministic and sample-accurate.
- Voice stealing, tail tracking, and excitation behavior must remain test-covered.
- When changing excitation/interaction behavior, cross-check `docs/new-detailed-specs/exciter-resonator-interaction.md`.

## ANTI-PATTERNS
- Don’t add file I/O, serde, logging, mutexes, or hidden allocation here.
- Don’t consume only part of the MIDI/event stream for a buffer.
- Don’t bury RT-safety decisions in helper code without tests.

## NOTES
- `mod.rs` carries both runtime logic and targeted unit tests; keep them aligned.
