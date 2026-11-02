#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 0 && ( $# -ne 2 || "$1" != "--version" ) ]]; then
    echo "Usage: ./uninstall.sh [--version <version>]" >&2
    exit 2
fi

if [[ $# -eq 2 ]]; then
    # Version-aware removal needs the current CLI; use a matching installed
    # binary, and direct users to install.sh if the CLI predates this command.
    MANTRA_BIN="${MANTRA_HOME:-$HOME/.mantra}/bin/mantra"
    if [[ ! -x "$MANTRA_BIN" ]]; then
        echo "No version-aware Mantra CLI is installed. Use ./uninstall.sh to remove all versions." >&2
        exit 1
    fi
    exec "$MANTRA_BIN" uninstall "$@"
fi

remove_path() {
    local path="$1"
    [[ -e "$path" || -L "$path" ]] || return 0
    if rm -rf "$path" 2>/dev/null; then
        return 0
    fi
    if command -v sudo >/dev/null 2>&1; then
        sudo rm -rf "$path"
    else
        echo "Cannot remove $path (permission denied; sudo is unavailable)." >&2
        return 1
    fi
}

if command -v brew >/dev/null 2>&1 && brew list --formula mantra >/dev/null 2>&1; then
    brew uninstall --formula mantra
fi

ROOTS=("${MANTRA_HOME:-$HOME/.mantra}")
[[ "${ROOTS[0]}" == "$HOME/.mantra" ]] || ROOTS+=("$HOME/.mantra")
for root in "${ROOTS[@]}"; do remove_path "$root"; done

for directory in /usr/local/bin /usr/bin /opt/homebrew/bin /home/linuxbrew/.linuxbrew/bin; do
    for name in mantra mantrac mantra-lsp; do remove_path "$directory/$name"; done
done
for path in /usr/share/applications/mantra.desktop /usr/share/icons/hicolor/512x512/apps/mantra.png; do
    remove_path "$path"
done

for profile in "$HOME/.profile" "$HOME/.bash_profile" "$HOME/.bashrc" "$HOME/.zprofile" "$HOME/.zshrc"; do
    [[ -f "$profile" ]] || continue
    TEMP_PROFILE="$profile.mantra-cleanup.$$"
    awk '!/MANTRA_HOME|\.mantra\/bin/' "$profile" > "$TEMP_PROFILE"
    if ! cmp -s "$profile" "$TEMP_PROFILE"; then
        cat "$TEMP_PROFILE" > "$profile"
    fi
    rm -f "$TEMP_PROFILE"
done

echo "Mantra and its managed versions have been removed. Open a new terminal for PATH changes to take effect."
