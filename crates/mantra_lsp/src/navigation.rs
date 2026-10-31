// ==============================================================================
// mantra_lsp::navigation — Go-to-Definition, References, Implementation
// ==============================================================================

use crate::analysis::{span_to_lsp_range, AnalysisResult};
use crate::document::Document;
use crate::protocol::LspRange;
use std::collections::HashMap;

pub struct Location {
    pub uri: String,
    pub range: LspRange,
}

impl Location {
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"uri":"{}","range":{{"start":{{"line":{},"character":{}}},"end":{{"line":{},"character":{}}}}}}}"#,
            self.uri.replace('"', "\\\""),
            self.range.start.line,
            self.range.start.character,
            self.range.end.line,
            self.range.end.character,
        )
    }
}

pub fn locations_to_json(locs: &[Location]) -> String {
    let items: Vec<String> = locs.iter().map(|l| l.to_json()).collect();
    format!("[{}]", items.join(","))
}

/// Go to definition for the symbol under cursor.
pub fn go_to_definition(
    document: &Document,
    line: u32,
    character: u32,
    analysis: &AnalysisResult,
    all_analyses: &HashMap<String, AnalysisResult>,
) -> Option<Location> {
    let word = document.word_at(line, character)?;
    find_symbol_definition(&word, &document.uri, analysis, all_analyses)
}

/// Find all references to the symbol under cursor.
pub fn find_references(
    document: &Document,
    line: u32,
    character: u32,
    all_documents: &HashMap<String, Document>,
    _all_analyses: &HashMap<String, AnalysisResult>,
) -> Vec<Location> {
    let word = match document.word_at(line, character) {
        Some(w) => w,
        None => return Vec::new(),
    };
    find_all_references(&word, all_documents)
}

/// Go to implementation (for interface methods/classes).
pub fn go_to_implementation(
    document: &Document,
    line: u32,
    character: u32,
    analysis: &AnalysisResult,
    all_analyses: &HashMap<String, AnalysisResult>,
) -> Vec<Location> {
    let word = match document.word_at(line, character) {
        Some(w) => w,
        None => return Vec::new(),
    };

    let mut results = Vec::new();

    // Collect from all analyses
    let search_analyses: Vec<(&String, &AnalysisResult)> = all_analyses
        .iter()
        .chain(std::iter::once((&document.uri, analysis)))
        .collect();

    for (uri, ar) in &search_analyses {
        let Some(st) = &ar.symbol_table else { continue };
        // If word is an interface, find all implementing classes
        if st.interfaces.contains_key(&word) {
            for (cls_name, cls) in &st.classes {
                if cls.interfaces.contains(&word) {
                    results.push(Location {
                        uri: uri.to_string(),
                        range: span_to_lsp_range(cls.span),
                    });
                }
                let _ = cls_name;
            }
        }
        // If word is an abstract class, find concrete subclasses
        if let Some(cls) = st.classes.get(&word) {
            if cls.is_abstract {
                for (_, sub) in &st.classes {
                    if sub.base_class.as_deref() == Some(&word) {
                        results.push(Location {
                            uri: uri.to_string(),
                            range: span_to_lsp_range(sub.span),
                        });
                    }
                }
            }
        }
    }

    results
}

// ─── Internal helpers ─────────────────────────────────────────────────────────

fn find_symbol_definition(
    word: &str,
    current_uri: &str,
    analysis: &AnalysisResult,
    all_analyses: &HashMap<String, AnalysisResult>,
) -> Option<Location> {
    // 1. Search current file's symbol table (classes, functions, methods, fields)
    let found = find_in_analysis(word, current_uri, analysis);
    if found.is_some() {
        return found;
    }

    // 2. Search current file's AST for local variables, loop vars, and parameters
    if let Some(prog) = &analysis.program {
        let ast_found = find_in_ast(word, current_uri, prog);
        if ast_found.is_some() {
            return ast_found;
        }
    }

    // 3. Search across all workspace files' symbol tables
    for (uri, ar) in all_analyses {
        if uri == current_uri {
            continue;
        }
        let found = find_in_analysis(word, uri, ar);
        if found.is_some() {
            return found;
        }
    }

    // 4. Search across all workspace files' ASTs
    for (uri, ar) in all_analyses {
        if uri == current_uri {
            continue;
        }
        if let Some(prog) = &ar.program {
            let ast_found = find_in_ast(word, uri, prog);
            if ast_found.is_some() {
                return ast_found;
            }
        }
    }

    // 5. Check if `word` matches an imported module or file stem (e.g. `home_screen` -> home_screen.mantra)
    let target_filename = format!("{}.mantra", word);
    for uri in all_analyses.keys() {
        if uri.ends_with(&format!("/{}", target_filename))
            || uri.ends_with(&format!("\\{}", target_filename))
        {
            return Some(Location {
                uri: uri.clone(),
                range: LspRange::new(0, 0, 0, 0),
            });
        }
    }

    None
}

