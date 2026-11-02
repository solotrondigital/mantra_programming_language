#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_SOURCE="$SCRIPT_DIR/bin"
if [[ ! -x "$BIN_SOURCE/mantra" || ! -x "$BIN_SOURCE/mantrac" ]]; then
    echo "Mantra binaries were not found in $BIN_SOURCE." >&2
    exit 1
fi

VERSION="$("$BIN_SOURCE/mantra" --version | sed -n '1s/.*MDK) v\([^ ]*\).*/\1/p')"
if [[ -z "$VERSION" || "$VERSION" == *[!A-Za-z0-9.+-]* ]]; then
    echo "Could not determine a valid Mantra version from the bundled CLI." >&2
    exit 1
fi

MANTRA_HOME="${MANTRA_HOME:-$HOME/.mantra}"
mkdir -p "$MANTRA_HOME/versions"
LEGACY_MANTRA="$(command -v mantra 2>/dev/null || true)"
if [[ -n "$LEGACY_MANTRA" && "$LEGACY_MANTRA" != "$MANTRA_HOME/bin/mantra" ]]; then
    LEGACY_VERSION="$("$LEGACY_MANTRA" --version 2>/dev/null | sed -n '1s/.*MDK) v\([^ ]*\).*/\1/p' || true)"
    if [[ -n "$LEGACY_VERSION" && "$LEGACY_VERSION" != "$VERSION" && "$LEGACY_VERSION" != *[!A-Za-z0-9.+-]* ]]; then
        LEGACY_BIN="$MANTRA_HOME/versions/$LEGACY_VERSION/bin"
        mkdir -p "$LEGACY_BIN"
        for name in mantra mantrac mantra-lsp; do
            legacy_path="$(command -v "$name" 2>/dev/null || true)"
            if [[ -n "$legacy_path" && -x "$legacy_path" ]]; then cp -n "$legacy_path" "$LEGACY_BIN/$name"; fi
        done
        echo "Preserved previously installed Mantra v$LEGACY_VERSION for version-aware uninstall."
    fi
fi
VERSION_BIN="$MANTRA_HOME/versions/$VERSION/bin"
ACTIVE_BIN="$MANTRA_HOME/bin"
mkdir -p "$VERSION_BIN" "$ACTIVE_BIN" "$MANTRA_HOME/examples" "$MANTRA_HOME/assets"
cp -f "$BIN_SOURCE/"* "$VERSION_BIN/"
chmod 755 "$VERSION_BIN/"*

for name in mantra mantrac mantra-lsp; do
    if [[ -x "$VERSION_BIN/$name" ]]; then
        ln -sfn "$VERSION_BIN/$name" "$ACTIVE_BIN/$name"
    else
        rm -f "$ACTIVE_BIN/$name"
    fi
done

if [[ -d "$SCRIPT_DIR/examples" ]]; then
    cp -R "$SCRIPT_DIR/examples/." "$MANTRA_HOME/examples/"
fi
if [[ -d "$SCRIPT_DIR/assets" ]]; then
    cp -R "$SCRIPT_DIR/assets/." "$MANTRA_HOME/assets/"
fi

printf -v QUOTED_MANTRA_HOME '%q' "$MANTRA_HOME"
printf -v PATH_LINE 'export MANTRA_HOME=%s\ncase ":${PATH}:" in\n    *":${MANTRA_HOME}/bin:"*) ;;\n    *) export PATH="${MANTRA_HOME}/bin:${PATH}" ;;\nesac' "$QUOTED_MANTRA_HOME"
for profile in "$HOME/.profile" "$HOME/.bash_profile" "$HOME/.zprofile"; do
    if [[ ! -f "$profile" ]] || ! grep -Fq 'export MANTRA_HOME=' "$profile"; then
        printf '\n%s\n' "$PATH_LINE" >> "$profile"
    fi
done

echo "Installed Mantra v$VERSION with mantra, mantrac, and mantra-lsp."
echo "Version files: $MANTRA_HOME/versions/$VERSION"
echo "Active commands: $ACTIVE_BIN"
echo "Open a new terminal or run: export PATH=\"$ACTIVE_BIN:\$PATH\""
"$ACTIVE_BIN/mantra" --version
