#!/bin/bash
set -e

echo "======================================================="
echo "   MANTRA PROGRAMMING LANGUAGE - CROSS-PLATFORM SETUP   "
echo "   Steward: Solotron Technologies Private Limited      "
echo "======================================================="

OS="$(uname -s)"
ARCH="$(uname -m)"

echo "Detected Platform: $OS ($ARCH)"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

case "$OS" in
    Darwin*)
        echo "Launching macOS installation..."
        "$SCRIPT_DIR/macos/install.sh"
        ;;
    Linux*)
        echo "Launching Linux installation..."
        "$SCRIPT_DIR/linux/install.sh"
        ;;
    MINGW*|MSYS*|CYGWIN*)
        echo "Launching Windows installation..."
        cmd.exe /c "$SCRIPT_DIR\\windows\\install.bat"
        ;;
    *)
        echo "Unsupported platform: $OS"
        echo "Please refer to installer/README.md for manual setup."
        exit 1
        ;;
esac
