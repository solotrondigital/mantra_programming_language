//! Anthropic Claude Provider implementation for Mantra AI — real HTTP calls via reqwest.

use crate::auth::AuthCredential;
use crate::error::{AIError, AIResult};
use crate::prompt::Prompt;
use crate::providers::{AIProvider, AIResponse};
use crate::streaming::{StreamChunk, StreamReceiver};
use crate::tokens::TokenUsage;
use serde::Deserialize;

pub struct AnthropicProvider;

impl AnthropicProvider {
    pub fn new() -> Self {
        Self
    }
}

// ── Response deserialization types ──────────────────────────────────────────

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Option<Vec<AnthropicContent>>,
    usage: Option<AnthropicUsage>,
    stop_reason: Option<String>,
}

#[derive(Deserialize)]
struct AnthropicContent {
    #[serde(rename = "type")]
    kind: String,
    text: Option<String>,
}

#[derive(Deserialize)]
struct AnthropicUsage {
    input_tokens: usize,
    output_tokens: usize,
}

// ── Provider implementation ─────────────────────────────────────────────────

impl AIProvider for AnthropicProvider {
    fn name(&self) -> &str {
        "anthropic"
    }

    fn generate(&self, model: &str, prompt: &Prompt, auth: &AuthCredential) -> AIResult<AIResponse> {
        let key = auth.resolve()?;
        if key.is_empty() {
            return Err(AIError::AuthenticationError("Anthropic API key is empty".to_string()));
        }

        let messages = vec![
            serde_json::json!({ "role": "user", "content": &prompt.user_prompt })
        ];

        let mut body = serde_json::json!({
            "model": model,
            "max_tokens": prompt.max_tokens.unwrap_or(8192),
            "messages": messages
        });

        if let Some(sys) = &prompt.system_instruction {
            body["system"] = serde_json::json!(sys);
        }

        let client = reqwest::blocking::Client::new();
        let http_resp = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .map_err(|e| AIError::ProviderError {
                provider: "anthropic".to_string(),
                message: format!("HTTP request failed: {}", e),
            })?;

        if !http_resp.status().is_success() {
            let status = http_resp.status();
            let err_body = http_resp.text().unwrap_or_default();
            return Err(AIError::ProviderError {
                provider: "anthropic".to_string(),
                message: format!("API error {}: {}", status, err_body),
            });
        }

        let anthro: AnthropicResponse = http_resp.json().map_err(|e| AIError::ProviderError {
            provider: "anthropic".to_string(),
            message: format!("Failed to parse response: {}", e),
        })?;

        let text = anthro
            .content
            .as_ref()
            .and_then(|cs| {
                cs.iter()
                    .find(|c| c.kind == "text")
                    .and_then(|c| c.text.clone())
            })
            .unwrap_or_default();

        let (prompt_tokens, completion_tokens) = anthro
            .usage
            .map(|u| (u.input_tokens, u.output_tokens))
            .unwrap_or_else(|| {
                let pt = prompt.user_prompt.len() / 4 + 1;
                let ct = text.len() / 4 + 1;
                (pt, ct)
            });

        let usage = TokenUsage::new(prompt_tokens, completion_tokens)
            .with_rates(0.003, 0.015);

        let mut res = AIResponse::new(model, self.name(), text);
        res.usage = usage;
        Ok(res)
    }

    fn stream(&self, model: &str, prompt: &Prompt, auth: &AuthCredential) -> AIResult<StreamReceiver> {
        // Delegate to generate and split into word chunks
        let resp = self.generate(model, prompt, auth)?;
        let words: Vec<String> = resp.text.split_whitespace().map(|w| format!("{} ", w)).collect();
        let mut chunks: Vec<StreamChunk> = words.into_iter().map(StreamChunk::new).collect();
        if let Some(last) = chunks.last_mut() {
            last.finish_reason = Some("stop".to_string());
        }
        if chunks.is_empty() {
            chunks.push(StreamChunk::final_chunk("", "stop"));
        }
        Ok(StreamReceiver::new(chunks))
    }
}

