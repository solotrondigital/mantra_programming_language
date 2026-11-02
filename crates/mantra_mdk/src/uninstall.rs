use std::cmp::Ordering;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
#[cfg(windows)]
use std::process::{Command, Stdio};

pub fn run(args: &[String]) -> Result<(), String> {
    if args.len() != 2 || args[0] != "--version" {
        return Err("Usage: mantra uninstall --version <version>".to_string());
    }

    let version = args[1].trim_start_matches(|character| character == 'v' || character == 'V');
    if !valid_version(version) {
        return Err(format!(
            "Invalid version '{version}'. Use a version such as 0.8.0."
        ));
    }

    let root = install_root()?;
    let version_dir = root.join("versions").join(version);
    if !version_dir.is_dir() {
        return Err(format!(
            "Mantra v{version} is not installed in {}.",
            version_dir.display()
        ));
    }

    let next_version = installed_versions(&root)
        .into_iter()
        .filter(|installed| installed != version)
        .max_by(|left, right| compare_versions(left, right).then_with(|| left.cmp(right)));

    #[cfg(windows)]
    schedule_windows_removal(&root, version, next_version.as_deref())?;

    #[cfg(not(windows))]
    {
        fs::remove_dir_all(&version_dir).map_err(|error| {
            format!(
                "Could not remove Mantra v{version} at {}: {error}",
                version_dir.display()
            )
        })?;
        activate_unix_version(&root, next_version.as_deref())?;
    }

    println!("Uninstalled Mantra v{version} and its compiler, CLI, and language server binaries.");
    if let Some(active) = next_version {
        println!("Mantra v{active} is now the active installed version.");
    } else {
        println!("No Mantra versions remain installed.");
    }
    Ok(())
}

fn valid_version(version: &str) -> bool {
    !version.is_empty()
        && version.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '+')
        })
        && version.chars().any(|character| character.is_ascii_digit())
}

fn install_root() -> Result<PathBuf, String> {
    if let Some(root) = env::var_os("MANTRA_HOME") {
        return Ok(PathBuf::from(root));
    }
    #[cfg(windows)]
    let base = env::var_os("LOCALAPPDATA").or_else(|| env::var_os("USERPROFILE"));
    #[cfg(not(windows))]
    let base = env::var_os("HOME");
    base.map(PathBuf::from)
        .map(|path| {
            #[cfg(windows)]
            {
                path.join("Mantra")
            }
            #[cfg(not(windows))]
            {
                path.join(".mantra")
            }
        })
        .ok_or_else(|| "Could not determine the Mantra installation directory.".to_string())
}

fn installed_versions(root: &Path) -> Vec<String> {
    let Ok(entries) = fs::read_dir(root.join("versions")) else {
        return Vec::new();
    };
    entries
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|version| valid_version(version))
        .collect()
}

fn compare_versions(left: &str, right: &str) -> Ordering {
    let (left_numbers, left_pre) = split_version(left);
    let (right_numbers, right_pre) = split_version(right);
    for index in 0..3 {
        let ordering = left_numbers[index].cmp(&right_numbers[index]);
        if ordering != Ordering::Equal {
            return ordering;
        }
    }
    match (left_pre, right_pre) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (Some(left), Some(right)) => compare_prerelease(left, right),
    }
}

fn split_version(version: &str) -> ([u64; 3], Option<&str>) {
    let without_build = version.split('+').next().unwrap_or(version);
    let (release, prerelease) = match without_build.split_once('-') {
        Some((release, prerelease)) => (release, Some(prerelease)),
        None => (without_build, None),
    };
    let mut numbers = [0; 3];
    for (index, part) in release.split('.').take(3).enumerate() {
        numbers[index] = part.parse().unwrap_or(0);
    }
    (numbers, prerelease)
}

fn compare_prerelease(left: &str, right: &str) -> Ordering {
    for (left_part, right_part) in left.split('.').zip(right.split('.')) {
        let ordering = match (left_part.parse::<u64>(), right_part.parse::<u64>()) {
            (Ok(left), Ok(right)) => left.cmp(&right),
            (Ok(_), Err(_)) => Ordering::Less,
            (Err(_), Ok(_)) => Ordering::Greater,
            (Err(_), Err(_)) => left_part.cmp(right_part),
        };
        if ordering != Ordering::Equal {
            return ordering;
        }
    }
    left.split('.').count().cmp(&right.split('.').count())
}

#[cfg(not(windows))]
fn activate_unix_version(root: &Path, version: Option<&str>) -> Result<(), String> {
    use std::os::unix::fs::symlink;

    let bin_dir = root.join("bin");
    fs::create_dir_all(&bin_dir)
        .map_err(|error| format!("Could not access {}: {error}", bin_dir.display()))?;
    for name in ["mantra", "mantrac", "mantra-lsp"] {
        let link = bin_dir.join(name);
        let _ = fs::remove_file(&link);
        if let Some(version) = version {
            let target = root.join("versions").join(version).join("bin").join(name);
            if target.is_file() {
                symlink(&target, &link)
                    .map_err(|error| format!("Could not activate {}: {error}", target.display()))?;
            }
        }
    }
    Ok(())
}

#[cfg(windows)]
fn schedule_windows_removal(
    root: &Path,
    version: &str,
    next_version: Option<&str>,
) -> Result<(), String> {
    use std::os::windows::process::CommandExt;

    let root = powershell_literal(&root.to_string_lossy());
    let version = powershell_literal(version);
    let next = powershell_literal(next_version.unwrap_or(""));
    let command = format!(
        "$ErrorActionPreference='Stop'; Start-Sleep -Seconds 2; $root='{root}'; Remove-Item -LiteralPath (Join-Path $root 'versions\\{version}') -Recurse -Force; $active=Join-Path $root 'active-version.txt'; if ('{next}' -ne '') {{ Set-Content -LiteralPath $active -Value '{next}' -NoNewline }} else {{ Remove-Item -LiteralPath $active -Force -ErrorAction SilentlyContinue; Remove-Item -LiteralPath (Join-Path $root 'bin\\mantra.cmd'),(Join-Path $root 'bin\\mantrac.cmd'),(Join-Path $root 'bin\\mantra-lsp.cmd') -Force -ErrorAction SilentlyContinue }}",
    );
    Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-WindowStyle",
            "Hidden",
            "-Command",
            &command,
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .creation_flags(0x08000000)
        .spawn()
        .map_err(|error| format!("Could not schedule version removal: {error}"))?;
    Ok(())
}

#[cfg(windows)]
fn powershell_literal(value: &str) -> String {
    value.replace('\'', "''")
}
