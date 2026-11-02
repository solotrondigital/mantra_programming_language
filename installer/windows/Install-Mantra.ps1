param([string]$Version)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$BinSource = Join-Path $ScriptDir "bin"
$MantraExe = Join-Path $BinSource "mantra.exe"
if (-not (Test-Path $MantraExe)) { throw "Mantra binaries were not found in $BinSource." }

if ([string]::IsNullOrWhiteSpace($Version)) {
    $VersionOutput = & $MantraExe --version | Select-Object -First 1
    if ($VersionOutput -notmatch 'MDK\) v([^ ]+)') { throw "Could not determine the bundled Mantra version." }
    $Version = $Matches[1]
}
if ($Version -notmatch '^[A-Za-z0-9.+-]+$') { throw "Invalid Mantra version: $Version" }

$InstallRoot = if ($env:MANTRA_HOME) { $env:MANTRA_HOME } else { Join-Path $env:LOCALAPPDATA "Mantra" }
$VersionBin = Join-Path $InstallRoot "versions\$Version\bin"
$CommandBin = Join-Path $InstallRoot "bin"
New-Item -ItemType Directory -Path $VersionBin, $CommandBin -Force | Out-Null

# Preserve the binary-only layout installed by earlier installers as an
# uninstallable version before replacing it with the stable command wrappers.
$LegacyMantra = Join-Path $CommandBin "mantra.exe"
if (Test-Path $LegacyMantra) {
    $LegacyOutput = & $LegacyMantra --version | Select-Object -First 1
    if ($LegacyOutput -match 'MDK\) v([^ ]+)') {
        $LegacyVersion = $Matches[1]
        if ($LegacyVersion -ne $Version -and $LegacyVersion -match '^[A-Za-z0-9.+-]+$') {
            $LegacyBin = Join-Path $InstallRoot "versions\$LegacyVersion\bin"
            New-Item -ItemType Directory -Path $LegacyBin -Force | Out-Null
            foreach ($Name in @("mantra", "mantrac", "mantra-lsp")) {
                $LegacyBinary = Join-Path $CommandBin "$Name.exe"
                if (Test-Path $LegacyBinary) {
                    Copy-Item -LiteralPath $LegacyBinary -Destination (Join-Path $LegacyBin "$Name.exe") -Force
                }
            }
        }
    }
}

Copy-Item -Path (Join-Path $BinSource "*") -Destination $VersionBin -Recurse -Force
Set-Content -LiteralPath (Join-Path $InstallRoot "active-version.txt") -Value $Version -NoNewline

foreach ($Name in @("mantra", "mantrac", "mantra-lsp")) {
    Remove-Item -LiteralPath (Join-Path $CommandBin "$Name.exe") -Force -ErrorAction SilentlyContinue
    $Wrapper = Join-Path $CommandBin "$Name.cmd"
    $Text = @"
@echo off
set "MANTRA_ROOT=$InstallRoot"
set "MANTRA_HOME=%MANTRA_ROOT%"
set /p "MANTRA_VERSION=" < "%MANTRA_ROOT%\active-version.txt"
"%MANTRA_ROOT%\versions\%MANTRA_VERSION%\bin\$Name.exe" %*
"@
    Set-Content -LiteralPath $Wrapper -Value $Text -Encoding Ascii
}

$Examples = Join-Path $ScriptDir "examples"
if (Test-Path $Examples) {
    New-Item -ItemType Directory -Path (Join-Path $InstallRoot "examples") -Force | Out-Null
    Copy-Item -Path (Join-Path $Examples "*") -Destination (Join-Path $InstallRoot "examples") -Recurse -Force
}
$Assets = Join-Path $ScriptDir "assets"
if (Test-Path $Assets) {
    New-Item -ItemType Directory -Path (Join-Path $InstallRoot "assets") -Force | Out-Null
    Copy-Item -Path (Join-Path $Assets "*") -Destination (Join-Path $InstallRoot "assets") -Recurse -Force
}

$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
$PathEntries = @($UserPath -split ';' | Where-Object { $_ })
$OtherPathEntries = @($PathEntries | Where-Object { $_.TrimEnd('\') -ine $CommandBin.TrimEnd('\') })
[Environment]::SetEnvironmentVariable("PATH", (@($CommandBin) + $OtherPathEntries) -join ';', "User")
$env:PATH = "$CommandBin;$env:PATH"

Write-Host "Installed Mantra v$Version with mantra, mantrac, and mantra-lsp."
Write-Host "Version files: $InstallRoot\versions\$Version"
Write-Host "Active commands: $CommandBin"
Write-Host "Open a new terminal to use the updated PATH."
& (Join-Path $VersionBin "mantra.exe") --version
