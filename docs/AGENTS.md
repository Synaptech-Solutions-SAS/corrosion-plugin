# DOCUMENTATION GUIDE

## OVERVIEW
`docs/` is not generic prose storage; it contains active engineering inputs, release-facing docs, and the authoritative detailed DSP spec pack.

## WHERE TO LOOK
| Task | Location | Notes |
|------|----------|-------|
| High-level execution / gating | `IMPLEMENTATION_PLAN.md`, `EXECUTION_TRACKER.md` | task ordering and closure rules |
| Product/PRD context | `corrosion_plugin_prd_and_specs.md` | scope and identity |
| Feature inventory | `full-feature-surface.md` | surfaced system map |
| Sonic intent | `sound-direction-brief.md` | target aesthetic and failure modes |
| Algorithm details | `new-detailed-specs/` | authoritative DSP-detail layer |

## CONVENTIONS
- Preserve terminology across docs: exciter/object/damage/space, not oscillator/filter/amp.
- Treat roadmap sequencing and detailed-spec math as separate authority layers.
- When docs define a source of truth, code and planning docs must align to it.
- Don’t remove stale references silently; reconcile them explicitly.

## ANTI-PATTERNS
- Don’t write docs that drift from exposed parameter names.
- Don’t flatten the distinction between high-level roadmap docs and low-level algorithm specs.
- Don’t mark release docs complete without cross-linking the authoritative design docs.

## NOTES
- `docs/new-detailed-specs/` has its own local rules; see child AGENTS.
