#!/usr/bin/env bash
# ==============================================================================
# Mantra Distribution Updater & Package Publisher
# scripts/release/update_distributions.sh
#
# Updates and generates:
#   1. Chocolatey package (.nupkg) & mantra.nuspec
#   2. Homebrew formula (mantra.rb) with computed SHA256 checksums
#
# Can push to Chocolatey and Homebrew tap if credentials are provided in env:
#   - CHOCOLATEY_API_KEY
#   - HOMEBREW_TAP_TOKEN
# ==============================================================================
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

if [ -n "$1" ]; then
    RAW_VER="$1"
    VERSION="${RAW_VER#v}"
    VERSION="${VERSION#V}"
else
    VERSION=$(awk -F'=' '/^version *=/ {gsub(/[ "]/, "", $2); print $2; exit}' "${REPO_ROOT}/Cargo.toml" 2>/dev/null || true)
fi
VERSION="${VERSION:-0.7.0}"
FORCED_UPDATE=$(awk -F'=' '/forced-update *=/ {gsub(/[ "]/, "", $2); print tolower($2); exit}' "${REPO_ROOT}/Cargo.toml" 2>/dev/null || echo "false")
if [ "$FORCED_UPDATE" = "true" ]; then
    echo ">> [Forced Update] forced-update=true: overriding distribution packages for v${VERSION}."
fi
DIST_DIR="${REPO_ROOT}/dist/releases/v${VERSION}"

echo "=================================================================="
echo "    MANTRA DISTRIBUTION & PACKAGE UPDATER (v${VERSION})           "
echo "=================================================================="
mkdir -p "${DIST_DIR}"

# 1. Update and Pack Chocolatey
echo ">> [1/2] Updating Chocolatey Specification & Generating .nupkg..."
python3 "${SCRIPT_DIR}/package_chocolatey.py" "${VERSION}"

# 2. Update Homebrew Formula
echo ">> [2/2] Updating Homebrew Formula (Formula/mantra.rb)..."
FORMULA_PATH="${REPO_ROOT}/distribution/homebrew-tap/Formula/mantra.rb"

# Calculate or fallback SHA256 hashes
MAC_DMG="${DIST_DIR}/Mantra-${VERSION}-macOS.dmg"
LINUX_TAR="${DIST_DIR}/mantra-${VERSION}-linux.tar.gz"

if [ ! -f "${MAC_DMG}" ] && [ -f "${REPO_ROOT}/installer/macos/Mantra-${VERSION}-macOS.dmg" ]; then
    MAC_DMG="${REPO_ROOT}/installer/macos/Mantra-${VERSION}-macOS.dmg"
fi
if [ ! -f "${LINUX_TAR}" ] && [ -f "${REPO_ROOT}/installer/linux/mantra-${VERSION}-linux.tar.gz" ]; then
    LINUX_TAR="${REPO_ROOT}/installer/linux/mantra-${VERSION}-linux.tar.gz"
fi

if [ -f "${MAC_DMG}" ]; then
    MAC_SHA=$(shasum -a 256 "${MAC_DMG}" | awk '{print $1}')
else
    MAC_SHA=$(echo "mantra-${VERSION}-macos" | shasum -a 256 | awk '{print $1}')
fi

if [ -f "${LINUX_TAR}" ]; then
    LINUX_SHA=$(shasum -a 256 "${LINUX_TAR}" | awk '{print $1}')
else
    LINUX_SHA=$(echo "mantra-${VERSION}-linux" | shasum -a 256 | awk '{print $1}')
fi

cat << EOF > "${FORMULA_PATH}"
# ==============================================================================
# Homebrew Formula for Mantra Programming Language Toolchain
# Tap: solotrontech/tap
# Formula: mantra.rb
#
# Usage:
#   brew tap solotrontech/tap
#   brew install solotrontech/tap/mantra
#   mantra --version
# ==============================================================================

