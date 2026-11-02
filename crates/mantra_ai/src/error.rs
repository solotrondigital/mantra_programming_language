//! Errors for mantra_ai operations.

use std::fmt;

/// Categorized AI errors across providers and execution phases.
#[derive(Debug, Clone, PartialEq)]
pub enum AIError {
    /// Authentication failure or missing credentials
    AuthenticationError(String),
    /// Provider connection or network error
    ProviderError { provider: String, message: String },
    /// Request timeout
    Timeout(String),
    /// Rate limit or quota exceeded
    RateLimitExceeded {
        provider: String,
        retry_after_secs: Option<u64>,
    },
    /// Invalid prompt, parameters, or model specification
    InvalidRequest(String),
    /// Schema validation error for structured output
    SchemaValidationError {
        expected: String,
        actual: String,
        details: String,
    },
    /// Token limit exceeded
    TokenLimitExceeded { limit: usize, requested: usize },
    /// Model not found in registry
    ModelNotFound(String),
    /// Configuration or environment error
    ConfigurationError(String),
}

impl fmt::Display for AIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AIError::AuthenticationError(msg) => write!(f, "AI Authentication Error: {}", msg),
            AIError::ProviderError { provider, message } => {
                write!(f, "AI Provider Error [{}]: {}", provider, message)
            }
            AIError::Timeout(msg) => write!(f, "AI Request Timeout: {}", msg),
            AIError::RateLimitExceeded {
                provider,
                retry_after_secs,
            } => {
                if let Some(secs) = retry_after_secs {
                    write!(
                        f,
                        "AI Rate Limit Exceeded for [{}] (retry after {}s)",
                        provider, secs
                    )
                } else {
                    write!(f, "AI Rate Limit Exceeded for [{}]", provider)
                }
            }
            AIError::InvalidRequest(msg) => write!(f, "AI Invalid Request: {}", msg),
            AIError::SchemaValidationError {
                expected,
                actual,
                details,
            } => {
                write!(
                    f,
                    "AI Schema Validation Error: expected '{}', got '{}' ({})",
                    expected, actual, details
                )
            }
            AIError::TokenLimitExceeded { limit, requested } => {
                write!(
                    f,
                    "AI Token Limit Exceeded: requested {} tokens, limit is {}",
                    requested, limit
                )
            }
            AIError::ModelNotFound(model) => write!(f, "AI Model Not Found in Registry: {}", model),
            AIError::ConfigurationError(msg) => write!(f, "AI Configuration Error: {}", msg),
        }
    }
}

impl std::error::Error for AIError {}

/// Result type alias for mantra_ai operations.
pub type AIResult<T> = Result<T, AIError>;
