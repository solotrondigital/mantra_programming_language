# mantra_mdk — Mantra Development Kit CLI & Hardware Interaction

## 1. Package Overview & Purpose

`mantra_mdk` is the primary command-line toolchain driver (`mantra`) for the Mantra programming language ecosystem. It serves as the developer's single entry point, orchestrating project scaffolding, building, static checking, testing, formatting, linting, benchmarking, security auditing, dependency management, and publishing.

### Core Responsibilities:
* **Project Lifecycle Management**: Commands `mantra new`, `build`, `run`, `check`, `test`.
* **Standard Quality & Tooling**: Commands `mantra fmt`, `lint`, `doc`, `bench`, `audit`, `lsp`.
* **Package & Dependency System**: Commands `mantra add`, `remove`, `lock`, `publish`, `package validate`.
* **Unified Toolchain Dispatcher**: Locates and invokes compiler binaries (`mantrac`), language server daemons (`mantra-lsp`), and platform toolchains (GCC/Clang/MSVC).

---

## 2. Modules, Data Structures & Functions Used

### Key Modules:
* [`src/main.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/main.rs): Command-line argument parsing, dispatch logic, and CLI help/version rendering.
* [`src/bench.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/bench.rs): Micro-benchmarking harness measuring execution latency and throughput.
* [`src/audit.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/audit.rs): Security auditing, SemVer verification, lockfile checksum authentication, and sandbox checks.
* [`src/manifest.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/manifest.rs): TOML manifest parser and lockfile generator (`mantra.toml`, `mantra.lock`).
* [`src/project.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/project.rs): Workspace scaffolding engine creating directories and initial files.
* [`src/formatter.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/formatter.rs): Source code indentation and normalization engine.
* [`src/linter.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/linter.rs): Idiomatic code style and AST smell detection.
* [`src/docgen.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/docgen.rs): Markdown and HTML documentation extractor from doc comments.
* [`src/runner.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/runner.rs): Subprocess process executor for compiled binaries and tests.

### Functions & Methods:
* `BenchRunner::run_benchmarks(root: &Path, iterations: usize)`: Discovers benchmark suites in `benches/*.mantra`, runs a 10-iteration CPU cache warmup, measures execution wall-clock time via `std::time::Instant`, and prints formatted ns/op and ops/sec metrics.
* `Auditor::audit_project(root: &Path)`: Inspects dependencies in `mantra.toml`, cross-checks cryptographic SHA-256 hashes in `mantra.lock`, evaluates edition compatibility (Edition 2026), and verifies build sandbox rules.
* `PackageManifest::load_from_dir(path: &Path) -> Result<PackageManifest, String>`: Parses project configuration, entry points, and dependency tree.
* `Lockfile::from_manifest(manifest: &PackageManifest) -> Self`: Computes reproducible deterministic SHA-256 digests for all project dependencies.
* `handle_publish(workspace: &Path)`: Validates package integrity, generates `.tar.gz` and `.sha256` archives into `dist/`, and prepares artifacts for Homebrew/Chocolatey taps.

---

## 3. Communication with Low-Level Language Binaries

`mantra_mdk` directly interfaces with native operating system binaries and compiler outputs:

1. **Native Compiler Execution (`mantrac`)**:
   - `mantra build` spawns the `mantrac` native compiler binary using POSIX `fork()`/`execve()` or Windows `CreateProcessW()`.
   - Passes backend compilation flags (`--jit`, `--wasm`, `-o <target>`).
2. **C Toolchain Orchestration (Clang / GCC / MSVC)**:
   - When lowering via `mantrac_codegen` (C99 emission), `mantra_mdk` invokes system C compilers to generate native object files (`.o`), static archives (`.a`), and native executable binaries (`ELF` on Linux, `Mach-O` on macOS, `PE/COFF` on Windows).
   - Links against the Mantra runtime library `libmantra_runtime.a` using OS linkers (`ld`, `ld64`, `lld`).
3. **Child Process I/O & Return Codes**:
   - Executes user target binaries (`mantra run`, `mantra test`) and captures stdout/stderr through operating system file descriptor pipes (`pipe(2)`).
   - Inspects exit status registers (`waitpid(2)`) to extract process termination status and signal numbers (e.g., `SIGSEGV`, `SIGBUS`).

---

## 4. Hardware Interaction: CPU, ALU, and MU (Memory Unit)

### Central Processing Unit (CPU)
* **High-Precision Timing Loops**:
  - `BenchRunner` executes benchmark loops that interface with hardware timestamp counters (x86 `RDTSC`/`RDTSCP` instructions, ARM64 `CNTVCT_EL0` virtual count registers) via `std::time::Instant::now()`.
* **Process Context Switches**:
  - Launching child processes (`mantra run`, `mantra lsp`) triggers OS scheduler context switches, saving CPU architectural registers (`RIP`, `RSP`, general-purpose registers) into thread control blocks (TCBs).
* **Multi-Core Dispatch**:
  - Future parallel test and benchmark execution utilizes OS hardware threads, running parallel tasks across physical CPU cores.

### Arithmetic Logic Unit (ALU)
* **Throughput & Latency Computations**:
  - In `BenchRunner`, the ALU executes 64-bit integer and floating-point divisions:
    $$\text{time\_per\_op} = \frac{\text{elapsed\_nanoseconds}}{\text{iterations}}$$
    $$\text{throughput} = \frac{10^9}{\text{time\_per\_op}}$$
  - Computations invoke `IDIV` (64-bit integer division) and `FDIV` / `DIVSD` (SSE2 double-precision floating-point division).
* **Cryptographic Hashing Operations**:
  - `Lockfile` integrity and `Auditor` checksum verification run SHA-256 compression loops. Each 512-bit message block performs 64 rounds of:
    - Bitwise rotations (`ROTR`), right shifts (`SHR`), bitwise `AND`, `OR`, `XOR`.
    - 32-bit modulo additions (`ADD modulo $2^{32}$`).
    - Leverages hardware cryptographic acceleration (Intel SHA-NI or ARMv8 Crypto extensions) when available.

### Memory Unit (MU) / Memory Management Unit (MMU)
* **File System Buffer Caching**:
  - Reading `mantra.toml`, `mantra.lock`, and source files triggers page cache interaction in OS kernel memory. Frequently read source files remain resident in the page cache (DRAM), reducing disk I/O latency to zero.
* **Heap Memory Management**:
  - Scaffolding templates and dependency graph trees are allocated on the process heap via `malloc` / `jemalloc`.
  - Contiguous allocations for file buffers minimize cache-line splits across 64-byte boundaries.
* **Memory Protection Flags**:
  - Target executables built by `mantra build` are written to disk with executable permission bits (`chmod 755`, `S_IXUSR`). When loaded by the kernel into RAM, the MMU marks code pages as `PROT_READ | PROT_EXEC` (no write) and data pages as `PROT_READ | PROT_WRITE` (no execute) enforcing hardware W^X security.
