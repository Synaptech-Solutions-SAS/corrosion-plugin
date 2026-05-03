# SCRIPT HELPERS GUIDE

## OVERVIEW
`scripts/` contains tiny QA/helper CLIs. Their exit code is part of the repo contract.

## WHERE TO LOOK
| Task | Location | Notes |
|------|----------|-------|
| WAV validity checks | `check_wav.py` | non-silent, non-clipping, non-NaN |
| Click detection | `check_clicks.py` | sample-delta smoke check |
| Human-readable + JSON WAV inspection | `analyze_wav.py` | metadata, previews, exported analysis |

## CONVENTIONS
- Stdlib-only unless the roadmap explicitly says otherwise.
- Single-purpose CLI shape: simple args, parseable stdout, exit code as truth.
- Prefer `ERROR:` / `FAIL:` prefixes for bad-path output.
- Keep outputs stable; these scripts are used by roadmap QA steps.

## ANTI-PATTERNS
- Don’t add third-party deps casually.
- Don’t make stdout noisy if another tool may parse it.
- Don’t change pass/fail exit semantics without updating roadmap QA steps.

## NOTES
- Common pitfalls: 24-bit WAV sign extension, float-vs-int decoding, path assumptions, and extra prints that break evidence parsing.
