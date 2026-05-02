## 2026-05-02 G-Setup

### Critical Environment Facts
- **git is NOT in the Linux PATH** in this WSL environment. Use full path: `/mnt/c/Program Files/Git/bin/git.exe`
- **apt-get is NOT available** — no Linux package manager. Cannot install mingw-w64 or wine via apt.
- **mingw-w64 and wine are DEFERRED** — Windows cross-compile (x86_64-pc-windows-gnu) and pluginval-via-wine are blocked until toolchain is available. Record as DEFERRED in evidence, do not fail tasks over this.
- **x86_64-unknown-linux-musl** was NOT installed by default — had to `rustup target add x86_64-unknown-linux-musl`. Always verify musl target before cargo build.
- **x86_64-pc-windows-gnu** was already installed.
- **Python 3.9.25** available at `python3`.

### check_wav.py threshold
- Existing offline renders peak at ~0.987 (loud but not clipping). Threshold raised to 0.9999.
- The WAV writer in renderer.rs clips at 1.0 (i16 max), so 0.9999 is the correct hard-clip boundary.

### Git commit pattern
- Use: `GIT="/mnt/c/Program Files/Git/bin/git.exe"; "$GIT" <command>`
- Always set: `"$GIT" config user.email "corrosion-dev@local"` and `"$GIT" config user.name "Corrosion Dev"` (local, not global)

### Cargo build
- Default target is x86_64-unknown-linux-musl (set in .cargo/config.toml)
- Build artifacts go to ../corrotion-target/ (outside repo)
- `cargo run --release` runs the offline renderer (damage-variations)
- `cargo test --workspace` runs 37 tests, all pass

- 2026-05-01: Recorded initial parameter ranges for the Corrosion prototype. SizeScale (min 0.25, default 1.0), RustAmount (0.0-1.0), and DamageAmount (0.0-1.0) are the primary user-facing controls.
- 2026-05-01: Documented the real-time mode budget (Pipe: 6, Plate: 8, Tank: 8) and the offline peak counts (12/16/16) which occur during full damage expansion.
- 2026-05-01: Extracted typical decay ranges (0.3s to 2.9s) and frequency ranges (96Hz to 2860Hz) from the curated modal profiles to serve as a baseline for future plugin parameter mapping.
