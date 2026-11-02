use crate::vector_store::SearchHit;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct RetrievalCase {
    pub query: String,
    pub relevant_ids: Vec<String>,
    pub retrieved: Vec<SearchHit>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetrievalMetrics {
    pub cases: usize,
    pub recall_at_k: f64,
    pub precision_at_k: f64,
    pub mean_reciprocal_rank: f64,
    pub hit_rate: f64,
}

/// Evaluate a fixed-k retrieval result set using Recall@k, Precision@k, MRR, and hit rate.
pub fn evaluate_retrieval(cases: &[RetrievalCase]) -> RetrievalMetrics {
    if cases.is_empty() {
        return RetrievalMetrics::default();
    }
    let mut recall = 0.0;
    let mut precision = 0.0;
    let mut reciprocal_rank = 0.0;
    let mut hit_rate = 0.0;
    let mut evaluated = 0usize;
    for case in cases {
        if case.relevant_ids.is_empty() {
            continue;
        }
        evaluated += 1;
        let relevant: std::collections::HashSet<&str> =
            case.relevant_ids.iter().map(String::as_str).collect();
        let found: Vec<&str> = case
            .retrieved
            .iter()
            .map(|hit| hit.record.id.as_str())
            .collect();
        let relevant_found = found.iter().filter(|id| relevant.contains(**id)).count();
        recall += relevant_found as f64 / relevant.len() as f64;
        precision += relevant_found as f64 / found.len().max(1) as f64;
        if relevant_found > 0 {
            hit_rate += 1.0;
        }
        if let Some(rank) = found.iter().position(|id| relevant.contains(*id)) {
            reciprocal_rank += 1.0 / (rank + 1) as f64;
        }
    }
    if evaluated == 0 {
        return RetrievalMetrics::default();
    }
    let count = evaluated as f64;
    RetrievalMetrics {
        cases: evaluated,
        recall_at_k: recall / count,
        precision_at_k: precision / count,
        mean_reciprocal_rank: reciprocal_rank / count,
        hit_rate: hit_rate / count,
    }
}
