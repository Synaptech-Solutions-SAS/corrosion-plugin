#!/usr/bin/env bash
set -euo pipefail

# Optional host-facing validator wrapper.
# This script is intentionally explicit about missing external tools so release
# validation can be run locally without hiding environment requirements.

PROJECT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "${PROJECT_DIR}"

if [ ! -d "target/bundled/Corrosion.vst3" ] || [ ! -f "target/bundled/Corrosion.clap/Corrosion.clap" ]; then
  echo "==> Bundles missing, building release bundles first"
  ./bundle.sh release
fi

if command -v pluginval >/dev/null 2>&1; then
  echo "==> pluginval"
  pluginval --strictness-level 5 --validate target/bundled/Corrosion.vst3 --skip-gui-tests
else
  echo "pluginval not found, skipping VST3 validator"
fi

if command -v clap-validator >/dev/null 2>&1; then
  echo "==> clap-validator"
  clap-validator validate target/bundled/Corrosion.clap/Corrosion.clap --only-failed
else
  echo "clap-validator not found, skipping CLAP validator"
fi
