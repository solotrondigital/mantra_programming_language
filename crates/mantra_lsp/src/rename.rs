// ==============================================================================
// mantra_lsp::rename — Symbol Rename (cross-file workspace edits)
// ==============================================================================

use crate::analysis::AnalysisResult;
use crate::document::Document;
use crate::navigation::find_references;
use crate::protocol::{escape_json_str, LspRange};
use std::collections::HashMap;

/// Check whether the word at (line, character) is renameable.
/// Returns the range of that token, or None if not renameable.
pub fn prepare_rename(
    document: &Document,
    line: u32,
    character: u32,
    analysis: &AnalysisResult,
) -> Option<LspRange> {
    let (word, start, end) = document.word_range_at(line, character)?;
    if word.is_empty() {
        return None;
    }
    // Reject renaming keywords and built-ins
    if is_keyword_or_builtin(&word) {
        return None;
    }
    // Must be a known user-defined symbol
    let st = analysis.symbol_table.as_ref()?;
    let known = st.classes.contains_key(&word)
        || st.interfaces.contains_key(&word)
        || st.functions.contains_key(&word)
        || st.classes.values().any(|c| {
            c.methods.contains_key(&word)
                || c.fields.contains_key(&word)
                || c.static_methods.contains_key(&word)
                || c.static_fields.contains_key(&word)
        })
        || st.interfaces.values().any(|i| i.methods.contains_key(&word));

    if known {
        Some(LspRange::new(line, start, line, end))
    } else {
        // Allow renaming even if not in symbol table (might be a variable)
        Some(LspRange::new(line, start, line, end))
    }
}

/// Build a workspace edit that renames all occurrences of the symbol under cursor.
pub fn rename_symbol(
    new_name: &str,
    document: &Document,
    line: u32,
    character: u32,
    all_documents: &HashMap<String, Document>,
    _all_analyses: &HashMap<String, AnalysisResult>,
) -> String {
    let word = match document.word_at(line, character) {
        Some(w) => w,
        None => return r#"{"changes":{}}"#.to_string(),
    };

    if is_keyword_or_builtin(&word) {
        return r#"{"changes":{}}"#.to_string();
    }

    // Find all references across all documents
    let refs = find_references(document, line, character, all_documents, _all_analyses);

    // Group by URI
    let mut by_uri: HashMap<String, Vec<LspRange>> = HashMap::new();
    for loc in refs {
        by_uri.entry(loc.uri).or_default().push(loc.range);
    }

    // Also include current document location
    if let Some((_, start, end)) = document.word_range_at(line, character) {
        by_uri
            .entry(document.uri.clone())
            .or_default()
            .push(LspRange::new(line, start, line, end));
    }

    // Build JSON
    let mut changes_parts = Vec::new();
    for (uri, ranges) in &by_uri {
        let edits: Vec<String> = ranges
            .iter()
            .map(|r| {
                format!(
                    r#"{{"range":{{"start":{{"line":{},"character":{}}},"end":{{"line":{},"character":{}}}}},"newText":"{}"}}"#,
                    r.start.line,
                    r.start.character,
                    r.end.line,
                    r.end.character,
                    escape_json_str(new_name),
                )
            })
            .collect();
        changes_parts.push(format!(
            r#""{}":[{}]"#,
            escape_json_str(uri),
            edits.join(",")
        ));
    }

    format!(r#"{{"changes":{{{}}}}}"#, changes_parts.join(","))
}

fn is_keyword_or_builtin(word: &str) -> bool {
    matches!(
        word,
        "def" | "class" | "interface" | "extends" | "implements" | "abstract"
            | "override" | "static" | "pub" | "private" | "protected" | "let" | "mut"
            | "if" | "elif" | "else" | "for" | "in" | "while" | "break" | "continue"
            | "async" | "await" | "return" | "module" | "import" | "from" | "as"
            | "self" | "super" | "and" | "or" | "not" | "True" | "False" | "None"
            | "print" | "len" | "range" | "str" | "int" | "float" | "assert"
            | "read_file" | "write_file" | "system_exec" | "get_env"
            | "Int" | "Float" | "Bool" | "String" | "Byte" | "Unit" | "Any"
            | "Option" | "Result" | "List" | "Map"
    )
}

