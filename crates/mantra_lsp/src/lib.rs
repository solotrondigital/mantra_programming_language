// ==============================================================================
// mantra_lsp — Language Server Protocol Server for Mantra
// ==============================================================================

pub mod protocol;
pub mod server;

pub use protocol::{LspCompletionItem, LspDiagnostic, LspHover, LspPosition, LspRange, LspTextEdit};
pub use server::LspServer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostics_on_valid_source() {
        let server = LspServer::new();
        let src = "def main() -> Int:\n    print(\"Hello, LSP!\")\n    return 0\n";
        let diags = server.compute_diagnostics(src);
        assert!(diags.is_empty(), "Valid source should have zero diagnostics: {:?}", diags);
    }

    #[test]
    fn test_diagnostics_on_syntax_error() {
        let server = LspServer::new();
        let src = "def main(\n"; // Unclosed paren
        let diags = server.compute_diagnostics(src);
        assert!(!diags.is_empty(), "Syntax error must produce LSP diagnostics");
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
        server.open_document(uri.to_string(), "def main() -> Int:\n    print(1)\n".to_string());
        let hover = server.get_hover(uri, &LspPosition::new(0, 1));
        assert!(hover.is_some());
        assert!(hover.unwrap().contents.contains("def name"));
    }

    #[test]
    fn test_format_document() {
        let mut server = LspServer::new();
        let uri = "file:///main.mantra";
        server.open_document(uri.to_string(), "def main() -> Int:\n  print(1)  \n".to_string());
        let edits = server.format_document(uri);
        assert!(edits.is_some());
        let formatted = &edits.unwrap()[0].new_text;
        assert!(!formatted.contains("  \n"));
    }

    #[test]
    fn test_initialize_message() {
        let mut server = LspServer::new();
        let init_msg = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
        let resp = server.handle_message(init_msg);
        assert!(resp.is_some());
        assert!(resp.unwrap().contains(r#""capabilities""#));
        assert!(server.initialized);
    }
}

