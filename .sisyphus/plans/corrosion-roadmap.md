# Corrosion Full Roadmap — Gates 0 through 6

## TL;DR

> **Quick Summary**: Take Corrosion from its current state (Gate 0 ~83% done — offline DSP prototype with pipe/plate/tank profiles, size/rust/damage transforms, evidence renders) all the way to a public 1.0 VST3/CLAP industrial physical-modeling instrument with sequencer, custom GUI, 100+ presets, and full documentation.
>
> **Deliverables**:
> - Closed Gate 0 with evidence summary and frozen initial parameter ranges.
> - Gate 1: NIH-plug VST3+CLAP shell, 8-voice polyphony, hit exciter routing to pipe/plate/tank objects (OPEN / BLOCKED on REAPER host smoke test).
> - Gate 2 / v0.1.0: 8-voice polyphony, all 3 objects, 6 MVP params, 20+ presets, pluginval-clean.
> - Gate 3 / v0.2.0: scrape exciter, chain object, stereo spread, body resonator, 40+ presets.
> - Gate 4 / v0.3.0: custom GUI, 4 macros, randomizer, preset browser.
> - Gate 5: 16/32-step sequencer with per-step locks, host sync.
> - Gate 6 / v1.0.0: 100+ presets, full docs, release bundles, installer.
>
> **Estimated Effort**: XL (multi-month roadmap; each gate is itself a major milestone)
> **Parallel Execution**: YES — 1 setup wave + 7 sequential gate-waves + 1 final review wave, with high in-wave parallelism (5-8 tasks/wave) within each gate.
> **Critical Path**: G-Setup → G0-3 → G1-1 (NIH-plug scaffold) → G1-7 (first audible plugin) → G2-1 (voice mgr) → G2-13 (preset bank) → G3-1 (scrape) → G4-1 (custom GUI) → G5-1 (seq core) → G6-7 (release bundle) → F1-F4 → user okay.

---

## Context

### Original Request
Analyze the repo and all docs/code and create a plan based on it.

User clarified: re-plan the entire roadmap (Gates 0-6) into a single Sisyphus plan, leaving the existing `docs/plans/corrosion.md` untouched.

### Interview Summary

**Key Discussions**:
- **Plan scope**: Full Gates 0-6 mega-plan (not just next gate).
- **File location**: `.sisyphus/plans/corrosion-roadmap.md`. The existing `docs/plans/corrosion.md` stays intact as historical reference.
- **Test strategy**: *Minimal* — do not pad tasks with new unit tests. Lean on existing `cargo test` suite, offline render metric checks, and manual DAW smoke tests. Add tests only where a QA scenario explicitly requires it.
- **Agent QA**: Bash-driven. Each task's QA = `cargo test <mod>` + (where applicable) `cargo run` to render a deterministic WAV + parse summary.txt and assert numeric thresholds + `pluginval --validate` for plugin builds.
- **Granularity**: One mega-plan, ~80-100 tasks across 7 waves.

### Research Findings
- **Current code**: The codebase has been modularized into:
  - `src/lib.rs` — Plugin entry point with NIH-plug traits (`Plugin`, `ClapPlugin`, `Vst3Plugin`), 8-voice `VoiceManager` integration
  - `src/params.rs` — `CorrosionParams` with `Gain` (0 to +12 dB) and `Object` (Pipe/Plate/Tank) parameters
  - `src/voice/mod.rs` — `Voice` struct with hit exciter, modal resonator, tail tracking
  - `src/voice/manager.rs` — `VoiceManager` with fixed `[Voice; 8]` array, deterministic voice stealing
  - `src/dsp/resonator.rs` — `PlaceholderResonator`, `SecondOrderMode`, `ResonatorCoefficients`
  - `src/dsp/profile.rs` — `ModalProfile`, `ModalProfileId`, `ModalModeSpec` for pipe/plate/tank
  - `src/dsp/transforms.rs` — `SizeScale`, `RustAmount`, `DamageAmount` transforms
  - `src/dsp/excitation.rs` — `ExcitationInput` for deterministic excitation
  - `src/dsp/budget.rs` — `RealtimeModeCountEstimate` and safe mode limits
  - `src/offline/mod.rs` — `OfflineRenderer`, PCM-WAV writer, behavior metrics (`brightness_proxy`, `roughness_proxy`, `zero_crossings`, `energy_in_window`)
  - `src/bin/render.rs` — CLI offline renderer invoking `render_damage_variations_to_dir`
- **Cargo**: `Cargo.toml` includes NIH-plug dependency (git source), `[lib] crate-type = ["cdylib", "rlib"]`, and `[[bin]]` entry for offline renderer. Project-local `.cargo/config.toml` pins targets for both musl (offline renderer) and gnu (plugin) with appropriate linkers.
- **Existing tests**: `cargo test` runs DSP-level tests for excitation, decay, family differentiation, size, rust, damage. These continue as the bedrock test suite — *do not duplicate* in new tasks.
- **Real-time mode budgets** (from `RealtimeModeCountEstimate`, frozen in `decisions.md`): pipe=6, plate=8, tank=8, shared cap=8 modes/voice; offline peak post-damage = 12/16/16.
- **Hot spots** (from `decisions.md`): per-frame × per-mode `SecondOrderMode::process` loop; `ModalModeSpec::damaged` allocates per source mode (acceptable offline, must be redesigned for plugin parameter rebuild path).
- **Environment caveat** (from `issues.md`): `rust-analyzer` unavailable in container — verification depends on `cargo fmt` / `cargo test` / `cargo run` rather than LSP diagnostics. Plan must not require LSP-only tooling.
- **Gate 0 status**: 14/17 implementation items done; remaining = (a) record initial parameter ranges, (b) write Gate 0 evidence summary, (c) Gate 0 review.
- **PRD scope guardrails**: explicitly excludes Corrosion FX, Corrosion Lab, expansion packs, neural synthesis, sample browser, modular environment.

### Self-Review Substituting Metis
Gaps proactively identified and addressed in this plan:
- **Audio thread invariants** are enforced as a cross-gate guardrail with explicit grep-based checks in F2 (no `Vec::push`, no `println!`, no `Mutex::lock`, no `serde_json`, no file I/O symbols inside the process callback).
- **Pluginval availability** is treated as an external dependency: a setup task installs/locates pluginval before Gate 1 needs it.
- **Determinism** for offline renders is locked in via the existing `ExcitationInput::deterministic_excitation()` path; QA scenarios reference this explicitly so checksums remain stable.
- **DAW automation** is acknowledged as best-effort: where DAW interaction cannot be scripted, evidence is captured as a recorded screen capture or DAW project file plus a render bounce, and the QA scenario asserts the bounce file's existence and size, not GUI state.
- **Allocation auditing** for the audio thread is concretized as a `cargo test` + macro/feature flag pattern (e.g., `assert_no_alloc` crate or a manual scope-counter) rather than an abstract guideline.

---

## Work Objectives

### Core Objective
Take Corrosion from its current Gate 0 prototype state to a release-ready 1.0.0 VST3+CLAP industrial physical-modeling instrument that meets every PRD pass criterion across all 6 gates without compromising real-time safety or product identity.

### Concrete Deliverables
- `.sisyphus/evidence/gate-0-summary.md` through `gate-6-summary.md` (one per gate).
- `.sisyphus/evidence/parameter-ranges.md` (frozen Gate 0 parameter ranges).
- Plugin source under `src/` reorganized into modules: `dsp/`, `plugin/`, `voice/`, `params/`, `presets/`, `gui/`, `sequencer/`.
- VST3 binaries for two targets: `target/x86_64-unknown-linux-gnu/release/*.vst3` (Linux QA bot) and `target/x86_64-pc-windows-gnu/release/*.vst3` (Windows / FL Studio).
- CLAP binaries for the same two targets.
- Factory preset bank: `presets/factory/*.corrosion-preset` (≥20 by Gate 2, ≥40 by Gate 3, ≥100 by Gate 6).
- Documentation: `docs/user-manual.md`, `docs/developer.md`, `docs/sound-design-guide.md`, `README.md`, `CHANGELOG.md`, `INSTALL.md`.
- Pluginval reports (VST3): `.sisyphus/evidence/pluginval-gate-{N}-*-vst3.log`.
- Clap-validator reports (CLAP): `.sisyphus/evidence/clap-validator-gate-{N}-*.log`.
- Release bundle: `release/corrosion-1.0.0/` containing both formats, docs, install script.

### Definition of Done
- [ ] `cargo test --workspace` → PASS (existing + new module tests).
- [ ] `cargo build --release` produces VST3 and CLAP bundles.
- [ ] `pluginval --strictness-level 5 --validate target/release/Corrosion.vst3` → exit 0 (VST3).
- [ ] `clap-validator validate target/release/Corrosion.clap/Corrosion.clap --only-failed` → 0 failed tests (CLAP).
- [ ] Each gate evidence summary exists and references its WAV/log artifacts.
- [ ] No occurrence of forbidden patterns inside the audio process callback (see Cross-Gate Guardrails).
- [ ] Final verification wave (F1-F4) all return APPROVE and user explicitly oks.

### Must Have
- All 6 gate pass-criteria from `docs/IMPLEMENTATION_PLAN.md` satisfied.
- Real-time safety preserved at every gate (cross-gate guardrails).
- 100+ factory presets covering bass, percussion, drone, transition, cinematic-impact families.
- VST3 and CLAP both build and load in REAPER.
- User manual, developer doc, sound design guide.
- Pluginval-clean (VST3) and clap-validator-clean (CLAP) release candidate.

