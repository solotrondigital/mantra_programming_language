//! Provider Manager: Multi-provider orchestration and dynamic registration.

use crate::error::{AIError, AIResult};
use crate::providers::anthropic::AnthropicProvider;
use crate::providers::gemini::GeminiProvider;
use crate::providers::openai::OpenAIProvider;
use crate::providers::AIProvider;
use std::collections::HashMap;
use std::sync::Arc;

/// Central manager that registers, resolves, and manages AI providers.
#[derive(Clone)]
pub struct ProviderManager {
    providers: HashMap<String, Arc<dyn AIProvider>>,
    default_provider: String,
}

impl Default for ProviderManager {
    fn default() -> Self {
        let mut mgr = Self {
            providers: HashMap::new(),
            default_provider: "openai".to_string(),
        };

        // Register default production providers
        mgr.register(Arc::new(OpenAIProvider::new()));
        mgr.register(Arc::new(GeminiProvider::new()));
        mgr.register(Arc::new(AnthropicProvider::new()));
        mgr
    }
}

impl ProviderManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a custom or standard provider adapter.
    pub fn register(&mut self, provider: Arc<dyn AIProvider>) {
        self.providers
            .insert(provider.name().to_lowercase(), provider);
    }

    /// Set the default fallback provider name.
    pub fn set_default_provider(&mut self, name: impl Into<String>) {
        self.default_provider = name.into().to_lowercase();
    }

    /// Resolve a provider by name.
    pub fn get(&self, name: &str) -> AIResult<Arc<dyn AIProvider>> {
        let key = name.to_lowercase();
        self.providers
            .get(&key)
            .cloned()
            .ok_or_else(|| AIError::ProviderError {
                provider: name.to_string(),
                message: format!(
                    "Provider '{}' is not registered. Available: {:?}",
                    name,
                    self.providers.keys().collect::<Vec<_>>()
                ),
            })
    }

    /// Get default configured provider.
    pub fn default_provider(&self) -> AIResult<Arc<dyn AIProvider>> {
        self.get(&self.default_provider)
    }

    /// List all registered provider names.
    pub fn list_providers(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }
}
