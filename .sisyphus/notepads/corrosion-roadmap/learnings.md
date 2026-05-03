## 2026-05-03 GATE 1 CLOSED - All Blockers Resolved

### Resolution Summary
All Gate 1 blockers have been resolved and the gate is now CLOSED.

### Completed Actions
1. ✅ Installed rustup via paru
2. ✅ Added musl target: `rustup target add x86_64-unknown-linux-musl`
3. ✅ Added Windows target: `rustup target add x86_64-pc-windows-gnu`
4. ✅ Installed pluginval: `paru -S pluginval`
5. ✅ Installed clap-validator: `paru -S clap-validator`
6. ✅ Installed REAPER: `paru -S reaper` (libGL issue resolved)
7. ✅ Installed mingw-w64: `paru -S mingw-w64-gcc mingw-w64-binutils`
8. ✅ Created bundle-win.sh for Windows cross-compile
9. ✅ Created tests/daw/run-reaper.sh for DAW smoke tests
10. ✅ All 51 tests passing
11. ✅ Pluginval strictness 5 SUCCESS
12. ✅ CLAP validator 18/18 passed
13. ✅ Git tag `gate-1-complete` created

### Current Environment State

**Rust Toolchain:**
- rustc 1.95.0
- cargo 1.95.0
- Targets: x86_64-unknown-linux-gnu, x86_64-unknown-linux-musl, x86_64-pc-windows-gnu

**Build Tools:**
- gcc 16.1.1
- mingw-w64-gcc 15.2.0 (for Windows cross-compile)

**Validation Tools:**
- pluginval 1.0.4
- clap-validator 0.3.2
- REAPER (installed and working)

**Platform Coverage:**
- ✅ Linux x86_64 native builds
- ✅ Windows x86_64 cross-compile
- ⚠️ macOS via GitHub Actions (not local)

### Build Scripts
- `bundle.sh` - Linux VST3 + CLAP bundles
- `bundle-win.sh` - Windows VST3 + CLAP bundles (cross-compile from Linux)

### Git Tag
```bash
gate-1-complete  # Created 2026-05-03
```

### Next Phase
Ready for Gate 2 (MVP 0.1.0):
- Expand parameters (Size, Rust, Damage, Drive, Output)
- MIDI note frequency scaling
- 20+ factory presets
- Generic editor
- Hard safety limiter

---

## 2026-05-02 G1-7..10 RESOLVED: gcc available, NIH-plug builds

### Resolution
- gcc 11.5.0 installed and available at `/usr/bin/gcc`
- `.cargo/config.toml` configured with `linker = "gcc"` for `x86_64-unknown-linux-gnu` target
- NIH-plug dependency added to Cargo.toml
- `cargo build --target x86_64-unknown-linux-gnu --lib` produces `libcorrotion.so`
- All 51 tests pass
- Plugin implements Plugin, ClapPlugin, Vst3Plugin traits

### Verification Commands That Work
```bash
cargo test --workspace  # 51 tests pass
./bundle.sh             # Creates Linux VST3/CLAP bundles
./bundle-win.sh         # Creates Windows VST3/CLAP bundles
pluginval --strictness-level 5 --validate target/bundled/Corrosion.vst3  # SUCCESS
clap-validator validate target/bundled/Corrosion.clap/Corrosion.clap  # 18/18 passed
```

### Remaining Blockers (NOW RESOLVED - see above)
~~- Windows cross-compile (`x86_64-pc-windows-gnu`) still needs `x86_64-w64-mingw32-gcc`~~
~~- Linux VST3 pluginval can run via `/tmp/pluginval-v1.0.3/pluginval` plus `LD_LIBRARY_PATH`~~
~~- CLAP pluginval is not the correct validator; `clap-validator` 0.3.2 works~~
~~- REAPER remains blocked: `/usr/local/bin/reaper` exists, but runtime libraries missing~~

---

## 2026-05-01 G-Setup COMPLETE

### What Was Done
1. Git initialized
2. `.cargo/config.toml` created with musl default + windows-gnu target
3. `.gitignore` created
4. Helper scripts `scripts/check_wav.py` and `scripts/check_clicks.py` created
5. Initial commit made
6. Baseline verified: `cargo test` and `cargo run` work

### Configuration
Target directory redirected to `../corrotion-target` to avoid polluting repo.

- Gate 2 param expansion: keep stable `#[id]` values on every host-facing param (`object`, `size`, `rust`, `damage`, `drive`, `output`) so automation survives reordering.
- Use `FloatParam::new(name, default, FloatRange::Linear { min, max })` for the MVP scalar controls; `Output` can stay as linear gain with a 0 dB default and +12 dB ceiling.

## 2026-05-03 - Drive/Output Wiring in Process Loop

- Applied per-sample drive saturation in `src/lib.rs` after `voice_manager.process_sample()`: `sample * (1.0 + drive * 3.0)` then `tanh()`.
- Applied output gain after drive, then kept the final safety clamp to `[-1.0, 1.0]`.
- Read `drive` and `output` from `CorrosionParams` inside the process loop so host automation can affect the hot path without touching voice code.

## 2026-05-03 - G2-7 Allocation Audit Notes

- `Voice::process_sample()` now flushes denormals with `clamped + 1e-20 - 1e-20` before returning.
- The no-alloc regression test must keep voice setup outside the guarded section; arming voices can allocate even when the render loop does not.
- `cargo test --workspace` still passed all 51 unit tests plus the integration tests after the guard change.
- The grep audit over `src/dsp/resonator.rs`, `src/voice/mod.rs`, `src/voice/manager.rs`, and `src/lib.rs` produced an empty result set.

## 2026-05-03 - G2-9 Preset IO Notes

- NIH-plug `Plugin` in this repo version does not expose `get_state` / `load_state`; preset snapshotting has to live as inherent helpers on `Corrosion` and/or in the `Params` snapshot path.
- The preset format works cleanly when the plugin params are rebuilt from a `Preset` and the `Arc<CorrosionParams>` is swapped on load.
- `preset-render` should resolve the fixture path from the repo root (`tests/fixtures/default.corrosion-preset`), not from `.sisyphus/`.
