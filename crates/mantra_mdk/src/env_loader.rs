// ==============================================================================
// mantra_mdk::env_loader — Multi-Environment Defines Loader (.env*)
// ==============================================================================

use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct EnvLoader;

impl EnvLoader {
    /// Loads key-value pairs from an environment file (.env, .env.development, etc.)
    /// and injects them into the current process environment via `std::env::set_var`.
    pub fn load_file(file_path: &Path) -> Result<HashMap<String, String>, String> {
        if !file_path.exists() {
            return Err(format!("Environment file not found: '{}'", file_path.display()));
        }

        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read environment file '{}': {}", file_path.display(), e))?;

        let mut vars = HashMap::new();
        for line in content.lines() {
            let trimmed = line.trim();
            // Skip empty lines and comment lines
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            if let Some((raw_key, raw_val)) = trimmed.split_once('=') {
                let key = raw_key.trim().to_string();
                let mut val = raw_val.trim().to_string();

                // Strip surrounding single or double quotes
                if (val.starts_with('"') && val.ends_with('"')) || (val.starts_with('\'') && val.ends_with('\'')) {
                    if val.len() >= 2 {
                        val = val[1..val.len() - 1].to_string();
                    }
                }

                // Expand simple ${VAR} templates from already set environment variables
                if val.starts_with("${") && val.ends_with('}') && val.len() > 3 {
                    let inner_var = &val[2..val.len() - 1];
                    if let Ok(resolved) = std::env::var(inner_var) {
                        val = resolved;
                    }
                }

                std::env::set_var(&key, &val);
                vars.insert(key, val);
            }
        }

        Ok(vars)
    }

    /// Prints a formatted summary of loaded variables.
    pub fn print_summary(source_path: &str, vars: &HashMap<String, String>) {
        println!("Loaded defines from '{}' ({} variables):", source_path, vars.len());
        let mut keys: Vec<&String> = vars.keys().collect();
        keys.sort();
        for k in keys {
            let v = &vars[k];
            let display_val = if k.contains("PASS") || k.contains("SECRET") || k.contains("TOKEN") {
                "********"
            } else {
                v.as_str()
            };
            println!("  • {}={}", k, display_val);
        }
    }
}

