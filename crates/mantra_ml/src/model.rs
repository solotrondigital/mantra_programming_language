use crate::error::{MlError, MlResult};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

/// Common numeric prediction interface for local and hosted model runtimes.
pub trait Predictor: Send + Sync {
    fn predict(&self, input: &[f64]) -> MlResult<Vec<f64>>;
    fn model_name(&self) -> &str;
    fn version(&self) -> &str;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearModel {
    pub name: String,
    pub version: String,
    pub weights: Vec<Vec<f64>>,
    pub bias: Vec<f64>,
}

impl LinearModel {
    /// Load a small dense linear model from JSON: `{name, version, weights, bias}`.
    pub fn load(path: impl AsRef<Path>) -> MlResult<Self> {
        let contents = fs::read_to_string(path).map_err(|e| MlError::Io(e.to_string()))?;
        let model: Self =
            serde_json::from_str(&contents).map_err(|e| MlError::Serialization(e.to_string()))?;
        model.validate()?;
        Ok(model)
    }

    pub fn validate(&self) -> MlResult<()> {
        if self.name.trim().is_empty() || self.version.trim().is_empty() || self.weights.is_empty()
        {
            return Err(MlError::InvalidInput(
                "linear model needs a name, version, and at least one output row".into(),
            ));
        }
        let input_dim = self.weights[0].len();
        if input_dim == 0
            || self.weights.iter().any(|row| row.len() != input_dim)
            || self.weights.len() != self.bias.len()
        {
            return Err(MlError::InvalidInput(
                "linear model weights must be rectangular and match the bias length".into(),
            ));
        }
        if self
            .weights
            .iter()
            .flatten()
            .chain(self.bias.iter())
            .any(|x| !x.is_finite())
        {
            return Err(MlError::InvalidInput(
                "linear model parameters must be finite".into(),
            ));
        }
        Ok(())
    }
}

impl Predictor for LinearModel {
    fn predict(&self, input: &[f64]) -> MlResult<Vec<f64>> {
        let expected = self.weights.first().map_or(0, Vec::len);
        if expected != input.len() {
            return Err(MlError::DimensionMismatch {
                expected,
                actual: input.len(),
            });
        }
        if input.iter().any(|x| !x.is_finite()) {
            return Err(MlError::InvalidInput(
                "inference input must contain only finite numbers".into(),
            ));
        }
        Ok(self
            .weights
            .iter()
            .zip(&self.bias)
            .map(|(row, bias)| {
                row.iter()
                    .zip(input)
                    .map(|(weight, value)| weight * value)
                    .sum::<f64>()
                    + bias
            })
            .collect())
    }

    fn model_name(&self) -> &str {
        &self.name
    }
    fn version(&self) -> &str {
        &self.version
    }
}

/// Cloud prediction adapter for JSON endpoints accepting `{input:[...]}` and returning `{output:[...]}`.
pub struct CloudHttpModel {
    name: String,
    version: String,
    endpoint: String,
    api_key: Option<String>,
    client: Client,
}

impl CloudHttpModel {
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        endpoint: impl Into<String>,
        api_key: Option<String>,
        timeout: Duration,
    ) -> MlResult<Self> {
        let endpoint = endpoint.into();
        if !endpoint.starts_with("https://")
            && !endpoint.starts_with("http://127.0.0.1")
            && !endpoint.starts_with("http://localhost")
        {
            return Err(MlError::InvalidInput(
                "cloud model endpoint must use HTTPS (HTTP is allowed only for localhost)".into(),
            ));
        }
        // Avoid implicit OS proxy discovery; it is unreliable in headless and
        // sandboxed environments. Deployments needing a proxy can add it explicitly.
        let client = Client::builder()
            .no_proxy()
            .timeout(timeout)
            .build()
            .map_err(|e| MlError::Provider(e.to_string()))?;
        Ok(Self {
            name: name.into(),
            version: version.into(),
            endpoint,
            api_key,
            client,
        })
    }
}

impl Predictor for CloudHttpModel {
    fn predict(&self, input: &[f64]) -> MlResult<Vec<f64>> {
        if input.iter().any(|x| !x.is_finite()) {
            return Err(MlError::InvalidInput(
                "inference input must contain only finite numbers".into(),
            ));
        }
        let mut request = self
            .client
            .post(&self.endpoint)
            .json(&serde_json::json!({"input": input}));
        if let Some(key) = &self.api_key {
            request = request.bearer_auth(key);
        }
        let response = request
            .send()
            .and_then(|r| r.error_for_status())
            .map_err(|e| MlError::Provider(e.to_string()))?;
        let value: serde_json::Value = response
            .json()
            .map_err(|e| MlError::Serialization(e.to_string()))?;
        serde_json::from_value(value.get("output").cloned().ok_or_else(|| {
            MlError::InvalidInput("cloud response must contain an 'output' array".into())
        })?)
        .map_err(|e| MlError::Serialization(e.to_string()))
    }
    fn model_name(&self) -> &str {
        &self.name
    }
    fn version(&self) -> &str {
        &self.version
    }
}

#[derive(Default)]
pub struct ModelRuntime {
    models: HashMap<String, Arc<dyn Predictor>>,
}

impl ModelRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_local(&mut self, alias: impl Into<String>, path: impl AsRef<Path>) -> MlResult<()> {
        let model = LinearModel::load(path)?;
        self.register(alias, Arc::new(model));
        Ok(())
    }

    pub fn register(&mut self, alias: impl Into<String>, model: Arc<dyn Predictor>) {
        self.models.insert(alias.into(), model);
    }

    pub fn predict(&self, alias: &str, input: &[f64]) -> MlResult<Vec<f64>> {
        self.models
            .get(alias)
            .ok_or_else(|| MlError::ModelNotFound(alias.to_string()))?
            .predict(input)
    }

    pub fn model_version(&self, alias: &str) -> MlResult<(&str, &str)> {
        let model = self
            .models
            .get(alias)
            .ok_or_else(|| MlError::ModelNotFound(alias.to_string()))?;
        Ok((model.model_name(), model.version()))
    }
}
