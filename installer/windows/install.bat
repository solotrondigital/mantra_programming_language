@echo off
setlocal
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0Install-Mantra.ps1" %*
exit /b %ERRORLEVEL%
