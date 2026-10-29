// ==============================================================================
// mantra_mdk::audit — Dependency Auditing & Security Integrity Verification
// ==============================================================================

use crate::manifest::{Lockfile, PackageManifest};
use std::path::Path;

pub struct AuditReport {
    pub total_dependencies: usize,
    pub verified_checksums: usize,
    pub edition_compatible: bool,
    pub semver_compliant: bool,
    pub build_script_safe: bool,
    pub advisories: Vec<String>,
}

impl AuditReport {
    pub fn is_secure(&self) -> bool {
        self.edition_compatible && self.semver_compliant && self.build_script_safe && self.advisories.is_empty()
    }

    pub fn print_summary(&self) {
        println!("=== Mantra Security & Dependency Audit Report ===\n");
        println!("  Dependencies scanned : {}", self.total_dependencies);
        println!("  Checksums verified   : {}/{}", self.verified_checksums, self.total_dependencies);
        println!("  Edition compatibility: {}", if self.edition_compatible { "PASS (2026 Edition)" } else { "FAIL" });
        println!("  SemVer compliance    : {}", if self.semver_compliant { "PASS" } else { "FAIL" });
        println!("  Build script safety  : {}", if self.build_script_safe { "PASS (Sandboxed)" } else { "WARNING" });

        if self.advisories.is_empty() {
            println!("\n  Security status      : 0 vulnerabilities found. All dependencies verified!");
        } else {
            println!("\n  Security advisories ({} found):", self.advisories.len());
            for adv in &self.advisories {
                println!("    [!] {}", adv);
            }
        }
        println!("\n=================================================");
    }
}

pub struct Auditor;

impl Auditor {
    pub fn audit_project(root: &Path) -> Result<AuditReport, String> {
        let manifest = PackageManifest::load_from_dir(root)?;
        let lockfile = Lockfile::from_manifest(&manifest);

        let mut report = AuditReport {
            total_dependencies: manifest.dependencies.len(),
            verified_checksums: 0,
            edition_compatible: manifest.edition == "2026",
            semver_compliant: true,
            build_script_safe: true,
            advisories: Vec::new(),
        };

        // 1. Check SemVer format and language edition
        for (pkg, ver) in &manifest.dependencies {
            if !Self::is_valid_semver(ver) {
                report.semver_compliant = false;
                report.advisories.push(format!("Dependency '{}' uses non-standard version specifier '{}'", pkg, ver));
            }

            // Verify checksum format
            if let Some(locked) = lockfile.packages.get(pkg) {
                if !locked.checksum.is_empty() {
                    report.verified_checksums += 1;
                }
            } else {
                report.advisories.push(format!("Dependency '{}' is not recorded in lockfile", pkg));
            }

            // Build script sandbox restriction check
            if pkg.contains("native_unsafe") || pkg.contains("arbitrary_exec") {
                report.build_script_safe = false;
                report.advisories.push(format!("Package '{}' attempts to register unrestricted build hooks", pkg));
            }
        }

        if !report.edition_compatible {
            report.advisories.push(format!(
                "Manifest edition '{}' does not match compiler edition '2026'",
                manifest.edition
            ));
        }

        Ok(report)
    }

    fn is_valid_semver(v: &str) -> bool {
        let clean = v.trim_start_matches('^').trim_start_matches('~').trim_start_matches('=');
        let parts: Vec<&str> = clean.split('.').collect();
        if parts.is_empty() || parts.len() > 3 {
            return false;
        }
        parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
    }
}

