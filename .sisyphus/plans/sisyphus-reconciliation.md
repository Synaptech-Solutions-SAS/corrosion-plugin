# .sisyphus Reconciliation Plan

## TL;DR
> **Summary**: Rewrite stale `.sisyphus` artifacts in place so plans, evidence, notepads, and Sisyphus session state match the current repository reality documented in the code review, without modifying anything outside `.sisyphus/`.
> **Deliverables**:
> - Corrected `.sisyphus/boulder.json` active-plan pointer
> - Reconciled Gate 0 / Gate 1 evidence docs with current file paths and architecture claims
> - Cleaned `.sisyphus/notepads/corrosion-roadmap/learnings.md` chronology/status drift
> - Updated `.sisyphus/plans/corrosion-roadmap.md` where current truth conflicts with stale assumptions
> - Reconciliation evidence logs under `.sisyphus/evidence/`
> **Effort**: Short
> **Parallel**: YES - 3 waves
> **Critical Path**: T1 → T3/T4 → T5 → T7 → F1-F4

## Context
### Original Request
Take all the information from the code review and rework the plan, evidence, notes, and everything in `.sisyphus`, without touching anything outside that folder.

### Interview Summary
- Scope is limited to `.sisyphus/` only.
- Reconciliation style: rewrite stale artifacts in place.
- Main mismatch is stale implementation/path claims, not incorrect gate labels.

### Metis Review (gaps addressed)
- Preserve historical integrity for old evidence docs by rewriting in place with explicit correction/addendum blocks rather than silently changing factual history.
- Do not line-by-line rewrite the entire roadmap; only correct sections that conflict with current `.sisyphus` truth.
- Do not hardcode verification that depends on unavailable Rust tooling.
- Treat `gate-1-summary.md` + `task-G1-9-blocked.md` as canonical Gate 1 state.

## Work Objectives
### Core Objective
Make `.sisyphus/` internally consistent so a future executor can trust it as the project’s planning/evidence layer without re-deriving which artifacts are stale.

### Deliverables
- `.sisyphus/boulder.json` with a valid absolute `active_plan` path.
- Updated `.sisyphus/evidence/gate-0-summary.md` and `.sisyphus/evidence/gate-0-review.md` with corrected module-path references.
- Updated `.sisyphus/evidence/parameter-ranges.md` with current source-path mapping while preserving frozen parameter values.
- Updated `.sisyphus/evidence/gate-1-summary.md` reflecting the current 8-voice/object-routing implementation and current blocker state.
- Updated `.sisyphus/notepads/corrosion-roadmap/learnings.md` with corrected Gate 1 status framing and explicit chronology.
- Updated `.sisyphus/plans/corrosion-roadmap.md` only where stale claims conflict with current `.sisyphus` truth.
- Reconciliation logs under `.sisyphus/evidence/` proving the cleanup touched only `.sisyphus/`.

### Definition of Done (verifiable conditions with commands)
- [ ] `python3 -c 'import json;print(json.load(open(".sisyphus/boulder.json"))["active_plan"])'` prints a path starting with `/home/german/projects/corrotion-vst/.sisyphus/plans/`
- [ ] `grep -R "src/renderer\\.rs" .sisyphus/` returns no matches
- [ ] `grep -R "src/main\\.rs" .sisyphus/` returns no matches
- [ ] `grep -n "Gate 1 Complete" .sisyphus/notepads/corrosion-roadmap/learnings.md` returns no matches
- [ ] `grep -n "single-voice" .sisyphus/evidence/gate-1-summary.md` returns no matches
- [ ] `grep -n "clap-validator" .sisyphus/plans/corrosion-roadmap.md` returns at least one relevant Gate 1 validation reference
- [ ] `git diff --name-only -- . ':!/.sisyphus'` or equivalent path-filtered status proves no non-`.sisyphus` files changed

### Must Have
- Only `.sisyphus/` files are modified.
- Gate 0 remains CLOSED and Gate 1 remains OPEN/BLOCKED unless `.sisyphus` evidence itself already proves otherwise.
- Historical evidence docs are corrected without changing underlying gate outcomes.
- Path references point to current module layout (`src/dsp/*`, `src/offline/mod.rs`, `src/bin/render.rs`, `src/voice/manager.rs`) where applicable.
- Future roadmap intent stays intact; only stale assumptions are reconciled.