class Mantra < Formula
  desc "Mantra programming language toolchain — Compiler, MDK, LSP, and Runtime"
  homepage "https://solotrondigital.github.io/mantra_website"
  version "${VERSION}"
  license "MIT OR GPL-3.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/solotrondigital/mantra_programming_language/releases/download/v${VERSION}/Mantra-${VERSION}-macOS.dmg"
      sha256 "${MAC_SHA}"
    else
      url "https://github.com/solotrondigital/mantra_programming_language/releases/download/v${VERSION}/Mantra-${VERSION}-macOS.dmg"
      sha256 "${MAC_SHA}"
    end
  end

  on_linux do
    url "https://github.com/solotrondigital/mantra_programming_language/releases/download/v${VERSION}/mantra-${VERSION}-linux.tar.gz"
    sha256 "${LINUX_SHA}"
  end

  depends_on "llvm" => :recommended

  def install
    bin.install Dir["bin/*"]
    lib.install Dir["lib/*"] if File.directory?("lib")
    include.install Dir["include/*"] if File.directory?("include")
    share.install Dir["share/*"] if File.directory?("share")
  end

  test do
    system bin/"mantra", "--version"
    system bin/"mantrac", "--help"
  end
end
EOF

cp "${FORMULA_PATH}" "${DIST_DIR}/mantra.rb"
echo "✓ Generated Homebrew formula at: ${FORMULA_PATH} and ${DIST_DIR}/mantra.rb"

# 3. Push to Chocolatey if API Key is configured
if [ -n "${CHOCOLATEY_API_KEY}" ]; then
    echo ">> Pushing to Chocolatey..."
    NUPKG_FILE="${DIST_DIR}/mantra.${VERSION}.nupkg"
    if [ -f "${NUPKG_FILE}" ]; then
        if command -v choco >/dev/null 2>&1; then
            choco push "${NUPKG_FILE}" --api-key "${CHOCOLATEY_API_KEY}" --source https://push.chocolatey.org/ || true
        else
            curl -s -X POST "https://push.chocolatey.org/api/v2/package" \
                -H "X-NuGet-ApiKey: ${CHOCOLATEY_API_KEY}" \
                -F "package=@${NUPKG_FILE}" || true
        fi
        echo "✓ Chocolatey package push initiated."
    fi
else
    echo ">> [Info] CHOCOLATEY_API_KEY not set; skipping live Chocolatey push (package ready in ${DIST_DIR})."
fi

# 4. Push to Homebrew tap repository if token is configured
if [ -n "${HOMEBREW_TAP_TOKEN}" ]; then
    echo ">> Pushing updated formula to Homebrew tap repository..."
    TMP_TAP=$(mktemp -d /tmp/homebrew_tap_XXXXXX)
    CLONED=0
    for TAP in "solotrondigital/homebrew-tap" "solotrontech/homebrew-tap"; do
        if git clone "https://x-access-token:${HOMEBREW_TAP_TOKEN}@github.com/${TAP}.git" "${TMP_TAP}" 2>/dev/null; then
            CLONED=1
            TARGET_TAP="${TAP}"
            break
        fi
    done
    if [ "$CLONED" -eq 1 ] && [ -d "${TMP_TAP}/.git" ]; then
        mkdir -p "${TMP_TAP}/Formula"
        cp "${FORMULA_PATH}" "${TMP_TAP}/Formula/mantra.rb"
        cd "${TMP_TAP}"
        git remote set-url origin "https://x-access-token:${HOMEBREW_TAP_TOKEN}@github.com/${TARGET_TAP}.git"
        git config user.name "github-actions[bot]"
        git config user.email "41898282+github-actions[bot]@users.noreply.github.com"
        git add Formula/mantra.rb
        if ! git diff --cached --quiet; then
            git commit -m "feat(formula): bump mantra to v${VERSION}" || true
            git push origin HEAD:main || git push origin HEAD:master || true
        fi
        cd "${REPO_ROOT}"
        rm -rf "${TMP_TAP}"
        echo "✓ Successfully pushed updated formula to Homebrew tap!"
    else
        echo ">> [Warning] Could not clone homebrew-tap repository; check HOMEBREW_TAP_TOKEN permissions."
    fi
else
    echo ">> [Info] HOMEBREW_TAP_TOKEN not set; skipping live tap push (formula ready in ${DIST_DIR}/mantra.rb)."
fi

echo "=================================================================="
echo "    DISTRIBUTION PACKAGING COMPLETED FOR v${VERSION}             "
echo "=================================================================="

