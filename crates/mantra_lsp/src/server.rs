// ==============================================================================
// mantra_lsp::server — Full LSP 3.17 Server (JSON-RPC dispatch)
// ==============================================================================

use crate::analysis::{analyze, AnalysisResult};
use crate::code_actions::{code_actions_to_json, get_code_actions};
use crate::completion::{completions_to_json, get_completions};
use crate::document::{Document, DocumentManager};
use crate::formatting::{format_document as fmt_doc, format_range as fmt_range};
use crate::hover::get_hover;
use crate::navigation::{
    find_references, go_to_definition, go_to_implementation, locations_to_json,
};
use crate::path_utils::{file_path_to_uri, uri_to_file_path};
use crate::protocol::*;
use crate::rename::{prepare_rename, rename_symbol};
use crate::semantic_tokens::get_semantic_tokens_json;
use crate::signature_help::get_signature_help_json;
use crate::symbol_index::SymbolIndex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// ─── Backward-compatible types (used by existing tests) ──────────────────────

pub struct LspServer {
    pub initialized: bool,
    pub documents: DocumentManager,
    pub symbol_index: SymbolIndex,
    /// Per-URI cached analysis results
    analyses: HashMap<String, AnalysisResult>,
    /// Workspace root URI
    workspace_root: Option<String>,
    /// Whether the workspace directory tree has been scanned
    workspace_scanned: bool,
    /// Pending notifications to flush (diagnostics, etc.)
    pending_notifications: Vec<String>,
}

impl Default for LspServer {
    fn default() -> Self {
        Self::new()
    }
}

impl LspServer {
    pub fn new() -> Self {
        Self {
            initialized: false,
            documents: DocumentManager::new(),
            symbol_index: SymbolIndex::new(),
            analyses: HashMap::new(),
            workspace_root: None,
            workspace_scanned: false,
            pending_notifications: Vec::new(),
        }
    }

    // ─── Core message dispatcher ──────────────────────────────────────────

    /// Process one raw JSON-RPC message body.
    /// Returns the primary response string (if any).
    /// Side-effect notifications (publishDiagnostics) are accumulated
    /// and should be flushed separately via `take_notifications()`.
    pub fn handle_message(&mut self, msg: &str) -> Option<String> {
        let v: serde_json::Value = match serde_json::from_str(msg) {
            Ok(v) => v,
            Err(_) => {
                let resp = error_response(-1, -32700, "Parse error");
                return Some(resp);
            }
        };

        let method = v["method"].as_str().unwrap_or("").to_string();
        let id = v["id"].clone();
        let params = v["params"].clone();

        match method.as_str() {
            "initialize" => Some(self.handle_initialize(id, params)),
            "initialized" => {
                self.initialized = true;
                None
            }
            "shutdown" => Some(self.handle_shutdown(id)),
            "exit" => None,

            // Text document lifecycle
            "textDocument/didOpen" => {
                self.handle_did_open(params);
                None
            }
            "textDocument/didChange" => {
                self.handle_did_change(params);
                None
            }
            "textDocument/didClose" => {
                self.handle_did_close(params);
                None
            }
            "textDocument/didSave" => {
                self.handle_did_save(params);
                None
            }

            // Language features
            "textDocument/completion" => Some(self.handle_completion(id, params)),
            "textDocument/hover" => Some(self.handle_hover(id, params)),
            "textDocument/definition" => Some(self.handle_definition(id, params)),
            "textDocument/declaration" => Some(self.handle_definition(id, params)),
            "textDocument/implementation" => Some(self.handle_implementation(id, params)),
            "textDocument/references" => Some(self.handle_references(id, params)),
            "textDocument/documentSymbol" => Some(self.handle_document_symbol(id, params)),
            "workspace/symbol" => Some(self.handle_workspace_symbol(id, params)),
            "textDocument/formatting" => Some(self.handle_formatting(id, params)),
            "textDocument/rangeFormatting" => Some(self.handle_range_formatting(id, params)),
            "textDocument/onTypeFormatting" => Some(self.handle_formatting(id, params)),
            "textDocument/semanticTokens/full" => Some(self.handle_semantic_tokens(id, params)),
            "textDocument/rename" => Some(self.handle_rename(id, params)),
            "textDocument/prepareRename" => Some(self.handle_prepare_rename(id, params)),
            "textDocument/codeAction" => Some(self.handle_code_action(id, params)),
            "textDocument/signatureHelp" => Some(self.handle_signature_help(id, params)),

            _ => {
                // Unknown request: send method not found only if it has an id
                if !id.is_null() {
                    Some(error_response(id.as_i64().unwrap_or(0), -32601, "Method not found"))
                } else {
                    None
                }
            }
        }
    }

