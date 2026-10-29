<div align="center">

<img src="images/mantra_logo.png" alt="Mantra Logo" width="180"/>

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
│   ├── mantra_mdk/         # Mantra Development Kit (CLI: mdk)
│   ├── mantra_web/         # Web framework (Router, Server, Middleware)
│   ├── mantra_rest/        # REST framework (Controllers, JSON, Routes)
│   └── mantra_sql_orm/     # SQL ORM (Model, QueryBuilder, Adapters)
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
| `mantra.sql.orm` | `Model`, `QueryBuilder`, `Migration`, SQLite / MySQL / PostgreSQL adapters |
| `mantra.mdk` | `DocGenerator`, `Formatter`, `PackageManifest`, `ProjectScaffolder` |

---

## 🛠️ MDK — Mantra Development Kit

The `mdk` CLI is the all-in-one project toolchain:

```bash
# Create a new project
mdk new my_project

# Build the project
mdk build

# Run the project
mdk run

# Format source files
mdk fmt

# Lint the codebase
mdk lint

# Generate API documentation
mdk doc

# Run tests
mdk test
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
- [x] MDK toolchain (fmt, lint, doc, scaffold)
- [x] Standard library: mantra.web, mantra.rest, mantra.sql.orm
- [ ] Generics (full implementation)
- [ ] Pattern matching (`match` expressions)
- [ ] Async/await support
- [ ] Package registry (mantra.pkg)
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
