//! Mantra ML — model inference, embeddings, vector retrieval, document ingestion,
//! evaluation, and retrieval-augmented generation.

pub mod document;
pub mod embedding;
pub mod error;
pub mod evaluation;
pub mod model;
pub mod rag;
pub mod vector_store;

pub use document::{Chunk, ChunkOptions, Document, DocumentProcessor};
pub use embedding::{EmbeddingEngine, EmbeddingProvider, LocalFeatureEmbedding, OpenAIEmbedding};
pub use error::{MlError, MlResult};
pub use evaluation::{evaluate_retrieval, RetrievalCase, RetrievalMetrics};
pub use model::{CloudHttpModel, LinearModel, ModelRuntime, Predictor};
pub use rag::{AiAnswerGenerator, AnswerGenerator, RagAnswer, RagCitation, RagPipeline};
pub use vector_store::{
    cosine_similarity, JsonVectorStore, MemoryVectorStore, MetadataFilter, SearchHit, VectorRecord,
    VectorStore,
};