    /// Return and clear any pending notifications (e.g. publishDiagnostics).
    pub fn take_notifications(&mut self) -> Vec<String> {
        std::mem::take(&mut self.pending_notifications)
    }

    // ─── Workspace discovery and background indexing ─────────────────────

    pub fn scan_workspace(&mut self, root_path: &Path) {
        if !root_path.exists() || !root_path.is_dir() {
            return;
        }

        let mut mantra_files = Vec::new();
        collect_mantra_files(root_path, &mut mantra_files, 0);

        for path in mantra_files {
            let uri = file_path_to_uri(&path);
            if self.documents.get(&uri).is_some() {
                continue;
            }
            if let Ok(text) = std::fs::read_to_string(&path) {
                let analysis = analyze(&text);
                self.symbol_index.update_file(&uri, &analysis);
                self.analyses.insert(uri.clone(), analysis);
                self.documents.open(uri, text, 0);
            }
        }
        self.workspace_scanned = true;
    }

    pub fn ensure_workspace_indexed(&mut self, current_uri: &str) {
        if self.workspace_scanned {
            return;
        }
        // Try workspace_root first
        if let Some(root_uri) = &self.workspace_root {
            if let Some(root_path) = uri_to_file_path(root_uri) {
                self.scan_workspace(&root_path);
                return;
            }
        }
        // Otherwise derive root from current document location
        if let Some(doc_path) = uri_to_file_path(current_uri) {
            let mut curr = doc_path.parent();
            let mut candidate = None;
            for _ in 0..6 {
                if let Some(p) = curr {
                    if p.join("mantra.toml").exists() || p.join("Cargo.toml").exists() {
                        candidate = Some(p.to_path_buf());
                        break;
                    }
                    curr = p.parent();
                } else {
                    break;
                }
            }
            let scan_root = candidate.or_else(|| doc_path.parent().map(|p| p.to_path_buf()));
            if let Some(root) = scan_root {
                self.scan_workspace(&root);
            }
        }
    }

    // ─── Handler implementations ──────────────────────────────────────────

    fn handle_initialize(&mut self, id: serde_json::Value, params: serde_json::Value) -> String {
        self.initialized = true;

        // Extract workspace root
        if let Some(root) = params["rootUri"].as_str() {
            self.workspace_root = Some(root.to_string());
        } else if let Some(root) = params["rootPath"].as_str() {
            self.workspace_root = Some(format!("file://{}", root));
        } else if let Some(folders) = params["workspaceFolders"].as_array() {
            if let Some(first) = folders.get(0).and_then(|f| f["uri"].as_str()) {
                self.workspace_root = Some(first.to_string());
            }
        }

        // Trigger eager indexing of workspace files
        if let Some(root_uri) = &self.workspace_root.clone() {
            if let Some(root_path) = uri_to_file_path(root_uri) {
                self.scan_workspace(&root_path);
            }
        }

        let token_types: Vec<String> = TOKEN_TYPES.iter().map(|s| format!("\"{}\"", s)).collect();
        let token_mods: Vec<String> = TOKEN_MODIFIERS.iter().map(|s| format!("\"{}\"", s)).collect();

        let id_val = id.as_i64().unwrap_or(1);
        format!(
            r#"{{"jsonrpc":"2.0","id":{id_val},"result":{{"capabilities":{{"textDocumentSync":{{"openClose":true,"change":1,"save":{{"includeText":true}}}},"completionProvider":{{"resolveProvider":false,"triggerCharacters":[".",":"," ","("]}},"hoverProvider":true,"definitionProvider":true,"declarationProvider":true,"implementationProvider":true,"referencesProvider":true,"documentSymbolProvider":true,"workspaceSymbolProvider":true,"documentFormattingProvider":true,"documentRangeFormattingProvider":true,"documentOnTypeFormattingProvider":{{"firstTriggerCharacter":"\n","moreTriggerCharacter":[":"]}},"semanticTokensProvider":{{"legend":{{"tokenTypes":[{}],"tokenModifiers":[{}]}},"full":true,"range":false}},"renameProvider":{{"prepareProvider":true}},"codeActionProvider":true,"signatureHelpProvider":{{"triggerCharacters":["(",","," "]}}}},"serverInfo":{{"name":"mantra-lsp","version":"0.7.0"}}}}}}"#,
            token_types.join(","),
            token_mods.join(","),
        )
    }