### Must NOT Have
- No edits outside `.sisyphus/`
- No source-code changes
- No changes to parameter values/ranges in `parameter-ranges.md`
- No invented build/test results
- No forced closure of Gate 1
- No full rewrite of future Gate 2+ roadmap tasks that are merely aspirational rather than contradictory

## Verification Strategy
> ZERO HUMAN INTERVENTION - all verification is agent-executed.
- Test decision: none + shell/grep/json verification only
- QA policy: Every task includes exact path/content assertions and a touched-files check
- Evidence: `.sisyphus/evidence/task-{N}-{slug}.log`

## Execution Strategy
### Parallel Execution Waves
Wave 1: pointer + Gate 0 evidence alignment
- T1 pointer/config correction
- T2 Gate 0 evidence path reconciliation

Wave 2: Gate 1 and notepad truth alignment
- T3 parameter-ranges path update section
- T4 Gate 1 summary rewrite
- T5 learnings chronology cleanup

Wave 3: roadmap reconciliation + final sweep
- T6 issues.md confirmation / no-op proof
- T7 roadmap reconciliation
- T8 global `.sisyphus` grep sweep and reconciliation manifest

### Dependency Matrix (full, all tasks)
| Task | Depends On | Blocks |
|---|---|---|
| T1 | none | T8 |
| T2 | none | T8 |
| T3 | none | T8 |
| T4 | none | T5, T7, T8 |
| T5 | T4 | T8 |
| T6 | none | T7 |
| T7 | T4, T6 | T8 |
| T8 | T1,T2,T3,T4,T5,T7 | F1-F4 |

### Agent Dispatch Summary
| Wave | Task Count | Categories |
|---|---:|---|
| 1 | 2 | `quick`, `writing` |
| 2 | 3 | `writing`, `quick` |
| 3 | 3 | `writing`, `quick`, `unspecified-high` |
| Final | 4 | `oracle`, `unspecified-high`, `deep` |

## TODOs

- [x] 1. Fix Sisyphus active-plan pointer

  **What to do**: Update `.sisyphus/boulder.json` so `active_plan` points to `/home/german/projects/corrotion-vst/.sisyphus/plans/corrosion-roadmap.md` or the currently intended canonical plan path under the real workspace root. Preserve valid JSON formatting.
  **Must NOT do**: Do not add new keys. Do not change any path outside `.sisyphus/`. Do not repoint to a non-existent plan.

  **Recommended Agent Profile**:
  - Category: `quick` - One-file deterministic correction.
  - Skills: [] - No special skill needed.
  - Omitted: [`git-master`] - No git operation required.

  **Parallelization**: Can Parallel: YES | Wave 1 | Blocks: T8 | Blocked By: none

  **References**:
  - Pattern: `.sisyphus/boulder.json` - existing JSON structure
  - Pattern: `.sisyphus/plans/corrosion-roadmap.md` - target path should exist
  - Source-of-truth: `.sisyphus/drafts/sisyphus-reconciliation.md` - scope and request context

  **Acceptance Criteria**:
  - [ ] `python3 -c 'import json; d=json.load(open(".sisyphus/boulder.json")); assert d["active_plan"].startswith("/home/german/projects/corrotion-vst/.sisyphus/plans/")'`
  - [ ] `python3 -c 'import json,os; d=json.load(open(".sisyphus/boulder.json")); assert os.path.exists(d["active_plan"])'`

  **QA Scenarios**:
  ```
  Scenario: active_plan points to a real in-repo .sisyphus plan
    Tool: Bash
    Steps: python3 - <<'PY'
import json, os
with open('.sisyphus/boulder.json') as f:
    d = json.load(f)
assert d['active_plan'].startswith('/home/german/projects/corrotion-vst/.sisyphus/plans/')
assert os.path.exists(d['active_plan'])
print(d['active_plan'])
PY
    Expected: exits 0 and prints a valid absolute plan path
    Evidence: .sisyphus/evidence/task-1-boulder-path.log

  Scenario: no unrelated JSON drift
    Tool: Bash
    Steps: python3 - <<'PY'
import json
with open('.sisyphus/boulder.json') as f:
    d = json.load(f)
assert set(d.keys()) >= {'active_plan'}
print(sorted(d.keys()))
PY
    Expected: valid JSON with expected key set retained
    Evidence: .sisyphus/evidence/task-1-boulder-shape.log
  ```

  **Commit**: NO | Message: `docs(sisyphus): fix boulder active plan path` | Files: [`.sisyphus/boulder.json`]

