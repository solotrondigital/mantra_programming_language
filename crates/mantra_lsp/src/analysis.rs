// ==============================================================================
// mantra_lsp::analysis — Compiler Analysis Bridge
// Runs lexer → parser → type-checker and normalises results for the LSP.
// ==============================================================================

use crate::protocol::{LspDiagnostic, LspRange, severity};
use mantrac_ast::Program;
use mantrac_checker::{GlobalSymbolTable, TypeChecker};
use mantrac_lexer::{span::Span, Lexer};
use mantrac_parser::Parser;

/// Full analysis result for one source document.
#[derive(Clone)]
pub struct AnalysisResult {
    /// Successfully parsed AST (None if parse failed)
    pub program: Option<Program>,
    /// Symbol table built from the AST (None if parse or check didn't succeed)
    pub symbol_table: Option<GlobalSymbolTable>,
    /// All diagnostics (lex + parse + type-check)
    pub diagnostics: Vec<LspDiagnostic>,
    /// Whether there was a hard parse error
    pub has_parse_error: bool,
}

/// Run the full Mantra analysis pipeline on `source` text.
/// Never panics – all errors are captured as diagnostics.
pub fn analyze(source: &str) -> AnalysisResult {
    let mut diagnostics = Vec::new();

    // ── Phase 1: Lex ────────────────────────────────────────────────────────
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            diagnostics.push(LspDiagnostic {
                range: span_to_lsp_range(e.span),
                severity: severity::ERROR,
                code: Some(e.code.to_string()),
                source: "mantra-lexer".to_string(),
                message: e.message,
            });
            return AnalysisResult {
                program: None,
                symbol_table: None,
                diagnostics,
                has_parse_error: true,
            };
        }
    };

    // ── Phase 2: Parse ──────────────────────────────────────────────────────
    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            diagnostics.push(LspDiagnostic {
                range: span_to_lsp_range(e.span),
                severity: severity::ERROR,
                code: Some("E0002".to_string()),
                source: "mantra-parser".to_string(),
                message: e.message,
            });
            return AnalysisResult {
                program: None,
                symbol_table: None,
                diagnostics,
                has_parse_error: true,
            };
        }
    };

    // ── Phase 3: Type-check ─────────────────────────────────────────────────
    let mut checker = TypeChecker::new();
    if let Err(type_diags) = checker.check_program(&program) {
        for td in &type_diags {
            diagnostics.push(LspDiagnostic {
                range: span_to_lsp_range(td.span),
                severity: match td.level {
                    mantrac_checker::diagnostics::DiagnosticLevel::Error => severity::ERROR,
                    mantrac_checker::diagnostics::DiagnosticLevel::Warning => severity::WARNING,
                    mantrac_checker::diagnostics::DiagnosticLevel::Note => severity::INFORMATION,
                },
                code: Some(td.code.to_string()),
                source: "mantra-checker".to_string(),
                message: td.message.clone(),
            });
        }
        // Still return symbol table + program for navigation
        AnalysisResult {
            program: Some(program),
            symbol_table: Some(checker.symbols),
            diagnostics,
            has_parse_error: false,
        }
    } else {
        AnalysisResult {
            program: Some(program),
            symbol_table: Some(checker.symbols),
            diagnostics,
            has_parse_error: false,
        }
    }
}

/// Convert a compiler `Span` (1-based line/col) to an `LspRange` (0-based).
pub fn span_to_lsp_range(span: Span) -> LspRange {
    let line = if span.line > 0 { span.line - 1 } else { 0 } as u32;
    let col = if span.col > 0 { span.col - 1 } else { 0 } as u32;
    // Estimate end column: use byte span length if meaningful, else col+1
    let len = if span.end > span.start { span.end - span.start } else { 1 };
    LspRange::new(line, col, line, col + len as u32)
}
