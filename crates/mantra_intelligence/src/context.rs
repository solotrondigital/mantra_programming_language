//! Compiler Intelligence Context Extractor
//!
//! Bridges compiler data structures (AST, scopes, symbols, diagnostics)
//! to compact, structured representations consumable by LLM pipelines.

use mantrac_ast::{ClassMember, Declaration, Program};
use mantrac_checker::TypeChecker;
use mantrac_lexer::Lexer;
use mantrac_parser::Parser;
use serde::{Deserialize, Serialize};

/// Summary of a symbol discovered in the compilation scope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolSummary {
    pub name: String,
    pub kind: String, // "class", "method", "function", "field", "variable"
    pub type_name: String,
    pub visibility: String,
}

/// Scope context surrounding a specific line/column cursor.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScopeContext {
    pub enclosing_class: Option<String>,
    pub enclosing_function: Option<String>,
    pub local_variables: Vec<(String, String)>,
}

/// Diagnostic captured from compiler phases.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticInfo {
    pub code: String,
    pub level: String,
    pub message: String,
    pub line: usize,
    pub col: usize,
    pub source_line: String,
}

/// Complete semantic intelligence context extracted from compiler state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerIntelligenceContext {
    pub file_path: String,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub scope: ScopeContext,
    pub symbols: Vec<SymbolSummary>,
    pub imports: Vec<String>,
    pub diagnostics: Vec<DiagnosticInfo>,
    pub surrounding_code: String,
}

