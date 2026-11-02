use mantra_ml::{
    evaluate_retrieval, AnswerGenerator, ChunkOptions, DocumentProcessor, EmbeddingEngine,
    EmbeddingProvider, JsonVectorStore, LinearModel, LocalFeatureEmbedding, MemoryVectorStore,
    MetadataFilter, MlError, ModelRuntime, Predictor, RagPipeline, RetrievalCase, VectorRecord,
    VectorStore,
};
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

fn temp_path(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("mantra_ml_{name}_{}.json", std::process::id()))
}

#[test]
fn ml_001_and_002_load_versioned_local_model_and_run_inference() {
    let path = temp_path("linear_model");
    fs::write(&path, r#"{"name":"risk-score","version":"1.2.0","weights":[[0.5,2.0],[-1.0,1.0]],"bias":[1.0,0.0]}"#).unwrap();
    let mut runtime = ModelRuntime::new();
    runtime.load_local("risk", &path).unwrap();
    assert_eq!(
        runtime.model_version("risk").unwrap(),
        ("risk-score", "1.2.0")
    );
    assert_eq!(
        runtime.predict("risk", &[2.0, 3.0]).unwrap(),
        vec![8.0, 1.0]
    );
    assert!(matches!(
        runtime.predict("risk", &[1.0]),
        Err(MlError::DimensionMismatch { .. })
    ));
    fs::remove_file(path).unwrap();
}

#[test]
fn ml_003_embedding_registry_supports_provider_switching() {
    struct FixedEmbedding(&'static str, Vec<f32>);
    impl EmbeddingProvider for FixedEmbedding {
        fn embed(&self, _text: &str) -> mantra_ml::MlResult<Vec<f32>> {
            Ok(self.1.clone())
        }
        fn name(&self) -> &str {
            self.0
        }
    }
    let mut engine = EmbeddingEngine::new();
    engine.register("local", Arc::new(LocalFeatureEmbedding::new(64).unwrap()));
    engine.register(
        "remote",
        Arc::new(FixedEmbedding("remote-test", vec![0.0, 1.0])),
    );
    let local = engine.embed("local", "employee leave policy").unwrap();
    assert_eq!(local.len(), 64);
    assert!(engine.embed("remote", "text").unwrap()[1] == 1.0);
    assert_eq!(engine.provider_name("remote").unwrap(), "remote-test");
}

#[test]
fn ml_004_and_005_vector_stores_upsert_and_rank_by_cosine_similarity() {
    let store = MemoryVectorStore::new();
    for (id, vector) in [
        ("same", vec![1.0, 0.0]),
        ("near", vec![0.8, 0.2]),
        ("other", vec![0.0, 1.0]),
    ] {
        store
            .upsert(VectorRecord {
                id: id.into(),
                vector,
                text: id.into(),
                metadata: HashMap::new(),
            })
            .unwrap();
    }
    let hits = store.search(&[1.0, 0.0], 2, None).unwrap();
    assert_eq!(hits[0].record.id, "same");
    assert!(hits[0].score > hits[1].score);
    assert_eq!(store.len(), 3);
}

#[test]
fn ml_004_json_vector_store_persists_across_reopen() {
    let path = temp_path("vectors");
    let store = JsonVectorStore::open(&path).unwrap();
    store
        .upsert(VectorRecord {
            id: "persisted".into(),
            vector: vec![1.0, 0.0],
            text: "hello".into(),
            metadata: HashMap::new(),
        })
        .unwrap();
    drop(store);
    let reopened = JsonVectorStore::open(&path).unwrap();
    assert_eq!(
        reopened.search(&[1.0, 0.0], 1, None).unwrap()[0]
            .record
            .text,
        "hello"
    );
    fs::remove_file(path).unwrap();
}

#[test]
fn ml_006_and_007_ingest_text_and_chunk_with_overlap() {
    let document = DocumentProcessor::from_text(
        "handbook",
        "handbook.txt",
        "alpha beta gamma delta epsilon zeta eta theta",
    )
    .unwrap();
    let chunks = DocumentProcessor::chunk(
        &document,
        ChunkOptions {
            max_chars: 20,
            overlap_chars: 5,
        },
    )
    .unwrap();
    assert!(chunks.len() > 1);
    assert!(
        chunks[0].end_char > chunks[1].start_char,
        "chunks should include configured overlap"
    );
    assert!(chunks.iter().all(|chunk| !chunk.text.is_empty()));
}

#[test]
fn ml_006_extracts_text_from_simple_pdf_content_stream() {
    let pdf =
        b"%PDF-1.4\nstream\n(Benefits policy) Tj\n(Annual leave is 20 days.) Tj\nendstream\n%%EOF";
    let text = DocumentProcessor::from_pdf_bytes("pdf1", "policy.pdf", pdf)
        .unwrap()
        .text;
    assert!(text.contains("Benefits policy"));
    assert!(text.contains("Annual leave is 20 days."));
}

struct EchoEvidence;
impl AnswerGenerator for EchoEvidence {
    fn generate(&self, system: &str, question: &str) -> mantra_ml::MlResult<String> {
        assert!(system.contains("Annual leave is 20 days"));
        assert!(system.contains("using only the retrieved evidence"));
        Ok(format!("Evidence for: {question}"))
    }
}

#[test]
fn ml_008_rag_ingests_retrieves_and_answers_with_citations() {
    let pipeline = RagPipeline::new(
        "local",
        Arc::new(LocalFeatureEmbedding::new(256).unwrap()),
        Arc::new(MemoryVectorStore::new()),
        Arc::new(EchoEvidence),
    );
    let count = pipeline
        .ingest_text(
            "policy",
            "hr/leave.txt",
            "Annual leave is 20 days. Sick leave is 10 days.",
            HashMap::new(),
        )
        .unwrap();
    assert_eq!(count, 1);
    let answer = pipeline.ask("How many annual leave days?").unwrap();
    assert!(answer.answer.contains("annual leave"));
    assert_eq!(answer.citations[0].source, "hr/leave.txt");
}

#[test]
fn ml_009_filter_applies_during_vector_search() {
    let store = MemoryVectorStore::new();
    for (id, department) in [("hr", "hr"), ("eng", "engineering")] {
        let mut metadata = HashMap::new();
        metadata.insert("department".into(), json!(department));
        metadata.insert("year".into(), json!(2025));
        store
            .upsert(VectorRecord {
                id: id.into(),
                vector: vec![1.0, 0.0],
                text: id.into(),
                metadata,
            })
            .unwrap();
    }
    let filter = MetadataFilter::And {
        filters: vec![
            MetadataFilter::Equals {
                key: "department".into(),
                value: json!("hr"),
            },
            MetadataFilter::NumberRange {
                key: "year".into(),
                min: Some(2024.0),
                max: None,
            },
        ],
    };
    let hits = store.search(&[1.0, 0.0], 10, Some(&filter)).unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].record.id, "hr");
}

