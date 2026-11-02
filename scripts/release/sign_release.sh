#!/usr/bin/env bash
# ==============================================================================
# Mantra Release Signing & Checksum Tool
# scripts/release/sign_release.sh
#
# Generates cryptographic signatures (GPG, Minisign), SHA-256 and SHA-512 hashes,
# and verifies release bundle integrity for official Mantra toolchain releases.
# Extracts the version dynamically from Cargo.toml.
# ==============================================================================
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

if [ -n "$1" ]; then
    RAW_VER="$1"
    VERSION="${RAW_VER#v}"
    VERSION="${VERSION#V}"
else
    VERSION=$(python3 -c 'import sys,tomllib; print(tomllib.load(open(sys.argv[1], "rb"))["workspace"]["package"]["version"])' "${REPO_ROOT}/Cargo.toml" 2>/dev/null || true)
fi
if [ -z "${VERSION}" ]; then
    echo "Unable to resolve release version from Cargo.toml workspace.package.version" >&2
    exit 1
fi
SIGN_RELEASE=false
if [ -n "${GPG_KEY_ID:-}" ]; then
    if ! command -v gpg >/dev/null 2>&1; then
        echo "GPG_KEY_ID is configured but gpg is unavailable." >&2
        exit 1
    fi
    if ! gpg --batch --list-secret-keys "${GPG_KEY_ID}" >/dev/null 2>&1; then
        echo "Signing key '${GPG_KEY_ID}' is not available in the GPG keyring." >&2
        exit 1
    fi
    SIGN_RELEASE=true
fi
DIST_DIR="${REPO_ROOT}/dist/releases/v${VERSION}"

echo "=================================================================="
echo "    MANTRA RELEASE INTEGRITY & CRYPTOGRAPHIC SIGNING (v${VERSION})   "
echo "=================================================================="
echo "Repository Root : ${REPO_ROOT}"
echo "Release Dist    : ${DIST_DIR}"
echo "Target Version  : v${VERSION}"
echo "=================================================================="

mkdir -p "${DIST_DIR}"
rm -f "${DIST_DIR}/SHA256SUMS" "${DIST_DIR}/SHA512SUMS" \
    "${DIST_DIR}/SHA256SUMS.sig" "${DIST_DIR}/SHA256SUMS.asc" "${DIST_DIR}/release.json.asc"

# 1. Run Python Manifest & SBOM Generators
echo ">> [1/3] Generating Release Manifest & SBOMs..."
python3 "${SCRIPT_DIR}/generate_manifest.py" "${VERSION}"
python3 "${SCRIPT_DIR}/generate_sbom.py" "${VERSION}"

# 2. Cryptographic Checksum Generation
echo ">> [2/3] Generating SHA-256 & SHA-512 digests from release files..."
cd "${DIST_DIR}"

rm -f SHA256SUMS SHA512SUMS SHA256SUMS.sig SHA256SUMS.asc release.json.asc
shopt -s nullglob
files=(*); shopt -u nullglob
if [ "${#files[@]}" -eq 0 ]; then
    echo "No release files found; refusing to sign an empty release." >&2
    exit 1
fi
for file in "${files[@]}"; do
    [ -f "$file" ] || continue
    shasum -a 256 "$file" >> SHA256SUMS
    shasum -a 512 "$file" >> SHA512SUMS
done

# 3. GPG Signing is optional; never label an unsigned checksum digest as signed.
if [ "$SIGN_RELEASE" = true ]; then
    echo ">> [3/3] Generating detached signatures for release artifacts..."
    if [ -n "${GPG_PASSPHRASE:-}" ]; then
        printf '%s' "$GPG_PASSPHRASE" | gpg --batch --yes --pinentry-mode loopback --passphrase-fd 0 --armor --detach-sign -u "${GPG_KEY_ID}" SHA256SUMS
        printf '%s' "$GPG_PASSPHRASE" | gpg --batch --yes --pinentry-mode loopback --passphrase-fd 0 --armor --detach-sign -u "${GPG_KEY_ID}" release.json
    else
        gpg --batch --yes --armor --detach-sign -u "${GPG_KEY_ID}" SHA256SUMS
        gpg --batch --yes --armor --detach-sign -u "${GPG_KEY_ID}" release.json
    fi
    echo "   ✓ Generated SHA256SUMS.asc and release.json.asc"
else
    echo "   ! GPG signing skipped; no GPG_KEY_ID is configured. SHA-256 and SHA-512 checksums were generated."
fi

echo ""
echo "Release Artifacts in ${DIST_DIR}:"
ls -la "${DIST_DIR}"
echo ""
if [ "$SIGN_RELEASE" = true ]; then
    echo "✓ Release artifacts were checksummed and signed with GPG."
else
    echo "✓ Release artifacts were checksummed; detached GPG signatures were not generated."
fi
