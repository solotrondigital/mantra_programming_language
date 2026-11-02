use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const REPOSITORY: &str = "solotrondigital/mantra_programming_language";

struct TempDir(PathBuf);

impl TempDir {
    fn create() -> Result<Self, String> {
        let path = env::temp_dir().join(format!(
            "mantra-install-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        fs::create_dir_all(&path)
            .map_err(|error| format!("Could not create temporary install directory: {error}"))?;
        Ok(Self(path))
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

pub fn run(args: &[String]) -> Result<(), String> {
    let requested_version = match args {
        [] => None,
        [flag, version] if flag == "--version" => Some(version.as_str()),
        _ => return Err("Usage: mantra install [--version <version>]".to_string()),
    };

    let version = if let Some(version) = requested_version {
        let version = version.trim_start_matches(|character| character == 'v' || character == 'V');
        validate_version(version)?;
        version.to_string()
    } else {
        latest_version()?
    };

    let temp = TempDir::create()?;
    #[cfg(target_os = "macos")]
    install_macos(&temp, &version)?;
    #[cfg(target_os = "linux")]
    install_linux(&temp, &version)?;
    #[cfg(windows)]
    install_windows(&temp, &version)?;
    #[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
    return Err(
        "`mantra install` is currently supported on macOS, Linux, and Windows.".to_string(),
    );

    println!("Mantra v{version} is installed and selected as the active version.");
    Ok(())
}

fn validate_version(version: &str) -> Result<(), String> {
    if version.is_empty()
        || !version.chars().any(|character| character.is_ascii_digit())
        || !version.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '+')
        })
    {
        return Err(format!(
            "Invalid version '{version}'. Use a version such as 0.8.0."
        ));
    }
    Ok(())
}

fn latest_version() -> Result<String, String> {
    let url = format!("https://api.github.com/repos/{REPOSITORY}/releases/latest");
    let response = output(
        "curl",
        &[
            "-fsSL",
            "--retry",
            "3",
            "-H",
            "User-Agent: mantra-mdk",
            &url,
        ],
    )?;
    let json: Value = serde_json::from_slice(&response)
        .map_err(|error| format!("GitHub returned invalid release metadata: {error}"))?;
    let tag = json
        .get("tag_name")
        .and_then(Value::as_str)
        .ok_or_else(|| "GitHub latest release response did not contain tag_name".to_string())?;
    let version = tag.trim_start_matches(|character| character == 'v' || character == 'V');
    validate_version(version)?;
    Ok(version.to_string())
}

fn download_asset(version: &str, filename: &str, destination: &Path) -> Result<(), String> {
    let url = format!("https://github.com/{REPOSITORY}/releases/download/v{version}/{filename}");
    let destination = destination.to_string_lossy().into_owned();
    run_command(
        "curl",
        &[
            "-fL",
            "--retry",
            "3",
            "--show-error",
            "-H",
            "User-Agent: mantra-mdk",
            "-o",
            &destination,
            &url,
        ],
    )?;
    if !Path::new(&destination).is_file() {
        return Err(format!("Release download did not create {destination}"));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn install_linux(temp: &TempDir, version: &str) -> Result<(), String> {
    let archive = temp.path().join("mantra-linux.tar.gz");
    download_asset(version, &format!("mantra-{version}-linux.tar.gz"), &archive)?;
    let extracted = temp.path().join("bundle");
    fs::create_dir_all(&extracted).map_err(|error| error.to_string())?;
    run_command(
        "tar",
        &[
            "-xzf",
            archive.to_string_lossy().as_ref(),
            "-C",
            extracted.to_string_lossy().as_ref(),
        ],
    )?;
    let installer = extracted.join("install.sh");
    if !installer.is_file() {
        return Err("Downloaded Linux release does not contain install.sh".to_string());
    }
    run_command("bash", &[installer.to_string_lossy().as_ref()])
}

#[cfg(target_os = "macos")]
fn install_macos(temp: &TempDir, version: &str) -> Result<(), String> {
    let image = temp.path().join("mantra-macos.dmg");
    download_asset(version, &format!("Mantra-{version}-macOS.dmg"), &image)?;
    let mount = temp.path().join("mounted");
    fs::create_dir_all(&mount).map_err(|error| error.to_string())?;
    run_command(
        "hdiutil",
        &[
            "attach",
            "-readonly",
            "-nobrowse",
            "-mountpoint",
            mount.to_string_lossy().as_ref(),
            image.to_string_lossy().as_ref(),
        ],
    )?;
    let installer = mount.join("install.sh");
    let result = if installer.is_file() {
        run_command("bash", &[installer.to_string_lossy().as_ref()])
    } else {
        Err("Downloaded macOS release does not contain install.sh".to_string())
    };
    let detach = run_command("hdiutil", &["detach", mount.to_string_lossy().as_ref()]);
    result.and(detach)
}

#[cfg(windows)]
fn install_windows(temp: &TempDir, version: &str) -> Result<(), String> {
    let archive = temp.path().join("mantra-windows.zip");
    download_asset(version, &format!("mantra-{version}-windows.zip"), &archive)?;
    let extracted = temp.path().join("bundle");
    fs::create_dir_all(&extracted).map_err(|error| error.to_string())?;
    let script = format!(
        "Expand-Archive -LiteralPath '{}' -DestinationPath '{}' -Force; & '{}' -Version '{}'",
        powershell_literal(&archive.to_string_lossy()),
        powershell_literal(&extracted.to_string_lossy()),
        powershell_literal(&extracted.join("Install-Mantra.ps1").to_string_lossy()),
        powershell_literal(version),
    );
    run_command(
        "powershell.exe",
        &[
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            &script,
        ],
    )
}

fn output(program: &str, args: &[&str]) -> Result<Vec<u8>, String> {
    let executable = command_name(program);
    let result = Command::new(&executable)
        .args(args)
        .output()
        .map_err(|error| format!("Could not start {program}: {error}"))?;
    if !result.status.success() {
        return Err(format!(
            "{program} failed ({}): {}",
            result.status,
            String::from_utf8_lossy(&result.stderr).trim()
        ));
    }
    Ok(result.stdout)
}

fn run_command(program: &str, args: &[&str]) -> Result<(), String> {
    let executable = command_name(program);
    let status = Command::new(&executable)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|error| format!("Could not start {program}: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{program} failed with status {status}"))
    }
}

fn command_name(program: &str) -> String {
    #[cfg(windows)]
    if program == "curl" {
        return "curl.exe".to_string();
    }
    program.to_string()
}

#[cfg(windows)]
fn powershell_literal(value: &str) -> String {
    value.replace('\'', "''")
}
