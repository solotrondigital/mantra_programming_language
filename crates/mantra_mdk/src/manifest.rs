use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct PackageManifest {
    pub name: String,
    pub version: String,
    pub edition: String,
    pub description: String,
    pub license: String,
    pub entry: String,
    pub project_type: Option<String>,
    pub service: Option<String>,
    pub dependencies: HashMap<String, String>,
}

impl Default for PackageManifest {
    fn default() -> Self {
        PackageManifest {
            name: "app".to_string(),
            version: "0.1.0".to_string(),
            edition: "2026".to_string(),
            description: "Mantra Application".to_string(),
            license: "MIT".to_string(),
            entry: "src/main.mantra".to_string(),
            project_type: None,
            service: None,
            dependencies: HashMap::new(),
        }
    }
}

impl PackageManifest {
    pub fn load_from_dir(dir: &Path) -> Result<Self, String> {
        let path = dir.join("mantra.toml");
        if !path.exists() {
            return Err(format!("Manifest file not found: {}", path.display()));
        }

        let content =
            fs::read_to_string(&path).map_err(|e| format!("Failed to read mantra.toml: {}", e))?;

        Self::parse(&content)
    }

    pub fn parse(content: &str) -> Result<Self, String> {
        let mut manifest = PackageManifest::default();
        let mut current_section = "";

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                current_section = &trimmed[1..trimmed.len() - 1];
                continue;
            }

