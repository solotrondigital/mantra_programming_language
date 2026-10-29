# mantrac — Compiler Driver CLI & Hardware Execution Pipeline

## 1. Package Overview & Purpose

`mantrac` is the core compiler driver binary executable for the Mantra programming language. It serves as the top-level orchestrator uniting the lexer, parser, type-checker, intermediate representation (IR), JIT engine, WebAssembly emitter, native C99 codegen, interactive debugger, self-hosting bootstrap pipeline, and the VM interpreter.

### Core Execution Modes:
* **Ahead-Of-Time Native Compilation (`mantrac <input.mantra> -o <binary>`)**: Drives the full compilation pipeline into native ELF, Mach-O, or PE executables.
* **Just-In-Time Execution (`--jit` / `--run`)**: Lowers to typed IR and compiles directly into executable RAM using Cranelift for near-instant native speed, with automated fallback to the interpreter on unsupported features.
* **WebAssembly Target Compilation (`--emit wasm`)**: Emits standalone `.wasm` binaries for browser and WASI environments.
* **Interpreter Execution (`--interpret`)**: Direct AST tree-walk execution for testing and recovery.
* **Interactive Debugging (`--debug`)**: Attaches `mantra_debugger` for line-by-line stepping, breakpoints, and runtime inspection.
* **Self-Hosting Bootstrap Pipeline (`--bootstrap`)**: Verifies 3-stage self-compilation reproducibility.
* **Canonical Language Regression Suite (`--regression`)**: Runs complete conformance suite against language specifications.

---

## 2. Modules, Data Structures & Functions Used

### Key Structure:
* [`src/main.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantrac/src/main.rs): Complete CLI driver, argument processor, pipeline sequencer, and error reporter.

### Integrated Subsystems:
* `mantrac_lexer::Lexer`: Lexical scanning and token stream generation.
* `mantrac_parser::Parser`: Grammar validation and AST synthesis.
* `mantrac_checker::TypeChecker`: Static semantics, type safety, and symbol table generation.
* `mantrac_ir::AstLowering`: AST-to-IR transformation.
* `mantrac_ir::JitEngine`: Memory machine code generation and dynamic invocation.
* `mantrac_ir::WasmEmitter`: Wasm binary serialization.
* `mantrac_codegen::{CEmitter, NativeCompiler}`: Native C99 emission and platform binary compilation.
* `mantrac_vm::Interpreter`: Canonical tree-walking reference engine.
* `mantra_bootstrap::{BootstrapPipeline, RegressionSuite, CompilerVersion}`: Conformance and self-hosting bootstrap controller.
* `mantra_debugger::DebugSession`: Interactive terminal REPL debugging.

### Primary Functions:
* `main()`: Command-line parsing and operational mode dispatch.
* Pipeline sequencing:
  1. `fs::read_to_string`: Ingests source file from disk into RAM.
  2. `Lexer::new(&source).tokenize()`: Generates tokens.
  3. `Parser::new(tokens).parse()`: Produces `Program` AST.
  4. `TypeChecker::new().check_program(&ast)`: Validates types.
  5. Target execution / emission based on flags (`--jit`, `--emit wasm`, `-o`, `--interpret`).

---

## 3. Communication with Low-Level Language Binaries

`mantrac` is the central nexus controlling native binary generation and execution:

1. **Native Executable Binary Output**:
   - Compiles native executables conforming to the host operating system's binary standards (`Mach-O` on macOS, `ELF` on Linux, `PE` on Windows).
2. **Object Code Linking & Library Resolution**:
   - Drives system linkers to resolve external native dependencies (`libm`, `libc`, `libpthread`).
3. **Execution of JIT Memory Code**:
   - Allocates executable memory blocks via OS page primitives, writes generated machine instructions, and directly executes function pointers at runtime.

---

## 4. Hardware Interaction: CPU, ALU, and MU (Memory Unit)

### Central Processing Unit (CPU)
* **Pipeline Scheduling & Profiling**:
  - `--profile` queries high-resolution CPU performance counters via `std::time::Instant`, measuring precise cycle distributions across lexing, parsing, checking, and codegen.
* **Instruction Execution Target Selection**:
  - Targets native CPU instruction set architectures: x86_64 (with AVX2 / SSE4.2 extensions) and AArch64 (ARMv8 / ARMv9 with NEON extensions).

### Arithmetic Logic Unit (ALU)
* **Instruction Opcode Selection**:
  - Configures backend code generators to emit optimal ALU instructions for target hardware (e.g., using hardware 3-operand instructions on ARM64 or single-cycle bit manipulation instructions).
* **Return Code Evaluation**:
  - Compares binary execution exit status codes via ALU condition flag checks.

### Memory Unit (MU) / Memory Management Unit (MMU)
* **Virtual Address Space Management**:
  - Coordinates process memory layout across text (`.text`), read-only data (`.rodata`), data (`.data`), BSS (`.bss`), stack, and heap segments.
* **Executable Page Permissions**:
  - Interacts with the kernel MMU to configure memory protection bits (`W^X` enforcement), ensuring generated native binaries execute safely and securely.
