# TEST / QA GUIDE

## OVERVIEW
`tests/` mixes Rust integration tests, DAW smoke scripts, and shared fixtures. This is the repo’s executable QA surface, not just unit-test overflow.

## WHERE TO LOOK
| Task | Location | Notes |
|------|----------|-------|
| Integration regressions | `tests/*.rs` | metrics, safety, presets, polyphony |
| DAW/host smoke checks | `tests/daw/` | REAPER and related scripted checks |
| Fixtures | `tests/fixtures/` | use repo-root-relative paths |

## CONVENTIONS
- Prefer deterministic metric assertions over subjective audio claims.
- Use repo-root-relative fixture paths (`tests/fixtures/...`), not `.sisyphus/...`.
- Capture host/DAW evidence under `.sisyphus/evidence/` with explicit task IDs.
- Validate rendered WAVs with `scripts/check_wav.py`; use `check_clicks.py` for click-sensitive bounces.

## ANTI-PATTERNS
- Don’t add tests the roadmap/task QA did not ask for.
- Don’t rely on manual DAW interaction when a scripted path is required.
- Don’t hide flaky timing or audio checks behind broad tolerances without evidence.

## NOTES
- `tests/daw/` scripts are smoke/regression runners, not substitutes for Rust integration tests.
