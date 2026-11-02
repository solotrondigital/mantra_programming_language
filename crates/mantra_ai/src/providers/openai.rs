//! OpenAI Provider implementation for Mantra AI — real HTTP calls via reqwest.

use crate::auth::AuthCredential;
use crate::error::{AIError, AIResult};
use crate::prompt::Prompt;
use crate::providers::{AIProvider, AIResponse};
use crate::streaming::{StreamChunk, StreamReceiver};
use crate::tokens::TokenUsage;
use serde::Deserialize;

pub struct OpenAIProvider;

impl OpenAIProvider {
    pub fn new() -> Self {
        Self
    }
}

// ── Response deserialization types ──────────────────────────────────────────

#[derive(Deserialize)]
struct OaiResponse {
    choices: Vec<OaiChoice>,
    usage: Option<OaiUsage>,
}

#[derive(Deserialize)]
struct OaiChoice {
    message: OaiMessage,
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct OaiMessage {
    content: Option<String>,
}

#[derive(Deserialize)]
struct OaiUsage {
    prompt_tokens: usize,
    completion_tokens: usize,
}

// ── Provider implementation ─────────────────────────────────────────────────

impl AIProvider for OpenAIProvider {
    fn name(&self) -> &str {
        "openai"
    }

    fn generate(
        &self,
        model: &str,
        prompt: &Prompt,
        auth: &AuthCredential,
    ) -> AIResult<AIResponse> {
        let key = auth.resolve()?;
        if key.is_empty() {
            return Err(AIError::AuthenticationError(
                "OpenAI API key is empty".to_string(),
            ));
        }

        // Build the request body
        let mut messages = Vec::new();
        if let Some(sys) = &prompt.system_instruction {
            messages.push(serde_json::json!({ "role": "system", "content": sys }));
        }
        messages.push(serde_json::json!({ "role": "user", "content": &prompt.user_prompt }));

        let body = serde_json::json!({
            "model": model,
            "messages": messages,
            "temperature": prompt.temperature.unwrap_or(0.7),
            "max_tokens": prompt.max_tokens.unwrap_or(2048)
        });

        let client = reqwest::blocking::Client::new();
        let http_resp = client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&key)
            .json(&body)
            .send()
            .map_err(|e| AIError::ProviderError {
                provider: "openai".to_string(),
                message: format!("HTTP request failed: {}", e),
            })?;

        if !http_resp.status().is_success() {
            let status = http_resp.status();
            let err_body = http_resp.text().unwrap_or_default();
            return Err(AIError::ProviderError {
                provider: "openai".to_string(),
                message: format!("API error {}: {}", status, err_body),
            });
        }

        let oai: OaiResponse = http_resp.json().map_err(|e| AIError::ProviderError {
            provider: "openai".to_string(),
            message: format!("Failed to parse response: {}", e),
        })?;

        let (text, _finish_reason) = oai
            .choices
            .into_iter()
            .next()
            .map(|c| {
                (
                    c.message.content.unwrap_or_default(),
                    c.finish_reason.unwrap_or_else(|| "stop".to_string()),
                )
            })
            .unwrap_or_else(|| (String::new(), "stop".to_string()));

        let (prompt_tokens, completion_tokens) = oai
            .usage
            .map(|u| (u.prompt_tokens, u.completion_tokens))
            .unwrap_or_else(|| {
                let pt = prompt.user_prompt.len() / 4 + 1;
                let ct = text.len() / 4 + 1;
                (pt, ct)
            });

        let usage = TokenUsage::new(prompt_tokens, completion_tokens).with_rates(0.0015, 0.002);

        let mut res = AIResponse::new(model, self.name(), text);
        res.usage = usage;
        Ok(res)
    }

    fn stream(
        &self,
        model: &str,
        prompt: &Prompt,
        auth: &AuthCredential,
    ) -> AIResult<StreamReceiver> {
        // For streaming we call generate and then split the result into chunks
        let resp = self.generate(model, prompt, auth)?;
        let words: Vec<String> = resp
            .text
            .split_whitespace()
            .map(|w| format!("{} ", w))
            .collect();
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
