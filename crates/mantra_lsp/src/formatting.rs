// ==============================================================================
// mantra_lsp::formatting — Mantra Code Formatter (mantra fmt)
// Normalises indentation, whitespace, and common style patterns.
// ==============================================================================

use crate::document::Document;
use crate::protocol::LspRange;

pub struct TextEdit {
    pub range: LspRange,
    pub new_text: String,
}

impl TextEdit {
    pub fn to_json(&self) -> String {
        // Escape new_text for JSON
        let mut escaped = String::with_capacity(self.new_text.len() + 8);
        for ch in self.new_text.chars() {
            match ch {
                '"' => escaped.push_str("\\\""),
                '\\' => escaped.push_str("\\\\"),
                '\n' => escaped.push_str("\\n"),
                '\r' => escaped.push_str("\\r"),
                '\t' => escaped.push_str("\\t"),
                c => escaped.push(c),
            }
        }
        format!(
            r#"{{"range":{{"start":{{"line":{},"character":{}}},"end":{{"line":{},"character":{}}}}},"newText":"{}"}}"#,
            self.range.start.line,
            self.range.start.character,
            self.range.end.line,
            self.range.end.character,
            escaped,
        )
    }
}

/// Format the entire document, returning a single full-replacement TextEdit.
pub fn format_document(document: &Document) -> Vec<TextEdit> {
    let formatted = format_source(&document.text);
    if formatted == document.text {
        return vec![];
    }
    let total_lines = document.lines.len() as u32;
    let last_col = document.lines.last().map(|l| l.len() as u32).unwrap_or(0);
    vec![TextEdit {
        range: LspRange::new(0, 0, total_lines, last_col),
        new_text: formatted,
    }]
}

/// Format a range within the document.
pub fn format_range(document: &Document, range: &LspRange) -> Vec<TextEdit> {
    let start_line = range.start.line as usize;
    let end_line = (range.end.line as usize).min(document.lines.len().saturating_sub(1));

    let subset: Vec<&str> = document.lines[start_line..=end_line]
        .iter()
        .map(|s| s.as_str())
        .collect();
    let raw = subset.join("\n");
    let formatted = format_source(&raw);

    if formatted == raw {
        return vec![];
    }

    let end_col = document.lines.get(end_line).map(|l| l.len() as u32).unwrap_or(0);
    vec![TextEdit {
        range: LspRange::new(
            start_line as u32,
            0,
            end_line as u32,
            end_col,
        ),
        new_text: formatted,
    }]
}

// ─── Core formatting logic ────────────────────────────────────────────────────

fn format_source(source: &str) -> String {
    let mut result = String::with_capacity(source.len());
    let mut blank_count = 0usize;
    let mut prev_was_top_level = false;

    for line in source.lines() {
        let trimmed_end = line.trim_end();

        if trimmed_end.is_empty() {
            blank_count += 1;
            continue;
        }

        // Count leading spaces (tabs → 4 spaces each)
        let leading = leading_indent(trimmed_end);
        let is_top_level = leading == 0;

        // Ensure at most 1 blank line between declarations; 2 between top-level
        if blank_count > 0 {
            let max_blanks = if is_top_level && prev_was_top_level { 2 } else { 1 };
            let blanks = blank_count.min(max_blanks);
            for _ in 0..blanks {
                result.push('\n');
            }
        }

        // Snap indentation to 4-space multiples
        let snapped = (leading / 4) * 4;
        let content = trimmed_end.trim_start();
        let normalised = normalize_line(content);

        result.push_str(&" ".repeat(snapped));
        result.push_str(&normalised);
        result.push('\n');

        blank_count = 0;
        prev_was_top_level = is_top_level;
    }

    // Preserve trailing newline behaviour
    result
}

fn leading_indent(line: &str) -> usize {
    let mut count = 0;
    for ch in line.chars() {
        match ch {
            ' ' => count += 1,
            '\t' => count += 4,
            _ => break,
        }
    }
    count
}

/// Normalize spacing within a single source line (content only, no leading ws).
fn normalize_line(line: &str) -> String {
    // We don't modify string literals or comments
    if line.starts_with('#') {
        return line.to_string();
    }

    let mut out = String::with_capacity(line.len());
    let mut in_str = false;
    let mut str_char = ' ';
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let ch = chars[i];

        if in_str {
            out.push(ch);
            if ch == '\\' && i + 1 < len {
                i += 1;
                out.push(chars[i]);
            } else if ch == str_char {
                in_str = false;
            }
            i += 1;
            continue;
        }

        if ch == '"' || ch == '\'' {
            in_str = true;
            str_char = ch;
            out.push(ch);
            i += 1;
            continue;
        }

        if ch == '#' {
            // Rest is comment
            out.push_str(&line[i..]);
            break;
        }

        // Normalize `->` spacing
        if ch == '-' && i + 1 < len && chars[i + 1] == '>' {
            // Ensure space before `->`
            if !out.ends_with(' ') {
                out.push(' ');
            }
            out.push_str("->");
            i += 2;
            // Ensure space after `->`
            if i < len && chars[i] != ' ' {
                out.push(' ');
            }
            continue;
        }

        // Normalize binary operators: = += -= etc. (not inside generics <>)
        if (ch == '=' || ch == '+' || ch == '-' || ch == '*' || ch == '/')
            && i + 1 < len
            && chars[i + 1] == '='
        {
            if !out.ends_with(' ') {
                out.push(' ');
            }
            out.push(ch);
            out.push('=');
            i += 2;
            if i < len && chars[i] != ' ' {
                out.push(' ');
            }
            continue;
        }

        if ch == '=' && (i == 0 || chars[i - 1] != '!' && chars[i - 1] != '<'
            && chars[i - 1] != '>' && chars[i - 1] != '=')
            && (i + 1 >= len || chars[i + 1] != '=')
        {
            // plain `=` assignment
            if !out.ends_with(' ') && !out.is_empty() {
                out.push(' ');
            }
            out.push('=');
            if i + 1 < len && chars[i + 1] != ' ' {
                out.push(' ');
            }
            i += 1;
            continue;
        }

        out.push(ch);
        i += 1;
    }

    out
}