- [x] 2. Reconcile Gate 0 evidence path references

  **What to do**: Update `.sisyphus/evidence/gate-0-summary.md` and `.sisyphus/evidence/gate-0-review.md` so any references to deleted `src/renderer.rs` / `src/main.rs` are replaced with the current module layout or explicit correction notes. Keep Gate 0 status CLOSED and preserve the original evidence meaning.
  **Must NOT do**: Do not change Gate 0 outcomes, metric values, pass/fail calls, or dates unless a value is clearly just a stale code-path citation.

  **Recommended Agent Profile**:
  - Category: `writing` - Historical-document reconciliation.
  - Skills: [] - No special skill needed.
  - Omitted: [`review-work`] - This is not final QA.

  **Parallelization**: Can Parallel: YES | Wave 1 | Blocks: T8 | Blocked By: none

  **References**:
  - Pattern: `.sisyphus/evidence/gate-0-summary.md` - stale `src/renderer.rs` / `src/main.rs` mentions
  - Pattern: `.sisyphus/evidence/gate-0-review.md` - stale WAV writer references
  - API/Type: `src/dsp/resonator.rs` - current `SecondOrderMode::process` location
  - API/Type: `src/offline/mod.rs` - current WAV writer / `float_sample_to_pcm_i16` location

  **Acceptance Criteria**:
  - [ ] `grep -n "src/renderer\.rs\|src/main\.rs" .sisyphus/evidence/gate-0-summary.md .sisyphus/evidence/gate-0-review.md` returns no matches
  - [ ] `grep -n "GATE 0 STATUS: CLOSED" .sisyphus/evidence/gate-0-summary.md` still matches

  **QA Scenarios**:
  ```
  Scenario: Gate 0 evidence keeps its verdict while removing dead-path refs
    Tool: Bash
    Steps: grep -n 'GATE 0 STATUS: CLOSED' .sisyphus/evidence/gate-0-summary.md && ! grep -nE 'src/renderer\.rs|src/main\.rs' .sisyphus/evidence/gate-0-summary.md .sisyphus/evidence/gate-0-review.md
    Expected: CLOSED status preserved; dead-path refs removed
    Evidence: .sisyphus/evidence/task-2-gate0-paths.log

  Scenario: Gate 0 metrics remain untouched
    Tool: Bash
    Steps: grep -n 'roughness=0.1016\|peak \(3.95\)\|Pipe, plate, and tank sound clearly distinct' .sisyphus/evidence/gate-0-summary.md .sisyphus/evidence/gate-0-review.md
    Expected: key historical metrics/verdict text still present
    Evidence: .sisyphus/evidence/task-2-gate0-metrics.log
  ```

  **Commit**: NO | Message: `docs(sisyphus): reconcile gate 0 evidence paths` | Files: [`.sisyphus/evidence/gate-0-summary.md`, `.sisyphus/evidence/gate-0-review.md`]

