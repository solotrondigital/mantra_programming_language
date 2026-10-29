# mantra_mdk — Mantra Development Kit CLI & Hardware Interaction

## 1. Package Overview & Purpose

`mantra_mdk` is the primary command-line toolchain driver (`mantra`) for the Mantra programming language ecosystem. It serves as the developer's single entry point, orchestrating project scaffolding, building, static checking, testing, formatting, linting, benchmarking, security auditing, dependency management, publishing, and cross-platform compilation targeting Browser (WebAssembly), Android (Kotlin Bridge), and iOS (Swift Bridge).

### Core Responsibilities:
* **Project Lifecycle & Cross-Platform Scaffolding**: Commands `mantra create`, `new`, `build`, `run`, `check`, `test`.
* **Multi-Environment Configuration**: Native loading of tiered `.env` files (`.env`, `.env.development`, `.env.test`, `.env.production`) via `--mantra-define-from-file=<file>`.
* **Standard Quality & Tooling**: Commands `mantra fmt`, `lint`, `doc`, `bench`, `audit`, `lsp`.
* **Package & Dependency System**: Commands `mantra add`, `remove`, `lock`, `publish`, `package validate`.
* **Unified Toolchain Dispatcher**: Locates and invokes compiler binaries (`mantrac`), language server daemons (`mantra-lsp`), and platform toolchains (GCC/Clang/MSVC, Android SDK/Gradle, Xcode).

---

## 2. Modules, Data Structures & Functions Used

