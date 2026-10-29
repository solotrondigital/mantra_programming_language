use std::fs;
use std::path::Path;

pub struct Formatter;

impl Formatter {
    pub fn format_file(path: &Path) -> Result<bool, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

        let formatted = Self::format_source(&content);

        if formatted != content {
            fs::write(path, &formatted)
                .map_err(|e| format!("Failed to write {}: {}", path.display(), e))?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn format_source(source: &str) -> String {
        let mut result = String::new();
        let lines: Vec<&str> = source.lines().collect();

        for line in lines {
            let trimmed_end = line.trim_end();
            if trimmed_end.is_empty() {
                result.push('\n');
                continue;
            }

            // Count leading spaces/tabs
            let mut leading_spaces = 0;
            for ch in trimmed_end.chars() {
                if ch == ' ' {
                    leading_spaces += 1;
                } else if ch == '\t' {
                    leading_spaces += 4;
                } else {
                    break;
                }
            }

            // Normalize indentation to multiples of 4
            let normalized_indent = (leading_spaces / 4) * 4;
            let indent_str = " ".repeat(normalized_indent);
            let code_part = trimmed_end.trim_start();

            result.push_str(&indent_str);
            result.push_str(code_part);
            result.push('\n');
        }

        result
    }
}

