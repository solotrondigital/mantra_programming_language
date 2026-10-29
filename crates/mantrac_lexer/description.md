# mantrac_lexer — Lexical Analysis, Tokenization & Hardware Interaction

## 1. Package Overview & Purpose

`mantrac_lexer` is the foundational lexical scanner of the Mantra programming language. It performs the first phase of compilation: transforming raw UTF-8 source text bytes into a discrete stream of strongly-typed lexical tokens (`Token`).

### Core Responsibilities:
* **UTF-8 Character Stream Scanning**: Efficiently iterates over Unicode character streams with line, column, and byte-offset tracking.
* **Significant Indentation & Off-side Rule**: Converts Python-style whitespace indentation into explicit synthetic tokens (`TokenKind::Indent`, `TokenKind::Dedent`, `TokenKind::Newline`), using an internal indentation stack.
* **Lexical Classification**: Recognizes keywords (`def`, `class`, `let`, `mut`, `if`, `else`, `while`, `for`, `return`, `async`, `await`, etc.), identifier names, numeric literals (integers, floats, scientific notation), string literals (with escape sequences), operators (`+`, `-`, `*`, `/`, `==`, `!=`, `<=`, `>=`, `->`), and delimiters (`(`, `)`, `[`, `]`, `{`, `}`, `:`, `,`).
* **Source Tracking**: Attaches an exact source `Span` to every emitted token for downstream error reporting and compiler diagnostics.

---

## 2. Modules, Data Structures & Functions Used

