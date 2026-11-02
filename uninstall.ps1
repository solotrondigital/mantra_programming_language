$ErrorActionPreference = "Stop"
$InstallRoot = if ($env:MANTRA_HOME) { $env:MANTRA_HOME } else { Join-Path $env:LOCALAPPDATA "Mantra" }
$CommandBin = Join-Path $InstallRoot "bin"
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
$OtherEntries = @($UserPath -split ';' | Where-Object { $_ -and $_.TrimEnd('\') -ine $CommandBin.TrimEnd('\') })
[Environment]::SetEnvironmentVariable("PATH", ($OtherEntries -join ';'), "User")
[Environment]::SetEnvironmentVariable("MANTRA_HOME", $null, "User")
if (Get-Command choco.exe -ErrorAction SilentlyContinue) {
    choco.exe uninstall mantra -y --limit-output
    if ($LASTEXITCODE -ne 0) { throw "Chocolatey could not uninstall Mantra." }
}
if (Test-Path $InstallRoot) { Remove-Item -LiteralPath $InstallRoot -Recurse -Force }
Write-Host "Mantra has been removed. Open a new terminal for the PATH change to take effect."
