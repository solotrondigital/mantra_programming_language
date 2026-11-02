//! Unified AI Provider Trait and Response Types.

pub mod anthropic;
pub mod gemini;
pub mod manager;
pub mod openai;

use crate::auth::AuthCredential;
use crate::error::AIResult;
use crate::prompt::Prompt;
use crate::schema::JsonValue;
use crate::streaming::StreamReceiver;
use crate::tokens::TokenUsage;

/// Unified response from an AI model invocation.
#[derive(Debug, Clone, PartialEq)]
pub struct AIResponse {
    /// Model name that fulfilled the request
    pub model: String,
    /// Provider name (e.g., "openai", "gemini", "anthropic")
    pub provider: String,
    /// Raw generated text response
    pub text: String,
    /// Parsed structured output, if requested
    pub structured: Option<JsonValue>,
    /// Token usage and cost metadata
    pub usage: TokenUsage,
    /// Stop reason ("stop", "max_tokens", etc.)
    pub finish_reason: String,
}

impl AIResponse {
    pub fn new(
        model: impl Into<String>,
        provider: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            model: model.into(),
            provider: provider.into(),
            text: text.into(),
            structured: None,
            usage: TokenUsage::default(),
            finish_reason: "stop".to_string(),
        }
    }
}

/// Common trait implemented by every AI provider adapter (OpenAI, Gemini, Anthropic, Custom).
pub trait AIProvider: Send + Sync {
    /// Canonical provider name (e.g. "openai", "gemini", "anthropic")
    fn name(&self) -> &str;

    /// Generate a completion response for the given prompt and model.
    fn generate(&self, model: &str, prompt: &Prompt, auth: &AuthCredential)
        -> AIResult<AIResponse>;

    /// Stream completion chunks for the given prompt and model.
    fn stream(
        &self,
        model: &str,
        prompt: &Prompt,
        auth: &AuthCredential,
    ) -> AIResult<StreamReceiver>;
}
