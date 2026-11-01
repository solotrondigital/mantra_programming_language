//! Authentication and Secret Management for AI providers.

use std::env;
use crate::error::{AIError, AIResult};

/// Represents an API authentication credential.
#[derive(Debug, Clone, PartialEq)]
pub enum AuthCredential {
    /// Direct API Key string
    ApiKey(String),
    /// Bearer Token string
    BearerToken(String),
    /// Loaded dynamically from an environment variable
    EnvVar(String),
}

impl AuthCredential {
    /// Resolves the actual secret string at runtime.
    pub fn resolve(&self) -> AIResult<String> {
        match self {
            AuthCredential::ApiKey(key) => {
                if key.trim().is_empty() {
                    Err(AIError::AuthenticationError("API key cannot be empty".to_string()))
                } else {
                    Ok(key.clone())
                }
            }
            AuthCredential::BearerToken(token) => {
                if token.trim().is_empty() {
                    Err(AIError::AuthenticationError("Bearer token cannot be empty".to_string()))
                } else {
                    Ok(token.clone())
                }
            }
            AuthCredential::EnvVar(var_name) => {
                // If the user placed a literal API key in the env field
                // (e.g. starts with "sk-", "AIza", or is a long key string)
                if var_name.starts_with("sk-")
                    || var_name.starts_with("AIza")
                    || (var_name.len() > 30 && !var_name.chars().all(|c| c.is_ascii_uppercase() || c == '_'))
                {
                    return Ok(var_name.clone());
                }

                env::var(var_name).map_err(|_| {
                    AIError::AuthenticationError(format!(
                        "Environment variable '{}' is not set or accessible",
                        var_name
                    ))
                })
            }
        }
    }

    /// Mask the secret key for logging and security display.
    pub fn masked(&self) -> String {
        if let Ok(secret) = self.resolve() {
            if secret.len() <= 8 {
                "***".to_string()
            } else {
                format!("{}...{}", &secret[..4], &secret[secret.len() - 4..])
            }
        } else {
            "***[UNRESOLVED]***".to_string()
        }
    }
}

