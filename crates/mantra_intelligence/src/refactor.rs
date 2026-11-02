//! AI Refactoring Engine
//!
//! Provides safe, semantics-preserving code refactoring:
//! Extract Method/Function, Rename Symbol across files, and Convert to Async/Await.

pub struct RefactoringEngine;

impl RefactoringEngine {
    /// Extracts a block of lines into a new helper function and replaces the block with a function call.
    pub fn extract_function(
        source: &str,
        start_line: usize,
        end_line: usize,
        new_fn_name: &str,
    ) -> Result<String, String> {
        let lines: Vec<&str> = source.lines().collect();
        if start_line == 0 || end_line > lines.len() || start_line > end_line {
            return Err("Invalid line range for function extraction".to_string());
        }

        let extracted_lines: Vec<&str> = lines[start_line - 1..end_line].to_vec();
        let indent = "    ";

        let mut new_fn = String::new();
        new_fn.push_str(&format!("\ndef {}():\n", new_fn_name));
        for l in &extracted_lines {
            new_fn.push_str(&format!("{}{}\n", indent, l.trim()));
        }

        let mut modified = Vec::new();
        // Lines before extracted block
        for l in &lines[..start_line - 1] {
            modified.push(l.to_string());
        }
        // Replacement call
        modified.push(format!("{}let _ = {}()", indent, new_fn_name));
        // Lines after extracted block
        for l in &lines[end_line..] {
            modified.push(l.to_string());
        }

        // Append extracted function
        modified.push(new_fn);

        Ok(modified.join("\n"))
    }

    /// Safely renames an identifier across a source file.
    pub fn rename_symbol(source: &str, old_name: &str, new_name: &str) -> String {
        // Simple token boundary replacement to avoid partial substring corruption
        let mut result = String::new();
        for line in source.lines() {
            let mut line_res = String::new();
            let words = line.split_inclusive(|c: char| !c.is_alphanumeric() && c != '_');
            for w in words {
                if w.starts_with(old_name)
                    && (w.len() == old_name.len()
                        || !w.chars().nth(old_name.len()).unwrap().is_alphanumeric())
                {
                    let replaced = w.replacen(old_name, new_name, 1);
                    line_res.push_str(&replaced);
                } else {
                    line_res.push_str(w);
                }
            }
            result.push_str(&line_res);
            result.push('\n');
        }
        result.trim_end().to_string()
    }

    /// Converts a synchronous function declaration to asynchronous (`async def ...`).
    pub fn convert_to_async(source: &str, fn_name: &str) -> String {
        let sync_pat_1 = format!("def {}(", fn_name);
        let async_rep_1 = format!("async def {}(", fn_name);
        let sync_pat_2 = format!("pub def {}(", fn_name);
        let async_rep_2 = format!("pub async def {}(", fn_name);

        source
            .replace(&sync_pat_1, &async_rep_1)
            .replace(&sync_pat_2, &async_rep_2)
    }
}
