## 2026-05-03 - NO OUTSTANDING ISSUES

All previously tracked issues have been resolved. Gate 1 is CLOSED.

### Resolved Issues

| Issue | Resolution Date | Resolution |
|-------|----------------|------------|
| Missing rustup | 2026-05-03 | Installed via paru -S rustup |
| Missing musl target | 2026-05-03 | rustup target add x86_64-unknown-linux-musl |
| Missing Windows target | 2026-05-03 | rustup target add x86_64-pc-windows-gnu |
| Missing pluginval | 2026-05-03 | paru -S pluginval |
| Missing clap-validator | 2026-05-03 | paru -S clap-validator |
| Missing REAPER | 2026-05-03 | paru -S reaper |
| REAPER libGL.so.1 error | 2026-05-03 | mesa/libglvnd already present, REAPER works |
| Missing mingw-w64 | 2026-05-03 | paru -S mingw-w64-gcc mingw-w64-binutils |
| Missing Windows bundle script | 2026-05-03 | Created bundle-win.sh |
| Missing DAW test infrastructure | 2026-05-03 | Created tests/daw/run-reaper.sh |

### Current Status: ALL CLEAR ✅

No blockers. Ready for Gate 2 development.

---

## Historical Issues (RESOLVED)

### 2026-05-02 - REAPER Runtime Dependencies
**Status**: RESOLVED 2026-05-03

REAPER installed at `/usr/local/bin/reaper` but failed to start:
- Initially: `libasound.so.2: cannot open shared object file`
- Then: `libX11.so.6: cannot open shared object file`
- Finally: `libGL.so.1: cannot open shared object file`

**Resolution**: All dependencies already present via mesa/libglvnd. REAPER now starts successfully.

### 2026-05-02 - Windows Cross-Compile
**Status**: RESOLVED 2026-05-03

Missing `x86_64-w64-mingw32-gcc` for Windows cross-compilation.

**Resolution**: Installed mingw-w64-gcc. Created bundle-win.sh. Windows bundles build successfully.

### 2026-05-02 - Validation Tools
**Status**: RESOLVED 2026-05-03

pluginval and clap-validator not installed.

**Resolution**: Both installed via paru. All validations passing.

---

## Carry-Forward Notes (Non-Blocking)

These are known limitations tracked for future gates, not blockers:

### Gate 2 Tasks
- Tank profile overshoot (~4× peak) - needs gain normalization
- ModalModeSpec::damaged() allocates Vec - redesign for real-time
- MIDI note pitch not applied to profiles - implement frequency scaling
- Gain parameter not applied in process() - wire it up

### Gate 3+ Tasks
- macOS in-host validation requires actual Mac hardware or cloud instance
- AU format support deferred - evaluate Truce framework in Gate 6 if needed
- LV2 support not planned - VST3 covers all Linux hosts

### No Action Required
- Windows bundles validated via file command (PE32+ format)
- In-host Windows testing would require Wine or Windows VM
- macOS bundles validated via GitHub Actions CI

- `lsp_diagnostics` could not run in this environment because `rust-analyzer` is not installed; build + pluginval were used instead for verification.

## 2026-05-03 - Verification Tooling Alignment

- `lsp_diagnostics` required `rust-analyzer` on PATH; the binary was present in the Rust toolchain but needed a symlink into `~/.local/bin` for this environment.
- `pluginval` validated the bundle successfully, but the external VST3 validator path was not installed, so the built-in validation suite ran without that extra validator layer.

## 2026-05-03 - G2-7 Allocation Audit Issue

- `assert_no_alloc` from crates.io conflicted with NIH-plug’s existing allocator setup when I tried to install a second `#[global_allocator]` in the test crate.
- Fix: use the matching `assert_no_alloc` git source already used by NIH-plug, and keep voice setup outside the guarded render loop.

## 2026-05-03 - G2-9 Smoke Test Path Mistake

- First preset-render smoke run failed because the preset fixture path was resolved under `.sisyphus/` instead of the repo root.
- Fixed by invoking the bin with `--preset tests/fixtures/default.corrosion-preset`.
