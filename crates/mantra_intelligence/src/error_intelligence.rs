//! Compiler and Runtime Error Intelligence Engine
//!
//! Translates cryptic compiler diagnostic codes (E0001–E0100) and runtime panics
//! into plain-English root causes, developer remedies, and concrete code fixes.

use crate::context::CompilerIntelligenceContext;
use serde::{Deserialize, Serialize};

/// Detailed explanation of a compiler or runtime error.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorExplanation {
    pub code: String,
    pub title: String,
    pub summary: String,
    pub root_cause: String,
    pub suggested_remedy: String,
    pub suggested_patch: Option<String>,
    pub diff_preview: Option<String>,
}

pub struct ErrorIntelligenceEngine;

impl ErrorIntelligenceEngine {
    /// Generates an intelligent, actionable explanation for a given compiler diagnostic.
    pub fn explain(
        code: &str,
        message: &str,
        source_line: &str,
        context: Option<&CompilerIntelligenceContext>,
    ) -> ErrorExplanation {
        // 1. Try rule-based deterministic compiler intelligence
        if let Some(explanation) = Self::rule_based_diagnosis(code, message, source_line, context) {
            return explanation;
        }

        // 2. Generic fallback diagnosis
        ErrorExplanation {
            code: code.to_string(),
            title: format!("Mantra Compiler Diagnostic [{}]", code),
            summary: message.to_string(),
            root_cause: format!("The compiler detected an issue at line: '{}'", source_line.trim()),
            suggested_remedy: "Review the surrounding syntax and type annotations according to the Mantra language guide.".to_string(),
            suggested_patch: None,
            diff_preview: None,
        }
    }

    /// Rule-based deterministic diagnosis ensuring offline, zero-latency explanations.
    fn rule_based_diagnosis(
        code: &str,
        message: &str,
        source_line: &str,
        _context: Option<&CompilerIntelligenceContext>,
    ) -> Option<ErrorExplanation> {
        let trimmed_line = source_line.trim();

        // ── E0021: Expected field or method declaration. Found keyword (e.g. 'from') ──
        if (code == "E0021" || code == "E0020") && message.contains("Found 'from'") {
            let fixed = trimmed_line.replace("from", "from_email");
            return Some(ErrorExplanation {
                code: code.to_string(),
                title: "Reserved Keyword Collision ('from')".to_string(),
                summary: "In Mantra, 'from' is a reserved language keyword reserved for module imports ('from x import y').".to_string(),
                root_cause: "Class fields, method parameters, and local variables cannot use reserved keywords as their identifier name.".to_string(),
                suggested_remedy: "Rename the identifier to 'from_email', 'from_addr', or 'sender'.".to_string(),
                suggested_patch: Some(fixed.clone()),
                diff_preview: Some(format!("- {}\n+ {}", trimmed_line, fixed)),
            });
        }

        // ── E0012: Type mismatch (e.g. expected Int, found String) ──
        if code == "E0012" || message.contains("Type mismatch") || message.contains("Cannot assign") {
            let (expected, found) = Self::extract_mismatched_types(message);
            let patch = if expected == "Int" && found == "String" {
                Some("Wrap the expression with int(...) to parse the string to integer.".to_string())
            } else if expected == "String" && (found == "Int" || found == "Float") {
                Some("Wrap the expression with str(...) to convert the number to string.".to_string())
            } else {
                None
            };

            return Some(ErrorExplanation {
                code: "E0012".to_string(),
                title: format!("Type Mismatch: Expected {}, Found {}", expected, found),
                summary: format!("The variable or parameter expects type '{}', but received type '{}'.", expected, found),
                root_cause: "Mantra is statically typed and prohibits implicit type coercions to maintain memory safety and predictability.".to_string(),
                suggested_remedy: format!("Explicitly convert the value to '{}' or update the target type annotation.", expected),
                suggested_patch: patch,
                diff_preview: None,
            });
        }

        // ── E0010: Undefined Symbol / Missing Import ──
        if code == "E0010" || message.contains("Undefined symbol") || message.contains("undefined variable") {
            let sym = message
                .split('\'')
                .nth(1)
                .unwrap_or_else(|| message.split_whitespace().last().unwrap_or(""));

            let (package_hint, import_stmt) = match sym {
                "PropertiesTOML" | "ENV" => ("mantra_util", "from mantra_util import PropertiesTOML, ENV"),
                "EmailMessage" | "SmtpClient" | "Email" => ("mantra_email", "from mantra_email import EmailMessage, SmtpClient, Email"),
                "AI" | "ModelHandle" | "Prompt" => ("mantra_ai", "from mantra_ai import AI, ModelHandle, Prompt"),
                _ => ("", ""),
            };

            let remedy = if !package_hint.is_empty() {
                format!("Symbol '{}' is provided by standard package '{}'. Add '{}' at the top of your file.", sym, package_hint, import_stmt)
            } else {
                format!("Check for spelling typos or ensure '{}' is declared and in scope.", sym)
            };

            return Some(ErrorExplanation {
                code: "E0010".to_string(),
                title: format!("Undefined Symbol '{}'", sym),
                summary: format!("The identifier '{}' was referenced before declaration or without an import.", sym),
                root_cause: "The compiler symbol resolution pass could not find this symbol in the local, enclosing, or imported scopes.".to_string(),
                suggested_remedy: remedy,
                suggested_patch: if !import_stmt.is_empty() { Some(import_stmt.to_string()) } else { None },
                diff_preview: None,
            });
        }

        // ── E0100: Cyclic Inheritance ──
        if code == "E0100" || message.contains("Cyclic inheritance") {
            return Some(ErrorExplanation {
                code: "E0100".to_string(),
                title: "Cyclic Inheritance Detected".to_string(),
                summary: "A class inheritance loop was detected (e.g., A extends B, and B extends A).".to_string(),
                root_cause: "Object layout memory size cannot be computed for recursive base class hierarchies.".to_string(),
                suggested_remedy: "Refactor inheritance to use composition or implement interfaces instead of recursive sub-classing.".to_string(),
                suggested_patch: None,
                diff_preview: None,
            });
        }

        // ── E0030: Missing return statement ──
        if code == "E0030" || message.contains("Missing return") {
            return Some(ErrorExplanation {
                code: "E0030".to_string(),
                title: "Non-Void Function Missing Return".to_string(),
                summary: "The function declares a return type but does not return a value along all execution paths.".to_string(),
                root_cause: "Static analysis requires every branch (including if/else) to yield a value of the declared return type.".to_string(),
                suggested_remedy: "Ensure all branches end with a 'return <expr>' or change the return type to 'Void'.".to_string(),
                suggested_patch: Some("return 0".to_string()),
                diff_preview: None,
            });
        }

        None
    }

    fn extract_mismatched_types(msg: &str) -> (String, String) {
        // e.g. "expected Int, found String"
        let mut expected = "ExpectedType".to_string();
        let mut found = "ActualType".to_string();

        if let Some(exp_pos) = msg.find("expected ") {
            let rest = &msg[exp_pos + 9..];
            if let Some(comma_pos) = rest.find(',') {
                expected = rest[..comma_pos].trim().to_string();
            }
        }
        if let Some(found_pos) = msg.find("found ") {
            let rest = &msg[found_pos + 6..];
            found = rest.split_whitespace().next().unwrap_or("ActualType").trim_end_matches('.').to_string();
        }

        (expected, found)
    }
}

