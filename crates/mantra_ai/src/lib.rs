//! # mantra_ai
//!
//! Official Native AI Integration Runtime for the Mantra Programming Language (MVP 1).
//!
//! This crate provides:
//! - **Model Registry**: Configure and look up models dynamically (`ModelRegistry`, `ModelSpec`).
//! - **Unified Provider Interface**: Pluggable providers (`AIProvider`) with production adapters for OpenAI, Gemini, and Anthropic.
//! - **Secure Authentication**: Environment variables (`AuthCredential::EnvVar`) and API keys with automatic secret masking.
//! - **Prompt Engine**: Templating with variable interpolation and system instructions (`PromptTemplate`, `Prompt`).
//! - **Structured Output**: Schema validation against typed objects (`StructuredSchema`, `JsonValue`).
//! - **Streaming Engine**: Real-time chunked responses (`StreamReceiver`, `StreamChunk`).
//! - **Token Manager**: Token usage counting, quota limits, and cost estimation (`TokenManager`, `TokenUsage`).
//! - **Project Configuration**: Load AI models and settings from `project.toml` or `mantra.toml`.

pub mod auth;
pub mod client;
pub mod config;
pub mod error;
pub mod prompt;
pub mod providers;
pub mod registry;
pub mod schema;
pub mod streaming;
pub mod tokens;

pub use auth::AuthCredential;
pub use client::{AI, AIModelHandle};
pub use config::AIConfig;
pub use error::{AIError, AIResult};
pub use prompt::{Prompt, PromptTemplate};
pub use providers::manager::ProviderManager;
pub use providers::{AIProvider, AIResponse};
pub use registry::{ModelRegistry, ModelSpec};
pub use schema::{JsonValue, SchemaType, StructuredSchema};
pub use streaming::{StreamChunk, StreamReceiver};
pub use tokens::{TokenManager, TokenUsage};

/// Convenience helper to create an environment variable credential.
pub fn env(var_name: &str) -> AuthCredential {
    AuthCredential::EnvVar(var_name.to_string())
}

/// Convenience helper to create a direct API key credential.
pub fn api_key(key: &str) -> AuthCredential {
    AuthCredential::ApiKey(key.to_string())
}

