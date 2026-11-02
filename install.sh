#!/usr/bin/env bash
set -euo pipefail

REPOSITORY="solotrondigital/mantra_programming_language"
VERSION=""
if [[ $# -eq 2 && "$1" == "--version" ]]; then
    VERSION="${2#v}"
elif [[ $# -ne 0 ]]; then
    echo "Usage: ./install.sh [--version <version>]" >&2
    exit 2
fi

if [[ -z "$VERSION" ]]; then
    RELEASE_JSON="$(curl -fsSL --retry 3 -H 'User-Agent: mantra-installer' "https://api.github.com/repos/$REPOSITORY/releases/latest")"
    VERSION="$(python3 -c 'import json,sys; print(json.load(sys.stdin)["tag_name"].lstrip("vV"))' <<< "$RELEASE_JSON")"
fi
if [[ ! "$VERSION" =~ ^[A-Za-z0-9.+-]+$ ]]; then
    echo "Invalid Mantra version: $VERSION" >&2
    exit 2
fi

TEMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/mantra-install.XXXXXX")"
cleanup() {
    if [[ -n "${MOUNT_DIR:-}" ]] && mount | grep -Fq "on $MOUNT_DIR "; then
        hdiutil detach "$MOUNT_DIR" -quiet || true
    fi
    rm -rf "$TEMP_DIR"
}
trap cleanup EXIT

case "$(uname -s)" in
    Darwin*)
        ASSET="Mantra-${VERSION}-macOS.dmg"
        IMAGE="$TEMP_DIR/$ASSET"
        curl -fL --retry 3 --show-error -H 'User-Agent: mantra-installer' \
            "https://github.com/$REPOSITORY/releases/download/v$VERSION/$ASSET" -o "$IMAGE"
        MOUNT_DIR="$TEMP_DIR/mounted"
        mkdir -p "$MOUNT_DIR"
        hdiutil attach -readonly -nobrowse -mountpoint "$MOUNT_DIR" "$IMAGE" >/dev/null
        INSTALLER="$MOUNT_DIR/install.sh"
        [[ -f "$INSTALLER" ]] || { echo "macOS release has no install.sh" >&2; exit 1; }
        bash "$INSTALLER"
        ;;
    Linux*)
        ASSET="mantra-${VERSION}-linux.tar.gz"
        ARCHIVE="$TEMP_DIR/$ASSET"
        BUNDLE="$TEMP_DIR/bundle"
        curl -fL --retry 3 --show-error -H 'User-Agent: mantra-installer' \
            "https://github.com/$REPOSITORY/releases/download/v$VERSION/$ASSET" -o "$ARCHIVE"
        mkdir -p "$BUNDLE"
        tar -xzf "$ARCHIVE" -C "$BUNDLE"
        INSTALLER="$BUNDLE/install.sh"
        [[ -f "$INSTALLER" ]] || { echo "Linux release has no install.sh" >&2; exit 1; }
        bash "$INSTALLER"
        ;;
    *)
        echo "Unsupported OS. On Windows, run: .\\install.ps1 [-Version $VERSION]" >&2
        exit 1
        ;;
esac

echo "Installed Mantra v$VERSION. Open a new terminal if the command is not on PATH."
