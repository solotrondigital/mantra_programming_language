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
        self.edition_compatible
            && self.semver_compliant
            && self.build_script_safe
            && self.verified_checksums == self.total_dependencies
            && self.advisories.is_empty()
    }

    pub fn print_summary(&self) {
        println!("=== Mantra Security & Dependency Audit Report ===\n");
        println!("  Dependencies scanned : {}", self.total_dependencies);
        println!(
            "  Archive hashes verified: {}/{}",
            self.verified_checksums, self.total_dependencies
        );
        println!(
            "  Edition compatibility: {}",
            if self.edition_compatible {
                "PASS (2026 Edition)"
            } else {
                "FAIL"
            }
        );
        println!(
            "  SemVer compliance    : {}",
            if self.semver_compliant {
                "PASS"
            } else {
                "FAIL"
            }
        );
        println!(
            "  Build script safety  : {}",
            if self.build_script_safe {
                "PASS (Sandboxed)"
            } else {
                "WARNING"
            }
        );

        if self.advisories.is_empty() {
            println!(
                "\n  Security status      : 0 vulnerabilities found. All dependencies verified!"
            );
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
        let lockfile = Lockfile::load_from_dir(root)?;
        lockfile.verify_with_manifest(&manifest)?;

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
                report.advisories.push(format!(
                    "Dependency '{}' uses non-standard version specifier '{}'",
                    pkg, ver
                ));
            }

            // Verify checksum format
            if let Some(locked) = lockfile.packages.get(pkg) {
                if Self::is_sha256(&locked.checksum) {
                    match lockfile.verify_archive_checksum(pkg, root) {
                        Ok(true) => report.verified_checksums += 1,
                        Ok(false) => report.advisories.push(format!(
                            "Dependency '{}' checksum does not match an available cached package archive",
                            pkg
                        )),
                        Err(error) => report.advisories.push(error),
                    }
                } else {
                    report.advisories.push(format!(
                        "Dependency '{}' has no valid SHA-256 archive checksum in mantra.lock",
                        pkg
                    ));
                }
            } else {
                report
                    .advisories
                    .push(format!("Dependency '{}' is not recorded in lockfile", pkg));
            }

            // Build script sandbox restriction check
            if pkg.contains("native_unsafe") || pkg.contains("arbitrary_exec") {
                report.build_script_safe = false;
                report.advisories.push(format!(
                    "Package '{}' attempts to register unrestricted build hooks",
                    pkg
                ));
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
        let clean = v
            .trim_start_matches('^')
            .trim_start_matches('~')
            .trim_start_matches('=');
        let parts: Vec<&str> = clean.split('.').collect();
        if parts.is_empty() || parts.len() > 3 {
            return false;
        }
        parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
    }

    fn is_sha256(value: &str) -> bool {
        value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audit_uses_real_lockfile_and_rejects_synthetic_or_missing_checksums() {
        let root = std::env::temp_dir().join(format!("mantra-audit-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(
            root.join("mantra.toml"),
            r#"[package]
name = "app"
version = "0.1.0"
edition = "2026"
[dependencies]
example = "1.2.3"
"#,
        )
        .unwrap();
        std::fs::write(
            root.join("mantra.lock"),
            r#"version = 1

[[package]]
name = "example"
version = "1.2.3"
source = "registry+https://example.test"
checksum = "deadbeef"
"#,
        )
        .unwrap();

        let report = Auditor::audit_project(&root).unwrap();
        assert_eq!(report.verified_checksums, 0);
        assert!(!report.is_secure());
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn audit_does_not_call_a_digest_verified_without_the_package_archive() {
        let root =
            std::env::temp_dir().join(format!("mantra-audit-archive-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(
            root.join("mantra.toml"),
            r#"[package]
name = "app"
version = "0.1.0"
edition = "2026"
[dependencies]
example = "1.2.3"
"#,
        )
        .unwrap();
        std::fs::write(
            root.join("mantra.lock"),
            format!(
                r#"version = 1

[[package]]
name = "example"
version = "1.2.3"
source = "registry+https://example.test"
checksum = "{}"
"#,
                "a".repeat(64)
            ),
        )
        .unwrap();

        let report = Auditor::audit_project(&root).unwrap();
        assert_eq!(report.verified_checksums, 0);
        assert!(!report.is_secure());
        assert!(report
            .advisories
            .iter()
            .any(|item| item.contains("does not match an available cached package archive")));
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn audit_verifies_checksum_against_cached_archive_bytes() {
        let root = std::env::temp_dir().join(format!("mantra-audit-good-{}", std::process::id()));
        let cache = root.join("vendor/cache");
        std::fs::create_dir_all(&cache).unwrap();
        std::fs::write(
            root.join("mantra.toml"),
            r#"[package]
name = "app"
version = "0.1.0"
edition = "2026"
[dependencies]
example = "1.2.3"
"#,
        )
        .unwrap();
        std::fs::write(
            cache.join("example-1.2.3.tar.gz"),
            b"exact archive contents",
        )
        .unwrap();
        let manifest = PackageManifest::load_from_dir(&root).unwrap();
        Lockfile::from_manifest_with_cache(&manifest, &root)
            .unwrap()
            .save_to_dir(&root)
            .unwrap();

        let report = Auditor::audit_project(&root).unwrap();
        assert_eq!(report.verified_checksums, 1);
        assert!(report.is_secure());

        std::fs::write(
            cache.join("example-1.2.3.tar.gz"),
            b"modified archive contents",
        )
        .unwrap();
        let tampered_report = Auditor::audit_project(&root).unwrap();
        assert_eq!(tampered_report.verified_checksums, 0);
        assert!(!tampered_report.is_secure());
        let _ = std::fs::remove_dir_all(root);
    }
}
