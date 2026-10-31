// ==============================================================================
// mantra_lsp::hover — Hover Documentation Provider
// ==============================================================================

use crate::analysis::AnalysisResult;
use crate::document::Document;
use crate::protocol::{escape_json_str, LspRange};

pub struct HoverResult {
    pub markdown: String,
    pub range: LspRange,
}

impl HoverResult {
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"contents":{{"kind":"markdown","value":"{}"}},"range":{{"start":{{"line":{},"character":{}}},"end":{{"line":{},"character":{}}}}}}}"#,
            escape_json_str(&self.markdown),
            self.range.start.line,
            self.range.start.character,
            self.range.end.line,
            self.range.end.character,
        )
    }
}

/// Return hover documentation for the symbol at (line, character).
pub fn get_hover(
    document: &Document,
    line: u32,
    character: u32,
    analysis: &AnalysisResult,
) -> Option<HoverResult> {
    let (word, start_col, end_col) = document.word_range_at(line, character)?;
    if word.is_empty() {
        return None;
    }

    let range = LspRange::new(line, start_col, line, end_col);

    // 1. Check symbol table
    if let Some(st) = &analysis.symbol_table {
        if let Some(cls) = st.classes.get(&word) {
            let mut md = format!("```mantra\n");
            if cls.is_abstract {
                md.push_str("abstract ");
            }
            md.push_str(&format!("class {}", cls.name));
            if let Some(base) = &cls.base_class {
                md.push_str(&format!(" extends {}", base));
            }
            if !cls.interfaces.is_empty() {
                md.push_str(&format!(" implements {}", cls.interfaces.join(", ")));
            }
            md.push_str("\n```");
            let fields = cls.fields.len() + cls.static_fields.len();
            let methods = cls.methods.len() + cls.static_methods.len();
            md.push_str(&format!("\n\n*{} fields, {} methods*", fields, methods));
            return Some(HoverResult { markdown: md, range });
        }

        if let Some(iface) = st.interfaces.get(&word) {
            let md = format!(
                "```mantra\ninterface {}\n```\n\n*{} method(s)*",
                iface.name,
                iface.methods.len()
            );
            return Some(HoverResult { markdown: md, range });
        }

        if let Some(func) = st.functions.get(&word) {
            let params: Vec<String> = func
                .params
                .iter()
                .map(|(n, t)| format!("{}: {}", n, t))
                .collect();
            let md = format!(
                "```mantra\ndef {}({}) -> {}\n```",
                func.name,
                params.join(", "),
                func.return_type
            );
            return Some(HoverResult { markdown: md, range });
        }

        // Check class methods by scanning all classes
        for (_cls_name, cls) in &st.classes {
            if let Some(method) = cls.methods.get(&word) {
                let params: Vec<String> = method
                    .params
                    .iter()
                    .map(|(n, t)| format!("{}: {}", n, t))
                    .collect();
                let modifiers = {
                    let mut m = String::new();
                    if method.is_static { m.push_str("static "); }
                    if method.is_abstract { m.push_str("abstract "); }
                    if method.is_override { m.push_str("override "); }
                    m
                };
                let md = format!(
                    "```mantra\n{}def {}({}) -> {}\n```\n\n*Member of `{}`*",
                    modifiers,
                    method.name,
                    params.join(", "),
                    method.return_type,
                    _cls_name
                );
                return Some(HoverResult { markdown: md, range });
            }
        }
    }

    // 2. Built-in functions
    let builtin_hover = builtin_hover(&word);
    if let Some(md) = builtin_hover {
        return Some(HoverResult { markdown: md, range });
    }

    // 3. Keywords
    let kw_hover = keyword_hover(&word);
    if let Some(md) = kw_hover {
        return Some(HoverResult { markdown: md, range });
    }

    // 4. Built-in types
    let type_hover = type_hover(&word);
    if let Some(md) = type_hover {
        return Some(HoverResult { markdown: md, range });
    }

    None
}

fn builtin_hover(word: &str) -> Option<String> {
    match word {
        "print" => Some("```mantra\nprint(value: Any) -> Unit\n```\nPrints value to standard output followed by a newline.".to_string()),
        "len" => Some("```mantra\nlen(collection: Any) -> Int\n```\nReturns the length of a list, string, or map.".to_string()),
        "range" => Some("```mantra\nrange(n: Int) -> List<Int>\n```\nReturns an integer range from `0` to `n-1`.".to_string()),
        "str" => Some("```mantra\nstr(val: Any) -> String\n```\nConverts a value to its string representation.".to_string()),
        "int" => Some("```mantra\nint(val: Any) -> Int\n```\nConverts a value to a 64-bit integer.".to_string()),
        "float" => Some("```mantra\nfloat(val: Any) -> Float\n```\nConverts a value to a 64-bit float.".to_string()),
        "assert" => Some("```mantra\nassert(condition: Bool) -> Unit\n```\nAsserts a boolean condition at runtime. Panics if false.".to_string()),
        "read_file" => Some("```mantra\nread_file(path: String) -> String\n```\nReads the entire contents of a file into a string.".to_string()),
        "write_file" => Some("```mantra\nwrite_file(path: String, content: String) -> Bool\n```\nWrites string content to a file. Returns `True` on success.".to_string()),
        "system_exec" => Some("```mantra\nsystem_exec(cmd: String) -> Int\n```\nExecutes a shell command. Returns the process exit code.".to_string()),
        "get_env" => Some("```mantra\nget_env(key: String, default_val: String) -> String\n```\nReturns the value of environment variable `key`, or `default_val` if not set.".to_string()),
        _ => None,
    }
}

