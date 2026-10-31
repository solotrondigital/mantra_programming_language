// ==============================================================================
// mantra_lsp::protocol — Language Server Protocol 3.17 Data Types & JSON-RPC
// ==============================================================================

// ---------------------------------------------------------------------------
// Backward-compatible types (used by existing tests)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct LspPosition {
    pub line: u32,
    pub character: u32,
}

impl LspPosition {
    pub fn new(line: u32, character: u32) -> Self {
        Self { line, character }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LspRange {
    pub start: LspPosition,
    pub end: LspPosition,
}

impl LspRange {
    pub fn new(start_line: u32, start_col: u32, end_line: u32, end_col: u32) -> Self {
        Self {
            start: LspPosition::new(start_line, start_col),
            end: LspPosition::new(end_line, end_col),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LspDiagnostic {
    pub range: LspRange,
    pub severity: u32,
    pub code: Option<String>,
    pub source: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct LspCompletionItem {
    pub label: String,
    pub kind: u32,
    pub detail: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LspHover {
    pub contents: String,
    pub range: Option<LspRange>,
}

#[derive(Debug, Clone)]
pub struct LspTextEdit {
    pub range: LspRange,
    pub new_text: String,
}

impl LspDiagnostic {
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"range":{{"start":{{"line":{},"character":{}}},"end":{{"line":{},"character":{}}}}},"severity":{},"code":"{}","source":"{}","message":"{}"}}"#,
            self.range.start.line,
            self.range.start.character,
            self.range.end.line,
            self.range.end.character,
            self.severity,
            self.code.as_deref().unwrap_or(""),
            self.source,
            self.message.replace('"', "\\\"").replace('\n', " ")
        )
    }
}

impl LspCompletionItem {
    pub fn to_json(&self) -> String {
        let detail_str = self
            .detail
            .as_deref()
            .map(|d| format!(r#","detail":"{}""#, d.replace('"', "\\\"")))
            .unwrap_or_default();
        let doc_str = self
            .documentation
            .as_deref()
            .map(|doc| format!(r#","documentation":"{}""#, doc.replace('"', "\\\"")))
            .unwrap_or_default();
        format!(
            r#"{{"label":"{}","kind":{}{}{}}}"#,
            self.label, self.kind, detail_str, doc_str
        )
    }
}

// ---------------------------------------------------------------------------
// Full LSP 3.17 types (used by the new server implementation)
// ---------------------------------------------------------------------------

/// DiagnosticSeverity constants
pub mod severity {
    pub const ERROR: u32 = 1;
    pub const WARNING: u32 = 2;
    pub const INFORMATION: u32 = 3;
    pub const HINT: u32 = 4;
}

/// CompletionItemKind constants
pub mod completion_kind {
    pub const TEXT: u32 = 1;
    pub const METHOD: u32 = 2;
    pub const FUNCTION: u32 = 3;
    pub const CONSTRUCTOR: u32 = 4;
    pub const FIELD: u32 = 5;
    pub const VARIABLE: u32 = 6;
    pub const CLASS: u32 = 7;
    pub const INTERFACE: u32 = 8;
    pub const MODULE: u32 = 9;
    pub const PROPERTY: u32 = 10;
    pub const UNIT: u32 = 11;
    pub const VALUE: u32 = 12;
    pub const ENUM: u32 = 13;
    pub const KEYWORD: u32 = 14;
    pub const SNIPPET: u32 = 15;
    pub const COLOR: u32 = 16;
    pub const FILE: u32 = 17;
    pub const REFERENCE: u32 = 18;
    pub const FOLDER: u32 = 19;
    pub const ENUM_MEMBER: u32 = 20;
    pub const CONSTANT: u32 = 21;
    pub const STRUCT: u32 = 22;
    pub const EVENT: u32 = 23;
    pub const OPERATOR: u32 = 24;
    pub const TYPE_PARAMETER: u32 = 25;
}

/// SymbolKind constants
pub mod symbol_kind {
    pub const FILE: u32 = 1;
    pub const MODULE: u32 = 2;
    pub const NAMESPACE: u32 = 3;
    pub const PACKAGE: u32 = 4;
    pub const CLASS: u32 = 5;
    pub const METHOD: u32 = 6;
    pub const PROPERTY: u32 = 7;
    pub const FIELD: u32 = 8;
    pub const CONSTRUCTOR: u32 = 9;
    pub const ENUM: u32 = 10;
    pub const INTERFACE: u32 = 11;
    pub const FUNCTION: u32 = 12;
    pub const VARIABLE: u32 = 13;
    pub const CONSTANT: u32 = 14;
    pub const STRING: u32 = 15;
    pub const NUMBER: u32 = 16;
    pub const BOOLEAN: u32 = 17;
    pub const ARRAY: u32 = 18;
    pub const OBJECT: u32 = 19;
    pub const KEY: u32 = 20;
    pub const NULL: u32 = 21;
    pub const ENUM_MEMBER: u32 = 22;
    pub const STRUCT: u32 = 23;
    pub const EVENT: u32 = 24;
    pub const OPERATOR: u32 = 25;
    pub const TYPE_PARAMETER: u32 = 26;
}

/// InsertTextFormat constants
pub mod insert_text_format {
    pub const PLAIN_TEXT: u32 = 1;
    pub const SNIPPET: u32 = 2;
}

/// Semantic token type indices (must match TOKEN_TYPES order)
pub mod semantic_token_type {
    pub const NAMESPACE: u32 = 0;
    pub const CLASS: u32 = 1;
    pub const INTERFACE: u32 = 2;
    pub const ENUM: u32 = 3;
    pub const ENUM_MEMBER: u32 = 4;
    pub const STRUCT: u32 = 5;
    pub const METHOD: u32 = 6;
    pub const FUNCTION: u32 = 7;
    pub const CONSTRUCTOR: u32 = 8;
    pub const PROPERTY: u32 = 9;
    pub const FIELD: u32 = 10;
    pub const VARIABLE: u32 = 11;
    pub const PARAMETER: u32 = 12;
    pub const TYPE: u32 = 13;
    pub const TYPE_PARAMETER: u32 = 14;
    pub const KEYWORD: u32 = 15;
    pub const MODIFIER: u32 = 16;
    pub const COMMENT: u32 = 17;
    pub const STRING: u32 = 18;
    pub const NUMBER: u32 = 19;
    pub const OPERATOR: u32 = 20;
    pub const DECORATOR: u32 = 21;
}

/// Semantic token modifier bits
pub mod semantic_token_modifier {
    pub const DECLARATION: u32 = 1 << 0;
    pub const DEFINITION: u32 = 1 << 1;
    pub const READONLY: u32 = 1 << 2;
    pub const STATIC: u32 = 1 << 3;
    pub const ABSTRACT: u32 = 1 << 4;
    pub const DEPRECATED: u32 = 1 << 5;
    pub const ASYNC: u32 = 1 << 6;
    pub const MODIFICATION: u32 = 1 << 7;
    pub const DOCUMENTATION: u32 = 1 << 8;
    pub const DEFAULT_LIBRARY: u32 = 1 << 9;
}

pub const TOKEN_TYPES: &[&str] = &[
    "namespace",
    "class",
    "interface",
    "enum",
    "enumMember",
    "struct",
    "method",
    "function",
    "constructor",
    "property",
    "field",
    "variable",
    "parameter",
    "type",
    "typeParameter",
    "keyword",
    "modifier",
    "comment",
    "string",
    "number",
    "operator",
    "decorator",
];

pub const TOKEN_MODIFIERS: &[&str] = &[
    "declaration",
    "definition",
    "readonly",
    "static",
    "abstract",
    "deprecated",
    "async",
    "modification",
    "documentation",
    "defaultLibrary",
];

// ---------------------------------------------------------------------------
// JSON helpers
// ---------------------------------------------------------------------------

pub fn escape_json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

pub fn extract_json_id(msg: &str) -> Option<i64> {
    let v: serde_json::Value = serde_json::from_str(msg).ok()?;
    match &v["id"] {
        serde_json::Value::Number(n) => n.as_i64(),
        serde_json::Value::String(s) => s.parse().ok(),
        _ => None,
    }
}

pub fn null_response(id: i64) -> String {
    format!(r#"{{"jsonrpc":"2.0","id":{id},"result":null}}"#)
}

pub fn error_response(id: i64, code: i32, message: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":{id},"error":{{"code":{code},"message":"{}"}}}}"#,
        escape_json_str(message)
    )
}

pub fn make_notification(method: &str, params: &str) -> String {
    format!(r#"{{"jsonrpc":"2.0","method":"{method}","params":{params}}}"#)
}