impl CompilerIntelligenceContext {
    /// Extracts intelligence context directly from source text and cursor location.
    pub fn from_source(file_path: &str, source: &str, cursor_line: usize, cursor_col: usize) -> Self {
        let lines: Vec<&str> = source.lines().collect();

        // 1. Surrounding code slice (5 lines before, cursor line, 5 lines after)
        let start_line = cursor_line.saturating_sub(5).max(1);
        let end_line = (cursor_line + 5).min(lines.len());
        let mut snippet_lines = Vec::new();
        for i in start_line..=end_line {
            if i <= lines.len() {
                let marker = if i == cursor_line { ">" } else { " " };
                snippet_lines.push(format!("{:>3} {} | {}", i, marker, lines[i - 1]));
            }
        }
        let surrounding_code = snippet_lines.join("\n");

        // 2. Lex, parse, and analyze AST
        let mut symbols = Vec::new();
        let mut imports = Vec::new();
        let mut scope = ScopeContext::default();
        let mut diagnostics = Vec::new();

        let mut lexer = Lexer::new(source);
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                let src_line = lines.get(e.span.line.saturating_sub(1)).copied().unwrap_or("").to_string();
                diagnostics.push(DiagnosticInfo {
                    code: e.code.to_string(),
                    level: "error".to_string(),
                    message: e.message,
                    line: e.span.line,
                    col: e.span.col,
                    source_line: src_line,
                });
                return Self {
                    file_path: file_path.to_string(),
                    cursor_line,
                    cursor_col,
                    scope,
                    symbols,
                    imports,
                    diagnostics,
                    surrounding_code,
                };
            }
        };

        let mut parser = Parser::new(tokens);
        let program = match parser.parse() {
            Ok(p) => p,
            Err(e) => {
                let src_line = lines.get(e.span.line.saturating_sub(1)).copied().unwrap_or("").to_string();
                diagnostics.push(DiagnosticInfo {
                    code: "E0020".to_string(),
                    level: "error".to_string(),
                    message: e.message,
                    line: e.span.line,
                    col: e.span.col,
                    source_line: src_line,
                });
                return Self {
                    file_path: file_path.to_string(),
                    cursor_line,
                    cursor_col,
                    scope,
                    symbols,
                    imports,
                    diagnostics,
                    surrounding_code,
                };
            }
        };

        // 3. Inspect declarations to build symbols & enclosing scope
        Self::inspect_ast(&program, cursor_line, &mut scope, &mut symbols, &mut imports);

        // 4. Run type checker for semantic diagnostics and symbol table
        let mut checker = TypeChecker::new();
        if let Err(type_diags) = checker.check_program(&program) {
            for td in type_diags {
                let src_line = lines.get(td.span.line.saturating_sub(1)).copied().unwrap_or("").to_string();
                diagnostics.push(DiagnosticInfo {
                    code: td.code.to_string(),
                    level: match td.level {
                        mantrac_checker::diagnostics::DiagnosticLevel::Error => "error".to_string(),
                        mantrac_checker::diagnostics::DiagnosticLevel::Warning => "warning".to_string(),
                        mantrac_checker::diagnostics::DiagnosticLevel::Note => "note".to_string(),
                    },
                    message: td.message,
                    line: td.span.line,
                    col: td.span.col,
                    source_line: src_line,
                });
            }
        }

        Self {
            file_path: file_path.to_string(),
            cursor_line,
            cursor_col,
            scope,
            symbols,
            imports,
            diagnostics,
            surrounding_code,
        }
    }

    fn inspect_ast(
        program: &Program,
        cursor_line: usize,
        scope: &mut ScopeContext,
        symbols: &mut Vec<SymbolSummary>,
        imports: &mut Vec<String>,
    ) {
        for decl in &program.declarations {
            match decl {
                Declaration::Import { module, alias, .. } => {
                    let mod_path = module.join(".");
                    if let Some(al) = alias {
                        imports.push(format!("{} as {}", mod_path, al));
                    } else {
                        imports.push(mod_path);
                    }
                }
                Declaration::Class(cls) => {
                    symbols.push(SymbolSummary {
                        name: cls.name.clone(),
                        kind: "class".to_string(),
                        type_name: cls.name.clone(),
                        visibility: format!("{:?}", cls.visibility),
                    });

                    // Check if cursor is within class span
                    if cursor_line >= cls.span.line {
                        scope.enclosing_class = Some(cls.name.clone());
                    }

                    for m in &cls.members {
                        match m {
                            ClassMember::Field(f) => {
                                symbols.push(SymbolSummary {
                                    name: f.name.clone(),
                                    kind: "field".to_string(),
                                    type_name: f.ty.as_ref().map_or("Any".to_string(), |t| format!("{:?}", t)),
                                    visibility: format!("{:?}", f.visibility),
                                });
                            }
                            ClassMember::Method(method) | ClassMember::StaticMethod(method) => {
                                symbols.push(SymbolSummary {
                                    name: method.name.clone(),
                                    kind: "method".to_string(),
                                    type_name: method.return_type.as_ref().map_or("Void".to_string(), |t| format!("{:?}", t)),
                                    visibility: format!("{:?}", method.visibility),
                                });
                                if cursor_line >= method.span.line {
                                    scope.enclosing_function = Some(method.name.clone());
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Declaration::Function(func) => {
                    symbols.push(SymbolSummary {
                        name: func.name.clone(),
                        kind: "function".to_string(),
                        type_name: func.return_type.as_ref().map_or("Void".to_string(), |t| format!("{:?}", t)),
                        visibility: format!("{:?}", func.visibility),
                    });
                    if cursor_line >= func.span.line {
                        scope.enclosing_function = Some(func.name.clone());
                    }
                }
                _ => {}
            }
        }
    }

    /// Formats a clean LLM prompt context string containing structured compiler knowledge.
    pub fn format_prompt_header(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("Mantra File: {}\n", self.file_path));
        if let Some(ref cls) = self.scope.enclosing_class {
            s.push_str(&format!("Enclosing Class: {}\n", cls));
        }
        if let Some(ref func) = self.scope.enclosing_function {
            s.push_str(&format!("Enclosing Function/Method: {}\n", func));
        }
        if !self.imports.is_empty() {
            s.push_str(&format!("Imported Packages: {}\n", self.imports.join(", ")));
        }
        if !self.diagnostics.is_empty() {
            s.push_str("Active Compiler Diagnostics:\n");
            for d in &self.diagnostics {
                s.push_str(&format!("  - [{}] {} (line {})\n", d.code, d.message, d.line));
            }
        }
        s.push_str("\nSurrounding Source Code:\n");
        s.push_str(&self.surrounding_code);
        s.push_str("\n");
        s
    }

    /// Serializes to formatted JSON.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
}
