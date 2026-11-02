//! Static AI Code Review, Security & Performance Auditor
//!
//! Scans source files and ASTs for security flaws (hardcoded credentials, injection),
//! performance bottlenecks (unbounded allocations), and code quality issues.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditIssue {
    pub severity: String, // "error", "warning", "info"
    pub category: String, // "Security", "Performance", "CodeQuality"
    pub message: String,
    pub line: usize,
    pub suggestion: String,
}

pub struct CodeAuditor;

impl CodeAuditor {
    /// Audits a Mantra source file for security, performance, and code quality issues.
    pub fn audit_source(_file_path: &str, source: &str) -> Vec<AuditIssue> {
        let mut issues = Vec::new();

        for (idx, line) in source.lines().enumerate() {
            let line_num = idx + 1;
            let trimmed = line.trim();

            // ── 1. Security: Hardcoded API keys or Secrets ──────────────────
            if (trimmed.contains("sk-proj-") || trimmed.contains("sk-ant-") || trimmed.contains("AIzaSy")) && !trimmed.starts_with('#') {
                issues.push(AuditIssue {
                    severity: "error".to_string(),
                    category: "Security".to_string(),
                    message: "Exposed API credential literal found directly in source code.".to_string(),
                    line: line_num,
                    suggestion: "Store the key in an environment variable and load it using ENV.get(\"KEY\") or mantra.toml.".to_string(),
                });
            }

            if trimmed.contains("password = \"") && !trimmed.contains("\"\"") && !trimmed.starts_with('#') {
                issues.push(AuditIssue {
                    severity: "warning".to_string(),
                    category: "Security".to_string(),
                    message: "Hardcoded password string detected.".to_string(),
                    line: line_num,
                    suggestion: "Use ENV.getOrDefault(\"SMTP_PASSWORD\", \"\") or external secrets manager.".to_string(),
                });
            }

            // ── 2. Performance: Inefficient String Accumulation in Loops ────
            if (trimmed.contains("self.") || trimmed.contains("let ")) && trimmed.contains(" + ") && (trimmed.contains("len(") || trimmed.contains("for ")) {
                issues.push(AuditIssue {
                    severity: "warning".to_string(),
                    category: "Performance".to_string(),
                    message: "Repeated string concatenation inside loop creates multiple heap allocations.".to_string(),
                    line: line_num,
                    suggestion: "Collect elements into a List and join them, or pre-size the buffer.".to_string(),
                });
            }

            // ── 3. Code Quality: Direct panic() calls in library code ───────
            if trimmed.contains("panic(") && !trimmed.starts_with('#') {
                issues.push(AuditIssue {
                    severity: "info".to_string(),
                    category: "CodeQuality".to_string(),
                    message: "Direct 'panic()' call terminates the runtime process ungracefully.".to_string(),
                    line: line_num,
                    suggestion: "Consider returning a Result type or gracefully logging the error.".to_string(),
                });
            }
        }

        issues
    }
}

