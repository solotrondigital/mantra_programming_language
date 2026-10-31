// ==============================================================================
// mantra_lsp::document — Document Manager & Text Buffer Utilities
// ==============================================================================

use std::collections::HashMap;

/// A single open document in the workspace
#[derive(Debug, Clone)]
pub struct Document {
    pub uri: String,
    pub text: String,
    pub version: i64,
    /// Lines split for fast access (no trailing newline per entry)
    pub lines: Vec<String>,
}

impl Document {
    pub fn new(uri: String, text: String, version: i64) -> Self {
        let lines = split_lines(&text);
        Document { uri, text, version, lines }
    }

    pub fn update(&mut self, text: String, version: i64) {
        self.lines = split_lines(&text);
        self.text = text;
        self.version = version;
    }

    /// Convert a byte offset into a 0-based (line, character) LSP position.
    /// Returns (line, character).
    pub fn offset_to_position(&self, offset: usize) -> (u32, u32) {
        let bytes = self.text.as_bytes();
        let clamped = offset.min(bytes.len());
        let mut line = 0u32;
        let mut col = 0u32;
        for (i, &b) in bytes[..clamped].iter().enumerate() {
            if b == b'\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
            let _ = i;
        }
        (line, col)
    }

    /// Convert a 0-based (line, character) LSP position to a byte offset.
    pub fn position_to_offset(&self, line: u32, character: u32) -> usize {
        let mut remaining_lines = line as usize;
        let mut offset = 0usize;
        for ch in self.text.chars() {
            if remaining_lines == 0 {
                break;
            }
            offset += ch.len_utf8();
            if ch == '\n' {
                remaining_lines -= 1;
            }
        }
        // Now advance character columns
        let line_start = offset;
        let mut col = 0u32;
        let remaining = &self.text[line_start..];
        for ch in remaining.chars() {
            if col >= character || ch == '\n' {
                break;
            }
            offset += ch.len_utf8();
            col += 1;
        }
        offset
    }

    /// Return the word (identifier) at the given 0-based position.
    pub fn word_at(&self, line: u32, character: u32) -> Option<String> {
        let ln = self.lines.get(line as usize)?;
        let col = character as usize;
        if col > ln.len() {
            return None;
        }

        // Find word boundaries using Mantra identifier chars: [a-zA-Z0-9_]
        let is_ident = |c: char| c.is_alphanumeric() || c == '_';

        let start = ln[..col]
            .rfind(|c: char| !is_ident(c))
            .map(|i| i + 1)
            .unwrap_or(0);

        let end = ln[col..]
            .find(|c: char| !is_ident(c))
            .map(|i| col + i)
            .unwrap_or(ln.len());

        let word = &ln[start..end];
        if word.is_empty() {
            None
        } else {
            Some(word.to_string())
        }
    }

    /// Return the word and its character range [start_col, end_col) at position.
    pub fn word_range_at(&self, line: u32, character: u32) -> Option<(String, u32, u32)> {
        let ln = self.lines.get(line as usize)?;
        let col = character as usize;
        if col > ln.len() {
            return None;
        }
        let is_ident = |c: char| c.is_alphanumeric() || c == '_';
        let start = ln[..col]
            .rfind(|c: char| !is_ident(c))
            .map(|i| i + 1)
            .unwrap_or(0);
        let end = ln[col..]
            .find(|c: char| !is_ident(c))
            .map(|i| col + i)
            .unwrap_or(ln.len());
        let word = &ln[start..end];
        if word.is_empty() {
            None
        } else {
            Some((word.to_string(), start as u32, end as u32))
        }
    }

    /// Return text from line start up to (not including) the cursor column.
    pub fn line_prefix(&self, line: u32, character: u32) -> &str {
        let ln = self.lines.get(line as usize).map(|s| s.as_str()).unwrap_or("");
        let end = (character as usize).min(ln.len());
        &ln[..end]
    }

    /// Return full text of a line (0-based).
    pub fn line_text(&self, line: u32) -> &str {
        self.lines.get(line as usize).map(|s| s.as_str()).unwrap_or("")
    }
}

/// Manages all open documents in the workspace
#[derive(Default)]
pub struct DocumentManager {
    documents: HashMap<String, Document>,
}

impl DocumentManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(&mut self, uri: String, text: String, version: i64) {
        self.documents.insert(uri.clone(), Document::new(uri, text, version));
    }

    pub fn close(&mut self, uri: &str) {
        self.documents.remove(uri);
    }

    pub fn update(&mut self, uri: &str, text: String, version: i64) {
        if let Some(doc) = self.documents.get_mut(uri) {
            doc.update(text, version);
        }
    }

    pub fn get(&self, uri: &str) -> Option<&Document> {
        self.documents.get(uri)
    }

    pub fn all_uris(&self) -> Vec<String> {
        self.documents.keys().cloned().collect()
    }
}

fn split_lines(text: &str) -> Vec<String> {
    let mut lines: Vec<String> = text.lines().map(|l| l.to_string()).collect();
    // Ensure there's always at least one line
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

