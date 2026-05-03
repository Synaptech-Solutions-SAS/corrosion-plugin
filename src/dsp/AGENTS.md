# DSP MODULE GUIDE

## OVERVIEW
`src/dsp/` is the algorithm core: modal objects, transforms, excitation primitives, resonator math, and realtime budgets.

## WHERE TO LOOK
| Task | Location | Notes |
|------|----------|-------|
| Modal objects / identities | `profile.rs` | pipe, plate, tank; future object expansion anchor |
| Resonator processing | `resonator.rs` | per-mode state, coefficient math, sample-rate path |
| Physical transforms | `transforms.rs` | size, rust, damage math |
| Excitation helpers | `excitation.rs` | deterministic excitation utilities |
| Budget/safety limits | `budget.rs` | realtime mode caps |

## CONVENTIONS
- Treat `docs/new-detailed-specs/*.md` as the algorithm-detail authority for Gate 3+ changes.
- Preserve modal/object identity; differences should come from math/structure, not cosmetic EQ swaps.
- Keep per-sample math explicit and auditable.
- Any staged approximation of a spec algorithm must be documented in evidence immediately.

## ANTI-PATTERNS
- Don’t replace object models with generic noise/reverb stand-ins.
- Don’t add heap allocation or hidden container growth in audio-rate paths.
- Don’t introduce parameter drift between code and `docs/full-feature-surface.md`.

## NOTES
- `mod.rs` is test-heavy and acts as the DSP evidence hub.
- `profile.rs` + `resonator.rs` are the main complexity hotspots.
