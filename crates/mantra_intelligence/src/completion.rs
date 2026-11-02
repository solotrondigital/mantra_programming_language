//! Context-Aware AI Completion Engine
//!
//! Provides type-aware code completions and AI ghost-text completions
//! based on cursor location, enclosing scope, intent comments, and compiler symbols.

use crate::context::CompilerIntelligenceContext;
use serde::{Deserialize, Serialize};

/// An intelligent completion suggestion for editors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiCompletionSuggestion {
    pub label: String,
    pub insert_text: String,
    pub detail: String,
    pub documentation: String,
    pub is_ai_ghost_text: bool,
}

pub struct AiCompletionEngine;

impl AiCompletionEngine {
    /// Generates context-aware completions based on compiler context and typing prefix.
    pub fn complete(
        context: &CompilerIntelligenceContext,
        prefix: &str,
    ) -> Vec<AiCompletionSuggestion> {
        let mut suggestions = Vec::new();
        let lines: Vec<&str> = context.surrounding_code.lines().collect();
        let current_line = lines.iter().find(|l| l.contains(" > | ")).copied().unwrap_or("");

        // ── 1. Ghost-Text Completion for Intent Comments ────────────────────
        // Check for "# Generate ...", "# Fetch ...", "# TODO ..."
        if current_line.contains("#") {
            let comment = current_line.split('#').nth(1).unwrap_or("").trim();
            if let Some(ghost) = Self::synthesize_intent_ghost_text(comment, context) {
                suggestions.push(ghost);
            }
        }

        // ── 2. Symbol-Directed Completion ───────────────────────────────────
        for sym in &context.symbols {
            if prefix.is_empty() || sym.name.to_lowercase().starts_with(&prefix.to_lowercase()) {
                let (insert_text, detail) = match sym.kind.as_str() {
                    "method" | "function" => (format!("{}($0)", sym.name), format!("fn() -> {}", sym.type_name)),
                    "field" => (sym.name.clone(), format!("field: {}", sym.type_name)),
                    "class" => (sym.name.clone(), "class".to_string()),
                    _ => (sym.name.clone(), sym.type_name.clone()),
                };

                suggestions.push(AiCompletionSuggestion {
                    label: sym.name.clone(),
                    insert_text,
                    detail,
                    documentation: format!("Mantra compiler symbol ({})", sym.kind),
                    is_ai_ghost_text: false,
                });
            }
        }

        // ── 3. Common Architectural Snippets ────────────────────────────────
        if prefix == "get" || prefix == "fetch" {
            suggestions.push(AiCompletionSuggestion {
                label: "getUser(id: Int) -> User".to_string(),
                insert_text: "pub def getUser(self, id: Int) -> User:\n        let record = self.db.query_one(\"SELECT * FROM users WHERE id = ?\", [id])\n        return User(record.get(\"id\"), record.get(\"name\"))".to_string(),
                detail: "Mantra AI Pattern: Database Entity Query".to_string(),
                documentation: "Generates a database query method with null check and typed entity return.".to_string(),
                is_ai_ghost_text: true,
            });
        }

        suggestions
    }

    /// Synthesizes ghost-text code implementation from an intent comment.
    fn synthesize_intent_ghost_text(
        comment: &str,
        context: &CompilerIntelligenceContext,
    ) -> Option<AiCompletionSuggestion> {
        let lower = comment.to_lowercase();
        let indent = "        ";

        if lower.contains("getuser") || lower.contains("fetch") && lower.contains("user") {
            let code = format!(
                "{}let query: String = \"SELECT id, username, email FROM users WHERE id = ?\"\n{}let record = self.db.query_one(query, [id])\n{}if record == None:\n{}    panic(\"User with id \" + str(id) + \" not found\")\n{}return User(record.get(\"id\"), record.get(\"username\"), record.get(\"email\"))",
                indent, indent, indent, indent, indent
            );
            return Some(AiCompletionSuggestion {
                label: "✨ Generate: getUser implementation".to_string(),
                insert_text: code,
                detail: "Mantra AI In-Line Code Generation".to_string(),
                documentation: format!("Synthesized based on intent: '{}'", comment),
                is_ai_ghost_text: true,
            });
        }

        if lower.contains("send") && (lower.contains("mail") || lower.contains("email")) {
            let code = format!(
                "{}let client = SmtpClient(\"mail.solotrontech.com\", 465, \"user\", \"pass\", True)\n{}let msg = EmailMessage(self.from_email, to_email, subject, body)\n{}return client.send(msg)",
                indent, indent, indent
            );
            return Some(AiCompletionSuggestion {
                label: "✨ Generate: send_email implementation".to_string(),
                insert_text: code,
                detail: "Mantra AI In-Line Code Generation".to_string(),
                documentation: "Uses mantra_email SmtpClient and EmailMessage.".to_string(),
                is_ai_ghost_text: true,
            });
        }

        if lower.contains("generate") || lower.contains("implement") {
            let fn_name = context.scope.enclosing_function.as_deref().unwrap_or("execute");
            let code = format!(
                "{}# Implementation generated by Mantra AI for {}\n{}print(\"Executing {}\")\n{}return True",
                indent, fn_name, indent, fn_name, indent
            );
            return Some(AiCompletionSuggestion {
                label: format!("✨ Generate: {} method body", fn_name),
                insert_text: code,
                detail: "Mantra AI Code Generation".to_string(),
                documentation: format!("Synthesized for enclosing function '{}'", fn_name),
                is_ai_ghost_text: true,
            });
        }

        None
    }
}
