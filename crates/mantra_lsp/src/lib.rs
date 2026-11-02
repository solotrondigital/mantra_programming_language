// ==============================================================================
// mantra_lsp — Universal Mantra Language Server
// LSP 3.17 compatible language server for the Mantra programming language.
// ==============================================================================

pub mod analysis;
pub mod code_actions;
pub mod completion;
pub mod document;
pub mod formatting;
pub mod hover;
pub mod navigation;
pub mod path_utils;
pub mod protocol;
pub mod rename;
pub mod semantic_tokens;
pub mod server;
pub mod signature_help;
pub mod symbol_index;

// Public re-exports (backward-compatible surface)
pub use protocol::{
    LspCompletionItem, LspDiagnostic, LspHover, LspPosition, LspRange, LspTextEdit,
};
pub use server::LspServer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostics_on_valid_source() {
        let server = LspServer::new();
        let src = "def main() -> Int:\n    print(\"Hello, LSP!\")\n    return 0\n";
        let diags = server.compute_diagnostics(src);
        assert!(
            diags.is_empty(),
            "Valid source should have zero diagnostics: {:?}",
            diags
        );
    }

    #[test]
    fn test_diagnostics_on_syntax_error() {
        let server = LspServer::new();
        let src = "def main(\n"; // Unclosed paren
        let diags = server.compute_diagnostics(src);
        assert!(
            !diags.is_empty(),
            "Syntax error must produce LSP diagnostics"
        );
    }

    #[test]
    fn test_completions() {
        let server = LspServer::new();
        let items = server.get_completions("file:///main.mantra", &LspPosition::new(0, 0));
        assert!(items.iter().any(|i| i.label == "def"));
        assert!(items.iter().any(|i| i.label == "class"));
        assert!(items.iter().any(|i| i.label == "print"));
    }

    #[test]
    fn test_hover() {
        let mut server = LspServer::new();
        let uri = "file:///main.mantra";
        server.open_document(
            uri.to_string(),
            "def main() -> Int:\n    print(1)\n".to_string(),
        );
        let hover = server.get_hover(uri, &LspPosition::new(0, 1));
        assert!(hover.is_some());
        assert!(hover.unwrap().contents.contains("def"));
    }

    #[test]
    fn test_format_document() {
        let mut server = LspServer::new();
        let uri = "file:///main.mantra";
        server.open_document(
            uri.to_string(),
            "def main() -> Int:\n  print(1)  \n".to_string(),
        );
        let edits = server.format_document(uri);
        assert!(edits.is_some());
        let formatted = &edits.unwrap()[0].new_text;
        assert!(
            !formatted.contains("  \n"),
            "Trailing whitespace must be stripped"
        );
    }

    #[test]
    fn test_initialize_message() {
        let mut server = LspServer::new();
        let init_msg = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
        let resp = server.handle_message(init_msg);
        assert!(resp.is_some());
        let resp_str = resp.unwrap();
        let _parsed: serde_json::Value =
            serde_json::from_str(&resp_str).expect("Initialize response must be valid JSON");
        assert!(resp_str.contains(r#""capabilities""#));
        assert!(server.initialized);
    }

    #[test]
    fn test_lsp_full_lifecycle() {
        let mut server = LspServer::new();

        // Initialize
        let init = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"rootUri":null}}"#;
        let resp = server.handle_message(init).unwrap();
        assert!(resp.contains("completionProvider"));
        assert!(resp.contains("hoverProvider"));
        assert!(resp.contains("definitionProvider"));
        assert!(resp.contains("referencesProvider"));
        assert!(resp.contains("semanticTokensProvider"));
        assert!(resp.contains("renameProvider"));

        // Open document
        let open_msg = r#"{"jsonrpc":"2.0","method":"textDocument/didOpen","params":{"textDocument":{"uri":"file:///test.mantra","languageId":"mantra","version":1,"text":"def hello() -> String:\n    return \"hi\"\n"}}}"#;
        server.handle_message(open_msg);

        // Completion
        let comp_msg = r#"{"jsonrpc":"2.0","id":2,"method":"textDocument/completion","params":{"textDocument":{"uri":"file:///test.mantra"},"position":{"line":0,"character":0}}}"#;
        let comp_resp = server.handle_message(comp_msg).unwrap();
        assert!(comp_resp.contains("\"label\""));

        // Shutdown
        let shut = r#"{"jsonrpc":"2.0","id":99,"method":"shutdown","params":null}"#;
        let shut_resp = server.handle_message(shut).unwrap();
        assert!(shut_resp.contains("null"));
    }

    #[test]
    fn test_document_symbols() {
        let mut server = LspServer::new();
        let uri = "file:///sym.mantra";
        let src = "class Foo:\n    def bar(self) -> Int:\n        return 1\n";
        server.open_document(uri.to_string(), src.to_string());

        let msg = format!(
            r#"{{"jsonrpc":"2.0","id":3,"method":"textDocument/documentSymbol","params":{{"textDocument":{{"uri":"{}"}}}}}}"#,
            uri
        );
        let resp = server.handle_message(&msg).unwrap();
        assert!(resp.contains("Foo"), "Should contain class name Foo");
    }

    #[test]
    fn test_semantic_tokens() {
        let mut server = LspServer::new();
        let uri = "file:///tok.mantra";
        server.open_document(
            uri.to_string(),
            "def main() -> Int:\n    return 0\n".to_string(),
        );
        let msg = format!(
            r#"{{"jsonrpc":"2.0","id":4,"method":"textDocument/semanticTokens/full","params":{{"textDocument":{{"uri":"{}"}}}}}}"#,
            uri
        );
        let resp = server.handle_message(&msg).unwrap();
        assert!(
            resp.contains("\"data\""),
            "Semantic tokens must contain data array"
        );
    }

    #[test]
    fn test_hover_keyword() {
        let mut server = LspServer::new();
        let uri = "file:///kw.mantra";
        server.open_document(
            uri.to_string(),
            "def main() -> Int:\n    return 0\n".to_string(),
        );
        // Hover over "def" at (0, 1)
        let hover = server.get_hover(uri, &LspPosition::new(0, 1));
        assert!(hover.is_some());
    }

    #[test]
    fn test_type_mismatch_diagnostic() {
        let server = LspServer::new();
        let src = "def main() -> Int:\n    let x: Int = \"invalid\"\n    return 0\n";
        let diags = server.compute_diagnostics(src);
        assert!(!diags.is_empty(), "Type error was not detected");
    }

    #[test]
    fn test_go_to_definition_resolves_local_function() {
        let uri = "file:///navigation.mantra";
        let source =
            "def helper() -> Int:\n    return 7\n\ndef main() -> Int:\n    return helper()\n";
        let document = document::Document::new(uri.to_string(), source.to_string(), 1);
        let analysis = analysis::analyze(source);
        let location = navigation::go_to_definition(
            &document,
            4,
            12,
            &analysis,
            &std::collections::HashMap::new(),
        )
        .expect("helper call resolves to a definition");
        assert_eq!(location.uri, uri);
        assert_eq!(location.range.start.line, 0);
    }

    #[test]
    fn test_ai_quick_fix_is_offered_as_reviewable_edit() {
        let mut server = LspServer::new();
        let uri = "file:///review.mantra";
        let source = "class Email:\n    pub from: String\n";
        server.open_document(uri.to_string(), source.to_string());

        let request = format!(
            r#"{{"jsonrpc":"2.0","id":32,"method":"textDocument/codeAction","params":{{"textDocument":{{"uri":"{}"}},"range":{{"start":{{"line":1,"character":4}},"end":{{"line":1,"character":20}}}},"context":{{"diagnostics":[{{"code":"E0021","message":"Expected field. Found 'from'"}}]}}}}}}"#,
            uri
        );
        let response = server.handle_message(&request).unwrap();
        assert!(response.contains("Mantra AI: Explain compiler error"));
        assert!(
            response.contains("\"edit\":"),
            "quick fix should be presented as an edit for user review: {response}"
        );
        assert_eq!(
            server.documents.get(uri).unwrap().text,
            source,
            "requesting actions must not change the open document"
        );
    }
}
