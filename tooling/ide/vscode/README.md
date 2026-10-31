<p align="center">
  <img src="https://raw.githubusercontent.com/solotron-tech/mantra/main/assets/mantra_color_logo.png" alt="Mantra Programming Language" width="180" />
</p>

# Mantra Language Support for Visual Studio Code

Official Visual Studio Code extension for the **Mantra Programming Language** (`.mantra`).

> **Ownership & Stewardship:**  
> Developed, owned, and maintained by **[Solotron Technologies Private Limited](http://www.solotrontech.com)**.  
> Official Website: [http://www.solotrontech.com](http://www.solotrontech.com)

---

## 📋 Details

The **Mantra Language Support** extension delivers an enterprise-grade developer experience for Mantra engineering teams. It connects seamlessly to the **Mantra Universal Language Server (`mantra-lsp`)** via Language Server Protocol (LSP 3.17) to provide real-time syntactic analysis, compiler diagnostics, type-aware autocompletion, semantic highlighting, and code formatting directly within Visual Studio Code and Antigravity IDE.

### Supported File Types
* `.mantra` — Source code files for the Mantra Programming Language.

---

## ⚡ Features

### 1. Rich Syntax Highlighting & Themes
- TextMate grammar parsing classes, interfaces, traits, decorators, visibility modifiers, and control flow.
- Includes official **Mantra Dark** and **Mantra Light** themes matching the Mantra brand specification.

### 2. Semantic Tokens Highlighting
- Dynamic, compiler AST-driven semantic token classification differentiating types, interfaces, methods, functions, parameters, variables, fields, and decorators.

### 3. IntelliSense & Smart Autocomplete
- Context-sensitive member completion when typing `.`.
- Inheritance auto-suggestions after `extends` and `implements`.
- Full completion support for all 22 Mantra language keywords and 11 standard library built-ins with documentation markdown.
- Standard function and class scaffolding snippets.

### 4. Code Navigation & Discovery
- **Go to Definition** (`F12` / `Cmd+Click` / `Ctrl+Click`).
- **Go to Declaration**.
- **Go to Implementation** for interface contracts and abstract classes.
- **Find All References** (`Shift+F12`).
- **Document Symbols / Outline** (`Cmd+Shift+O` / `Ctrl+Shift+O`).
- **Workspace Symbol Search** (`Cmd+T` / `Ctrl+T`).

### 5. Compiler Diagnostics & Error Checking
- Instant feedback on syntax errors, missing tokens, and type mismatches.
- Non-blocking in-memory analysis without writing intermediate artifacts to disk.

### 6. Signature Help & Hover Documentation
- Parameter hints and active parameter tracking when typing function/method call arguments.
- Rich hover cards with signature prototypes and documentation for declarations.

### 7. Formatting & Refactoring
- Deterministic 4-space indentation adhering to the Mantra style guide (`mantra fmt`).
- Project-wide safe symbol rename (`F2`).
- Code actions for organizing imports and removing unused definitions.

---

## 📦 Dependencies & Prerequisites

To utilize all language server features, ensure the following components are installed on your workstation:

| Dependency | Required Version | Purpose |
| :--- | :--- | :--- |
| **`mantra-lsp`** | `≥ v0.5.0` | Central Language Server Protocol daemon on system `PATH` |
| **`mantra` CLI** | `≥ v0.6.0` | Mantra Development Kit (MDK) for building, testing, and formatting |
| **`mantrac`** | `≥ v0.6.0` | Production Compiler (Rust bootstrap / Native / WASM emitter) |
| **VS Code / IDE** | `^1.85.0` | Compatible with VS Code, Antigravity IDE, VSCodium, and Cursor |

### Installing Prerequisites
The Mantra toolchain binaries can be verified and installed via:
```bash
# Verify toolchain health
mantra doctor

# Ensure mantra-lsp is in your PATH
which mantra-lsp
```
If `mantra-lsp` is in a custom path, configure it in the Extension Settings below.

---

## ⚙️ Extension Settings

This extension contributes the following configurable settings (`mantra.*`):

| Setting | Default | Description |
| :--- | :--- | :--- |
| `mantra.languageServerPath` | `"mantra-lsp"` | Path or executable name for the `mantra-lsp` daemon. |
| `mantra.enableDiagnostics` | `true` | Enable real-time compiler diagnostics and error squiggles. |
| `mantra.enableSemanticHighlighting` | `true` | Enable compiler AST-driven semantic syntax highlighting. |
| `mantra.formatOnSave` | `false` | Automatically format files on save using 4-space indentation. |

---

## 🛠️ Contributed Commands

* `Mantra: Restart Language Server` — Gracefully stops and restarts the `mantra-lsp` daemon process.
* `Mantra: Show Language Server Output` — Opens the dedicated log stream in the Output panel.

---

## 🏢 Stewardship

* **Author:** Solotron Technologies Private Limited
* **Website:** [http://www.solotrontech.com](http://www.solotrontech.com)
* **License:** MIT or GPL-3.0
