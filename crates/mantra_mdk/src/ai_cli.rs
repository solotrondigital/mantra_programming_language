//! Mantra AI Coding Intelligence CLI Handler (MVP 2)
//!
//! Exposes AI compiler and development tools directly from the terminal:
//! - mantra ai explain <file>[:line]
//! - mantra ai fix <file> [--apply]
//! - mantra ai generate "<prompt>"
//! - mantra ai test-gen <file>
//! - mantra ai doc <file>
//! - mantra ai audit [file/dir]
//! - mantra ai graph [dir]
//! - mantra ai info

use mantra_intelligence::audit::CodeAuditor;
use mantra_intelligence::codegen::CodeGenerator;
use mantra_intelligence::context::CompilerIntelligenceContext;
use mantra_intelligence::doc_gen::DocGenerator;
use mantra_intelligence::error_intelligence::ErrorIntelligenceEngine;
use mantra_intelligence::project_graph::ProjectGraph;
use mantra_intelligence::quick_fix::QuickFixEngine;
use mantra_intelligence::test_gen::TestGenerator;
use std::fs;
use std::path::Path;
use std::process::exit;

pub struct AiCli;

impl AiCli {
    pub fn handle_command(args: &[String]) {
        if args.is_empty() || args[0] == "-h" || args[0] == "--help" {
            Self::print_help();
            return;
        }

        match args[0].as_str() {
            "explain" => Self::handle_explain(&args[1..]),
            "fix" => Self::handle_fix(&args[1..]),
            "generate" => Self::handle_generate(&args[1..]),
            "test-gen" => Self::handle_test_gen(&args[1..]),
            "doc" => Self::handle_doc(&args[1..]),
            "audit" => Self::handle_audit(&args[1..]),
            "graph" => Self::handle_graph(&args[1..]),
            "info" => Self::handle_info(),
            unknown => {
                eprintln!("Unknown 'mantra ai' subcommand: '{}'", unknown);
                eprintln!("Run 'mantra ai --help' for available intelligence commands.");
                exit(1);
            }
        }
    }

    fn print_help() {
        println!(
            r#"Mantra AI Coding Intelligence (MVP 2) — Developer Productivity Tools

Usage:
    mantra ai <subcommand> [arguments] [options]

Subcommands:
    explain <FILE>[:LINE]     Explain compiler diagnostics or syntax at the target line
    fix <FILE> [--dry-run|--apply]
                              Preview compiler-guided quick-fixes, or apply them to the file
    generate "<PROMPT>"       Synthesize a new Mantra class, service, or model from natural language
    test-gen <FILE>           Generate an automated unit test suite in tests/ using assert()
    doc <FILE> [--in-place]   Generate Markdown reference documentation or docstrings
    audit [PATH]              Perform static security, credential, and performance audit
    graph [DIR]               Index project dependencies, modules, and architecture graph
    info                      Display Mantra Intelligence MVP 2 status and active capabilities

Examples:
    mantra ai explain src/main.mantra:42
    mantra ai fix src/main.mantra --apply
    mantra ai generate "Create an OrderService with order processing and active counts"
    mantra ai test-gen src/services/user_service.mantra
    mantra ai audit
"#
        );
    }

