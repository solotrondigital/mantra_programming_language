# mantrac_vm — Tree-Walk Virtual Machine, Interpreter & Hardware Interaction

## 1. Package Overview & Purpose

`mantrac_vm` is the canonical interpreter and runtime execution engine for the Mantra programming language. Introduced in Mantra v0.1.0 and maintained as the trusted baseline reference engine through v0.4.0, it provides:
1. **Immediate Execution Without Ahead-of-Time Compilation**: Evaluates Mantra Abstract Syntax Trees (`mantrac_ast::Program`) directly in a managed memory environment.
2. **Dynamic Value Model**: Represents all Mantra runtime entities—primitives (`Int`, `Float`, `Bool`, `String`), heap-allocated objects, closures, and built-in function pointers.
3. **Object-Oriented Runtime Mechanics**: Handles class instantiation, method dispatch, single inheritance, `super` resolution, and static fields.
4. **Standard Runtime Built-in Primitives**: Implements core host operations (`print`, `len`, `range`, `assert`, `read_file`, `write_file`, `system_exec`, `get_env`).
5. **Trusted Verification Baseline**: Used as the differential ground truth against which JIT execution and self-hosted compiler outputs are verified.

---

## 2. Modules, Data Structures & Functions Used

### Key Modules:
* [`src/value.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantrac_vm/src/value.rs): Defines the dynamic runtime `Value` tagged union and heap `ObjectInstance`.
* [`src/interpreter.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantrac_vm/src/interpreter.rs): Tree-walking execution engine, call stack environment, and control flow manager.

### Data Structures & Enums:
* `Value`: Tagged union enum representing runtime values:
  * `Int(i64)`: 64-bit signed integer.
  * `Float(f64)`: 64-bit IEEE 754 floating-point number.
  * `Bool(bool)`: Boolean value.
  * `String(String)`: Heap-allocated UTF-8 string.
  * `List(Vec<Value>)`: Dynamic array of values.
  * `Dict(HashMap<String, Value>)`: Key-value hash map.
  * `Object(Rc<RefCell<ObjectInstance>>)`: Reference-counted instance of a user-defined class.
  * `BuiltinFunction(String)`: Direct handle to native runtime builtins.
  * `Null`: Unit/null value.
* `ObjectInstance`: Struct storing `class_name: String`, instance fields `fields: HashMap<String, Value>`, and reference to parent instance if inherited.
* `ControlFlow`: Internal state enum (`None`, `Return(Value)`, `Break`, `Continue`).
* `RuntimeError`: Struct encapsulating error `message` and source `Span` (line and column numbers).
* `Interpreter`: Primary VM struct maintaining global variables (`globals: HashMap<String, Value>`), static fields, class definitions, function definitions, and active `current_self` instance context.

### Functions & Methods:
* `Interpreter::new() -> Self`: Initializes the runtime environment and registers built-in standard functions.
* `Interpreter::register_builtins(&mut self)`: Inserts callable wrappers for `print`, `read_file`, `write_file`, `system_exec`, etc.
* `Interpreter::execute_program(&mut self, program: &Program) -> Result<Value, RuntimeError>`: Ingests AST, registers top-level functions and classes, and executes top-level statements or the `main()` function.
* `Interpreter::eval_statement(&mut self, stmt: &Statement, env: &mut HashMap<String, Value>) -> Result<ControlFlow, RuntimeError>`: Dispatches `let`, `if`, `while`, `for`, `return`, `expr`, and assignment statements.
* `Interpreter::eval_expression(&mut self, expr: &Expression, env: &mut HashMap<String, Value>) -> Result<Value, RuntimeError>`: Evaluates arithmetic operations, logical connectives, variable lookups, and function calls.
* `Interpreter::call_function(&mut self, func: &FunctionDecl, args: Vec<Value>) -> Result<Value, RuntimeError>`: Allocates a new local scope environment, binds parameters, and executes the function body.
* `Interpreter::instantiate_class(&mut self, name: &str, args: Vec<Value>) -> Result<Value, RuntimeError>`: Allocates a new `ObjectInstance`, links inherited parent classes, and calls `__init__`.

