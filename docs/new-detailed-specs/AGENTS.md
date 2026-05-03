# DETAILED SPEC PACK GUIDE

## OVERVIEW
This directory is the authoritative algorithm-detail layer from Gate 3 onward. It defines model families, signal-chain placement, interaction semantics, and exposed DSP vocabulary.

## WHERE TO LOOK
| Task | Location | Notes |
|------|----------|-------|
| Global ordering / rate boundaries | `signal-chain.md` | modulation bus, exciter, interaction, resonator, post, space |
| Exciters | `exciter-algorithms.md` | categories, named models, tweakable params |
| Object/resonator families | `resonator-algorithms.md` | modal object math and expansion targets |
| Transforms | `transformation-algorithms.md` | size/rust/damage plus future transforms |
| Coupling / MSEG / strike position | `exciter-resonator-interaction.md` | bidirectional interaction semantics |
| Post / space / output | `post-processing.md` | filter/drive/body/stereo/space/output |

## CONVENTIONS
- If the roadmap and these files disagree on **when**, the roadmap wins.
- If they disagree on **how**, these files win.
- Preserve named algorithm families even when staging approximations.
- Record every simplification with an explicit “Implemented Now / Deferred From Spec” evidence section.

## ANTI-PATTERNS
- Don’t treat these files as inspiration-only.
- Don’t rename model families or user-facing control vocabulary casually.
- Don’t collapse bidirectional interaction into simple feed-forward without documenting the compromise.

## NOTES
- These files are intentionally dense; they should map directly to roadmap tasks and future developer docs.
