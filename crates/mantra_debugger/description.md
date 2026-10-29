# mantra_debugger — Interactive Source Debugger & Hardware Inspection

## 1. Package Overview & Purpose

`mantra_debugger` is the interactive source-level debugging subsystem for the Mantra programming language. Introduced in Mantra v0.2.0, it allows developers to debug Mantra programs with a familiar GDB/LLDB-style interactive REPL terminal without needing external debugging tools.

### Core Responsibilities:
* **Interactive Debugging REPL**: Terminal interface supporting `run`, `step`, `next`, `continue`, `break`, `delete`, `info breakpoints`, `list`, `backtrace`, `print`, and `quit`.
* **Breakpoint Management**: Dynamic setting, verification, and clearing of line-based source breakpoints.
* **Source Context Rendering**: Visualizes source code with current execution pointers (`->`) and active breakpoint markers (`[*]`).
* **Runtime Error Interception**: Traps runtime errors and immediately exposes the failing line context and call backtrace.

---

## 2. Modules, Data Structures & Functions Used

### Key Structure:
* [`src/lib.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_debugger/src/lib.rs): Complete implementation of `DebugSession` and debugging commands.

### Data Structures & Enums:
* `DebugSession`:
  * `file_path: String`: Path to active source file being debugged.
  * `source_lines: Vec<String>`: Pre-split lines of source code for context printing.
  * `breakpoints: HashSet<usize>`: Set of 1-indexed line numbers with active breakpoints.
  * `current_line: usize`: Active execution line position.

### Functions & Methods:
* `DebugSession::new(file_path: String, source: &str) -> Self`: Initializes the debug session with parsed source lines.
* `DebugSession::add_breakpoint(&mut self, line: usize) -> bool`: Inserts a breakpoint after verifying line number bounds.
* `DebugSession::remove_breakpoint(&mut self, line: usize) -> bool`: Deletes an active breakpoint.
* `DebugSession::print_source_context(&self, line: usize, window: usize)`: Displays a window of source lines centered around the current line with status markers.
* `DebugSession::start_repl(&mut self, program: &Program) -> Result<(), String>`: Launches the interactive debugging loop over `stdin`/`stdout`, intercepting commands and delegating execution to `mantrac_vm::Interpreter`.

---

## 3. Communication with Low-Level Language Binaries

1. **Mapping Machine State to Source Constructs**:
   - In low-level native binaries, CPU instruction pointers (`RIP`/`PC`) point to raw memory addresses. `mantra_debugger` provides the high-level semantic model that translates these addresses back to source code lines and variables.
2. **Standard Terminal I/O Streams**:
   - Interacts with the host OS through standard file descriptors (`STDIN_FILENO = 0`, `STDOUT_FILENO = 1`).
   - Uses line-buffered terminal reads and explicit buffer flushes (`io::stdout().flush()`).

---

## 4. Hardware Interaction: CPU, ALU, and MU (Memory Unit)

### Central Processing Unit (CPU)
* **Single-Step Execution Emulation**:
  - In hardware debugging (e.g., GDB/LLDB with x86), single-stepping sets the **Trap Flag (TF)** in the CPU `EFLAGS` register, triggering a CPU hardware interrupt vector (`INT 1`) after every instruction.
  - `mantra_debugger` implements virtual single-stepping by intercepting statement boundaries inside the VM interpreter loop.
* **Control Flow Trapping**:
  - Breakpoints emulate software interrupt instructions (`INT 3`, opcode `0xCC` on x86, `BRK #0` on ARM64) by checking line numbers against `HashSet<usize>` prior to statement evaluation.

### Arithmetic Logic Unit (ALU)
* **Window Offset Boundary Math**:
  - `print_source_context` performs ALU boundary arithmetic:
    $$start = \max(1, line - window)$$
    $$end = \min(total\_lines, line + window)$$
  - Executed using `SUB` and conditional move (`CMOV`) instructions.

### Memory Unit (MU) / Memory Management Unit (MMU)
* **Call Stack Frame Walking**:
  - The `backtrace` command inspects the virtual call stack, mirroring the behavior of native debuggers walking the chain of saved base pointers (`RBP`) on the CPU hardware stack.
* **Memory Inspection**:
  - The `print <var>` command queries the memory values associated with variable names, inspecting either stack-allocated primitive values or heap-allocated object instance references.
