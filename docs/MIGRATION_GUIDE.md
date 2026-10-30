# Mantra Migration Guide — Upgrading from v0.5.0 to v0.6.0
## Production Toolchain, ABI Stabilization & Security Hardening

> **Document Status:** 📘 OFFICIAL MIGRATION MANUAL  
> **Target Version:** Mantra v0.6.0+ (Edition 2026)

---

## 1. Overview of Changes in v0.6.0

Mantra v0.6.0 marks the **Stability and Production Toolchain Milestone (MVP 6)**. This release transitions Mantra from experimental preview to an enterprise-grade, production-hardened platform.

### Key Additions in v0.6.0:
1. **Frozen v1.0 Language Specification:** Formal EBNF grammar, strict indentation rules (tabs strictly disallowed), and frozen OOP semantics.
2. **Deterministic Ownership & ARC:** Automated Reference Counting with stack allocation elision and `__del__()` destructor hooks.
3. **C-Compatible Stable Native ABI:** Official `include/mantra_abi.h` header with frozen vtable layout and `extern "C"` FFI conventions.
4. **Toolchain Security Hardening:** Package install hash verification, build script sandboxing, and runtime capability boundary controls.
5. **Signed Distribution & Provenance:** Cryptographic release signing, release manifest (`release.json`), and SPDX/CycloneDX SBOMs.
6. **New CLI Subsystems:** `mantra audit` (dependency security scanner) and `mantra bench` (micro-benchmarking harness).

---

## 2. Breaking Changes & Strictness Enforcements

While v0.6.0 is predominantly backward-compatible with v0.5.0, several compiler strictness checks are now enforced:

### 2.1 Disallow Tabs in Indentation (`E0001`)
* **Previous behavior (v0.5.0):** Tabs were tolerated or treated as 4 spaces.
* **New behavior (v0.6.0):** Tab characters (`\t`) in indentation positions are strictly forbidden and trigger error `E0001: Tabs Disallowed in Indentation`.
* **Remediation:** Run `mantra fmt` on your codebase to automatically convert tabs to 4 ASCII spaces:
  ```bash
  $ mantra fmt
  ```

### 2.2 Strict Nominal Interface Implementations (`E0103`)
* **Previous behavior (v0.5.0):** Subclasses inheriting an interface occasionally permitted relaxed parameter naming.
* **New behavior (v0.6.0):** Subclasses implementing an interface MUST match method names, parameter counts, and types exactly.
* **Remediation:** Verify that any class declaring `implements IMyInterface` provides all declared methods with exact signatures.

### 2.3 Lockfile Hash Verification (`SEC001`)
* **New behavior:** `mantra install` validates the SHA-256 checksum recorded in `mantra.lock`.
* **Remediation:** If upgrading third-party dependencies, regenerate the lockfile:
  ```bash
  $ mantra check --update-lock
  ```

---

## 3. Step-by-Step Upgrade Guide

### Step 1: Update the Toolchain
Download the v0.6.0 release for your platform or update via package managers:

* **macOS / Linux (Homebrew):**
  ```bash
  brew update
  brew upgrade mantra
  ```
* **Windows (Chocolatey):**
  ```powershell
  choco upgrade mantra -y
  ```

### Step 2: Verify Toolchain Version
```bash
$ mantra --version
Mantra Development Kit (MDK) v0.6.0 (Stable Language & Production Toolchain)
Compiler: mantrac 0.6.0
```

### Step 3: Run Codebase Formatting
Ensure all source files comply with the 4-space indentation rule:
```bash
$ cd my_project
$ mantra fmt
```

### Step 4: Run Static Type-Check & Verification
Check for any interface or type signature mismatches:
```bash
$ mantra check
```

### Step 5: Run Security Audit
Scan dependencies and verify package integrity:
```bash
$ mantra audit
```

### Step 6: Execute Project Tests
Run test suites under v0.6.0:
```bash
$ mantra test
```

---

## 4. Troubleshooting & Assistance

If you encounter migration issues:
* **Documentation Portal:** [https://solotrondigital.github.io/mantra_website/mantra_docs/](https://solotrondigital.github.io/mantra_website/mantra_docs/)
* **Issue Tracker:** [https://github.com/solotrondigital/mantra_programming_language/issues](https://github.com/solotrondigital/mantra_programming_language/issues)
* **Security Advisories:** `security@solotron.tech`

---

*End of Migration Guide — Mantra v0.6.0*

