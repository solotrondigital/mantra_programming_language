//! Google Gemini Provider implementation for Mantra AI — real HTTP calls via reqwest.

use crate::auth::AuthCredential;
use crate::error::{AIError, AIResult};
use crate::prompt::Prompt;
use crate::providers::{AIProvider, AIResponse};
use crate::streaming::{StreamChunk, StreamReceiver};
use crate::tokens::TokenUsage;
use serde::Deserialize;

pub struct GeminiProvider;

impl GeminiProvider {
    pub fn new() -> Self {
        Self
    }
}

// ── Response deserialization types ──────────────────────────────────────────

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<GeminiCandidate>>,
    #[serde(rename = "usageMetadata")]
    usage_metadata: Option<GeminiUsage>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: Option<GeminiContent>,
    #[serde(rename = "finishReason")]
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct GeminiContent {
    parts: Option<Vec<GeminiPart>>,
}

#[derive(Deserialize)]
struct GeminiPart {
    text: Option<String>,
}

#[derive(Deserialize)]
struct GeminiUsage {
    #[serde(rename = "promptTokenCount")]
    prompt_token_count: Option<usize>,
    #[serde(rename = "candidatesTokenCount")]
    candidates_token_count: Option<usize>,
}

// ── Provider implementation ─────────────────────────────────────────────────

impl AIProvider for GeminiProvider {
    fn name(&self) -> &str {
        "gemini"
    }

    fn generate(&self, model: &str, prompt: &Prompt, auth: &AuthCredential) -> AIResult<AIResponse> {
        let key = auth.resolve()?;
        if key.is_empty() {
            return Err(AIError::AuthenticationError("Gemini API key is empty".to_string()));
        }

        // Build contents array; Gemini uses a single "contents" with role "user"
        let mut contents = Vec::new();
        if let Some(sys) = &prompt.system_instruction {
            // Gemini supports system instruction via systemInstruction field
            let _ = sys; // handled below in body
        }
        contents.push(serde_json::json!({
            "role": "user",
            "parts": [{ "text": &prompt.user_prompt }]
        }));

        let mut body = serde_json::json!({
            "contents": contents,
            "generationConfig": {
                "temperature": prompt.temperature.unwrap_or(0.2),
                "maxOutputTokens": prompt.max_tokens.unwrap_or(4096)
            }
        });

        // Add system instruction if present
        if let Some(sys) = &prompt.system_instruction {
            body["systemInstruction"] = serde_json::json!({
                "parts": [{ "text": sys }]
            });
        }

        // Normalize model name: strip "models/" prefix if user-supplied, then construct URL
        let model_id = model.trim_start_matches("models/");
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model_id, key
        );

        let client = reqwest::blocking::Client::new();
        let http_resp = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .map_err(|e| AIError::ProviderError {
                provider: "gemini".to_string(),
                message: format!("HTTP request failed: {}", e),
            })?;

        if !http_resp.status().is_success() {
            let status = http_resp.status();
            let err_body = http_resp.text().unwrap_or_default();
            return Err(AIError::ProviderError {
                provider: "gemini".to_string(),
                message: format!("API error {}: {}", status, err_body),
            });
        }

        let gem: GeminiResponse = http_resp.json().map_err(|e| AIError::ProviderError {
            provider: "gemini".to_string(),
            message: format!("Failed to parse response: {}", e),
        })?;

        let text = gem
            .candidates
            .as_ref()
            .and_then(|cs| cs.first())
            .and_then(|c| c.content.as_ref())
            .and_then(|c| c.parts.as_ref())
            .and_then(|ps| ps.first())
            .and_then(|p| p.text.clone())
            .unwrap_or_default();

        let (prompt_tokens, completion_tokens) = gem
            .usage_metadata
            .map(|u| {
                (
                    u.prompt_token_count.unwrap_or(0),
                    u.candidates_token_count.unwrap_or(0),
                )
            })
            .unwrap_or_else(|| {
                let pt = prompt.user_prompt.len() / 4 + 1;
                let ct = text.len() / 4 + 1;
                (pt, ct)
            });

        let usage = TokenUsage::new(prompt_tokens, completion_tokens)
            .with_rates(0.0005, 0.0015);

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

