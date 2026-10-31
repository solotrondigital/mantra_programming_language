#!/usr/bin/env bash
# ==============================================================================
# Mantra Programming Language — Universal Installer Generator
# Developed & Maintained by Solotron Technologies Private Limited
#
# Generates versioned installers (macOS .dmg, Linux .tar.gz, Linux self-extracting .sh,
# Windows portable bundle .zip/.bat/.ps1) and installs them to:
#   1. /Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/installer
#   2. /Users/user/Documents/workspace/Mantra-Development/mantra_website/mantra_web/installer
#
# Usage:
#   ./create_installers.sh [VERSION] [--rebuild]
#   Example: ./create_installers.sh 0.2.0
# ==============================================================================
set -e

# Detect script directory and resolve workspace roots
SCRIPT_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [ -d "$SCRIPT_PATH/mantra_programming_language" ]; then
    WORKSPACE_ROOT="$SCRIPT_PATH"
elif [ -d "$SCRIPT_PATH/../mantra_programming_language" ]; then
    WORKSPACE_ROOT="$(cd "$SCRIPT_PATH/.." && pwd)"
elif [ -d "$SCRIPT_PATH/../../mantra_programming_language" ]; then
    WORKSPACE_ROOT="$(cd "$SCRIPT_PATH/../.." && pwd)"
else
    WORKSPACE_ROOT="/Users/user/Documents/workspace/Mantra-Development"
fi

PKG_DIR="$WORKSPACE_ROOT/mantra_programming_language"
WEB_DIR="$WORKSPACE_ROOT/mantra_website/mantra_web"
TARGET_RELEASE="$PKG_DIR/target/release"

DEST_PKG_INSTALLER="$PKG_DIR/installer"
DEST_PKG_DIST="$PKG_DIR/dist"

# Determine Version:
# 1. From CLI argument ($1 if not starting with --)
# 2. Or from Cargo.toml in crates/mantrac or workspace
# 3. Default to 0.2.0
VERSION_ARG=""
REBUILD=false

for arg in "$@"; do
    if [ "$arg" = "--rebuild" ]; then
        REBUILD=true
    elif [ -z "$VERSION_ARG" ] && [[ "$arg" != --* ]]; then
        VERSION_ARG="$arg"
    fi
done

if [ -f "$PKG_DIR/scripts/release/sync_versions.py" ]; then
    python3 "$PKG_DIR/scripts/release/sync_versions.py" || true
fi

if [ -z "$VERSION_ARG" ]; then
    if [ -f "$PKG_DIR/Cargo.toml" ]; then
        VERSION_ARG=$(awk -F'=' '/^version *=/ {gsub(/[ "]/, "", $2); print $2; exit}' "$PKG_DIR/Cargo.toml")
    fi
    if [ -z "$VERSION_ARG" ] && [ -f "$PKG_DIR/crates/mantrac/Cargo.toml" ]; then
        VERSION_ARG=$(awk -F'=' '/^version *=/ {gsub(/[ "]/, "", $2); print $2; exit}' "$PKG_DIR/crates/mantrac/Cargo.toml")
    fi
fi
VERSION_ARG="${VERSION_ARG:-0.7.0}"
VERSION="${VERSION_ARG#v}"
VERSION="${VERSION#V}"

FORCED_UPDATE=$(awk -F'=' '/forced-update *=/ {gsub(/[ "]/, "", $2); print tolower($2); exit}' "$PKG_DIR/Cargo.toml" 2>/dev/null || echo "false")

DEST_RELEASE_DIR="$PKG_DIR/dist/releases/v${VERSION}"
if [ "$FORCED_UPDATE" != "true" ] && [ -d "$DEST_RELEASE_DIR" ] && [ -n "$(ls -A "$DEST_RELEASE_DIR" 2>/dev/null)" ]; then
    echo ">> [Skip] Distribution packages for v${VERSION} already exist in $DEST_RELEASE_DIR and forced-update is false. Skipping installer build."
    exit 0
fi