- [x] 3. Add current path mapping to frozen parameter ranges evidence

  **What to do**: Update `.sisyphus/evidence/parameter-ranges.md` so stale `src/renderer.rs` citations are replaced with current module references or a dedicated “Post-G1-3 module split path mapping” section. Preserve all parameter values and frozen-range semantics.
  **Must NOT do**: Do not alter min/max/default/range values. Do not imply the parameter ranges themselves changed.

  **Recommended Agent Profile**:
  - Category: `writing` - Controlled evidence clarification.
  - Skills: [] - No special skill needed.
  - Omitted: [`oracle`] - No design decision required.

  **Parallelization**: Can Parallel: YES | Wave 2 | Blocks: T8 | Blocked By: none

  **References**:
  - Pattern: `.sisyphus/evidence/parameter-ranges.md` - frozen values plus stale citations
  - API/Type: `src/dsp/transforms.rs` - `SizeScale`, `RustAmount`, `DamageAmount`
  - API/Type: `src/dsp/budget.rs` - mode-budget references
  - API/Type: `src/dsp/profile.rs` - decay/frequency family data
  - API/Type: `src/offline/mod.rs` - `RenderConfig` defaults

  **Acceptance Criteria**:
  - [ ] `! grep -n 'src/renderer\.rs' .sisyphus/evidence/parameter-ranges.md`
  - [ ] `grep -n 'Frozen: 2026-05-02' .sisyphus/evidence/parameter-ranges.md`
  - [ ] existing values like `min: 0.25`, `max: 1.0`, `default: 1.0`, `sample_rate: default: 48,000` still exist

  **QA Scenarios**:
  ```
  Scenario: stale citations removed without changing frozen values
    Tool: Bash
    Steps: ! grep -n 'src/renderer\.rs' .sisyphus/evidence/parameter-ranges.md && grep -nE 'min: 0.25|default: 1.0|sample_rate: default: 48,000' .sisyphus/evidence/parameter-ranges.md
    Expected: dead-path refs gone; core values unchanged
    Evidence: .sisyphus/evidence/task-3-parameter-ranges.log

  Scenario: current module mapping is present
    Tool: Bash
    Steps: grep -nE 'src/dsp/transforms\.rs|src/dsp/budget\.rs|src/dsp/profile\.rs|src/offline/mod\.rs' .sisyphus/evidence/parameter-ranges.md
    Expected: at least one current module reference per stale citation area
    Evidence: .sisyphus/evidence/task-3-parameter-mapping.log
  ```

  **Commit**: NO | Message: `docs(sisyphus): update parameter-range source mappings` | Files: [`.sisyphus/evidence/parameter-ranges.md`]

- [x] 4. Rewrite Gate 1 evidence summary to match current implementation truth

  **What to do**: Update `.sisyphus/evidence/gate-1-summary.md` so it reflects the actual implementation state documented by the code review: 8-slot voice manager, `Object` parameter, pipe/plate/tank routing, current module tree including `src/voice/manager.rs`, and Gate 1 still OPEN/BLOCKED because REAPER smoke remains blocked. Preserve the distinction between implemented scope and blocked closure.
  **Must NOT do**: Do not claim Gate 1 is closed. Do not invent successful REAPER validation. Do not claim Gain is applied if it still is not.

  **Recommended Agent Profile**:
  - Category: `writing` - Canonical status document rewrite.
  - Skills: [] - No special skill needed.
  - Omitted: [`artistry`] - No unconventional design needed.

  **Parallelization**: Can Parallel: YES | Wave 2 | Blocks: T5, T7, T8 | Blocked By: none

  **References**:
  - Pattern: `.sisyphus/evidence/gate-1-summary.md` - canonical Gate 1 state doc
  - Pattern: `.sisyphus/evidence/task-G1-9-blocked.md` - blocker truth
  - Pattern: `.sisyphus/notepads/corrosion-roadmap/issues.md` - validator/runtime blockers
  - API/Type: `src/lib.rs` - current plugin path / object routing / process limitations
  - API/Type: `src/params.rs` - `gain` + `object`
  - API/Type: `src/voice/manager.rs` - 8-slot manager

  **Acceptance Criteria**:
  - [ ] `grep -n 'GATE 1 STATUS: OPEN' .sisyphus/evidence/gate-1-summary.md`
  - [ ] `! grep -n 'Single-voice' .sisyphus/evidence/gate-1-summary.md`
  - [ ] `grep -n 'VoiceManager\|8-slot\|Object' .sisyphus/evidence/gate-1-summary.md`
  - [ ] `grep -n 'REAPER' .sisyphus/evidence/gate-1-summary.md`

  **QA Scenarios**:
  ```
  Scenario: Gate 1 summary matches current architecture and blocker state
    Tool: Bash
    Steps: grep -n 'GATE 1 STATUS: OPEN' .sisyphus/evidence/gate-1-summary.md && grep -nE 'VoiceManager|8-slot|Object|Pipe|Plate|Tank' .sisyphus/evidence/gate-1-summary.md && grep -n 'REAPER' .sisyphus/evidence/gate-1-summary.md
    Expected: open status, current architecture, and blocker text all present
    Evidence: .sisyphus/evidence/task-4-gate1-summary.log

  Scenario: stale single-voice framing removed
    Tool: Bash
    Steps: ! grep -nE 'single-voice|hit→pipe path' .sisyphus/evidence/gate-1-summary.md
    Expected: no stale single-voice wording remains
    Evidence: .sisyphus/evidence/task-4-gate1-wording.log
  ```

  **Commit**: NO | Message: `docs(sisyphus): align gate 1 summary with current state` | Files: [`.sisyphus/evidence/gate-1-summary.md`]

