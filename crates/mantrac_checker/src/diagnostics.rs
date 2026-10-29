use mantrac_lexer::span::Span;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub code: &'static str,
    pub message: String,
    pub span: Span,
    pub note: Option<String>,
}

impl Diagnostic {
    pub fn error(code: &'static str, message: impl Into<String>, span: Span) -> Self {
        Diagnostic {
            level: DiagnosticLevel::Error,
            code,
            message: message.into(),
            span,
            note: None,
        }
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    pub fn format_with_source(&self, filename: &str, source: &str) -> String {
        let lines: Vec<&str> = source.lines().collect();
        let line_num = self.span.line;
        let col_num = self.span.col;

        let level_str = match self.level {
            DiagnosticLevel::Error => "error",
            DiagnosticLevel::Warning => "warning",
            DiagnosticLevel::Note => "note",
        };

        let mut output = format!(
            "{}[{}]: {} at {}:{}:{}\n",
            level_str, self.code, self.message, filename, line_num, col_num
        );

        if line_num > 0 && line_num <= lines.len() {
            let src_line = lines[line_num - 1];
            output.push_str(&format!("{:4} | {}\n", line_num, src_line));
            let padding = " ".repeat(col_num.saturating_sub(1));
            output.push_str(&format!("     | {}^\n", padding));
        }

        if let Some(ref note) = self.note {
            output.push_str(&format!("  = note: {}\n", note));
        }

        output
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error[{}]: {} at {}:{}",
            self.code, self.message, self.span.line, self.span.col
        )
    }
}