if [ "$FORCED_UPDATE" = "true" ]; then
    echo ">> [Forced Update] forced-update=true: overriding all distribution packages for v${VERSION} irrespective of versions."
    rm -rf "$DEST_RELEASE_DIR"
    mkdir -p "$DEST_RELEASE_DIR"
fi

echo "=================================================================="
echo "   MANTRA PROGRAMMING LANGUAGE — INSTALLER GENERATOR (v${VERSION})"
echo "   Steward: Solotron Technologies Private Limited"
echo "=================================================================="
echo "Workspace Root : $WORKSPACE_ROOT"
echo "Package Dir    : $PKG_DIR"
echo "Web Dir        : $WEB_DIR"
echo "Target Version : v${VERSION}"
echo "=================================================================="

# Check / Build binaries
if [ ! -f "$TARGET_RELEASE/mantra" ] || [ ! -f "$TARGET_RELEASE/mantrac" ] || [ "$REBUILD" = true ]; then
    echo ">> Compiling release binaries with Cargo..."
    if command -v cargo >/dev/null 2>&1; then
        (cd "$PKG_DIR" && cargo build --release)
    else
        echo "Error: cargo not found on PATH. Please ensure Rust is installed."
        exit 1
    fi
fi

if [ ! -f "$TARGET_RELEASE/mantra" ] || [ ! -f "$TARGET_RELEASE/mantrac" ]; then
    echo "Error: Binaries not found in $TARGET_RELEASE."
    exit 1
fi

echo ">> Verified binaries in $TARGET_RELEASE:"
ls -lh "$TARGET_RELEASE/mantra" "$TARGET_RELEASE/mantrac"

# Create a temporary staging area
TMP_STAGE="$(mktemp -d /tmp/mantra_build_installer_XXXXXX)"
trap 'rm -rf "$TMP_STAGE"' EXIT

STAGE_BIN="$TMP_STAGE/bin"
STAGE_EXAMPLES="$TMP_STAGE/examples"
STAGE_ASSETS="$TMP_STAGE/assets/images"

mkdir -p "$STAGE_BIN" "$STAGE_EXAMPLES" "$STAGE_ASSETS"

# Copy binaries
cp "$TARGET_RELEASE/mantra" "$STAGE_BIN/mantra"
cp "$TARGET_RELEASE/mantrac" "$STAGE_BIN/mantrac"
chmod 755 "$STAGE_BIN/mantra" "$STAGE_BIN/mantrac"
if [ -f "$TARGET_RELEASE/mantra-lsp" ]; then
    cp "$TARGET_RELEASE/mantra-lsp" "$STAGE_BIN/mantra-lsp"
    chmod 755 "$STAGE_BIN/mantra-lsp"
fi

# Copy latest examples if available
if [ -d "$PKG_DIR/examples" ]; then
    cp -r "$PKG_DIR/examples/"* "$STAGE_EXAMPLES/" 2>/dev/null || true
elif [ -d "$WEB_DIR/examples" ]; then
    cp -r "$WEB_DIR/examples/"* "$STAGE_EXAMPLES/" 2>/dev/null || true
fi

# Copy logo asset
if [ -f "$WEB_DIR/assets/images/mantra_logo.png" ]; then
    cp "$WEB_DIR/assets/images/mantra_logo.png" "$STAGE_ASSETS/mantra_logo.png"
elif [ -f "$PKG_DIR/installer/assets/images/mantra_logo.png" ]; then
    cp "$PKG_DIR/installer/assets/images/mantra_logo.png" "$STAGE_ASSETS/mantra_logo.png"
fi

# Staging directories for platform packages
MACOS_DIR="$TMP_STAGE/macos_stage"
LINUX_DIR="$TMP_STAGE/linux_stage"
WINDOWS_DIR="$TMP_STAGE/windows_stage"
mkdir -p "$MACOS_DIR" "$LINUX_DIR" "$WINDOWS_DIR"

