Mantra Programming Language — MVP Release Plan
Mantra is now the language name. I recommend building it as a sequence of independently testable MVP releases, with v0.1.0 establishing a usable language and v0.2.0 expanding execution targets, concurrency, and libraries.

The key design decision is to make Mantra’s first release usable without requiring users to install Python or Rust. Rust can be used internally to implement the bootstrap compiler, while Clang provides the initial native C compilation path.
MVP 1 — Mantra v0.1.0
Bootstrap Toolchain
First usable release · Language core and developer tools

Foundation

Goal: Write, check, run, test, and compile Mantra programs with a consistent language specification.

- Python-like syntax with mandatory four-space indentation.
- Lexer, indentation-aware parser, abstract syntax tree (AST), and source diagnostics.
- Static type checking and contract checking before execution.
- Full core object-oriented programming: classes, objects, encapsulation, inheritance, abstraction, interfaces, polymorphism, constructors, method overriding, and access control.
- Instant tree-walking interpreter for fast development feedback.
- C99 code generation followed by Clang compilation for native executables.
- Basic modules, imports, collections, strings, error handling, and file I/O.
- Unit test runner, formatter, project scaffolding, and package manifest.
- Windows, macOS, and Linux bootstrap toolchain builds.

Required v0.1.0 architecture
Mantra source files (.man)

Lexer → Indentation parser → AST
Static type checker + contract checker
Tree-walking VM

Instant execution


C99 backend

Clang → native binary




Important distinction: the tree-walking interpreter is the development execution engine; it is not a bytecode VM or a native JIT. C99 generation plus Clang is the initial native compilation backend. Static checks must run consistently before either execution path, unless a documented development option explicitly relaxes them.

Mantra should not claim to be self-hosting in v0.1.0 unless the compiler itself can be compiled from Mantra source. That is a separate milestone with reproducible bootstrap tests.
MVP 2 — Mantra v0.2.0
JIT, WebAssembly and Async Preview
Execution expansion · Networking and standard library

Preview

Goal: Add faster execution options, browser portability, asynchronous I/O, and a more capable application ecosystem.

- Cranelift-based JIT execution with a documented fallback to the v0.1.0 interpreter.
- WebAssembly compilation for supported browser and WASI environments.
- async/await, task scheduling, cancellation rules, and coroutine state management.
- Async networking with timeouts, cancellation, connection errors, and configurable resource limits.
- mantra.web for web application primitives and HTTP server integration.
- mantra.rest for REST clients, routing, middleware, JSON handling, and request validation.
- mantra.sql.orm for database connections, models, queries, migrations, and transactions.
- Expanded standard library for dates, paths, environment variables, serialization, logging, and testing.
- Dependency resolution and lockfiles for repeatable builds.
- Profiling hooks, JIT diagnostics, benchmark suites, and backend conformance tests.

v0.2.0 acceptance criteria
- Programs can choose interpreter or JIT execution using documented CLI flags.
- Interpreter and JIT produce equivalent observable results for the supported language subset.
- WebAssembly builds pass a defined compatibility test suite.
- Async tasks do not block the scheduler during supported nonblocking I/O.
- REST and SQL examples include error handling, tests, and secure configuration guidance.
- Dependency versions are reproducible from the lockfile.
- Unsupported language features fail with clear diagnostics rather than silently generating incorrect output.
Scope safeguard: Cranelift JIT and WebAssembly are distinct targets with different constraints. Start with a shared typed intermediate representation (IR), but do not assume every native feature automatically works in WebAssembly.

================================
MVP 3 — Mantra v0.3.0
Self-Hosting Compiler
Compiler written in Mantra


Goal: Make Mantra capable of compiling its own compiler source, while preserving a trusted bootstrap path.

- Implement the compiler frontend in Mantra where practical.
- Compile Mantra compiler source using the v0.1/v0.2 bootstrap compiler.
- Use the resulting compiler to compile the same source again.
- Add reproducible build checks, compiler version metadata, and regression tests.
- Keep the initial Rust implementation available as a recovery compiler until the self-hosted pipeline is dependable.
- Define stable AST/IR interfaces and backend contracts.
- Add compiler plugins only if their security and stability model is clearly specified.

Acceptance criteria: A clean build environment can bootstrap the compiler using documented steps; the resulting compiler passes the language conformance suite; and repeated builds produce equivalent outputs or documented reproducibility exceptions.


================================

MVP 4 — Mantra v0.4.0
Package Ecosystem and ADK
Project management · Distribution · Developer experience


Goal: Make Mantra projects easy to create, build, test, share, and maintain.

- mantra new, mantra run, mantra check, mantra build, and mantra test.
- mantra fmt, mantra lint, mantra doc, and mantra bench.
- Package registry, package publishing, version resolution, and lockfile verification.
- Semantic versioning and explicit language edition compatibility.
- Package integrity checks, dependency auditing, and build-script restrictions.
- Documentation generator and API reference output.
- Editor support through Language Server Protocol (LSP): diagnostics, completion, navigation, and formatting.
- Debugging support with stack traces, source locations, and variable inspection.

Proposed CLI
mantra new hello
cd hello
mantra check
mantra run
mantra test
mantra build --release
mantra fmt
mantra doc
mantra add mantra.rest
mantra package publish


