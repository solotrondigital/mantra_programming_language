//! Project-level AI Configuration parser and runtime settings.
//! Supports `project.toml` or `mantra.toml` [ai] and [ai.models] sections.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::auth::AuthCredential;
use crate::error::{AIError, AIResult};
use crate::registry::{ModelRegistry, ModelSpec};

/// Project-level AI Configuration loaded from manifest files.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AIConfig {
    /// Default provider (e.g., "openai", "gemini", "anthropic")
    pub default_provider: String,
    /// Default model name
    pub default_model: String,
    /// Global timeout in milliseconds
    pub timeout_ms: u64,
    /// Maximum retries for temporary failures
    pub max_retries: u32,
    /// Global API keys / constants loaded from [ai] or [ai.keys]
    pub keys: HashMap<String, String>,
    /// Model specifications defined in the project
    pub models: HashMap<String, ModelSpec>,
}

impl AIConfig {
    pub fn new() -> Self {
        Self {
            default_provider: "openai".to_string(),
            default_model: "gpt-4o-mini".to_string(),
            timeout_ms: 30000,
            max_retries: 3,
            keys: HashMap::new(),
            models: HashMap::new(),
        }
    }

    /// Load AI configuration from `project.toml` or `mantra.toml` in project directory.
    pub fn load_from_dir(dir: &Path) -> AIResult<Self> {
        let project_toml = dir.join("project.toml");
        let mantra_toml = dir.join("mantra.toml");

        let target_path = if project_toml.exists() {
            project_toml
        } else if mantra_toml.exists() {
            mantra_toml
        } else {
            return Err(AIError::ConfigurationError(format!(
                "Neither project.toml nor mantra.toml was found in {}",
                dir.display()
            )));
        };

        // Also check if .env exists in dir and load variables into current process env
        let env_path = dir.join(".env");
        if env_path.exists() {
            if let Ok(content) = fs::read_to_string(&env_path) {
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() || trimmed.starts_with('#') {
                        continue;
                    }
                    if let Some((k, v)) = trimmed.split_once('=') {
                        let key = k.trim().trim_start_matches("export ").trim();
                        let val = v.trim().trim_matches('"').trim_matches('\'');
                        if !key.is_empty() && std::env::var(key).is_err() {
                            std::env::set_var(key, val);
                        }
                    }
                }
            }
        }

        let content = fs::read_to_string(&target_path).map_err(|e| {
            AIError::ConfigurationError(format!("Failed to read {}: {}", target_path.display(), e))
        })?;

