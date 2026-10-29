# mantrac_codegen — Native Code Generator, C99 Emitter & Hardware Interaction

## 1. Package Overview & Purpose

`mantrac_codegen` is the native ahead-of-time (AOT) code generation backend for the Mantra compiler. It transforms verified Mantra Abstract Syntax Trees (`mantrac_ast::Program`) into standard ISO C99 source code, then invokes platform-native C compilers (Clang, GCC, MSVC) to produce standalone, highly-optimized machine executables (`ELF` on Linux, `Mach-O` on macOS, `PE/COFF` on Windows).

### Core Responsibilities:
* **AST to C99 Transpilation**: Generates clean, type-safe C source code preserving Mantra's lexical scoping, class definitions, method dispatch, and memory layout.
* **Native Toolchain Interfacing**: Detects and drives system C compilers (`clang`, `gcc`, `cl.exe`, `cc`) with architecture-specific optimization flags (`-O2`, `-std=c99`, `-lm`).
* **Direct Binary Production**: Generates native machine binaries that execute directly on bare-metal hardware without requiring any virtual machine or bytecode interpreter.

---

## 2. Modules, Data Structures & Functions Used

### Key Modules:
* [`src/c_emitter.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantrac_codegen/src/c_emitter.rs): Emits C99 source text from AST nodes and symbol tables.
* [`src/compiler.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantrac_codegen/src/compiler.rs): Discovers and invokes the platform C compiler to assemble and link native binaries.
* [`src/lib.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantrac_codegen/src/lib.rs): Public API interface.

### Data Structures & Enums:
* `CEmitter<'a>`: Transpiler state struct storing:
  * `symbols: &'a GlobalSymbolTable`: Read-only reference to the verified type-checker symbol table.
  * `code: String`: Accumulated C99 code buffer.
  * `indent_level: usize`: Code indentation formatting counter.
  * `var_types: HashMap<String, String>`: Tracks variable-to-C-type mappings (`int64_t`, `double`, `char*`, `bool`).
  * `declared_vars: HashSet<String>`: Prevents duplicate variable declarations within C scope blocks.
  * `current_class: Option<String>`: Tracks enclosing class context for method renaming and `self` pointer injection.
* `NativeCompiler`: Utility struct for orchestrating the native compilation and linking pipeline.

### Functions & Methods:
* `CEmitter::new(symbols: &'a GlobalSymbolTable) -> Self`: Constructs the emitter tied to the program's symbol table.
* `CEmitter::emit_program(&mut self, program: &Program) -> String`: Generates standard C headers (`#include <stdio.h>`, `#include <stdlib.h>`, `#include <stdint.h>`, `#include <stdbool.h>`, `#include <string.h>`, `#include <math.h>`), forward declarations, struct definitions for Mantra classes, runtime helper functions, and the `int main(int argc, char** argv)` entry point.
* `CEmitter::emit_statement(&mut self, stmt: &Statement)`: Emits variable definitions, conditionals (`if`/`else`), loops (`while`/`for`), return statements, and expression statements.
* `CEmitter::emit_expression(&mut self, expr: &Expression) -> String`: Emits binary arithmetic, logical operations, function calls, class instantiations (`malloc`), and property accesses.
* `NativeCompiler::compile_c_to_binary(c_code: &str, output_path: &Path, optimize: bool) -> Result<PathBuf, String>`:
  1. Writes C code to a unique temporary file (`/tmp/mantrac_build_<pid>.c`).
  2. Discovers host compiler (`clang`, `gcc`, `cl`, `cc`).
  3. Executes compiler with `-std=c99`, `-O2`, `-lm`, producing the final machine binary.
  4. Cleans up temporary artifacts.

---

## 3. Communication with Low-Level Language Binaries

`mantrac_codegen` operates as the primary producer of low-level native binaries in the Mantra toolchain:

1. **Native Executable Generation (`ELF` / `Mach-O` / `PE`)**:
   - The emitted C99 source is transformed by Clang/LLVM or GCC into platform-specific object files (`.o`), then linked into native executables.
   - Outputs include:
     - **Linux**: Executable and Linkable Format (`ELF 64-bit LSB executable, x86-64 / aarch64`).
     - **macOS**: Mach-O 64-bit dynamically linked executable.
     - **Windows**: Portable Executable (`PE32+ executable for MS Windows`).
2. **Standard C ABI Compliance**:
   - Functions are emitted with standard C linkage:
     ```c
     int64_t mantra_math_add(int64_t a, int64_t b);
     ```
   - This allows any Mantra library compiled via `mantrac_codegen` to be linked directly with external C, C++, Rust, or Fortran libraries without translation wrappers.
3. **Native Dynamic Linker & libc Integration**:
   - The resulting binary links against the host standard C library (`libc.so.6` on Linux, `libSystem.B.dylib` on macOS, `msvcrt.dll` on Windows).
   - Resolves dynamic loader (`ld-linux.so`, `dyld`) headers and program interpreters at load time.

---

## 4. Hardware Interaction: CPU, ALU, and MU (Memory Unit)

The machine code generated via `mantrac_codegen` executes directly on bare-metal hardware:

### Central Processing Unit (CPU)
* **Instruction Stream & Pipeline Flow**:
  - The compiler optimizes generated C code with `-O2`, enabling LLVM/GCC backends to generate optimal instruction sequences for modern multi-stage superscalar CPU pipelines (e.g., Intel Golden Cove, AMD Zen 4, Apple M-series Firestorm).
* **Superscalar Execution & Instruction-Level Parallelism (ILP)**:
  - Loop unrolling and instruction scheduling allow modern CPUs to issue up to 6–8 instructions per cycle across multiple execution units simultaneously.
* **Hardware Register Assignment**:
  - Emitted local variables are assigned directly to CPU hardware registers by the C compiler's graph-coloring register allocator, completely avoiding stack memory loads when possible.

### Arithmetic Logic Unit (ALU)
* **Direct Hardware Arithmetic Instructions**:
  - Mantra additions (`a + b`) become native `ADD` instructions executed in 1 CPU clock cycle ($0.2$–$0.3$ nanoseconds at 3–5 GHz clock frequencies).
  - Multiplications compile to hardware `IMUL` operations; integer division compiles to `IDIV`.
  - Bitwise manipulations (`&`, `|`, `^`, `<<`, `>>`) translate directly to single-cycle ALU gate operations (`AND`, `OR`, `XOR`, `SHL`, `SAR`).
* **Condition Flags & Branchless Code**:
  - With `-O2` optimization, many simple conditional assignments compile to **branchless conditional moves** (`CMOVcc` on x86, `CSEL` on ARM64), eliminating branch prediction penalties entirely.

### Memory Unit (MU) / Memory Management Unit (MMU)
* **Stack Memory (Activation Frames)**:
  - Local variables live inside the function's physical hardware activation frame, addressed relative to the stack pointer `[rsp + offset]`.
  - When returning from functions, the stack pointer is restored with a single `ADD rsp, <frame_size>` instruction—requiring zero dynamic memory allocator overhead.
* **Heap Memory & MMU Allocations**:
  - Class instances translate to native `malloc(sizeof(struct ...))`.
  - The C allocator manages chunks using memory arenas and free lists. Large allocations invoke `mmap` to request physical page frames from the OS MMU.
* **Cache Alignment & Data Locality**:
  - Struct fields generated by `CEmitter` are aligned to 8-byte boundaries (matching 64-bit machine words).
  - This prevents **unaligned memory access penalties** and ensures that data structs fit neatly within 64-byte L1 Data Cache lines, avoiding split-cache line fetches.
