// ==============================================================================
// mantra_lsp::server — Language Server Core Engine
// ==============================================================================

use crate::protocol::*;
use mantrac_checker::TypeChecker;
use mantrac_lexer::Lexer;
use mantrac_parser::Parser;
use std::collections::HashMap;

pub struct LspServer {
    pub documents: HashMap<String, String>,
    pub initialized: bool,
}

impl Default for LspServer {
    fn default() -> Self {
        Self::new()
    }
}

impl LspServer {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
            initialized: false,
        }
    }

    pub fn open_document(&mut self, uri: String, text: String) -> Vec<LspDiagnostic> {
        self.documents.insert(uri.clone(), text.clone());
        self.compute_diagnostics(&text)
    }

    pub fn update_document(&mut self, uri: &str, text: String) -> Vec<LspDiagnostic> {
        self.documents.insert(uri.to_string(), text.clone());
        self.compute_diagnostics(&text)
    }

    pub fn compute_diagnostics(&self, text: &str) -> Vec<LspDiagnostic> {
        let mut diags = Vec::new();

        // Phase 1: Lexing
        let mut lexer = Lexer::new(text);
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                let line = if e.span.line > 0 { e.span.line - 1 } else { 0 } as u32;
                let col = if e.span.col > 0 { e.span.col - 1 } else { 0 } as u32;
                diags.push(LspDiagnostic {
                    range: LspRange::new(line, col, line, col + 1),
                    severity: 1, // Error
                    code: Some("E0001".to_string()),
                    source: "mantra-lexer".to_string(),
                    message: e.message,
                });
                return diags;
            }
        };

        // Phase 2: Parsing
        let mut parser = Parser::new(tokens);
        let program = match parser.parse() {
            Ok(p) => p,
            Err(e) => {
                let line = if e.span.line > 0 { e.span.line - 1 } else { 0 } as u32;
                let col = if e.span.col > 0 { e.span.col - 1 } else { 0 } as u32;
                diags.push(LspDiagnostic {
                    range: LspRange::new(line, col, line, col + 1),
                    severity: 1, // Error
                    code: Some("E0002".to_string()),
                    source: "mantra-parser".to_string(),
                    message: e.message,
                });
                return diags;
            }
        };

        // Phase 3: Type Checking
        let mut checker = TypeChecker::new();
        if let Err(type_diags) = checker.check_program(&program) {
            for td in type_diags {
                let line = if td.span.line > 0 { td.span.line - 1 } else { 0 } as u32;
                let col = if td.span.col > 0 { td.span.col - 1 } else { 0 } as u32;
                diags.push(LspDiagnostic {
                    range: LspRange::new(line, col, line, col + 1),
                    severity: match td.level {
                        mantrac_checker::diagnostics::DiagnosticLevel::Error => 1,
                        mantrac_checker::diagnostics::DiagnosticLevel::Warning => 2,
                        mantrac_checker::diagnostics::DiagnosticLevel::Note => 3,
                    },
                    code: Some(td.code.to_string()),
                    source: "mantra-checker".to_string(),
                    message: td.message,
                });
            }
        }

        diags
    }

    pub fn get_completions(&self, uri: &str, _pos: &LspPosition) -> Vec<LspCompletionItem> {
        let mut items = Vec::new();

        // 1. Language Keywords
        let keywords = [
            ("def", "Define a new function or method"),
            ("class", "Define a class with fields, constructor, and methods"),
            ("interface", "Define an abstract interface contract"),
            ("return", "Return a value from function"),
            ("if", "Conditional branching statement"),
            ("elif", "Else-if conditional branch"),
            ("else", "Fallback branch"),
            ("while", "Loop while condition holds true"),
            ("for", "Iterate over collection"),
            ("in", "Membership iteration keyword"),
            ("async", "Declare an asynchronous function or coroutine"),
            ("await", "Pause execution until task completes"),
            ("pub", "Public visibility modifier"),
            ("private", "Private visibility modifier (class-scoped)"),
            ("protected", "Protected visibility modifier"),
            ("static", "Static member modifier"),
            ("abstract", "Abstract class or method modifier"),
            ("override", "Explicit method override modifier"),
            ("extends", "Class inheritance declaration"),
            ("implements", "Interface implementation declaration"),
            ("let", "Immutable variable binding"),
            ("mut", "Mutable variable binding"),
        ];

        for (kw, doc) in keywords {
            items.push(LspCompletionItem {
                label: kw.to_string(),
                kind: 14, // Keyword
                detail: Some("Keyword".to_string()),
                documentation: Some(doc.to_string()),
            });
        }

        // 2. Built-in functions
        let builtins = [
            ("print(value)", "Print formatted value to standard output"),
            ("len(collection)", "Return length of list, string, or map"),
            ("range(n)", "Return integer range from 0 to n-1"),
            ("str(val)", "Convert value to string representation"),
            ("int(val)", "Convert value to 64-bit integer"),
            ("float(val)", "Convert value to 64-bit float"),
            ("assert(condition)", "Validate condition; panic on assertion failure"),
            ("read_file(path)", "Read entire file contents into string"),
            ("write_file(path, content)", "Write string content to file (returns Bool)"),
            ("system_exec(cmd)", "Execute shell command (returns status code Int)"),
            ("get_env(key, default_val)", "Retrieve environment variable with fallback"),
        ];

        for (b, doc) in builtins {
            let label = b.split('(').next().unwrap_or(b);
            items.push(LspCompletionItem {
                label: label.to_string(),
                kind: 3, // Function
                detail: Some(b.to_string()),
                documentation: Some(doc.to_string()),
            });
        }

        // 3. Document symbols
        if let Some(content) = self.documents.get(uri) {
            for line in content.lines() {
                let trimmed = line.trim();
                if let Some(rest) = trimmed.strip_prefix("class ") {
                    let name = rest.split([' ', ':', '(']).next().unwrap_or(rest);
                    items.push(LspCompletionItem {
                        label: name.to_string(),
                        kind: 7, // Class
                        detail: Some("Class declaration".to_string()),
                        documentation: None,
                    });
                } else if let Some(rest) = trimmed.strip_prefix("def ") {
                    let name = rest.split(['(', ':']).next().unwrap_or(rest);
                    items.push(LspCompletionItem {
                        label: name.to_string(),
                        kind: 2, // Method/Function
                        detail: Some("Function declaration".to_string()),
                        documentation: None,
                    });
                }
            }
        }

        items
    }

    pub fn get_hover(&self, uri: &str, pos: &LspPosition) -> Option<LspHover> {
        let content = self.documents.get(uri)?;
        let lines: Vec<&str> = content.lines().collect();
        let line = lines.get(pos.line as usize)?;

        // Find word at character position
        let col = pos.character as usize;
        if col >= line.len() {
            return None;
        }

        let start = line[..col]
            .rfind(|c: char| !c.is_alphanumeric() && c != '_')
            .map(|i| i + 1)
            .unwrap_or(0);
        let end = line[col..]
            .find(|c: char| !c.is_alphanumeric() && c != '_')
            .map(|i| col + i)
            .unwrap_or(line.len());

        let word = &line[start..end];
        if word.is_empty() {
            return None;
        }

        let hover_text = match word {
            "def" => "```mantra\ndef name(params) -> ReturnType:\n```\nDeclares a function or method in Mantra.",
            "class" => "```mantra\nclass Name [extends Base] [implements Interface]:\n```\nDeclares an object-oriented class with encapsulated fields, constructors, and methods.",
            "interface" => "```mantra\ninterface Name:\n```\nDeclares an abstract interface with method signatures.",
            "async" => "```mantra\nasync def name() -> Task[T]:\n```\nDeclares an asynchronous task that can be awaited.",
            "await" => "```mantra\nresult = await task\n```\nSuspends current coroutine until the asynchronous task resolves.",
            "print" => "```mantra\nprint(val: Any) -> Unit\n```\nPrints value to standard output followed by a newline.",
            "read_file" => "```mantra\nread_file(path: String) -> String\n```\nReads file contents from disk into a string.",
            "write_file" => "```mantra\nwrite_file(path: String, content: String) -> Bool\n```\nWrites string contents to file on disk.",
            "system_exec" => "```mantra\nsystem_exec(cmd: String) -> Int\n```\nExecutes shell command and returns the process exit code.",
            "assert" => "```mantra\nassert(cond: Bool) -> Unit\n```\nValidates boolean invariant at runtime.",
            _ => return None,
        };

        Some(LspHover {
            contents: hover_text.to_string(),
            range: Some(LspRange::new(
                pos.line,
                start as u32,
                pos.line,
                end as u32,
            )),
        })
    }

    pub fn format_document(&self, uri: &str) -> Option<Vec<LspTextEdit>> {
        let content = self.documents.get(uri)?;
        let mut formatted = String::new();
        let lines: Vec<&str> = content.lines().collect();

        for line in &lines {
            let trimmed = line.trim_end();
            if trimmed.is_empty() {
                formatted.push('\n');
                continue;
            }

            let mut leading_spaces = 0;
            for ch in trimmed.chars() {
                if ch == ' ' {
                    leading_spaces += 1;
                } else if ch == '\t' {
                    leading_spaces += 4;
                } else {
                    break;
                }
            }

            let normalized = (leading_spaces / 4) * 4;
            formatted.push_str(&" ".repeat(normalized));
            formatted.push_str(trimmed.trim_start());
            formatted.push('\n');
        }

        let total_lines = lines.len() as u32;
        let last_line_len = lines.last().map(|l| l.len() as u32).unwrap_or(0);

        Some(vec![LspTextEdit {
            range: LspRange::new(0, 0, total_lines, last_line_len),
            new_text: formatted,
        }])
    }

    /// Process incoming JSON-RPC 2.0 message and return response string if required
    pub fn handle_message(&mut self, msg: &str) -> Option<String> {
        if msg.contains(r#""method":"initialize""#) {
            self.initialized = true;
            let id = extract_json_id(msg).unwrap_or(1);
            let resp = format!(
                r#"{{"jsonrpc":"2.0","id":{},"result":{{"capabilities":{{"textDocumentSync":1,"completionProvider":{{"resolveProvider":false,"triggerCharacters":["." ,":"]}},"hoverProvider":true,"documentFormattingProvider":true}}}}}}"#,
                id
            );
            return Some(resp);
        }

        if msg.contains(r#""method":"shutdown""#) {
            let id = extract_json_id(msg).unwrap_or(1);
            return Some(format!(r#"{{"jsonrpc":"2.0","id":{},"result":null}}"#, id));
        }

        if msg.contains(r#""method":"textDocument/completion""#) {
            let id = extract_json_id(msg).unwrap_or(1);
            let completions = self.get_completions("", &LspPosition::new(0, 0));
            let items_json: Vec<String> = completions.iter().map(|c| c.to_json()).collect();
            return Some(format!(
                r#"{{"jsonrpc":"2.0","id":{},"result":[{}]}}"#,
                id,
                items_json.join(",")
            ));
        }

        if msg.contains(r#""method":"textDocument/hover""#) {
            let id = extract_json_id(msg).unwrap_or(1);
            return Some(format!(
                r#"{{"jsonrpc":"2.0","id":{},"result":{{"contents":"Mantra Language Symbol"}}}}"#,
                id
            ));
        }

        None
    }
}

fn extract_json_id(msg: &str) -> Option<i64> {
    if let Some(pos) = msg.find(r#""id":"#) {
        let rest = &msg[pos + 5..];
        let id_str: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        id_str.parse().ok()
    } else {
        None
    }
}
