<div align="center">

<img src="images/mantra_color_logo.png" alt="Mantra Logo" width="180"/>

# Mantra Programming Language

**A modern, expressive, statically-typed programming language built for clarity, safety, and performance.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE-GPL)
[![Version](https://img.shields.io/badge/version-0.2.0-cyan.svg)](#)
[![Built with Rust](https://img.shields.io/badge/built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![Maintained by Solotron](https://img.shields.io/badge/maintained%20by-Solotron%20Technologies-blueviolet.svg)](https://github.com/solotrondigital)

[Website](https://solotrondigital.github.io/mantra_website/mantra_web/) · [Docs](https://solotrondigital.github.io/mantra_website/mantra_web/mantra_docs/) · [Examples](#examples) · [Download](#installation)

</div>

---

## What is Mantra?

**Mantra** is a clean, Python-inspired programming language with static typing, object-oriented design, and a native compiler built in Rust. It combines the readability of Python with the safety and performance guarantees of a compiled, type-checked system.

Mantra is designed to be the language you reach for when you want **expressive syntax without runtime surprises** — whether you're building CLI tools, REST APIs, or systems software.

```mantra
def add(a: Int, b: Int) -> Int:
    return a + b

def main() -> Int:
    let result = add(42, 58)
    print("Result: " + str(result))   # Output: Result: 100
    return 0
```

---

## ✨ Key Features

| Feature | Description |
|---------|-------------|
| **Static Typing** | Every variable and function has a type, checked at compile time |
| **OOP First-Class** | Classes, inheritance, interfaces, abstract classes, polymorphism |
| **Python-like Syntax** | Indentation-based, readable, no braces or semicolons |
| **Native Compilation** | Compiles to C then to native binaries via `mantrac` |
| **VM / Interpreter** | Run directly with `--run` for fast iteration and scripting |
| **Type Inference** | `let x = 42` — types inferred where obvious |
| **Immutable by Default** | Use `mut` to declare mutable variables explicitly |
| **Standard Library** | Built-in packages for I/O, HTTP, REST, SQL ORM, collections |
| **MDK Toolchain** | Package manager, formatter, linter, doc generator in one tool |

---

## 📦 Project Structure

```
mantra_programming_language/
├── crates/
│   ├── mantrac/            # Compiler entry point (CLI: mantrac)
│   ├── mantrac_lexer/      # Tokenizer / Lexical analyser
│   ├── mantrac_ast/        # Abstract Syntax Tree definitions
│   ├── mantrac_parser/     # Parser: tokens → AST
│   ├── mantrac_checker/    # Type checker & semantic analyser
│   ├── mantrac_codegen/    # C code emitter + native compiler
│   ├── mantrac_vm/         # Bytecode interpreter / VM
│   ├── mantrac_ir/         # IR lowering & WebAssembly binary emitter
│   ├── mantra_mdk/         # Mantra Development Kit (CLI: mantra)
│   ├── mantra_mobile/      # Hybrid Mobile Engine (Android & iOS Bridges)
│   ├── mantra_lsp/         # Language Server Protocol daemon
│   ├── mantra_bootstrap/   # Self-hosting bootstrap verification harness
│   ├── mantra_web/         # Web framework (Router, Server, Middleware)
│   ├── mantra_rest/        # REST framework (Controllers, JSON, Routes)
│   ├── mantra_graphql/     # Native GraphQL runtime engine, SchemaBuilder, AST & SDL
│   └── mantra_sql_orm/     # Enterprise ORM (Oracle, MSSQL, MongoDB, SQLite, Postgres, MySQL)
├── examples/               # Sample Mantra programs
├── tests/                  # Conformance + negative test suite
├── installer/              # Platform installer scripts
├── images/                 # Logo and branding assets
├── create_installers.sh    # Build versioned installer packages
├── Cargo.toml              # Rust workspace manifest
├── LICENSE-MIT             # MIT License
└── LICENSE-GPL             # GNU GPL v3.0 License
```

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70+ with Cargo
- A C compiler (`cc` / `clang` / `gcc`) for native compilation

### Build from Source

```bash
# Clone the repo
git clone https://github.com/solotrondigital/mantra_programming_language.git
cd mantra_programming_language

# Build all crates (debug)
cargo build

# Build for release (optimized)
cargo build --release

# The compiler binary will be at:
./target/release/mantrac
```

### Installation via Installer Script

```bash
# Run the installer generator for your version
chmod +x create_installers.sh
./create_installers.sh 0.2.0

# Then run the platform installer:
# macOS
open installer/macos/*.dmg

# Linux
bash installer/linux/mantra-installer-linux.sh

# Windows (PowerShell as Admin)
.\installer\windows\Install-Mantra.ps1
```

---

## ⚡ Application Scaffolding CLI (`mantra create`)

Mantra provides a production-ready application scaffolding CLI (`mantra create`) supporting frontend hybrid applications and modular backend APIs (REST and GraphQL).

### 1. Frontend Hybrid Application (iOS, Android, Web)

```bash
mantra create \
  --type frontend \
  --org com.testapp \
  --project-name testapp \
  --platforms ios,android,web \
  --description "TestApp cross-platform application"
```

Generated architecture:
```text
testapp/
├── mantra.toml
├── README.md
├── .gitignore
├── src/
│   ├── app/                 # Root application bootstrap
│   ├── components/          # Reusable UI components
│   ├── screens/             # Application views & screens
│   ├── navigation/          # Screen routing controller
│   ├── services/            # Cross-platform HTTP/API client
│   ├── state/               # Global state store
│   ├── styles/              # Global styling & theme constants
│   └── main.mantra          # Application entry point
├── assets/images/           # Static asset directory
├── platforms/               # Platform-specific bridge projects
│   ├── ios/                 # iOS Swift bridge (WebKit WKWebView)
│   ├── android/             # Android Kotlin bridge (WebView)
│   └── web/                 # WebAssembly build & PWA shell
└── tests/                   # Unit & integration test suite
```

### 2. Backend REST API Application (MVC Architecture)

```bash
mantra create \
  --type backend \
  --service REST \
  --org com.testapp \
  --project-name testapp \
  --description "TestApp REST API backend"
```

Generated architecture:
```text
testapp/
├── mantra.toml
├── README.md
├── .gitignore
├── src/
│   ├── main.mantra          # Server entry point & route binding
│   ├── config/              # Application environment config
│   ├── routes/              # HTTP endpoint router
│   ├── controllers/         # Request handlers & response dispatch
│   ├── models/              # Domain entities
│   ├── services/            # Core business logic
│   ├── repositories/        # Data access abstraction
│   ├── middleware/          # Auth, logging, CORS middleware
│   ├── validators/          # Payload validators
│   └── utils/               # Response envelope formatters
├── tests/
│   ├── unit/                # Service unit tests
│   └── integration/         # Endpoint integration tests
└── docs/
    └── api_spec.md          # REST API documentation
```

### 3. Backend GraphQL API Application

```bash
mantra create \
  --type backend \
  --service GRAPHQL \
  --org com.testapp \
  --project-name testapp \
  --description "TestApp GraphQL backend"
```

Generated architecture:
```text
testapp/
├── mantra.toml
├── README.md
├── .gitignore
├── src/
│   ├── main.mantra          # GraphQL server entry point
│   ├── config/              # Server configuration
│   ├── graphql/
│   │   ├── schema/          # GraphQL schema definitions
│   │   ├── resolvers/       # Query and Mutation field resolvers
│   │   ├── types/           # GraphQL object type definitions
│   │   ├── queries/         # Query execution operations
│   │   ├── mutations/       # Mutation execution operations
│   │   └── scalars/         # Custom scalar types (DateTime)
│   ├── services/            # Reusable business logic
│   ├── models/              # Domain entities
│   ├── repositories/        # Data access layer
│   ├── middleware/          # Auth & logging context middleware
│   └── utils/               # GraphQL response formatters
├── tests/
│   ├── unit/                # Resolver unit tests
│   └── integration/         # GraphQL execution integration tests
└── docs/
    └── schema.graphql       # GraphQL SDL schema
```

### 4. Conditional Argument Rules & Options

| Argument | Frontend | Backend REST | Backend GraphQL |
|---|---|---|---|
| `--type` | Required: `frontend` | Required: `backend` | Required: `backend` |
| `--service` | Not applicable (rejected) | Required: `REST` | Required: `GRAPHQL` |
| `--org` | Supported (e.g. `com.example`) | Supported | Supported |
| `--project-name` | Required | Required | Required |
| `--platforms` | Supported (`ios,android,web`) | Not applicable (rejected) | Not applicable (rejected) |
| `--description` | Optional | Optional | Optional |

> **Notes:**
> - All Mantra source files strictly use the `.mantra` extension (never `.mr`).
> - The application entry point is always `src/main.mantra`.
> - Project names must adhere to the `lowercase_with_underscores` naming convention.
> - Service names are case-insensitive (`rest` / `graphql` normalize to `REST` / `GRAPHQL`).

### 2. Multi-Environment Runtime & Build Commands

Mantra projects support native loading of tiered environment configurations via `--mantra-define-from-file=<file>`:

#### Web Targets
```bash
# Web production:
mantra run -d chrome --web-port 4300 --mantra-define-from-file=.env.production

# Web development:
mantra run -d chrome --web-port 4300 --mantra-define-from-file=.env.development

# Web development build (WebAssembly wasm32-unknown-unknown):
mantra build web --release --mantra-define-from-file=.env.development
```

#### Mobile Android Targets
```bash
# Mobile local (Android):
mantra run -d android --mantra-define-from-file=.env

# Mobile Android test:
mantra run -d android --mantra-define-from-file=.env.test

# Mobile Android test APK:
mantra build apk --release --mantra-define-from-file=.env.test

# Mobile Android test AAB (Google Play Store):
mantra build appbundle --release --mantra-define-from-file=.env.test
```

#### Mobile iOS Targets
```bash
# Mobile iOS local:
mantra run -d ios --mantra-define-from-file=.env

# Mobile iOS development:
mantra run -d ios --mantra-define-from-file=.env.development

# Mobile iOS test:
mantra run -d ios --mantra-define-from-file=.env.test

# Mobile iOS prod:
mantra run -d ios --mantra-define-from-file=.env.production
```

---

## 💻 Using the Compiler (`mantrac`)

```
mantrac <input.mantra> [options]

Options:
  -o <path>       Specify the output executable or file path
  --run           Execute the program directly using the integrated VM
  --check         Parse and type-check the source without generating code
  --emit-c        Emit generated C intermediate source code
  --release       Build with native optimizations enabled (-O2)
  -v, --version   Print mantrac version
  -h, --help      Display this help message
```

### Quick Examples

```bash
# Run a file directly (interpreter mode — no compile step)
mantrac hello.mantra --run

# Compile to a native binary
mantrac hello.mantra -o hello

# Type-check only (no code generation)
mantrac hello.mantra --check

# Compile with optimizations
mantrac hello.mantra -o hello --release

# Emit C intermediate source
mantrac hello.mantra --emit-c
```

---

## 📝 Language Examples

### Hello World

```mantra
def main() -> Int:
    print("Hello, Mantra! 🚀")
    return 0
```

### Variables, Types & Conditionals

```mantra
def main() -> Int:
    let name: String = "Mantra"
    let version: Float = 0.2
    mut counter: Int = 0

    if version >= 0.2:
        print(name + " v" + str(version) + " is ready!")

    while counter < 3:
        counter = counter + 1
        print("Iteration: " + str(counter))

    return 0
```

### Classes & OOP

```mantra
class BankAccount:
    def __init__(self, owner: String, initial_balance: Float):
        self.owner = owner
        self.balance = initial_balance

    pub def deposit(self, amount: Float):
        self.balance = self.balance + amount
        print("Deposited $" + str(amount))

    pub def get_balance(self) -> Float:
        return self.balance

def main() -> Int:
    let account = BankAccount("Alice", 1000.0)
    account.deposit(500.0)
    print("Balance: $" + str(account.get_balance()))
    return 0
```

### Interfaces & Polymorphism

```mantra
interface Shape:
    def area(self) -> Float
    def describe(self) -> String

class Circle:
    def __init__(self, radius: Float):
        self.radius = radius

    pub def area(self) -> Float:
        return 3.14159 * self.radius * self.radius

    pub def describe(self) -> String:
        return "Circle with radius " + str(self.radius)

class Rectangle:
    def __init__(self, width: Float, height: Float):
        self.width = width
        self.height = height

    pub def area(self) -> Float:
        return self.width * self.height

    pub def describe(self) -> String:
        return "Rectangle " + str(self.width) + "x" + str(self.height)
```

### REST API with `mantra.rest`

```mantra
class UserController:
    def __init__(self):
        self.users = []

    pub def get_users(self) -> String:
        return "[" + ", ".join(self.users) + "]"

    pub def create_user(self, name: String, email: String) -> String:
        self.users.append("{\"name\": \"" + name + "\", \"email\": \"" + email + "\"}")
        return "{\"status\": \"created\"}"

def main() -> Int:
    let controller = UserController()
    print(controller.get_users())
    return 0
```

### GraphQL API with `mantra.graphql`

```mantra
import mantra.graphql.*

def main() -> Int:
    # Build typed GraphQL schema with query and mutation resolvers
    let schema = SchemaBuilder("Query")
        .mutation("Mutation")
        .resolver("Query", "hello", def(ctx):
            return "Hello from Mantra GraphQL!"
        )
        .build()

    let engine = GraphQLEngine(schema)
    let response = engine.execute("{ hello }")
    print(response.to_json())   # Output: {"data":{"hello":"Hello from Mantra GraphQL!"}}
    return 0
```

### Enterprise Persistence with `mantra.sql.orm`

```mantra
import mantra.sql.orm.*

def main() -> Int:
    # Connect to Oracle, MSSQL, PostgreSQL, MySQL, MongoDB, or Generic DB
    let db = OracleAdapter("oracle://user:pass@localhost:1521/ORCLPDB")
    db.connect()

    # 1. Execute Stored Procedure with IN / OUT parameters
    let params = [
        ProcedureParam("p_dept_id", FieldValue.Int(10), ParamDirection.In),
        ProcedureParam("p_total_emp", FieldValue.Null, ParamDirection.Out)
    ]
    let result = db.execute_procedure("get_dept_employee_count", params)
    print("Total Employees: " + str(result.output_params.get("p_total_emp")))

    # 2. Database Cursors
    let cursor = db.open_cursor("emp_cursor", "SELECT emp_id, emp_name FROM employees")
    let page = cursor.fetch_many(50)

    # 3. Views & Materialized Views
    db.create_view("active_customers_mv", "SELECT * FROM customers WHERE status = 'ACTIVE'", true)
    db.refresh_materialized_view("active_customers_mv")

    db.disconnect()
    return 0
```

---

## 📚 Standard Library Packages

| Package | Description |
|---------|-------------|
| `mantra.lang` | Core types: `Object`, `String`, `List`, `Equatable`, `BaseException` |
| `mantra.io` | File I/O, Console, Environment |
| `mantra.util.collections` | `HashMap`, `Stack`, `Queue` |
| `mantra.net.http` | `HttpServer`, `HttpRequest`, `HttpResponse` |
| `mantra.web` | `Router`, `Server`, `Middleware`, `Request`, `Response`, `Template` |
| `mantra.rest` | `RestController`, `CrudController`, `RouteHandler`, `JsonSerializer`, `ApiResponse` |
| `mantra.graphql` | `GraphQLEngine`, `SchemaBuilder`, `GraphQLType`, `GraphQLValue`, `DateTime`/`JSON`/`UUID` Scalars, Introspection |
| `mantra.sql.orm` | Enterprise ORM: Oracle, MSSQL, MongoDB, SQLite, PostgreSQL, MySQL, Generic DB, Stored Procedures, Cursors, Views, Functions |
| `mantra.mdk` | `DocGenerator`, `Formatter`, `PackageManifest`, `ProjectScaffolder` |

---

## 🛠️ MDK — Mantra Development Kit

The `mdk` CLI is the all-in-one project toolchain:

```bash
# Create a new project
mantra new my_project

# Build the project
mantra build

# Run the project
mantra run

# Format source files
mantra fmt

# Lint the codebase
mantra lint

# Generate API documentation
mantra doc

# Run tests
mantra test
```

**`mantra.toml`** — Project manifest (similar to `Cargo.toml`):

```toml
[package]
name    = "my_app"
version = "0.1.0"
author  = "Your Name"
lang    = "mantra"

[dependencies]
mantra.web = "0.2.0"
mantra.rest = "0.2.0"
```

---

## 🧪 Running Tests

```bash
# Run all conformance and negative tests
bash tests/run_all.sh

# Or run via Cargo (Rust unit tests for compiler internals)
cargo test
```

The test suite includes:
- **Conformance tests** — Classes, encapsulation, inheritance, polymorphism, generics, constructors, equality, static members
- **Negative tests** — Expected compiler errors for invalid programs (abstract instantiation, missing interfaces, private access violations, etc.)

---

## 🏗️ Compiler Architecture

```
Source (.mantra)
      │
      ▼
  mantrac_lexer       → Tokens
      │
      ▼
  mantrac_parser      → AST (Abstract Syntax Tree)
      │
      ▼
  mantrac_checker     → Type-checked, semantically verified AST
      │
      ├──── --run ──→  mantrac_vm   → Bytecode Interpreter (direct execution)
      │
      └──── default →  mantrac_codegen
                            │
                            ├─ CEmitter       → C intermediate source
                            └─ NativeCompiler → Binary via system C compiler
```

---

## 📋 Roadmap

- [x] Lexer, Parser, AST
- [x] Type Checker with OOP semantics (classes, interfaces, inheritance)
- [x] C code emitter (`--emit-c`)
- [x] Native binary compilation (`--release`)
- [x] Integrated VM / interpreter (`--run`)
- [x] MDK toolchain (fmt, lint, doc, scaffold, benchmark, audit)
- [x] Standard library: mantra.web, mantra.rest, mantra.sql.orm
- [x] MVP 4: Package Ecosystem, Lockfile, Homebrew/Chocolatey Distribution
- [x] MVP 5: Web & Mobile Application Platform (Browser WASM, Android Kotlin Bridge, iOS Swift Bridge)
- [x] Unified Cross-Platform CLI Scaffolding (`mantra create --org ... --project-name ...`)
- [x] Multi-Environment Support (`.env`, `.env.development`, `.env.test`, `.env.production`)
- [x] WebAssembly Target Build (`mantra build web`)
- [ ] Generics (full implementation)
- [ ] Pattern matching (`match` expressions)
- [ ] Package registry online cloud service
- [ ] Self-hosting compiler (Mantra compiled in Mantra)

---

## 📄 License

Mantra is dual-licensed. You may use it under either:

- **MIT License** — See [LICENSE-MIT](LICENSE-MIT)
- **GNU General Public License v3.0** — See [LICENSE-GPL](LICENSE-GPL)

You may choose whichever license best suits your use case.

---

## 🏢 About

Mantra is developed and maintained by **[Solotron Technologies Private Limited](https://github.com/solotrondigital)**.

> Built with ❤️ and Rust. Designed for developers who value clarity, safety, and performance.

---

<div align="center">

**© 2026 Mantra Programming Language · Solotron Technologies Private Limited**

[Website](https://solotrondigital.github.io/mantra_website/mantra_web/) · [Report a Bug](https://github.com/solotrondigital/mantra_programming_language/issues) · [Request a Feature](https://github.com/solotrondigital/mantra_programming_language/issues)

</div>