---

## 3. Communication with Low-Level Language Binaries

Even as a high-level AST interpreter, `mantrac_vm` interacts directly with native OS libraries and binaries:

1. **Host Operating System System Calls**:
   - Built-ins like `read_file` and `write_file` issue native file I/O system calls (`open(2)`, `read(2)`, `write(2)`, `close(2)`).
   - Built-in `system_exec` launches external OS processes via `std::process::Command`, executing native binaries and returning their exit codes.
   - Built-in `get_env` reads operating system environment block variables (`getenv(3)`).
2. **C ABI Native Memory Layouts**:
   - Native strings inside `Value::String` store continuous bytes in memory compatible with null-terminated C strings when passing to OS APIs.
   - Raw 64-bit integer (`i64`) and floating-point (`f64`) values match the native word size and register representation of the host architecture.
3. **Differential Ground Truth for Native Compilers**:
   - The outputs of native Cranelift JIT builds and C99 compiled binaries are systematically cross-checked against `mantrac_vm` results in automated regression suites to guarantee behavioral equivalence.

---

## 4. Hardware Interaction: CPU, ALU, and MU (Memory Unit)

### Central Processing Unit (CPU)
* **Virtual Machine Dispatch Loop**:
  - The interpreter loop repeatedly evaluates AST nodes via recursive pattern matching. On the host CPU, this compiles to nested indirect branches (computed goto / jump tables) matching `enum` discriminant integers.
* **Call Stack Mirroring**:
  - Every interpreted Mantra function call (`call_function`) creates a Rust stack frame on the host CPU stack. Local environment variables are held in a `HashMap` whose references live within that host frame.
* **Branch Prediction**:
  - The CPU branch predictor learns interpreter dispatch patterns. In tight loops (e.g., `while` loops evaluating identical condition expressions), branch prediction accuracy approaches 99%, keeping the CPU execution pipelines full.

### Arithmetic Logic Unit (ALU)
* **Mathematical & Logical Computation**:
  - When evaluating binary expressions (`eval_expression`):
    - `Value::Int(a) + Value::Int(b)` triggers the CPU ALU's signed 64-bit addition (`ADD rax, rbx`).
    - Bitwise logic (`&`, `|`, `^`) directly invokes hardware ALU logic gates (`AND`, `OR`, `XOR`).
    - Comparison expressions (`<`, `>`, `==`) use ALU arithmetic subtraction to update condition flags (`ZF`, `SF`, `OF`).
* **Numeric Type Coercion**:
  - Operations between `Int` and `Float` invoke hardware conversion instructions:
    - `CVTSI2SD` on x86_64: Converts a signed 64-bit integer in an integer register to a double-precision float in an `XMM` vector register.
    - `SCVTF` on ARM64: Converts signed integer in `Xn` to floating-point in `Dn`.

### Memory Unit (MU) / Memory Management Unit (MMU)
* **Heap Allocation via `Rc<RefCell<ObjectInstance>>`**:
  - Class instances are dynamically allocated on the OS virtual memory heap.
  - An `Rc` pointer stores a 64-bit pointer to a control block containing the strong reference count, weak reference count, and the `RefCell<ObjectInstance>` payload.
  - When reference count drops to zero (`DEC` to 0), the memory unit frees the heap chunk back to the allocator's free-list.
* **Cache Line Locality & Pointer Chasing**:
  - Primitive values (`Int`, `Float`, `Bool`) are stored inline inside the `Value` enum on the stack (16–24 bytes), fitting completely within a single 64-byte L1 cache line.
  - Object access navigates pointer indirections: `Rc` $\rightarrow$ `ObjectInstance` $\rightarrow$ `fields HashMap`. This demonstrates standard memory pointer dereferencing through L1/L2 data caches, incurring small cache misses when objects are spread across DRAM pages.
* **Virtual Memory Protection**:
  - All interpreter execution takes place strictly in data pages marked `PROT_READ | PROT_WRITE`. No dynamic code is written to memory, ensuring full compatibility with locked-down hardened OS kernels where `PROT_EXEC` heap allocation is forbidden.
