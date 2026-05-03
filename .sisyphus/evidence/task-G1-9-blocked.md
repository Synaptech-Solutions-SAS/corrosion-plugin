# G1-9 Pluginval + Host Smoke Evidence

## Status

G1-9 remains **BLOCKED** for full gate closure because the scripted REAPER bounce cannot run yet.

## What Passed

- Linux VST3 bundle exists at `target/bundled/Corrosion.vst3/Contents/x86_64-linux/Corrosion.so`.
- Linux CLAP bundle exists at `target/bundled/Corrosion.clap/Corrosion.clap`.
- `pluginval` v1.0.3 was downloaded without sudo and made runnable by extracting AlmaLinux runtime RPMs into `/tmp/pluginval-rpm-libs`.
- VST3 validation passes with exit code 0:
  - Command: `LD_LIBRARY_PATH=/tmp/pluginval-rpm-libs/extract/usr/lib64:/tmp/pluginval-rpm-libs/extract/usr/lib /tmp/pluginval-v1.0.3/pluginval --validate target/bundled/Corrosion.vst3 --strictness-level 5 --skip-gui-tests`
  - Evidence: `.sisyphus/evidence/pluginval-gate-1-linux-vst3.log`
- CLAP validation passes with `clap-validator`:
  - Command: `$HOME/.local/bin/clap-validator validate target/bundled/Corrosion.clap/Corrosion.clap --only-failed`
  - Evidence: `.sisyphus/evidence/clap-validator-gate-1-linux.log`

## What Failed / Is Blocked

- REAPER is installed at `/usr/local/bin/reaper`, but cannot start in this environment:
  - Without extracted libs: `libasound.so.2: cannot open shared object file`.
  - With extracted ALSA/font libs: `libX11.so.6: cannot open shared object file`.
  - After installing ALSA/X11 dependencies, current blocker is `libGL.so.1: cannot open shared object file`.
- `tests/daw/run-reaper.sh` and `tests/daw/gate-1.rpp` do not exist yet, so scripted host bounce evidence cannot be produced.
- Windows validation remains blocked because `wine` and `x86_64-w64-mingw32-gcc` are unavailable.

## Gate Decision

Gate 1 cannot be closed as `CLOSED` until the REAPER scripted bounce can run or an approved scripted host substitute is added.
