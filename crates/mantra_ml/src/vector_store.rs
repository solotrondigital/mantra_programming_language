use crate::embedding::validate_embedding;
use crate::error::{MlError, MlResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorRecord {
    pub id: String,
    pub vector: Vec<f32>,
    pub text: String,
    #[serde(default)]
    pub metadata: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit {
    pub record: VectorRecord,
    /// Cosine similarity in [-1, 1]. Larger is more similar.
    pub score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum MetadataFilter {
    Equals {
        key: String,
        value: Value,
    },
    In {
        key: String,
        values: Vec<Value>,
    },
    NumberRange {
        key: String,
        min: Option<f64>,
        max: Option<f64>,
    },
    And {
        filters: Vec<MetadataFilter>,
    },
}

impl MetadataFilter {
    fn matches(&self, metadata: &HashMap<String, Value>) -> bool {
        match self {
            Self::Equals { key, value } => metadata.get(key) == Some(value),
            Self::In { key, values } => metadata
                .get(key)
                .is_some_and(|value| values.contains(value)),
            Self::NumberRange { key, min, max } => metadata
                .get(key)
                .and_then(Value::as_f64)
                .is_some_and(|value| {
                    min.map_or(true, |bound| value >= bound)
                        && max.map_or(true, |bound| value <= bound)
                }),
            Self::And { filters } => filters.iter().all(|filter| filter.matches(metadata)),
        }
    }
}

pub trait VectorStore: Send + Sync {
    fn upsert(&self, record: VectorRecord) -> MlResult<()>;
    fn search(
        &self,
        query: &[f32],
        limit: usize,
        filter: Option<&MetadataFilter>,
    ) -> MlResult<Vec<SearchHit>>;
    fn delete(&self, id: &str) -> MlResult<bool>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Default)]
pub struct MemoryVectorStore {
    records: Mutex<HashMap<String, VectorRecord>>,
}

impl MemoryVectorStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl VectorStore for MemoryVectorStore {
    fn upsert(&self, record: VectorRecord) -> MlResult<()> {
        validate_embedding(&record.vector)?;
        if record.id.trim().is_empty() {
            return Err(MlError::InvalidInput(
                "vector record ID cannot be empty".into(),
            ));
        }
        self.records
            .lock()
            .map_err(|_| MlError::Io("vector store lock poisoned".into()))?
            .insert(record.id.clone(), record);
        Ok(())
    }

    fn search(
        &self,
        query: &[f32],
        limit: usize,
        filter: Option<&MetadataFilter>,
    ) -> MlResult<Vec<SearchHit>> {
        validate_embedding(query)?;
        if limit == 0 {
            return Ok(Vec::new());
        }
        let records = self
            .records
            .lock()
            .map_err(|_| MlError::Io("vector store lock poisoned".into()))?;
        let mut hits = Vec::new();
        for record in records.values() {
            if filter.is_some_and(|filter| !filter.matches(&record.metadata)) {
                continue;
            }
            if record.vector.len() != query.len() {
                return Err(MlError::DimensionMismatch {
                    expected: query.len(),
                    actual: record.vector.len(),
                });
            }
            hits.push(SearchHit {
                score: cosine_similarity(query, &record.vector)?,
                record: record.clone(),
            });
        }
        hits.sort_by(|a, b| {
            b.score
                .total_cmp(&a.score)
                .then_with(|| a.record.id.cmp(&b.record.id))
        });
        hits.truncate(limit);
        Ok(hits)
    }

    fn delete(&self, id: &str) -> MlResult<bool> {
        Ok(self
            .records
            .lock()
            .map_err(|_| MlError::Io("vector store lock poisoned".into()))?
            .remove(id)
            .is_some())
    }
    fn len(&self) -> usize {
        self.records
            .lock()
            .map(|records| records.len())
            .unwrap_or(0)
    }
}

/// Durable, provider-neutral JSON vector store suitable for local development and small datasets.
pub struct JsonVectorStore {
    path: PathBuf,
    records: Mutex<HashMap<String, VectorRecord>>,
}

impl JsonVectorStore {
    pub fn open(path: impl Into<PathBuf>) -> MlResult<Self> {
        let path = path.into();
        let records = match fs::read_to_string(&path) {
            Ok(contents) => serde_json::from_str(&contents)
                .map_err(|e| MlError::Serialization(e.to_string()))?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => HashMap::new(),
            Err(error) => return Err(MlError::Io(error.to_string())),
        };
        Ok(Self {
            path,
            records: Mutex::new(records),
        })
    }

    fn persist(&self, records: &HashMap<String, VectorRecord>) -> MlResult<()> {
        if let Some(parent) = self
            .path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            fs::create_dir_all(parent).map_err(|e| MlError::Io(e.to_string()))?;
        }
        let contents = serde_json::to_vec_pretty(records)
            .map_err(|e| MlError::Serialization(e.to_string()))?;
        let temp = self.path.with_extension("tmp");
        fs::write(&temp, contents).map_err(|e| MlError::Io(e.to_string()))?;
        fs::rename(&temp, &self.path).map_err(|e| MlError::Io(e.to_string()))
    }
}

impl VectorStore for JsonVectorStore {
    fn upsert(&self, record: VectorRecord) -> MlResult<()> {
        validate_embedding(&record.vector)?;
        let mut records = self
            .records
            .lock()
            .map_err(|_| MlError::Io("vector store lock poisoned".into()))?;
        records.insert(record.id.clone(), record);
        self.persist(&records)
    }
    fn search(
        &self,
        query: &[f32],
        limit: usize,
        filter: Option<&MetadataFilter>,
    ) -> MlResult<Vec<SearchHit>> {
        let memory = MemoryVectorStore {
            records: Mutex::new(
                self.records
                    .lock()
                    .map_err(|_| MlError::Io("vector store lock poisoned".into()))?
                    .clone(),
            ),
        };
        memory.search(query, limit, filter)
    }
    fn delete(&self, id: &str) -> MlResult<bool> {
        let mut records = self
            .records
            .lock()
            .map_err(|_| MlError::Io("vector store lock poisoned".into()))?;
        let removed = records.remove(id).is_some();
        if removed {
            self.persist(&records)?;
        }
        Ok(removed)
    }
    fn len(&self) -> usize {
        self.records
            .lock()
            .map(|records| records.len())
            .unwrap_or(0)
    }
}

impl From<&Path> for JsonVectorStore {
    fn from(path: &Path) -> Self {
        Self::open(path.to_path_buf()).expect("failed to open JSON vector store")
    }
}

pub fn cosine_similarity(left: &[f32], right: &[f32]) -> MlResult<f32> {
    validate_embedding(left)?;
    validate_embedding(right)?;
    if left.len() != right.len() {
        return Err(MlError::DimensionMismatch {
            expected: left.len(),
            actual: right.len(),
        });
    }
    let dot = left.iter().zip(right).map(|(a, b)| a * b).sum::<f32>();
    let left_norm = left.iter().map(|x| x * x).sum::<f32>().sqrt();
    let right_norm = right.iter().map(|x| x * x).sum::<f32>().sqrt();
    if left_norm == 0.0 || right_norm == 0.0 {
        return Err(MlError::InvalidInput(
            "zero vectors cannot be compared".into(),
        ));
    }
    Ok(dot / (left_norm * right_norm))
}