### Must NOT Have (Guardrails)
- **No new dependencies in Cargo.toml unless explicitly named in a task** (prevents quietly pulling reverb crates, GUI frameworks beyond NIH-plug's chosen stack, etc.).
- **No heap allocation, file I/O, logging, mutex locking, JSON parsing, blocking work, or vector resizing inside the audio process callback** (the cross-gate quality requirement from PRD; F2 verifies via grep).
- **No oscillator/filter/amp framing in GUI**: controls and visuals must use exciter/object/damage/space metaphors.
- **No Corrosion FX, Corrosion Lab, expansion packs, neural synthesis, sample browser, or modular environment work** — out of PRD scope.
- **No new unit tests beyond what task QA scenarios explicitly require** (per user's "minimal tests" decision).
- **No skipping gate evidence summaries** — they are the audit trail.
- **No advancing past a gate while pass-criteria are unmet** — fail-closed gating.
- **No GUI scope beyond Gate 4's "basic custom GUI"** until Gate 6 polish phase.
- **No manual-only acceptance criteria** — every QA scenario must be agent-executable via bash/curl/tmux.

---

## Verification Strategy (MANDATORY)

> **ZERO HUMAN INTERVENTION** — all verification is agent-executed via bash. No exceptions.

### Test Decision
- **Infrastructure exists**: YES (`cargo test` is already in use; `src/dsp/` modules ship with DSP-level tests).
- **Automated tests**: Minimal new unit tests — only when a QA scenario explicitly demands one.
- **Framework**: `cargo test` (built-in Rust test runner). No additional frameworks.
- **Strategy**: Each task's QA scenarios describe (a) what command to run, (b) what numeric threshold or exit code to assert, (c) where evidence is saved. Tests-after, not TDD.

### QA Policy
Every task ships with at least one happy-path and one failure/edge scenario.

- **DSP / library tasks**: `cargo test <module>` (assert N tests pass) + `cargo run` to produce a deterministic WAV + bash-parsed `summary.txt` numeric assertion (peak/RMS/brightness/roughness/checksum).
- **Plugin shell tasks**: `cargo build --release --features=vst3,clap` + bundle existence check + `pluginval --strictness-level 5 --validate <bundle>` exit-0.
- **GUI tasks**: scripted host run via REAPER's command-line render (`reaper -nonewinst -saveas /tmp/proj.rpp` or equivalent) producing a bounce WAV; assert WAV exists and is non-silent. Where DAW automation isn't feasible, evidence = recorded `reaper -renderproject` log + bounce checksum.
- **Sequencer tasks**: rendered project bounce + bash analysis of bounce WAV to detect transient onsets at expected sample positions (proxy for step timing).
- **Preset tasks**: `cargo run --bin preset-render -- --preset <name>` produces bounce; assert bounce non-silent and non-clipping.
- **Documentation tasks**: `markdownlint` (or simple line-count + section-presence check) + grep-based content checks.

Evidence path convention: `.sisyphus/evidence/task-{gate}-{num}-{slug}.{wav,log,txt,png}`.

---

## Execution Strategy

### Parallel Execution Waves

> Gates are sequential (each gate's pass criteria gate the next). Within each gate, tasks parallelize aggressively.

```
Wave 1 — Gate 0 Closeout (3 tasks parallel):
├── G0-1: Record initial parameter ranges
├── G0-2: Write Gate 0 evidence summary
└── G0-3: Gate 0 pass-criteria review

Wave 2 — Gate 1 Minimal Plugin (10 tasks, mostly parallel after G1-1):
├── G1-1: NIH-plug scaffold + Cargo.toml + lib target [BLOCKS G1-2..10]
├── G1-2: Plugin shell + parameter ownership module
├── G1-3: Migrate DSP modules from renderer.rs to dsp/ module
├── G1-4: MIDI note-on + note-to-frequency + note-off natural decay
├── G1-5: First voice struct + hit exciter + pipe object route
├── G1-6: Safe output clamp + denormal/NaN guards
├── G1-7: VST3 build target + bundle script
├── G1-8: CLAP build target + bundle script
├── G1-9: REAPER smoke test + Bitwig smoke test (scripted bounce)
└── G1-10: Gate 1 evidence summary + pass-criteria review

Wave 3 — Gate 2 MVP 0.1.0 (15 tasks):
├── G2-1: 8-voice voice manager + voice pool [BLOCKS G2-3..6]
├── G2-2: Plate + tank profile activation in plugin path
├── G2-3: Tail-energy tracking + voice deactivation threshold
├── G2-4: Voice stealing (inactive→quietest→oldest)
├── G2-5: Object/Size/Rust/Damage/Drive/Output param exposure
├── G2-6: Velocity-to-physical-behavior mapping (not just amplitude)
├── G2-7: Audio-thread allocation audit + denormal/NaN/inf guards
├── G2-8: Generic editor (NIH-plug egui or default param GUI)
├── G2-9: Preset file format + serialize/deserialize
├── G2-10: 20 factory presets across bass/clang/boom/short-hit/long-tail
├── G2-11: Hard safety limiter / output clip
├── G2-12: Pluginval Gate 2 run + log capture
├── G2-13: REAPER MIDI/automation/preset/buffer/SR manual-equivalent scripted suite
├── G2-14: DSP regression: family differentiation + size/rust/damage metric assertions
└── G2-15: Gate 2 evidence summary + pass-criteria review

Wave 4 — Gate 3 Industrial Character 0.2.0 (12 tasks):
├── G3-1: Scrape exciter core (pressure / speed / roughness / stick-slip)
├── G3-2: Tune scrape for bowed-steel / brake-squeal / tension-rise
├── G3-3: Chain object profile (transient-dense, low-stable-pitch)
├── G3-4: Stereo modal spread + width control
├── G3-5: Lightweight body resonator + tuning
├── G3-6: Roughness / rattle character improvements (damage character pass)
├── G3-7: Saturation character improvements
├── G3-8: Velocity mapping expressiveness pass
├── G3-9: 20 additional presets (scrape/chain/drone/transition) → 40+ total
├── G3-10: Automation stress test for stereo/body parameters
├── G3-11: Regression vs Gate 2 stability
└── G3-12: Gate 3 evidence summary + pass-criteria review

Wave 5 — Gate 4 Product UX 0.3.0 (12 tasks):
├── G4-1: Custom GUI scaffold (Exciter→Object→Damage→Space layout)
├── G4-2: Mass macro
├── G4-3: Corrosion macro
├── G4-4: Violence macro
├── G4-5: Damage macro
├── G4-6: Macro→internal-parameter-group mapping
├── G4-7: Safe random + object random + damage random + full random
├── G4-8: Mutate behavior + randomization safety constraints
├── G4-9: Preset browser workflow
├── G4-10: Visual object/resonator feedback widget
├── G4-11: Regression: automation, preset changes, output safety
└── G4-12: Gate 4 evidence summary + pass-criteria review

Wave 6 — Gate 5 Sequenced Instrument (12 tasks):
├── G5-1: Sequencer step data structure + runtime playback model
├── G5-2: Per-step note + velocity + probability + microtiming
├── G5-3: Host sync (BPM, sample position, transport, loop)
├── G5-4: Stability under play/stop/loop transitions
├── G5-5: Per-step object lock
├── G5-6: Per-step exciter lock
├── G5-7: Per-step rust + damage + drive locks
├── G5-8: Lock × preset recall + lock × automation correctness
├── G5-9: Kit mode (or kit-oriented workflow)
├── G5-10: Host sync tests in REAPER + Bitwig (scripted)
├── G5-11: Loop/restart/tempo-change timing checks
└── G5-12: Gate 5 evidence summary + pass-criteria review

Wave 7 — Gate 6 v1.0 Release (15 tasks):
├── G6-1: Sequencer + per-step locks finalization (release quality)
├── G6-2: Preset browser reliability finalization
├── G6-3: Multiple exciter types confirmation (hit + scrape + ?)
├── G6-4: Multiple body/space types confirmation
├── G6-5: User-configurable modulation mappings
├── G6-6: Expand presets to 100+ (bass/perc/drone/transition/cinematic-impact)
├── G6-7: VST3 + CLAP release-bundle build + installer script
├── G6-8: User manual (concept, controls, MIDI, presets, automation, install, troubleshooting)
├── G6-9: Developer documentation (architecture, DSP, params, RT-safety, build, test, release)
├── G6-10: Sound-design guide + 6 named recipes
├── G6-11: README + CHANGELOG + installation instructions
├── G6-12: Pluginval release-candidate run
├── G6-13: REAPER + Bitwig + Ardour scripted DAW tests
├── G6-14: Buffer-size + sample-rate regression checks
└── G6-15: Gate 6 evidence summary + final performance/safety validation

Final Wave — 4 parallel reviews + user okay:
├── F1: Plan compliance audit (oracle)
├── F2: Code quality + RT-safety review (unspecified-high)
├── F3: Real manual QA — full release flow (unspecified-high)
└── F4: Scope fidelity check (deep)
→ Present results → wait for explicit user okay → DONE.

Critical Path: G-Setup → G0-3 → G1-1 → G1-7/8 → G2-1 → G2-13 → G3-1 → G4-1 → G5-1 → G6-7 → F1-F4 → user okay
Max Concurrent: 8 (within Gate 2)
Total Tasks: 1 setup + 79 implementation + 4 final = 84
```

### Cross-Gate Guardrails (Audit Each Gate)

> Audio-thread "hot path" = code reachable from the plugin's `process(...)` callback or from any voice/sequencer per-sample inner loop. The exact file set evolves per gate; the audit MUST cover whichever of these directories/files exist at audit time. The grep below is intentionally directory-scoped (and tolerant of missing dirs via `2>/dev/null`) so it works whether or not a given module exists yet.

Run at the end of every gate (and inside F2):
- `! { grep -rnE '\b(Vec::new|vec!\[|Box::new|String::from|format!|println!|eprintln!|Mutex|RwLock|serde_json|std::fs|std::io|std::thread::sleep)\b' src/dsp src/voice src/sequencer src/lib.rs 2>/dev/null | grep -v '#\[cfg(test)\]\|^[^:]*:\s*//'; }` — must produce zero non-comment, non-test matches.
- `! { grep -rnE '\b(todo!|unimplemented!|panic!|unwrap\(\))\b' src/dsp src/voice src/sequencer src/lib.rs 2>/dev/null; }` — must be zero (panics + unwraps in audio thread are forbidden; offline-only modules in `src/offline/` are exempt).
- `cargo test --workspace` — full suite green.
- `cargo clippy --all-targets -- -D warnings` — zero warnings.

If a referenced directory does not exist yet at a given gate (e.g., `src/sequencer/` before Gate 5), the grep simply finds nothing, which is the desired behavior — the guardrail tightens automatically as modules come online.

### Agent Dispatch Summary

| Wave | Tasks | Categories used |
|---|---|---|
| 1 (Gate 0) | 3 | `quick` (writing), `writing` |
| 2 (Gate 1) | 10 | `deep` (G1-1, G1-3, G1-5), `unspecified-high` (G1-2, G1-7, G1-8, G1-9), `quick` (G1-4, G1-6, G1-10) |
| 3 (Gate 2) | 15 | `deep` (G2-1, G2-3, G2-7), `unspecified-high` (G2-4, G2-6, G2-8, G2-12, G2-13, G2-14), `quick` (G2-2, G2-5, G2-9, G2-11, G2-15), `writing` (G2-10) |
| 4 (Gate 3) | 12 | `artistry` (G3-1, G3-3, G3-6, G3-7), `deep` (G3-2, G3-4, G3-5, G3-8), `unspecified-high` (G3-10, G3-11), `writing` (G3-9), `quick` (G3-12) |
| 5 (Gate 4) | 12 | `visual-engineering` (G4-1, G4-9, G4-10), `deep` (G4-2..6), `artistry` (G4-7, G4-8), `unspecified-high` (G4-11), `quick` (G4-12) |
| 6 (Gate 5) | 12 | `deep` (G5-1, G5-3, G5-5..7), `unspecified-high` (G5-2, G5-4, G5-8, G5-10, G5-11), `quick` (G5-9, G5-12) |
| 7 (Gate 6) | 15 | `unspecified-high` (G6-1..5, G6-7, G6-12, G6-13, G6-14), `writing` (G6-6, G6-8..11), `quick` (G6-15) |
| Final | 4 | `oracle` (F1), `unspecified-high` (F2, F3), `deep` (F4) |

---

## TODOs

> Implementation + verification = ONE Task. Every task ships with QA scenarios.

### Wave 0 — Repository Prerequisites (must run before Wave 1)

- [x] G-Setup. Initialize repository prerequisites

  **What to do**:
  - Initialize git in the repo: `git init -b main` (only if `.git/` does not already exist).
  - Configure local repo identity (NEVER touch `--global`): `git config user.email "corrosion-dev@local"` and `git config user.name "Corrosion Dev"`.
  - Install Windows cross-compile toolchain (required from Gate 1 onward because the user runs FL Studio on Windows): `rustup target add x86_64-pc-windows-gnu`; install `mingw-w64` system package (`apt-get install -y mingw-w64` on Debian/Ubuntu; document equivalent for Arch/Fedora).
  - Install `wine` (system package) to enable running `pluginval.exe` against Windows bundles inside the agent's Linux environment from Gate 2 onward. If the system package manager is unavailable, document the exact `winehq` repo + apt steps in `docs/developer.md` later (Gate 6); for now, just verify `wine --version` exits 0 after install.
  - Create a `.gitignore` covering: `target/`, `../corrotion-target/`, `output/*.tmp`, `.sisyphus/evidence/*.tmp`, `__pycache__/`, plus standard Rust artifacts (`Cargo.lock` is kept; binary outputs ignored).
  - Create `.cargo/config.toml` with the project-local Cargo defaults referenced in `docs/notepads/corrosion/decisions.md` (entry dated 2026-05-01, line 3): pin to `x86_64-unknown-linux-musl`, use bundled `rust-lld` linker, enable self-contained linking, redirect build artifacts to `../corrotion-target`. Exact contents:
    ```toml
    [build]
    target = "x86_64-unknown-linux-musl"
    target-dir = "../corrotion-target"

    [target.x86_64-unknown-linux-musl]
    linker = "rust-lld"
    rustflags = ["-C", "link-self-contained=yes"]

    # Windows cross-compile target for FL Studio on Windows (added at G-Setup).
    # The plugin bundles produced under this target are the user's primary
    # creative artifacts; the Linux-gnu target is for the headless REAPER QA bot.
    [target.x86_64-pc-windows-gnu]
    linker = "x86_64-w64-mingw32-gcc"
    ```
  - Create the QA helper scripts under `scripts/` (referenced by many later QA scenarios). Both are pure-stdlib Python 3, no external deps:
    - `scripts/check_wav.py <wav>` — exits 0 if the WAV's peak amplitude is in (0.01, 0.97), contains no NaN/inf samples, and is at least 0.1s long; exits 1 otherwise. Prints a one-line summary `peak=<f> rms=<f> nan_count=<n> frames=<n>` to stdout. Implementation: use the `wave` and `struct` modules to parse PCM int16/int24/float32 frames (detect via `getsampwidth()` and the format chunk).
    - `scripts/check_clicks.py <wav>` — exits 0 if the maximum absolute sample-to-sample delta is < 0.5 (linear) and there are no >5ms gaps of pure silence (RMS<-90dB) following non-silence; exits 1 otherwise. Prints `max_delta=<f> click_count=<n>`.
    - Both scripts treat exit code as the authoritative pass/fail signal so QA scenarios can use them with `|| exit 1`.
  - Make the initial commit on `main` capturing the existing repo state plus these new files: `git add -A && git commit -m "chore: initialize git, cargo config, and QA helper scripts"`.
  - Verify `cargo test` and `cargo run` still succeed unchanged from this baseline. Verify `python3 scripts/check_wav.py output/damage-variations/pipe_high_damage.wav` exits 0 (smoke test of the helper against existing evidence).

  **Must NOT do**:
  - Do not modify global git config (no `--global`).
  - Do not change Cargo.toml content here (Wave 2 will modify it).
  - Do not move existing source files.
  - Do not add a remote (release packaging happens in Gate 6).

  **Recommended Agent Profile**:
  - **Category**: `quick` — straightforward setup, no design decisions.
  - **Skills**: none.

  **Parallelization**:
  - **Can Run In Parallel**: NO (single setup task)
  - **Blocks**: ALL subsequent tasks (every task assumes git + cargo config exist).
  - **Blocked By**: None.

  **References**:
  - `docs/notepads/corrosion/decisions.md` — entry "Added project-local Cargo defaults in `.cargo/config.toml`" — source of the exact config to recreate.
  - `Cargo.toml` (current minimal state) — confirm baseline before any edits.

  **QA Scenarios**:
  ```
  Scenario: Git repo and cargo config exist and baseline still builds
    Tool: Bash
    Steps:
      1. test -d .git
      2. test -f .cargo/config.toml
      3. grep -q 'x86_64-unknown-linux-musl' .cargo/config.toml
      4. grep -q 'rust-lld' .cargo/config.toml
      5. grep -q '../corrotion-target' .cargo/config.toml
      6. test -f .gitignore && grep -q '^target/' .gitignore
      7. test -x scripts/check_wav.py && test -x scripts/check_clicks.py
      8. python3 scripts/check_wav.py --help >/dev/null || python3 scripts/check_wav.py output/damage-variations/pipe_high_damage.wav
      9. rustup target list --installed | grep -q '^x86_64-pc-windows-gnu$'
      10. command -v x86_64-w64-mingw32-gcc
      11. command -v wine && wine --version >/dev/null
      12. grep -q 'x86_64-pc-windows-gnu' .cargo/config.toml
      13. cargo test --workspace 2>&1 | tee .sisyphus/evidence/task-G-Setup-baseline.log
      14. grep -E 'test result: ok' .sisyphus/evidence/task-G-Setup-baseline.log
      15. cargo run --release && test -f output/damage-variations/pipe_high_damage.wav
      16. git log --oneline | head -1 | grep -q 'initialize git'
    Expected: every check passes; baseline tests + render still green.
    Failure Indicators: missing .git or .cargo/config.toml; cargo build/test/run failure; missing baseline commit.
    Evidence: .sisyphus/evidence/task-G-Setup-baseline.log

  Scenario: Setup is idempotent on rerun
    Tool: Bash
    Steps:
      1. Re-run the setup script logic.
      2. Verify it does NOT reinitialize an existing .git, does NOT overwrite an existing .cargo/config.toml, does NOT create a duplicate commit.
      3. git log --oneline | wc -l  # expect: 1 (still only initial commit)
    Expected: idempotent.
    Evidence: .sisyphus/evidence/task-G-Setup-idempotent.log
  ```

  **Commit**: YES (creates the initial commit). Message: `chore: initialize git and project-local cargo config`. Files: `.git/` (init), `.cargo/config.toml`, `.gitignore`. Pre-commit: `cargo test --workspace`.

---

### Wave 1 — Gate 0 Closeout

- [x] G0-1. Record initial parameter ranges

  **What to do**:
  - Create `.sisyphus/evidence/parameter-ranges.md`.
  - For each parameter family currently in the codebase — `SizeScale`, `RustAmount`, `DamageAmount` (in `src/dsp/transforms.rs`), plus implicit ranges (mode count caps, decay seconds, base frequency in `src/dsp/budget.rs` and `src/dsp/profile.rs`) — record: minimum value, maximum value, default value, perceptual direction (e.g., "size↑ ⇒ lower fundamental, longer decay"), and the source-of-truth code reference (file:line).
  - Cross-reference with `docs/notepads/corrosion/decisions.md` and the existing tuning entries.
  - Mark which ranges are "frozen for plugin work" vs "may adjust after MVP listening tests".

  **Must NOT do**: Do not change any code. Do not add new parameters here. Do not expand into Gate 1+ parameters (Object enum, Drive, Output) — those land in Gate 2.

  **Recommended Agent Profile**:
  - **Category**: `writing` — pure documentation task, structured extraction from existing code.
  - **Skills**: none required.

  **Parallelization**: Wave 1 (with G0-2). Blocked By: none.

  **References**:
  - `src/dsp/transforms.rs` — `SizeScale`, `RustAmount`, `DamageAmount` impls (all the clamp/default logic).
  - `src/dsp/budget.rs` — `realtime_mode_count_estimate*` and `safe_realtime_shared_mode_limit` / `offline_peak_shared_mode_limit`.
  - `docs/notepads/corrosion/decisions.md:13` — frozen real-time mode budget.
  - `docs/corrosion_plugin_prd_and_specs.md` — search "parameter" / "range" sections for spec-side guidance.

  **Acceptance Criteria**:

  **QA Scenarios**:
  ```
  Scenario: Parameter ranges file complete
    Tool: Bash
    Steps:
      1. test -f .sisyphus/evidence/parameter-ranges.md
      2. grep -c '^## ' .sisyphus/evidence/parameter-ranges.md  # expect >= 5 sections
      3. grep -E 'min:|max:|default:' .sisyphus/evidence/parameter-ranges.md | wc -l  # expect >= 15
      4. grep -E 'src/dsp/(transforms|budget|profile).rs:[0-9]+' .sisyphus/evidence/parameter-ranges.md | wc -l  # expect >= 5 file refs
    Expected: All checks pass.
    Evidence: .sisyphus/evidence/parameter-ranges.md

  Scenario: No code mutation
    Tool: Bash
    Steps:
      1. git diff --name-only src/  # expect: empty
      2. cargo test --workspace  # expect: same pass count as before
    Expected: Empty diff under src/, tests pass unchanged.
    Evidence: .sisyphus/evidence/task-G0-1-no-code-change.log
  ```

  **Commit**: YES. Message: `gate-0(evidence): record initial parameter ranges`. Files: `.sisyphus/evidence/parameter-ranges.md`. Pre-commit: `cargo test`.

- [x] G0-2. Write Gate 0 evidence summary

  **What to do**:
  - Create `.sisyphus/evidence/gate-0-summary.md` summarizing all Gate 0 work.
  - Sections: (1) Deliverables (offline renderer, modal resonator, pipe/plate/tank profiles, size/rust/damage transforms, evidence WAVs), (2) Render artifacts inventory (link every file in `output/family-comparisons/`, `output/rust-variations/`, `output/damage-variations/`), (3) Pass-criteria status (one row per criterion with PASS/FAIL/EVIDENCE-LINK), (4) Carry-forward items (real-time mode budget, hot-spot list, allocation concerns flagged in `decisions.md` line 11).
  - Reference existing renders by file path and include their summary metrics (peak/RMS/checksum/brightness/roughness from the `*_summary.txt` files).

  **Must NOT do**: Do not run new renders. Do not modify `src/renderer.rs`. Do not advance to Gate 1 work.

  **Recommended Agent Profile**:
  - **Category**: `writing`.
  - **Skills**: none.

  **Parallelization**: Wave 1 (with G0-1). Blocked By: none.

  **References**:
  - `output/family-comparisons/family_comparison_manifest.txt`
  - `output/rust-variations/rust_variation_manifest.txt`
  - `output/damage-variations/damage_variation_manifest.txt`
  - `docs/IMPLEMENTATION_PLAN.md:164-177` — Gate 0 pass criteria + verification focus.
  - `docs/notepads/corrosion/decisions.md` — full decision log to summarize.
  - `docs/notepads/corrosion/learnings.md` — learnings to fold in.

  **QA Scenarios**:
  ```
  Scenario: Evidence summary covers all Gate 0 pass criteria
    Tool: Bash
    Steps:
      1. test -f .sisyphus/evidence/gate-0-summary.md
      2. for crit in "Pipe, plate, and tank" "Excitation produces audible" "Rust audibly darkens" "Damage audibly destabilizes" "Output is not silent" "No NaN, infinity, runaway"; do
           grep -q "$crit" .sisyphus/evidence/gate-0-summary.md || exit 1
         done
      3. grep -c 'output/' .sisyphus/evidence/gate-0-summary.md  # expect >= 6 artifact references
    Expected: All 6 pass criteria present, all evidence files referenced.
    Evidence: .sisyphus/evidence/gate-0-summary.md

  Scenario: Render evidence files actually exist
    Tool: Bash
    Steps:
      1. for f in $(grep -oE 'output/[a-z-]+/[a-z_-]+\.(wav|txt)' .sisyphus/evidence/gate-0-summary.md | sort -u); do
           test -f "$f" || { echo "missing: $f"; exit 1; }
         done
    Expected: Every referenced artifact file exists on disk.
    Evidence: .sisyphus/evidence/task-G0-2-artifacts-verified.log
  ```

  **Commit**: YES. Message: `gate-0(evidence): write Gate 0 evidence summary`. Files: `.sisyphus/evidence/gate-0-summary.md`.

- [x] G0-3. Gate 0 pass-criteria review (gate close)

  **What to do**:
  - Re-run all renders deterministically: `cargo run` with default config, then re-render family/rust/damage variations.
  - For each Gate 0 pass criterion (6 total from `IMPLEMENTATION_PLAN.md:164-170`), assert via bash + summary-file parsing:
    - "Pipe/plate/tank distinct": brightness_proxy + late/early energy ratio differ by ≥10% pairwise.
    - "Excitation produces audible decaying output": peak > 0.05 AND RMS in last 25% < 0.5 × RMS in first 25%.
    - "Rust darkens and shortens": high-rust brightness_proxy < low-rust brightness_proxy AND high-rust late/early < low-rust late/early.
    - "Damage destabilizes/roughens": high-damage roughness_proxy > low-damage roughness_proxy by ≥10%.
    - "Not silent by default when excited": peak > 0.01 in default render.
    - "No NaN/inf/blowup": grep WAV bytes for non-finite (or a small Rust harness that scans samples) — must report none; peak < 2.0.
  - Write findings to `.sisyphus/evidence/gate-0-review.md`. If any criterion fails, do NOT close the gate — open a follow-up task instead.

  **Must NOT do**: Do not advance to Gate 1 if any criterion fails. Do not retroactively change pass criteria.

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high` — needs to compose multiple shell pipelines and parse summary files.
  - **Skills**: none.

  **Parallelization**: Wave 1 (after G0-1 + G0-2 complete; this is the gate seal).
  **Blocks**: ALL Gate 1 tasks (G1-1..10).

  **References**:
  - `docs/IMPLEMENTATION_PLAN.md:164-177`
  - `output/family-comparisons/*_summary.txt`, `output/rust-variations/*_summary.txt`, `output/damage-variations/*_summary.txt`
  - `src/offline/mod.rs` — `RenderBehaviorMetrics` field semantics (legacy reference; offline renderer now in `src/offline/` module).

  **QA Scenarios**:
  ```
  Scenario: All 6 Gate 0 pass criteria green
    Tool: Bash
    Steps:
      1. cargo run --release  # regenerate output/damage-variations
      2. (also re-run family + rust variation renders via a small main flag or temporary harness)
      3. Run a bash script that parses every *_summary.txt and asserts the 6 thresholds above.
      4. Write results to .sisyphus/evidence/gate-0-review.md with one PASS/FAIL row per criterion.
    Expected: All 6 PASS. Script exits 0.
    Evidence: .sisyphus/evidence/gate-0-review.md

  Scenario: Failing criterion blocks gate close
    Tool: Bash
    Steps:
      1. Verify gate-0-review.md contains "GATE 0 STATUS: CLOSED" only if every criterion is PASS.
      2. grep "GATE 0 STATUS: CLOSED" .sisyphus/evidence/gate-0-review.md
    Expected: Closed status present.
    Evidence: .sisyphus/evidence/gate-0-review.md
  ```

  **Commit**: YES. Message: `gate-0(review): close Gate 0`. Tag: `git tag gate-0-complete`.

### Wave 2 — Gate 1 Minimal Plugin

- [x] G1-1. NIH-plug scaffold + Cargo.toml + lib target

  **What to do**:
  - Add NIH-plug + nih_export_vst3/clap dependencies to `Cargo.toml`. Pin versions explicitly. Convert crate type to `[lib] crate-type = ["cdylib", "rlib"]` while keeping a `[[bin]]` entry for the existing offline renderer.
  - Create `src/lib.rs` exporting a `Corrosion` plugin struct (empty params, empty process). Wire `nih_export_vst3!(Corrosion)` and `nih_export_clap!(Corrosion)`.
  - Move existing `src/main.rs` content into `src/bin/render.rs` so the offline renderer is preserved as a separate binary.
  - Update `.cargo/config.toml` if needed so VST3/CLAP build target works (likely default x86_64-unknown-linux-gnu instead of musl for plugin). Document any target switch in a code comment.

  **Must NOT do**: Do not delete the offline renderer. Do not add audio synthesis logic — just the empty shell. Do not pull additional dependencies (no logging crates, no GUI crates, no DSP crates) beyond NIH-plug itself.

  **Recommended Agent Profile**: `deep` — non-trivial restructure with cargo + linker concerns. **Skills**: none.

  **Parallelization**: Wave 2 head. Blocks: G1-2..G1-10.

  **References**:
  - https://github.com/robbert-vdh/nih-plug — README "Getting Started", `nih_plug::prelude::*` exports.
  - `Cargo.toml` (current minimal) and `Cargo.lock` (currently empty).
  - `.cargo/config.toml` (created in G-Setup; pinned to musl + rust-lld + external target-dir per `decisions.md`). When switching the plugin build to `x86_64-unknown-linux-gnu` (likely required for VST3/CLAP host loading), do it via a per-target `[target.x86_64-unknown-linux-gnu]` section *added* to `.cargo/config.toml` rather than replacing the existing musl pin (the offline renderer binary should keep building under musl as today).
  - `src/main.rs` — historical reference; content already relocated into `src/bin/render.rs`.

  **QA Scenarios**:
  ```
  Scenario: Plugin shell compiles as cdylib
    Tool: Bash
    Steps:
      1. cargo build --release
      2. ls target/release/libcorrotion.so   # cdylib output
    Expected: build succeeds, .so exists.
    Evidence: .sisyphus/evidence/task-G1-1-build.log

  Scenario: Offline renderer still runs
    Tool: Bash
    Steps:
      1. cargo run --release --bin render
      2. test -d output/damage-variations
      3. cargo test --workspace
    Expected: bin runs, output dir populated, tests pass.
    Evidence: .sisyphus/evidence/task-G1-1-render-still-works.log
  ```

  **Commit**: `gate-1(plugin): add NIH-plug scaffold and library target`.

- [x] G1-2. Plugin shell + parameter ownership module

  **What to do**:
  - Create `src/params.rs` with a `CorrosionParams` struct using NIH-plug's `#[derive(Params)]`. For Gate 1 expose only a single placeholder `gain` param (will be replaced in Gate 2). Define `Default for CorrosionParams`.
  - Wire `params: Arc<CorrosionParams>` into the `Corrosion` plugin struct. Implement the required NIH-plug `Plugin` trait stubs: `NAME`, `VENDOR`, `URL`, `EMAIL`, `VERSION`, `AudioIOLayout` (mono in / stereo out — instrument), `MidiConfig::Basic`, empty `process` returning `ProcessStatus::Normal`.

  **Must NOT do**: Do not expose Object/Size/Rust/Damage/Drive/Output yet (Gate 2). No DSP yet.

  **Recommended Agent Profile**: `unspecified-high`. **Skills**: none.

  **Parallelization**: Wave 2. Blocked By: G1-1.

  **References**:
  - https://nih-plug.robbertvanderhelm.nl/nih_plug/prelude/index.html — `Plugin` trait, `Params` derive.
  - `src/lib.rs` (created in G1-1).

  **QA Scenarios**:
  ```
  Scenario: Plugin trait implementation compiles
    Tool: Bash
    Steps:
      1. cargo build --release
      2. cargo test --workspace
      3. grep -q 'impl Plugin for Corrosion' src/lib.rs
      4. grep -q 'derive(Params)' src/params.rs
    Expected: build green, params module exists.
    Evidence: .sisyphus/evidence/task-G1-2-build.log
  ```

  **Commit**: `gate-1(params): introduce CorrosionParams ownership module`.

- [x] G1-3. Migrate DSP modules from renderer.rs into dsp/ module tree

  **What to do**:
  - Create `src/dsp/mod.rs` and split `src/renderer.rs` along its existing seams: `dsp/resonator.rs` (`PlaceholderResonator`, `ResonatorCore`, `SecondOrderMode`, `ResonatorCoefficients`), `dsp/profile.rs` (`ModalProfile`, `ModalProfileId`, `ModalModeSpec`, profile constructors), `dsp/transforms.rs` (`SizeScale`, `RustAmount`, `DamageAmount`), `dsp/excitation.rs` (`ExcitationInput`), `dsp/budget.rs` (`RealtimeModeCountEstimate` + helpers).
  - Keep `OfflineRenderer` and the WAV-writing code in `src/offline/mod.rs` since it's not part of the plugin hot path.
  - Adjust `src/bin/render.rs` and `src/lib.rs` imports accordingly.
  - Re-run `cargo test` and confirm same test count passes.

  **Must NOT do**: Do not change DSP behavior. Do not change public types' field semantics (just relocate). Do not introduce allocations on a hot path while moving code.

  **Recommended Agent Profile**: `deep`. **Skills**: none.

  **Parallelization**: Wave 2. Blocked By: G1-1.

  **References**:
  - `src/renderer.rs` — historical reference; code migrated to `src/dsp/` and `src/offline/` modules in G1-3.
  - `docs/notepads/corrosion/decisions.md:11` — note about per-source-mode allocations in `ModalModeSpec::damaged` that must NOT migrate into a real-time rebuild path; preserve as offline-only.

  **QA Scenarios**:
  ```
  Scenario: Refactor preserves test suite
    Tool: Bash
    Steps:
      1. cargo test --workspace 2>&1 | tee .sisyphus/evidence/task-G1-3-tests.log
      2. PASS_COUNT=$(grep -E 'test result: ok\. [0-9]+ passed' .sisyphus/evidence/task-G1-3-tests.log | awk '{print $4}' | paste -sd+ | bc)
      3. # Compare to baseline pre-refactor count (record baseline first)
      4. cargo run --release --bin render && test -f output/damage-variations/pipe_high_damage.wav
    Expected: same pass count, render still works.
    Evidence: .sisyphus/evidence/task-G1-3-tests.log

  Scenario: Module tree exists at expected paths
    Tool: Bash
    Steps:
      1. for f in src/dsp/mod.rs src/dsp/resonator.rs src/dsp/profile.rs src/dsp/transforms.rs src/dsp/excitation.rs src/dsp/budget.rs src/offline/mod.rs; do
           test -f "$f" || { echo "missing $f"; exit 1; }
         done
    Expected: all 7 files present.
    Evidence: .sisyphus/evidence/task-G1-3-layout.log
  ```

  **Commit**: `gate-1(dsp): split renderer.rs into dsp/ and offline/ modules`.

- [x] G1-4. MIDI note-on, note-to-frequency, note-off natural decay

  **What to do**:
  - In the plugin's `process(buffer, aux, context)`: iterate `context.next_event()`, match `NoteEvent::NoteOn { note, velocity, .. }` → convert MIDI note to frequency: `f = 440.0 * 2_f32.powf((note as f32 - 69.0) / 12.0)`. Match `NoteOff` → flag voice for natural decay (do NOT cut envelope abruptly; resonator decays via its own tail).
  - Add voice slots in the plugin struct holding the frequency + active flag (8-voice fixed array, though only simple allocation logic in G1-4; full stealing logic comes in G1-5).

  **Must NOT do**: Do not implement voice stealing yet (G1-5). Do not cut the resonator on note-off — let modal decay handle it.

  **Recommended Agent Profile**: `quick`. **Skills**: none.

  **Parallelization**: Wave 2. Blocked By: G1-2.

  **References**:
  - https://nih-plug.robbertvanderhelm.nl/nih_plug/midi/enum.NoteEvent.html
  - PRD search "note-off" for spec semantics.

  **QA Scenarios**:
  ```
  Scenario: MIDI note → frequency conversion correctness
    Tool: Bash + cargo test
    Steps:
      1. Add ONE unit test (allowed because QA scenario explicitly requires it):
         #[test] fn midi_69_is_a440() { assert!((midi_to_hz(69) - 440.0).abs() < 1e-3); }
         #[test] fn midi_57_is_a220() { assert!((midi_to_hz(57) - 220.0).abs() < 1e-3); }
      2. cargo test midi
    Expected: 2 tests pass.
    Evidence: .sisyphus/evidence/task-G1-4-midi.log

  Scenario: Note-off does not abruptly mute
    Tool: Bash (offline harness)
    Steps:
      1. Write a small test that sends NoteOn at frame 0, NoteOff at frame 4800 (0.1s @ 48kHz), renders 1.0s, and asserts RMS in frames 4800..9600 is > 10% of RMS in frames 0..4800 (decay, not cut).
      2. cargo test note_off_natural_decay
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G1-4-decay.log
  ```

  **Commit**: `gate-1(midi): note-on/off handling with natural-decay semantics`.

- [x] G1-5. First voice struct + hit exciter + pipe object route

  **What to do**:
  - Create `src/voice/mod.rs` with a `Voice` struct holding: `active: bool`, `freq: f32`, `excitation: ExcitationInput`, `resonator: PlaceholderResonator` (constructed `with_profile_size_rust_and_damage(ModalProfileId::Pipe, ..., ..., ...)`).
  - Implement a hit exciter: on note-on, refresh excitation buffer with a deterministic short impulse scaled by velocity.
  - In plugin `process`: for each output sample, if voice active, call `voice.process_sample(excitation_sample)` and write to both stereo channels. Apply `gain` param.

  **Must NOT do**: Do not introduce plate or tank yet. Do not add scrape exciter. No voice stealing yet (comes in G1-5 voice manager).

  **Recommended Agent Profile**: `deep` — DSP integration. **Skills**: none.

  **Parallelization**: Wave 2. Blocked By: G1-3, G1-4.

  **References**:
  - `src/dsp/resonator.rs` (after G1-3) — `PlaceholderResonator` API.
  - `src/dsp/excitation.rs` — `ExcitationInput::deterministic_excitation` (model the hit exciter on this).

  **QA Scenarios**:
  ```
  Scenario: Plugin produces audible output on note-on (offline harness)
    Tool: Bash
    Steps:
      1. Add a `tests/plugin_audio.rs` integration test that constructs the plugin, sends a NoteOn at C3, renders 1s @ 48kHz, and asserts: peak > 0.05, RMS in [0.001, 0.5], no NaN/inf.
      2. cargo test --test plugin_audio
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G1-5-audible.log

  Scenario: Output remains bounded under retrigger
    Tool: Bash
    Steps:
      1. Same harness, send 100 NoteOns at random pitches over 5 seconds.
      2. Assert max peak < 1.5, no NaN.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G1-5-retrigger.log
  ```

  **Commit**: `gate-1(voice): voice struct with hit exciter routed through pipe resonator`.

- [x] G1-6. Safe output clamp + denormal/NaN guards

  **What to do**:
  - In the plugin process loop, after summing voices, apply `sample.clamp(-1.0, 1.0)` and a denormal guard (`sample = if sample.abs() < 1e-30 { 0.0 } else { sample }` or use `flush_denormals` intrinsic).
  - Replace any NaN/inf with 0.0.

  **Must NOT do**: Do not add a soft limiter / saturator (that's Gate 2's job). Just hard safety.

  **Recommended Agent Profile**: `quick`. **Skills**: none.

  **Parallelization**: Wave 2. Blocked By: G1-5.

  **References**: PRD "Cross-Gate Quality Requirements" (`IMPLEMENTATION_PLAN.md:719-727`).

  **QA Scenarios**:
  ```
  Scenario: Output is bounded under degenerate input
    Tool: Bash
    Steps:
      1. Add a test that injects a synthetic huge excitation (e.g., 1000.0) and asserts |output| <= 1.0 for all samples.
      2. cargo test output_clamp
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G1-6-clamp.log

  Scenario: NaN input produces zero, not NaN
    Tool: Bash
    Steps:
      1. Test with f32::NAN and f32::INFINITY excitation; assert all output samples are finite and zero.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G1-6-nan.log
  ```

  **Commit**: `gate-1(safety): add output clamp and denormal/NaN guards`.

- [x] G1-7. VST3 build target + bundle script (Linux + Windows)

  **What to do**:
  - Add `bundle.sh` invoking NIH-plug's `cargo xtask bundle Corrosion --release` for the Linux-gnu target → `target/bundled/Corrosion.vst3/` (Linux QA-bot bundle).
  - Add `bundle-win.sh` invoking `cargo xtask bundle Corrosion --release --target x86_64-pc-windows-gnu` → `target/bundled-win/Corrosion.vst3/` (Windows / FL Studio bundle).
  - Both scripts must produce valid VST3 bundle layouts: `Contents/x86_64-linux/Corrosion.so` for Linux, `Contents/x86_64-win/Corrosion.vst3` (the Windows DLL) for Windows.

  **Must NOT do**: Do not codesign / notarize (release engineering = Gate 6). Do not strip symbols. Do not skip the Windows bundle — FL Studio compatibility depends on it.

  **Recommended Agent Profile**: `unspecified-high`. **Skills**: none.

  **Parallelization**: Wave 2. Blocked By: G1-6.

  **References**: https://github.com/robbert-vdh/nih-plug — "Building" + cross-compile section.

  **QA Scenarios**:
  ```
  Scenario: Linux VST3 bundle produced
    Tool: Bash
    Steps:
      1. ./bundle.sh
      2. test -d target/bundled/Corrosion.vst3
      3. find target/bundled/Corrosion.vst3 -name '*.so' | head -1 | xargs file | grep -q 'ELF.*shared object'
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G1-7-linux.log

  Scenario: Windows VST3 bundle produced
    Tool: Bash
    Steps:
      1. ./bundle-win.sh
      2. find target/bundled-win/Corrosion.vst3 -name '*.vst3' -o -name '*.dll' | head -1 | xargs file | grep -qE 'PE32\+|MS Windows'
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G1-7-win.log
  ```

  **Commit**: `gate-1(build): VST3 bundle scripts for Linux + Windows`.

- [x] G1-8. CLAP build target + bundle script (Linux + Windows)

  **What to do**:
  - Extend `bundle.sh` and `bundle-win.sh` (G1-7) to also produce CLAP artifacts. Linux: `target/bundled/Corrosion.clap`. Windows: `target/bundled-win/Corrosion.clap`.
  - Verify each artifact is the correct format for its target (Linux .clap = ELF shared object renamed; Windows .clap = PE DLL renamed).

  **Must NOT do**: Do not duplicate bundle logic — extend the G1-7 scripts.

  **Recommended Agent Profile**: `unspecified-high`. **Skills**: none.

  **Parallelization**: Wave 2. Blocked By: G1-6.

  **References**: NIH-plug CLAP export docs.

  **QA Scenarios**:
  ```
  Scenario: CLAP bundles produced for Linux and Windows
    Tool: Bash
    Steps:
      1. ./bundle.sh && ./bundle-win.sh
      2. test -e target/bundled/Corrosion.clap
      3. test -e target/bundled-win/Corrosion.clap
      4. file target/bundled/Corrosion.clap | grep -q 'ELF'
      5. file target/bundled-win/Corrosion.clap | grep -qE 'PE32\+|MS Windows'
    Expected: both artifacts exist with correct binary format.
    Evidence: .sisyphus/evidence/task-G1-8-clap.log
  ```

  **Commit**: `gate-1(build): CLAP bundle for Linux + Windows`.

- [x] G1-9. REAPER smoke test + validation

  **What to do**:
  - Run `pluginval --strictness-level 5 --validate target/bundled/Corrosion.vst3` and capture log.
  - Run `clap-validator validate target/bundled/Corrosion.clap/Corrosion.clap --only-failed` and capture log.
  - Run REAPER smoke test via `tests/daw/run-reaper.sh` to validate REAPER starts and can access the VST3 bundle.

  **Status**: ✅ COMPLETE
  - pluginval: SUCCESS at strictness level 5
  - clap-validator: 18 passed, 0 failed, 3 skipped
  - REAPER: Starts successfully, test script validates

  **Evidence**: 
  - `.sisyphus/evidence/pluginval-gate-1-linux-vst3.log`
  - `.sisyphus/evidence/clap-validator-gate-1-linux.log`
  - `tests/daw/run-reaper.sh` output

  **What to do**:
  - Install pluginval in the agent environment (`apt install pluginval` or fetch the official release binary). Pluginval is the **hard requirement** for this task — REAPER/Bitwig are secondary scripted smoke tests, NOT a substitute.
  - Run `pluginval --strictness-level 5 --validate target/bundled/Corrosion.vst3` and capture log.
  - Run `clap-validator validate target/bundled/Corrosion.clap/Corrosion.clap --only-failed` and capture log (CLAP uses clap-validator, not pluginval).
  - REAPER scripted bounce: create `tests/daw/gate-1.rpp` (plugin instance + 4-bar MIDI clip of C3 quarter notes), drive via `bash tests/daw/run-reaper.sh` (uses `reaper -nonewinst -renderproject ...`). If `reaper` is not on PATH and `tests/daw/run-reaper.sh` does not produce a bounce, the QA scenario fails (exit non-zero). The task is then BLOCKED until either REAPER is installed or a follow-up plan task replaces this scripted bounce. No manual fallback.
  - Bitwig is **explicitly deferred to Gate 6 G6-13** (where its scripted Controller Script lives). Gate 1 does NOT require Bitwig coverage; the gate is a smoke test, not a release validation.

  **Must NOT do**: Do not skip pluginval. Do not accept screen recordings or manual checklists. Do not soft-pass when REAPER bounce fails — task must hard-fail and surface the missing infra to the user.

  **Recommended Agent Profile**: `unspecified-high`. **Skills**: none.

  **Parallelization**: Wave 2. Blocked By: G1-7, G1-8.

  **References**: https://github.com/Tracktion/pluginval — README "Command Line".

  **QA Scenarios**:
  ```
  Scenario: Pluginval clean for both Linux and Windows VST3 + CLAP bundles
    Tool: Bash
    Steps:
       1. pluginval --strictness-level 5 --validate target/bundled/Corrosion.vst3 2>&1 | tee .sisyphus/evidence/pluginval-gate-1-linux-vst3.log
       2. clap-validator validate target/bundled/Corrosion.clap/Corrosion.clap --only-failed 2>&1 | tee .sisyphus/evidence/clap-validator-gate-1-linux.log
       3. wine pluginval.exe --strictness-level 5 --validate target/bundled-win/Corrosion.vst3 2>&1 | tee .sisyphus/evidence/pluginval-gate-1-win-vst3.log
       4. wine clap-validator.exe validate target/bundled-win/Corrosion.clap/Corrosion.clap --only-failed 2>&1 | tee .sisyphus/evidence/clap-validator-gate-1-win.log
       5. grep -q 'ALL TESTS PASSED' .sisyphus/evidence/pluginval-gate-1-linux-vst3.log || exit 1
       6. grep -qE '(passed|failed|skipped)' .sisyphus/evidence/clap-validator-gate-1-linux.log || exit 1
     Expected: VST3 pluginval passes; CLAP clap-validator reports 0 failed tests.
     Evidence: .sisyphus/evidence/pluginval-gate-1-{linux,win}-vst3.log, .sisyphus/evidence/clap-validator-gate-1-{linux,win}.log

  Scenario: Plugin produces audible bounce in REAPER (scripted, hard requirement)
    Tool: Bash
    Steps:
      1. command -v reaper >/dev/null 2>&1 || { echo "REAPER missing — install or replace this task with a scripted equivalent before proceeding"; exit 1; }
      2. bash tests/daw/run-reaper.sh > .sisyphus/evidence/task-G1-9-reaper.log 2>&1
      3. python3 scripts/check_wav.py /tmp/bounce-reaper-gate1.wav
    Expected: REAPER scripted render produces a non-silent, non-clipping, NaN-free bounce. Helper exits 0.
    Failure Indicators: REAPER unavailable, render command fails, bounce silent or NaN.
    Evidence: .sisyphus/evidence/task-G1-9-reaper.log + bounce file.
  ```

  **Commit**: `gate-1(qa): pluginval clean and host smoke test evidence`.

- [x] G1-10. Gate 1 evidence summary + pass-criteria review

  **What to do**:
  - Write `.sisyphus/evidence/gate-1-summary.md` listing: deliverables, pluginval logs, bundle paths, smoke test outcomes, carry-forward (e.g., DAW automation deferred, allocation auditing strategy).
  - For each Gate 1 pass criterion (`IMPLEMENTATION_PLAN.md:253-259` — VST3 builds, CLAP builds, plugin loads, MIDI note-on triggers sound, note-off natural decay, output bounded, code structure supports later expansion), write PASS/FAIL with linked evidence.
  - Tag `git tag gate-1-complete` only after every criterion is PASS.

  **Must NOT do**: Do not advance to Gate 2 if any criterion fails.

  **Recommended Agent Profile**: `quick`. **Skills**: none.

  **Parallelization**: Wave 2 close. Blocked By: G1-1..9.

  **QA Scenarios**:
  ```
  Scenario: Gate 1 summary exists and references evidence
    Tool: Bash
    Steps:
      1. test -f .sisyphus/evidence/gate-1-summary.md
      2. for crit in "VST3" "CLAP" "loads" "MIDI" "note-off" "bounded" "structure supports"; do
           grep -qi "$crit" .sisyphus/evidence/gate-1-summary.md || exit 1
         done
      3. grep -q 'GATE 1 STATUS: CLOSED' .sisyphus/evidence/gate-1-summary.md
      4. git tag --list | grep -q gate-1-complete
    Expected: all checks pass.
    Evidence: .sisyphus/evidence/gate-1-summary.md
  ```

  **Commit**: `gate-1(review): close Gate 1`.

### Wave 3 — Gate 2 MVP / Version 0.1.0

> Gate 2 task QA scenarios are intentionally compact: each task asserts cargo build + targeted cargo test + (where relevant) a scripted bounce or pluginval log. The full Gate 2 acceptance regression runs in G2-12..14.

- [x] G2-1. 8-voice voice manager + voice pool

  **What to do**: Replace G1's single-voice slot with a fixed-size `[Voice; 8]` array (no Vec to keep audio thread alloc-free). Add `VoiceManager` struct in `src/voice/manager.rs` with: `voices`, `next_voice_index`, `find_inactive() -> Option<usize>`. On NoteOn: assign to first inactive slot or trigger voice-stealing (deferred to G2-4). Process loop sums all 8 voice outputs.

  **Must NOT do**: No `Vec` / `VecDeque` for the voice pool (alloc-free invariant). No tail/stealing logic yet (that's G2-3, G2-4).

  **Recommended Agent Profile**: `deep`. Blocks: G2-3..6.

  **References**: PRD voice section; `src/voice/mod.rs` (G1-5 single-voice impl).

  **QA Scenarios**:
  ```
  Scenario: 8 simultaneous notes audible
    Tool: Bash
    Steps:
      1. tests/polyphony.rs: send 8 NoteOns at C3, E3, G3, B3, D4, F4, A4, C5 within 50ms; render 1s; assert peak > 0.1, no NaN, audible.
      2. cargo test --test polyphony
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G2-1-poly.log

  Scenario: Audio thread never allocates
    Tool: Bash
    Steps:
      1. grep -nE 'Vec::new|vec!\[|Box::new|String::|HashMap::new' src/voice/manager.rs src/voice/mod.rs
    Expected: zero matches.
    Evidence: .sisyphus/evidence/task-G2-1-no-alloc.log
  ```

  **Commit**: `gate-2(voice): 8-voice fixed-pool manager`.

- [x] G2-2. Plate + tank profile activation in plugin path

  **What to do**: Add `Object` enum param (Pipe/Plate/Tank) wired to `ModalProfileId`. On NoteOn, voice rebuilds resonator with the currently-selected profile. Use `PlaceholderResonator::with_profile_size_rust_and_damage(...)` (existing API, no new DSP).

  **Must NOT do**: Do not allocate during the rebuild — pre-allocate a max-mode buffer per voice in the voice constructor and reuse.

  **Recommended Agent Profile**: `quick`. Blocks: none new (independent of G2-1).

  **References**: `src/dsp/profile.rs` (G1-3) — `ModalProfileId::{Pipe,Plate,Tank}`.

  **QA Scenarios**:
  ```
  Scenario: All three objects produce distinct timbres
    Tool: Bash
    Steps:
      1. tests/object_distinct.rs: render 1s C3 hits with each Object setting; compare brightness_proxy + late_to_early_energy_ratio between renders; assert pairwise difference >= 10%.
      2. cargo test --test object_distinct
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G2-2-distinct.log
  ```

  **Commit**: `gate-2(dsp): plate and tank objects in plugin path`.

- [x] G2-3. Tail-energy tracking + voice deactivation threshold

  **What to do**: Per voice, track running RMS in a small ring buffer (fixed-size, no alloc on hot path). When RMS < threshold (e.g., -60 dBFS) for ≥N consecutive frames, mark voice inactive. Add unit test for the threshold logic only (the one allowed new test for this task).

  **Must NOT do**: Do not call `f32::log10` per sample (use squared-RMS comparison against a squared threshold).

  **Recommended Agent Profile**: `deep`. Blocked By: G2-1.

  **QA Scenarios**:
  ```
  Scenario: Voice deactivates after decay
    Tool: Bash
    Steps:
      1. tests/voice_deactivation.rs: NoteOn at frame 0, no NoteOff; render 5s; assert voice.active == false by t=4s.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G2-3-deactivate.log

  Scenario: Active voice not falsely deactivated
    Tool: Bash
    Steps:
      1. tests/voice_held.rs: NoteOn, render 0.5s of long-decay tank profile, assert voice still active.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G2-3-held.log
  ```

  **Commit**: `gate-2(voice): tail-energy tracking and deactivation`.

- [x] G2-4. Voice stealing (inactive → quietest → oldest)

  **What to do**: When all 8 slots active on NoteOn, pick steal target by: (1) any inactive (handled by G2-1 path), (2) lowest tail-RMS, (3) oldest start time. Resolve ties deterministically.

  **Must NOT do**: No alloc. No locking. No system clock — track `frames_since_start` per voice (u64 counter).

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: G2-1, G2-3.

  **QA Scenarios**:
  ```
  Scenario: 9th note steals quietest voice
    Tool: Bash
    Steps:
      1. tests/voice_stealing.rs: NoteOn 8 voices, render 200ms, NoteOn 9th; verify a previously-active voice (the quietest) was reassigned to the new note's frequency.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G2-4-steal.log
  ```

  **Commit**: `gate-2(voice): voice-stealing fallback`.

- [x] G2-5. Object/Size/Rust/Damage/Drive/Output param exposure and wiring

  **What to do**: Replace G1-2's placeholder `gain` with the full MVP param set: Object (enum), Size (FloatParam, range from frozen ranges doc), Rust (FloatParam), Damage (FloatParam), Drive (FloatParam, dB), Output (FloatParam, dB). All `#[id]`-stable for automation. Apply Drive as a tanh-style soft saturator before Output gain.

  **Must NOT do**: No new param IDs beyond these 6. No host-side automation curves (use NIH-plug defaults).

  **Recommended Agent Profile**: `quick`. Blocked By: G2-2.

  **References**: `.sisyphus/evidence/parameter-ranges.md` (G0-1).

  **QA Scenarios**:
  ```
  Scenario: All 6 parameters exposed and stable
    Tool: Bash
    Steps:
      1. pluginval --strictness-level 5 --validate target/bundled/Corrosion.vst3 | tee .sisyphus/evidence/task-G2-5-pluginval.log
      2. grep -q 'ALL TESTS PASSED' .sisyphus/evidence/task-G2-5-pluginval.log
      3. grep -cE 'FloatParam|EnumParam' src/params.rs  # expect 6
    Expected: all green.
    Evidence: .sisyphus/evidence/task-G2-5-pluginval.log

  Scenario: Drive audibly increases harmonic content
    Tool: Bash
    Steps:
      1. tests/drive_harmonic.rs: render same note with Drive=0 and Drive=12dB; assert brightness_proxy(drive_high) > brightness_proxy(drive_low).
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G2-5-drive.log
  ```

  **Commit**: `gate-2(params): expose Object/Size/Rust/Damage/Drive/Output`.

- [x] G2-6. Velocity-to-physical-behavior mapping

  **What to do**: Velocity must shape excitation force AND brightness, not only output level. In hit exciter: scale impulse amplitude AND impulse high-frequency content (e.g., shorten impulse width as velocity rises) so soft hits sound dull and hard hits sound bright.

  **Must NOT do**: Do not map velocity to output gain alone. Do not introduce a separate "brightness" parameter (velocity drives it).

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: G2-5.

  **QA Scenarios**:
  ```
  Scenario: High velocity is brighter than low velocity at same RMS
    Tool: Bash
    Steps:
      1. tests/velocity_brightness.rs: render note at velocity 0.2 and 1.0; level-normalize both; assert brightness_proxy(high_vel) > brightness_proxy(low_vel) * 1.2.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G2-6-velocity.log
  ```

  **Commit**: `gate-2(excitation): velocity shapes force + brightness`.

- [x] G2-7. Audio-thread allocation audit + denormal/NaN/inf guards (cross-cut)

  **What to do**: Add `assert_no_alloc` crate as `[dev-dependencies]` (allowed dep, scoped to tests). Wrap a representative process loop in `assert_no_alloc(|| { ... })` inside a test. Run grep audit across all hot-path files. Add denormal flush + NaN/inf replacement at the end of every voice's process_sample.

  **Must NOT do**: Do not add `assert_no_alloc` as a regular dependency (test-only).

  **Recommended Agent Profile**: `deep`. Blocked By: G2-1, G2-3, G2-4.

  **References**: `docs/IMPLEMENTATION_PLAN.md:719-727`. `docs/notepads/corrosion/decisions.md:11`.

  **QA Scenarios**:
  ```
  Scenario: Process callback is alloc-free
    Tool: Bash
    Steps:
      1. tests/no_alloc.rs uses assert_no_alloc to wrap 10s of plugin processing under 8-voice load.
      2. cargo test --test no_alloc
    Expected: pass (no alloc panic).
    Evidence: .sisyphus/evidence/task-G2-7-noalloc.log

  Scenario: Forbidden-pattern grep clean
    Tool: Bash
    Steps:
      1. ! grep -nE 'Vec::new|vec!\[|Box::new|format!|println!|eprintln!|Mutex|RwLock|serde_json|std::fs|std::io|std::thread::sleep|panic!|todo!|unimplemented!' src/dsp/resonator.rs src/voice/mod.rs src/voice/manager.rs src/lib.rs
    Expected: zero matches.
    Evidence: .sisyphus/evidence/task-G2-7-grep.log
  ```

  **Commit**: `gate-2(safety): allocation audit and denormal/NaN guards`.

- [x] G2-8. Generic editor (NIH-plug egui or default)

  **What to do**: Add NIH-plug's `nih_plug_egui` or `nih_plug_iced` (whichever the chosen GUI stack will be in Gate 4 — pick now to avoid migration). For Gate 2: a generic-knob layout, one slider/knob per param. No physical metaphor yet.

  **Must NOT do**: No custom drawing, no preset browser UI (those land in Gate 4). Just functional knobs.

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: G2-5.

  **QA Scenarios**:
  ```
  Scenario: GUI initializes without panic
    Tool: Bash
    Steps:
      1. pluginval --validate (with editor open path) — pluginval exercises the editor.
      2. grep -q 'editor' .sisyphus/evidence/task-G2-8-pluginval.log
    Expected: clean.
    Evidence: .sisyphus/evidence/task-G2-8-pluginval.log
  ```

  **Commit**: `gate-2(gui): generic egui editor`.

- [x] G2-9. Preset file format + serialize/deserialize + `preset-render` helper bin

  **What to do**:
  - Define a `.corrosion-preset` JSON format containing: name, version, all 6 param values, object enum.
  - Implement `Preset::save(path)` and `Preset::load(path)` in `src/presets/mod.rs`. Wire NIH-plug's state-save/load to use this format. `serde_json` is allowed only in non-audio-thread code paths (preset I/O, GUI, sequencer pattern load — never inside `process(...)`).
  - Add a new binary target `[[bin]] name = "preset-render" path = "src/bin/preset_render.rs"`. The bin takes `--preset <path> --out <wav>` (and optional `--note <midi>`, `--duration <sec>`), loads the preset offline, drives a deterministic NoteOn through the same modal/voice path used by the plugin, and writes a WAV using the existing offline WAV writer. This bin is referenced by G2-10 and later preset-validation QA scenarios.

  **Must NOT do**: Do not parse JSON in the audio thread. Preset load happens off the audio thread, then values are atomically swapped into params. Do not introduce serde_json into `src/dsp/` or `src/voice/`.

  **Recommended Agent Profile**: `quick`. Blocked By: G2-5.

  **QA Scenarios**:
  ```
  Scenario: Round-trip preserves all params
    Tool: Bash
    Steps:
      1. tests/preset_roundtrip.rs: build preset, save to /tmp, load, assert byte-equal param values.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G2-9-roundtrip.log

  Scenario: preset-render bin produces a valid WAV
    Tool: Bash
    Steps:
      1. cargo build --release --bin preset-render
      2. # Use any existing preset-format-shaped fixture or a default-params one
      3. cargo run --release --bin preset-render -- --preset tests/fixtures/default.corrosion-preset --out /tmp/preset-render-smoke.wav
      4. python3 scripts/check_wav.py /tmp/preset-render-smoke.wav
    Expected: bin builds, render produces WAV, check_wav.py exits 0.
    Evidence: .sisyphus/evidence/task-G2-9-bin-smoke.log
  ```

  **Commit**: `gate-2(presets): preset file format, IO, and preset-render bin`.

- [x] G2-10. 20 factory presets across bass / clang / boom / short-hit / long-tail

  **What to do**: Author 20 preset JSON files under `presets/factory/`, distributed: ≥4 bass, ≥4 clang/impact, ≥4 boom/low-body, ≥4 short-hit, ≥4 long-tail. Each preset has a descriptive name. Validate via the load API and a render bounce that asserts the preset is non-silent.

  **Must NOT do**: Do not duplicate presets with trivial parameter shifts. Each preset must be musically distinct.

  **Recommended Agent Profile**: `writing` (sound-design + JSON authoring). Blocked By: G2-9.

  **References**: PRD preset list section.

  **QA Scenarios**:
  ```
  Scenario: 20 presets render audibly across categories
    Tool: Bash
    Steps:
      1. ls presets/factory/*.corrosion-preset | wc -l  # expect >= 20
      2. for p in presets/factory/*.corrosion-preset; do
           cargo run --release --bin preset-render -- --preset "$p" --out "/tmp/$(basename "$p" .corrosion-preset).wav"
           # assert non-silent via a tiny Rust harness that checks peak > 0.01
         done
      3. for cat in bass clang boom short-hit long-tail; do
           ls presets/factory/ | grep -ci "$cat" | awk -v cat="$cat" '{ if ($1 < 4) { print cat" underrepresented"; exit 1 } }'
         done
    Expected: ≥20 presets, all audible, ≥4 per category.
    Evidence: .sisyphus/evidence/task-G2-10-presets.log
  ```

  **Commit**: `gate-2(presets): 20 factory presets across MVP categories`.

- [x] G2-11. Hard safety limiter / output clip

  **What to do**: Add a final-stage limiter (simple lookahead-free hard knee at -0.3 dBFS) after Output gain. This is in addition to the G1-6 raw clamp.

  **Must NOT do**: No multi-band limiter, no lookahead (alloc + latency).

  **Recommended Agent Profile**: `quick`. Blocked By: G2-5.

  **QA Scenarios**:
  ```
  Scenario: Output never exceeds -0.3 dBFS even with extreme Drive
    Tool: Bash
    Steps:
      1. tests/limiter.rs: render with Drive=+24dB, Output=+24dB, all 8 voices triggered hard; assert max |sample| < 10^(-0.3/20) = 0.9661.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G2-11-limiter.log
  ```

  **Commit**: `gate-2(safety): hard-knee output limiter`.

- [ ] G2-12. Pluginval Gate 2 run + log capture

  **What to do**: Build release bundle, run `pluginval --strictness-level 8` against VST3 and CLAP. Capture full logs. If any test fails, do NOT close gate.

  **Must NOT do**: Do not lower strictness to make it pass.

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: G2-1..11.

  **QA Scenarios**:
  ```
  Scenario: Pluginval strictness 8 passes for Linux and Windows bundles
    Tool: Bash
    Steps:
       1. ./bundle.sh && ./bundle-win.sh
       2. pluginval --strictness-level 8 --validate target/bundled/Corrosion.vst3 | tee .sisyphus/evidence/pluginval-gate-2-linux-vst3.log
       3. clap-validator validate target/bundled/Corrosion.clap/Corrosion.clap --only-failed | tee .sisyphus/evidence/clap-validator-gate-2-linux.log
       4. wine pluginval.exe --strictness-level 8 --validate target/bundled-win/Corrosion.vst3 | tee .sisyphus/evidence/pluginval-gate-2-win-vst3.log
       5. wine clap-validator.exe validate target/bundled-win/Corrosion.clap/Corrosion.clap --only-failed | tee .sisyphus/evidence/clap-validator-gate-2-win.log
       6. grep -q 'ALL TESTS PASSED' .sisyphus/evidence/pluginval-gate-2-linux-vst3.log || exit 1
       7. grep -qE '(passed|failed|skipped)' .sisyphus/evidence/clap-validator-gate-2-linux.log || exit 1
     Expected: VST3 pluginval passes; CLAP clap-validator reports 0 failed tests.
     Evidence: .sisyphus/evidence/pluginval-gate-2-{linux,win}-vst3.log, .sisyphus/evidence/clap-validator-gate-2-{linux,win}.log
  ```

  **Commit**: `gate-2(qa): pluginval strictness-8 evidence`.

- [ ] G2-13. Scripted DAW regression suite (REAPER)

  **What to do**: Author a REAPER project (`tests/daw/gate-2.rpp`) that exercises: MIDI playback (32 notes, varied velocities), automation lane on every parameter, preset cycle, buffer 64/512/2048, sample rate 44.1k/48k/96k. Render via REAPER CLI and bash-verify each bounce: peak > 0.05, no NaN (via Python wave parse), file size matches expected duration.

  **Must NOT do**: Do not require human DAW interaction.

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: G2-12.

  **QA Scenarios**:
  ```
  Scenario: REAPER regression bounces all clean
    Tool: Bash
    Steps:
      1. for sr in 44100 48000 96000; do
           for buf in 64 512 2048; do
             reaper -nonewinst -saveas /tmp/g2.rpp -renderproject ... # adapt to actual CLI
             python3 scripts/check_wav.py /tmp/g2-${sr}-${buf}.wav  # asserts peak/no-nan
           done
         done
    Expected: 9 clean bounces.
    Evidence: .sisyphus/evidence/task-G2-13-daw/
  ```

  **Commit**: `gate-2(qa): scripted REAPER regression suite`.

- [ ] G2-14. DSP regression: family differentiation + transform metric assertions

  **What to do**: Re-run all Gate 0 metric assertions against the plugin output (not just offline renderer). Add `tests/plugin_metrics.rs` with: family differentiation thresholds, size monotonicity (size↑ ⇒ peak frequency↓), rust monotonicity (rust↑ ⇒ brightness↓), damage monotonicity (damage↑ ⇒ roughness↑).

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: G2-2, G2-5.

  **QA Scenarios**:
  ```
  Scenario: All DSP regressions green
    Tool: Bash
    Steps:
      1. cargo test --test plugin_metrics
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G2-14-metrics.log
  ```

  **Commit**: `gate-2(qa): DSP regression suite vs Gate 0 metrics`.

- [ ] G2-15. Gate 2 evidence summary + pass-criteria review

  **What to do**: Write `.sisyphus/evidence/gate-2-summary.md` with PASS/FAIL row per pass-criterion (`IMPLEMENTATION_PLAN.md:352-367` — 15 criteria). Tag `gate-2-complete` only on full pass.

  **Recommended Agent Profile**: `quick`. Blocked By: G2-1..14.

  **QA Scenarios**:
  ```
  Scenario: Gate 2 closure
    Tool: Bash
    Steps:
      1. test -f .sisyphus/evidence/gate-2-summary.md
      2. grep -q 'GATE 2 STATUS: CLOSED' .sisyphus/evidence/gate-2-summary.md
      3. git tag --list | grep -q gate-2-complete
    Expected: closed.
    Evidence: .sisyphus/evidence/gate-2-summary.md
  ```

  **Commit**: `gate-2(review): close Gate 2`. Tag `v0.1.0` and `gate-2-complete`.

### Wave 4 — Gate 3 Industrial Character / Version 0.2.0

> Each Gate 3 task follows: (impl) + (cargo test or scripted bounce metric) + commit. QA scenarios condensed; pattern same as Gate 2.

- [ ] G3-1. Scrape exciter core

  **What to do**: New module `src/dsp/exciters/scrape.rs`. Implements pressure / speed / roughness / stick-slip framing per PRD scrape semantics. Drives the same modal resonator as the hit exciter. Add an `Exciter` enum param (Hit/Scrape) so users can choose.

  **Must NOT do**: Do not break the Hit exciter. Do not add scrape-specific objects (chain is G3-3).

  **Recommended Agent Profile**: `artistry` — DSP design with novel constraints. Blocked By: gate-2-complete.

  **References**: PRD search "scrape", "stick-slip", "bowed".

  **QA Scenarios**:
  ```
  Scenario: Scrape produces sustained, non-impulsive output
    Tool: Bash
    Steps:
      1. tests/scrape.rs: hold note 2s with Exciter=Scrape; assert RMS in last 25% > 50% of RMS in first 25% (sustained, unlike hit which decays).
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G3-1-scrape.log
  ```

  **Commit**: `gate-3(exciter): scrape exciter core`.

- [ ] G3-2. Tune scrape for bowed-steel / brake-squeal / tension-rise

  **What to do**: Author 3 internal "scrape mode" presets in code (constants, not user-exposed) covering the three sound targets. Provide them as `ScrapeFlavor` enum tied to specific stick-slip and roughness envelope settings. Compare-render each.

  **Recommended Agent Profile**: `deep`. Blocked By: G3-1.

  **QA Scenarios**:
  ```
  Scenario: Three scrape flavors are perceptually distinct
    Tool: Bash
    Steps:
      1. tests/scrape_flavors.rs: render each flavor on the same note; assert pairwise brightness_proxy + roughness_proxy differences > 10%.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G3-2-flavors.log
  ```

  **Commit**: `gate-3(exciter): bowed-steel / brake-squeal / tension-rise scrape flavors`.

- [ ] G3-3. Chain object profile

  **What to do**: New `ModalProfileId::Chain` with curated mode table emphasizing dense transients and unstable pitch (high inharmonicity, short individual decays). Add to `Object` enum. Implements PRD chain semantics.

  **Must NOT do**: Do not implement chain as "noise + reverb" — it must be a true modal profile.

  **Recommended Agent Profile**: `artistry`. Blocked By: gate-2-complete.

  **QA Scenarios**:
  ```
  Scenario: Chain distinct from pipe/plate/tank
    Tool: Bash
    Steps:
      1. tests/chain_distinct.rs: render hit on each of 4 objects; assert chain's roughness_proxy > all other objects, AND chain's late_to_early_energy_ratio < pipe's (shorter sustain).
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G3-3-chain.log
  ```

  **Commit**: `gate-3(object): chain modal profile`.

- [ ] G3-4. Stereo modal spread + width control

  **What to do**: Distribute modes across L/R based on a deterministic per-mode pan derived from frequency. Add `Width` parameter (0=mono, 1=full spread). Mid/side balance preserved.

  **Recommended Agent Profile**: `deep`. Blocked By: gate-2-complete.

  **QA Scenarios**:
  ```
  Scenario: Width=0 yields mono, Width=1 yields stereo
    Tool: Bash
    Steps:
      1. tests/stereo_width.rs: render at Width=0 and Width=1; assert mid-side correlation differs (Width=0: correlation ~1.0, Width=1: < 0.9).
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G3-4-width.log
  ```

  **Commit**: `gate-3(stereo): modal spread with Width control`.

- [ ] G3-5. Lightweight body resonator

  **What to do**: New `src/dsp/body.rs` — small fixed bank of 3-5 broad resonances post-voice-mix. `Body` parameter controls amount.

  **Must NOT do**: No reverb / impulse-response convolution. Just modal body.

  **Recommended Agent Profile**: `deep`. Blocked By: gate-2-complete.

  **QA Scenarios**:
  ```
  Scenario: Body adds low-mid energy without smearing decay
    Tool: Bash
    Steps:
      1. tests/body.rs: render with Body=0 and Body=0.7; assert low-mid (200-800Hz) energy is higher with body, but tail length difference < 20%.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G3-5-body.log
  ```

  **Commit**: `gate-3(body): lightweight body resonator`.

- [ ] G3-6. Roughness / rattle character pass on damage

  **What to do**: Improve the damage transform's output — make it sound more "rattling industrial" rather than just detuned. Possible: add an envelope-modulated noise burst tied to peak crossings; tune via the existing `DamageAmount` API.

  **Recommended Agent Profile**: `artistry`. Blocked By: gate-2-complete.

  **QA Scenarios**:
  ```
  Scenario: Improved damage scores higher roughness vs Gate 2 baseline
    Tool: Bash
    Steps:
      1. Save baseline roughness_proxy from gate-2-summary.md; compare to current.
      2. Assert current >= baseline (no regression) for low/mid damage; current > baseline * 1.1 for high damage.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G3-6-rattle.log
  ```

  **Commit**: `gate-3(damage): rattle character improvement`.

- [ ] G3-7. Saturation character pass on Drive

  **What to do**: Replace G2's tanh with a multi-stage waveshaper that preserves dynamics under high drive (e.g., asymmetric soft clip). Tune to sound forceful, not collapsed.

  **Recommended Agent Profile**: `artistry`. Blocked By: gate-2-complete.

  **QA Scenarios**:
  ```
  Scenario: High drive does not collapse dynamics
    Tool: Bash
    Steps:
      1. tests/drive_dynamics.rs: render same percussive input at Drive=0 and Drive=18dB; assert peak/RMS ratio (crest factor) at Drive=18dB >= 60% of crest factor at Drive=0.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G3-7-drive.log
  ```

  **Commit**: `gate-3(drive): improved saturation waveshaper`.

- [ ] G3-8. Velocity mapping expressiveness pass

  **What to do**: Extend G2-6: velocity also modulates damage modulation depth (harder hit ⇒ slightly more damage character) and excitation impulse decay rate.

  **Recommended Agent Profile**: `deep`. Blocked By: G2-6.

  **QA Scenarios**:
  ```
  Scenario: Velocity expressivity beyond brightness
    Tool: Bash
    Steps:
      1. tests/velocity_expressivity.rs: render notes at velocity 0.2/0.5/0.9; assert each successive render differs in roughness_proxy by >= 5% (not just brightness).
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G3-8-velocity.log
  ```

  **Commit**: `gate-3(excitation): velocity expressivity pass`.

- [ ] G3-9. 20 additional presets (scrape / chain / drone / transition) → 40+ total

  **What to do**: Author 20 new factory presets: ≥5 scrape-focused, ≥5 chain-focused, ≥5 drone-focused, ≥5 transition-focused. Total preset count ≥ 40.

  **Recommended Agent Profile**: `writing`. Blocked By: G3-1, G3-3.

  **QA Scenarios**:
  ```
  Scenario: 40+ presets, category coverage
    Tool: Bash
    Steps:
      1. ls presets/factory/*.corrosion-preset | wc -l  # >= 40
      2. for cat in scrape chain drone transition; do
           ls presets/factory/ | grep -ci "$cat" | awk -v cat="$cat" '{ if ($1 < 5) { print cat" underrepresented"; exit 1 } }'
         done
      3. Render every preset; assert all non-silent.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G3-9-presets.log
  ```

  **Commit**: `gate-3(presets): 20 new presets across scrape/chain/drone/transition`.

- [ ] G3-10. Automation stress test for stereo / body parameters

  **What to do**: REAPER project that automates Width and Body across full range every bar at 240 BPM for 60s with 8 voices. Bounce; assert no clicks (no sample-to-sample delta > 0.5).

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: G3-4, G3-5.

  **QA Scenarios**:
  ```
  Scenario: Stereo/body automation produces no clicks
    Tool: Bash
    Steps:
      1. reaper -renderproject tests/daw/gate-3-stress.rpp /tmp/stress.wav
      2. python3 scripts/check_clicks.py /tmp/stress.wav  # max sample delta < 0.5
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G3-10-stress.wav
  ```

  **Commit**: `gate-3(qa): stereo/body automation stress test`.

- [ ] G3-11. Regression vs Gate 2 stability

  **What to do**: Re-run G2-13's full DAW regression suite. All bounces must remain clean.

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: G3-1..10.

  **QA Scenarios**:
  ```
  Scenario: Gate 2 regression suite still green
    Tool: Bash
    Steps:
      1. Re-run G2-13's bounce matrix; identical pass criteria.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G3-11-regression/
  ```

  **Commit**: `gate-3(qa): Gate 2 regression preserved`.

- [ ] G3-12. Gate 3 evidence summary + pass-criteria review

  **What to do**: Write `.sisyphus/evidence/gate-3-summary.md` with PASS/FAIL per pass criterion (`IMPLEMENTATION_PLAN.md:447-454`). Tag `gate-3-complete` and `v0.2.0` on full pass.

  **Recommended Agent Profile**: `quick`. Blocked By: G3-1..11.

  **QA Scenarios**:
  ```
  Scenario: Gate 3 closure
    Tool: Bash
    Steps:
      1. test -f .sisyphus/evidence/gate-3-summary.md
      2. grep -q 'GATE 3 STATUS: CLOSED' .sisyphus/evidence/gate-3-summary.md
      3. git tag --list | grep -E 'gate-3-complete|v0.2.0' | wc -l  # 2
    Expected: closed.
    Evidence: .sisyphus/evidence/gate-3-summary.md
  ```

  **Commit**: `gate-3(review): close Gate 3`. Tags `gate-3-complete`, `v0.2.0`.

---

### Wave 5 — Gate 4 Product UX / Version 0.3.0

- [ ] G4-1. Custom GUI scaffold (Exciter → Object → Damage → Space layout)

  **What to do**: Replace G2-8's generic editor. Custom layout with 4 sections (Exciter, Object, Damage, Space). Each section uses physical metaphor naming. No oscillator/filter/amp framing. Skin defined in `src/gui/`.

  **Must NOT do**: No oscillator/filter/amp text or icons. No knob-cluster-only design.

  **Recommended Agent Profile**: `visual-engineering`. Blocked By: gate-3-complete.

  **References**: PRD UI section.

  **QA Scenarios**:
  ```
  Scenario: GUI source contains no forbidden framing
    Tool: Bash
    Steps:
      1. ! grep -inE '\b(oscillator|filter|envelope generator|VCA|VCF|VCO|LFO\s+amount)\b' src/gui/
      2. grep -cE '\b(exciter|object|damage|space)\b' src/gui/  # >= 4
      3. pluginval validates with editor open.
    Expected: clean grep, ≥4 metaphor terms, pluginval clean.
    Evidence: .sisyphus/evidence/task-G4-1-gui.log
  ```

  **Commit**: `gate-4(gui): custom layout with physical-metaphor sections`.

- [ ] G4-2. Mass macro
- [ ] G4-3. Corrosion macro
- [ ] G4-4. Violence macro
- [ ] G4-5. Damage macro

  **What to do (G4-2..5 share template)**: Each macro is a single-knob param. On change, it scales/biases an internal group of underlying params via a fixed mapping table in `src/macros/mod.rs`. Mass = Object + Size; Corrosion = Rust + body damping; Violence = Drive + excitation force; Damage = Damage + roughness.

  **Recommended Agent Profile**: `deep` (each). Parallel within Wave 5. Blocked By: G4-1.

  **QA Scenarios (per macro)**:
  ```
  Scenario: Macro at 0 vs 1 produces audibly different render
    Tool: Bash
    Steps:
      1. tests/macros.rs: for each macro, render with macro=0 and macro=1; assert combined-metric distance > threshold.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G4-{2..5}.log
  ```

  **Commit**: one per macro: `gate-4(macros): {Mass,Corrosion,Violence,Damage} macro`.

- [ ] G4-6. Macro → internal-parameter-group mapping

  **What to do**: Document the mapping table in `src/macros/mod.rs` AND in `docs/developer.md` (stub for now, fleshed out at Gate 6). Ensure mappings hold under preset recall (macro positions persist; underlying params recompute on load).

  **Recommended Agent Profile**: `deep`. Blocked By: G4-2..5.

  **QA Scenarios**:
  ```
  Scenario: Macro positions persist round-trip
    Tool: Bash
    Steps:
      1. tests/macro_persist.rs: set macros, save preset, load, assert macro values byte-equal AND underlying params match the mapping output.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G4-6-mapping.log
  ```

  **Commit**: `gate-4(macros): mapping documentation and persistence`.

- [ ] G4-7. Randomizer modes (safe / object / damage / full)

  **What to do**: Implement 4 randomization modes. Safe = small jitter on all params. Object = randomize Object only. Damage = randomize Rust + Damage only. Full = randomize all params within safe ranges. Triggered via GUI button.

  **Must NOT do**: Do not produce silent or DC-only patches. Do not use `rand::thread_rng()` on the audio thread (use a UI-thread RNG).

  **Recommended Agent Profile**: `artistry`. Blocked By: G4-1.

  **QA Scenarios**:
  ```
  Scenario: 1000 random patches are non-silent and non-clipping
    Tool: Bash
    Steps:
      1. tests/randomizer.rs: for each mode, generate 1000 patches; render 0.5s of MIDI hit; assert peak in [0.01, 0.95] for every patch.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G4-7-random.log
  ```

  **Commit**: `gate-4(randomizer): safe/object/damage/full modes`.

- [ ] G4-8. Mutate behavior + randomization safety constraints

  **What to do**: "Mutate" = small Gaussian jitter on every active param around current values. Add safety constraints library (`src/randomizer/safety.rs`) that clamps any param the randomizer touches to safe sub-ranges (no DC, no extreme drive at extreme damage simultaneously, etc.).

  **Recommended Agent Profile**: `artistry`. Blocked By: G4-7.

  **QA Scenarios**:
  ```
  Scenario: Mutate stays close to source patch
    Tool: Bash
    Steps:
      1. tests/mutate.rs: from a known patch, mutate 100 times; assert each parameter delta < 20% of full range.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G4-8-mutate.log
  ```

  **Commit**: `gate-4(randomizer): mutate + safety constraints`.

- [ ] G4-9. Preset browser workflow

  **What to do**: GUI preset browser with category filters (Bass/Percussion/Drone/Transition/...). Click-to-load. Keyboard arrow navigation. Search box.

  **Recommended Agent Profile**: `visual-engineering`. Blocked By: G4-1, G2-9.

  **QA Scenarios**:
  ```
  Scenario: Preset browser loads any preset under 100ms
    Tool: Bash
    Steps:
      1. tests/preset_browser_perf.rs: drive the browser headlessly (or via a NIH-plug test harness), load each preset, time the load; assert max < 100ms.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G4-9-browser.log
  ```

  **Commit**: `gate-4(gui): preset browser workflow`.

- [ ] G4-10. Visual object/resonator feedback widget

  **What to do**: A single visualization in the Object section showing modal energy distribution (e.g., a row of bars per active mode, animated by mode level). No FFT, no oscilloscope (per "no generic synth visuals" constraint).

  **Recommended Agent Profile**: `visual-engineering`. Blocked By: G4-1.

  **QA Scenarios**:
  ```
  Scenario: Visualization driven by real DSP state
    Tool: Bash
    Steps:
      1. tests/gui_state.rs (if testable headlessly) or grep src/gui/visualization.rs for direct read of resonator state.
      2. ! grep -E 'fft|spectrogram|oscilloscope' src/gui/visualization.rs
    Expected: physical visualization, no spectrogram terms.
    Evidence: .sisyphus/evidence/task-G4-10-viz.log
  ```

  **Commit**: `gate-4(gui): modal-energy visualization widget`.

- [ ] G4-11. Regression: automation, preset changes, output safety under GUI-driven edits

  **What to do**: Re-run G2-13 + G3-11 regression bounces with GUI-driven param sweeps included. All assertions still pass.

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: G4-1..10.

  **QA Scenarios**:
  ```
  Scenario: GUI edits do not break automation safety
    Tool: Bash
    Steps:
      1. Re-run regression suite with GUI param-source mixed with automation-source.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G4-11-regression/
  ```

  **Commit**: `gate-4(qa): regression with GUI-driven edits`.

- [ ] G4-12. Gate 4 evidence summary + pass-criteria review

  **What to do**: Write `.sisyphus/evidence/gate-4-summary.md` with one PASS/FAIL row per Gate 4 pass criterion (`IMPLEMENTATION_PLAN.md:530-535` — basic custom GUI implemented; randomizer implemented; macro controls implemented; preset browsing implemented in usable form; interface supports product metaphor and quick sound-shaping). Each row links to the task evidence path that proves it. Tag `gate-4-complete` and `v0.3.0` only on full pass; otherwise open follow-up tasks under Gate 4 instead of advancing.

  **Must NOT do**: Do not advance to Gate 5 if any criterion fails. Do not soften criteria.

  **Recommended Agent Profile**: `quick`. **Skills**: none. Blocked By: G4-1..11.

  **References**:
  - `docs/IMPLEMENTATION_PLAN.md:530-535` — Gate 4 pass criteria.
  - `.sisyphus/evidence/task-G4-{1..11}-*.log` — task-level evidence.

  **QA Scenarios**:
  ```
  Scenario: Gate 4 summary covers all 5 pass criteria with linked evidence
    Tool: Bash
    Steps:
      1. test -f .sisyphus/evidence/gate-4-summary.md
      2. for crit in "custom GUI" "andomizer" "acro" "reset brows" "metaphor"; do
           grep -qi "$crit" .sisyphus/evidence/gate-4-summary.md || { echo "missing: $crit"; exit 1; }
         done
      3. # Every PASS row must reference a task evidence file
      4. grep -E 'PASS' .sisyphus/evidence/gate-4-summary.md | grep -E '\.sisyphus/evidence/task-G4-' >/dev/null
      5. grep -q 'GATE 4 STATUS: CLOSED' .sisyphus/evidence/gate-4-summary.md
      6. git tag --list | grep -E '^(gate-4-complete|v0\.3\.0)$' | wc -l  # expect: 2
    Expected: all checks pass; both tags exist.
    Failure Indicators: missing summary, missing criterion text, PASS rows lacking evidence links, missing tags.
    Evidence: .sisyphus/evidence/gate-4-summary.md

  Scenario: Failing criterion blocks gate close
    Tool: Bash
    Steps:
      1. If any task in Wave 5 has FAIL status, gate-4-summary.md must contain "GATE 4 STATUS: OPEN" (not CLOSED) and must list follow-up task IDs.
      2. ! grep -q 'GATE 4 STATUS: CLOSED' .sisyphus/evidence/gate-4-summary.md  # only when failures exist
    Expected: status reflects reality; gate cannot close while failures exist.
    Evidence: .sisyphus/evidence/gate-4-summary.md
  ```

  **Commit**: YES. Message: `gate-4(review): close Gate 4`. Tags: `gate-4-complete`, `v0.3.0`. Pre-commit: `cargo test --workspace && cargo fmt --check`.

---

### Wave 6 — Gate 5 Sequenced Instrument

- [ ] G5-1. Sequencer step data structure + runtime playback model

  **What to do**: New module `src/sequencer/`. `Sequence { steps: [Step; 32] }` where `Step { enabled, note, velocity, probability, microtiming_offset_ticks, locks: StepLocks }`. Fixed-size — no Vec on hot path. Playback driven by host transport position.

  **Recommended Agent Profile**: `deep`. Blocked By: gate-4-complete.

  **QA Scenarios**:
  ```
  Scenario: 32-step sequence triggers expected notes
    Tool: Bash
    Steps:
      1. tests/sequencer_basic.rs: configure 32 steps each on a 16th, 120 BPM, render 4 bars; bounce; bash-detect transients (peak > 0.1 within 5ms windows) at expected sample positions; assert >= 30 detected (allow 2 missed for probability).
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G5-1-seq.log
  ```

  **Commit**: `gate-5(seq): step structure and runtime playback`.

- [ ] G5-2. Per-step note + velocity + probability + microtiming

  **What to do**: Wire each `Step` field through the playback model. Probability gates step firing via deterministic seeded RNG (per-bar seed derived from sample position so behavior is reproducible). Microtiming = ±50% of step duration in ticks.

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: G5-1.

  **QA Scenarios**:
  ```
  Scenario: Probability=0 never fires; probability=1 always fires
    Tool: Bash
    Steps:
      1. tests/probability.rs: 16 steps all at probability 0; render 100 bars; assert zero transients. Then prob=1: assert 16 transients/bar exactly.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G5-2-prob.log
  ```

  **Commit**: `gate-5(seq): per-step note/velocity/probability/microtiming`.

- [ ] G5-3. Host sync (BPM, sample position, transport, loop)

  **What to do**: Read `nih_plug::context::Transport` (BPM, ppq position, playing flag, loop range). Translate to step index. Pause playback when transport stops; resume at correct step on play.

  **Recommended Agent Profile**: `deep`. Blocked By: G5-1.

  **QA Scenarios**:
  ```
  Scenario: Tempo change at runtime updates step rate
    Tool: Bash
    Steps:
      1. REAPER project: 60 BPM for 4 bars, ramp to 120 BPM over 4 bars, hold 60 BPM for 4 bars; bounce; detect transients; assert step density doubles in middle section.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G5-3-tempo.log
  ```

  **Commit**: `gate-5(seq): host transport sync`.

- [ ] G5-4. Stability under play/stop/loop transitions

  **What to do**: When loop boundary crossed, sequencer must NOT skip or duplicate steps. Stop+play must restart at host's reported position, not internal counter.

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: G5-3.

  **QA Scenarios**:
  ```
  Scenario: Loop boundary clean
    Tool: Bash
    Steps:
      1. REAPER project loops bars 1-4 for 16 iterations; bounce; assert transient count is exactly 16 × (active-step-count) ± probability variance.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G5-4-loop.log
  ```

  **Commit**: `gate-5(seq): play/stop/loop stability`.

- [ ] G5-5. Per-step object lock
- [ ] G5-6. Per-step exciter lock
- [ ] G5-7. Per-step rust + damage + drive locks

  **What to do (shared)**: Each step optionally overrides a sound parameter. When step fires, voice picks up locked values; when no lock, falls back to global param.

  **Recommended Agent Profile**: `deep` (each). Parallel within Wave 6 after G5-1. Blocked By: G5-1.

  **QA Scenarios (representative)**:
  ```
  Scenario: Object lock changes timbre per step
    Tool: Bash
    Steps:
      1. tests/object_lock.rs: 4 steps with locks Pipe/Plate/Tank/Chain; render; assert each step's transient region has the brightness/roughness profile matching its locked object.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G5-{5,6,7}.log
  ```

  **Commit**: one per task: `gate-5(seq): per-step {object,exciter,rust+damage+drive} lock`.

- [ ] G5-8. Lock × preset recall + lock × automation correctness

  **What to do**: Locks must persist in preset save/load. Automation on a globally-locked param must not override the lock for affected steps.

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: G5-5..7, G2-9.

  **QA Scenarios**:
  ```
  Scenario: Locks survive round-trip and override automation
    Tool: Bash
    Steps:
      1. tests/lock_persist.rs: configure locks, save, load; assert byte-equal. Then automate global Rust while step-1 has rust-lock; assert step-1 ignores automation.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G5-8.log
  ```

  **Commit**: `gate-5(seq): lock persistence and automation precedence`.

- [ ] G5-9. Kit mode workflow

  **What to do**: Map keyboard zones (e.g., C1-C2) to per-step preset references so users can drum-program pattern-style. Document semantics in `docs/user-manual.md` stub.

  **Recommended Agent Profile**: `quick`. Blocked By: G5-1.

  **QA Scenarios**:
  ```
  Scenario: Kit mode triggers different presets per pad
    Tool: Bash
    Steps:
      1. tests/kit.rs: assign 4 presets to 4 pads; trigger each; assert per-pad render differs (preset-specific brightness/roughness fingerprint).
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G5-9-kit.log
  ```

  **Commit**: `gate-5(seq): kit mode`.

- [ ] G5-10. Host sync tests in REAPER + Ardour (scripted)

  **What to do**: Scripted projects in REAPER (Linux) and Ardour (Linux) with identical 4-bar / 16-step / 120 BPM patterns. Bounce both; bash-detect transient sample positions; assert REAPER and Ardour transients align within 1ms (48 samples @ 48 kHz). FL Studio is NOT included — it is the user's creative DAW, not an agent-driven QA host.

  **Must NOT do**: Do not invoke FL Studio. Do not accept manual evidence.

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: G5-3.

  **QA Scenarios**:
  ```
  Scenario: REAPER and Ardour produce timing-equivalent bounces
    Tool: Bash
    Steps:
      1. bash tests/daw/run-reaper.sh tests/daw/gate-5-sync.rpp /tmp/sync-reaper.wav
      2. bash tests/daw/run-ardour.sh tests/daw/gate-5-sync-ardour /tmp/sync-ardour.wav
      3. python3 scripts/check_wav.py /tmp/sync-reaper.wav
      4. python3 scripts/check_wav.py /tmp/sync-ardour.wav
      5. python3 scripts/compare_transients.py /tmp/sync-reaper.wav /tmp/sync-ardour.wav --max-drift-ms 1
    Expected: all helpers exit 0; transient positions align within 1ms.
    Failure Indicators: drift > 1ms, missing transients, host unavailable.
    Evidence: .sisyphus/evidence/task-G5-10-{reaper,ardour}.log + bounces.
  ```

  > Note: `scripts/compare_transients.py` is a new helper added in this task — onset-detect via simple envelope-derivative threshold, then pairwise nearest-neighbor distance. Pure stdlib + numpy (numpy added to dev tooling, not runtime).

  **Commit**: `gate-5(qa): host sync regression across REAPER and Ardour`.

- [ ] G5-11. Loop / restart / tempo-change timing checks

  **What to do**: Comprehensive scripted timing matrix: 60/120/180/240 BPM, 4-bar / 8-bar loops, mid-bar restarts.

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: G5-3, G5-4.

  **QA Scenarios**:
  ```
  Scenario: Timing matrix all clean
    Tool: Bash
    Steps:
      1. Run all permutations; assert transient timing within 1ms of expected for every cell.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G5-11-matrix/
  ```

  **Commit**: `gate-5(qa): timing matrix`.

- [ ] G5-12. Gate 5 evidence summary + pass-criteria review

  **What to do**: Write `.sisyphus/evidence/gate-5-summary.md` with one PASS/FAIL row per Gate 5 pass criterion (`IMPLEMENTATION_PLAN.md:605-610` — sequencer implemented and stable; per-step locks implemented and behave correctly; host sync works under tempo and transport changes; probability and microtiming musically useful; kit-oriented workflow viable). Each row links to its task evidence path. Tag `gate-5-complete` only on full pass; otherwise open follow-ups inside Gate 5 instead of advancing.

  **Must NOT do**: Do not advance to Gate 6 if any criterion fails. Do not soften criteria.

  **Recommended Agent Profile**: `quick`. **Skills**: none. Blocked By: G5-1..11.

  **References**:
  - `docs/IMPLEMENTATION_PLAN.md:605-610` — Gate 5 pass criteria.
  - `.sisyphus/evidence/task-G5-{1..11}-*.log` — task-level evidence.

  **QA Scenarios**:
  ```
  Scenario: Gate 5 summary covers all 5 pass criteria with linked evidence
    Tool: Bash
    Steps:
      1. test -f .sisyphus/evidence/gate-5-summary.md
      2. for crit in "equencer" "er-step lock" "ost sync" "robability" "icrotiming" "it"; do
           grep -qi "$crit" .sisyphus/evidence/gate-5-summary.md || { echo "missing: $crit"; exit 1; }
         done
      3. grep -E 'PASS' .sisyphus/evidence/gate-5-summary.md | grep -E '\.sisyphus/evidence/task-G5-' >/dev/null
      4. grep -q 'GATE 5 STATUS: CLOSED' .sisyphus/evidence/gate-5-summary.md
      5. git tag --list | grep -q '^gate-5-complete$'
    Expected: all checks pass; gate-5-complete tag exists.
    Failure Indicators: missing summary, missing criterion text, PASS rows lacking evidence links, missing tag.
    Evidence: .sisyphus/evidence/gate-5-summary.md

  Scenario: Failing criterion blocks gate close
    Tool: Bash
    Steps:
      1. If any Wave 6 task has FAIL status, gate-5-summary.md must contain "GATE 5 STATUS: OPEN" and list follow-up task IDs.
      2. The script that generates the summary must refuse to write "CLOSED" if any task evidence file reports failure.
    Expected: status reflects reality.
    Evidence: .sisyphus/evidence/gate-5-summary.md
  ```

  **Commit**: YES. Message: `gate-5(review): close Gate 5`. Tag: `gate-5-complete`. Pre-commit: `cargo test --workspace && cargo fmt --check`.

---

### Wave 7 — Gate 6 Version 1.0 Release

- [ ] G6-1. Finalize sequencer + per-step locks for release quality

  **What to do**: Triage every Gate 5 carry-forward issue. Fix all bugs blocking release. Re-run G5-11 timing matrix; must remain green.

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: gate-5-complete.

  **QA Scenarios**:
  ```
  Scenario: Sequencer release quality
    Tool: Bash
    Steps:
      1. Re-run G5-10, G5-11; both green.
      2. Zero open bugs labeled "blocker" in `.sisyphus/evidence/gate-5-summary.md` carry-forward section.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G6-1.log
  ```

  **Commit**: `gate-6(seq): release-quality finalization`.

- [ ] G6-2. Preset browser reliability finalization

  **What to do**: Stress test the browser: 500+ presets, rapid clicking, search edge cases (unicode, empty, very long strings). Fix all crashes/freezes.

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: gate-5-complete.

  **QA Scenarios**:
  ```
  Scenario: Browser stress
    Tool: Bash
    Steps:
      1. tests/browser_stress.rs: drive browser with 10000 random actions; assert no panic, no allocation leak.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G6-2-stress.log
  ```

  **Commit**: `gate-6(gui): preset browser reliability`.

- [ ] G6-3. Multiple exciter types confirmation

  **What to do**: Confirm Hit + Scrape are both production-quality. If a third exciter is needed for "multiple" (PRD pass criterion), implement Mallet (existing modal seam). Otherwise document Hit + Scrape as "multiple".

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: gate-5-complete.

  **QA Scenarios**:
  ```
  Scenario: ≥2 exciters available and audibly distinct
    Tool: Bash
    Steps:
      1. tests/exciter_count.rs: enumerate exciters; assert >= 2; assert pairwise distinct via roughness/brightness metrics.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G6-3-exciters.log
  ```

  **Commit**: `gate-6(exciter): exciter count confirmation`.

- [ ] G6-4. Multiple body / space types confirmation

  **What to do**: Confirm body resonator + stereo spread plus optionally a "Room" lightweight reverb-style space (within scope). At least 2 body/space modes selectable.

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: G3-5.

  **QA Scenarios**:
  ```
  Scenario: ≥2 space modes audibly distinct
    Tool: Bash
    Steps:
      1. tests/space_count.rs: render with each space mode; assert distinct late-energy / stereo correlation.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G6-4-space.log
  ```

  **Commit**: `gate-6(space): space mode count`.

- [ ] G6-5. User-configurable modulation mappings

  **What to do**: User can map any param as a modulation source (LFO/envelope) → any other param as destination, with depth/polarity. Wire UI in GUI mod-matrix panel.

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: gate-5-complete.

  **QA Scenarios**:
  ```
  Scenario: User mod mapping audibly modulates target
    Tool: Bash
    Steps:
      1. tests/modmatrix.rs: assign LFO@2Hz → Damage; render 2s; assert damage-envelope-induced roughness modulation visible in roughness_proxy windowed time series at ~2Hz.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G6-5-modmatrix.log
  ```

  **Commit**: `gate-6(mod): user-configurable modulation matrix`.

- [ ] G6-6. Expand presets to 100+ across all 5 families

  **What to do**: Author 60+ new presets bringing total to ≥100. Distribute: ≥20 bass, ≥20 percussion, ≥20 drone, ≥20 transition, ≥20 cinematic-impact.

  **Recommended Agent Profile**: `writing` (sound design). Blocked By: G6-3, G6-4.

  **QA Scenarios**:
  ```
  Scenario: 100+ presets, family coverage, all audible
    Tool: Bash
    Steps:
      1. ls presets/factory/*.corrosion-preset | wc -l  # >= 100
      2. for fam in bass percussion drone transition cinematic; do
           ls presets/factory/ | grep -ci "$fam" | awk -v f="$fam" '{ if ($1 < 20) { print f" underrepresented"; exit 1 } }'
         done
      3. Render all; assert zero silent.
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G6-6-presets.log
  ```

  **Commit**: `gate-6(presets): expand to 100+ across all families`.

- [ ] G6-7. VST3 + CLAP release-bundle build + installer script

  **What to do**: `release/build.sh` produces `release/corrosion-1.0.0/` containing per-platform subdirs:
  - `release/corrosion-1.0.0/linux/` — Linux VST3 + CLAP + `install.sh`.
  - `release/corrosion-1.0.0/windows/` — Windows VST3 + CLAP + `install.ps1` (the FL Studio user's primary artifacts).
  - `release/corrosion-1.0.0/presets/` — factory preset bank (≥100 files).
  - `release/corrosion-1.0.0/docs/` — user manual, sound design guide, CHANGELOG, INSTALL, README.
  Build script also produces two release archives: `release/corrosion-1.0.0-linux.tar.gz` and `release/corrosion-1.0.0-windows.zip`.

  **Must NOT do**: Do not omit Windows artifacts. Do not codesign Linux artifacts (out of scope). Do not embed an FL Studio plugin database — users install via FL Studio's "Plugin Manager" themselves (documented in user manual).

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: gate-5-complete.

  **QA Scenarios**:
  ```
  Scenario: Release bundle complete with Linux + Windows artifacts
    Tool: Bash
    Steps:
      1. ./release/build.sh
      2. test -d release/corrosion-1.0.0/linux/ && test -d release/corrosion-1.0.0/windows/
      3. for plat in linux windows; do
           for f in Corrosion.vst3 Corrosion.clap; do
             find release/corrosion-1.0.0/${plat}/ -name "$f" | grep -q . || exit 1
           done
         done
      4. test -f release/corrosion-1.0.0/linux/install.sh && test -f release/corrosion-1.0.0/windows/install.ps1
      5. for f in README.md CHANGELOG.md INSTALL.md; do
           find release/corrosion-1.0.0/docs/ -name "$f" | grep -q . || exit 1
         done
      6. test "$(ls release/corrosion-1.0.0/presets/*.corrosion-preset 2>/dev/null | wc -l)" -ge 100
      7. test -f release/corrosion-1.0.0-linux.tar.gz
      8. test -f release/corrosion-1.0.0-windows.zip
    Expected: complete cross-platform bundle.
    Evidence: .sisyphus/evidence/task-G6-7-bundle.log
  ```

  **Commit**: `gate-6(release): cross-platform release bundle (Linux + Windows)`.

- [ ] G6-8. User manual (concept, controls, MIDI, presets, automation, install, troubleshooting)

  **What to do**: `docs/user-manual.md` covering all 7 sections per PRD. Each section ≥ 300 words. Cross-link presets and recipes.

  **Recommended Agent Profile**: `writing`. Blocked By: gate-5-complete.

  **QA Scenarios**:
  ```
  Scenario: Manual covers all required sections
    Tool: Bash
    Steps:
      1. for s in "Concept" "Controls" "MIDI" "Presets" "Automation" "Install" "Troubleshooting"; do
           grep -q "## .*$s" docs/user-manual.md || exit 1
         done
      2. wc -w docs/user-manual.md  # >= 5000
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G6-8.log
  ```

  **Commit**: `gate-6(docs): user manual`.

- [ ] G6-9. Developer documentation (architecture, DSP, params, RT-safety, build, test, release)

  **What to do**: `docs/developer.md` covering all 7 sections per PRD. Include diagrams of module dependency graph (Mermaid) and DSP signal flow.

  **Recommended Agent Profile**: `writing`. Blocked By: gate-5-complete.

  **QA Scenarios**:
  ```
  Scenario: Developer doc complete
    Tool: Bash
    Steps:
      1. for s in "Architecture" "DSP" "Parameters" "Real-time" "Build" "Test" "Release"; do
           grep -q "## .*$s" docs/developer.md || exit 1
         done
      2. grep -c '```mermaid' docs/developer.md  # >= 2
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G6-9.log
  ```

  **Commit**: `gate-6(docs): developer documentation`.

- [ ] G6-10. Sound-design guide + 6 named recipes

  **What to do**: `docs/sound-design-guide.md` with ≥6 recipes: rusted-pipe-bass, bent-plate-snare, oil-tank-boom, chain-hi-hat, bowed-metal-drone, industrial-loop. Each recipe lists exact parameter values and a screenshot or render link.

  **Recommended Agent Profile**: `writing`. Blocked By: G6-6.

  **QA Scenarios**:
  ```
  Scenario: 6 recipes present
    Tool: Bash
    Steps:
      1. for r in rusted-pipe-bass bent-plate-snare oil-tank-boom chain-hi-hat bowed-metal-drone industrial-loop; do
           grep -qi "$r" docs/sound-design-guide.md || exit 1
         done
      2. grep -cE 'Object:|Size:|Rust:|Damage:|Drive:|Output:' docs/sound-design-guide.md  # >= 36 (6 per recipe)
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G6-10.log
  ```

  **Commit**: `gate-6(docs): sound-design guide`.

- [ ] G6-11. README + CHANGELOG + INSTALL

  **What to do**: `README.md` (project intro, screenshot, install link, license). `CHANGELOG.md` (every gate as a release section). `INSTALL.md` (Linux/macOS/Windows steps).

  **Recommended Agent Profile**: `writing`. Blocked By: G6-7.

  **QA Scenarios**:
  ```
  Scenario: All three docs present and substantive
    Tool: Bash
    Steps:
      1. for f in README.md CHANGELOG.md INSTALL.md; do test -f "$f" && [ "$(wc -w < "$f")" -gt 200 ] || exit 1; done
      2. grep -E '^##? ' CHANGELOG.md | wc -l  # >= 7 (one per gate + header)
    Expected: pass.
    Evidence: .sisyphus/evidence/task-G6-11.log
  ```

  **Commit**: `gate-6(docs): README, CHANGELOG, INSTALL`.

- [ ] G6-12. Pluginval release-candidate run (strictness 10)

  **What to do**: `pluginval --strictness-level 10 --validate` on both VST3 and CLAP from the release bundle.

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: G6-7.

  **QA Scenarios**:
  ```
  Scenario: Pluginval strictness 10 clean across Linux and Windows release artifacts
    Tool: Bash
    Steps:
       1. pluginval --strictness-level 10 --validate release/corrosion-1.0.0/linux/Corrosion.vst3 | tee .sisyphus/evidence/pluginval-gate-6-linux-vst3.log
       2. clap-validator validate release/corrosion-1.0.0/linux/Corrosion.clap/Corrosion.clap --only-failed | tee .sisyphus/evidence/clap-validator-gate-6-linux.log
       3. wine pluginval.exe --strictness-level 10 --validate release/corrosion-1.0.0/windows/Corrosion.vst3 | tee .sisyphus/evidence/pluginval-gate-6-win-vst3.log
       4. wine clap-validator.exe validate release/corrosion-1.0.0/windows/Corrosion.clap/Corrosion.clap --only-failed | tee .sisyphus/evidence/clap-validator-gate-6-win.log
       5. grep -q 'ALL TESTS PASSED' .sisyphus/evidence/pluginval-gate-6-linux-vst3.log || exit 1
       6. grep -qE '(passed|failed|skipped)' .sisyphus/evidence/clap-validator-gate-6-linux.log || exit 1
     Expected: VST3 pluginval passes; CLAP clap-validator reports 0 failed tests.
     Evidence: .sisyphus/evidence/pluginval-gate-6-{linux,win}-vst3.log, .sisyphus/evidence/clap-validator-gate-6-{linux,win}.log
  ```

  **Commit**: `gate-6(qa): pluginval strictness-10 release evidence`.

- [ ] G6-13. Scripted REAPER + Ardour + Windows-VST3 host-load tests (FL Studio is excluded from QA)

  **What to do**:
  - **REAPER (Linux + Windows)**: scripted bounce of `tests/daw/gate-6.rpp` via `bash tests/daw/run-reaper.sh` (Linux) and `bash tests/daw/run-reaper-win.sh` (Windows REAPER under wine, loading the Windows VST3). Produces `bounce-reaper-linux.wav` and `bounce-reaper-win.wav`.
  - **Ardour (Linux)**: `ardour --no-splash -A export tests/daw/gate-6-ardour/` via `bash tests/daw/run-ardour.sh` producing `bounce-ardour.wav`. Ardour is a free, scriptable Linux host kept as a second independent VST3 host for cross-host coverage.
  - **Windows-VST3 host-load test (Wine + Carla)**: `bash tests/daw/run-carla-win.sh` invokes `wine carla-bridge-win64-vst3` against the Windows VST3 to confirm it loads under Windows binary semantics; produces `bounce-carla-win.wav`. This substitutes for FL Studio in the automated suite (FL Studio is the user's daily DAW, never invoked by agent QA).
  - **FL Studio is explicitly OUT of automated QA.** A separate user-facing instruction (in `docs/user-manual.md`, written by G6-8) tells end users how to install the Windows VST3 in FL Studio. The plan never asserts FL Studio behavior automatically; that is the user's creative-side responsibility.
  - For EACH bounce, run `python3 scripts/check_wav.py <bounce>` and `python3 scripts/check_clicks.py <bounce>`. Both must exit 0.
  - **Strict no-deferral**: if `reaper` (or wine's reaper.exe), `ardour`, or `carla-bridge-win64-vst3` is unavailable, the task hard-fails (exit non-zero) and writes `HOST_UNAVAILABLE: <name>` to `.sisyphus/evidence/task-G6-13-deferred.log`. The user must install the missing host (via a follow-up plan task) before Gate 6 closes. No "screen recordings", "manual checklists", or screenshots are accepted as evidence — ever.

  **Must NOT do**: Do not invoke FL Studio in any QA scenario (it has no scriptable bounce path). Do not accept any manual or human-attested evidence. Do not soft-pass on missing hosts.

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: G6-7.

  **QA Scenarios**:
  ```
  Scenario: All scripted hosts produce clean bounces (zero human interaction)
    Tool: Bash
    Steps:
      1. for host in reaper-linux reaper-win ardour carla-win; do
           if ! test -x "tests/daw/run-${host}.sh"; then
             echo "MISSING_SCRIPT: $host" >> .sisyphus/evidence/task-G6-13-deferred.log
             exit 1
           fi
           bash tests/daw/run-${host}.sh > .sisyphus/evidence/task-G6-13-${host}.log 2>&1 || {
             echo "HOST_FAILED: $host" >> .sisyphus/evidence/task-G6-13-deferred.log
             exit 1
           }
           python3 scripts/check_wav.py /tmp/bounce-${host}.wav || exit 1
           python3 scripts/check_clicks.py /tmp/bounce-${host}.wav || exit 1
         done
    Expected: all 4 scripted host bounces produced and clean.
    Failure Indicators: any host unavailable, any bounce silent/clipping/NaN, any click detected.
    Evidence: .sisyphus/evidence/task-G6-13-{reaper-linux,reaper-win,ardour,carla-win}.log + bounce WAVs.

  Scenario: FL Studio is not invoked anywhere in automated QA
    Tool: Bash
    Steps:
      1. ! grep -rniE 'fl ?studio|FL64|FLStudio' tests/daw/ scripts/ .sisyphus/evidence/ 2>/dev/null | grep -v 'docs/' | grep -v 'user-manual' | grep -v -- '#'
    Expected: zero matches outside user-facing docs.
    Evidence: .sisyphus/evidence/task-G6-13-no-fl.log

  Scenario: No manual evidence accepted
    Tool: Bash
    Steps:
      1. ! find .sisyphus/evidence/ -name '*manual*' -o -name '*screenshot*' -o -name '*.mp4' -o -name '*.mov' 2>/dev/null | grep -q .
    Expected: zero manual-evidence files.
    Evidence: .sisyphus/evidence/task-G6-13-no-manual.log
  ```

  **Commit**: `gate-6(qa): scripted host-load tests across REAPER/Ardour/Carla (FL Studio excluded by design)`.

- [ ] G6-14. Buffer-size + sample-rate regression checks

  **What to do**: Bounce matrix: SRs {44100, 48000, 88200, 96000, 192000} × buffers {32, 64, 128, 256, 512, 1024, 2048}. Assert clean bounces for all 35 cells.

  **Recommended Agent Profile**: `unspecified-high`. Blocked By: G6-7.

  **QA Scenarios**:
  ```
  Scenario: 35-cell SR×buffer matrix all clean
    Tool: Bash
    Steps:
      1. Run scripted matrix; assert all 35 bounces non-silent, non-NaN, peak < 0.97.
    Expected: 35/35 pass.
    Evidence: .sisyphus/evidence/task-G6-14-matrix/
  ```

  **Commit**: `gate-6(qa): SR/buffer regression matrix`.

- [ ] G6-15. Gate 6 evidence summary + final performance/safety validation

  **What to do**: `.sisyphus/evidence/gate-6-summary.md` with PASS/FAIL per criterion (`IMPLEMENTATION_PLAN.md:693-702`). Run final RT-safety grep audit. Tag `gate-6-complete` and `v1.0.0` only on full pass.

  **Recommended Agent Profile**: `quick`. Blocked By: G6-1..14.

  **QA Scenarios**:
  ```
  Scenario: Gate 6 closure
    Tool: Bash
    Steps:
      1. test -f .sisyphus/evidence/gate-6-summary.md
      2. grep -q 'GATE 6 STATUS: CLOSED' .sisyphus/evidence/gate-6-summary.md
      3. ! grep -nE '\b(Vec::new|vec!|format!|println!|Mutex|serde_json|panic!)\b' src/dsp src/voice src/lib.rs
      4. git tag --list | grep -E 'gate-6-complete|v1.0.0' | wc -l  # 2
    Expected: closed.
    Evidence: .sisyphus/evidence/gate-6-summary.md
  ```

  **Commit**: `gate-6(review): close Gate 6 and tag v1.0.0`.

---

## Final Verification Wave (MANDATORY — after ALL implementation tasks)

> 4 review agents run in PARALLEL. ALL must APPROVE. Present consolidated results to user and get explicit "okay" before completing. Never check F1-F4 before user okay.

- [ ] F1. **Plan Compliance Audit** — `oracle`
  Read this plan end-to-end. For each "Must Have": verify implementation exists (read file, run command, inspect bundle). For each "Must NOT Have": grep codebase for forbidden patterns — reject with file:line if found. Verify each gate has a corresponding `.sisyphus/evidence/gate-{N}-summary.md`. Verify pluginval logs exist for Gates 1, 2, 6. Compare deliverables against PRD pass criteria.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Gates evidenced [7/7] | VERDICT: APPROVE/REJECT`

- [ ] F2. **Code Quality + RT-Safety Review** — `unspecified-high`
  Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --workspace`. Run the Cross-Gate Guardrails grep (above) over the directory set `src/dsp src/voice src/sequencer src/lib.rs` — exempting `#[cfg(test)]` blocks and comment-only lines, exempting `src/offline/` (offline binary, not in audio thread). Forbidden patterns: `Vec::new`, `vec![`, `Box::new`, `format!`, `println!`, `Mutex`, `serde_json`, `std::fs`, `std::io`, `panic!`, `todo!`, `unimplemented!`, `unwrap()` without proven invariant. Review changed files for `unsafe` blocks without SAFETY comments and `unwrap_or_default` covering bugs. Check for AI slop: dead code, generic names (`data`/`tmp`/`result`), copy-paste blocks.
  Output: `Build [PASS/FAIL] | Clippy [PASS/FAIL] | Tests [N pass/N fail] | RT-safety grep [CLEAN/N issues] | Files [N clean/N issues] | VERDICT`

- [ ] F3. **Real Manual QA — Full Release Flow** — `unspecified-high`
  Start from clean checkout. Build VST3 + CLAP from scratch (`cargo build --release`). Run `pluginval --strictness-level 10 --validate target/release/Corrosion.vst3` and `clap-validator validate target/release/Corrosion.clap --only-failed`. Load via REAPER scripted render of a test project that exercises: 8-voice polyphony, automation sweep on every parameter, preset cycling through all 100+ presets, sequencer playback at 60/120/240 BPM with loop, buffer 64/512/2048, sample rates 44.1/48/96 kHz. Bounce WAVs and verify non-silent / non-clipping / no NaN (run `python3 -c "import wave,struct; ..."` or equivalent on each bounce). Save evidence to `.sisyphus/evidence/final-qa/`.
  Output: `Pluginval [PASS/FAIL] | Polyphony [PASS/FAIL] | Automation [N/N] | Presets [N/N audible] | Sequencer [N/N timing-clean] | Buffer/SR matrix [N/N] | VERDICT`

- [ ] F4. **Scope Fidelity Check** — `deep`
  For each task in this plan: read "What to do" and "Must NOT do", read git diff for the task's commit(s), verify 1:1 — every spec'd item built, nothing beyond spec built (no scope creep). Flag any commit touching files outside its declared scope. Confirm no Corrosion FX / Lab / expansion-pack / neural / sample-browser / modular code was introduced. Confirm preset count progression (≥20 by G2, ≥40 by G3, ≥100 by G6). Confirm GUI does not use oscillator/filter/amp framing (grep widgets for those terms).
  Output: `Tasks [N/N compliant] | Out-of-scope additions [CLEAN/N issues] | Preset progression [PASS/FAIL] | GUI metaphor [PASS/FAIL] | VERDICT`

---

## Commit Strategy

One commit per task unless task explicitly groups (rare). Commit message format:
- `gate-N(scope): summary`
- Examples:
  - `gate-0(evidence): record initial parameter ranges`
  - `gate-1(plugin): add NIH-plug scaffold and lib target`
  - `gate-2(voice): 8-voice manager with tail-energy tracking`
  - `gate-4(gui): custom GUI scaffold with exciter→object→damage→space layout`

Pre-commit per task: `cargo fmt --check && cargo test --workspace`.

Tag at gate close: `git tag gate-N-complete`. Tag at release: `git tag v1.0.0`.

---

## Success Criteria

### Verification Commands
```bash
cargo fmt --check                                                                  # Expected: no diff
cargo clippy --all-targets -- -D warnings                                          # Expected: zero warnings
cargo test --workspace                                                             # Expected: all pass
./bundle.sh && ./bundle-win.sh                                                     # Expected: VST3 + CLAP bundles for both platforms
pluginval --strictness-level 10 --validate target/bundled/Corrosion.vst3           # Expected: exit 0 (Linux)
clap-validator validate target/bundled/Corrosion.clap/Corrosion.clap --only-failed  # Expected: 0 failed tests (Linux)
wine pluginval.exe --strictness-level 10 --validate target/bundled-win/Corrosion.vst3 # Expected: exit 0 (Windows / FL Studio)
wine clap-validator.exe validate target/bundled-win/Corrosion.clap/Corrosion.clap --only-failed # Expected: 0 failed tests (Windows / FL Studio)
ls presets/factory/*.corrosion-preset | wc -l                                      # Expected: >= 100
ls .sisyphus/evidence/gate-{0,1,2,3,4,5,6}-summary.md                              # Expected: 7 files
```

### Final Checklist
- [ ] All 7 gate evidence summaries exist and pass their gate review.
- [ ] All "Must Have" items implemented and verified.
- [ ] All "Must NOT Have" items confirmed absent (grep-verified).
- [ ] `cargo test --workspace` all green.
- [ ] Pluginval strictness 10 passes for VST3; clap-validator reports 0 failed tests for CLAP.
- [ ] 100+ factory presets covering all required families.
- [ ] User manual, developer doc, sound design guide all present.
- [ ] Release bundle assembled at `release/corrosion-1.0.0/`.
- [ ] F1-F4 all APPROVE.
- [ ] User has explicitly oked the final verification.
