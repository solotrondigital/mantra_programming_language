// ==============================================================================
// mantra_lsp::completion — Context-aware IntelliSense Completion Engine
// ==============================================================================

use crate::analysis::AnalysisResult;
use crate::document::Document;
use crate::protocol::{completion_kind, escape_json_str, insert_text_format};

pub struct CompletionItem {
    pub label: String,
    pub kind: u32,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: Option<String>,
    pub insert_text_format: u32,
    pub sort_text: Option<String>,
}

impl CompletionItem {
    fn keyword(label: &str, doc: &str) -> Self {
        Self {
            label: label.to_string(),
            kind: completion_kind::KEYWORD,
            detail: Some("keyword".to_string()),
            documentation: Some(doc.to_string()),
            insert_text: None,
            insert_text_format: insert_text_format::PLAIN_TEXT,
            sort_text: Some(format!("z_{label}")),
        }
    }

    fn builtin_fn(label: &str, sig: &str, doc: &str) -> Self {
        Self {
            label: label.to_string(),
            kind: completion_kind::FUNCTION,
            detail: Some(sig.to_string()),
            documentation: Some(doc.to_string()),
            insert_text: Some(format!("{label}($0)")),
            insert_text_format: insert_text_format::SNIPPET,
            sort_text: Some(format!("b_{label}")),
        }
    }

    fn class_item(name: &str, detail: Option<String>) -> Self {
        Self {
            label: name.to_string(),
            kind: completion_kind::CLASS,
            detail,
            documentation: None,
            insert_text: None,
            insert_text_format: insert_text_format::PLAIN_TEXT,
            sort_text: Some(format!("a_{name}")),
        }
    }

    fn interface_item(name: &str) -> Self {
        Self {
            label: name.to_string(),
            kind: completion_kind::INTERFACE,
            detail: Some("interface".to_string()),
            documentation: None,
            insert_text: None,
            insert_text_format: insert_text_format::PLAIN_TEXT,
            sort_text: Some(format!("a_{name}")),
        }
    }

    fn function_item(name: &str, sig: &str) -> Self {
        Self {
            label: name.to_string(),
            kind: completion_kind::FUNCTION,
            detail: Some(sig.to_string()),
            documentation: None,
            insert_text: Some(format!("{name}($0)")),
            insert_text_format: insert_text_format::SNIPPET,
            sort_text: Some(format!("a_{name}")),
        }
    }

    fn field_item(name: &str, ty: &str) -> Self {
        Self {
            label: name.to_string(),
            kind: completion_kind::FIELD,
            detail: Some(ty.to_string()),
            documentation: None,
            insert_text: None,
            insert_text_format: insert_text_format::PLAIN_TEXT,
            sort_text: Some(format!("a_{name}")),
        }
    }

    fn method_item(name: &str, sig: &str) -> Self {
        Self {
            label: name.to_string(),
            kind: completion_kind::METHOD,
            detail: Some(sig.to_string()),
            documentation: None,
            insert_text: Some(format!("{name}($0)")),
            insert_text_format: insert_text_format::SNIPPET,
            sort_text: Some(format!("a_{name}")),
        }
    }

    fn type_item(name: &str) -> Self {
        Self {
            label: name.to_string(),
            kind: completion_kind::CLASS,
            detail: Some("type".to_string()),
            documentation: None,
            insert_text: None,
            insert_text_format: insert_text_format::PLAIN_TEXT,
            sort_text: Some(format!("c_{name}")),
        }
    }

    #[allow(dead_code)]
    fn variable_item(name: &str, ty: &str) -> Self {
        Self {
            label: name.to_string(),
            kind: completion_kind::VARIABLE,
            detail: Some(ty.to_string()),
            documentation: None,
            insert_text: None,
            insert_text_format: insert_text_format::PLAIN_TEXT,
            sort_text: Some(format!("a_{name}")),
        }
    }

