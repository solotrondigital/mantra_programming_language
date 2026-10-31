// ==============================================================================
// mantra_lsp::symbol_index — LSP Document & Workspace Symbol Index
// Builds a queryable symbol tree from the compiler AST.
// ==============================================================================

use crate::analysis::{span_to_lsp_range, AnalysisResult};
use crate::protocol::{symbol_kind, LspRange};
use mantrac_ast::{ClassMember, Declaration, FunctionDecl, Program, Visibility};
use std::collections::HashMap;

/// A symbol entry for the document/workspace symbol providers.
#[derive(Debug, Clone)]
pub struct IndexedSymbol {
    pub name: String,
    pub kind: u32,
    pub range: LspRange,
    pub selection_range: LspRange,
    pub detail: Option<String>,
    pub children: Vec<IndexedSymbol>,
}

/// Maintains symbol indexes keyed by document URI.
#[derive(Default)]
pub struct SymbolIndex {
    file_symbols: HashMap<String, Vec<IndexedSymbol>>,
}

impl SymbolIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Re-index a file from its AnalysisResult.
    pub fn update_file(&mut self, uri: &str, result: &AnalysisResult) {
        let syms = if let Some(prog) = &result.program {
            index_program(prog)
        } else {
            Vec::new()
        };
        self.file_symbols.insert(uri.to_string(), syms);
    }

    /// Return document symbols as a JSON array string.
    pub fn document_symbols_json(&self, uri: &str) -> String {
        let syms = self.file_symbols.get(uri).cloned().unwrap_or_default();
        symbols_to_json_array(&syms)
    }

    /// Return workspace symbols matching `query` as a JSON array string.
    pub fn workspace_symbols_json(&self, query: &str) -> String {
        let q = query.to_lowercase();
        let mut results = Vec::new();
        for (uri, syms) in &self.file_symbols {
            collect_flat(syms, uri, &q, &mut results);
        }
        results.join(",")
    }
}

// ─── JSON serialisation helpers ───────────────────────────────────────────────

fn range_json(r: &LspRange) -> String {
    format!(
        r#"{{"start":{{"line":{},"character":{}}},"end":{{"line":{},"character":{}}}}}"#,
        r.start.line, r.start.character, r.end.line, r.end.character
    )
}