Acceptance criteria: a new developer can install the toolchain, create a project, run tests, produce a native build, and use a dependency without manually assembling the compiler pipeline.

================================

MVP 5 — Mantra v0.5.0
Web and Mobile Application Platform
Browser, Android and iOS foundations


Goal: Reuse Mantra application logic across server, browser, and mobile targets.

- WebAssembly target and browser-host interoperability.
- mantra.web application scaffolding, routing, and API integration.
- Hybrid mobile architecture with a WebView UI and Mantra-managed business logic.
- Android bridge for lifecycle events, storage, networking, and permissions.
- iOS bridge for lifecycle events, storage, networking, and permissions.
- Shared validation, data models, API clients, and application services.
- Platform-specific build and signing instructions.
- Explicit security boundaries between web content and native capabilities.

Acceptance criteria: sample applications run on the supported browser target and on at least one Android and one iOS test configuration before the release is declared stable. Platform SDKs, signing, and deployment prerequisites must be documented.

================================

MVP 6 — Mantra v1.0.0
Stable Language and Production Toolchain
Compatibility commitments · Security · Release engineering

Stability milestone

Goal: Establish a stable language edition, toolchain compatibility policy, and production-ready release process.

- Frozen v1 language specification and formal compatibility policy.
- Defined ownership, memory safety, object lifetime, and resource cleanup rules.
- Complete type-checking and runtime error behavior for the supported language.
- Stable native ABI and documented FFI rules.
- Security review of package installation, build scripts, runtime boundaries, and native interop.
- Signed toolchain releases, checksums, release manifests, and software bill of materials (SBOM).
- CI across supported operating systems and architectures.
- Long-term regression, fuzzing, stress, and performance testing.
- Migration guide and compatibility policy for future releases.

Acceptance criteria: the published specification matches the implementation, release artifacts are verifiable, core tests pass on all declared targets, and breaking changes are governed by a documented policy.

Requirements that apply to every Mantra MVP
These requirements should be treated as a shared engineering contract, not optional features that can be postponed indefinitely.
1. Language and OOP specification
Mantra must define the syntax and exact behavior of its language features before claiming them as supported.
Area	Required behavior
Types	Static types, inference rules, generic types, nullable/optional values
Classes	Fields, methods, constructors, instance and static members
Encapsulation	Public, private, and any supported protected access rules
Inheritance	Single class inheritance and multiple interface implementation initially
Polymorphism	Overriding, dynamic dispatch, interface dispatch, subtype compatibility
Abstraction	Abstract classes and methods, interfaces, implementation checks
Properties	Getters, setters, visibility, and validation behavior
Overloading	Defined rules for method/function overload resolution
Object behavior	Equality, hashing, string representation, runtime type checks
Generics	Type parameters, constraints, and type-checking rules
Resource management	Ownership/lifetime model, cleanup, and error-path behavior
Operators	Defined operator overloading rules, where permitted

A feature is not complete just because the parser accepts it. Each feature needs type-checking rules, runtime or backend behavior, diagnostics, and conformance tests.
2. Compiler and runtime requirements
- One canonical language specification and a versioned grammar.
- A shared typed intermediate representation for the interpreter, C99 backend, and future Cranelift/WASM backends.
- Consistent type checking and language semantics across backends.
- Source maps and meaningful error messages with file, line, column, and error codes.
- A defined standard library and runtime ABI.
- A memory-management strategy documented before native backend development expands.
- No Python runtime dependency.
- No requirement for end users to install Rust when using prebuilt Mantra releases.
- Explicit target and feature support tables, so unsupported behavior is never implied to work.
3. Cross-platform release requirements
Desktop toolchain

Windows, macOS, and Linux. Provide versioned downloads, installation and upgrade instructions, and compiler/Clang prerequisite checks.



Web

Browser WebAssembly and/or documented WASI targets. Clearly distinguish browser APIs from server-side WASI capabilities.



Mobile

Android APK/AAB and iOS build/archive workflows, including platform SDKs, signing, native bridges, and permissions.



4. Performance and reliability requirements
Measure these metrics for every release rather than promising performance without evidence:
- Compiler startup time, compile time, and peak memory.
- Interpreter and native executable startup time.
- Binary size and idle memory use.
- Throughput, latency, allocations, and object-dispatch overhead.
- JIT warm-up time and WebAssembly startup cost.
- Async task scalability and network resource usage.
- Compatibility and correctness across execution backends.
Every release should publish reproducible benchmark instructions, test results, known limitations, and a list of implemented versus planned features.
Recommended implementation order
01

v0.1.0

Lexer, parser, type checker, OOP, interpreter, C99 backend


02

v0.2.0

Cranelift JIT, WASM, async runtime, web/REST/SQL foundations


03

v0.3.0

Self-hosting compiler and bootstrap verification


04

v0.4.0

ADK, package registry, LSP, formatter and debugger


05

v0.5.0

Browser and hybrid mobile platform


06

v1.0.0

Stable specification, security, compatibility and production release



My recommendation is to finish and freeze the v0.1.0 language specification before starting its implementation. That means agreeing on grammar, static type rules, OOP semantics, memory management, runtime interfaces, and the exact C99 backend contract. Otherwise, each new backend and library will risk implementing slightly different versions of Mantra.

All releases above are a proposed roadmap, not a claim that these components have already been implemented.