            if let Some((key, val)) = trimmed.split_once('=') {
                let k = key.trim();
                let v = val.trim().trim_matches('"');

                match current_section {
                    "package" | "project" => match k {
                        "name" => manifest.name = v.to_string(),
                        "version" => manifest.version = v.to_string(),
                        "edition" => manifest.edition = v.to_string(),
                        "description" => manifest.description = v.to_string(),
                        "license" => manifest.license = v.to_string(),
                        "entry" => manifest.entry = v.to_string(),
                        "type" => manifest.project_type = Some(v.to_string()),
                        "service" => manifest.service = Some(v.to_string()),
                        _ => {}
                    },
                    "build" => match k {
                        "entry" => manifest.entry = v.to_string(),
                        _ => {}
                    },
                    "dependencies" => {
                        manifest.dependencies.insert(k.to_string(), v.to_string());
                    }
                    _ => {}
                }
            }
        }

        Ok(manifest)
    }

    pub fn to_toml_string(&self) -> String {
        let mut toml = String::new();
        toml.push_str("[package]\n");
        toml.push_str(&format!("name = \"{}\"\n", self.name));
        toml.push_str(&format!("version = \"{}\"\n", self.version));
        toml.push_str(&format!("edition = \"{}\"\n", self.edition));
        toml.push_str(&format!("description = \"{}\"\n", self.description));
        toml.push_str(&format!("license = \"{}\"\n\n", self.license));

        toml.push_str("[dependencies]\n");
        for (k, v) in &self.dependencies {
            toml.push_str(&format!("{} = \"{}\"\n", k, v));
        }
        toml.push('\n');

        toml.push_str("[build]\n");
        toml.push_str(&format!("entry = \"{}\"\n", self.entry));

        toml
    }

    pub fn save_to_dir(&self, dir: &Path) -> Result<(), String> {
        let path = dir.join("mantra.toml");
        fs::write(&path, self.to_toml_string())
            .map_err(|e| format!("Failed to write mantra.toml: {}", e))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Lockfile {
    pub version: u32,
    pub packages: HashMap<String, LockedPackage>,
}

#[derive(Debug, Clone)]
pub struct LockedPackage {
    pub name: String,
    pub version: String,
    pub checksum: String,
    pub source: String,
}

impl Lockfile {
    pub fn from_manifest(manifest: &PackageManifest) -> Self {
        let mut packages = HashMap::new();
        for (pkg_name, ver) in &manifest.dependencies {
            packages.insert(
                pkg_name.clone(),
                LockedPackage {
                    name: pkg_name.clone(),
                    version: ver.clone(),
                    // A checksum must come from the exact package archive. Do
                    // not synthesize a digest from the package name/version.
                    checksum: String::new(),
                    source:
                        "registry+https://github.com/solotrondigital/mantra_programming_language"
                            .to_string(),
                },
            );
        }
        Lockfile {
            version: 1,
            packages,
        }
    }

    /// Resolves archive digests from `vendor/cache/<name>-<version>.tar.gz`.
    /// Missing archives remain unhashed so audit can fail closed instead of
    /// manufacturing integrity metadata.
    pub fn from_manifest_with_cache(
        manifest: &PackageManifest,
        root: &Path,
    ) -> Result<Self, String> {
        let mut lockfile = Self::from_manifest(manifest);
        let cache_dir = root.join("vendor").join("cache");
        for package in lockfile.packages.values_mut() {
            let safe_name: String = package
                .name
                .chars()
                .map(|ch| {
                    if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
                        ch
                    } else {
                        '_'
                    }
                })
                .collect();
            let safe_version: String = package
                .version
                .chars()
                .map(|ch| {
                    if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
                        ch
                    } else {
                        '_'
                    }
                })
                .collect();
            let archive = cache_dir.join(format!("{}-{}.tar.gz", safe_name, safe_version));
            if !archive.exists() {
                continue;
            }
            package.checksum = sha256_file(&archive)?;
        }
        Ok(lockfile)
    }

    pub fn verify_archive_checksum(&self, package_name: &str, root: &Path) -> Result<bool, String> {
        let Some(package) = self.packages.get(package_name) else {
            return Ok(false);
        };
        if package.checksum.len() != 64 || !package.checksum.bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Ok(false);
        }
        let safe_name: String = package
            .name
            .chars()
            .map(|ch| {
                if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
                    ch
                } else {
                    '_'
                }
            })
            .collect();
        let safe_version: String = package
            .version
            .chars()
            .map(|ch| {
                if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
                    ch
                } else {
                    '_'
                }
            })
            .collect();
        let archive = root
            .join("vendor")
            .join("cache")
            .join(format!("{}-{}.tar.gz", safe_name, safe_version));
        if !archive.is_file() {
            return Ok(false);
        }
        Ok(sha256_file(&archive)?.eq_ignore_ascii_case(&package.checksum))
    }

    pub fn load_from_dir(dir: &Path) -> Result<Self, String> {
        let path = dir.join("mantra.lock");
        let content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
        let mut lockfile = Lockfile::default();
        let mut current: Option<LockedPackage> = None;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line == "[[package]]" {
                if let Some(package) = current.take() {
                    lockfile.packages.insert(package.name.clone(), package);
                }
                current = Some(LockedPackage {
                    name: String::new(),
                    version: String::new(),
                    checksum: String::new(),
                    source: String::new(),
                });
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let key = key.trim();
            let value = value.trim().trim_matches('"').to_string();
            if key == "version" && current.is_none() {
                lockfile.version = value
                    .parse()
                    .map_err(|_| "Invalid lockfile format version".to_string())?;
                continue;
            }
            let Some(package) = current.as_mut() else {
                continue;
            };
            match key {
                "name" => package.name = value,
                "version" => package.version = value,
                "checksum" => package.checksum = value,
                "source" => package.source = value,
                _ => {}
            }
        }
        if let Some(package) = current {
            lockfile.packages.insert(package.name.clone(), package);
        }
        if lockfile.version != 1 {
            return Err(format!(
                "Unsupported mantra.lock format version {}",
                lockfile.version
            ));
        }
        Ok(lockfile)
    }

    pub fn to_lock_string(&self) -> String {
        let mut out = String::new();
        out.push_str("# This file is automatically generated by Mantra MDK. Do not edit.\n");
        out.push_str("version = 1\n\n");
        for (name, pkg) in &self.packages {
            out.push_str("[[package]]\n");
            out.push_str(&format!("name = \"{}\"\n", name));
            out.push_str(&format!("version = \"{}\"\n", pkg.version));
            out.push_str(&format!("source = \"{}\"\n", pkg.source));
            out.push_str(&format!("checksum = \"{}\"\n\n", pkg.checksum));
        }
        out
    }

    pub fn save_to_dir(&self, dir: &Path) -> Result<(), String> {
        let path = dir.join("mantra.lock");
        fs::write(&path, self.to_lock_string())
            .map_err(|e| format!("Failed to write mantra.lock: {}", e))
    }

    pub fn verify_with_manifest(&self, manifest: &PackageManifest) -> Result<bool, String> {
        for (dep_name, dep_ver) in &manifest.dependencies {
            if let Some(locked) = self.packages.get(dep_name) {
                if &locked.version != dep_ver {
                    return Err(format!(
                        "Lockfile version mismatch for '{}': locked at {}, manifest requests {}",
                        dep_name, locked.version, dep_ver
                    ));
                }
            } else {
                return Err(format!(
                    "Dependency '{}' in mantra.toml is not locked in mantra.lock",
                    dep_name
                ));
            }
        }
        Ok(true)
    }
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let mut file = fs::File::open(path)
        .map_err(|err| format!("Failed to open package archive {}: {err}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 16 * 1024];
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|err| format!("Failed to read package archive {}: {err}", path.display()))?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}
