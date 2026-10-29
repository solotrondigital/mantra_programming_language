# mantrac_ast — Abstract Syntax Tree Data Structures & Hardware Interaction

## 1. Package Overview & Purpose

`mantrac_ast` defines the canonical Abstract Syntax Tree (AST) data models and type representations for the Mantra programming language. It serves as the shared structural data contract across the entire compiler pipeline:
* Built by `mantrac_parser`.
* Analyzed and typed by `mantrac_checker`.
* Lowered into intermediate representation by `mantrac_ir`.
* Transpiled to C99 code by `mantrac_codegen`.
* Directly interpreted by `mantrac_vm`.

---

## 2. Modules, Data Structures & Functions Used

### Key Modules:
* [`src/ast.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantrac_ast/src/ast.rs): Core AST node definitions for declarations, statements, and expressions.
* [`src/types.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantrac_ast/src/types.rs): Type system representation, type variance, and type equality.
* [`src/lib.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantrac_ast/src/lib.rs): Public exports.

### Data Structures & Enums:
* `Program`: Root AST node containing `declarations: Vec<Declaration>`.
* `Declaration`:
  * `Class(ClassDecl)`: Name, base class (`extends`), members, decorators, visibility.
  * `Function(FunctionDecl)`: Name, parameters, return type, body statements, `is_async`, decorators.
  * `Variable(VarDecl)`: `let` or `mut` global binding.
* `ClassMember`:
  * `Field(FieldDecl)`: Instance field name, type, default value, visibility.
  * `StaticField(FieldDecl)`: Class-level static field.
  * `Method(FunctionDecl)`: Instance method.
  * `StaticMethod(FunctionDecl)`: Class static method.
* `Statement`:
  * `Let(VarDecl)`, `Assignment { target, value }`.
  * `If { condition, then_branch, elif_branches, else_branch }`.
  * `While { condition, body }`, `For { var_name, iterable, body }`.
  * `Return(Option<Expression>)`, `Break`, `Continue`, `Pass`.
  * `Expr(Expression)`: Standalone expression statement.
* `Expression`:
  * `Literal(LiteralExpr)`: `Int(i64)`, `Float(f64)`, `Bool(bool)`, `String(String)`, `NoneVal`.
  * `Identifier(String)`.
  * `Binary { left: Box<Expression>, op: BinaryOp, right: Box<Expression> }`.
  * `Unary { op: UnaryOp, expr: Box<Expression> }`.
  * `Call { callee: Box<Expression>, args: Vec<Expression> }`.
  * `MemberAccess { object: Box<Expression>, member: String }`.
  * `Index { target: Box<Expression>, index: Box<Expression> }`.
  * `Await(Box<Expression>)`.
* `Type`: `Int`, `Float`, `Bool`, `String`, `Void`, `List(Box<Type>)`, `Dict(Box<Type>, Box<Type>)`, `Custom(String)`, `Any`.

---

## 3. Communication with Low-Level Language Binaries

The AST directly reflects the semantic structure that dictates low-level binary layout:

1. **Memory Layout Specifications**:
   - `ClassDecl` nodes define the sequence of fields that determine struct offset layouts (`offsetof(struct, field)`) in compiled machine code.
2. **Virtual Method Tables (vtable)**:
   - Method hierarchies in `ClassDecl` define the function pointer array (vtable) emitted into binary read-only data segments (`.rodata`).
3. **Symbol Export Tables**:
   - Top-level `FunctionDecl` and `VarDecl` nodes with public visibility (`Visibility::Public`) specify exported symbol names written into ELF/Mach-O dynamic symbol tables (`.dynsym` / `__LINKEDIT`).

---

## 4. Hardware Interaction: CPU, ALU, and MU (Memory Unit)

### Central Processing Unit (CPU)
* **Polymorphic Tree Traversal**:
  - The AST is an algebraic data type (Rust `enum`). Traversing AST nodes (in type-checking, IR lowering, or interpretation) involves CPU pattern matching dispatch.
  - Rust optimizes enum matching using discriminant comparison instructions (`CMP [rdi], <discriminant>`) followed by conditional jumps or jump table lookups (`JMP [rax + rbx*8]`).
* **Hardware Instruction Pointer & Call Frames**:
  - Recursive AST visitors create regular call patterns on the CPU execution stack, maintaining localized instruction working sets within L1 Instruction Cache.

### Arithmetic Logic Unit (ALU)
* **BinaryOp Representation**:
  - `BinaryOp` variants (`Add`, `Sub`, `Mul`, `Div`, `Mod`, `BitAnd`, `BitOr`, `BitXor`, `Shl`, `Shr`) directly correspond to specific CPU ALU operational primitives.
  - When the compiler translates a `Binary` AST node, it assigns the exact matching ALU opcode to the target machine instruction.

### Memory Unit (MU) / Memory Management Unit (MMU)
* **Pointer Graph in Virtual Memory**:
  - Recursive expressions use `Box<Expression>`, which represents an owned 64-bit virtual memory address pointer allocated on the heap.
  - Dereferencing `*left` loads the address into a CPU register (`mov rax, [rbp - 16]`) and issues a memory read (`mov rbx, [rax]`), pulling the target node through the CPU cache hierarchy.
* **Cache Miss Mitigation**:
  - AST nodes are allocated linearly during parsing, giving sibling nodes high memory proximity (spatial locality) in adjacent cache lines.
