//! mantra_intelligence — Mantra Coding Intelligence (MVP 2)
//!
//! Provides native, compiler-integrated AI coding assistance for the Mantra Programming Language:
//! - Code Intelligence Engine (AST & symbol context extraction)
//! - Context-Aware Code Completion (Type-directed & ghost text)
//! - Natural-Language Code Generation (Scaffolding classes, methods, models)
//! - Compiler & Runtime Error Intelligence (Plain-English diagnostic explanation)
//! - AI Debugger & Quick Fixes (Concrete AST/text patches with diffs)
//! - AI Refactoring Engine (Extract method, async conversion, rename)
//! - Documentation Generator (Docstrings & Markdown export)
//! - Unit & Integration Test Generator (Automated assert test suites)
//! - Static Code Review & Security Auditor (Leak detection, performance tips)
//! - Project Understanding (Multi-crate dependency and manifest indexing)

pub mod audit;
pub mod codegen;
pub mod completion;
pub mod context;
pub mod doc_gen;
pub mod error_intelligence;
pub mod project_graph;
pub mod quick_fix;
pub mod refactor;
pub mod test_gen;

pub use audit::{AuditIssue, CodeAuditor};
pub use codegen::CodeGenerator;
pub use completion::{AiCompletionEngine, AiCompletionSuggestion};
pub use context::{CompilerIntelligenceContext, DiagnosticInfo, ScopeContext, SymbolSummary};
pub use doc_gen::DocGenerator;
pub use error_intelligence::{ErrorExplanation, ErrorIntelligenceEngine};
pub use project_graph::{ProjectGraph, ProjectModule};
pub use quick_fix::{CodePatch, QuickFixEngine};
pub use refactor::RefactoringEngine;
pub use test_gen::TestGenerator;

/// Embedded Mantra source for package `mantra.intelligence` / `mantra_intelligence`.
pub const MANTRA_INTELLIGENCE_SOURCE: &str = include_str!("../lib/mantra_intelligence.mantra");

/// High-level facade orchestrating all Mantra Intelligence capabilities.
pub struct MantraIntelligence;

impl MantraIntelligence {
    /// Explains a compiler diagnostic using semantic analysis.
    pub fn explain_error(
        code: &str,
        message: &str,
        source_line: &str,
        context: Option<&CompilerIntelligenceContext>,
    ) -> ErrorExplanation {
        ErrorIntelligenceEngine::explain(code, message, source_line, context)
    }

    /// Suggests concrete quick-fix code patches for a compile error.
    pub fn suggest_fixes(
        file_path: &str,
        source: &str,
        code: &str,
        message: &str,
        line: usize,
        col: usize,
    ) -> Vec<CodePatch> {
        QuickFixEngine::suggest_fixes(file_path, source, code, message, line, col)
    }

    /// Generates context-aware completions at the given cursor position.
    pub fn complete(
        file_path: &str,
        source: &str,
        line: usize,
        col: usize,
        prefix: &str,
    ) -> Vec<AiCompletionSuggestion> {
        let ctx = CompilerIntelligenceContext::from_source(file_path, source, line, col);
        AiCompletionEngine::complete(&ctx, prefix)
    }

    /// Generates a complete Mantra class from natural language.
    pub fn generate_class(prompt: &str) -> Result<String, String> {
        CodeGenerator::generate_class(prompt)
    }

    /// Generates a unit test suite for a given source file.
    pub fn generate_tests(file_path: &str, source: &str) -> Result<String, String> {
        TestGenerator::generate_tests_for_source(file_path, source)
    }

    /// Generates Markdown API documentation for a given source file.
    pub fn generate_docs(file_path: &str, source: &str) -> Result<String, String> {
        DocGenerator::generate_markdown_doc(file_path, source)
    }

    /// Audits a source file for security and performance issues.
    pub fn audit(file_path: &str, source: &str) -> Vec<AuditIssue> {
        CodeAuditor::audit_source(file_path, source)
    }

    /// Provides capability description for MVP 2 intelligence.
    pub fn describe(feature: &str) -> String {
        format!("Mantra Intelligence (MVP 2): {}", feature)
    }
}
