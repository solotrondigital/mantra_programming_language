use crate::error::{MlError, MlResult};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

pub trait EmbeddingProvider: Send + Sync {
    fn embed(&self, text: &str) -> MlResult<Vec<f32>>;
    fn name(&self) -> &str;
}

/// Dependency-free local feature hashing. This is useful for offline tests and
/// lexical retrieval; use a semantic embedding provider for production RAG.
pub struct LocalFeatureEmbedding {
    dimensions: usize,
}

impl LocalFeatureEmbedding {
    pub fn new(dimensions: usize) -> MlResult<Self> {
        if dimensions < 8 {
            return Err(MlError::InvalidInput(
                "local embedding dimension must be at least 8".into(),
            ));
        }
        Ok(Self { dimensions })
    }
}

impl EmbeddingProvider for LocalFeatureEmbedding {
    fn embed(&self, text: &str) -> MlResult<Vec<f32>> {
        if text.trim().is_empty() {
            return Err(MlError::InvalidInput("cannot embed empty text".into()));
        }
        let mut vector = vec![0.0f32; self.dimensions];
        let tokens: Vec<String> = text
            .to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|token| !token.is_empty())
            .map(str::to_string)
            .collect();
        for token in &tokens {
            add_feature(&mut vector, token, 1.0);
        }
        for pair in tokens.windows(2) {
            add_feature(&mut vector, &format!("{}_{}", pair[0], pair[1]), 0.5);
        }
        normalize(&mut vector);
        Ok(vector)
    }

    fn name(&self) -> &str {
        "local-feature-hash"
    }
}

fn add_feature(vector: &mut [f32], feature: &str, weight: f32) {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in feature.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    let index = (hash as usize) % vector.len();
    let sign = if hash & (1 << 63) == 0 { 1.0 } else { -1.0 };
    vector[index] += sign * weight;
}

fn normalize(vector: &mut [f32]) {
    let norm = vector.iter().map(|value| value * value).sum::<f32>().sqrt();
    if norm > 0.0 {
        for value in vector {
            *value /= norm;
        }
    }
}

pub struct OpenAIEmbedding {
    model: String,
    api_key: String,
    endpoint: String,
    client: Client,
}

impl OpenAIEmbedding {
    pub fn new(model: impl Into<String>, api_key: impl Into<String>) -> MlResult<Self> {
        Self::with_endpoint(
            model,
            api_key,
            "https://api.openai.com/v1/embeddings",
            Duration::from_secs(30),
        )
    }

    pub fn with_endpoint(
        model: impl Into<String>,
        api_key: impl Into<String>,
        endpoint: impl Into<String>,
        timeout: Duration,
    ) -> MlResult<Self> {
        let endpoint = endpoint.into();
        if !endpoint.starts_with("https://")
            && !endpoint.starts_with("http://127.0.0.1")
            && !endpoint.starts_with("http://localhost")
        {
            return Err(MlError::InvalidInput(
                "embedding endpoint must use HTTPS (HTTP is allowed only for localhost)".into(),
            ));
        }
        let api_key = api_key.into();
        if api_key.trim().is_empty() {
            return Err(MlError::InvalidInput(
                "embedding API key cannot be empty".into(),
            ));
        }
        // Avoid implicit OS proxy discovery in headless/sandboxed runtimes.
        let client = Client::builder()
            .no_proxy()
            .timeout(timeout)
            .build()
            .map_err(|e| MlError::Provider(e.to_string()))?;
        Ok(Self {
            model: model.into(),
            api_key,
            endpoint,
            client,
        })
    }
}

#[derive(Deserialize)]
struct OpenAIEmbeddingResponse {
    data: Vec<OpenAIEmbeddingItem>,
}
#[derive(Deserialize)]
struct OpenAIEmbeddingItem {
    embedding: Vec<f32>,
}

impl EmbeddingProvider for OpenAIEmbedding {
    fn embed(&self, text: &str) -> MlResult<Vec<f32>> {
        if text.trim().is_empty() {
            return Err(MlError::InvalidInput("cannot embed empty text".into()));
        }
        let response = self
            .client
            .post(&self.endpoint)
            .bearer_auth(&self.api_key)
            .json(&serde_json::json!({"model": self.model, "input": text}))
            .send()
            .and_then(|r| r.error_for_status())
            .map_err(|e| MlError::Provider(e.to_string()))?
            .json::<OpenAIEmbeddingResponse>()
            .map_err(|e| MlError::Serialization(e.to_string()))?;
        let vector = response
            .data
            .into_iter()
            .next()
            .ok_or_else(|| MlError::Provider("embedding response returned no vectors".into()))?
            .embedding;
        validate_embedding(&vector)?;
        Ok(vector)
    }
    fn name(&self) -> &str {
        &self.model
    }
}

pub struct EmbeddingEngine {
    providers: HashMap<String, Arc<dyn EmbeddingProvider>>,
}

impl Default for EmbeddingEngine {
    fn default() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }
}

impl EmbeddingEngine {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register(&mut self, alias: impl Into<String>, provider: Arc<dyn EmbeddingProvider>) {
        self.providers.insert(alias.into(), provider);
    }
    pub fn embed(&self, alias: &str, text: &str) -> MlResult<Vec<f32>> {
        let provider = self
            .providers
            .get(alias)
            .ok_or_else(|| MlError::ModelNotFound(alias.to_string()))?;
        let vector = provider.embed(text)?;
        validate_embedding(&vector)?;
        Ok(vector)
    }
    pub fn provider_name(&self, alias: &str) -> MlResult<&str> {
        self.providers
            .get(alias)
            .map(|provider| provider.name())
            .ok_or_else(|| MlError::ModelNotFound(alias.to_string()))
    }
}

pub(crate) fn validate_embedding(vector: &[f32]) -> MlResult<()> {
    if vector.is_empty() || vector.iter().any(|value| !value.is_finite()) {
        return Err(MlError::InvalidInput(
            "embedding must be a non-empty vector of finite numbers".into(),
        ));
    }
    Ok(())
}
