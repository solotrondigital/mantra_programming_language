# Mantra Programming Language — Native ABI & FFI Specification
## Stable Application Binary Interface (v1.0 / Edition 2026)

> **Document Status:** 🔒 STABLE ABI SPECIFICATION  
> **Target Release:** Mantra v0.6.0+  
> **C Header:** [`include/mantra_abi.h`](../../include/mantra_abi.h)

---

## 1. Overview & ABI Stability Guarantees

The **Mantra Native Application Binary Interface (ABI)** defines the low-level machine conventions governing how compiled Mantra programs interact with:
1. Native compiled modules and libraries (`.so`, `.dylib`, `.dll`).
2. Foreign code written in C, C++, Rust, Zig, Swift, and assembly.
3. Operating system APIs and system libraries.

### Stability Commitments
* **C ABI Compatibility:** All Mantra primitive types and object structures map cleanly to standard C99/C11 types.
* **Layout Invariants:** Structure padding, alignments, and object header layouts are frozen for Edition 2026.
* **Forward Compatibility:** Any dynamic library compiled against Mantra ABI v1.0 will link and execute without recompilation on all Mantra 0.6.x and 1.x runtime engines.

---

## 2. Primitive Type Representations

All primitives conform to IEEE and platform-standard binary representations:

| Mantra Type | C Equivalent | Size (Bytes) | Alignment | Notes |
|---|---|:---:|:---:|---|
| `Int` | `int64_t` | 8 | 8 | Two's complement signed integer |
| `Float` | `double` | 8 | 8 | IEEE 754 double precision float |
| `Boolean` | `uint8_t` | 1 | 1 | `0x00` = `false`, `0x01` = `true` |
| `Null` | `NULL` (`(void*)0`) | 8 | 8 | Nil pointer |
| `String` | `struct MantraString` | 24 | 8 | Pointer + length + hash + flags |
| `Any` | `void*` | 8 | 8 | Pointer to boxed `MantraObject` |

---

## 3. Object Memory Layout & VTables

### 3.1 Object Header (`MantraHeader`)
Every heap-allocated Mantra object begins with a 24-byte `MantraHeader`:

```text
+-----------------------------------------------------------+
| Offset | Field        | Type                | Size        |
+-----------------------------------------------------------+
| 0x00   | magic        | uint64_t            | 8 bytes     | (0x4D414E5452410001)
| 0x08   | vtable       | const MantraVTable* | 8 bytes     | Function pointer table
| 0x10   | refcount     | _Atomic uint32_t    | 4 bytes     | Atomic reference counter
| 0x14   | flags        | uint32_t            | 4 bytes     | GC & runtime status flags
+-----------------------------------------------------------+
| 0x18   | Field 0 ...  | (User class fields follow)        |
+-----------------------------------------------------------+
```

### 3.2 Virtual Method Table (`MantraVTable`)
Dynamic method dispatch utilizes a contiguous vtable array:
* Offset `0x00`: `const MantraTypeInfo* type_info` (RTTI pointer).
* Offset `0x08`: `size_t instance_size` (Object size in bytes).
* Offset `0x10`: `MantraDestructorFn destructor` (Called upon `refcount == 0`).
* Offset `0x18`: `size_t method_count`.
* Offset `0x20+`: Array of function pointers `void (*methods[])(void)`.

---

## 4. Name Mangling Scheme

To prevent symbol collisions and support method overloading and module hierarchies, the compiler applies standard name mangling:

### Mangling Syntax:
```text
_M_<PackageLength><Package>_<ModuleLength><Module>_<ClassLength><Class>_<MethodLength><Method>_<SignatureHash>
```

### Examples:
* Function `greet` in `app.main`:
  `_M_3app_4main_5greet_v0`
* Method `deposit` in class `BankAccount` in `finance.account`:
  `_M_7finance_7account_11BankAccount_7deposit_f64`

### Demangling Rule:
When exported with `extern "C" pub def function_name():`, name mangling is bypassed, and the symbol is emitted verbatim.

---

## 5. Foreign Function Interface (FFI) Rules

### 5.1 Declaring Foreign C Functions in Mantra
Use the `extern "C"` declaration syntax:

```mantra
# Declare external C function
extern "C":
    def puts(s: String) -> Int
    def sqrt(x: Float) -> Float
    def getpid() -> Int

def main() -> Int:
    puts("Hello from native C puts!")
    let root = sqrt(49.0)
    Console.println("Sqrt: " + str(root))
    return 0
```

### 5.2 Exporting Mantra Functions to C
Mark functions with `pub extern "C"`:

```mantra
pub extern "C" def mantra_calculate_tax(income: Float, rate: Float) -> Float:
    return income * rate
```

Compiled C header signature:
```c
MANTRA_API double MANTRA_CALL mantra_calculate_tax(double income, double rate);
```

### 5.3 Memory Ownership & Cross-Boundary Rules
1. **Primitives:** Passed by value across FFI boundaries with zero copying.
2. **Strings:**
   - Mantra passes `MantraString` views (`const char* data`, `size_t length`).
   - If C allocates memory for Mantra, it MUST use `mantra_alloc()`.
   - If Mantra passes a pointer to C, C MUST NOT free it.

---

*End of Native ABI Specification — Edition 2026*