- [x] 5. Normalize learnings chronology and Gate 1 status notes

  **What to do**: Rewrite `.sisyphus/notepads/corrosion-roadmap/learnings.md` so it reads as chronological historical notes instead of contradictory current truth. Replace or reframe the top “Gate 1 Complete” wording to match the canonical Gate 1 OPEN/BLOCKED state, keep dated entries, and explicitly mark older test counts / single-voice notes as historical snapshots when needed.
  **Must NOT do**: Do not erase useful historical context. Do not let this file contradict `gate-1-summary.md` after cleanup.

  **Recommended Agent Profile**:
  - Category: `writing` - Historical-note cleanup.
  - Skills: [] - No special skill needed.
  - Omitted: [`momus`] - Not needed before draft plan exists.

  **Parallelization**: Can Parallel: NO | Wave 2 | Blocks: T8 | Blocked By: T4

  **References**:
  - Pattern: `.sisyphus/notepads/corrosion-roadmap/learnings.md` - chronology with drift
  - Pattern: `.sisyphus/evidence/gate-1-summary.md` - canonical current Gate 1 state
  - Pattern: `.sisyphus/evidence/task-G1-9-blocked.md` - blocker reference

  **Acceptance Criteria**:
  - [ ] `! grep -n 'Gate 1 Complete' .sisyphus/notepads/corrosion-roadmap/learnings.md`
  - [ ] `grep -nE 'Gate 1 .*OPEN|blocked|historical|snapshot' .sisyphus/notepads/corrosion-roadmap/learnings.md`
  - [ ] file still contains dated entries like `2026-05-02`

  **QA Scenarios**:
  ```
  Scenario: learnings no longer contradict canonical Gate 1 state
    Tool: Bash
    Steps: ! grep -n 'Gate 1 Complete' .sisyphus/notepads/corrosion-roadmap/learnings.md && grep -nE 'OPEN|BLOCKED|historical snapshot|2026-05-02' .sisyphus/notepads/corrosion-roadmap/learnings.md
    Expected: contradictory header removed; chronology remains explicit
    Evidence: .sisyphus/evidence/task-5-learnings-status.log

  Scenario: stale single-voice note is contextualized or removed
    Tool: Bash
    Steps: ! grep -nE '^### Voice Architecture$|single-voice hit exciter$' .sisyphus/notepads/corrosion-roadmap/learnings.md || grep -n 'historical' .sisyphus/notepads/corrosion-roadmap/learnings.md
    Expected: no unqualified current-state single-voice claim remains
    Evidence: .sisyphus/evidence/task-5-learnings-voice.log
  ```

  **Commit**: NO | Message: `docs(sisyphus): reconcile learnings chronology` | Files: [`.sisyphus/notepads/corrosion-roadmap/learnings.md`]

