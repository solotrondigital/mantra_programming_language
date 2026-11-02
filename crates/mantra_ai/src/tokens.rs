//! Token management, usage counters, limits, and cost estimation.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Token usage metadata returned from a generation call.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TokenUsage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
    pub estimated_cost_usd: f64,
}

impl TokenUsage {
    pub fn new(prompt_tokens: usize, completion_tokens: usize) -> Self {
        Self {
            prompt_tokens,
            completion_tokens,
            total_tokens: prompt_tokens + completion_tokens,
            estimated_cost_usd: 0.0,
        }
    }

    /// Calculate approximate cost based on rates per 1,000 tokens.
    pub fn with_rates(mut self, prompt_per_1k: f64, completion_per_1k: f64) -> Self {
        let p_cost = (self.prompt_tokens as f64 / 1000.0) * prompt_per_1k;
        let c_cost = (self.completion_tokens as f64 / 1000.0) * completion_per_1k;
        self.estimated_cost_usd = p_cost + c_cost;
        self
    }
}

/// Token Manager that tracks token usage per session or globally across an application.
#[derive(Debug, Clone, Default)]
pub struct TokenManager {
    total_prompt_tokens: Arc<AtomicUsize>,
    total_completion_tokens: Arc<AtomicUsize>,
    max_token_limit: Option<usize>,
}

impl TokenManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_limit(limit: usize) -> Self {
        Self {
            total_prompt_tokens: Arc::new(AtomicUsize::new(0)),
            total_completion_tokens: Arc::new(AtomicUsize::new(0)),
            max_token_limit: Some(limit),
        }
    }

    /// Record new token usage. Returns an error if the overall limit would be exceeded.
    pub fn record(&self, usage: &TokenUsage) -> Result<(), usize> {
        let current_total = self.total_tokens();
        if let Some(limit) = self.max_token_limit {
            if current_total + usage.total_tokens > limit {
                return Err(limit);
            }
        }
        self.total_prompt_tokens
            .fetch_add(usage.prompt_tokens, Ordering::Relaxed);
        self.total_completion_tokens
            .fetch_add(usage.completion_tokens, Ordering::Relaxed);
        Ok(())
    }

    pub fn prompt_tokens(&self) -> usize {
        self.total_prompt_tokens.load(Ordering::Relaxed)
    }

    pub fn completion_tokens(&self) -> usize {
        self.total_completion_tokens.load(Ordering::Relaxed)
    }

    pub fn total_tokens(&self) -> usize {
        self.prompt_tokens() + self.completion_tokens()
    }

    /// Reset token counters
    pub fn reset(&self) {
        self.total_prompt_tokens.store(0, Ordering::Relaxed);
        self.total_completion_tokens.store(0, Ordering::Relaxed);
    }
}
