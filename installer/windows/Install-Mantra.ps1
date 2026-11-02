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
