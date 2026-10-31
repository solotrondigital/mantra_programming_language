// ==============================================================================
// mantra_lsp::signature_help — Function Signature Help Provider
// ==============================================================================

use crate::analysis::AnalysisResult;
use crate::document::Document;
use crate::protocol::escape_json_str;

/// Compute signature help JSON for the call site at (line, character).
pub fn get_signature_help_json(
    document: &Document,
    line: u32,
    character: u32,
    analysis: &AnalysisResult,
) -> Option<String> {
    let prefix = document.line_prefix(line, character);

    // Find the nearest open paren to determine function name and active param
    let (func_name, active_param) = find_call_context(prefix)?;

    let st = analysis.symbol_table.as_ref()?;

    // Look up function in symbol table
    if let Some(func) = st.functions.get(&func_name) {
        return Some(build_signature_json(&func_name, &func.params, active_param));
    }

    // Look up methods across all classes
    for cls in st.classes.values() {
        if let Some(method) = cls.methods.get(&func_name) {
            return Some(build_signature_json(&func_name, &method.params, active_param));
        }
        if let Some(method) = cls.static_methods.get(&func_name) {
            return Some(build_signature_json(&func_name, &method.params, active_param));
        }
    }

    // Built-in signatures
    builtin_signature_json(&func_name, active_param)
}

fn find_call_context(prefix: &str) -> Option<(String, u32)> {
    // Walk backwards from end to find nearest unmatched `(`
    let bytes = prefix.as_bytes();
    let mut depth = 0i32;
    let mut comma_count = 0u32;
    let mut paren_pos = None;

    for i in (0..bytes.len()).rev() {
        match bytes[i] {
            b')' | b']' => depth += 1,
            b'(' | b'[' => {
                if depth == 0 {
                    paren_pos = Some(i);
                    break;
                }
                depth -= 1;
            }
            b',' if depth == 0 => comma_count += 1,
            _ => {}
        }
    }

    let paren_pos = paren_pos?;
    let before_paren = &prefix[..paren_pos].trim_end();

    // Extract the function/method name before `(`
    let name_end = before_paren.len();
    let name_start = before_paren
        .rfind(|c: char| !c.is_alphanumeric() && c != '_')
        .map(|i| i + 1)
        .unwrap_or(0);
    let name = &before_paren[name_start..name_end];

    if name.is_empty() {
        None
    } else {
        Some((name.to_string(), comma_count))
    }
}

fn build_signature_json(
    name: &str,
    params: &[(String, mantrac_ast::Type)],
    active_param: u32,
) -> String {
    let param_labels: Vec<String> = params
        .iter()
        .map(|(n, t)| format!("{}: {}", n, t))
        .collect();
    let signature_label = format!("{}({})", name, param_labels.join(", "));

    let param_infos: Vec<String> = param_labels
        .iter()
        .map(|p| format!(r#"{{"label":"{}"}}"#, escape_json_str(p)))
        .collect();

    format!(
        r#"{{"signatures":[{{"label":"{}","parameters":[{}]}}],"activeSignature":0,"activeParameter":{}}}"#,
        escape_json_str(&signature_label),
        param_infos.join(","),
        active_param.min(params.len().saturating_sub(1) as u32),
    )
}

fn builtin_signature_json(name: &str, active_param: u32) -> Option<String> {
    let (label, params) = match name {
        "print" => ("print(value: Any) -> Unit", vec!["value: Any"]),
        "len" => ("len(collection: Any) -> Int", vec!["collection: Any"]),
        "range" => ("range(n: Int) -> List<Int>", vec!["n: Int"]),
        "str" => ("str(val: Any) -> String", vec!["val: Any"]),
        "int" => ("int(val: Any) -> Int", vec!["val: Any"]),
        "float" => ("float(val: Any) -> Float", vec!["val: Any"]),
        "assert" => ("assert(condition: Bool) -> Unit", vec!["condition: Bool"]),
        "read_file" => ("read_file(path: String) -> String", vec!["path: String"]),
        "write_file" => (
            "write_file(path: String, content: String) -> Bool",
            vec!["path: String", "content: String"],
        ),
        "system_exec" => ("system_exec(cmd: String) -> Int", vec!["cmd: String"]),
        "get_env" => (
            "get_env(key: String, default_val: String) -> String",
            vec!["key: String", "default_val: String"],
        ),
        _ => return None,
    };

    let param_infos: Vec<String> = params
        .iter()
        .map(|p| format!(r#"{{"label":"{}"}}"#, escape_json_str(p)))
        .collect();

    Some(format!(
        r#"{{"signatures":[{{"label":"{}","parameters":[{}]}}],"activeSignature":0,"activeParameter":{}}}"#,
        escape_json_str(label),
        param_infos.join(","),
        active_param.min(params.len().saturating_sub(1) as u32),
    ))
}

