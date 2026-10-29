use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct PackageManifest {
    pub name: String,
    pub version: String,
    pub edition: String,
    pub description: String,
    pub license: String,
    pub entry: String,
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

        let content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read mantra.toml: {}", e))?;

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
                    "package" => match k {
                        "name" => manifest.name = v.to_string(),
                        "version" => manifest.version = v.to_string(),
                        "edition" => manifest.edition = v.to_string(),
                        "description" => manifest.description = v.to_string(),
                        "license" => manifest.license = v.to_string(),
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

