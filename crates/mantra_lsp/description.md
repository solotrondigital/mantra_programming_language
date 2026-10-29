# mantra_lsp — Language Server Protocol Specification & Hardware Interaction

## 1. Package Overview & Purpose

`mantra_lsp` is the official Language Server Protocol (LSP 3.17 compatible) implementation for the Mantra programming language. It acts as the intelligent bridge between developer IDEs/editors (Visual Studio Code, Neovim, Emacs, Helix, Zed) and the internal Mantra compiler toolchain (`mantrac_lexer`, `mantrac_parser`, `mantrac_checker`, `mantra_mdk::formatter`).

### Core Purpose:
* **Real-time Diagnostic Feedback**: Continuously parses and type-checks code as developers type, generating syntax and semantic diagnostics without emitting intermediate disk artifacts.
* **Intelligent Code Completion**: Computes context-aware auto-completion items (22 language keywords, 11 standard library built-ins, and local symbol definitions).
* **Symbol Hover Inspection**: Queries type environments and docstrings to provide inline signatures and markdown hover cards.
* **Deterministic Source Formatting**: Exposes formatting actions over the standardized LSP RPC protocol.
* **Lightweight Zero-Dependency Footprint**: Implemented purely using the Rust standard library (`std::io`, `std::collections`) without heavyweight asynchronous runtime crates like `tokio` or `tower-lsp`, guaranteeing sub-millisecond cold starts and minimal RAM overhead.

---

## 2. Modules, Data Structures & Functions Used