### Key Modules:
* [`src/token.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantrac_lexer/src/token.rs): Enumerates all language token types (`TokenKind`) and defines `Token`.
* [`src/span.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantrac_lexer/src/span.rs): Encapsulates source coordinate tracking (`start`, `end`, `line`, `col`).
* [`src/lib.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantrac_lexer/src/lib.rs): Contains `Lexer` state machine, indentation tracker, and character scanning logic.

### Data Structures & Enums:
* `Span`: Struct storing `start: usize` (byte offset), `end: usize`, `line: usize` (1-indexed), and `col: usize` (1-indexed).
* `Token`: Atomic lexical unit storing `kind: TokenKind` and `span: Span`.
* `TokenKind`: Exhaustive enum of token variants:
  * Keywords: `Def`, `Class`, `Let`, `Mut`, `If`, `Else`, `While`, `For`, `In`, `Return`, `Import`, `Async`, `Await`, `Priv`, `Pub`, `Pass`, `True`, `False`, `NoneVal`, `Abstract`, `Static`.
  * Literals: `Identifier(String)`, `IntLiteral(i64)`, `FloatLiteral(f64)`, `StringLiteral(String)`.
  * Operators & Punctuation: `Plus`, `Minus`, `Star`, `Slash`, `Percent`, `EqualEqual`, `BangEqual`, `Less`, `Greater`, `LessEqual`, `GreaterEqual`, `Arrow` (`->`), `Colon`, `Comma`, `Dot`, `At` (`@`).
  * Layout: `Indent`, `Dedent`, `Newline`, `Eof`.
* `Lexer<'a>`: Scanner state storing:
  * `source: &'a str`: Borrowed slice of input source text.
  * `chars: Vec<(usize, char)>`: Pre-indexed Unicode character stream.
  * `cursor: usize`: Current position in the character buffer.
  * `line: usize`, `col: usize`: Running coordinate trackers.
  * `indent_stack: Vec<usize>`: Stack of indentation column depths.
  * `paren_depth: usize`: Tracks open parentheses to suppress newlines inside multiline expressions.
  * `pending_tokens: Vec<Token>`: Queue holding synthesized `Dedent` or layout tokens.
  * `at_line_start: bool`: State flag for measuring line indentation.

### Functions & Methods:
* `Lexer::new(source: &'a str) -> Self`: Initializes scanner with the input string and pushes root indentation level `0`.
* `Lexer::tokenize(&mut self) -> Result<Vec<Token>, LexerError>`: Runs the scan loop until EOF, emitting closing `Dedent` tokens for unclosed indentation blocks.
* `Lexer::next_token(&mut self) -> Result<Option<Token>, LexerError>`: Advances cursor and identifies the next single token.
* `Lexer::scan_number(&mut self, start_pos: usize) -> Result<Token, LexerError>`: Scans digit sequences, decimal points (`.`), and exponent markers (`e`/`E`), parsing into `IntLiteral` or `FloatLiteral`.
* `Lexer::scan_string(&mut self, quote: char, start_pos: usize) -> Result<Token, LexerError>`: Scans string bodies, decoding escape sequences (`\n`, `\t`, `\"`, `\\`).
* `Lexer::scan_identifier_or_keyword(&mut self, start_pos: usize) -> Token`: Scans alphanumeric sequences and performs keyword dictionary matching.

---

## 3. Communication with Low-Level Language Binaries

The lexer is the direct interface between raw binary text encodings and the compiler:

1. **UTF-8 Byte Stream Decoding**:
   - Modern programming languages represent source files as UTF-8 encoded binary files.
   - The lexer reads 8-bit byte streams directly from memory buffers loaded via OS file reads (`read(2)` / `mmap(2)`).
   - Validates UTF-8 multi-byte sequences (1 to 4 bytes per code point) using CPU bitwise masking.
2. **Binary Number Conversion**:
   - Numeric text literals like `"42"` or `"-100"` are parsed into native binary two's-complement 64-bit integers (`i64`), ready for direct insertion into native machine instruction immediate fields.
   - Floating-point text literals like `"3.14159"` are parsed directly into standard 64-bit IEEE 754 double-precision binary representations.

---

## 4. Hardware Interaction: CPU, ALU, and MU (Memory Unit)

The lexer processes millions of bytes per second, relying on tight CPU loops and hardware cache performance:

### Central Processing Unit (CPU)
* **Tight Inner Loop Execution**:
  - `Lexer::next_token` executes a tight character inspection loop. The CPU instruction cache (L1I) holds the entire decoding loop without cache evictions.
* **Branch Prediction in Lexical Matching**:
  - The switch statement over character classes (`'a'..='z'`, `'0'..='9'`, whitespace, punctuation) translates to CPU jump tables.
  - Predictors quickly learn common code distributions (e.g., lowercase letters occurring in runs for identifiers), achieving high instruction pipeline throughput.

### Arithmetic Logic Unit (ALU)
* **Numeric Parsing Arithmetic**:
  - In `scan_number`, ASCII character conversion subtracts the byte value `'0'` (`0x30`):
    $$\text{digit} = \text{byte} - \text{0x30}$$
  - Accumulation performs integer multiply-and-add operations in the ALU:
    $$\text{accumulator} = (\text{accumulator} \times 10) + \text{digit}$$
  - Implemented on x86 using `LEA` (Load Effective Address) and `IMUL` hardware instructions.
* **Indentation Level Calculations**:
  - Comparing current indentation depth with `indent_stack.last()` performs ALU integer subtraction (`SUB`) and flag checks (`CMP`), triggering multiple `Dedent` token generations when unwinding nested blocks.

### Memory Unit (MU) / Memory Management Unit (MMU)
* **Sequential Streaming Memory Access**:
  - The lexer reads source text sequentially from start to finish.
  - This sequential pattern triggers the CPU's **hardware stream prefetcher**, which continuously loads subsequent 64-byte memory blocks from DRAM into L3, L2, and L1 Data Caches before the lexer requests them.
* **Cache Line Alignment**:
  - The `Token` struct is compact (32 bytes on 64-bit systems), meaning exactly two tokens fit into a standard 64-byte CPU cache line. This enables the parser to traverse tokens with minimal memory bandwidth consumption.