fn find_in_ast(word: &str, uri: &str, program: &mantrac_ast::Program) -> Option<Location> {
    for decl in &program.declarations {
        match decl {
            mantrac_ast::Declaration::Class(cls) => {
                if cls.name == word {
                    return Some(Location {
                        uri: uri.to_string(),
                        range: span_to_lsp_range(cls.span),
                    });
                }
                for member in &cls.members {
                    match member {
                        mantrac_ast::ClassMember::Method(m) => {
                            if m.name == word {
                                return Some(Location {
                                    uri: uri.to_string(),
                                    range: span_to_lsp_range(m.span),
                                });
                            }
                            for p in &m.params {
                                if p.name == word {
                                    return Some(Location {
                                        uri: uri.to_string(),
                                        range: span_to_lsp_range(p.span),
                                    });
                                }
                            }
                            if let Some(loc) = find_in_stmts(word, uri, &m.body) {
                                return Some(loc);
                            }
                        }
                        mantrac_ast::ClassMember::StaticMethod(m) => {
                            if m.name == word {
                                return Some(Location {
                                    uri: uri.to_string(),
                                    range: span_to_lsp_range(m.span),
                                });
                            }
                            if let Some(loc) = find_in_stmts(word, uri, &m.body) {
                                return Some(loc);
                            }
                        }
                        mantrac_ast::ClassMember::Field(f) => {
                            if f.name == word {
                                return Some(Location {
                                    uri: uri.to_string(),
                                    range: span_to_lsp_range(f.span),
                                });
                            }
                        }
                        mantrac_ast::ClassMember::StaticField(f) => {
                            if f.name == word {
                                return Some(Location {
                                    uri: uri.to_string(),
                                    range: span_to_lsp_range(f.span),
                                });
                            }
                        }
                        mantrac_ast::ClassMember::Property(p) => {
                            if p.name == word {
                                return Some(Location {
                                    uri: uri.to_string(),
                                    range: span_to_lsp_range(p.span),
                                });
                            }
                        }
                    }
                }
            }
            mantrac_ast::Declaration::Function(f) => {
                if f.name == word {
                    return Some(Location {
                        uri: uri.to_string(),
                        range: span_to_lsp_range(f.span),
                    });
                }
                for p in &f.params {
                    if p.name == word {
                        return Some(Location {
                            uri: uri.to_string(),
                            range: span_to_lsp_range(p.span),
                        });
                    }
                }
                if let Some(loc) = find_in_stmts(word, uri, &f.body) {
                    return Some(loc);
                }
            }
            mantrac_ast::Declaration::Var(v) => {
                if v.name == word {
                    return Some(Location {
                        uri: uri.to_string(),
                        range: span_to_lsp_range(v.span),
                    });
                }
            }
            mantrac_ast::Declaration::Stmt(s) => {
                if let Some(loc) = find_in_stmt(word, uri, s) {
                    return Some(loc);
                }
            }
            _ => {}
        }
    }
    None
}

fn find_in_stmts(word: &str, uri: &str, stmts: &[mantrac_ast::Stmt]) -> Option<Location> {
    for s in stmts {
        if let Some(loc) = find_in_stmt(word, uri, s) {
            return Some(loc);
        }
    }
    None
}

