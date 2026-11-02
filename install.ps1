param([string]$Version)

$ErrorActionPreference = "Stop"
$Repository = "solotrondigital/mantra_programming_language"
if ([string]::IsNullOrWhiteSpace($Version)) {
    $Release = Invoke-RestMethod -Headers @{ "User-Agent" = "mantra-installer" } `
        -Uri "https://api.github.com/repos/$Repository/releases/latest"
    $Version = ([string]$Release.tag_name).TrimStart('v', 'V')
} else {
    $Version = $Version.TrimStart('v', 'V')
}
if ($Version -notmatch '^[A-Za-z0-9.+-]+$') { throw "Invalid Mantra version: $Version" }

$Temporary = Join-Path ([IO.Path]::GetTempPath()) ("mantra-install-" + [guid]::NewGuid().ToString("N"))
$Archive = Join-Path $Temporary "mantra-windows.zip"
$Bundle = Join-Path $Temporary "bundle"
New-Item -ItemType Directory -Path $Bundle -Force | Out-Null
try {
    $Url = "https://github.com/$Repository/releases/download/v$Version/mantra-$Version-windows.zip"
    Invoke-WebRequest -UseBasicParsing -Headers @{ "User-Agent" = "mantra-installer" } -Uri $Url -OutFile $Archive
    Expand-Archive -LiteralPath $Archive -DestinationPath $Bundle -Force
    $Installer = Join-Path $Bundle "Install-Mantra.ps1"
    if (-not (Test-Path $Installer)) { throw "Windows release archive does not contain Install-Mantra.ps1" }
    & $Installer
} finally {
    Remove-Item -LiteralPath $Temporary -Recurse -Force -ErrorAction SilentlyContinue
}
