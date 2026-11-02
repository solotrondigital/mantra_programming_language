//! Model Registry: Register, configure, and resolve AI models.

use crate::auth::AuthCredential;
use crate::error::{AIError, AIResult};
use std::collections::HashMap;

/// Specification of a registered AI model.
#[derive(Debug, Clone, PartialEq)]
pub struct ModelSpec {
    /// Model alias/identifier in application (e.g., "assistant", "code_model")
    pub alias: String,
    /// Provider name (e.g. "openai", "gemini", "anthropic")
    pub provider: String,
    /// Real engine/model identifier (e.g. "gpt-4o", "gemini-1.5-pro", "claude-3-5-sonnet")
    pub model_name: String,
    /// Model authentication credential
    pub auth: AuthCredential,
    /// Default temperature
    pub default_temperature: Option<f32>,
    /// Default max tokens
    pub default_max_tokens: Option<usize>,
    /// Optional context window limit
    pub context_window: Option<usize>,
}

impl ModelSpec {
    pub fn new(
        alias: impl Into<String>,
        provider: impl Into<String>,
        model_name: impl Into<String>,
        auth: AuthCredential,
    ) -> Self {
        Self {
            alias: alias.into(),
            provider: provider.into().to_lowercase(),
            model_name: model_name.into(),
            auth,
            default_temperature: None,
            default_max_tokens: None,
            context_window: None,
        }
    }

    pub fn with_temperature(mut self, temp: f32) -> Self {
        self.default_temperature = Some(temp);
        self
    }

    pub fn with_max_tokens(mut self, tokens: usize) -> Self {
        self.default_max_tokens = Some(tokens);
        self
    }
}

/// Central registry storing configured models for project use.
#[derive(Debug, Clone, Default)]
pub struct ModelRegistry {
    models: HashMap<String, ModelSpec>,
    default_model: Option<String>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new model configuration.
    pub fn register(&mut self, spec: ModelSpec) {
        let key = spec.alias.clone();
        if self.default_model.is_none() {
            self.default_model = Some(key.clone());
        }
        self.models.insert(key, spec);
    }

    /// Retrieve model specification by alias.
    pub fn get(&self, alias: &str) -> AIResult<&ModelSpec> {
        self.models.get(alias).ok_or_else(|| {
            AIError::ModelNotFound(format!(
                "Model '{}' not found in registry. Registered: {:?}",
                alias,
                self.models.keys().collect::<Vec<_>>()
            ))
        })
    }

    /// Set default model alias.
    pub fn set_default(&mut self, alias: impl Into<String>) {
        self.default_model = Some(alias.into());
    }

    /// Get default model specification.
    pub fn get_default(&self) -> AIResult<&ModelSpec> {
        match &self.default_model {
            Some(alias) => self.get(alias),
            None => Err(AIError::ModelNotFound(
                "No default model set in registry".to_string(),
            )),
        }
    }

    /// Number of registered models.
    pub fn len(&self) -> usize {
        self.models.len()
    }

    pub fn is_empty(&self) -> bool {
        self.models.is_empty()
    }
}
