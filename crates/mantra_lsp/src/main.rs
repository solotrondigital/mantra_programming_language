// ==============================================================================
// mantra-lsp — Mantra Language Server Protocol Daemon (stdio)
// Speaks LSP 3.17 over stdin/stdout using HTTP-framed JSON-RPC 2.0.
// ==============================================================================

use mantra_lsp::LspServer;
use std::env;
use std::io::{self, BufRead, Read, Write};

fn print_help() {
    eprintln!(
        "Mantra Language Server (mantra-lsp) v0.5.0
Usage:
    mantra-lsp [options]

Options:
    --stdio             Run LSP server over standard I/O (default)
    --test              Run self-verification suite
    -v, --version       Print version
    -h, --help          Show this help message

The server communicates via JSON-RPC 2.0 over stdin/stdout using
HTTP Content-Length framing as specified by LSP 3.17.
"
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
                println!("mantra-lsp 0.5.0");
                return;
            }
            "--test" => {
                run_self_test();
                return;
            }
            _ => {}
        }
    }

    // Default: run stdio loop
    run_stdio();
}

fn run_stdio() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut reader = io::BufReader::new(stdin.lock());

    let mut server = LspServer::new();

    loop {
        // ── Read headers ──────────────────────────────────────────────────
        let mut content_length: Option<usize> = None;
        loop {
            let mut header_line = String::new();
            match reader.read_line(&mut header_line) {
                Ok(0) => {
                    // EOF
                    return;
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("[mantra-lsp] header read error: {}", e);
                    return;
                }
            }
            let trimmed = header_line.trim_end_matches(|c| c == '\r' || c == '\n');
            if trimmed.is_empty() {
                break; // End of headers
            }
            if let Some(rest) = trimmed.strip_prefix("Content-Length:") {
                if let Ok(n) = rest.trim().parse::<usize>() {
                    content_length = Some(n);
                }
            }
        }

        let length = match content_length {
            Some(n) => n,
            None => {
                eprintln!("[mantra-lsp] missing Content-Length, skipping");
                continue;
            }
        };

        // ── Read body ─────────────────────────────────────────────────────
        let mut body = vec![0u8; length];
        if let Err(e) = reader.read_exact(&mut body) {
            eprintln!("[mantra-lsp] body read error: {}", e);
            return;
        }
        let msg = match std::str::from_utf8(&body) {
            Ok(s) => s.to_string(),
            Err(e) => {
                eprintln!("[mantra-lsp] UTF-8 decode error: {}", e);
                continue;
            }
        };

        // ── Dispatch ──────────────────────────────────────────────────────
        let response = server.handle_message(&msg);

        // Flush any notifications first (e.g. publishDiagnostics)
        for notif in server.take_notifications() {
            write_message(&mut out, &notif);
        }

        // Then send the response
        if let Some(resp) = response {
            write_message(&mut out, &resp);
        }
    }
}

fn write_message<W: Write>(writer: &mut W, msg: &str) {
    let bytes = msg.as_bytes();
    let header = format!("Content-Length: {}\r\n\r\n", bytes.len());
    let _ = writer.write_all(header.as_bytes());
    let _ = writer.write_all(bytes);
    let _ = writer.flush();
}

fn run_self_test() {
    eprintln!("Running mantra-lsp self-test...");
    let mut server = LspServer::new();

    // 1. Initialize
    let init_req = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"rootUri":null}}"#;
    let resp = server.handle_message(init_req).expect("LSP initialize failed");
    assert!(resp.contains("\"capabilities\""), "initialize must return capabilities");
    assert!(resp.contains("completionProvider"), "must advertise completionProvider");
    assert!(resp.contains("hoverProvider"), "must advertise hoverProvider");
    assert!(resp.contains("definitionProvider"), "must advertise definitionProvider");
    assert!(resp.contains("renameProvider"), "must advertise renameProvider");
    assert!(resp.contains("semanticTokensProvider"), "must advertise semanticTokensProvider");
    eprintln!("  ✓ LSP initialization handshake: PASS");

    // 2. Open valid document
    let uri = "file:///sample.mantra";
    let valid_src = "def main() -> Int:\n    print(\"Hello LSP\")\n    return 0\n";
    let diags = server.open_document(uri.to_string(), valid_src.to_string());
    assert!(diags.is_empty(), "Valid source produced errors: {:?}", diags);
    eprintln!("  ✓ Diagnostic analysis on valid source: PASS (0 errors)");

    // 3. Type error detection
    let invalid_src = "def main() -> Int:\n    let x: Int = \"invalid\"\n    return 0\n";
    let err_diags = server.update_document(uri, invalid_src.to_string());
    assert!(!err_diags.is_empty(), "Type error was not detected");
    eprintln!("  ✓ Type mismatch diagnostic: PASS ({} error(s))", err_diags.len());

    // 4. Completions
    let comps = server.get_completions(uri, &mantra_lsp::LspPosition::new(0, 0));
    assert!(!comps.is_empty(), "Completions list is empty");
    assert!(comps.iter().any(|i| i.label == "def"), "Missing keyword 'def'");
    assert!(comps.iter().any(|i| i.label == "class"), "Missing keyword 'class'");
    assert!(comps.iter().any(|i| i.label == "print"), "Missing builtin 'print'");
    eprintln!("  ✓ Code completion: PASS ({} items)", comps.len());

    // 5. Hover
    let hover = server.get_hover(uri, &mantra_lsp::LspPosition::new(0, 1));
    assert!(hover.is_some(), "Hover on 'def' returned None");
    eprintln!("  ✓ Hover documentation: PASS");

    // 6. Formatting
    server.open_document("file:///fmt.mantra".to_string(),
        "def main() -> Int:\n  print(1)  \n".to_string());
    let fmt_edits = server.format_document("file:///fmt.mantra");
    assert!(fmt_edits.is_some(), "Document formatting failed");
    eprintln!("  ✓ Document formatting: PASS");

    // 7. Document symbols via JSON-RPC
    let sym_msg = format!(
        r#"{{"jsonrpc":"2.0","id":2,"method":"textDocument/documentSymbol","params":{{"textDocument":{{"uri":"{}"}}}}}}"#,
        uri
    );
    let sym_resp = server.handle_message(&sym_msg).unwrap();
    assert!(sym_resp.contains("\"result\""), "documentSymbol must return result");
    eprintln!("  ✓ Document symbols: PASS");

    // 8. Semantic tokens
    let sem_msg = format!(
        r#"{{"jsonrpc":"2.0","id":3,"method":"textDocument/semanticTokens/full","params":{{"textDocument":{{"uri":"{}"}}}}}}"#,
        uri
    );
    let sem_resp = server.handle_message(&sem_msg).unwrap();
    assert!(sem_resp.contains("\"data\""), "Semantic tokens must contain data");
    eprintln!("  ✓ Semantic tokens: PASS");

    eprintln!("\nmantra-lsp self-test PASSED ✓");
}
