#!/usr/bin/env bash
set -euo pipefail

PLUGIN_NAME="Corrosion"
TARGET="x86_64-unknown-linux-gnu"
PROFILE="${1:-release}"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR"
TARGET_DIR="${PROJECT_DIR}/../corrotion-target"

echo "Building ${PLUGIN_NAME} for ${TARGET} (${PROFILE})..."
cargo build --target "${TARGET}" --"${PROFILE}" --lib

BUNDLE_DIR="${PROJECT_DIR}/target/bundled/${PLUGIN_NAME}.vst3"
rm -rf "${BUNDLE_DIR}"
mkdir -p "${BUNDLE_DIR}/Contents/x86_64-linux"

SO_PATH="${TARGET_DIR}/${TARGET}/${PROFILE}/libcorrotion.so"
if [ ! -f "${SO_PATH}" ]; then
    echo "Error: Built library not found at ${SO_PATH}"
    exit 1
fi

cp "${SO_PATH}" "${BUNDLE_DIR}/Contents/x86_64-linux/${PLUGIN_NAME}.so"
echo "VST3 bundle created at: ${BUNDLE_DIR}"

CLAP_DIR="${PROJECT_DIR}/target/bundled/${PLUGIN_NAME}.clap"
rm -rf "${CLAP_DIR}"
mkdir -p "${CLAP_DIR}"
cp "${SO_PATH}" "${CLAP_DIR}/${PLUGIN_NAME}.clap"
echo "CLAP bundle created at: ${CLAP_DIR}"

echo "Done."