fn symbol_to_json(s: &IndexedSymbol) -> String {
    let detail = s
        .detail
        .as_deref()
        .map(|d| format!(r#","detail":"{}""#, d.replace('"', "\\\"")))
        .unwrap_or_default();
    let children = if s.children.is_empty() {
        String::new()
    } else {
        format!(r#","children":{}"#, symbols_to_json_array(&s.children))
    };
    format!(
        r#"{{"name":"{}","kind":{}{},"range":{},"selectionRange":{}{}}}"#,
        s.name.replace('"', "\\\""),
        s.kind,
        detail,
        range_json(&s.range),
        range_json(&s.selection_range),
        children,
    )
}

fn symbols_to_json_array(syms: &[IndexedSymbol]) -> String {
    let items: Vec<String> = syms.iter().map(symbol_to_json).collect();
    format!("[{}]", items.join(","))
}

fn collect_flat(
    syms: &[IndexedSymbol],
    uri: &str,
    query: &str,
    out: &mut Vec<String>,
) {
    for s in syms {
        if query.is_empty() || s.name.to_lowercase().contains(query) {
            out.push(format!(
                r#"{{"name":"{}","kind":{},"location":{{"uri":"{}","range":{}}}}}"#,
                s.name.replace('"', "\\\""),
                s.kind,
                uri.replace('"', "\\\""),
                range_json(&s.selection_range)
            ));
        }
        collect_flat(&s.children, uri, query, out);
    }
}

// ─── AST → symbol tree ────────────────────────────────────────────────────────

fn index_program(program: &Program) -> Vec<IndexedSymbol> {
    let mut syms = Vec::new();
    for decl in &program.declarations {
        match decl {
            Declaration::Class(cls) => {
                let range = span_to_lsp_range(cls.span);
                let mut children = Vec::new();
                for member in &cls.members {
                    match member {
                        ClassMember::Field(f) => {
                            let r = span_to_lsp_range(f.span);
                            children.push(IndexedSymbol {
                                name: f.name.clone(),
                                kind: symbol_kind::FIELD,
                                range: r.clone(),
                                selection_range: r,
                                detail: None,
                                children: vec![],
                            });
                        }
                        ClassMember::Method(m) | ClassMember::StaticMethod(m) => {
                            let r = span_to_lsp_range(m.span);
                            let is_init = m.name == "__init__";
                            let kind = if is_init {
                                symbol_kind::CONSTRUCTOR
                            } else {
                                symbol_kind::METHOD
                            };
                            children.push(IndexedSymbol {
                                name: m.name.clone(),
                                kind,
                                range: r.clone(),
                                selection_range: r,
                                detail: Some(function_signature(m)),
                                children: vec![],
                            });
                        }
                        ClassMember::StaticField(f) => {
                            let r = span_to_lsp_range(f.span);
                            children.push(IndexedSymbol {
                                name: f.name.clone(),
                                kind: symbol_kind::FIELD,
                                range: r.clone(),
                                selection_range: r,
                                detail: None,
                                children: vec![],
                            });
                        }
                        ClassMember::Property(p) => {
                            let r = span_to_lsp_range(p.span);
                            children.push(IndexedSymbol {
                                name: p.name.clone(),
                                kind: symbol_kind::PROPERTY,
                                range: r.clone(),
                                selection_range: r,
                                detail: None,
                                children: vec![],
                            });
                        }
                    }
                }
                syms.push(IndexedSymbol {
                    name: cls.name.clone(),
                    kind: symbol_kind::CLASS,
                    range: range.clone(),
                    selection_range: range,
                    detail: cls.base_class.clone(),
                    children,
                });
            }
            Declaration::Interface(iface) => {
                let range = span_to_lsp_range(iface.span);
                let children: Vec<IndexedSymbol> = iface
                    .methods
                    .iter()
                    .map(|m| {
                        let r = span_to_lsp_range(m.span);
                        IndexedSymbol {
                            name: m.name.clone(),
                            kind: symbol_kind::METHOD,
                            range: r.clone(),
                            selection_range: r,
                            detail: None,
                            children: vec![],
                        }
                    })
                    .collect();
                syms.push(IndexedSymbol {
                    name: iface.name.clone(),
                    kind: symbol_kind::INTERFACE,
                    range: range.clone(),
                    selection_range: range,
                    detail: None,
                    children,
                });
            }
            Declaration::Function(f) => {
                let range = span_to_lsp_range(f.span);
                syms.push(IndexedSymbol {
                    name: f.name.clone(),
                    kind: symbol_kind::FUNCTION,
                    range: range.clone(),
                    selection_range: range,
                    detail: Some(function_signature(f)),
                    children: vec![],
                });
            }
            Declaration::Var(v) => {
                let range = span_to_lsp_range(v.span);
                syms.push(IndexedSymbol {
                    name: v.name.clone(),
                    kind: symbol_kind::VARIABLE,
                    range: range.clone(),
                    selection_range: range,
                    detail: v.ty.as_ref().map(|t| t.to_string()),
                    children: vec![],
                });
            }
            Declaration::Module(name, span) => {
                let range = span_to_lsp_range(*span);
                syms.push(IndexedSymbol {
                    name: name.clone(),
                    kind: symbol_kind::MODULE,
                    range: range.clone(),
                    selection_range: range,
                    detail: None,
                    children: vec![],
                });
            }
            _ => {}
        }
    }
    syms
}

fn function_signature(f: &FunctionDecl) -> String {
    let params: Vec<String> = f
        .params
        .iter()
        .map(|p| {
            if let Some(ty) = &p.ty {
                format!("{}: {}", p.name, ty)
            } else {
                p.name.clone()
            }
        })
        .collect();
    let ret = f
        .return_type
        .as_ref()
        .map(|t| format!(" -> {}", t))
        .unwrap_or_default();
    let vis = match f.visibility {
        Visibility::Pub => "pub ",
        _ => "",
    };
    let asynck = if f.is_async { "async " } else { "" };
    format!("{}{}def {}({}){}", vis, asynck, f.name, params.join(", "), ret)
}

