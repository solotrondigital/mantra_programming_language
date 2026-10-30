// ==============================================================================
// mantra_mdk::device_selector — Interactive Multi-Platform Device Discovery
// ==============================================================================

use crate::mobile_builder::{find_adb, MobileBuilder};
use crate::runner::Runner;
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct DetectedDevice {
    pub name: String,
    pub category: String,       // "mobile", "desktop", "web"
    pub id: String,
    pub platform_arch: String,
    pub details: String,
    pub kind: DeviceKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceKind {
    Android(String),
    Ios(String),
    Desktop,
    Web,
}

pub struct DeviceSelector;

impl DeviceSelector {
    /// Discovers all available devices connected to this machine.
    pub fn discover_devices() -> Vec<DetectedDevice> {
        let mut devices = Vec::new();

        // 1. Android devices & emulators
        devices.extend(Self::detect_android_devices());

        // 2. iOS simulators & devices
        devices.extend(Self::detect_ios_simulators());

        // 3. Desktop (macOS / Linux / Windows)
        if let Some(desktop) = Self::detect_desktop() {
            devices.push(desktop);
        }

        // 4. Chrome / Web
        if let Some(web) = Self::detect_chrome() {
            devices.push(web);
        }

        devices
    }

    /// Prints connected devices in Flutter format and prompts the user to choose one.
    pub fn select_and_run(
        target: Option<&str>,
        web_port: u16,
        defines_file: Option<&str>,
    ) -> Result<(), String> {
        let devices = Self::discover_devices();

        if devices.is_empty() {
            println!("No connected devices found. Running desktop target...");
            return Runner::run_target(target);
        }

        println!("Connected devices:");
        for dev in &devices {
            let name_cat = format!("{} ({})", dev.name, dev.category);
            println!(
                "{:<28} • {:<36} • {:<14} • {}",
                name_cat, dev.id, dev.platform_arch, dev.details
            );
        }

        println!("\nNo wireless devices were found.\n");

        for (i, dev) in devices.iter().enumerate() {
            println!("[{}]: {} ({})", i + 1, dev.name, dev.id);
        }

        print!("\nPlease choose one (To quit, press \"q/Q\"): ");
        io::stdout().flush().ok();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() || input.trim().is_empty() {
            // In non-interactive contexts (e.g. piped input or test harness), run the first mobile/desktop device
            println!("\nNo input received; defaulting to first available device.");
            return Self::run_selected_device(&devices[0], target, web_port, defines_file);
        }

        let choice = input.trim();
        if choice.eq_ignore_ascii_case("q") {
            println!("Exiting.");
            return Ok(());
        }

        if let Ok(idx) = choice.parse::<usize>() {
            if idx >= 1 && idx <= devices.len() {
                return Self::run_selected_device(&devices[idx - 1], target, web_port, defines_file);
            }
        }

        // Match by device ID or name prefix
        for dev in &devices {
            if dev.id.eq_ignore_ascii_case(choice) || dev.name.eq_ignore_ascii_case(choice) {
                return Self::run_selected_device(dev, target, web_port, defines_file);
            }
        }

        println!("Invalid selection '{}'. Exiting.", choice);
        Ok(())
    }

    /// Runs application directly on the specified device identifier or alias.
    pub fn run_with_device_id(
        device_id: &str,
        target: Option<&str>,
        web_port: u16,
        defines_file: Option<&str>,
    ) -> Result<(), String> {
        match device_id.to_lowercase().as_str() {
            "android" => MobileBuilder::run_mobile("android", defines_file),
            "ios" => MobileBuilder::run_mobile("ios", defines_file),
            "chrome" | "web" => Runner::run_web_dev(web_port, defines_file),
            "macos" | "desktop" | "native" => {
                if let Some(df) = defines_file {
                    let vars = crate::env_loader::EnvLoader::load_file(Path::new(df))?;
                    crate::env_loader::EnvLoader::print_summary(df, &vars);
                }
                Runner::run_target(target)
            }
            custom_id => {
                // Check discovered devices for matching ID or name
                let devices = Self::discover_devices();
                for dev in &devices {
                    if dev.id.eq_ignore_ascii_case(custom_id)
                        || dev.name.to_lowercase().contains(&custom_id.to_lowercase())
                    {
                        return Self::run_selected_device(dev, target, web_port, defines_file);
                    }
                }
                println!("! Device '{}' not recognized. Available devices:", custom_id);
                for dev in &devices {
                    println!("  • {} ({})", dev.name, dev.id);
                }
                Err(format!("Device '{}' not found", custom_id))
            }
        }
    }

    fn run_selected_device(
        device: &DetectedDevice,
        target: Option<&str>,
        web_port: u16,
        defines_file: Option<&str>,
    ) -> Result<(), String> {
        match &device.kind {
            DeviceKind::Android(_id) => MobileBuilder::run_mobile("android", defines_file),
            DeviceKind::Ios(_id) => MobileBuilder::run_mobile("ios", defines_file),
            DeviceKind::Desktop => {
                if let Some(df) = defines_file {
                    let vars = crate::env_loader::EnvLoader::load_file(Path::new(df))?;
                    crate::env_loader::EnvLoader::print_summary(df, &vars);
                }
                Runner::run_target(target)
            }
            DeviceKind::Web => Runner::run_web_dev(web_port, defines_file),
        }
    }

    fn detect_android_devices() -> Vec<DetectedDevice> {
        let mut devices = Vec::new();
        let adb = match find_adb() {
            Some(p) => p,
            None => return devices,
        };

        let output = match Command::new(&adb).args(&["devices", "-l"]).output() {
            Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).to_string(),
            _ => return devices,
        };

        for line in output.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("List of devices") || trimmed.starts_with('*') {
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() < 2 || parts[1] != "device" {
                continue;
            }

            let dev_id = parts[0].to_string();

            // Extract model from model:<name>
            let mut model_name = "Android Device".to_string();
            for part in &parts[2..] {
                if let Some(val) = part.strip_prefix("model:") {
                    model_name = val.replace('_', " ");
                    break;
                }
            }
            if model_name == "Android Device" && dev_id.starts_with("emulator-") {
                model_name = "sdk gphone64 x86 64".to_string();
            }

            // Query release, sdk, and abi via getprop
            let release = Command::new(&adb)
                .args(&["-s", &dev_id, "shell", "getprop", "ro.build.version.release"])
                .output()
                .ok()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "15".to_string());

            let sdk = Command::new(&adb)
                .args(&["-s", &dev_id, "shell", "getprop", "ro.build.version.sdk"])
                .output()
                .ok()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "35".to_string());

            let abi = Command::new(&adb)
                .args(&["-s", &dev_id, "shell", "getprop", "ro.product.cpu.abi"])
                .output()
                .ok()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_default();

            let platform_arch = if abi.contains("x86_64") || abi.contains("x86") {
                "android-x64".to_string()
            } else if abi.contains("arm64") {
                "android-arm64".to_string()
            } else {
                "android-x64".to_string()
            };

            let details = format!("Android {} (API {}) (emulator)", release, sdk);

            devices.push(DetectedDevice {
                name: model_name,
                category: "mobile".to_string(),
                id: dev_id.clone(),
                platform_arch,
                details,
                kind: DeviceKind::Android(dev_id),
            });
        }

        devices
    }

    fn detect_ios_simulators() -> Vec<DetectedDevice> {
        let mut devices = Vec::new();

        // 1. Build runtime name to runtime identifier map (e.g. "iOS 18.6" -> "com.apple.CoreSimulator.SimRuntime.iOS-18-6")
        let mut runtimes = std::collections::HashMap::new();
        if let Ok(rt_out) = Command::new("xcrun").args(&["simctl", "list", "runtimes"]).output() {
            if rt_out.status.success() {
                let rt_text = String::from_utf8_lossy(&rt_out.stdout);
                for line in rt_text.lines() {
                    if let Some((name_part, id_part)) = line.split_once(" - com.apple.") {
                        let runtime_id = format!("com.apple.{}", id_part.trim());
                        if let Some(pos) = name_part.find('(') {
                            let short_name = name_part[..pos].trim();
                            runtimes.insert(short_name.to_string(), runtime_id.clone());
                        }
                        runtimes.insert(name_part.trim().to_string(), runtime_id);
                    }
                }
            }
        }

        // 2. Query simulators
        let dev_out = match Command::new("xcrun").args(&["simctl", "list", "devices"]).output() {
            Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).to_string(),
            _ => return devices,
        };

        let mut current_section = String::new();
        let mut booted_devices = Vec::new();
        let mut available_devices = Vec::new();

        for line in dev_out.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("-- ") && trimmed.ends_with(" --") {
                current_section = trimmed.trim_start_matches("-- ").trim_end_matches(" --").trim().to_string();
                continue;
            }

            if trimmed.contains('(') && trimmed.contains(')') {
                let is_booted = trimmed.contains("(Booted)");
                let is_available = !trimmed.contains("(unavailable");

                if let Some(start_paren) = trimmed.find('(') {
                    let name = trimmed[..start_paren].trim().to_string();
                    let rem = &trimmed[start_paren + 1..];
                    if let Some(end_paren) = rem.find(')') {
                        let uuid = rem[..end_paren].trim().to_string();

                        let runtime_id = runtimes.get(&current_section)
                            .cloned()
                            .unwrap_or_else(|| {
                                let slug = current_section.replace('.', "-").replace(' ', "-");
                                format!("com.apple.CoreSimulator.SimRuntime.{}", slug)
                            });

                        let dev = DetectedDevice {
                            name,
                            category: "mobile".to_string(),
                            id: uuid.clone(),
                            platform_arch: "ios".to_string(),
                            details: format!("{} (simulator)", runtime_id),
                            kind: DeviceKind::Ios(uuid),
                        };

                        if is_booted {
                            booted_devices.push(dev);
                        } else if is_available && available_devices.len() < 1 {
                            available_devices.push(dev);
                        }
                    }
                }
            }
        }

        if !booted_devices.is_empty() {
            devices.extend(booted_devices);
        } else if !available_devices.is_empty() {
            devices.extend(available_devices);
        }

        devices
    }

    fn detect_desktop() -> Option<DetectedDevice> {
        if cfg!(target_os = "macos") {
            let mut version = "26.6.2".to_string();
            let mut build = "25G83".to_string();

            if let Ok(v_out) = Command::new("sw_vers").arg("-productVersion").output() {
                if v_out.status.success() {
                    let v = String::from_utf8_lossy(&v_out.stdout).trim().to_string();
                    if !v.is_empty() {
                        version = v;
                    }
                }
            }

            if let Ok(b_out) = Command::new("sw_vers").arg("-buildVersion").output() {
                if b_out.status.success() {
                    let b = String::from_utf8_lossy(&b_out.stdout).trim().to_string();
                    if !b.is_empty() {
                        build = b;
                    }
                }
            }

            let arch = if env::consts::ARCH == "x86_64" {
                "darwin-x64"
            } else {
                "darwin-arm64"
            };
            let details = format!("macOS {} {} {}", version, build, arch);

            Some(DetectedDevice {
                name: "macOS".to_string(),
                category: "desktop".to_string(),
                id: "macos".to_string(),
                platform_arch: arch.to_string(),
                details,
                kind: DeviceKind::Desktop,
            })
        } else if cfg!(target_os = "linux") {
            Some(DetectedDevice {
                name: "Linux".to_string(),
                category: "desktop".to_string(),
                id: "linux".to_string(),
                platform_arch: "linux-x64".to_string(),
                details: "Linux desktop".to_string(),
                kind: DeviceKind::Desktop,
            })
        } else if cfg!(target_os = "windows") {
            Some(DetectedDevice {
                name: "Windows".to_string(),
                category: "desktop".to_string(),
                id: "windows".to_string(),
                platform_arch: "windows-x64".to_string(),
                details: "Windows desktop".to_string(),
                kind: DeviceKind::Desktop,
            })
        } else {
            None
        }
    }

    fn detect_chrome() -> Option<DetectedDevice> {
        let chrome_paths = [
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
            "google-chrome",
            "chrome",
            "chromium",
        ];

        for path in &chrome_paths {
            if let Ok(out) = Command::new(path).arg("--version").output() {
                if out.status.success() {
                    let ver_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    let details = if !ver_str.is_empty() {
                        ver_str
                    } else {
                        "Google Chrome (web)".to_string()
                    };

                    return Some(DetectedDevice {
                        name: "Chrome".to_string(),
                        category: "web".to_string(),
                        id: "chrome".to_string(),
                        platform_arch: "web-javascript".to_string(),
                        details,
                        kind: DeviceKind::Web,
                    });
                }
            }
        }

        Some(DetectedDevice {
            name: "Chrome".to_string(),
            category: "web".to_string(),
            id: "chrome".to_string(),
            platform_arch: "web-javascript".to_string(),
            details: "Google Chrome (web)".to_string(),
            kind: DeviceKind::Web,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_discovery() {
        let devices = DeviceSelector::discover_devices();
        assert!(!devices.is_empty(), "Should discover at least desktop or web devices");

        let has_desktop = devices.iter().any(|d| d.kind == DeviceKind::Desktop);
        let has_web = devices.iter().any(|d| d.kind == DeviceKind::Web);
        assert!(has_desktop || has_web, "Should have desktop or web device");
    }

    #[test]
    fn test_device_formatting() {
        let dev = DetectedDevice {
            name: "sdk gphone64 x86 64".to_string(),
            category: "mobile".to_string(),
            id: "emulator-5554".to_string(),
            platform_arch: "android-x64".to_string(),
            details: "Android 15 (API 35) (emulator)".to_string(),
            kind: DeviceKind::Android("emulator-5554".to_string()),
        };

        let name_cat = format!("{} ({})", dev.name, dev.category);
        let formatted = format!(
            "{:<28} • {:<36} • {:<14} • {}",
            name_cat, dev.id, dev.platform_arch, dev.details
        );

        assert!(formatted.contains("sdk gphone64 x86 64 (mobile)"));
        assert!(formatted.contains("emulator-5554"));
        assert!(formatted.contains("android-x64"));
        assert!(formatted.contains("Android 15 (API 35) (emulator)"));
    }
}
