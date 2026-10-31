// ==============================================================================
// mantra_lsp::code_actions — Code Actions & Quick Fixes
// ==============================================================================

use crate::analysis::AnalysisResult;
use crate::document::Document;
use crate::protocol::{escape_json_str, LspRange};

pub struct CodeAction {
    pub title: String,
    pub kind: String,
    pub edit_json: Option<String>,
    pub command: Option<String>,
}

impl CodeAction {
    pub fn to_json(&self) -> String {
        let edit = self
            .edit_json
            .as_deref()
            .map(|e| format!(r#","edit":{}"#, e))
            .unwrap_or_default();
        let cmd = self
            .command
            .as_deref()
            .map(|c| format!(r#","command":{{"command":"{}","title":"{}"}}"#, escape_json_str(c), escape_json_str(&self.title)))
            .unwrap_or_default();
        format!(
            r#"{{"title":"{}","kind":"{}"{}{}}}"#,
            escape_json_str(&self.title),
            escape_json_str(&self.kind),
            edit,
            cmd,
        )
    }
}

pub fn get_code_actions(
    document: &Document,
    _range: &LspRange,
    diagnostics_json: &[String],
    _analysis: &AnalysisResult,
) -> Vec<CodeAction> {
    let mut actions = Vec::new();

    // ── Organize imports ─────────────────────────────────────────────────
    let organized = organize_imports(document);
    if let Some(new_text) = organized {
        let total_lines = document.lines.len() as u32;
        let last_col = document.lines.last().map(|l| l.len() as u32).unwrap_or(0);
        let edit_json = format!(
            r#"{{"changes":{{"{}":[{{"range":{{"start":{{"line":0,"character":0}},"end":{{"line":{},"character":{}}}}},"newText":"{}"}}]}}}}"#,
            escape_json_str(&document.uri),
            total_lines,
            last_col,
            escape_json_str(&new_text),
        );
        actions.push(CodeAction {
            title: "Organize Imports".to_string(),
            kind: "source.organizeImports".to_string(),
            edit_json: Some(edit_json),
            command: None,
        });
    }

    // ── Remove trailing whitespace ────────────────────────────────────────
    if document.text.lines().any(|l| l != l.trim_end()) {
        actions.push(CodeAction {
            title: "Remove Trailing Whitespace".to_string(),
            kind: "source".to_string(),
            edit_json: None,
            command: Some("mantra.removeTrailingWhitespace".to_string()),
        });
    }

    // ── Diagnostic-based quick fixes ─────────────────────────────────────
    for diag in diagnostics_json {
        if diag.contains("undefined") || diag.contains("Undefined") {
            actions.push(CodeAction {
                title: "Add import for undefined symbol".to_string(),
                kind: "quickfix".to_string(),
                edit_json: None,
                command: Some("mantra.addImport".to_string()),
            });
            break;
        }
    }

    actions
}

pub fn code_actions_to_json(actions: &[CodeAction]) -> String {
    let items: Vec<String> = actions.iter().map(|a| a.to_json()).collect();
    format!("[{}]", items.join(","))
}

fn organize_imports(document: &Document) -> Option<String> {
    let mut import_lines: Vec<&str> = Vec::new();
    let mut other_lines: Vec<&str> = Vec::new();
    let mut changed = false;

    for line in &document.lines {
        let trimmed = line.trim();
        if trimmed.starts_with("import ") || trimmed.starts_with("from ") {
            import_lines.push(line.as_str());
        } else {
            other_lines.push(line.as_str());
        }
    }

    if import_lines.is_empty() {
        return None;
    }

    let mut sorted = import_lines.clone();
    sorted.sort_unstable();
    sorted.dedup();

    if sorted != import_lines {
        changed = true;
    }

    if !changed {
        return None;
    }

    let mut result = sorted.join("\n");
    if !other_lines.is_empty() {
        result.push('\n');
        result.push_str(&other_lines.join("\n"));
    }
    Some(result)
}

