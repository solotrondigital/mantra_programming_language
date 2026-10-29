// ==============================================================================
// mantra_lsp::protocol — Language Server Protocol Data Types & JSON-RPC
// ==============================================================================

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
    pub severity: u32, // 1: Error, 2: Warning, 3: Information, 4: Hint
    pub code: Option<String>,
    pub source: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct LspCompletionItem {
    pub label: String,
    pub kind: u32, // 1: Text, 2: Method, 3: Function, 6: Variable, 7: Class, 14: Keyword
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

