<p align="center">
  <img src="https://raw.githubusercontent.com/solotron-tech/mantra/main/assets/mantra_color_logo.png" alt="Mantra Programming Language" width="180" />
</p>

# Mantra CLI & Project Manager for Visual Studio Code

Official Visual Studio Code extension for the **Mantra Development Kit (MDK) CLI**.

> **Ownership & Stewardship:**  
> Developed, owned, and maintained by **[Solotron Technologies Private Limited](http://www.solotrontech.com)**.  
> Official Website: [http://www.solotrontech.com](http://www.solotrontech.com)

---

## 📋 Details

The **Mantra CLI & Project Manager** extension brings the full power of the Mantra Development Kit (`mantra`) directly into Visual Studio Code and Antigravity IDE. It streamlines full-stack development, multi-platform mobile builds (iOS & Android), WebAssembly bundle creation, micro-benchmarking, automated testing, code formatting, and environment diagnostics into a native graphical and terminal interface.

Developers no longer need to remember complex CLI parameters or switch between multiple windows to scaffold services, deploy to emulators, or verify compiler health.

---

## ⚡ Features

### 1. Interactive Project Scaffolding
- Quick-pick guided project wizard (`mantra create`).
- Support for **Frontend Hybrid Applications** (iOS, Android, and Web in a single shared codebase).
- Support for **Backend REST Services** and **Backend GraphQL Services**.

### 2. Multi-Platform Run & Debug Targets
- **Run on Web**: Compiles WebAssembly and starts local development dev server (`mantra run -d web`).
- **Run on Android**: Connects to active emulators and physical Android devices (`mantra run -d android`).
- **Run on iOS**: Connects to iOS Simulators and attached devices (`mantra run -d ios`).

### 3. Production Compilation & Packaging
- **WebAssembly**: Generate optimized `.wasm` bundles (`mantra build web`).
- **Android APK**: Build release and debug `.apk` application packages (`mantra build apk`).
- **Android App Bundle (AAB)**: Prepare signed bundles ready for Google Play upload (`mantra build appbundle`).
- **iOS Application Package**: Prepare complete native iOS bundles with Swift runtime bridges (`mantra mobile build ios`).

### 4. Code Quality & Formatting
- Format source files adhering to official 4-space indentation rules with one click (`mantra fmt`).
- Static lint diagnostics without writing build artifacts (`mantra lint`).
- Fast syntax and type checking (`mantra check`).

### 5. Automated Testing & Mobile Bridge Security
- Discover and execute unit and integration test suites (`mantra test`).
- Run the official **Mobile Bridge Security & Capability Conformance Test** (`mantra mobile test`).
- High-resolution CPU cycle benchmarking (`mantra bench`).

### 6. Visual Studio Code UI Integration
- **Activity Bar View**: Dedicated "Mantra" icon on the sidebar with one-click action buttons.
- **Toolchain Health View**: Live visual status indicators for compiler, LSP, and SDK dependencies.
- **Status Bar Item**: `$(terminal) Mantra CLI` item in the status bar for instant command palette access.
- **Integrated Terminal**: Automatically handles commands in a clean, dedicated "Mantra CLI" terminal.

---

## 📦 Dependencies & Prerequisites

This extension connects directly with the installed Mantra toolchain:

| Dependency | Required Version | Purpose |
| :--- | :--- | :--- |
| **`mantra-lang.mantra-language-support`** | `≥ v0.5.0` | Companion extension for syntax highlighting, autocomplete, and LSP |
| **`mantra` CLI** | `≥ v0.6.0` | Mantra Development Kit (MDK) binary on system `PATH` |
| **`mantrac` Compiler** | `≥ v0.6.0` | Mantra native and WebAssembly compiler |
| **`mantra-lsp`** | `≥ v0.5.0` | Language Server Protocol daemon |
| **Node.js** | `≥ v18.0.0` | Runtime environment for tooling |

### Verification
Run `mantra doctor` from the status bar or terminal to verify your development environment:
```bash
mantra doctor
```

---

## 🛠️ Contributed Commands

* `Mantra: Open Command Menu` — Displays the interactive quick-pick menu of all available actions.
* `Mantra CLI: Check Toolchain Health` — Runs `mantra doctor` to verify environment prerequisites.
* `Mantra CLI: Create New Project...` — Launches interactive project creation wizard.
* `Mantra CLI: Run Application` — Runs application on default target.
* `Mantra CLI: Run Application on Web` — Launches browser WebAssembly session.
* `Mantra CLI: Run Application on Android` — Deploys and executes on Android.
* `Mantra CLI: Run Application on iOS` — Deploys and executes on iOS Simulator.
* `Mantra CLI: Build WebAssembly Bundle` — Compiles WebAssembly binary.
* `Mantra CLI: Build Android APK` — Builds distributable Android APK.
* `Mantra CLI: Build Android App Bundle` — Builds Google Play AAB bundle.
* `Mantra CLI: Build iOS Application` — Builds iOS application bundle.
* `Mantra CLI: Run Test Suite` — Discovers and executes all unit tests.
* `Mantra CLI: Run Mobile Security & Bridge Test` — Tests native bridge security.
* `Mantra CLI: Format Files` — Formats files with deterministic 4-space indentation.
* `Mantra CLI: Lint Source Code` — Performs static analysis checks.
* `Mantra CLI: Run Benchmark Suite` — Profiles throughput and cycle costs.

---

## ⚙️ Extension Settings

| Setting | Default | Description |
| :--- | :--- | :--- |
| `mantra.cliPath` | `"mantra"` | Executable name or absolute path to the `mantra` CLI binary. |
| `mantra.autoShowTerminal` | `true` | Automatically bring the integrated terminal into view on command run. |

---

## 🏢 Stewardship

* **Author:** Solotron Technologies Private Limited
* **Website:** [http://www.solotrontech.com](http://www.solotrontech.com)
* **License:** MIT or GPL-3.0