# ------------------------------------------------------------------------------
# 1. GENERATE macOS INSTALLER (DMG + Terminal script)
# ------------------------------------------------------------------------------
echo ">> Generating macOS Installer Packages (v${VERSION})..."
mkdir -p "$MACOS_DIR/bin" "$MACOS_DIR/examples"
cp "$STAGE_BIN/"* "$MACOS_DIR/bin/"
cp -r "$STAGE_EXAMPLES/"* "$MACOS_DIR/examples/" 2>/dev/null || true

cat << 'EOF' > "$MACOS_DIR/install.sh"
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
EOF
chmod 755 "$MACOS_DIR/install.sh"

cat << EOF > "$MACOS_DIR/README.md"
# Mantra Programming Language (v${VERSION}) — macOS Installer

Developed and maintained by **Solotron Technologies Private Limited**.
Open source dual-licensed under MIT and GNU GPL v3.0.

### Installation:
Run the install script in terminal:
\`\`\`bash
./install.sh
\`\`\`
Or copy \`bin/mantrac\` and \`bin/mantra\` to \`/usr/local/bin/\`.

Verify:
\`\`\`bash
mantra --version
mantrac --help
\`\`\`
EOF

DMG_OUTPUT="$TMP_STAGE/Mantra-${VERSION}-macOS.dmg"
if command -v hdiutil >/dev/null 2>&1; then
    hdiutil create -volname "Mantra ${VERSION} (macOS)" -srcfolder "$MACOS_DIR" -ov -format UDZO "$DMG_OUTPUT" >/dev/null
    echo "   ✓ Created Apple Disk Image: Mantra-${VERSION}-macOS.dmg"
else
    tar -czf "$TMP_STAGE/Mantra-${VERSION}-macOS.tar.gz" -C "$MACOS_DIR" .
    echo "   ✓ Created macOS archive: Mantra-${VERSION}-macOS.tar.gz"
fi

# ------------------------------------------------------------------------------
# 2. GENERATE LINUX INSTALLER (TAR.GZ + Self-Extracting .sh)
# ------------------------------------------------------------------------------
echo ">> Generating Linux Installer Packages (v${VERSION})..."
mkdir -p "$LINUX_DIR/bin" "$LINUX_DIR/examples" "$LINUX_DIR/assets/images"
cp "$STAGE_BIN/"* "$LINUX_DIR/bin/"
cp -r "$STAGE_EXAMPLES/"* "$LINUX_DIR/examples/" 2>/dev/null || true
cp "$STAGE_ASSETS/mantra_logo.png" "$LINUX_DIR/assets/images/" 2>/dev/null || true

cat << 'EOF' > "$LINUX_DIR/mantra.desktop"
[Desktop Entry]
Version=1.0
Type=Application
Name=Mantra Programming Language
GenericName=Programming Language Compiler and MDK
Comment=Modern Object-Oriented Mantra Programming Language Toolchain
Exec=mantra run
Icon=mantra
Terminal=true
Categories=Development;Building;IDE;
Keywords=mantra;compiler;programming;oop;mdk;
EOF

cat << 'EOF' > "$LINUX_DIR/install.sh"
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
EOF
chmod 755 "$LINUX_DIR/install.sh"

cat << EOF > "$LINUX_DIR/README.md"
# Mantra Programming Language (v${VERSION}) — Linux Installer

Developed and maintained by **Solotron Technologies Private Limited**.
Dual-licensed under MIT and GNU GPL v3.0.

### Installation:
\`\`\`bash
sudo ./install.sh
\`\`\`
EOF

TAR_OUTPUT="$TMP_STAGE/mantra-${VERSION}-linux.tar.gz"
tar -czf "$TAR_OUTPUT" -C "$LINUX_DIR" .
echo "   ✓ Created Linux tarball: mantra-${VERSION}-linux.tar.gz"

RUN_OUTPUT="$TMP_STAGE/mantra-installer-linux.sh"
cat << EOF > "$RUN_OUTPUT"
#!/bin/bash
set -e

echo "Extracting Mantra Programming Language Installer (v${VERSION})..."
TMPDIR=\$(mktemp -d /tmp/mantra_install_XXXXXX)
trap 'rm -rf "\$TMPDIR"' EXIT

ARCHIVE_LINE=\$(awk '/^__ARCHIVE_BELOW__/ {print NR + 1; exit 0; }' "\$0")
tail -n +"\$ARCHIVE_LINE" "\$0" | tar -xz -C "\$TMPDIR"

cd "\$TMPDIR"
chmod +x install.sh
./install.sh "\$@"

exit 0
__ARCHIVE_BELOW__
EOF
cat "$TAR_OUTPUT" >> "$RUN_OUTPUT"
chmod +x "$RUN_OUTPUT"
echo "   ✓ Created Linux self-extracting installer: mantra-installer-linux.sh"

# ------------------------------------------------------------------------------
# 3. GENERATE WINDOWS INSTALLER (BAT + PowerShell + ZIP)
# ------------------------------------------------------------------------------
echo ">> Generating Windows Installer Packages (v${VERSION})..."
mkdir -p "$WINDOWS_DIR/bin" "$WINDOWS_DIR/examples" "$WINDOWS_DIR/assets"
cp "$STAGE_BIN/"* "$WINDOWS_DIR/bin/"
cp -r "$STAGE_EXAMPLES/"* "$WINDOWS_DIR/examples/" 2>/dev/null || true
cp "$STAGE_ASSETS/mantra_logo.png" "$WINDOWS_DIR/assets/" 2>/dev/null || true

cat << 'EOF' > "$WINDOWS_DIR/install.bat"
@echo off
setlocal enabledelayedexpansion

echo =======================================================
echo     MANTRA PROGRAMMING LANGUAGE - WINDOWS INSTALLER     
echo     Steward: Solotron Technologies Private Limited      
echo =======================================================
echo.

set "DEFAULT_DIR=%LOCALAPPDATA%\Mantra"
set "INSTALL_DIR=%DEFAULT_DIR%"

echo Installing Mantra to: %INSTALL_DIR%
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"
if not exist "%INSTALL_DIR%\bin" mkdir "%INSTALL_DIR%\bin"
if not exist "%INSTALL_DIR%\assets" mkdir "%INSTALL_DIR%\assets"
if not exist "%INSTALL_DIR%\examples" mkdir "%INSTALL_DIR%\examples"

copy /y "%~dp0bin\*" "%INSTALL_DIR%\bin\" >nul 2>&1
copy /y "%~dp0assets\*" "%INSTALL_DIR%\assets\" >nul 2>&1
copy /y "%~dp0examples\*" "%INSTALL_DIR%\examples\" >nul 2>&1

echo.
echo Adding %INSTALL_DIR%\bin to User PATH...
for /f "tokens=2*" %%a in ('reg query "HKCU\Environment" /v Path 2^>nul') do set "USER_PATH=%%b"

echo %USER_PATH% | find /i "%INSTALL_DIR%\bin" >nul
if errorlevel 1 (
    setx PATH "%USER_PATH%;%INSTALL_DIR%\bin" >nul
    echo PATH updated successfully!
) else (
    echo %INSTALL_DIR%\bin is already in your PATH.
)

echo.
echo =======================================================
echo Installation Complete!
echo.
echo You can run Mantra from any Command Prompt or PowerShell:
echo   mantra run examples\01_hello_world.mantra
echo   mantrac examples\01_hello_world.mantra --run
echo =======================================================
pause
EOF

cat << 'EOF' > "$WINDOWS_DIR/Install-Mantra.ps1"
# Mantra Programming Language - Windows PowerShell Installer
# Run: Set-ExecutionPolicy Bypass -Scope Process -Force; .\Install-Mantra.ps1

$ErrorActionPreference = "Stop"

Write-Host "=======================================================" -ForegroundColor Cyan
Write-Host "    MANTRA PROGRAMMING LANGUAGE - POWERSHELL SETUP      " -ForegroundColor Cyan
Write-Host "    Steward: Solotron Technologies Private Limited      " -ForegroundColor Cyan
Write-Host "=======================================================" -ForegroundColor Cyan
Write-Host ""

$InstallDir = "$env:LOCALAPPDATA\Mantra"
$BinDir = "$InstallDir\bin"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

Write-Host "Target Directory: $InstallDir" -ForegroundColor Green

if (-not (Test-Path $BinDir)) { New-Item -ItemType Directory -Path $BinDir -Force | Out-Null }
if (-not (Test-Path "$InstallDir\examples")) { New-Item -ItemType Directory -Path "$InstallDir\examples" -Force | Out-Null }
if (-not (Test-Path "$InstallDir\assets")) { New-Item -ItemType Directory -Path "$InstallDir\assets" -Force | Out-Null }

Write-Host "Copying Mantra binaries and assets..."
if (Test-Path "$ScriptDir\bin") { Copy-Item -Path "$ScriptDir\bin\*" -Destination $BinDir -Recurse -Force }
if (Test-Path "$ScriptDir\examples") { Copy-Item -Path "$ScriptDir\examples\*" -Destination "$InstallDir\examples" -Recurse -Force }
if (Test-Path "$ScriptDir\assets") { Copy-Item -Path "$ScriptDir\assets\*" -Destination "$InstallDir\assets" -Recurse -Force }

Write-Host "Configuring Environment PATH..."
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -notlike "*$BinDir*") {
    $NewPath = "$UserPath;$BinDir"
    [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
    $env:PATH = "$env:PATH;$BinDir"
    Write-Host "Added $BinDir to User PATH." -ForegroundColor Yellow
} else {
    Write-Host "$BinDir is already in User PATH." -ForegroundColor DarkGray
}

Write-Host ""
Write-Host "=======================================================" -ForegroundColor Cyan
Write-Host "Mantra Installed Successfully!" -ForegroundColor Green
Write-Host "  Compiler: $BinDir\mantrac"
Write-Host "  MDK CLI:  $BinDir\mantra"
Write-Host "=======================================================" -ForegroundColor Cyan
EOF

ZIP_OUTPUT="$TMP_STAGE/mantra-${VERSION}-windows.zip"
if command -v zip >/dev/null 2>&1; then
    (cd "$WINDOWS_DIR" && zip -r -q "$ZIP_OUTPUT" .)
    echo "   ✓ Created Windows ZIP bundle: mantra-${VERSION}-windows.zip"
fi

# ------------------------------------------------------------------------------
# 4. GENERATE UNIVERSAL ROOT INSTALLER SCRIPT & README
# ------------------------------------------------------------------------------
ROOT_INSTALL_SH="$TMP_STAGE/install.sh"
cat << 'EOF' > "$ROOT_INSTALL_SH"
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
EOF
chmod 755 "$ROOT_INSTALL_SH"

ROOT_README_MD="$TMP_STAGE/README.md"
cat << EOF > "$ROOT_README_MD"
# Mantra Programming Language - Official Installers (v${VERSION})

Developed and maintained by **Solotron Technologies Private Limited**.
Dual-licensed under **The MIT License** and **GNU General Public License v3.0**.

![Mantra Logo](assets/images/mantra_logo.png)

This folder contains pre-built installation packages for macOS, Linux, and Windows.

---

## Directory Overview

\`\`\`text
installer/
├── install.sh                    # Universal installer (auto-detects macOS/Linux)
├── bin/                          # Direct compiler and MDK binaries (mantra, mantrac)
├── macos/
│   ├── Mantra-${VERSION}-macOS.dmg # Apple macOS Disk Image (.dmg) Installer
│   └── install.sh                # macOS terminal installer script
├── linux/
│   ├── mantra-installer-linux.sh   # Linux self-extracting .sh installer
│   ├── mantra-${VERSION}-linux.tar.gz # Linux portable archive
│   ├── mantra.desktop             # Freedesktop application entry
│   └── install.sh                # Linux terminal installer script
└── windows/
    ├── mantra-${VERSION}-windows.zip # Windows portable ZIP bundle
    ├── install.bat               # Windows Command Prompt batch installer
    └── Install-Mantra.ps1         # Windows PowerShell installer
\`\`\`

---

## Quick Install Commands

### macOS:
\`\`\`bash
./installer/macos/install.sh
\`\`\`
Or open \`installer/macos/Mantra-${VERSION}-macOS.dmg\`.

### Linux:
\`\`\`bash
sudo ./installer/linux/install.sh
\`\`\`
Or run the self-extracting installer:
\`\`\`bash
sudo ./installer/linux/mantra-installer-linux.sh
\`\`\`

### Windows:
Run with PowerShell:
\`\`\`powershell
.\\installer\\windows\\Install-Mantra.ps1
\`\`\`
Or double-click \`installer\\windows\\install.bat\`.

---

## Verifying Installation
\`\`\`bash
mantra --version
mantrac --help
\`\`\`
EOF

# ------------------------------------------------------------------------------
# 5. ASSEMBLE FULL INSTALLER FOLDER IN STAGING
# ------------------------------------------------------------------------------
FINAL_STAGE="$TMP_STAGE/final_installer"
mkdir -p "$FINAL_STAGE/bin" "$FINAL_STAGE/assets/images" "$FINAL_STAGE/macos/bin" "$FINAL_STAGE/linux/bin" "$FINAL_STAGE/windows/bin"

# Root level
cp "$ROOT_INSTALL_SH" "$FINAL_STAGE/install.sh"
cp "$ROOT_README_MD" "$FINAL_STAGE/README.md"
cp "$STAGE_BIN/"* "$FINAL_STAGE/bin/"
cp "$STAGE_ASSETS/mantra_logo.png" "$FINAL_STAGE/assets/images/" 2>/dev/null || true

# macOS
cp "$MACOS_DIR/install.sh" "$FINAL_STAGE/macos/install.sh"
cp "$STAGE_BIN/"* "$FINAL_STAGE/macos/bin/"
if [ -f "$DMG_OUTPUT" ]; then
    cp "$DMG_OUTPUT" "$FINAL_STAGE/macos/Mantra-${VERSION}-macOS.dmg"
    # Backward compatible copy
    cp "$DMG_OUTPUT" "$FINAL_STAGE/macos/Mantra-0.1.0-macOS.dmg" 2>/dev/null || true
fi

# Linux
cp "$LINUX_DIR/install.sh" "$FINAL_STAGE/linux/install.sh"
cp "$LINUX_DIR/mantra.desktop" "$FINAL_STAGE/linux/mantra.desktop"
cp "$STAGE_BIN/"* "$FINAL_STAGE/linux/bin/"
cp "$TAR_OUTPUT" "$FINAL_STAGE/linux/mantra-${VERSION}-linux.tar.gz"
cp "$TAR_OUTPUT" "$FINAL_STAGE/linux/mantra-0.1.0-linux.tar.gz" 2>/dev/null || true
cp "$RUN_OUTPUT" "$FINAL_STAGE/linux/mantra-installer-linux.sh"

# Windows
cp "$WINDOWS_DIR/install.bat" "$FINAL_STAGE/windows/install.bat"
cp "$WINDOWS_DIR/Install-Mantra.ps1" "$FINAL_STAGE/windows/Install-Mantra.ps1"
cp "$STAGE_BIN/"* "$FINAL_STAGE/windows/bin/"
if [ -f "$ZIP_OUTPUT" ]; then
    cp "$ZIP_OUTPUT" "$FINAL_STAGE/windows/mantra-${VERSION}-windows.zip"
fi

# ------------------------------------------------------------------------------
# 6. DEPLOY TO TARGET DESTINATION
# ------------------------------------------------------------------------------
echo ">> Deploying generated installer packages..."

# Target: installer directory
mkdir -p "$DEST_PKG_INSTALLER"
if command -v rsync >/dev/null 2>&1; then
    rsync -a --delete "$FINAL_STAGE/" "$DEST_PKG_INSTALLER/"
else
    cp -Rf "$FINAL_STAGE/"* "$DEST_PKG_INSTALLER/" 2>/dev/null || cp -rf "$FINAL_STAGE"/* "$DEST_PKG_INSTALLER/" || true
fi
echo "   ✓ Successfully populated: $DEST_PKG_INSTALLER"

# Also sync to website installer directory if available
if [ -d "$WEB_DIR" ]; then
    mkdir -p "$WEB_DIR/installer"
    if command -v rsync >/dev/null 2>&1; then
        rsync -a --delete "$FINAL_STAGE/" "$WEB_DIR/installer/" 2>/dev/null || true
    else
        cp -Rf "$FINAL_STAGE/"* "$WEB_DIR/installer/" 2>/dev/null || cp -rf "$FINAL_STAGE"/* "$WEB_DIR/installer/" 2>/dev/null || true
    fi
    echo "   ✓ Successfully synced to website: $WEB_DIR/installer"
fi

# Populate dist/ and versioned dist/releases/v${VERSION}
DEST_RELEASE_DIR="$PKG_DIR/dist/releases/v${VERSION}"
mkdir -p "$DEST_PKG_DIST" "$DEST_RELEASE_DIR"

if [ -f "$DMG_OUTPUT" ]; then
    cp -f "$DMG_OUTPUT" "$DEST_PKG_DIST/Mantra-${VERSION}-macOS.dmg"
    cp -f "$DMG_OUTPUT" "$DEST_RELEASE_DIR/Mantra-${VERSION}-macOS.dmg"
fi
if [ -f "$TAR_OUTPUT" ]; then
    cp -f "$TAR_OUTPUT" "$DEST_PKG_DIST/mantra-${VERSION}-linux.tar.gz"
    cp -f "$TAR_OUTPUT" "$DEST_RELEASE_DIR/mantra-${VERSION}-linux.tar.gz"
fi
if [ -f "$RUN_OUTPUT" ]; then
    cp -f "$RUN_OUTPUT" "$DEST_PKG_DIST/mantra-installer-linux.sh"
    cp -f "$RUN_OUTPUT" "$DEST_RELEASE_DIR/mantra-installer-linux.sh"
fi
if [ -f "$ZIP_OUTPUT" ]; then
    cp -f "$ZIP_OUTPUT" "$DEST_PKG_DIST/mantra-${VERSION}-windows.zip"
    cp -f "$ZIP_OUTPUT" "$DEST_RELEASE_DIR/mantra-${VERSION}-windows.zip"
fi
echo "   ✓ Successfully populated: $DEST_RELEASE_DIR"

# Generate Chocolatey & Homebrew release distribution packages
if [ -f "$PKG_DIR/scripts/release/update_distributions.sh" ]; then
    echo ">> Updating Chocolatey (.nupkg) and Homebrew (mantra.rb)..."
    bash "$PKG_DIR/scripts/release/update_distributions.sh" "$VERSION" || true
fi

# Generate Release Manifest, SBOMs (SPDX, CycloneDX) and Checksums
if [ -f "$PKG_DIR/scripts/release/sign_release.sh" ]; then
    echo ">> Generating Release Manifest, SBOMs, and Checksums..."
    bash "$PKG_DIR/scripts/release/sign_release.sh" "$VERSION" || true
fi

echo ""
echo "=================================================================="
echo "   BUILD COMPLETE! INSTALLERS GENERATED FOR v${VERSION}"
echo "=================================================================="
echo "Location updated: $DEST_PKG_INSTALLER"
echo ""
echo "Files created:"
ls -lh "$DEST_PKG_INSTALLER/macos/"*.dmg 2>/dev/null || true
ls -lh "$DEST_PKG_INSTALLER/linux/"*.tar.gz "$DEST_PKG_INSTALLER/linux/"*.sh 2>/dev/null || true
ls -lh "$DEST_PKG_INSTALLER/windows/"*.zip 2>/dev/null || true
echo "=================================================================="

