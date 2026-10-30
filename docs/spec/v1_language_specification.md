# Mantra Programming Language — Frozen v1.0 Language Specification
## Edition 2026 · Formal Language Reference & Compatibility Policy

> **Document Status:** 🔒 FROZEN V1 SPECIFICATION  
> **Target Release:** Mantra v0.6.0+ (Edition 2026)  
> **Classification:** Official Standard Reference Manual  
> **Maintainer:** Solotron Technologies & Mantra Language Core Team

---

## Table of Contents

1. [Scope & Conformance](#1-scope--conformance)
2. [Lexical Structure](#2-lexical-structure)
   - 2.1 Character Set & Source Encoding
   - 2.2 Whitespace, Indentation & Off-side Rule
   - 2.3 Comments
   - 2.4 Keywords & Reserved Words
   - 2.5 Identifiers
   - 2.6 Literals
3. [Syntactic Grammar (EBNF)](#3-syntactic-grammar-ebnf)
4. [Type System & Operational Semantics](#4-type-system--operational-semantics)
   - 4.1 Primitive Types
   - 4.2 Class Types & Single Inheritance
   - 4.3 Interfaces & Structural Trait Contracts
   - 4.4 Abstract Classes & Blueprint Enforcement
   - 4.5 Generics & Parameterized Types
   - 4.6 Subtyping & Variance Rules
5. [Statements & Execution Semantics](#5-statements--execution-semantics)
   - 5.1 Variable Bindings (`let` and `mut`)
   - 5.2 Control Flow (`if`, `while`, `for..in`)
   - 5.3 Exception Handling (`try..catch..finally`)
   - 5.4 Resource Management (`with..as`)
6. [Object Lifetime, Ownership & Memory Safety](#6-object-lifetime-ownership--memory-safety)
   - 6.1 Deterministic Reference Counting (ARC)
   - 6.2 Destructors & Cleanup Hooks (`__del__`)
   - 6.3 Memory Invariants & Bounds Safety
7. [Module & Namespace System](#7-module--namespace-system)
8. [Formal Compatibility Policy](#8-formal-compatibility-policy)
   - 8.1 Semantic Versioning Commitments
   - 8.2 Edition Mechanism (`edition = "2026"`)
   - 8.3 Deprecation & Removal Lifecycle
   - 8.4 Binary & Source Stability Invariants

---

## 1. Scope & Conformance

This document defines the formal language specification for **Mantra Edition 2026 (v1.0 Frozen Specification)**. A conforming Mantra implementation (compiler or interpreter) MUST strictly adhere to the lexical, grammatical, type-theoretic, and runtime semantics defined herein.

Any compiler feature, syntax extension, or standard library API not documented in this specification is considered non-standard and MUST NOT be accepted in strict conformance mode (`mantrac --check --strict`).

---

## 2. Lexical Structure

### 2.1 Character Set & Source Encoding
* Source files MUST be encoded in **UTF-8**.
* Source files MAY include a UTF-8 Byte Order Mark (BOM), which the lexer MUST silently skip.
* Newline sequences: Standard Unix LF (`\n`) and Windows CRLF (`\r\n`) are normalized to a single logical newline token `NEWLINE`.

### 2.2 Whitespace, Indentation & Off-side Rule
Mantra uses indentation to define block boundaries (the off-side rule):
* **Spaces Only:** Blocks MUST be indented using standard ASCII spaces (`0x20`). Tab characters (`\t`) are strictly forbidden in indentation positions and MUST trigger lexical error `E0001: Tabs Disallowed in Indentation`.
* **Standard Width:** Canonical block indentation is **4 spaces**. The lexer emits an `INDENT` token when the indentation level increases and a `DEDENT` token for each level it decreases.
* Blank lines containing only whitespace and comment lines are ignored by the indentation scanner.

### 2.3 Comments
* **Single-line Comments:** Begin with `#` and extend to the end of the line.
  ```mantra
  # This is a line comment
  let x = 42  # Inline comment
  ```
* **Multi-line Block Comments:** Enclosed within triple hashes `### ... ###`.
  ```mantra
  ###
  Multi-line block comment
  Documenting complex algorithm logic
  ###
  ```

### 2.4 Keywords & Reserved Words
The following tokens are reserved keywords and cannot be used as identifiers:

```text
abstract    as          break       catch       class
continue    def         elif        else        extends
finally     fn          for         if          implements
import      in          interface   let         mut
override    pass        pub         return      self
static      super       try         while       with
```

### 2.5 Identifiers
* Regular identifiers start with an ASCII letter (`a-z`, `A-Z`) or underscore `_`, followed by letters, digits (`0-9`), or underscores.
* **Naming Conventions:**
  * Types, Classes, Interfaces: `UpperCamelCase` (e.g., `HttpRequest`, `DatabaseAdapter`).
  * Functions, Methods, Variables: `snake_case` (e.g., `calculate_checksum`, `max_retries`).
  * Private members: Prefix with a single underscore `_` (e.g., `_balance`, `_connect_internal`).
  * Constants: `UPPER_SNAKE_CASE` (e.g., `MAX_BUFFER_SIZE`).

### 2.6 Literals
* **Integers (`Int`):** 64-bit signed integers in two's complement format.
  * Decimal: `42`, `-1024`, `1_000_000` (underscores permitted).
  * Hexadecimal: `0xFF`, `0x1A2B`.
  * Binary: `0b101010`.
  * Octal: `0o755`.
* **Floats (`Float`):** IEEE 754 double-precision 64-bit floats.
  * `3.14159`, `-0.001`, `1.5e10`, `2.0E-4`.
* **Booleans (`Boolean`):** `true` and `false`.
* **Strings (`String`):** Immutable, UTF-8 encoded text literals enclosed in double quotes `"..."`.
  * Escape sequences: `\n`, `\r`, `\t`, `\\`, `\"`, `\0`, `\u{XXXX}` (Unicode code point).
* **Null (`Null`):** The singleton value `null`.

---

## 3. Syntactic Grammar (EBNF)

```ebnf
Program         ::= Statement* EOF ;

Statement       ::= ClassDef
                  | InterfaceDef
                  | FunctionDef
                  | VarDecl
                  | Assignment
                  | IfStmt
                  | WhileStmt
                  | ForStmt
                  | TryStmt
                  | WithStmt
                  | ReturnStmt
                  | ExprStmt ;

ClassDef        ::= [ "abstract" ] "class" IDENTIFIER [ "extends" IDENTIFIER ] [ "implements" TypeList ] ":" NEWLINE INDENT ClassMember* DEDENT ;

InterfaceDef    ::= "interface" IDENTIFIER [ "extends" TypeList ] ":" NEWLINE INDENT InterfaceMember* DEDENT ;

ClassMember     ::= [ "pub" ] [ "static" | "override" | "abstract" ] FunctionDef
                  | VarDecl ;

InterfaceMember ::= [ "pub" ] "def" IDENTIFIER "(" ParameterList? ")" [ "->" Type ] NEWLINE ;

FunctionDef     ::= "def" IDENTIFIER "(" ParameterList? ")" [ "->" Type ] ":" NEWLINE INDENT Statement+ DEDENT ;

ParameterList   ::= Parameter ( "," Parameter )* ;
Parameter       ::= IDENTIFIER [ ":" Type ] [ "=" Expression ] ;

VarDecl         ::= ( "let" | "mut" ) IDENTIFIER [ ":" Type ] "=" Expression NEWLINE ;
Assignment      ::= LValue "=" Expression NEWLINE ;

IfStmt          ::= "if" Expression ":" NEWLINE INDENT Statement+ DEDENT
                    ( "elif" Expression ":" NEWLINE INDENT Statement+ DEDENT )*
                    [ "else" ":" NEWLINE INDENT Statement+ DEDENT ] ;

WhileStmt       ::= "while" Expression ":" NEWLINE INDENT Statement+ DEDENT ;
ForStmt         ::= "for" IDENTIFIER "in" Expression ":" NEWLINE INDENT Statement+ DEDENT ;

TryStmt         ::= "try" ":" NEWLINE INDENT Statement+ DEDENT
                    "catch" IDENTIFIER ":" Type ":" NEWLINE INDENT Statement+ DEDENT
                    [ "finally" ":" NEWLINE INDENT Statement+ DEDENT ] ;

WithStmt        ::= "with" Expression "as" IDENTIFIER ":" NEWLINE INDENT Statement+ DEDENT ;

ReturnStmt      ::= "return" [ Expression ] NEWLINE ;

Type            ::= IDENTIFIER [ "[" TypeList "]" ] ;
TypeList        ::= Type ( "," Type )* ;
```

---

## 4. Type System & Operational Semantics

Mantra is **statically typed** with strong compile-time type verification and optional local type inference.

### 4.1 Primitive Types
| Type | Representation | Zero/Default Value | Notes |
|---|---|---|---|
| `Int` | 64-bit signed int | `0` | Overflow traps in debug, wraps in release |
| `Float` | 64-bit IEEE 754 | `0.0` | NaN / Infinity supported |
| `Boolean` | 8-bit boolean | `false` | Strictly `true` or `false` |
| `String` | Immutable UTF-8 buffer | `""` | Length-prefixed, O(1) size |
| `Null` | Nil pointer | `null` | Only assignable to nullable types |
| `Any` | Boxed dynamic container | `null` | Dynamic reflection wrapper |

### 4.2 Class Types & Single Inheritance
* Single inheritance: A class may extend at most one superclass via `extends`.
* Subclass constructors MUST invoke `super(...)` before accessing `self`.
* Dynamic method dispatch occurs via an immutable compiler-synthesized virtual method table (`vtable`).

### 4.3 Interfaces & Structural Trait Contracts
* An interface defines a set of method signatures that conforming classes must implement.
* A class declares conformance using `implements Interface1, Interface2`.
* Conformance is verified statically at compile time. Failure to implement all declared methods triggers `E0103: Missing Interface Implementation`.

### 4.4 Abstract Classes & Blueprint Enforcement
* Marked with `abstract class`. Direct instantiation via `ClassName()` is forbidden and triggers `E0101: Abstract Class Instantiation`.
* Methods marked with `abstract def` have no body and MUST be overridden by non-abstract subclasses. Failure triggers `E0105: Unimplemented Abstract Method`.

### 4.5 Generics & Parameterized Types
* Generic classes and functions accept type parameters: `class Box[T]:`, `class Map[K, V]:`.
* Compile-time monomorphization generates specialized native machine code for primitive type arguments, eliminating boxing overhead.

### 4.6 Subtyping & Variance Rules
* Covariant return types: An overriding method may return a subtype of the overridden method's return type.
* Contravariant parameter types: An overriding method may accept a supertype of the overridden method's parameter type.

---

## 5. Statements & Execution Semantics

### 5.1 Variable Bindings (`let` and `mut`)
* `let` introduces an **immutable binding**. Reassignment is a compile-time error (`E0201: Cannot Reassign Immutable Binding`).
* `mut` introduces a **mutable binding**.

### 5.2 Control Flow
* `if/elif/else` conditions MUST evaluate strictly to `Boolean`. Truthy/falsy implicit coercion of integers or pointers is forbidden.
* `while condition:` executes repeatedly while the condition remains `true`.
* `for item in collection:` desugars to iterator calls (`collection.iterator()`, `iter.has_next()`, `iter.next()`).

### 5.3 Exception Handling
* Exceptions derive from `BaseException`.
* The `try..catch..finally` block guarantees that the `finally` block executes under all normal and exceptional control-flow paths.

### 5.4 Resource Management (`with..as`)
* Guarantees deterministic resource cleanup (files, sockets, database transactions).
* Desugaring:
  ```mantra
  with open("data.txt") as f:
      f.write("content")
  ```
  Desugars to:
  ```mantra
  let _res = open("data.txt")
  let f = _res.__enter__()
  try:
      f.write("content")
  finally:
      _res.__exit__()
  ```

---

## 6. Object Lifetime, Ownership & Memory Safety

### 6.1 Deterministic Reference Counting (ARC)
Mantra uses **Automated Reference Counting (ARC)** with compiler-assisted cycle detection and local stack elision:
* Object headers include a 32-bit atomic reference counter.
* When references leave scope, the counter decrements. When the counter reaches zero, the memory and all child references are freed immediately.
* Local variables whose lifetimes are proven not to escape the stack frame are allocated directly on the native call stack, requiring zero heap allocation.

### 6.2 Destructors & Cleanup Hooks (`__del__`)
* Classes may define an explicit destructor method `pub def __del__(self) -> void`.
* The compiler guarantees `__del__` is called exactly once when the object's reference count drops to zero, prior to deallocating memory.

### 6.3 Memory Invariants & Bounds Safety
* Array/List index access (`list[index]`) is strictly bounds-checked. Out-of-bounds access raises `IndexOutOfBoundsException` rather than memory corruption.
* Raw pointers and unsafe memory manipulations are restricted exclusively to FFI modules marked with `extern`.

---

## 7. Module & Namespace System

* Every file corresponds to a module.
* Packages are declared in `mantra.toml`.
* Modules are imported using `import package.module` or `from package.module import ClassName`.
* Encapsulation: Symbols are private to their module by default unless explicitly annotated with `pub`.

---

## 8. Formal Compatibility Policy

### 8.1 Semantic Versioning Commitments
Mantra strictly complies with **Semantic Versioning 2.0.0 (SemVer)**:
* **MAJOR (v1.0.0, v2.0.0):** Incompatible language changes, grammar modifications, or ABI breaking changes.
* **MINOR (v0.6.0, v0.7.0):** Backward-compatible new features, compiler optimizations, and additions to the standard library.
* **PATCH (v0.6.1, v0.6.2):** Backward-compatible bug fixes and diagnostic improvements.

### 8.2 Edition Mechanism (`edition = "2026"`)
* To allow language evolution without breaking existing code, Mantra uses **Editions**.
* The current edition is **2026**.
* A compiler supporting Edition 2026 MUST compile Edition 2026 source files indefinitely, even in future major compiler releases.

### 8.3 Deprecation & Removal Lifecycle
1. **Notice Phase:** An API or feature is marked with `@deprecated("message", since="0.6.0")`. The compiler emits warnings during `mantra check` and `mantra build`.
2. **Grace Period:** Deprecated features remain functional across at least **two minor versions**.
3. **Removal Phase:** A deprecated feature can only be removed in a **new Language Edition** or a **MAJOR version bump**.

### 8.4 Binary & Source Stability Invariants
* **Source Stability:** Valid Edition 2026 source code will never fail to compile on newer compilers supporting Edition 2026.
* **Native ABI Stability:** Native dynamic libraries (`.dylib`, `.so`, `.dll`) compiled against the Mantra v1 ABI (`include/mantra_abi.h`) maintain binary compatibility across all minor and patch releases.

---

*End of Formal Specification — Mantra Edition 2026*

