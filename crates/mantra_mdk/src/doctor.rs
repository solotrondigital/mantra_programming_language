// ==============================================================================
// mantra_mdk::doctor — Toolchain & Environment Diagnostics (mantra doctor)
// ==============================================================================

use std::env;
use std::path::PathBuf;
use std::process::Command;

pub struct ToolStatus {
    pub name: String,
    pub status: bool,
    pub version: Option<String>,
    pub path: Option<String>,
    pub note: Option<String>,
}

pub struct DoctorReport {
    pub tools: Vec<ToolStatus>,
}

impl DoctorReport {
    pub fn is_healthy(&self) -> bool {
        self.tools.iter().all(|t| t.status)
    }

    pub fn print_summary(&self) {
        println!("Doctor summary (to see all details, run mantra doctor -v):");
        for t in &self.tools {
            let symbol = if t.status { "✓" } else { "✗" };
            let ver = t.version.as_deref().unwrap_or("");
            let note = t.note.as_deref().map(|n| format!(" ({})", n)).unwrap_or_default();
            println!("  [{}] {}: {} {}", symbol, t.name, ver, note);
        }
    }
}

pub struct Doctor;

impl Doctor {
    pub fn run(verbose: bool) -> DoctorReport {
        println!("Mantra Doctor — Environment and Toolchain Diagnostics");
        println!("====================================================\n");

        let mut report = DoctorReport { tools: Vec::new() };

        // 1. Mantra Compiler (mantrac)
        report.tools.push(check_tool(
            "Mantra Compiler (mantrac)",
            &["mantrac"],
            &["-v"],
            true,
        ));

        // 2. Mantra Language Server (mantra-lsp)
        report.tools.push(check_tool(
            "Mantra Language Server (mantra-lsp)",
            &["mantra-lsp"],
            &["-v"],
            true,
        ));

        // 3. Rust Toolchain (rustc / cargo)
        report.tools.push(check_tool(
            "Rust Compiler (rustc)",
            &["rustc"],
            &["--version"],
            true,
        ));

        // 4. C/C++ Native Toolchain (clang / gcc)
        report.tools.push(check_tool(
            "C/C++ Compiler (clang/gcc)",
            &["clang", "gcc", "cc"],
            &["--version"],
            true,
        ));

        // 5. Node.js (Web & Extension Tooling)
        report.tools.push(check_tool(
            "Node.js Runtime",
            &["node"],
            &["--version"],
            false,
        ));

        // 6. Visual Studio Code
        report.tools.push(check_tool(
            "Visual Studio Code (code)",
            &["code"],
            &["--version"],
            false,
        ));

        // 7. Neovim (nvim)
        report.tools.push(check_tool(
            "Neovim (nvim)",
            &["nvim"],
            &["--version"],
            false,
        ));

        // 8. Mobile Platforms
        if cfg!(target_os = "macos") {
            report.tools.push(check_xcode());
        }
        report.tools.push(check_android_sdk());

        // Output results
        for t in &report.tools {
            let marker = if t.status { "✓" } else { "!" };
            let color_marker = if t.status {
                format!("\x1b[32m[{}]\x1b[0m", marker)
            } else {
                format!("\x1b[33m[{}]\x1b[0m", marker)
            };

            print!("{} {}", color_marker, t.name);
            if let Some(ref v) = t.version {
                print!(" (version: {})", v.trim());
            }
            println!();

            if verbose {
                if let Some(ref p) = t.path {
                    println!("    Path: {}", p);
                }
                if let Some(ref note) = t.note {
                    println!("    Note: {}", note);
                }
            }
        }

        println!();
        if report.is_healthy() {
            println!("\x1b[32mAll core development components are verified and operational!\x1b[0m");
        } else {
            println!("\x1b[33mSome optional or auxiliary components were not detected. See above for details.\x1b[0m");
        }

        report
    }
}

fn check_tool(label: &str, binary_names: &[&str], version_args: &[&str], required: bool) -> ToolStatus {
    for &bin in binary_names {
        // Also check target/release and ~/.mantra/bin
        let candidates = resolve_binary_candidates(bin);
        for candidate in candidates {
            if let Ok(output) = Command::new(&candidate).args(version_args).output() {
                if output.status.success() {
                    let text = String::from_utf8_lossy(&output.stdout);
                    let line = text.lines().next().unwrap_or("").to_string();
                    return ToolStatus {
                        name: label.to_string(),
                        status: true,
                        version: Some(line),
                        path: Some(candidate.to_string_lossy().to_string()),
                        note: None,
                    };
                }
            }
        }
    }

    ToolStatus {
        name: label.to_string(),
        status: !required,
        version: None,
        path: None,
        note: if required {
            Some("Required component missing from PATH and local build".to_string())
        } else {
            Some("Optional editor/platform component not detected".to_string())
        },
    }
}

fn resolve_binary_candidates(name: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    // 1. Plain PATH lookup
    paths.push(PathBuf::from(name));

    // 2. Next to running executable
    if let Ok(current) = env::current_exe() {
        if let Some(parent) = current.parent() {
            paths.push(parent.join(name));
        }
    }

    // 3. Workspace target/release and target/debug
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    paths.push(cwd.join("target/release").join(name));
    paths.push(cwd.join("target/debug").join(name));

    // 4. ~/.mantra/bin
    if let Ok(home) = env::var("HOME") {
        paths.push(PathBuf::from(home).join(".mantra/bin").join(name));
    }

    paths
}

fn check_xcode() -> ToolStatus {
    if let Ok(output) = Command::new("xcode-select").arg("-p").output() {
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            return ToolStatus {
                name: "Apple Xcode Developer Tools".to_string(),
                status: true,
                version: Some("Installed".to_string()),
                path: Some(path_str),
                note: None,
            };
        }
    }
    ToolStatus {
        name: "Apple Xcode Developer Tools".to_string(),
        status: true, // Not strictly required for non-iOS
        version: None,
        path: None,
        note: Some("Xcode not active (needed for iOS builds)".to_string()),
    }
}

fn check_android_sdk() -> ToolStatus {
    let sdk_root = env::var("ANDROID_HOME")
        .or_else(|_| env::var("ANDROID_SDK_ROOT"));
    match sdk_root {
        Ok(path) => ToolStatus {
            name: "Android SDK".to_string(),
            status: true,
            version: Some("Configured".to_string()),
            path: Some(path),
            note: None,
        },
        Err(_) => ToolStatus {
            name: "Android SDK".to_string(),
            status: true,
            version: None,
            path: None,
            note: Some("ANDROID_HOME environment variable not set (needed for Android APK builds)".to_string()),
        },
    }
}