### Key Modules:
* [`src/main.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/main.rs): Command-line argument parsing, dispatch logic, and CLI help/version rendering.
* [`src/env_loader.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/env_loader.rs): Multi-tier `.env` environment file loader, variable interpolation (`${VAR}`), secret masking for logs, and process environment variable injection.
* [`src/project.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/project.rs): Workspace scaffolding engine creating directory layouts, manifest files, and platform bridge projects (`web/`, `android/`, `ios/`).
* [`src/mobile_builder.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/mobile_builder.rs): Android APK / AAB packager, iOS application bundler, asset synchronization, and bridge capability verifier.
* [`src/runner.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/runner.rs): Subprocess process executor, embedded HTTP server for WebAssembly browser target (`mantra run -d chrome`), and compiler invoker.
* [`src/bench.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/bench.rs): Micro-benchmarking harness measuring execution latency and throughput.
* [`src/audit.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/audit.rs): Security auditing, SemVer verification, lockfile checksum authentication, and sandbox checks.
* [`src/manifest.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/manifest.rs): TOML manifest parser and lockfile generator (`mantra.toml`, `mantra.lock`).
* [`src/formatter.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/formatter.rs): Source code indentation and normalization engine.
* [`src/linter.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/linter.rs): Idiomatic code style and AST smell detection.
* [`src/docgen.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/docgen.rs): Markdown documentation extractor from doc comments.

### Functions & Methods:
* `validate_project_name(name: &str) -> Result<(), String>`: Enforces the `lowercase_with_underscores` naming convention for all new projects.
* `ProjectScaffolder::create_cross_platform(args: &CreateArgs) -> Result<(), String>`: Scaffolds the complete multi-platform workspace (`lib/`, `src/shared/`, `web/`, `android/`, `ios/`, `assets/`, `.env*`, `mantra.toml`).
* `EnvLoader::load_file(path: &Path) -> Result<HashMap<String, String>, String>`: Parses `.env` lines, resolves variable substitutions, injects entries into `std::env::set_var`, and returns loaded key-value pairs.
* `EnvLoader::print_summary(source_label: &str, vars: &HashMap<String, String>)`: Formats loaded variables for developer console output, masking sensitive tokens (keys containing `KEY`, `SECRET`, `PASSWORD`, `TOKEN`).
* `MobileBuilder::build_apk(root: &Path, release: bool, defines_file: Option<&str>)`: Builds release or debug Android APK artifact in `android/app/build/outputs/apk/`.
* `MobileBuilder::build_appbundle(root: &Path, release: bool, defines_file: Option<&str>)`: Generates Google Play Store distribution bundle (AAB) in `android/app/build/outputs/bundle/`.
* `MobileBuilder::run_mobile(device: &str, defines_file: Option<&str>)`: Executes simulated bridge runtime test harness for Android / iOS targets.
* `Runner::run_web_dev(web_port: u16, defines_file: Option<&str>)`: Launches embedded multi-threaded HTTP server with MIME type dispatch (`application/wasm`, `text/html`, `text/css`, `application/javascript`) on `http://127.0.0.1:<web_port>`.
* `Runner::build_web(target_file: Option<&str>, release: bool, custom_out: Option<&str>, defines_file: Option<&str>)`: Compiles WebAssembly output (`web/app.wasm`, `web/app.wat`) from Mantra source.
* `BenchRunner::run_benchmarks(root: &Path, iterations: usize)`: Discovers benchmark suites in `benches/*.mantra`, runs a 10-iteration CPU cache warmup, measures execution wall-clock time via `std::time::Instant`, and prints formatted ns/op and ops/sec metrics.
* `Auditor::audit_project(root: &Path)`: Inspects dependencies in `mantra.toml`, cross-checks cryptographic SHA-256 hashes in `mantra.lock`, evaluates edition compatibility (Edition 2026), and verifies build sandbox rules.
* `PackageManifest::load_from_dir(path: &Path) -> Result<PackageManifest, String>`: Parses project configuration, entry points, and dependency tree.
* `Lockfile::from_manifest(manifest: &PackageManifest) -> Self`: Computes reproducible deterministic SHA-256 digests for all project dependencies.
* `handle_publish(workspace: &Path)`: Validates package integrity, generates `.tar.gz` and `.sha256` archives into `dist/`, and prepares artifacts for package distribution.

---

## 3. Communication with Low-Level Language Binaries

`mantra_mdk` directly interfaces with native operating system binaries and compiler outputs:

1. **Native Compiler Execution (`mantrac`)**:
   - `mantra build` spawns the `mantrac` native compiler binary using POSIX `fork()`/`execve()` or Windows `CreateProcessW()`.
   - Passes backend compilation flags (`--jit`, `--wasm`, `-o <target>`, `--mantra-define-from-file=<file>`).
2. **C Toolchain Orchestration (Clang / GCC / MSVC)**:
   - When lowering via `mantrac_codegen` (C99 emission), `mantra_mdk` invokes system C compilers to generate native object files (`.o`), static archives (`.a`), and native executable binaries (`ELF` on Linux, `Mach-O` on macOS, `PE/COFF` on Windows).
   - Links against the Mantra runtime library `libmantra_runtime.a` using OS linkers (`ld`, `ld64`, `lld`).
3. **WebAssembly Toolchain (`wasm32-unknown-unknown`)**:
   - Emits raw WebAssembly binary bytecode (`web/app.wasm`) and textual format (`web/app.wat`), validated against the Wasm standard specification for browser execution.
4. **Mobile Native Toolchains**:
   - Interfaces with the Android Gradle plugin, `kotlinc`, and `apksigner` / `jarsigner`.
   - Interfaces with Apple `xcodebuild` and `xcrun` for iOS compilation and code-signing verification.
5. **Child Process I/O & Return Codes**:
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
  - Parallel test, benchmark, and compilation pipelines utilize OS hardware threads, running worker tasks across physical CPU cores.

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
  - Reading `mantra.toml`, `mantra.lock`, `.env*`, and source files triggers page cache interaction in OS kernel memory. Frequently read source files remain resident in the page cache (DRAM), reducing disk I/O latency to zero.
* **Heap Memory Management**:
  - Scaffolding templates, environment map lookups, and dependency graph trees are allocated on the process heap via `malloc` / `jemalloc`.
  - Contiguous allocations for file buffers minimize cache-line splits across 64-byte boundaries.
* **Memory Protection Flags**:
  - Target executables built by `mantra build` are written to disk with executable permission bits (`chmod 755`, `S_IXUSR`). When loaded by the kernel into RAM, the MMU marks code pages as `PROT_READ | PROT_EXEC` (no write) and data pages as `PROT_READ | PROT_WRITE` (no execute) enforcing hardware W^X security.

---

## 5. Command Reference & Usage Examples

### Project Creation

```bash
# Basic project creation (name must follow lowercase_with_underscores)
mantra create my_awesome_app

# Create minimal project (skips default counter/model template)
mantra create my_app --empty

# Create with custom reverse-DNS organization domain
mantra create my_app --org com.company

# Limit supported platforms (default: "ios,android,web")
mantra create my_app --platforms="ios,android,web"

# Custom project template (app, module, package, plugin, skeleton)
mantra create my_plugin -t plugin

# Custom description written to mantra.toml
mantra create my_app --description "A cool cross-platform app"

# Full options example
mantra create \
  --org com.test.app \
  --project-name testapp \
  --platforms="ios,android,web" \
  --description="TestApp cross-platform application" \
  testapp
```

### Multi-Environment Runtime & Build Commands

```bash
# Web production runtime:
mantra run -d chrome --web-port 4300 --mantra-define-from-file=.env.production

# Web development runtime:
mantra run -d chrome --web-port 4300 --mantra-define-from-file=.env.development

# Web release compilation:
mantra build web --release --mantra-define-from-file=.env.development

# Mobile local runtime (Android):
mantra run -d android --mantra-define-from-file=.env

# Mobile Android test runtime:
mantra run -d android --mantra-define-from-file=.env.test

# Mobile Android test APK:
mantra build apk --release --mantra-define-from-file=.env.test

# Mobile Android test AAB (Google Play Store):
mantra build appbundle --release --mantra-define-from-file=.env.test

# Mobile iOS local runtime:
mantra run -d ios --mantra-define-from-file=.env

# Mobile iOS development runtime:
mantra run -d ios --mantra-define-from-file=.env.development

# Mobile iOS test runtime:
mantra run -d ios --mantra-define-from-file=.env.test

# Mobile iOS production runtime:
mantra run -d ios --mantra-define-from-file=.env.production
```
