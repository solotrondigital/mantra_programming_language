# Mantra Programming Language — Security Review & Threat Model
## Production Toolchain Security, Runtime Isolation & FFI Hardening

> **Document Status:** 🔒 COMPLETED & RATIFIED  
> **Target Release:** Mantra v0.6.0+  
> **Security Baseline:** ISO/IEC 27001 & NIST SP 800-218 (SSDF) Compliant

---

## Table of Contents

1. [Executive Summary & Security Posture](#1-executive-summary--security-posture)
2. [Threat Model & Attack Surface](#2-threat-model--attack-surface)
3. [Package Installation & Supply Chain Integrity](#3-package-installation--supply-chain-integrity)
4. [Build Script Sandboxing & Execution Isolation](#4-build-script-sandboxing--execution-isolation)
5. [Runtime Capability Boundaries](#5-runtime-capability-boundaries)
6. [Native Interop (FFI) Memory Safety & Sanitization](#6-native-interop-ffi-memory-safety--sanitization)
7. [Dependency Auditing Tool (`mantra audit`)](#7-dependency-auditing-tool-mantra-audit)
8. [Vulnerability Disclosure & Security Response Policy](#8-vulnerability-disclosure--security-response-policy)

---

## 1. Executive Summary & Security Posture

Mantra v0.6.0 establishes a comprehensive security baseline for the language compiler, package manager (`mantra`), standard library, and runtime. Unlike legacy environments where package managers run arbitrary code during installation and native FFI crashes the whole process, Mantra enforces **defense-in-depth**:

1. **Supply Chain Defense:** Cryptographic SHA-256/512 checksums verified against lockfiles before any package code reaches the disk.
2. **Build Isolation:** Build scripts run within restricted process jails; network calls and arbitrary shell invocations are blocked by default.
3. **Memory Safety by Construction:** The compiler eliminates dangling pointers and buffer overflows via deterministic Automated Reference Counting (ARC) and strict bounds checking.
4. **Hardened Native FFI:** Foreign calls crossing native boundaries are validated, wrapped in exception handlers, and subjected to address boundary checks.

---

## 2. Threat Model & Attack Surface

| Attack Vector | Threat Scenario | Mitigation Strategy | Severity |
|---|---|---|:---:|
| **Package Tampering** | Attacker compromises registry or MITMs package download | Lockfile SHA-256 verification; HTTPS/TLS 1.3 only; signed release manifests | **CRITICAL** |
| **Malicious Post-Install Script** | Trojanized dependency runs `rm -rf` or exfiltrates SSH keys | **Zero arbitrary post-install scripts allowed**. Packages can only contain pure Mantra code unless explicit `--allow-build-scripts` is passed | **CRITICAL** |
| **Path Traversal ("Zip Slip")** | Malicious archive contains `../../etc/cron` | Archive extraction sanitization; path normalization rejects any path outside target root | **HIGH** |
| **FFI Memory Corruption** | Bad C library passes wild pointer causing SIGSEGV | Pointer validation guards; signal handlers map SIGSEGV to catchable `NativeMemoryException` | **HIGH** |
| **Dependency Hijacking** | Typo-squatting or dependency confusion | Scope-enforced naming (`@org/package`); registry namespace reservation | **MEDIUM** |
| **Denial of Service** | Deeply nested AST crashing compiler with stack overflow | Parser recursion depth limit (max 512 nested scopes) with graceful syntax errors | **MEDIUM** |

---

## 3. Package Installation & Supply Chain Integrity

### 3.1 Lockfile Verification Protocol
When `mantra install` or `mantra add` executes:
1. The package archive (`.tar.gz`) is downloaded into an isolated memory buffer.
2. The SHA-256 hash is computed and compared against `mantra.lock`:
   ```toml
   [[package]]
   name = "mantra.graphql"
   version = "0.5.0"
   checksum = "06fbed656bb4841a9107c3ab796a5ec3536a7092cdf8648ee500d4a98ad12c05"
   ```
3. If hashes do not match byte-for-byte, installation aborts immediately with error `SEC001: Package Integrity Verification Failed`.

### 3.2 Zip-Slip & Path Traversal Prevention
Archive extraction is strictly bound to the target directory:
```rust
fn validate_extract_path(dest_dir: &Path, entry_path: &Path) -> Result<PathBuf, SecurityError> {
    let clean = entry_path.components().filter(|c| !matches!(c, Component::ParentDir)).collect();
    let target = dest_dir.join(clean);
    if !target.starts_with(dest_dir) {
        return Err(SecurityError::PathTraversalAttempt(entry_path.to_path_buf()));
    }
    Ok(target)
}
```

---

## 4. Build Script Sandboxing & Execution Isolation

In Mantra, packages are **declarative**. Packages are NOT allowed to execute arbitrary code at build time by default.

### Build Script Security Rules:
* Pure compilation: `mantrac` transforms `.mantra` source to target binaries without executing package code.
* If a package requires native compilation (e.g. compiling a bundled C file), it must declare it in `mantra.toml`:
  ```toml
  [build.native]
  source = "native/binding.c"
  allow_network = false
  allow_shell = false
  ```
* Any attempt to open network sockets or spawn subprocesses during `mantra build` will be terminated by the process capability sandbox.

---

## 5. Runtime Capability Boundaries

Mantra applications can be run in **Hardened Capability Mode** (`mantra run --sandbox`):

```text
+-----------------------------------------------------------+
|               MANTRA RUNTIME ENVIRONMENT                  |
+-----------------------------------------------------------+
| Allowed Capabilities (configured via mantra.toml)         |
|  [x] fs.read: ["./data", "./config"]                      |
|  [ ] fs.write: (BLOCKED)                                  |
|  [x] net.connect: ["api.example.com:443"]                 |
|  [ ] process.spawn: (BLOCKED)                             |
+-----------------------------------------------------------+
```

When sandbox mode is active:
* Filesystem reads outside allowed directories throw `SecurityException`.
* Unlisted outbound network connections are refused.
* Process spawning (`Environment.exec()`) is completely disabled.

---

## 6. Native Interop (FFI) Memory Safety & Sanitization

1. **Null Pointer Checks:** Passing a `null` Mantra reference to a non-nullable C pointer generates compile-time diagnostic `E0310: FFI Null Safety Violation`.
2. **Buffer Bounds Checks:** Strings crossing FFI boundaries carry explicit byte lengths (`MantraString.length`), preventing classic C string null-byte truncation exploits.
3. **Signal Trapping:** In native mode, the Mantra runtime installs custom handlers for `SIGSEGV`, `SIGBUS`, and `SIGFPE`. If a third-party C library causes a segmentation fault, Mantra captures the signal, formats a diagnostic stack trace, and aborts cleanly without leaving dangling file locks or leaking credentials.

---

## 7. Dependency Auditing Tool (`mantra audit`)

The built-in audit engine analyzes all project dependencies:

```bash
$ mantra audit
=== Mantra Security & Dependency Audit Report ===

  Dependencies scanned : 8
  Checksums verified   : 8/8
  Edition compatibility: PASS (2026 Edition)
  SemVer compliance    : PASS
  Build script safety  : PASS (Sandboxed)

  Security status      : 0 vulnerabilities found. All dependencies verified!

=================================================
```

---

## 8. Vulnerability Disclosure & Security Response Policy

* **Reporting:** Security researchers can report vulnerabilities privately to `security@solotron.tech`.
* **SLA:**
  * Critical vulnerabilities acknowledged within **24 hours**.
  * Patch released within **72 hours** alongside CVE publication.
* **Release Signing:** All official toolchain releases and security patches are cryptographically signed with Solotron's release signing key.

---

*End of Security Review — Mantra v0.6.0*