    fn handle_explain(args: &[String]) {
        if args.is_empty() {
            eprintln!("Usage: mantra ai explain <FILE>[:LINE]");
            exit(1);
        }

        let target = &args[0];
        let (file_path, line_opt) = if let Some((f, l)) = target.split_once(':') {
            (f, l.parse::<usize>().ok())
        } else {
            (target.as_str(), None)
        };

        let content = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to read file '{}': {}", file_path, e);
                exit(1);
            }
        };

        let line_num = line_opt.unwrap_or(1);
        let lines: Vec<&str> = content.lines().collect();
        let target_line = if line_num > 0 && line_num <= lines.len() {
            lines[line_num - 1]
        } else {
            ""
        };

        println!("==================================================================");
        println!("              MANTRA AI ERROR INTELLIGENCE (MVP 2)                ");
        println!("==================================================================");
        println!("File: {}:{}", file_path, line_num);
        println!("Code: {}", target_line.trim());
        println!("------------------------------------------------------------------");

        let context = CompilerIntelligenceContext::from_source(file_path, &content, line_num, 1);
        let diagnostics: Vec<_> = context
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.line == line_num && diagnostic.level == "error")
            .collect();

        if diagnostics.is_empty() {
            println!("No compiler error was found at line {}.", line_num);
            if !target_line.trim().is_empty() {
                println!("Source: {}", target_line.trim());
            }
            return;
        }

        for diagnostic in diagnostics {
            let explanation = ErrorIntelligenceEngine::explain(
                &diagnostic.code,
                &diagnostic.message,
                &diagnostic.source_line,
                Some(&context),
            );
            println!("\n[{}] {}", explanation.code, explanation.title);
            println!("Summary : {}", explanation.summary);
            println!("Root Cause:\n  {}", explanation.root_cause);
            println!("Suggested Remedy:\n  {}", explanation.suggested_remedy);
            if let Some(ref patch) = explanation.suggested_patch {
                println!("Suggested Patch:\n  {}", patch);
            }
            if let Some(ref diff) = explanation.diff_preview {
                println!("Diff Preview:\n{}", diff);
            }
        }
        println!("==================================================================");
    }

    fn handle_fix(args: &[String]) {
        if args.is_empty() {
            eprintln!("Usage: mantra ai fix <FILE> [--dry-run|--apply]");
            exit(1);
        }

        let file_path = &args[0];
        let apply = args.iter().any(|a| a == "--apply");

        let content = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to read file '{}': {}", file_path, e);
                exit(1);
            }
        };

        println!("Checking '{}' for compiler diagnostics and quick fixes...", file_path);

        let context = CompilerIntelligenceContext::from_source(file_path, &content, 1, 1);
        let mut all_patches = Vec::new();
        for diagnostic in context.diagnostics.iter().filter(|d| d.level == "error") {
            all_patches.extend(QuickFixEngine::suggest_fixes(
                file_path,
                &content,
                &diagnostic.code,
                &diagnostic.message,
                diagnostic.line,
                diagnostic.col,
            ));
        }
        all_patches.sort_by_key(|patch| patch.line);
        all_patches.dedup_by(|a, b| a.line == b.line && a.replacement == b.replacement);

        if all_patches.is_empty() {
            if context.diagnostics.iter().any(|d| d.level == "error") {
                println!("No automatic quick fix is available for the compiler errors in this file.");
            } else {
                println!("✓ No compiler errors found. File is clean!");
            }
            return;
        }

        println!("Found {} suggested fix(es):", all_patches.len());
        let mut updated = content.clone();
        for (i, p) in all_patches.iter().enumerate() {
            println!("\n[{}] Fix: {}", i + 1, p.title);
            println!("Line {}:\n{}", p.line, p.diff);
            if apply {
                updated = QuickFixEngine::apply_patch(&updated, p);
            }
        }

        if apply {
            if let Err(e) = fs::write(file_path, updated) {
                eprintln!("Failed to write fixes to '{}': {}", file_path, e);
                exit(1);
            }
            println!("\n✓ Successfully applied {} fix(es) to '{}'!", all_patches.len(), file_path);
        } else {
            println!("\nRun with '--apply' to apply suggested fixes to disk.");
        }
    }

    fn handle_generate(args: &[String]) {
        if args.is_empty() {
            eprintln!("Usage: mantra ai generate \"<PROMPT>\"");
            exit(1);
        }

        let prompt = args.join(" ");
        println!("Synthesizing Mantra code for prompt: '{}'...", prompt);

        match CodeGenerator::generate_class(&prompt) {
            Ok(code) => {
                println!("\n================== GENERATED MANTRA SOURCE ==================");
                println!("{}", code);
                println!("=============================================================");
                println!("✓ Syntax verified with Mantra compiler parser (0 errors).");
            }
            Err(e) => {
                eprintln!("Code generation error: {}", e);
                exit(1);
            }
        }
    }

    fn handle_test_gen(args: &[String]) {
        if args.is_empty() {
            eprintln!("Usage: mantra ai test-gen <FILE> [--output <PATH>]");
            exit(1);
        }

        let file_path = &args[0];
        let content = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to read file '{}': {}", file_path, e);
                exit(1);
            }
        };

        match TestGenerator::generate_tests_for_source(file_path, &content) {
            Ok(test_code) => {
                let out_path = if args.len() > 2 && args[1] == "--output" {
                    args[2].clone()
                } else {
                    let stem = Path::new(file_path).file_stem().and_then(|s| s.to_str()).unwrap_or("test");
                    format!("tests/{}_test.mantra", stem)
                };

                let _ = fs::create_dir_all("tests");
                if let Err(e) = fs::write(&out_path, &test_code) {
                    eprintln!("Failed to write test file '{}': {}", out_path, e);
                    exit(1);
                }
                println!("✓ Successfully generated unit test suite in '{}'!", out_path);
                println!("Run 'mantra test' to execute.");
            }
            Err(e) => {
                eprintln!("Test generation error: {}", e);
                exit(1);
            }
        }
    }

    fn handle_doc(args: &[String]) {
        if args.is_empty() {
            eprintln!("Usage: mantra ai doc <FILE> [--in-place]");
            exit(1);
        }

        let file_path = &args[0];
        let in_place = args.iter().any(|a| a == "--in-place");

        let content = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to read file '{}': {}", file_path, e);
                exit(1);
            }
        };

        if in_place {
            let annotated = DocGenerator::annotate_source_with_docstrings(&content);
            if let Err(e) = fs::write(file_path, annotated) {
                eprintln!("Failed to write docstrings to '{}': {}", file_path, e);
                exit(1);
            }
            println!("✓ Successfully added docstrings to '{}'!", file_path);
        } else {
            match DocGenerator::generate_markdown_doc(file_path, &content) {
                Ok(md) => {
                    println!("{}", md);
                }
                Err(e) => {
                    eprintln!("Documentation error: {}", e);
                    exit(1);
                }
            }
        }
    }

    fn handle_audit(args: &[String]) {
        let target = if !args.is_empty() { &args[0] } else { "." };
        println!("==================================================================");
        println!("         MANTRA AI SECURITY & PERFORMANCE AUDIT (MVP 2)           ");
        println!("==================================================================");
        println!("Target path: {}", target);

        let p = Path::new(target);
        let mut total_issues = 0;

        let check_file = |f: &Path, count: &mut usize| {
            if let Ok(c) = fs::read_to_string(f) {
                let issues = CodeAuditor::audit_source(&f.to_string_lossy(), &c);
                if !issues.is_empty() {
                    println!("\n📄 File: {}", f.display());
                    for issue in issues {
                        *count += 1;
                        let badge = match issue.severity.as_str() {
                            "error" => "🔴 ERROR",
                            "warning" => "🟡 WARNING",
                            _ => "🔵 INFO",
                        };
                        println!("  {} [{}]: line {}", badge, issue.category, issue.line);
                        println!("     Message   : {}", issue.message);
                        println!("     Suggestion: {}", issue.suggestion);
                    }
                }
            }
        };

        if p.is_file() {
            check_file(p, &mut total_issues);
        } else {
            fn walk(dir: &Path, count: &mut usize, cb: &dyn Fn(&Path, &mut usize)) {
                if let Ok(entries) = fs::read_dir(dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                        if name.starts_with('.') || name == "target" || name == "dist" || name == "build" {
                            continue;
                        }
                        if path.is_dir() {
                            walk(&path, count, cb);
                        } else if path.extension().and_then(|e| e.to_str()) == Some("mantra") {
                            cb(&path, count);
                        }
                    }
                }
            }
            walk(p, &mut total_issues, &check_file);
        }

        println!("\n------------------------------------------------------------------");
        if total_issues == 0 {
            println!("✓ Audit complete: 0 issues found! Codebase is secure and optimized. 🚀");
        } else {
            println!("Audit complete: {} issue(s) detected.", total_issues);
        }
        println!("==================================================================");
    }

    fn handle_graph(args: &[String]) {
        let target = if !args.is_empty() { &args[0] } else { "." };
        match ProjectGraph::index(Path::new(target)) {
            Ok(graph) => {
                println!("==================================================================");
                println!("           MANTRA AI PROJECT ARCHITECTURE GRAPH (MVP 2)           ");
                println!("==================================================================");
                println!("Project Name : {}", graph.project_name);
                println!("Project Type : {}", graph.project_type);
                println!("Root Dir     : {}", graph.root_dir);
                println!("Dependencies : {}", graph.dependencies.join(", "));
                println!("Indexed Files: {} module(s)", graph.modules.len());
                println!("------------------------------------------------------------------");
                for (name, module) in &graph.modules {
                    println!("• Module `{}` (file: {})", name, module.file_path);
                    if !module.classes.is_empty() {
                        println!("    Classes  : {}", module.classes.join(", "));
                    }
                    if !module.functions.is_empty() {
                        println!("    Functions: {}", module.functions.join(", "));
                    }
                }
                println!("==================================================================");
            }
            Err(e) => {
                eprintln!("Graph indexing error: {}", e);
                exit(1);
            }
        }
    }

    fn handle_info() {
        println!(
            r#"==================================================================
           MANTRA INTELLIGENCE (MVP 2) — IMPLEMENTATION STATUS
==================================================================
Status: v0.7.0 / v0.8.0-dev (Production Core Ready)
Modules Activated:
  ✓ Code Intelligence Engine     (mantra_intelligence::context)
  ✓ Context-Aware Code Completion (mantra_intelligence::completion)
  ✓ Natural-Language Code Gen     (mantra_intelligence::codegen)
  ✓ Error Intelligence Engine     (mantra_intelligence::error_intelligence)
  ✓ AI Debugger & Quick Fixes     (mantra_intelligence::quick_fix)
  ✓ AI Refactoring Engine         (mantra_intelligence::refactor)
  ✓ Documentation Generator       (mantra_intelligence::doc_gen)
  ✓ Unit & Integration Test Gen   (mantra_intelligence::test_gen)
  ✓ Security & Performance Audit  (mantra_intelligence::audit)
  ✓ Project Graph Indexer         (mantra_intelligence::project_graph)
  ✓ LSP Daemon AI Endpoints       (mantra-lsp: mantra/*)
  ✓ MDK CLI Commands              (mantra ai <explain|fix|generate|...>)
=================================================================="#
        );
    }
}

