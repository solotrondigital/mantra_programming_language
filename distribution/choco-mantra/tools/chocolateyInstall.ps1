# ==============================================================================
# Chocolatey Installation Script for Mantra Programming Language Toolchain
# distribution/choco-mantra/tools/chocolateyInstall.ps1
# ==============================================================================
$ErrorActionPreference = 'Stop'

$packageName = 'mantra'
$version     = '0.4.0'
$url64       = "https://github.com/solotrondigital/mantra_programming_language/releases/download/v$version/mantra-$version-x86_64-pc-windows-msvc.zip"
$checksum64  = 'e57ed68903e9d80ebc40a10564f9e3121efd0bf71e121918b06a4488f230f7c6'
$checksumType = 'sha256'

$toolsDir    = "$(Split-Path -Parent $MyInvocation.MyCommand.Definition)"
$installDir  = Join-Path $toolsDir 'mantra-sdk'

Write-Host "Installing Mantra Programming Language SDK v$version..."

$packageArgs = @{
    packageName   = $packageName
    unzipLocation = $installDir
    url64bit      = $url64
    checksum64    = $checksum64
    checksumType64= $checksumType
}

# Download and unpack SDK archive
Install-ChocolateyZipPackage @packageArgs

# Add bin directory to user environment PATH
$binDir = Join-Path $installDir 'bin'
if (Test-Path $binDir) {
    Install-ChocolateyPath $binDir 'User'
    Write-Host "Added $binDir to user PATH."
} else {
    # If unzipped directly into installDir
    Install-ChocolateyPath $installDir 'User'
    Write-Host "Added $installDir to user PATH."
}

Write-Host "Mantra Programming Language v$version installed successfully!"
Write-Host "To verify installation, open a new shell and execute:"
Write-Host "    mantra --version"
Write-Host "    mantrac --version"
