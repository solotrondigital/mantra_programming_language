//! Streaming Engine for real-time AI chunk delivery.

/// Single chunk yielded by an AI streaming response.
#[derive(Debug, Clone, PartialEq)]
pub struct StreamChunk {
    /// Delta text received in this chunk
    pub delta: String,
    /// Whether this chunk marks the final message chunk
    pub is_final: bool,
    /// Finish reason, if final (e.g. "stop", "length")
    pub finish_reason: Option<String>,
}

impl StreamChunk {
    pub fn new(delta: impl Into<String>) -> Self {
        Self {
            delta: delta.into(),
            is_final: false,
            finish_reason: None,
        }
    }

    pub fn final_chunk(delta: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            delta: delta.into(),
            is_final: true,
            finish_reason: Some(reason.into()),
        }
    }
}

/// Simulated/buffered stream iterator that delivers chunks.
#[derive(Debug, Clone)]
pub struct StreamReceiver {
    chunks: Vec<StreamChunk>,
    cursor: usize,
}

impl StreamReceiver {
    pub fn new(chunks: Vec<StreamChunk>) -> Self {
        Self { chunks, cursor: 0 }
    }

    /// Read next chunk synchronously.
    pub fn next_chunk(&mut self) -> Option<StreamChunk> {
        if self.cursor < self.chunks.len() {
            let chunk = self.chunks[self.cursor].clone();
            self.cursor += 1;
            Some(chunk)
        } else {
            None
        }
    }

    /// Collect all remaining streamed text into a single string.
    pub fn collect_text(&mut self) -> String {
        let mut buffer = String::new();
        while let Some(chunk) = self.next_chunk() {
            buffer.push_str(&chunk.delta);
        }
        buffer
    }
}

