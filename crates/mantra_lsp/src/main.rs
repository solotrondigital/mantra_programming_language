// ==============================================================================
// mantra-lsp — Mantra Language Server Protocol CLI Daemon
// ==============================================================================

use mantra_lsp::LspServer;
use std::env;
use std::io::{self, BufRead, Write};

fn print_help() {
    println!(
        r#"Mantra Language Server (mantra-lsp) v0.4.0
Usage:
    mantra-lsp [options]

Options:
    --stdio             Run LSP server over standard I/O (default)
    --test              Run self-test diagnostic and completion check
    -v, --version       Print mantra-lsp version
    -h, --help          Display this help message
"#
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in &args[1..] {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                return;
            }
            "-v" | "--version" => {
                println!("mantra-lsp 0.4.0 (Mantra Language Server Protocol daemon)");
                return;
            }
            "--test" => {
                println!("Running mantra-lsp automated verification...");
                let mut server = LspServer::new();
                let init_req = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
                let resp = server.handle_message(init_req);
                assert!(resp.is_some(), "LSP initialize failed");
                println!("  ✓ LSP initialization handshake: PASS");

                let uri = "file:///sample.mantra";
                let valid_src = "def main() -> Int:\n    print(\"Hello LSP\")\n    return 0\n";
                let diags = server.open_document(uri.to_string(), valid_src.to_string());
                assert!(diags.is_empty(), "Valid source produced errors: {:?}", diags);
                println!("  ✓ Diagnostic analysis on valid source: PASS (0 errors)");

                let invalid_src = "def main() -> Int:\n    let x: Int = \"invalid\"\n    return 0\n";
                let err_diags = server.update_document(uri, invalid_src.to_string());
                assert!(!err_diags.is_empty(), "Type error was not detected");
                println!("  ✓ Diagnostic analysis on type mismatch: PASS ({} diagnostic detected)", err_diags.len());

                let comps = server.get_completions(uri, &mantra_lsp::LspPosition::new(0, 0));
                assert!(!comps.is_empty(), "Completions list is empty");
                println!("  ✓ Code completion suggestions: PASS ({} items available)", comps.len());

                let fmt_edits = server.format_document(uri);
                assert!(fmt_edits.is_some(), "Document formatting failed");
                println!("  ✓ Document formatting engine: PASS");

                println!("mantra-lsp self-test PASSED successfully!");
                return;
            }
            _ => {}
        }
    }

    // Default: run stdio loop
    let mut server = LspServer::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut lines = stdin.lock().lines();
    while let Some(Ok(line)) = lines.next() {
        if line.starts_with("Content-Length:") {
            // Read until empty line
            while let Some(Ok(header_line)) = lines.next() {
                if header_line.trim().is_empty() {
                    break;
                }
            }

            // Next line is payload or read length
            if let Some(Ok(body)) = lines.next() {
                if let Some(resp) = server.handle_message(&body) {
                    let full_resp = format!("Content-Length: {}\r\n\r\n{}", resp.len(), resp);
                    let _ = stdout.write_all(full_resp.as_bytes());
                    let _ = stdout.flush();
                }
            }
        } else if line.trim().starts_with('{') {
            // Direct JSON line mode (useful for testing and pipes)
            if let Some(resp) = server.handle_message(&line) {
                let _ = writeln!(stdout, "{}", resp);
                let _ = stdout.flush();
            }
        }
    }
}

