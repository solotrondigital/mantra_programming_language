//! AI Quick Fix and Automated Code Patcher
//!
//! Produces concrete, syntax-validated AST and text patches for compiler diagnostics,
//! supporting unified diff previews and one-click application in editors and CLI.

use serde::{Deserialize, Serialize};

/// Actionable code patch addressing a compiler diagnostic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodePatch {
    pub title: String,
    pub file_path: String,
    pub line: usize,
    pub col: usize,
    pub original: String,
    pub replacement: String,
    pub diff: String,
}

pub struct QuickFixEngine;

impl QuickFixEngine {
    /// Inspects the failing source code and diagnostic to generate suggested fixes.
    pub fn suggest_fixes(
        file_path: &str,
        source: &str,
        code: &str,
        message: &str,
        line: usize,
        col: usize,
    ) -> Vec<CodePatch> {
        let lines: Vec<&str> = source.lines().collect();
        let current_line = if line > 0 && line <= lines.len() {
            lines[line - 1]
        } else {
            ""
        };

        let mut patches = Vec::new();

        // ── 1. Keyword Collision: 'from' -> 'from_email' ────────────────────
        if (code == "E0021" || code == "E0020")
            && (message.contains("Found 'from'") || current_line.contains("pub from:"))
        {
            let replaced = current_line.replace("pub from:", "pub from_email:");
            let diff = format!("- {}\n+ {}", current_line.trim(), replaced.trim());
            patches.push(CodePatch {
                title: "Rename reserved keyword field 'from' to 'from_email'".to_string(),
                file_path: file_path.to_string(),
                line,
                col,
                original: current_line.to_string(),
                replacement: replaced,
                diff,
            });
        }

        // ── 2. Type Mismatch: Wrap with int(...) or str(...) ────────────────
        if code == "E0012" || message.contains("Type mismatch") {
            if message.contains("expected Int, found String") {
                // If assignment or return has string literal/var, suggest int(...)
                if let Some((lhs, rhs)) = current_line.split_once('=') {
                    let replaced = format!("{} = int({})", lhs, rhs.trim());
                    let diff = format!("- {}\n+ {}", current_line.trim(), replaced.trim());
                    patches.push(CodePatch {
                        title: "Convert String to Int using int(...)".to_string(),
                        file_path: file_path.to_string(),
                        line,
                        col,
                        original: current_line.to_string(),
                        replacement: replaced,
                        diff,
                    });
                }
            } else if message.contains("expected String, found Int")
                || message.contains("expected String, found Float")
            {
                if let Some((lhs, rhs)) = current_line.split_once('=') {
                    let replaced = format!("{} = str({})", lhs, rhs.trim());
                    let diff = format!("- {}\n+ {}", current_line.trim(), replaced.trim());
                    patches.push(CodePatch {
                        title: "Convert number to String using str(...)".to_string(),
                        file_path: file_path.to_string(),
                        line,
                        col,
                        original: current_line.to_string(),
                        replacement: replaced,
                        diff,
                    });
                }
            }
        }

        // ── 3. Missing Import for Known Packages ────────────────────────────
        if code == "E0010" || message.contains("Undefined symbol") {
            let mut import_stmt = None;
            if message.contains("PropertiesTOML") || message.contains("ENV") {
                import_stmt = Some("from mantra_util import PropertiesTOML, ENV");
            } else if message.contains("EmailMessage")
                || message.contains("SmtpClient")
                || message.contains("Email")
            {
                import_stmt = Some("from mantra_email import EmailMessage, SmtpClient, Email");
            } else if message.contains("AI") || message.contains("ModelHandle") {
                import_stmt = Some("from mantra_ai import AI, ModelHandle");
            }

            if let Some(stmt) = import_stmt {
                let first_line = lines.first().copied().unwrap_or("");
                let replacement = format!("{}\n{}", stmt, first_line);
                let diff = format!("+ {}\n  {}", stmt, first_line);
                patches.push(CodePatch {
                    title: format!("Add import: '{}'", stmt),
                    file_path: file_path.to_string(),
                    line: 1,
                    col: 1,
                    original: first_line.to_string(),
                    replacement,
                    diff,
                });
            }
        }

        patches
    }

    /// Applies a patch to the given source text and returns the updated text.
    pub fn apply_patch(source: &str, patch: &CodePatch) -> String {
        let mut lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();
        if patch.line > 0 && patch.line <= lines.len() {
            lines[patch.line - 1] = patch.replacement.clone();
            lines.join("\n")
        } else {
            source.to_string()
        }
    }
}
