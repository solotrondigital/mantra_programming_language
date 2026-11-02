use crate::error::{MlError, MlResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub source: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub id: String,
    pub document_id: String,
    pub source: String,
    pub index: usize,
    pub start_char: usize,
    pub end_char: usize,
    pub text: String,
}

#[derive(Debug, Clone, Copy)]
pub struct ChunkOptions {
    pub max_chars: usize,
    pub overlap_chars: usize,
}

impl Default for ChunkOptions {
    fn default() -> Self {
        Self {
            max_chars: 1000,
            overlap_chars: 120,
        }
    }
}

pub struct DocumentProcessor;

impl DocumentProcessor {
    pub fn from_pdf_bytes(
        id: impl Into<String>,
        source: impl Into<String>,
        bytes: &[u8],
    ) -> MlResult<Document> {
        Self::from_text(id, source, extract_pdf_text(bytes)?)
    }

    pub fn load(path: impl AsRef<Path>) -> MlResult<Document> {
        let path = path.as_ref();
        let bytes = fs::read(path).map_err(|e| MlError::Io(e.to_string()))?;
        let text = match path
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or("")
            .to_ascii_lowercase()
            .as_str()
        {
            "txt" | "md" | "markdown" | "csv" | "json" | "log" => String::from_utf8(bytes)
                .map_err(|e| MlError::InvalidInput(format!("document is not valid UTF-8: {e}")))?,
            "pdf" => extract_pdf_text(&bytes)?,
            extension => {
                return Err(MlError::InvalidInput(format!(
                    "unsupported document extension '.{extension}'"
                )))
            }
        };
        let id = stable_id(&path.to_string_lossy());
        Self::from_text(id, path.to_string_lossy(), text)
    }

    pub fn from_text(
        id: impl Into<String>,
        source: impl Into<String>,
        text: impl Into<String>,
    ) -> MlResult<Document> {
        let id = id.into();
        let source = source.into();
        let text = normalize_text(&text.into());
        if id.trim().is_empty() || text.trim().is_empty() {
            return Err(MlError::InvalidInput(
                "document ID and extracted text must be non-empty".into(),
            ));
        }
        Ok(Document { id, source, text })
    }

    pub fn chunk(document: &Document, options: ChunkOptions) -> MlResult<Vec<Chunk>> {
        if options.max_chars == 0 || options.overlap_chars >= options.max_chars {
            return Err(MlError::InvalidInput(
                "chunk size must be positive and overlap must be smaller than chunk size".into(),
            ));
        }
        let chars: Vec<char> = document.text.chars().collect();
        if chars.is_empty() {
            return Ok(Vec::new());
        }
        let mut chunks = Vec::new();
        let mut start = 0usize;
        while start < chars.len() {
            let hard_end = (start + options.max_chars).min(chars.len());
            let end = if hard_end == chars.len() {
                hard_end
            } else {
                (start + options.max_chars / 2..hard_end)
                    .rev()
                    .find(|index| chars[*index].is_whitespace())
                    .unwrap_or(hard_end)
            };
            let content: String = chars[start..end]
                .iter()
                .collect::<String>()
                .trim()
                .to_string();
            if !content.is_empty() {
                let index = chunks.len();
                chunks.push(Chunk {
                    id: format!("{}:{index}", document.id),
                    document_id: document.id.clone(),
                    source: document.source.clone(),
                    index,
                    start_char: start,
                    end_char: end,
                    text: content,
                });
            }
            if end == chars.len() {
                break;
            }
            let next_start = end.saturating_sub(options.overlap_chars);
            start = if next_start <= start { end } else { next_start };
        }
        Ok(chunks)
    }
}

fn normalize_text(text: &str) -> String {
    text.replace("\r\n", "\n")
        .replace('\r', "\n")
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

fn stable_id(text: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in text.bytes() {
        hash = (hash ^ byte as u64).wrapping_mul(0x100000001b3);
    }
    format!("doc-{hash:016x}")
}

/// Extract literal text from uncompressed PDF text operators. For compressed,
/// scanned, or encrypted PDFs callers should supply a production PdfExtractor.
fn extract_pdf_text(bytes: &[u8]) -> MlResult<String> {
    if !bytes.starts_with(b"%PDF-") {
        return Err(MlError::InvalidInput(
            "file does not have a PDF header".into(),
        ));
    }
    let source = String::from_utf8_lossy(bytes);
    let mut output = String::new();
    let mut cursor = 0usize;
    let bytes = source.as_bytes();
    while cursor < bytes.len() {
        if bytes[cursor] != b'(' {
            cursor += 1;
            continue;
        }
        let (literal, end) = parse_pdf_literal(bytes, cursor)?;
        cursor = end;
        let mut lookahead = cursor;
        while lookahead < bytes.len() && bytes[lookahead].is_ascii_whitespace() {
            lookahead += 1;
        }
        let is_text_operand = bytes.get(lookahead..lookahead + 2) == Some(b"Tj")
            || bytes.get(lookahead..lookahead + 2) == Some(b"TJ")
            || lookahead < bytes.len() && bytes[lookahead] == b'(';
        if is_text_operand && !literal.trim().is_empty() {
            if !output.is_empty() {
                output.push(' ');
            }
            output.push_str(&literal);
        }
    }
    let extracted = normalize_text(&output);
    if extracted.is_empty() {
        return Err(MlError::InvalidInput("no extractable text found in PDF; compressed/scanned PDFs need a PDF extraction backend".into()));
    }
    Ok(extracted)
}

fn parse_pdf_literal(bytes: &[u8], start: usize) -> MlResult<(String, usize)> {
    let mut out = Vec::new();
    let mut cursor = start + 1;
    let mut depth = 1usize;
    while cursor < bytes.len() {
        let byte = bytes[cursor];
        cursor += 1;
        match byte {
            b'\\' if cursor < bytes.len() => {
                let escaped = bytes[cursor];
                cursor += 1;
                match escaped {
                    b'n' => out.push(b'\n'),
                    b'r' => out.push(b'\r'),
                    b't' => out.push(b'\t'),
                    b'b' => out.push(8),
                    b'f' => out.push(12),
                    b'(' | b')' | b'\\' => out.push(escaped),
                    b'\n' => {}
                    b'\r' => {
                        if bytes.get(cursor) == Some(&b'\n') {
                            cursor += 1;
                        }
                    }
                    digit @ b'0'..=b'7' => {
                        let mut value = (digit - b'0') as u16;
                        for _ in 0..2 {
                            if let Some(next @ b'0'..=b'7') = bytes.get(cursor).copied() {
                                value = value * 8 + (next - b'0') as u16;
                                cursor += 1;
                            } else {
                                break;
                            }
                        }
                        out.push(value as u8);
                    }
                    other => out.push(other),
                }
            }
            b'(' => {
                depth += 1;
                out.push(byte);
            }
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Ok((String::from_utf8_lossy(&out).to_string(), cursor));
                }
                out.push(byte);
            }
            _ => out.push(byte),
        }
    }
    Err(MlError::InvalidInput(
        "unterminated PDF text literal".into(),
    ))
}
