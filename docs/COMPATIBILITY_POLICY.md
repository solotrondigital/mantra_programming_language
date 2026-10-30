# Mantra Compatibility Policy & Semantic Versioning Commitments
## Edition 2026 · Release Governance & Evolution Framework

> **Document Status:** 🔒 RATIFIED & BINDING  
> **Effective Date:** October 2026 (v0.6.0+)  
> **Applies to:** Mantra Compiler (`mantrac`), MDK (`mantra`), Standard Library, and Native ABI

---

## 1. Core Commitment

The primary principle of Mantra's language design is **stability without stagnation**. Code written in Mantra today must continue to compile and execute predictably for years to come. 

Mantra establishes two immutable stability contracts:
1. **Source Stability:** Valid Mantra code written under an active language edition (e.g., `edition = "2026"`) will never be broken by point releases or minor compiler updates.
2. **Binary & ABI Stability:** Native shared libraries (`.so`, `.dylib`, `.dll`) compiled against the stable C ABI (`include/mantra_abi.h`) maintain forward and backward link compatibility across minor versions.

---

## 2. Semantic Versioning Specification (SemVer 2.0.0)

Mantra releases adhere to `MAJOR.MINOR.PATCH`:

### 2.1 Major Releases (`v1.0.0`, `v2.0.0`)
* **May contain breaking changes** to language grammar, type rules, or standard library APIs.
* May advance the default Language Edition.
* May modify the native C ABI.
* Always accompanied by automated migration tooling (`mantra fix`) and a detailed Migration Guide.

### 2.2 Minor Releases (`v0.6.0`, `v0.7.0`)
* **Strictly backward-compatible.**
* May introduce new standard library packages, new CLI commands, new compiler optimizations, and new language keywords *only* if they do not conflict with existing valid programs.
* May deprecate APIs with informative warnings, but **cannot remove them**.
* Preserves ABI layout and function signatures.

### 2.3 Patch Releases (`v0.6.1`, `v0.6.2`)
* **Strictly backward-compatible bug fixes and security patches.**
* Zero behavioral changes to valid programs.
* Zero API additions or removals.

---

## 3. The Language Edition Mechanism

Mantra decouples compiler releases from language syntax evolution through **Language Editions**.

### 3.1 Declaring an Edition
Every Mantra project specifies its language edition in `mantra.toml`:

```toml
[package]
name = "my_service"
version = "1.0.0"
edition = "2026"
```

### 3.2 Edition Rules:
1. **Never Deprecate an Old Edition:** Future compilers (e.g., Mantra v2.0 in 2030) will continue to support `edition = "2026"` mode.
2. **Interoperability:** Packages running different editions can depend on one another within the same project. The compiler parses each crate according to its declared edition before linking them into a single binary.

---

## 4. Deprecation & Removal Lifecycle

Breaking changes cannot occur abruptly. Any deprecation follows a three-stage lifecycle:

```text
+-----------------------+     +-----------------------+     +-----------------------+
|  Stage 1: DEPRECATED  | --> |   Stage 2: WARNING    | --> |   Stage 3: REMOVAL    |
| (Compiler Warns)      |     | (Across 2+ Minors)    |     | (Next Edition Only)   |
+-----------------------+     +-----------------------+     +-----------------------+
```

1. **Stage 1 (Introduction):** An API is annotated with `@deprecated("use new_method()", since="0.6.0")`. The compiler continues to generate working code and emits a warning during compilation.
2. **Stage 2 (Grace Period):** The warning remains active for at least **two consecutive minor releases** (e.g., v0.6.0 and v0.7.0).
3. **Stage 3 (Removal):** The deprecated symbol is removed only when a new Language Edition is released or during a major version transition.

---

## 5. Standard Library Stability Matrix

| Package | Stability Status | ABI Level | Breaking Changes Allowed |
|---|:---:|:---:|:---:|
| `mantra.lang` | 🔒 **FROZEN** | ABI v1 | Major Only |
| `mantra.io` | 🔒 **FROZEN** | ABI v1 | Major Only |
| `mantra.util.collections` | 🔒 **FROZEN** | ABI v1 | Major Only |
| `mantra.net.http` | 🔒 **FROZEN** | ABI v1 | Major Only |
| `mantra.web` | 🟢 **STABLE** | ABI v1 | Major Only |
| `mantra.rest` | 🟢 **STABLE** | ABI v1 | Major Only |
| `mantra.graphql` | 🟢 **STABLE** | ABI v1 | Major Only |
| `mantra.mobile` | 🟢 **STABLE** | ABI v1 | Major Only |
| `mantra.sql.orm` | 🟢 **STABLE** | ABI v1 | Major Only |

---

*End of Compatibility Policy — Edition 2026*