- [x] 6. Confirm issues note remains the live blocker source

  **What to do**: Inspect `.sisyphus/notepads/corrosion-roadmap/issues.md`. If already accurate, leave content unchanged and record a no-op verification note in `.sisyphus/evidence/`. If one or two wording fixes are needed to align with the rest of `.sisyphus`, keep them minimal.
  **Must NOT do**: Do not expand scope by turning `issues.md` into a second roadmap. Do not change accurate blocker facts just for stylistic consistency.

  **Recommended Agent Profile**:
  - Category: `quick` - Verify or minimal touch.
  - Skills: [] - No special skill needed.
  - Omitted: [`oracle`] - No complex reasoning needed.

  **Parallelization**: Can Parallel: YES | Wave 3 | Blocks: T7 | Blocked By: none

  **References**:
  - Pattern: `.sisyphus/notepads/corrosion-roadmap/issues.md` - live blockers source
  - Pattern: `.sisyphus/evidence/gate-1-summary.md` - must stay aligned

  **Acceptance Criteria**:
  - [ ] `grep -n 'clap-validator' .sisyphus/notepads/corrosion-roadmap/issues.md`
  - [ ] `grep -n 'libGL\.so\.1' .sisyphus/notepads/corrosion-roadmap/issues.md`
  - [ ] a no-op or update note is written to `.sisyphus/evidence/task-6-issues-check.log`

  **QA Scenarios**:
  ```
  Scenario: issues.md still states the correct CLAP and REAPER blockers
    Tool: Bash
    Steps: grep -n 'clap-validator' .sisyphus/notepads/corrosion-roadmap/issues.md && grep -n 'libGL.so.1' .sisyphus/notepads/corrosion-roadmap/issues.md > .sisyphus/evidence/task-6-issues-check.log
    Expected: both current blocker/tool references present
    Evidence: .sisyphus/evidence/task-6-issues-check.log

  Scenario: issues check stays .sisyphus-only
    Tool: Bash
    Steps: test -f .sisyphus/evidence/task-6-issues-check.log
    Expected: evidence log exists under .sisyphus only
    Evidence: .sisyphus/evidence/task-6-issues-check.log
  ```

  **Commit**: NO | Message: `docs(sisyphus): confirm issues note truth source` | Files: [`.sisyphus/notepads/corrosion-roadmap/issues.md`, `.sisyphus/evidence/task-6-issues-check.log`]

- [x] 7. Reconcile the canonical roadmap with current .sisyphus truth

  **What to do**: Update `.sisyphus/plans/corrosion-roadmap.md` only where it directly conflicts with current `.sisyphus` truth. Required fixes include: stale current-state context/research references to `src/renderer.rs` / `src/main.rs`, stale single-voice Gate 1 framing in present-state summaries, stale CLAP validation guidance that still uses pluginval instead of clap-validator, and any QA expectation that currently requires Gate 1 to be CLOSED despite the blocker evidence. Preserve long-term roadmap intent for future gates.
  **Must NOT do**: Do not rewrite all future tasks just because the code evolved. Do not alter out-of-scope product direction. Do not mark blocked Gate 1 work as complete.

  **Recommended Agent Profile**:
  - Category: `writing` - Large but bounded document reconciliation.
  - Skills: [] - No special skill needed.
  - Omitted: [`deep`] - Research is already complete.

  **Parallelization**: Can Parallel: NO | Wave 3 | Blocks: T8 | Blocked By: T4, T6

  **References**:
  - Pattern: `.sisyphus/plans/corrosion-roadmap.md` - canonical roadmap doc
  - Pattern: `.sisyphus/evidence/gate-1-summary.md` - current Gate 1 truth
  - Pattern: `.sisyphus/notepads/corrosion-roadmap/issues.md` - correct CLAP validator / REAPER blocker
  - Pattern: `.sisyphus/evidence/parameter-ranges.md` - frozen Gate 0 ranges
  - External: `docs/code-review-2026-05-02.md` - consolidated mismatch list (read-only reference; do not modify)

  **Acceptance Criteria**:
  - [ ] `! grep -nE 'src/renderer\.rs|src/main\.rs' .sisyphus/plans/corrosion-roadmap.md`
  - [ ] `grep -n 'clap-validator' .sisyphus/plans/corrosion-roadmap.md`
  - [ ] `! grep -n 'GATE 1 STATUS: CLOSED' .sisyphus/plans/corrosion-roadmap.md`

  **QA Scenarios**:
  ```
  Scenario: roadmap current-state references are reconciled
    Tool: Bash
    Steps: ! grep -nE 'src/renderer\.rs|src/main\.rs' .sisyphus/plans/corrosion-roadmap.md && grep -n 'clap-validator' .sisyphus/plans/corrosion-roadmap.md
    Expected: dead-path refs removed; correct CLAP validator present
    Evidence: .sisyphus/evidence/task-7-roadmap-paths.log

  Scenario: roadmap no longer contradicts Gate 1 blocker truth
    Tool: Bash
    Steps: ! grep -n 'GATE 1 STATUS: CLOSED' .sisyphus/plans/corrosion-roadmap.md && grep -nE 'OPEN|blocked|REAPER' .sisyphus/plans/corrosion-roadmap.md
    Expected: no forced-closed Gate 1 wording remains where current truth is discussed
    Evidence: .sisyphus/evidence/task-7-roadmap-gate1.log
  ```

  **Commit**: NO | Message: `docs(sisyphus): reconcile roadmap current-state assumptions` | Files: [`.sisyphus/plans/corrosion-roadmap.md`]

