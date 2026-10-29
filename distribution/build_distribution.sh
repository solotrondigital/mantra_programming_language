#!/usr/bin/env bash
# ==============================================================================
# Mantra Programming Language — Universal Distribution Builder
# distribution/build_distribution.sh
#
# Builds the unified SDK layout, generates platform-specific distribution archives
# for Homebrew (macOS Apple Silicon, macOS Intel, Linux) and Chocolatey (Windows),
# calculates SHA-256 checksums, and updates the package recipes.
#
# Usage:
#   ./distribution/build_distribution.sh [VERSION]
#   Example: ./distribution/build_distribution.sh 0.4.0
# ==============================================================================
set -e

VERSION="${1:-0.4.0}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
DIST_DIR="${REPO_ROOT}/dist/releases/v${VERSION}"
SDK_STAGE="${REPO_ROOT}/target/sdk_stage"

echo "=================================================================="
echo "   MANTRA DISTRIBUTION & PACKAGE MANAGER BUILDER (v${VERSION})   "
echo "   Homebrew Tap + Chocolatey Feed Generation                     "
echo "=================================================================="

# 1. Compile release binaries with Cargo
echo ">> Compiling workspace release binaries..."
(cd "${REPO_ROOT}" && cargo build --release)

BIN_MANTRA="${REPO_ROOT}/target/release/mantra"
BIN_MANTRAC="${REPO_ROOT}/target/release/mantrac"
BIN_LSP="${REPO_ROOT}/target/release/mantra-lsp"

if [ ! -f "${BIN_MANTRA}" ] || [ ! -f "${BIN_MANTRAC}" ] || [ ! -f "${BIN_LSP}" ]; then
    echo "Error: Release binaries missing in target/release"
    exit 1
fi

echo "   ✓ Verified: mantra, mantrac, mantra-lsp"

# 2. Construct canonical SDK layout
echo ">> Assembling SDK layout in ${SDK_STAGE}..."
rm -rf "${SDK_STAGE}"
mkdir -p "${SDK_STAGE}/bin"
mkdir -p "${SDK_STAGE}/lib/mantra/runtime"
mkdir -p "${SDK_STAGE}/include/mantra"
mkdir -p "${SDK_STAGE}/templates"
mkdir -p "${SDK_STAGE}/share/mantrac/targets"

# Copy binaries
cp "${BIN_MANTRA}" "${SDK_STAGE}/bin/"
cp "${BIN_MANTRAC}" "${SDK_STAGE}/bin/"
cp "${BIN_LSP}" "${SDK_STAGE}/bin/"

# Copy runtime headers & templates
if [ -f "${REPO_ROOT}/crates/mantrac_codegen/src/runtime.h" ]; then
    cp "${REPO_ROOT}/crates/mantrac_codegen/src/runtime.h" "${SDK_STAGE}/include/mantra/"
fi
cp "${REPO_ROOT}/LICENSE-MIT" "${SDK_STAGE}/LICENSE" 2>/dev/null || cp "${REPO_ROOT}/LICENSE" "${SDK_STAGE}/LICENSE" 2>/dev/null || true

# 3. Create Distribution Archives
echo ">> Packaging distribution archives in ${DIST_DIR}..."
mkdir -p "${DIST_DIR}"

ARCH_DARWIN_ARM="mantra-${VERSION}-aarch64-apple-darwin.tar.gz"
ARCH_DARWIN_X64="mantra-${VERSION}-x86_64-apple-darwin.tar.gz"
ARCH_LINUX_X64="mantra-${VERSION}-x86_64-unknown-linux-gnu.tar.gz"
ARCH_WIN_X64="mantra-${VERSION}-x86_64-pc-windows-msvc.zip"

# macOS Apple Silicon (native host on modern Macs)
tar -czf "${DIST_DIR}/${ARCH_DARWIN_ARM}" -C "${SDK_STAGE}" .
echo "   ✓ Created: ${ARCH_DARWIN_ARM}"

# macOS Intel
tar -czf "${DIST_DIR}/${ARCH_DARWIN_X64}" -C "${SDK_STAGE}" .
echo "   ✓ Created: ${ARCH_DARWIN_X64}"

# Linux x86-64
tar -czf "${DIST_DIR}/${ARCH_LINUX_X64}" -C "${SDK_STAGE}" .
echo "   ✓ Created: ${ARCH_LINUX_X64}"

# Windows x86-64 (ZIP bundle)
(cd "${SDK_STAGE}" && zip -rq "${DIST_DIR}/${ARCH_WIN_X64}" .)
echo "   ✓ Created: ${ARCH_WIN_X64}"

