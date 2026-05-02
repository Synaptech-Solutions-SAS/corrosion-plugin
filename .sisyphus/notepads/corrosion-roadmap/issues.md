## 2026-05-02 G-Setup

### DEFERRED: mingw-w64 + wine (Windows cross-compile toolchain)
- apt-get not available in this WSL environment
- x86_64-pc-windows-gnu target is installed in rustup but the linker (x86_64-w64-mingw32-gcc) is missing
- wine is missing (needed for pluginval.exe against Windows bundles)
- Impact: G1-7 (Windows VST3 bundle), G1-8 (Windows CLAP bundle), and all Windows pluginval QA scenarios are BLOCKED
- Resolution: User must install mingw-w64 and wine manually, OR we skip Windows bundles until a CI environment with these tools is available
- Workaround for now: Linux bundles only for Gates 1-5; Windows bundle deferred to Gate 6 release engineering

### git not in Linux PATH
- git.exe is at /mnt/c/Program Files/Git/bin/git.exe (Windows Git)
- All git commands must use this full path
- Consider adding to PATH in future sessions: export PATH="$PATH:/mnt/c/Program Files/Git/bin"