fn find_in_stmt(word: &str, uri: &str, stmt: &mantrac_ast::Stmt) -> Option<Location> {
    match stmt {
        mantrac_ast::Stmt::Let { name, span, .. } => {
            if name == word {
                return Some(Location {
                    uri: uri.to_string(),
                    range: span_to_lsp_range(*span),
                });
            }
        }
        mantrac_ast::Stmt::Mut { name, span, .. } => {
            if name == word {
                return Some(Location {
                    uri: uri.to_string(),
                    range: span_to_lsp_range(*span),
                });
            }
        }
        mantrac_ast::Stmt::For { var, span, body, .. } => {
            if var == word {
                return Some(Location {
                    uri: uri.to_string(),
                    range: span_to_lsp_range(*span),
                });
            }
            if let Some(loc) = find_in_stmts(word, uri, body) {
                return Some(loc);
            }
        }
        mantrac_ast::Stmt::While { body, .. } => {
            if let Some(loc) = find_in_stmts(word, uri, body) {
                return Some(loc);
            }
        }
        mantrac_ast::Stmt::If {
            then_branch,
            elif_branches,
            else_branch,
            ..
        } => {
            if let Some(loc) = find_in_stmts(word, uri, then_branch) {
                return Some(loc);
            }
            for (_, elif_body) in elif_branches {
                if let Some(loc) = find_in_stmts(word, uri, elif_body) {
                    return Some(loc);
                }
            }
            if let Some(else_body) = else_branch {
                if let Some(loc) = find_in_stmts(word, uri, else_body) {
                    return Some(loc);
                }
            }
        }
        _ => {}
    }
    None
}

fn find_in_analysis(word: &str, uri: &str, analysis: &AnalysisResult) -> Option<Location> {
    let st = analysis.symbol_table.as_ref()?;

    if let Some(cls) = st.classes.get(word) {
        return Some(Location { uri: uri.to_string(), range: span_to_lsp_range(cls.span) });
    }
    if let Some(iface) = st.interfaces.get(word) {
        return Some(Location { uri: uri.to_string(), range: span_to_lsp_range(iface.span) });
    }
    if let Some(func) = st.functions.get(word) {
        return Some(Location { uri: uri.to_string(), range: span_to_lsp_range(func.span) });
    }

    // Check class methods and fields
    for cls in st.classes.values() {
        if let Some(method) = cls.methods.get(word) {
            return Some(Location { uri: uri.to_string(), range: span_to_lsp_range(method.span) });
        }
        if let Some(method) = cls.static_methods.get(word) {
            return Some(Location { uri: uri.to_string(), range: span_to_lsp_range(method.span) });
        }
        if let Some(field) = cls.fields.get(word) {
            return Some(Location { uri: uri.to_string(), range: span_to_lsp_range(field.span) });
        }
        if let Some(field) = cls.static_fields.get(word) {
            return Some(Location { uri: uri.to_string(), range: span_to_lsp_range(field.span) });
        }
    }

    // Also scan interface methods
    for iface in st.interfaces.values() {
        if let Some(method) = iface.methods.get(word) {
            return Some(Location { uri: uri.to_string(), range: span_to_lsp_range(method.span) });
        }
    }

    None
}

/// Scan all document texts for occurrences of `word` as an identifier.
fn find_all_references(word: &str, all_documents: &HashMap<String, Document>) -> Vec<Location> {
    let mut locs = Vec::new();
    for (uri, doc) in all_documents {
        for (line_idx, line_text) in doc.lines.iter().enumerate() {
            let mut search_from = 0;
            while let Some(pos) = line_text[search_from..].find(word) {
                let abs_pos = search_from + pos;
                // Verify word boundary
                let before_ok = abs_pos == 0 || {
                    let ch = line_text.as_bytes()[abs_pos - 1] as char;
                    !ch.is_alphanumeric() && ch != '_'
                };
                let after_ok = {
                    let end = abs_pos + word.len();
                    end >= line_text.len() || {
                        let ch = line_text.as_bytes()[end] as char;
                        !ch.is_alphanumeric() && ch != '_'
                    }
                };
                if before_ok && after_ok {
                    let range = LspRange::new(
                        line_idx as u32,
                        abs_pos as u32,
                        line_idx as u32,
                        (abs_pos + word.len()) as u32,
                    );
                    locs.push(Location { uri: uri.clone(), range });
                }
                search_from = abs_pos + 1;
            }
        }
    }
    locs
}

