# PROJECT KNOWLEDGE BASE

**Generated:** 2026-05-03
**Commit:** `9bf3614`
**Branch:** `main`

## OVERVIEW
Rust `nih_plug` instrument plugin repo. Core work splits across real-time DSP (`src/`), design/spec docs (`docs/`), verification (`tests/`, `scripts/`), and planning/evidence (`.sisyphus/`).

## STRUCTURE
```text
./
├── src/                    # plugin, DSP, voice, presets
├── docs/                   # PRD, implementation plan, detailed DSP specs
├── tests/                  # integration tests + DAW smoke scripts
├── scripts/                # stdlib QA helpers for WAV analysis/validation
├── presets/factory/        # user-facing preset bank
├── .sisyphus/              # roadmap, evidence, notepads, gate history
├── bundle.sh               # Linux plugin bundle builder
└── bundle-win.sh           # Windows cross-compile bundle builder
```

## WHERE TO LOOK
| Task | Location | Notes |
|------|----------|-------|
| Plugin entry / MIDI / processing | `src/lib.rs` | host-facing process loop, exports, editor hook |
| DSP math / modal objects | `src/dsp/` | highest complexity area |
| Real-time voice behavior | `src/voice/` | note lifecycle, stealing, excitation, RT constraints |
| Preset/state persistence | `src/presets/` | JSON shape, host snapshot helpers |
| Build targets / packaging | `Cargo.toml`, `.cargo/config.toml`, `bundle*.sh` | default musl target; bundled outputs |
| Detailed algorithm intent | `docs/new-detailed-specs/` | authoritative DSP-detail layer |
| Feature inventory / sound intent | `docs/full-feature-surface.md`, `docs/sound-direction-brief.md` | keep aligned with code |
| QA / regression / host checks | `tests/`, `scripts/` | deterministic tests + DAW scripts + WAV helpers |
| Gate requirements / evidence rules | `.sisyphus/plans/corrosion-roadmap.md` | source of truth for sequencing + closure |

## CODE MAP
| Symbol | Type | Location | Role |
|--------|------|----------|------|
| `Corrosion` | struct | `src/lib.rs` | plugin root: params + voice manager |
| `process()` | plugin method | `src/lib.rs` | MIDI event consumption + sample loop |
| `VoiceManager` | struct | `src/voice/manager.rs` | polyphony, stealing, summing |
| `Voice` | struct | `src/voice/mod.rs` | excitation + resonator + note lifecycle |
| `CorrosionParams` | struct | `src/params.rs` | host parameter surface |
| `Preset` | struct | `src/presets/mod.rs` | file persistence/state bridge |

## CONVENTIONS
- Default Cargo target is `x86_64-unknown-linux-musl`; GNU and Windows builds are explicit.
- Build artifacts go to `../corrotion-target`, then are copied into `target/bundled*` by shell wrappers.
- Gate/roadmap docs are active engineering inputs, not archival prose.
- `docs/new-detailed-specs/*.md` is authoritative for algorithm identity from Gate 3 onward.
- Evidence files live under `.sisyphus/evidence/`; don’t invent alternate evidence locations without reason.

## ANTI-PATTERNS (THIS PROJECT)
- Don’t add audio-thread allocation, file I/O, logging, mutexes, JSON parsing, or blocking work in `process()` paths.
- Don’t use oscillator/filter/amp synth framing in UI/docs; use exciter/object/damage/space vocabulary.
- Don’t silently replace a named algorithm family from `docs/new-detailed-specs/` with a cheap substitute.
- Don’t mark gate tasks done without evidence artifacts and roadmap checkbox alignment.
- Don’t treat generated folders (`target/`, `output/`, `.sisyphus/evidence/`) as source-of-truth code.

## UNIQUE STYLES
- Verification is gate-based and artifact-heavy: tests, logs, bounces, summaries, and tags all matter.
- Python helpers are tiny stdlib CLIs with parseable stdout and exit-code-as-contract.
- Preset names/files are semantic and category-driven, not generic numbered variants.

## COMMANDS
```bash
cargo test --workspace --no-default-features
cargo build --target x86_64-unknown-linux-gnu
./bundle.sh
./bundle-win.sh
python3 scripts/check_wav.py <wav>
python3 scripts/check_clicks.py <wav>
python3 scripts/analyze_wav.py <wav>
```

## NOTES
- No existing `AGENTS.md` were present before this initialization.
- `docs/` has its own workflow and terminology; see `docs/AGENTS.md`.
- `src/dsp/` and `src/voice/` have distinct local rules; use the nearest child AGENTS file when editing there.
