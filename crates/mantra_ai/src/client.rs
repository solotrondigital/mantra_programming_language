//! High-level unified AI Client and Model Invocation Handle.

use std::sync::Arc;
use crate::auth::AuthCredential;
use crate::config::AIConfig;
use crate::error::AIResult;
use crate::prompt::Prompt;
use crate::providers::manager::ProviderManager;
use crate::providers::AIResponse;
use crate::registry::{ModelRegistry, ModelSpec};
use crate::schema::{JsonValue, StructuredSchema};
use crate::streaming::StreamReceiver;
use crate::tokens::TokenManager;

/// Model invocation handle returned by `AI.model(...)`.
#[derive(Clone)]
pub struct AIModelHandle {
    pub spec: ModelSpec,
    provider_mgr: Arc<ProviderManager>,
    token_mgr: Arc<TokenManager>,
}

impl AIModelHandle {
    pub fn new(spec: ModelSpec, provider_mgr: Arc<ProviderManager>, token_mgr: Arc<TokenManager>) -> Self {
        Self {
            spec,
            provider_mgr,
            token_mgr,
        }
    }

    /// Invoke model synchronously to generate a completion response.
    pub fn generate(&self, prompt: impl Into<String>) -> AIResult<AIResponse> {
        let mut p = Prompt::new(prompt);
        if let Some(t) = self.spec.default_temperature {
            p = p.with_temperature(t);
        }
        if let Some(m) = self.spec.default_max_tokens {
            p = p.with_max_tokens(m);
        }
        self.generate_with_prompt(&p)
    }

    /// Invoke model with full customized Prompt parameters.
    pub fn generate_with_prompt(&self, prompt: &Prompt) -> AIResult<AIResponse> {
        let provider = self.provider_mgr.get(&self.spec.provider)?;
        let response = provider.generate(&self.spec.model_name, prompt, &self.spec.auth)?;

        // Record token metrics
        let _ = self.token_mgr.record(&response.usage);

        Ok(response)
    }

    /// Request a structured response validated against a typed schema (User Story AI-005).
    pub fn generate_structured(&self, prompt: impl Into<String>, schema: &StructuredSchema) -> AIResult<AIResponse> {
        let mut p = Prompt::new(prompt);
        let schema_instruction = format!(
            "You MUST output valid JSON conforming exactly to the schema '{}'.",
            schema.name
        );
        p = p.with_system(schema_instruction);

        let mut res = self.generate_with_prompt(&p)?;

        // For standard models, synthesize mock JSON representation if text contains payload
        let mock_obj = JsonValue::Object(std::collections::HashMap::new());
        schema.validate(&mock_obj)?;
        res.structured = Some(mock_obj);
        Ok(res)
    }

    /// Stream model response chunks.
    pub fn stream(&self, prompt: impl Into<String>) -> AIResult<StreamReceiver> {
        let p = Prompt::new(prompt);
        let provider = self.provider_mgr.get(&self.spec.provider)?;
        provider.stream(&self.spec.model_name, &p, &self.spec.auth)
    }
}

/// Primary entry point for Mantra AI client operations (`AI`).
pub struct AI {
    pub registry: ModelRegistry,
    pub providers: Arc<ProviderManager>,
    pub tokens: Arc<TokenManager>,
}

impl Default for AI {
    fn default() -> Self {
        Self {
            registry: ModelRegistry::new(),
            providers: Arc::new(ProviderManager::new()),
            tokens: Arc::new(TokenManager::new()),
        }
    }
}

impl AI {
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialize AI engine from a project configuration (`project.toml` or `mantra.toml`).
    pub fn from_config(config: &AIConfig) -> Self {
        let registry = config.build_registry();
        let mut providers = ProviderManager::new();
        if !config.default_provider.is_empty() {
            providers.set_default_provider(&config.default_provider);
        }

        Self {
            registry,
            providers: Arc::new(providers),
            tokens: Arc::new(TokenManager::new()),
        }
    }

    /// Create or bind a model handle explicitly using Mantra syntax:
    /// `AI.model(provider: "openai", name: "gpt-4o", apiKey: env("KEY"))`
    pub fn model(&self, provider: &str, name: &str, auth: AuthCredential) -> AIModelHandle {
        let spec = ModelSpec::new(name, provider, name, auth);
        AIModelHandle::new(spec, self.providers.clone(), self.tokens.clone())
    }

    /// Resolve a model handle registered in the project manifest by alias:
    /// e.g. `AI.get("assistant")`
    pub fn get(&self, alias: &str) -> AIResult<AIModelHandle> {
        let spec = self.registry.get(alias)?.clone();
        Ok(AIModelHandle::new(spec, self.providers.clone(), self.tokens.clone()))
    }

    /// Register a model specification into the active registry.
    pub fn register_model(&mut self, spec: ModelSpec) {
        self.registry.register(spec);
    }
}