fn keyword_hover(word: &str) -> Option<String> {
    match word {
        "def" => Some("```mantra\ndef name(params) -> ReturnType:\n```\nDeclares a function or method.".to_string()),
        "class" => Some("```mantra\nclass Name [extends Base] [implements Interface]:\n```\nDeclares an object-oriented class.".to_string()),
        "interface" => Some("```mantra\ninterface Name:\n    def method(params) -> ReturnType\n```\nDeclares an abstract interface contract.".to_string()),
        "async" => Some("```mantra\nasync def name() -> Task[T]:\n```\nDeclares an asynchronous coroutine.".to_string()),
        "await" => Some("```mantra\nresult = await task\n```\nSuspends execution until an async task resolves.".to_string()),
        "return" => Some("```mantra\nreturn value\n```\nReturns a value from the current function.".to_string()),
        "let" => Some("```mantra\nlet name: Type = value\n```\nDeclares an immutable variable binding.".to_string()),
        "mut" => Some("```mantra\nmut name: Type = value\n```\nDeclares a mutable variable binding.".to_string()),
        "if" | "elif" | "else" => Some("```mantra\nif condition:\n    ...\nelif other:\n    ...\nelse:\n    ...\n```\nConditional control flow.".to_string()),
        "for" => Some("```mantra\nfor item in iterable:\n    ...\n```\nIterates over a collection.".to_string()),
        "while" => Some("```mantra\nwhile condition:\n    ...\n```\nRepeats while condition is true.".to_string()),
        "import" => Some("```mantra\nimport module.path\nfrom module import Name\n```\nImports a module or symbol.".to_string()),
        "extends" => Some("Inherits from a parent class.".to_string()),
        "implements" => Some("Declares that a class implements an interface.".to_string()),
        "pub" => Some("Makes a declaration publicly accessible.".to_string()),
        "private" => Some("Restricts access to the declaring class.".to_string()),
        "protected" => Some("Restricts access to the declaring class and subclasses.".to_string()),
        "static" => Some("Marks a member as belonging to the class, not instances.".to_string()),
        "abstract" => Some("Marks a class or method as abstract (must be overridden).".to_string()),
        "override" => Some("Marks a method as explicitly overriding a parent method.".to_string()),
        "self" => Some("Reference to the current class instance.".to_string()),
        "super" => Some("Reference to the parent class instance or constructor.".to_string()),
        "True" => Some("```mantra\nTrue: Bool\n```\nBoolean true literal.".to_string()),
        "False" => Some("```mantra\nFalse: Bool\n```\nBoolean false literal.".to_string()),
        "None" => Some("```mantra\nNone\n```\nThe null/absent value.".to_string()),
        _ => None,
    }
}

fn type_hover(word: &str) -> Option<String> {
    match word {
        "Int" => Some("```mantra\nInt\n```\n64-bit signed integer type.".to_string()),
        "Float" => Some("```mantra\nFloat\n```\n64-bit floating point type.".to_string()),
        "Bool" => Some("```mantra\nBool\n```\nBoolean type (`True` / `False`).".to_string()),
        "String" => Some("```mantra\nString\n```\nUTF-8 string type.".to_string()),
        "Byte" => Some("```mantra\nByte\n```\n8-bit unsigned byte.".to_string()),
        "Unit" => Some("```mantra\nUnit\n```\nThe unit type — indicates no meaningful return value.".to_string()),
        "Any" => Some("```mantra\nAny\n```\nDynamic type — accepts any value.".to_string()),
        "Option" => Some("```mantra\nOption<T>\n```\nRepresents an optional value (`Some(T)` or `None`).".to_string()),
        "Result" => Some("```mantra\nResult<T, E>\n```\nRepresents success (`Ok(T)`) or failure (`Err(E)`).".to_string()),
        "List" => Some("```mantra\nList<T>\n```\nOrdered dynamic array of elements of type `T`.".to_string()),
        "Map" => Some("```mantra\nMap<K, V>\n```\nHash map from keys of type `K` to values of type `V`.".to_string()),
        _ => None,
    }
}