    pub fn to_json(&self) -> String {
        let detail = self
            .detail
            .as_deref()
            .map(|d| format!(r#","detail":"{}""#, escape_json_str(d)))
            .unwrap_or_default();
        let doc = self
            .documentation
            .as_deref()
            .map(|d| format!(r#","documentation":{{"kind":"markdown","value":"{}"}}"#, escape_json_str(d)))
            .unwrap_or_default();
        let ins = self
            .insert_text
            .as_deref()
            .map(|t| {
                format!(
                    r#","insertText":"{}","insertTextFormat":{}"#,
                    escape_json_str(t),
                    self.insert_text_format
                )
            })
            .unwrap_or_default();
        let sort = self
            .sort_text
            .as_deref()
            .map(|s| format!(r#","sortText":"{}""#, escape_json_str(s)))
            .unwrap_or_default();
        format!(
            r#"{{"label":"{}","kind":{}{}{}{}{}}}"#,
            escape_json_str(&self.label),
            self.kind,
            detail,
            doc,
            ins,
            sort,
        )
    }
}

// ─── Context detection ────────────────────────────────────────────────────────

#[derive(Debug)]
enum CompletionContext {
    /// After `object.` — member access
    MemberAccess(String),
    /// After `extends ` or `implements ` — class/interface name
    TypeName,
    /// After `import ` or `from ` — module path
    Import,
    /// General: keywords + symbols
    General,
}

fn detect_context(prefix: &str) -> CompletionContext {
    let trimmed = prefix.trim_end();

    // Member access: ends with `.`
    if trimmed.ends_with('.') {
        let obj = trimmed
            .rsplit(|c: char| !c.is_alphanumeric() && c != '_' && c != '.')
            .next()
            .unwrap_or("")
            .trim_end_matches('.');
        return CompletionContext::MemberAccess(obj.to_string());
    }

    // Type position after extends/implements/->
    if trimmed.ends_with("extends ")
        || trimmed.ends_with("implements ")
        || trimmed.ends_with("-> ")
        || trimmed.ends_with(": ")
    {
        return CompletionContext::TypeName;
    }

    // Import context
    if trimmed.contains("import ") || trimmed.starts_with("from ") {
        return CompletionContext::Import;
    }

    CompletionContext::General
}

// ─── Main entry point ─────────────────────────────────────────────────────────

pub fn get_completions(
    document: &Document,
    line: u32,
    character: u32,
    analysis: &AnalysisResult,
) -> Vec<CompletionItem> {
    let prefix = document.line_prefix(line, character);
    let context = detect_context(prefix);

    match context {
        CompletionContext::MemberAccess(obj_name) => {
            member_completions(&obj_name, analysis)
        }
        CompletionContext::TypeName => type_completions(analysis),
        CompletionContext::Import => import_completions(analysis),
        CompletionContext::General => general_completions(document, analysis),
    }
}

pub fn completions_to_json(items: &[CompletionItem]) -> String {
    let list: Vec<String> = items.iter().map(|i| i.to_json()).collect();
    format!("[{}]", list.join(","))
}

// ─── Context-specific completers ──────────────────────────────────────────────

fn member_completions(obj_name: &str, analysis: &AnalysisResult) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    let Some(st) = &analysis.symbol_table else {
        return items;
    };

    // Try to find this as a class name for static access
    if let Some(cls) = st.classes.get(obj_name) {
        for (name, field) in &cls.static_fields {
            items.push(CompletionItem::field_item(name, &field.ty.to_string()));
        }
        for (name, method) in &cls.static_methods {
            let params: Vec<String> = method
                .params
                .iter()
                .map(|(n, t)| format!("{}: {}", n, t))
                .collect();
            items.push(CompletionItem::method_item(
                name,
                &format!("{}({}) -> {}", name, params.join(", "), method.return_type),
            ));
        }
        // Also instance members for object references
        for (name, field) in &cls.fields {
            items.push(CompletionItem::field_item(name, &field.ty.to_string()));
        }
        for (name, method) in &cls.methods {
            if name == "__init__" {
                continue;
            }
            let params: Vec<String> = method
                .params
                .iter()
                .map(|(n, t)| format!("{}: {}", n, t))
                .collect();
            items.push(CompletionItem::method_item(
                name,
                &format!("{}({}) -> {}", name, params.join(", "), method.return_type),
            ));
        }
        return items;
    }

    // Fall back: show all class members across all types (best-effort)
    for (_cls_name, cls) in &st.classes {
        for (name, field) in &cls.fields {
            items.push(CompletionItem::field_item(name, &field.ty.to_string()));
        }
        for (name, method) in &cls.methods {
            if name == "__init__" {
                continue;
            }
            let params: Vec<String> = method
                .params
                .iter()
                .map(|(n, t)| format!("{}: {}", n, t))
                .collect();
            items.push(CompletionItem::method_item(
                name,
                &format!("{}({}) -> {}", name, params.join(", "), method.return_type),
            ));
        }
    }
    items
}

fn type_completions(analysis: &AnalysisResult) -> Vec<CompletionItem> {
    let mut items = vec![
        CompletionItem::type_item("Int"),
        CompletionItem::type_item("Float"),
        CompletionItem::type_item("Bool"),
        CompletionItem::type_item("String"),
        CompletionItem::type_item("Byte"),
        CompletionItem::type_item("Unit"),
        CompletionItem::type_item("None"),
        CompletionItem::type_item("Any"),
        CompletionItem::type_item("Option"),
        CompletionItem::type_item("Result"),
        CompletionItem::type_item("List"),
        CompletionItem::type_item("Map"),
    ];
    if let Some(st) = &analysis.symbol_table {
        for name in st.classes.keys() {
            items.push(CompletionItem::class_item(name, None));
        }
        for name in st.interfaces.keys() {
            items.push(CompletionItem::interface_item(name));
        }
    }
    items
}

fn import_completions(analysis: &AnalysisResult) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    if let Some(st) = &analysis.symbol_table {
        for name in st.classes.keys() {
            items.push(CompletionItem::class_item(name, None));
        }
        for name in st.interfaces.keys() {
            items.push(CompletionItem::interface_item(name));
        }
        for name in st.functions.keys() {
            items.push(CompletionItem::function_item(name, name));
        }
    }
    items
}

fn general_completions(_document: &Document, analysis: &AnalysisResult) -> Vec<CompletionItem> {
    let mut items = Vec::new();

    // ── Symbols from symbol table ─────────────────────────────────────────
    if let Some(st) = &analysis.symbol_table {
        for (name, cls) in &st.classes {
            let detail = cls.base_class.as_deref().map(|b| format!("extends {}", b));
            items.push(CompletionItem::class_item(name, detail));
        }
        for name in st.interfaces.keys() {
            items.push(CompletionItem::interface_item(name));
        }
        for (name, func) in &st.functions {
            let params: Vec<String> = func
                .params
                .iter()
                .map(|(n, t)| format!("{}: {}", n, t))
                .collect();
            items.push(CompletionItem::function_item(
                name,
                &format!("def {}({}) -> {}", name, params.join(", "), func.return_type),
            ));
        }
    }

    // ── Built-in functions ────────────────────────────────────────────────
    let builtins: &[(&str, &str, &str)] = &[
        ("print", "print(value: Any) -> Unit", "Print value to standard output"),
        ("len", "len(collection: Any) -> Int", "Return length of list, string, or map"),
        ("range", "range(n: Int) -> List<Int>", "Return integer range 0..n-1"),
        ("str", "str(val: Any) -> String", "Convert value to string"),
        ("int", "int(val: Any) -> Int", "Convert value to Int"),
        ("float", "float(val: Any) -> Float", "Convert value to Float"),
        ("assert", "assert(condition: Bool) -> Unit", "Assert condition at runtime"),
        ("read_file", "read_file(path: String) -> String", "Read file contents"),
        ("write_file", "write_file(path: String, content: String) -> Bool", "Write file contents"),
        ("system_exec", "system_exec(cmd: String) -> Int", "Execute shell command"),
        ("get_env", "get_env(key: String, default_val: String) -> String", "Get environment variable"),
    ];
    for (label, sig, doc) in builtins {
        items.push(CompletionItem::builtin_fn(label, sig, doc));
    }

    // ── Built-in types ────────────────────────────────────────────────────
    for t in &["Int", "Float", "Bool", "String", "Byte", "Unit", "Any", "None",
               "Option", "Result", "List", "Map"] {
        items.push(CompletionItem::type_item(t));
    }

    // ── Keywords ──────────────────────────────────────────────────────────
    let keywords: &[(&str, &str)] = &[
        ("def", "Define a function or method"),
        ("class", "Define a class"),
        ("interface", "Define an interface"),
        ("return", "Return a value"),
        ("if", "Conditional branch"),
        ("elif", "Else-if branch"),
        ("else", "Else branch"),
        ("while", "While loop"),
        ("for", "For loop"),
        ("in", "Iteration keyword"),
        ("async", "Async function"),
        ("await", "Await async result"),
        ("pub", "Public visibility"),
        ("private", "Private visibility"),
        ("protected", "Protected visibility"),
        ("static", "Static member"),
        ("abstract", "Abstract class/method"),
        ("override", "Override parent method"),
        ("extends", "Class inheritance"),
        ("implements", "Interface implementation"),
        ("let", "Immutable binding"),
        ("mut", "Mutable binding"),
        ("import", "Import module"),
        ("from", "From import"),
        ("as", "Alias import"),
        ("module", "Module declaration"),
        ("self", "Current instance"),
        ("super", "Parent class"),
        ("break", "Break from loop"),
        ("continue", "Continue loop"),
        ("and", "Logical and"),
        ("or", "Logical or"),
        ("not", "Logical not"),
        ("True", "Boolean true"),
        ("False", "Boolean false"),
        ("None", "Null/None value"),
    ];
    for (kw, doc) in keywords {
        items.push(CompletionItem::keyword(kw, doc));
    }

    // ── Snippets ──────────────────────────────────────────────────────────
    items.push(CompletionItem {
        label: "def ...".to_string(),
        kind: completion_kind::SNIPPET,
        detail: Some("Function definition snippet".to_string()),
        documentation: Some("Define a new function".to_string()),
        insert_text: Some("def ${1:name}(${2:params}) -> ${3:Unit}:\n    ${0:pass}".to_string()),
        insert_text_format: insert_text_format::SNIPPET,
        sort_text: Some("z_snippet_def".to_string()),
    });
    items.push(CompletionItem {
        label: "class ...".to_string(),
        kind: completion_kind::SNIPPET,
        detail: Some("Class definition snippet".to_string()),
        documentation: Some("Define a new class".to_string()),
        insert_text: Some("class ${1:Name}:\n    def __init__(self):\n        ${0:pass}".to_string()),
        insert_text_format: insert_text_format::SNIPPET,
        sort_text: Some("z_snippet_class".to_string()),
    });

    items
}