### Key Modules:
* [`src/protocol.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_lsp/src/protocol.rs): Defines JSON-RPC 2.0 wire models and standard LSP data structures.
* [`src/server.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_lsp/src/server.rs): Core language server state machine, document cache, and query evaluator.
* [`src/main.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_lsp/src/main.rs): CLI executable wrapper supporting standard I/O streaming (`--stdio`) and self-test verification (`--test`).

### Data Structures & Enums:
* `LspPosition`: 0-indexed line and character offset (`line: usize`, `character: usize`).
* `LspRange`: Start and end `LspPosition` defining a text selection or error span.
* `LspDiagnostic`: Structured error/warning payload containing `range`, `severity` (1=Error, 2=Warning, 3=Information, 4=Hint), `code`, `source`, and `message`.
* `LspCompletionItem`: Suggested text payload with label, `kind` (Keyword=14, Function=3, Class=7, Variable=6), and documentation.
* `LspHover`: Markdown hover response detailing symbol types and doc comments.
* `LspTextEdit`: Atomic file replacement edit with `range` and `new_text`.
* `LspServer`: Central state holder maintaining `documents: HashMap<String, String>` and `initialized: bool`.

### Functions & Methods:
* `LspServer::new() -> Self`: Initializes the server instance with an empty document cache.
* `LspServer::open_document(&mut self, uri: String, text: String)`: Ingests `textDocument/didOpen` and caches file buffers in RAM.
* `LspServer::update_document(&mut self, uri: String, text: String)`: Handles `textDocument/didChange` events for incremental workspace edits.
* `LspServer::compute_diagnostics(&self, source: &str) -> Vec<LspDiagnostic>`:
  1. Spawns `mantrac_lexer::Lexer` to tokenize source text. Captures tokenization errors into diagnostic spans.
  2. Passes token stream to `mantrac_parser::Parser`. Catches syntax tree malformations and unclosed delimiters.
  3. Executes `mantrac_checker::TypeChecker` over the AST to extract symbol tables and type mismatch diagnostics.
* `LspServer::get_completions(&self, uri: &str, pos: &LspPosition) -> Vec<LspCompletionItem>`: Assembles keyword lists, built-in definitions, and parses document identifiers up to cursor position.
* `LspServer::get_hover(&self, uri: &str, pos: &LspPosition) -> Option<LspHover>`: Resolves symbol under cursor and renders type signature.
* `LspServer::format_document(&self, uri: &str) -> Option<Vec<LspTextEdit>>`: Invokes `mantra_mdk::formatter::Formatter` to compute whole-document formatting edits.
* `LspServer::handle_message(&mut self, line: &str) -> Option<String>`: Dispatches incoming JSON-RPC requests (`initialize`, `textDocument/completion`, etc.) and serializes JSON responses.
* `LspServer::run_stdio(&mut self)`: Infinite loop processing framed messages from standard input (`stdin`) and flushing to `stdout`.

---

## 3. Communication with Low-Level Language Binaries

While `mantra_lsp` runs as a high-level language server daemon, it communicates with native binaries and system layers through:

1. **Host OS Binary Dispatch (`mantra lsp`)**:
   - The `mantra` native CLI binary (`target/release/mantra`) searches for `mantra-lsp` executable in adjacent binary directories (`std::env::current_exe()`), workspace targets, or system `$PATH`.
   - It invokes the `mantra-lsp` binary via standard OS process creation (`fork()` + `execve()` on Unix; `CreateProcessW()` on Windows).
2. **Binary IPC Pipe Communication**:
   - Communication with IDEs (VS Code, Neovim) takes place across OS-managed anonymous unidirectional pipes (`stdin`/`stdout`).
   - The binary reads byte streams framed by HTTP-style headers:
     ```http
     Content-Length: 142\r\n
     \r\n
     {"jsonrpc":"2.0","method":"textDocument/completion",...}
     ```
   - Buffering uses POSIX `read(2)` / `write(2)` system calls via `std::io::BufReader` and `std::io::BufWriter`, avoiding context switch churn.
3. **Compiler Subsystem Linkage**:
   - Statically links compiled object archives (`.rlib`/`.a`) of `mantrac_lexer`, `mantrac_parser`, and `mantrac_checker`. Symbol resolution happens at native static link-time via `ld64` / `lld`.

---

## 4. Hardware Interaction: CPU, ALU, and MU (Memory Unit)

Even though `mantra_lsp` handles syntax and analysis, its execution maps directly onto physical microarchitectural units:

### Central Processing Unit (CPU)
* **Instruction Cycle**: The core loop in `run_stdio` performs continuous `Fetch -> Decode -> Execute` cycles over byte streams. The JSON-RPC dispatch loop utilizes lookup tables and jump tables generated by rustc for pattern matching on string methods.
* **Branch Prediction & Pipeline**:
  - Token scanning in `compute_diagnostics` checks characters in tight loops. Predictable character branches (e.g., whitespace skips) achieve near-100% branch target buffer (BTB) prediction accuracy in modern CPU branch predictors.
  - Unexpected syntax errors constitute cold branches, causing occasional branch misprediction pipeline flushes (12–18 CPU cycles on modern x86/ARM pipelines).
* **Hardware Registers**:
  - Register allocation by LLVM places cursor pointers, line indexes, and token references into general-purpose registers:
    - **x86_64**: `RSI`/`RDI` for string slice memory pointers, `RCX`/`RDX` for slice length and buffer counters, `RAX` for function return codes.
    - **AArch64 (Apple Silicon / ARM64)**: `X0`-`X7` for procedure call arguments and results, `X19`-`X28` for callee-saved document buffers and token counts.

### Arithmetic Logic Unit (ALU)
* **Position & Offset Calculations**:
  - The ALU performs unsigned 64-bit integer addition (`ADD`) and subtraction (`SUB`) when tracking UTF-8 byte offsets, converting line numbers, and comparing cursor coordinates (`pos.line == current_line`).
* **Bitwise Operations in Hashing & String Matching**:
  - `HashMap<String, String>` lookups for document URIs invoke SipHash-1-3 / AHash operations inside the ALU:
    - Rotate operations (`ROL`/`ROR`), exclusive-OR (`XOR`), and 64-bit integer multiplication (`IMUL`) compute bucket indexes.
* **Flag Registers**:
  - Comparisons (`CMP cursor, len`) set the Zero Flag (`ZF`), Carry Flag (`CF`), and Sign Flag (`SF`), which trigger conditional jumps (`JE`, `JNE`, `JAE`, `JB`) during text buffer parsing.

### Memory Unit (MU) / Memory Management Unit (MMU)
* **Stack Memory & Call Frames**:
  - Each recursive descent parsing call triggered by `compute_diagnostics` pushes a new stack frame onto the thread's stack. The Stack Pointer (`RSP` on x86, `SP` on ARM) decrements by the size of the frame (local variables, return address).
  - Deeply nested expressions allocate proportional stack frames; bounded parser recursion limits prevent stack overflow page faults.
* **Heap Allocation & Virtual Memory Pages**:
  - Document text stored in `documents: HashMap<String, String>` resides in heap memory managed by the system allocator (`jemalloc` or system `malloc`).
  - Memory requests invoke `mmap(2)` / `brk(2)` system calls when heap arenas grow, requesting 4 KiB virtual memory pages from the OS MMU.
* **CPU Cache Hierarchy (L1 / L2 / L3)**:
  - Token vectors and AST nodes maintain contiguous memory layouts (`Vec<Token>`), ensuring high spatial locality.
  - When the lexer iterates over source code, 64-byte hardware cache lines are prefetched into L1 Data Cache (L1D, ~1 ns latency), minimizing high-latency main RAM access cycles (~60–80 ns).
* **Translation Lookaside Buffer (TLB)**:
  - Document text buffers accessed repeatedly during typing benefit from warm TLB translation cache entries, preventing page table walk stalls.
