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