- [x] 8. Run a global .sisyphus reconciliation sweep and write manifest

  **What to do**: Run a final sweep across `.sisyphus/` to confirm stale path refs and contradictory Gate 1 wording are gone, then write a reconciliation manifest under `.sisyphus/evidence/` listing touched files, checks run, and any intentionally untouched files with rationale.
  **Must NOT do**: Do not change files outside `.sisyphus/`. Do not silently leave known stale references behind.

  **Recommended Agent Profile**:
  - Category: `unspecified-high` - Multi-file validation + manifest assembly.
  - Skills: [] - No special skill needed.
  - Omitted: [`playwright`] - No browser needed.

  **Parallelization**: Can Parallel: NO | Wave 3 | Blocks: F1-F4 | Blocked By: T1,T2,T3,T4,T5,T7

  **References**:
  - Pattern: `.sisyphus/` - full reconciliation target
  - Pattern: `.sisyphus/evidence/` - output manifest path
  - Pattern: `.sisyphus/notepads/corrosion-roadmap/issues.md` - intentionally likely unchanged

  **Acceptance Criteria**:
  - [ ] `! grep -R 'src/renderer\.rs' .sisyphus/`
  - [ ] `! grep -R 'src/main\.rs' .sisyphus/`
  - [ ] `! grep -R 'Gate 1 Complete' .sisyphus/notepads/corrosion-roadmap/`
  - [ ] `.sisyphus/evidence/sisyphus-reconciliation-manifest.md` exists and lists touched files + untouched rationale

  **QA Scenarios**:
  ```
  Scenario: all known stale-path refs are removed from .sisyphus
    Tool: Bash
    Steps: ! grep -R 'src/renderer\.rs' .sisyphus/ && ! grep -R 'src/main\.rs' .sisyphus/
    Expected: zero matches for dead path references
    Evidence: .sisyphus/evidence/task-8-global-grep.log

  Scenario: reconciliation manifest proves scope containment
    Tool: Bash
    Steps: test -f .sisyphus/evidence/sisyphus-reconciliation-manifest.md && git diff --name-only -- . | (grep '^\.sisyphus/' || true)
    Expected: manifest exists; modified files are .sisyphus-scoped
    Evidence: .sisyphus/evidence/task-8-manifest.log
  ```

  **Commit**: NO | Message: `docs(sisyphus): add reconciliation manifest` | Files: [`.sisyphus/evidence/sisyphus-reconciliation-manifest.md`, `.sisyphus/evidence/task-8-*.log`]

## Final Verification Wave (MANDATORY — after ALL implementation tasks)
> 4 review agents run in PARALLEL. ALL must APPROVE. Present consolidated results to user and get explicit "okay" before completing.
> **Do NOT auto-proceed after verification. Wait for user's explicit approval before marking work complete.**
> **Never mark F1-F4 as checked before getting user's okay.** Rejection or user feedback -> fix -> re-run -> present again -> wait for okay.
- [x] F1. Plan Compliance Audit — oracle
- [x] F2. Code Quality Review — unspecified-high
- [x] F3. Real Manual QA — unspecified-high
- [x] F4. Scope Fidelity Check — deep

## Commit Strategy
- No commits unless the user explicitly requests one.
- If later requested, commit only `.sisyphus/` changes and keep historical-evidence corrections grouped logically.

## Success Criteria
- `.sisyphus/` becomes internally self-consistent for current project state.
- Gate 0 evidence remains closed and trustworthy.
- Gate 1 evidence remains open/blocked and accurately describes the current implementation.
- Future roadmap intent survives while stale present-state assumptions are removed.
- No file outside `.sisyphus/` is modified.