#[test]
fn ml_010_retrieval_metrics_cover_recall_precision_mrr_and_hit_rate() {
    let hit = |id: &str| mantra_ml::SearchHit {
        record: VectorRecord {
            id: id.into(),
            vector: vec![1.0],
            text: String::new(),
            metadata: HashMap::new(),
        },
        score: 1.0,
    };
    let metrics = evaluate_retrieval(&[
        RetrievalCase {
            query: "q1".into(),
            relevant_ids: vec!["a".into(), "b".into()],
            retrieved: vec![hit("x"), hit("b")],
        },
        RetrievalCase {
            query: "q2".into(),
            relevant_ids: vec!["c".into()],
            retrieved: vec![hit("c")],
        },
    ]);
    assert!((metrics.recall_at_k - 0.75).abs() < 1e-9);
    assert!((metrics.precision_at_k - 0.75).abs() < 1e-9);
    assert!((metrics.mean_reciprocal_rank - 0.75).abs() < 1e-9);
    assert_eq!(metrics.hit_rate, 1.0);
}

#[test]
fn ml_011_switch_between_memory_and_durable_vector_store_adapters() {
    let record = VectorRecord {
        id: "x".into(),
        vector: vec![0.0, 1.0],
        text: "x".into(),
        metadata: HashMap::new(),
    };
    let stores: Vec<Arc<dyn VectorStore>> = vec![
        Arc::new(MemoryVectorStore::new()),
        Arc::new(JsonVectorStore::open(temp_path("provider_switch")).unwrap()),
    ];
    for store in &stores {
        store.upsert(record.clone()).unwrap();
        assert_eq!(
            store.search(&[0.0, 1.0], 1, None).unwrap()[0].record.id,
            "x"
        );
    }
    drop(stores);
    let _ = fs::remove_file(temp_path("provider_switch"));
}

#[test]
fn ml_012_cloud_inference_configuration_requires_secure_remote_endpoint() {
    assert!(mantra_ml::CloudHttpModel::new(
        "cloud",
        "v1",
        "http://remote.example/model",
        None,
        std::time::Duration::from_secs(2)
    )
    .is_err());
    assert!(mantra_ml::CloudHttpModel::new(
        "cloud",
        "v1",
        "https://ml.example/model",
        Some("test-key".into()),
        std::time::Duration::from_secs(2)
    )
    .is_ok());
    let local = LinearModel {
        name: "local".into(),
        version: "v1".into(),
        weights: vec![vec![1.0]],
        bias: vec![0.0],
    };
    assert_eq!(local.predict(&[3.0]).unwrap(), vec![3.0]);
}

#[test]
fn ml_012_openai_embedding_adapter_accepts_cloud_or_local_compatible_endpoint() {
    assert!(mantra_ml::OpenAIEmbedding::new("text-embedding", "key").is_ok());
    assert!(mantra_ml::OpenAIEmbedding::with_endpoint(
        "test",
        "key",
        "http://example.com",
        std::time::Duration::from_secs(1)
    )
    .is_err());
}
