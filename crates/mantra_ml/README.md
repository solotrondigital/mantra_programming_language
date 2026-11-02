# mantra_ml

Native ML and knowledge retrieval interfaces for Mantra. This crate deliberately uses provider traits and adapters instead of implementing a deep-learning framework.

## MVP4 stories implemented

| Story | Runtime surface |
|---|---|
| ML-001/002 | `ModelRuntime`, versioned `LinearModel`, and `CloudHttpModel` prediction |
| ML-003 | `EmbeddingEngine`, local feature hashing, and OpenAI-compatible embeddings |
| ML-004/005 | `VectorStore`, in-memory and durable JSON stores, cosine similarity |
| ML-006/007 | Text and simple PDF ingestion, normalization, overlapping chunks |
| ML-008 | `RagPipeline` ingestion, filtered retrieval, answer generation, source citations |
| ML-009 | Exact, set membership, numeric range, and AND metadata filters |
| ML-010 | Recall@k, precision@k, MRR, and hit-rate evaluation |
| ML-011 | `VectorStore` trait permits switching store adapters; memory and JSON adapters included |
| ML-012 | Local JSON linear models and configurable HTTPS cloud predictor |

## Model inference

Local linear model file:

```json
{"name":"risk-score","version":"1.0.0","weights":[[0.5,2.0]],"bias":[1.0]}
```

```rust
use mantra_ml::ModelRuntime;

let mut runtime = ModelRuntime::new();
runtime.load_local("risk", "models/risk.json")?;
let scores = runtime.predict("risk", &[2.0, 3.0])?;
```

`CloudHttpModel` accepts a JSON prediction endpoint using `{"input":[...]}` and `{"output":[...]}`. Remote endpoints require HTTPS; plain HTTP is permitted only for localhost development. A model host can also implement `Predictor` for ONNX, Candle, or another local/cloud inference runtime.

## Embeddings and retrieval

```rust
use mantra_ml::{EmbeddingEngine, LocalFeatureEmbedding, MemoryVectorStore};
use std::sync::Arc;

let mut embeddings = EmbeddingEngine::new();
embeddings.register("local", Arc::new(LocalFeatureEmbedding::new(512)?));
let vector = embeddings.embed("local", "company leave policy")?;
let vectors = Arc::new(MemoryVectorStore::new());
```

For semantic retrieval, register `OpenAIEmbedding` or a custom `EmbeddingProvider`. `LocalFeatureEmbedding` uses normalized word and bigram feature hashing; it is deterministic and offline but lexical, not a semantic neural model.

## Document processing and RAG

`DocumentProcessor::load` reads UTF-8 text formats and PDF files. The bundled PDF reader handles uncompressed text operators and escaped literal strings. Scanned, encrypted, or compressed-content PDFs need a production PDF extraction backend; this reader returns a clear error when it cannot extract text.

Use `RagPipeline` with an embedding provider, any `VectorStore` adapter, and an `AnswerGenerator` (the `AiAnswerGenerator` adapter connects `mantra_ai`). It stores source and chunk metadata, supports filters during retrieval, and grounds answers with retrieved excerpts and citation metadata. The system prompt instructs the answer model to treat retrieved content as untrusted evidence and say when it is insufficient.

Metadata filters are composable: `Equals`, `In`, `NumberRange`, and `And`. For a hosted vector database, implement `VectorStore` and pass it into the same pipeline; the application does not need to change its RAG logic.

## Evaluation

`evaluate_retrieval` calculates Recall@k, Precision@k, mean reciprocal rank, and hit rate over labelled retrieval cases. It is intentionally separate from online retrieval so datasets and ranking changes can be compared repeatably.

Run the acceptance tests with `cargo test -p mantra_ml`.