        Self::parse_toml(&content)
    }

    /// Parse a TOML string containing `[ai]` configuration sections.
    pub fn parse_toml(content: &str) -> AIResult<Self> {
        let mut config = Self::new();
        let mut current_section = "";
        let mut current_model_name: Option<String> = None;

        let mut model_provider: Option<String> = None;
        let mut model_real_name: Option<String> = None;
        let mut model_api_key: Option<String> = None;
        let mut model_env_key: Option<String> = None;
        let mut model_temp: Option<f32> = None;
        let mut model_max_tokens: Option<usize> = None;

        let flush_model = |config: &mut AIConfig,
                           name: Option<String>,
                           provider: Option<String>,
                           real_name: Option<String>,
                           api_key: Option<String>,
                           env_key: Option<String>,
                           temp: Option<f32>,
                           max_tokens: Option<usize>| {
            if let Some(alias) = name {
                let prov = provider.unwrap_or_else(|| config.default_provider.clone());
                let m_name = real_name.unwrap_or_else(|| alias.clone());

                let prov_lower = prov.to_lowercase();
                let default_env_var = match prov_lower.as_str() {
                    "gemini" => "GEMINI_API_KEY",
                    "anthropic" => "ANTHROPIC_API_KEY",
                    _ => "OPENAI_API_KEY",
                };

                let auth = if let Some(key) = api_key {
                    AuthCredential::ApiKey(key)
                } else if let Some(env_val) = env_key {
                    // Check if env_val is a literal key (e.g. starts with "sk-", "AIza", or long key)
                    if env_val.starts_with("sk-")
                        || env_val.starts_with("AIza")
                        || (env_val.len() > 30 && !env_val.chars().all(|c| c.is_ascii_uppercase() || c == '_'))
                    {
                        AuthCredential::ApiKey(env_val)
                    } else if let Some(constant_key) = config.keys.get(&env_val) {
                        AuthCredential::ApiKey(constant_key.clone())
                    } else {
                        AuthCredential::EnvVar(env_val)
                    }
                } else if let Some(constant_key) = config.keys.get(&prov_lower)
                    .or_else(|| config.keys.get(&format!("{}_api_key", prov_lower)))
                    .or_else(|| config.keys.get(default_env_var))
                {
                    AuthCredential::ApiKey(constant_key.clone())
                } else {
                    AuthCredential::EnvVar(default_env_var.to_string())
                };

                let mut spec = ModelSpec::new(&alias, prov, m_name, auth);
                spec.default_temperature = temp;
                spec.default_max_tokens = max_tokens;
                config.models.insert(alias, spec);
            }
        };

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                let section = &trimmed[1..trimmed.len() - 1];

                // Flush previous model if changing section
                if current_section.starts_with("ai.models.") || current_section == "ai.model" {
                    flush_model(
                        &mut config,
                        current_model_name.take(),
                        model_provider.take(),
                        model_real_name.take(),
                        model_api_key.take(),
                        model_env_key.take(),
                        model_temp.take(),
                        model_max_tokens.take(),
                    );
                }

                current_section = section;
                if let Some(model_alias) = section.strip_prefix("ai.models.") {
                    current_model_name = Some(model_alias.to_string());
                }
                continue;
            }

            if let Some((k, v)) = trimmed.split_once('=') {
                let key = k.trim();
                let raw_val = v.trim();
                let val = if raw_val.starts_with('"') && raw_val.ends_with('"') && raw_val.len() >= 2 {
                    &raw_val[1..raw_val.len() - 1]
                } else {
                    raw_val
                };

                match current_section {
                    "ai" => match key {
                        "default_provider" | "provider" => config.default_provider = val.to_string(),
                        "default_model" | "model" => config.default_model = val.to_string(),
                        "timeout_ms" => {
                            if let Ok(num) = val.parse::<u64>() {
                                config.timeout_ms = num;
                            }
                        }
                        "max_retries" => {
                            if let Ok(num) = val.parse::<u32>() {
                                config.max_retries = num;
                            }
                        }
                        // Support constants/keys directly in [ai]
                        "api_key" | "apiKey" | "openai_api_key" | "gemini_api_key" | "anthropic_api_key"
                        | "OPENAI_API_KEY" | "GEMINI_API_KEY" | "ANTHROPIC_API_KEY" => {
                            config.keys.insert(key.to_string(), val.to_string());
                        }
                        _ => {}
                    },
                    "ai.keys" | "ai.constants" | "constants" => {
                        config.keys.insert(key.to_string(), val.to_string());
                    }
                    sec if sec.starts_with("ai.models.") => match key {
                        "provider" => model_provider = Some(val.to_string()),
                        "name" | "model" => model_real_name = Some(val.to_string()),
                        "api_key" | "apiKey" | "key" => model_api_key = Some(val.to_string()),
                        "env" | "api_key_env" | "apiKeyEnv" => model_env_key = Some(val.to_string()),
                        "temperature" => model_temp = val.parse::<f32>().ok(),
                        "max_tokens" => model_max_tokens = val.parse::<usize>().ok(),
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        // Flush last model block
        flush_model(
            &mut config,
            current_model_name.take(),
            model_provider.take(),
            model_real_name.take(),
            model_api_key.take(),
            model_env_key.take(),
            model_temp.take(),
            model_max_tokens.take(),
        );

        Ok(config)
    }

    /// Convert project configuration into an active ModelRegistry.
    pub fn build_registry(&self) -> ModelRegistry {
        let mut reg = ModelRegistry::new();
        for (_, spec) in &self.models {
            reg.register(spec.clone());
        }
        if !self.default_model.is_empty() && reg.get(&self.default_model).is_ok() {
            reg.set_default(&self.default_model);
        }
        reg
    }
}

