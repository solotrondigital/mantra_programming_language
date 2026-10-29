#!/bin/bash
set -e

echo "=========================================================="
echo "   MANTRA PROGRAMMING LANGUAGE - LINUX INSTALLER          "
echo "   Steward: Solotron Technologies Private Limited         "
echo "=========================================================="

PREFIX="${PREFIX:-/usr/local}"
BINDIR="$PREFIX/bin"
APPLICATIONS_DIR="/usr/share/applications"
ICONS_DIR="/usr/share/icons/hicolor/512x512/apps"

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

SUDO=""
if [ "$(id -u)" -ne 0 ]; then
    if [ "$PREFIX" = "/usr/local" ] || [ "$PREFIX" = "/usr" ]; then
        SUDO="sudo"
    fi
fi

echo "Installing Mantra binaries to $BINDIR..."
$SUDO mkdir -p "$BINDIR"
$SUDO cp "$MANTRAC_SRC" "$BINDIR/mantrac"
$SUDO cp "$MANTRA_SRC" "$BINDIR/mantra"
$SUDO chmod 755 "$BINDIR/mantrac" "$BINDIR/mantra"

echo "Installing desktop entry and application logo..."
LOGO_SRC="$SCRIPT_DIR/assets/images/mantra_logo.png"
if [ -f "$LOGO_SRC" ] && [ -d "$ICONS_DIR" ]; then
    $SUDO cp "$LOGO_SRC" "$ICONS_DIR/mantra.png" 2>/dev/null || true
fi

if [ -f "$SCRIPT_DIR/mantra.desktop" ] && [ -d "$APPLICATIONS_DIR" ]; then
    $SUDO cp "$SCRIPT_DIR/mantra.desktop" "$APPLICATIONS_DIR/" 2>/dev/null || true
    $SUDO update-desktop-database 2>/dev/null || true
fi

SAMPLE_DIR="$HOME/.mantra/examples"
mkdir -p "$SAMPLE_DIR"
if [ -d "$SCRIPT_DIR/examples" ]; then
    cp -r "$SCRIPT_DIR/examples/"* "$SAMPLE_DIR/" 2>/dev/null || true
fi

echo ""
echo "Installed components:"
echo "  - Compiler:      $BINDIR/mantrac"
echo "  - MDK CLI:       $BINDIR/mantra"
echo "  - Examples:      $SAMPLE_DIR"
echo ""
echo "Verifying installation:"
"$BINDIR/mantra" --version || true
echo ""
echo "Mantra successfully installed! Run 'mantra --help' to begin."