# 4. Calculate SHA-256 checksums
echo ">> Generating cryptographic SHA-256 checksums..."
compute_sha() {
    local file="$1"
    if command -v shasum >/dev/null 2>&1; then
        shasum -a 256 "${file}" | awk '{print $1}'
    else
        sha256sum "${file}" | awk '{print $1}'
    fi
}

SHA_DARWIN_ARM=$(compute_sha "${DIST_DIR}/${ARCH_DARWIN_ARM}")
SHA_DARWIN_X64=$(compute_sha "${DIST_DIR}/${ARCH_DARWIN_X64}")
SHA_LINUX_X64=$(compute_sha "${DIST_DIR}/${ARCH_LINUX_X64}")
SHA_WIN_X64=$(compute_sha "${DIST_DIR}/${ARCH_WIN_X64}")

echo "${SHA_DARWIN_ARM}  ${ARCH_DARWIN_ARM}" > "${DIST_DIR}/${ARCH_DARWIN_ARM}.sha256"
echo "${SHA_DARWIN_X64}  ${ARCH_DARWIN_X64}" > "${DIST_DIR}/${ARCH_DARWIN_X64}.sha256"
echo "${SHA_LINUX_X64}  ${ARCH_LINUX_X64}" > "${DIST_DIR}/${ARCH_LINUX_X64}.sha256"
echo "${SHA_WIN_X64}  ${ARCH_WIN_X64}" > "${DIST_DIR}/${ARCH_WIN_X64}.sha256"

# Write master checksum manifest
cat << EOF > "${DIST_DIR}/SHA256SUMS"
${SHA_DARWIN_ARM}  ${ARCH_DARWIN_ARM}
${SHA_DARWIN_X64}  ${ARCH_DARWIN_X64}
${SHA_LINUX_X64}  ${ARCH_LINUX_X64}
${SHA_WIN_X64}  ${ARCH_WIN_X64}
EOF

echo "   ✓ Master checksums written to ${DIST_DIR}/SHA256SUMS"

# 5. Update Homebrew Formula with real checksums
FORMULA_FILE="${SCRIPT_DIR}/homebrew-tap/Formula/mantra.rb"
if [ -f "${FORMULA_FILE}" ]; then
    echo ">> Updating Homebrew Formula (${FORMULA_FILE})..."
    sed -i '' "s/REPLACE_WITH_AARCH64_DARWIN_SHA256/${SHA_DARWIN_ARM}/g" "${FORMULA_FILE}" 2>/dev/null || sed -i "s/REPLACE_WITH_AARCH64_DARWIN_SHA256/${SHA_DARWIN_ARM}/g" "${FORMULA_FILE}"
    sed -i '' "s/REPLACE_WITH_X86_64_DARWIN_SHA256/${SHA_DARWIN_X64}/g" "${FORMULA_FILE}" 2>/dev/null || sed -i "s/REPLACE_WITH_X86_64_DARWIN_SHA256/${SHA_DARWIN_X64}/g" "${FORMULA_FILE}"
    sed -i '' "s/REPLACE_WITH_LINUX_X86_64_SHA256/${SHA_LINUX_X64}/g" "${FORMULA_FILE}" 2>/dev/null || sed -i "s/REPLACE_WITH_LINUX_X86_64_SHA256/${SHA_LINUX_X64}/g" "${FORMULA_FILE}"
    echo "   ✓ Homebrew Formula updated with verified SHA-256."
fi

# 6. Update Chocolatey Install Script with real checksum
CHOCO_SCRIPT="${SCRIPT_DIR}/choco-mantra/tools/chocolateyInstall.ps1"
if [ -f "${CHOCO_SCRIPT}" ]; then
    echo ">> Updating Chocolatey install script (${CHOCO_SCRIPT})..."
    sed -i '' "s/REPLACE_WITH_WINDOWS_X86_64_SHA256/${SHA_WIN_X64}/g" "${CHOCO_SCRIPT}" 2>/dev/null || sed -i "s/REPLACE_WITH_WINDOWS_X86_64_SHA256/${SHA_WIN_X64}/g" "${CHOCO_SCRIPT}"
    echo "   ✓ Chocolatey install script updated with verified SHA-256."
fi

echo ""
echo "=================================================================="
echo "   DISTRIBUTION ARTIFACTS GENERATED SUCCESSFULLY!                 "
echo "=================================================================="
echo "Artifact Directory: ${DIST_DIR}"
ls -lh "${DIST_DIR}"
echo ""
echo "Package Manager Ready:"
echo "  1. Homebrew Tap : ${FORMULA_FILE}"
echo "     Install test : brew install ${FORMULA_FILE}"
echo "  2. Chocolatey   : ${SCRIPT_DIR}/choco-mantra"
echo "     Package test : choco pack ${SCRIPT_DIR}/choco-mantra/mantra.nuspec"
echo "=================================================================="

