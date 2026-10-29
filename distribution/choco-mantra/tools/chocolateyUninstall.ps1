# ==============================================================================
# Chocolatey Uninstallation Script for Mantra Programming Language Toolchain
# distribution/choco-mantra/tools/chocolateyUninstall.ps1
# ==============================================================================
$ErrorActionPreference = 'Stop'

$packageName = 'mantra'
$toolsDir    = "$(Split-Path -Parent $MyInvocation.MyCommand.Definition)"
$installDir  = Join-Path $toolsDir 'mantra-sdk'
$binDir      = Join-Path $installDir 'bin'

Write-Host "Uninstalling Mantra Programming Language..."

# Remove from PATH
if (Test-Path $binDir) {
    Uninstall-ChocolateyPath $binDir 'User'
}
Uninstall-ChocolateyPath $installDir 'User'

# Clean up unpacked SDK directory
if (Test-Path $installDir) {
    Remove-Item $installDir -Recurse -Force
}

Write-Host "Mantra Programming Language uninstalled successfully."

