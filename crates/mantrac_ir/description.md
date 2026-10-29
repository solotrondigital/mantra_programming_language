# mantrac_ir — Intermediate Representation, JIT Engine, and WASM Backend

## 1. Package Overview & Purpose

`mantrac_ir` is the intermediate compilation and execution engine of the Mantra language. It bridges the high-level Abstract Syntax Tree (`mantrac_ast`) and low-level target machine code by providing:
1. **A Strongly-Typed Three-Address Intermediate Representation (IR)**: Decouples the frontend parser and type checker from specific backend architectures.
2. **Cranelift-Powered Just-In-Time (JIT) Execution Engine**: Compiles Mantra IR functions directly into native machine instructions in memory for immediate, near-native execution speed.
3. **WebAssembly (WASM) Binary Emitter**: Serializes Mantra IR into standard Wasm bytecode files (`.wasm`) for browser execution and WASI runtimes.
4. **AST-to-IR Lowering Engine**: Flattens nested expressions, control structures (`if`, `while`), function calls, and async coroutines into linear, basic-block IR instructions.

---

## 2. Modules, Data Structures & Functions Used

### Key Modules:
* [`src/ir.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantrac_ir/src/ir.rs): Defines types, values, binary operations, and instruction sets.
* [`src/lower.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantrac_ir/src/lower.rs): Transforms `mantrac_ast::Program` into `IrProgram`.
* [`src/jit.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantrac_ir/src/jit.rs): Compiles IR instructions into native machine code pages and manages execution.
* [`src/wasm.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantrac_ir/src/wasm.rs): Generates WebAssembly binary format (magic bytes, type section, function section, code section).
* [`src/diagnostics.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantrac_ir/src/diagnostics.rs): Tracks execution time and JIT compilation overhead.

### Data Structures & Enums:
* `IrType`: Supported primitive data types: `Int` (i64), `Float` (f64), `Bool` (bool), `String` (str/ptr), `Void`, `Custom(String)`.
* `IrValue`: Typed constants: `Int(i64)`, `Float(f64)`, `Bool(bool)`, `String(String)`, `Var(String)`, `Null`.
* `IrBinaryOp`: Operations: `Add`, `Sub`, `Mul`, `Div`, `Mod`, `Eq`, `NotEq`, `Lt`, `Gt`, `LtEq`, `GtEq`.
* `IrInstruction`:
  * `Const { dest, val }`: Assigns immediate value to virtual register.
  * `Copy { dest, src }`: Register-to-register move.
  * `Binary { op, dest, left, right }`: Two-operand arithmetic/logic computation.
  * `Call { dest, func, args }` / `CallAsync`: Direct and asynchronous function calls.
  * `Await { dest, task }`: Coroutine suspension/resumption point.
  * `Jump { target }` / `JumpIf { cond, then_target, else_target }`: Unconditional and conditional branches.
  * `Return { val }`: Function return.
* `IrFunction`: Name, parameter list, return type, local variable definitions, and instruction sequence.
* `IrProgram`: Collection of compiled functions and type metadata.
* `JitEngine`: Cranelift JIT compilation context, module manager, and code generator.

### Functions & Methods:
* `AstLowering::lower_program(program: &Program) -> IrProgram`: Traverses the AST and decomposes statements into three-address code with generated temporary registers (`t0`, `t1`, ...).
* `JitEngine::new() -> Result<Self, String>`: Configures native target machine architecture, ISA flags (SSE4.2, AVX2, NEON), and initializes Cranelift JIT module.
* `JitEngine::compile_function(&mut self, func: &IrFunction) -> Result<*const u8, String>`: Translates IR instructions into native machine code, links external symbols, finalizes function bytes into executable memory, and returns a function pointer.
* `JitEngine::execute_main(&mut self, program: &IrProgram) -> Result<i64, String>`: Compiles all functions in the program and calls the native entry point `main()`.
* `WasmEmitter::emit_program(program: &IrProgram) -> Vec<u8>`: Encodes WASM binary header (`0x00 0x61 0x73 0x6D`) and encodes instruction opcodes (`i64.add`, `i64.sub`, `call`, `br_if`).

---

## 3. Communication with Low-Level Language Binaries

`mantrac_ir` is the heart of low-level binary interaction:

1. **Direct Machine Code Generation in Memory**:
   - `JitEngine` emits real CPU machine code (opcodes such as `0x48 0x89 0xE5` for `mov rbp, rsp` on x86_64, or `0xD10043FF` for `sub sp, sp, #16` on ARM64).
   - Allocates memory pages using OS primitives (`mmap` on POSIX, `VirtualAlloc` on Windows).
   - Transitions memory page protections from `PROT_READ | PROT_WRITE` (during code generation) to `PROT_READ | PROT_EXEC` (during execution) using `mprotect(2)` / `VirtualProtect()`.
2. **C ABI Interoperability (System V / Microsoft x64)**:
   - Compiled JIT functions conform to standard native calling conventions:
     - **x86_64 System V (Linux/macOS)**: Arguments in `RDI`, `RSI`, `RDX`, `RCX`, `R8`, `R9`. Return value in `RAX`.
     - **Windows x64**: Arguments in `RCX`, `RDX`, `R8`, `R9`. Shadow space (32 bytes) allocated on caller stack. Return value in `RAX`.
     - **ARM64 (AAPCS64)**: Arguments in `X0`-`X7`. Return value in `X0`.
   - Native C functions (e.g., standard C library `printf`, `malloc`, `write`) can be directly registered and invoked by JIT code via function pointers.
3. **WebAssembly Binary Output**:
   - Emits byte streams conforming to the W3C WebAssembly Core Specification 1.0/2.0. These binaries run in standalone WASI runtimes (Wasmtime, Wasmer) or browser JavaScript engines (V8, JavaScriptCore, SpiderMonkey).

---

## 4. Hardware Interaction: CPU, ALU, and MU (Memory Unit)

`mantrac_ir` operates at the closest layer to the physical silicon:

### Central Processing Unit (CPU)
* **Instruction Execution Pipeline**:
  - JIT-compiled instructions are fetched by the CPU's Instruction Fetch Unit into the L1 Instruction Cache (L1I).
  - Decoded by hardware decoders into internal micro-operations ($\mu$ops).
  - Out-of-Order (OoO) execution engines in x86/ARM cores schedule independent IR operations concurrently across multiple execution ports.
* **Instruction Pointer & Branch Target Prediction**:
  - `IrInstruction::Jump` maps directly to unconditional jump instructions (`JMP rel32` on x86, `B label` on ARM).
  - `IrInstruction::JumpIf` maps to conditional branches (`JNE`, `JE`, `B.EQ`, `B.NE`). The CPU Branch Target Buffer (BTB) and two-level adaptive branch predictors track loop and condition branches.
* **Hardware Register Allocation**:
  - Cranelift's register allocator (`regalloc2`) maps virtual IR registers (`t0`, `t1`, etc.) to physical hardware registers, eliminating unnecessary memory spills:
    - High-frequency local variables reside in `RBX`, `R12`-`R15` (callee-saved) or `R10`-`R11` (caller-saved).

### Arithmetic Logic Unit (ALU)
* **Direct ALU Instruction Mapping**:
  - Every `IrBinaryOp` maps to dedicated hardware ALU execution units:
    * `IrBinaryOp::Add` $\rightarrow$ `ADD dst, src` (single-cycle integer adder).
    * `IrBinaryOp::Sub` $\rightarrow$ `SUB dst, src` (two's complement subtractor).
    * `IrBinaryOp::Mul` $\rightarrow$ `IMUL dst, src` (pipelined integer multiplier unit, typically 3 cycles latency).
    * `IrBinaryOp::Div` $\rightarrow$ `IDIV src` (hardware integer division unit, non-pipelined, 10–25 cycles).
    * `IrBinaryOp::Mod` $\rightarrow$ `IDIV src` (extracts remainder from `RDX` register on x86).
* **Hardware Status Flags**:
  - Comparisons (`Eq`, `Lt`, `Gt`) execute `CMP left, right` (which internally performs ALU subtraction without storing the result) and evaluate hardware status flags:
    * Zero Flag (`ZF`): Set if `left == right`.
    * Sign Flag (`SF`) & Overflow Flag (`OF`): Checked via `JL` (`SF != OF`) for signed less-than.
    * Carry Flag (`CF`): Used for unsigned arithmetic and multiprecision carry chains.
* **Floating-Point Unit (FPU / SIMD)**:
  - `Float` operations (`f64`) utilize SSE2/AVX instructions on x86 (`ADDSD`, `SUBSD`, `MULSD`, `DIVSD`) and NEON scalar registers (`FADD d0, d0, d1`) on ARM, executing on dedicated floating-point ALUs.

### Memory Unit (MU) / Memory Management Unit (MMU)
* **Stack Memory & Activation Records**:
  - When a JIT-compiled function is called:
    1. Caller pushes return address onto stack (`RSP` decrements by 8).
    2. Prologue executes: `push rbp; mov rbp, rsp; sub rsp, <frame_size>`.
    3. Memory offsets `[rbp - 8]`, `[rbp - 16]` hold spilled local variables.
    4. Epilogue executes: `mov rsp, rbp; pop rbp; ret`.
* **Hardware L1 / L2 / L3 Data Cache Behavior**:
  - Consecutive struct and array access operates on 64-byte aligned cache lines.
  - Hardware prefetchers detect linear memory access strides, preloading adjacent memory pages from main DRAM into L3 and L2 caches ahead of CPU execution.
* **Memory Protection & MMU Page Tables**:
  - The OS MMU enforces page-level execution permissions. Generating code in writable pages and executing without switching permissions causes hardware **DEP/NX (Data Execution Prevention / No-Execute)** page faults (`EXC_BAD_ACCESS` / `SIGSEGV`). `mantrac_ir` coordinates with the OS MMU to safely manage page state.
