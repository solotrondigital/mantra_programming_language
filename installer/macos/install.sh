#!/bin/bash
set -e

echo "=========================================================="
echo "   Installing Mantra Programming Language (macOS)         "
echo "   Steward: Solotron Technologies Private Limited         "
echo "=========================================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MANTRAC_SRC=""
MANTRA_SRC=""

CANDIDATES=(
    "$SCRIPT_DIR/bin"
    "$SCRIPT_DIR/../bin"
    "$SCRIPT_DIR/../../target/release"
)

for dir in "${CANDIDATES[@]}"; do
    if [ -f "$dir/mantrac" ] && [ -f "$dir/mantra" ]; then
        MANTRAC_SRC="$dir/mantrac"
        MANTRA_SRC="$dir/mantra"
        break
    fi
done

if [ -z "$MANTRAC_SRC" ] || [ -z "$MANTRA_SRC" ]; then
    echo "Error: Could not locate Mantra binaries in $SCRIPT_DIR/bin"
    exit 1
fi

TARGET_DIR="${TARGET_DIR:-/usr/local/bin}"
if [ ! -d "$TARGET_DIR" ]; then
    if [ ! -w "$(dirname "$TARGET_DIR")" ]; then
        sudo mkdir -p "$TARGET_DIR"
    else
        mkdir -p "$TARGET_DIR"
    fi
fi

SUDO=""
if [ ! -w "$TARGET_DIR" ]; then
    echo "Notice: Administrator privileges required to install into $TARGET_DIR"
    SUDO="sudo"
fi

echo "Installing binaries into $TARGET_DIR..."
$SUDO cp "$MANTRAC_SRC" "$TARGET_DIR/mantrac"
$SUDO cp "$MANTRA_SRC" "$TARGET_DIR/mantra"
$SUDO chmod 755 "$TARGET_DIR/mantrac" "$TARGET_DIR/mantra"

echo ""
echo "Installed successfully:"
echo "  - Compiler: $TARGET_DIR/mantrac"
echo "  - MDK CLI:  $TARGET_DIR/mantra"
echo ""

if ! echo "$PATH" | tr ':' '\n' | grep -qx "$TARGET_DIR"; then
    echo "Note: $TARGET_DIR is not currently in your PATH."
    echo "Add it by adding this line to ~/.zshrc or ~/.bash_profile:"
    echo "  export PATH=\"$TARGET_DIR:\$PATH\""
    echo ""
fi

echo "Verifying installation:"
"$TARGET_DIR/mantra" --version || true
echo ""
echo "Installation complete! Run 'mantra --help' to begin."
