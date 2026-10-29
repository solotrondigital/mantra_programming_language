# mantra_mdk — Mantra Development Kit CLI & Hardware Interaction

## 1. Package Overview & Purpose

`mantra_mdk` is the primary command-line toolchain driver (`mantra`) for the Mantra programming language ecosystem. It serves as the developer's single entry point, orchestrating project scaffolding, building, static checking, testing, formatting, linting, benchmarking, security auditing, dependency management, publishing, and cross-platform compilation targeting Browser (WebAssembly), Android (Kotlin Bridge), iOS (Swift Bridge), and backend server services (REST and GraphQL).

### Core Responsibilities:
* **Application Scaffolding Architecture (`mantra create`)**:
  - **Frontend Hybrid Application**: Scaffolds cross-platform hybrid apps targeting Web (WASM/PWA), Android (WebView Kotlin Bridge), and iOS (WKWebView Swift Bridge).
  - **Backend REST API Application**: Scaffolds modular MVC REST APIs with routers, controllers, services, repositories, validators, and middleware.
  - **Backend GraphQL API Application**: Scaffolds modular GraphQL servers with schema definitions, query/mutation resolvers, custom scalars, and middleware.
* **Multi-Environment Configuration**: Native loading of tiered `.env` files (`.env`, `.env.development`, `.env.test`, `.env.production`) via `--mantra-define-from-file=<file>`.
* **Standard Quality & Tooling**: Commands `mantra fmt`, `lint`, `doc`, `bench`, `audit`, `lsp`.
* **Package & Dependency System**: Commands `mantra add`, `remove`, `lock`, `publish`, `package validate`.
* **Unified Toolchain Dispatcher**: Locates and invokes compiler binaries (`mantrac`), language server daemons (`mantra-lsp`), and platform toolchains (GCC/Clang/MSVC, Android SDK/Gradle, Xcode).

---

## 2. Modules, Data Structures & Functions Used

### Key Modules:
* [`src/main.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/main.rs): Command-line argument parsing, dispatch logic, and CLI help/version rendering.
* [`src/scaffold/mod.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/scaffold/mod.rs): Primary entry point (`CreateCommand`) and argument tokenizer (`ArgumentParser`).
* [`src/scaffold/validator.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/scaffold/validator.rs): Strict conditional options validator (`ProjectOptionsValidator`) enforcing rules for frontend, REST, and GraphQL.
* [`src/scaffold/generator.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/scaffold/generator.rs): Project creation coordinator (`ProjectGenerator`).
* [`src/scaffold/frontend.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/scaffold/frontend.rs): Hybrid mobile/web scaffolding engine (`FrontendTemplateGenerator`).
* [`src/scaffold/rest.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/scaffold/rest.rs): MVC REST backend scaffolding engine (`RestTemplateGenerator`).
* [`src/scaffold/graphql.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/scaffold/graphql.rs): GraphQL API backend scaffolding engine (`GraphQLTemplateGenerator`).
* [`src/scaffold/metadata.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/scaffold/metadata.rs): Manifest (`mantra.toml`), `.gitignore`, and customized `README.md` writer (`ProjectMetadataGenerator`).
* [`src/scaffold/fs_service.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/scaffold/fs_service.rs): Safe filesystem service preventing accidental file overwrites (`FilesystemService`).
* [`src/env_loader.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/env_loader.rs): Multi-tier `.env` environment file loader, variable interpolation (`${VAR}`), secret masking for logs, and process environment variable injection.
* [`src/mobile_builder.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/mobile_builder.rs): Android APK / AAB packager, iOS application bundler, asset synchronization, and bridge capability verifier.
* [`src/runner.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/runner.rs): Subprocess process executor, embedded HTTP server for WebAssembly browser target (`mantra run -d chrome`), and compiler invoker.
* [`src/bench.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/bench.rs): Micro-benchmarking harness measuring execution latency and throughput.
* [`src/audit.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/audit.rs): Security auditing, SemVer verification, lockfile checksum authentication, and sandbox checks.
* [`src/manifest.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mdk/src/manifest.rs): TOML manifest parser and lockfile generator (`mantra.toml`, `mantra.lock`).

### Functions & Methods:
* `CreateCommand::execute(args: &[String]) -> Result<(), String>`: High-level command execution pipeline executing argument parsing, option validation, template generation, and success messaging.
* `ProjectOptionsValidator::validate(raw: RawProjectOptions) -> Result<ValidatedOptions, String>`: Validates `--type`, `--service`, `--platforms`, `--project-name`, `--org`, checks for existing directory collisions, and normalizes service identifiers.
* `ProjectGenerator::generate(opts: &ValidatedOptions) -> Result<GenerationSummary, String>`: Emits project metadata and dispatches generation to `FrontendTemplateGenerator`, `RestTemplateGenerator`, or `GraphQLTemplateGenerator`.
* `validate_project_name(name: &str) -> Result<(), String>`: Enforces `lowercase_with_underscores` naming convention for all project packages.
* `EnvLoader::load_file(path: &Path) -> Result<HashMap<String, String>, String>`: Parses `.env` lines, resolves variable substitutions, injects entries into `std::env::set_var`, and returns loaded key-value pairs.
* `MobileBuilder::build_apk(root: &Path, release: bool, defines_file: Option<&str>)`: Builds release or debug Android APK artifact in `android/app/build/outputs/apk/`.
* `MobileBuilder::build_appbundle(root: &Path, release: bool, defines_file: Option<&str>)`: Generates Google Play Store distribution bundle (AAB) in `android/app/build/outputs/bundle/`.
* `Runner::run_web_dev(web_port: u16, defines_file: Option<&str>)`: Launches embedded multi-threaded HTTP server with MIME type dispatch (`application/wasm`, `text/html`, `text/css`, `application/javascript`) on `http://127.0.0.1:<web_port>`.
* `Runner::build_web(target_file: Option<&str>, release: bool, custom_out: Option<&str>, defines_file: Option<&str>)`: Compiles WebAssembly output (`platforms/web/app.wasm` or `web/app.wasm`) from Mantra source.

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
   - Emits raw WebAssembly binary bytecode (`app.wasm`) and textual format (`app.wat`), validated against the Wasm standard specification for browser execution.
4. **Mobile Native Toolchains**:
   - Interfaces with Android Gradle plugin, `kotlinc`, and `apksigner` / `jarsigner`.
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

### Application Scaffolding (`mantra create`)

#### Frontend Hybrid Application
```bash
mantra create \
  --type frontend \
  --org com.testapp \
  --project-name testapp \
  --platforms ios,android,web \
  --description "TestApp cross-platform application"
```

#### Backend REST API Application
```bash
mantra create \
  --type backend \
  --service REST \
  --org com.testapp \
  --project-name testapp \
  --description "TestApp REST API backend"
```

#### Backend GraphQL API Application
```bash
mantra create \
  --type backend \
  --service GRAPHQL \
  --org com.testapp \
  --project-name testapp \
  --description "TestApp GraphQL backend"
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