    fn handle_shutdown(&mut self, id: serde_json::Value) -> String {
        null_response(id.as_i64().unwrap_or(0))
    }

    fn handle_did_open(&mut self, params: serde_json::Value) {
        let uri = params["textDocument"]["uri"].as_str().unwrap_or("").to_string();
        let text = params["textDocument"]["text"].as_str().unwrap_or("").to_string();
        let version = params["textDocument"]["version"].as_i64().unwrap_or(0);
        self.documents.open(uri.clone(), text, version);
        self.analyze_and_publish(&uri);
    }

    fn handle_did_change(&mut self, params: serde_json::Value) {
        let uri = params["textDocument"]["uri"].as_str().unwrap_or("").to_string();
        let version = params["textDocument"]["version"].as_i64().unwrap_or(0);
        // We use full sync (TextDocumentSyncKind=1), so take first change
        if let Some(text) = params["contentChanges"][0]["text"].as_str() {
            self.documents.update(&uri, text.to_string(), version);
            self.analyze_and_publish(&uri);
        }
    }

    fn handle_did_close(&mut self, params: serde_json::Value) {
        let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
        self.documents.close(uri);
        // Clear diagnostics
        let notif = make_notification(
            "textDocument/publishDiagnostics",
            &format!(r#"{{"uri":"{}","diagnostics":[]}}"#, escape_json_str(uri)),
        );
        self.pending_notifications.push(notif);
    }

    fn handle_did_save(&mut self, params: serde_json::Value) {
        let uri = params["textDocument"]["uri"].as_str().unwrap_or("").to_string();
        if let Some(text) = params["text"].as_str() {
            self.documents.update(&uri, text.to_string(), 0);
        }
        self.analyze_and_publish(&uri);
    }

    fn handle_completion(&mut self, id: serde_json::Value, params: serde_json::Value) -> String {
        let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
        let line = params["position"]["line"].as_u64().unwrap_or(0) as u32;
        let character = params["position"]["character"].as_u64().unwrap_or(0) as u32;

        let id_val = id.as_i64().unwrap_or(0);

        let items_json = if let Some(doc) = self.documents.get(uri) {
            let analysis = self.analyses.get(uri).cloned().unwrap_or_else(|| analyze(&doc.text));
            let items = get_completions(doc, line, character, &analysis);
            completions_to_json(&items)
        } else {
            "[]".to_string()
        };

        format!(r#"{{"jsonrpc":"2.0","id":{id_val},"result":{items_json}}}"#)
    }

    fn handle_hover(&mut self, id: serde_json::Value, params: serde_json::Value) -> String {
        let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
        let line = params["position"]["line"].as_u64().unwrap_or(0) as u32;
        let character = params["position"]["character"].as_u64().unwrap_or(0) as u32;
        let id_val = id.as_i64().unwrap_or(0);

        if let Some(doc) = self.documents.get(uri) {
            let analysis = self.analyses.get(uri).cloned().unwrap_or_else(|| analyze(&doc.text));
            if let Some(hover) = get_hover(doc, line, character, &analysis) {
                return format!(
                    r#"{{"jsonrpc":"2.0","id":{id_val},"result":{}}}"#,
                    hover.to_json()
                );
            }
        }
        format!(r#"{{"jsonrpc":"2.0","id":{id_val},"result":null}}"#)
    }

    fn handle_definition(&mut self, id: serde_json::Value, params: serde_json::Value) -> String {
        let uri = params["textDocument"]["uri"].as_str().unwrap_or("").to_string();
        let line = params["position"]["line"].as_u64().unwrap_or(0) as u32;
        let character = params["position"]["character"].as_u64().unwrap_or(0) as u32;
        let id_val = id.as_i64().unwrap_or(0);

        self.ensure_workspace_indexed(&uri);

        let result = if let Some(doc) = self.documents.get(&uri) {
            let analysis = self.analyses.get(&uri).cloned().unwrap_or_else(|| analyze(&doc.text));
            match go_to_definition(doc, line, character, &analysis, &self.analyses) {
                Some(loc) => loc.to_json(),
                None => "null".to_string(),
            }
        } else {
            "null".to_string()
        };

        format!(r#"{{"jsonrpc":"2.0","id":{id_val},"result":{result}}}"#)
    }

    fn handle_implementation(&mut self, id: serde_json::Value, params: serde_json::Value) -> String {
        let uri = params["textDocument"]["uri"].as_str().unwrap_or("").to_string();
        let line = params["position"]["line"].as_u64().unwrap_or(0) as u32;
        let character = params["position"]["character"].as_u64().unwrap_or(0) as u32;
        let id_val = id.as_i64().unwrap_or(0);

        self.ensure_workspace_indexed(&uri);

        let result = if let Some(doc) = self.documents.get(&uri) {
            let analysis = self.analyses.get(&uri).cloned().unwrap_or_else(|| analyze(&doc.text));
            let locs = go_to_implementation(doc, line, character, &analysis, &self.analyses);
            locations_to_json(&locs)
        } else {
            "[]".to_string()
        };

        format!(r#"{{"jsonrpc":"2.0","id":{id_val},"result":{result}}}"#)
    }

    fn handle_references(&mut self, id: serde_json::Value, params: serde_json::Value) -> String {
        let uri = params["textDocument"]["uri"].as_str().unwrap_or("").to_string();
        let line = params["position"]["line"].as_u64().unwrap_or(0) as u32;
        let character = params["position"]["character"].as_u64().unwrap_or(0) as u32;
        let id_val = id.as_i64().unwrap_or(0);

        let result = if let Some(doc) = self.documents.get(&uri) {
            // Build document map for cross-file references
            let all_docs: HashMap<String, Document> = self
                .documents
                .all_uris()
                .into_iter()
                .filter_map(|u| {
                    self.documents.get(&u).map(|d| (u, d.clone()))
                })
                .collect();
            let locs = find_references(doc, line, character, &all_docs, &self.analyses);
            locations_to_json(&locs)
        } else {
            "[]".to_string()
        };

        format!(r#"{{"jsonrpc":"2.0","id":{id_val},"result":{result}}}"#)
    }

    fn handle_document_symbol(&mut self, id: serde_json::Value, params: serde_json::Value) -> String {
        let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
        let id_val = id.as_i64().unwrap_or(0);
        let result = self.symbol_index.document_symbols_json(uri);
        format!(r#"{{"jsonrpc":"2.0","id":{id_val},"result":{result}}}"#)
    }

    fn handle_workspace_symbol(&mut self, id: serde_json::Value, params: serde_json::Value) -> String {
        let query = params["query"].as_str().unwrap_or("");
        let id_val = id.as_i64().unwrap_or(0);
        let items = self.symbol_index.workspace_symbols_json(query);
        format!(r#"{{"jsonrpc":"2.0","id":{id_val},"result":[{items}]}}"#)
    }

    fn handle_formatting(&mut self, id: serde_json::Value, params: serde_json::Value) -> String {
        let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
        let id_val = id.as_i64().unwrap_or(0);

        let result = if let Some(doc) = self.documents.get(uri) {
            let edits = fmt_doc(doc);
            let json: Vec<String> = edits.iter().map(|e| e.to_json()).collect();
            format!("[{}]", json.join(","))
        } else {
            "[]".to_string()
        };

        format!(r#"{{"jsonrpc":"2.0","id":{id_val},"result":{result}}}"#)
    }

    fn handle_range_formatting(&mut self, id: serde_json::Value, params: serde_json::Value) -> String {
        let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
        let id_val = id.as_i64().unwrap_or(0);

        let result = if let Some(doc) = self.documents.get(uri) {
            let sl = params["range"]["start"]["line"].as_u64().unwrap_or(0) as u32;
            let sc = params["range"]["start"]["character"].as_u64().unwrap_or(0) as u32;
            let el = params["range"]["end"]["line"].as_u64().unwrap_or(0) as u32;
            let ec = params["range"]["end"]["character"].as_u64().unwrap_or(0) as u32;
            let range = LspRange::new(sl, sc, el, ec);
            let edits = fmt_range(doc, &range);
            let json: Vec<String> = edits.iter().map(|e| e.to_json()).collect();
            format!("[{}]", json.join(","))
        } else {
            "[]".to_string()
        };

        format!(r#"{{"jsonrpc":"2.0","id":{id_val},"result":{result}}}"#)
    }

    fn handle_semantic_tokens(&mut self, id: serde_json::Value, params: serde_json::Value) -> String {
        let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
        let id_val = id.as_i64().unwrap_or(0);

        let result = if let Some(doc) = self.documents.get(uri) {
            let analysis = self.analyses.get(uri).cloned().unwrap_or_else(|| analyze(&doc.text));
            get_semantic_tokens_json(doc, &analysis)
        } else {
            r#"{"data":[]}"#.to_string()
        };

        format!(r#"{{"jsonrpc":"2.0","id":{id_val},"result":{result}}}"#)
    }

    fn handle_rename(&mut self, id: serde_json::Value, params: serde_json::Value) -> String {
        let uri = params["textDocument"]["uri"].as_str().unwrap_or("").to_string();
        let line = params["position"]["line"].as_u64().unwrap_or(0) as u32;
        let character = params["position"]["character"].as_u64().unwrap_or(0) as u32;
        let new_name = params["newName"].as_str().unwrap_or("_renamed");
        let id_val = id.as_i64().unwrap_or(0);

        let result = if let Some(doc) = self.documents.get(&uri) {
            let all_docs: HashMap<String, Document> = self
                .documents
                .all_uris()
                .into_iter()
                .filter_map(|u| self.documents.get(&u).map(|d| (u, d.clone())))
                .collect();
            rename_symbol(new_name, doc, line, character, &all_docs, &self.analyses)
        } else {
            r#"{"changes":{}}"#.to_string()
        };

        format!(r#"{{"jsonrpc":"2.0","id":{id_val},"result":{result}}}"#)
    }

    fn handle_prepare_rename(&mut self, id: serde_json::Value, params: serde_json::Value) -> String {
        let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
        let line = params["position"]["line"].as_u64().unwrap_or(0) as u32;
        let character = params["position"]["character"].as_u64().unwrap_or(0) as u32;
        let id_val = id.as_i64().unwrap_or(0);

        let result = if let Some(doc) = self.documents.get(uri) {
            let analysis = self.analyses.get(uri).cloned().unwrap_or_else(|| analyze(&doc.text));
            match prepare_rename(doc, line, character, &analysis) {
                Some(r) => format!(
                    r#"{{"start":{{"line":{},"character":{}}},"end":{{"line":{},"character":{}}}}}"#,
                    r.start.line, r.start.character, r.end.line, r.end.character
                ),
                None => "null".to_string(),
            }
        } else {
            "null".to_string()
        };

        format!(r#"{{"jsonrpc":"2.0","id":{id_val},"result":{result}}}"#)
    }

    fn handle_code_action(&mut self, id: serde_json::Value, params: serde_json::Value) -> String {
        let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
        let sl = params["range"]["start"]["line"].as_u64().unwrap_or(0) as u32;
        let sc = params["range"]["start"]["character"].as_u64().unwrap_or(0) as u32;
        let el = params["range"]["end"]["line"].as_u64().unwrap_or(0) as u32;
        let ec = params["range"]["end"]["character"].as_u64().unwrap_or(0) as u32;
        let id_val = id.as_i64().unwrap_or(0);

        let result = if let Some(doc) = self.documents.get(uri) {
            let analysis = self.analyses.get(uri).cloned().unwrap_or_else(|| analyze(&doc.text));
            let range = LspRange::new(sl, sc, el, ec);
            let diag_codes: Vec<String> = if let Some(arr) = params["context"]["diagnostics"].as_array() {
                arr.iter().map(|d| d.to_string()).collect()
            } else {
                vec![]
            };
            let actions = get_code_actions(doc, &range, &diag_codes, &analysis);
            code_actions_to_json(&actions)
        } else {
            "[]".to_string()
        };

        format!(r#"{{"jsonrpc":"2.0","id":{id_val},"result":{result}}}"#)
    }

    fn handle_signature_help(&mut self, id: serde_json::Value, params: serde_json::Value) -> String {
        let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
        let line = params["position"]["line"].as_u64().unwrap_or(0) as u32;
        let character = params["position"]["character"].as_u64().unwrap_or(0) as u32;
        let id_val = id.as_i64().unwrap_or(0);

        let result = if let Some(doc) = self.documents.get(uri) {
            let analysis = self.analyses.get(uri).cloned().unwrap_or_else(|| analyze(&doc.text));
            get_signature_help_json(doc, line, character, &analysis)
                .unwrap_or_else(|| "null".to_string())
        } else {
            "null".to_string()
        };

        format!(r#"{{"jsonrpc":"2.0","id":{id_val},"result":{result}}}"#)
    }

    // ─── Internal helpers ─────────────────────────────────────────────────

    /// Analyze a document and publish diagnostics notification.
    fn analyze_and_publish(&mut self, uri: &str) {
        let text = match self.documents.get(uri) {
            Some(doc) => doc.text.clone(),
            None => return,
        };

        let result = analyze(&text);

        // Update symbol index
        if let Some(doc) = self.documents.get(uri) {
            self.symbol_index.update_file(uri, &result);
            let _ = doc;
        }

        // Build diagnostics JSON
        let diag_json: Vec<String> = result.diagnostics.iter().map(|d| d.to_json()).collect();
        let notif = make_notification(
            "textDocument/publishDiagnostics",
            &format!(
                r#"{{"uri":"{}","diagnostics":[{}]}}"#,
                escape_json_str(uri),
                diag_json.join(",")
            ),
        );
        self.pending_notifications.push(notif);
        self.analyses.insert(uri.to_string(), result);
    }

    // ─── Backward-compatible public API (used by existing tests) ──────────

    pub fn open_document(&mut self, uri: String, text: String) -> Vec<LspDiagnostic> {
        self.documents.open(uri.clone(), text.clone(), 0);
        let result = analyze(&text);
        self.symbol_index.update_file(&uri, &result);
        let diags = result.diagnostics.clone();
        self.analyses.insert(uri, result);
        diags
    }

    pub fn update_document(&mut self, uri: &str, text: String) -> Vec<LspDiagnostic> {
        self.documents.update(uri, text.clone(), 0);
        let result = analyze(&text);
        self.symbol_index.update_file(uri, &result);
        let diags = result.diagnostics.clone();
        self.analyses.insert(uri.to_string(), result);
        diags
    }

    pub fn compute_diagnostics(&self, text: &str) -> Vec<LspDiagnostic> {
        analyze(text).diagnostics
    }

    pub fn get_completions(&self, uri: &str, pos: &LspPosition) -> Vec<LspCompletionItem> {
        let doc = match self.documents.get(uri) {
            Some(d) => d,
            None => {
                return default_keyword_completions();
            }
        };
        let analysis = self.analyses.get(uri).cloned().unwrap_or_else(|| analyze(&doc.text));
        let items = get_completions(doc, pos.line, pos.character, &analysis);
        items
            .into_iter()
            .map(|i| LspCompletionItem {
                label: i.label,
                kind: i.kind,
                detail: i.detail,
                documentation: i.documentation,
            })
            .collect()
    }

    pub fn get_hover(&self, uri: &str, pos: &LspPosition) -> Option<LspHover> {
        let doc = self.documents.get(uri)?;
        let analysis = self.analyses.get(uri).cloned().unwrap_or_else(|| analyze(&doc.text));
        let h = get_hover(doc, pos.line, pos.character, &analysis)?;
        Some(LspHover {
            contents: h.markdown,
            range: Some(h.range),
        })
    }

    pub fn format_document(&self, uri: &str) -> Option<Vec<LspTextEdit>> {
        let doc = self.documents.get(uri)?;
        let edits = fmt_doc(doc);
        if edits.is_empty() {
            // Return identity edit so callers know formatting ran
            let total_lines = doc.lines.len() as u32;
            let last_col = doc.lines.last().map(|l| l.len() as u32).unwrap_or(0);
            return Some(vec![LspTextEdit {
                range: LspRange::new(0, 0, total_lines, last_col),
                new_text: doc.text.clone(),
            }]);
        }
        Some(
            edits
                .into_iter()
                .map(|e| LspTextEdit {
                    range: e.range,
                    new_text: e.new_text,
                })
                .collect(),
        )
    }
}

// ─── Default keyword completions (no document) ────────────────────────────────

fn default_keyword_completions() -> Vec<LspCompletionItem> {
    let keywords = [
        ("def", "Define a function or method", 14u32),
        ("class", "Define a class", 14),
        ("interface", "Define an interface", 14),
        ("return", "Return a value", 14),
        ("if", "Conditional branch", 14),
        ("elif", "Else-if branch", 14),
        ("else", "Else branch", 14),
        ("while", "While loop", 14),
        ("for", "For loop", 14),
        ("async", "Async function", 14),
        ("await", "Await async result", 14),
        ("pub", "Public visibility", 14),
        ("private", "Private visibility", 14),
        ("static", "Static member", 14),
        ("let", "Immutable binding", 14),
        ("mut", "Mutable binding", 14),
    ];
    let builtins = [
        ("print", "print(value: Any) -> Unit", 3u32),
        ("len", "len(collection: Any) -> Int", 3),
        ("range", "range(n: Int) -> List<Int>", 3),
        ("str", "str(val: Any) -> String", 3),
        ("int", "int(val: Any) -> Int", 3),
        ("float", "float(val: Any) -> Float", 3),
        ("assert", "assert(condition: Bool) -> Unit", 3),
    ];
    let mut items: Vec<LspCompletionItem> = keywords
        .iter()
        .map(|(label, doc, kind)| LspCompletionItem {
            label: label.to_string(),
            kind: *kind,
            detail: Some("keyword".to_string()),
            documentation: Some(doc.to_string()),
        })
        .collect();
    for (label, doc, kind) in &builtins {
        items.push(LspCompletionItem {
            label: label.to_string(),
            kind: *kind,
            detail: Some(doc.to_string()),
            documentation: None,
        });
    }
    items
}

fn collect_mantra_files(dir: &Path, out: &mut Vec<PathBuf>, depth: usize) {
    if depth > 12 {
        return;
    }
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with('.')
                || name_str == "node_modules"
                || name_str == "target"
                || name_str == "build"
                || name_str == "dist"
                || name_str == "installer"
                || name_str == ".gradle"
                || name_str == ".idea"
            {
                continue;
            }
            collect_mantra_files(&path, out, depth + 1);
        } else if path.extension().and_then(|s| s.to_str()) == Some("mantra") {
            out.push(path);
        }
    }
}
