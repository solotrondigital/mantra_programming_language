# mantra_sql_orm — Database ORM, Query Builder & Hardware Interaction

## 1. Package Overview & Purpose

`mantra_sql_orm` is the native Object-Relational Mapping (ORM) and database persistence library for the Mantra programming language. Exposed to user applications via `import mantra.sql.orm`, it enables type-safe schema modeling, dynamic SQL query construction, migration lifecycle tracking, and database adapter abstraction across SQLite, PostgreSQL, and MySQL.

### Core Responsibilities:
* **Active Record & Data Mapping (`Model`, `Field`)**: Maps Mantra class instances to relational database tables with automatic primary key and field mapping.
* **Type-Safe Query Builder (`QueryBuilder`, `WhereClause`, `OrderBy`)**: Fluent SQL query synthesis supporting parameterized queries to prevent SQL injection vulnerabilities.
* **Pluggable Database Adapters (`DatabaseAdapter`)**: Native client drivers for SQLite (embedded), PostgreSQL (wire protocol), and MySQL.
* **Schema Migrations Engine (`Migration`, `MigrationRunner`, `SchemaBuilder`)**: Programmatic schema definition and versioned database schema migration tracking.

---

## 2. Modules, Data Structures & Functions Used

### Key Modules:
* [`src/model.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_sql_orm/src/model.rs): Data model definitions, field metadata, and value containers.
* [`src/query.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_sql_orm/src/query.rs): Composable SQL query constructor.
* [`src/adapters/`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_sql_orm/src/adapters): Database drivers (`sqlite.rs`, `postgres.rs`, `mysql.rs`).
* [`src/migration.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_sql_orm/src/migration.rs): Schema migration runner and DDL generator.

### Key Data Structures:
* `Model`: Represents a database entity table with fields and primary key.
* `FieldType`: `Integer`, `Float`, `VarChar(usize)`, `Text`, `Boolean`, `DateTime`.
* `FieldValue`: `Int(i64)`, `Float(f64)`, `Text(String)`, `Bool(bool)`, `Null`.
* `QueryBuilder`: Constructing `SELECT`, `INSERT`, `UPDATE`, `DELETE` operations with `WHERE`, `ORDER BY`, `LIMIT`, and `OFFSET`.
* `DatabaseAdapter`: Trait defining `connect()`, `execute(query, params)`, `query(sql)`, and `transaction()`.

### Primary Functions:
* `QueryBuilder::table(name: &str) -> Self`: Starts building a query for a table.
* `QueryBuilder::where_eq(column: &str, val: FieldValue) -> Self`: Appends parameterized equality condition.
* `DatabaseAdapter::execute(&self, sql: &str) -> Result<u64, String>`: Sends SQL statement to database engine.
* `MigrationRunner::apply_all(&self) -> Result<usize, String>`: Runs pending versioned migrations in transaction blocks.

---

## 3. Communication with Low-Level Language Binaries

`mantra_sql_orm` interfaces directly with low-level database engine libraries:

1. **Embedded C Engine FFI (`libsqlite3`)**:
   - For SQLite, the adapter calls directly into the native SQLite C library (`libsqlite3.so` / `libsqlite3.dylib` / `sqlite3.dll`) using foreign function interfaces:
     - `sqlite3_open_v2()`
     - `sqlite3_prepare_v2()`
     - `sqlite3_step()`
     - `sqlite3_column_int64()`
     - `sqlite3_finalize()`
     - `sqlite3_close()`
2. **Binary Wire Protocol Serialization (PostgreSQL / MySQL)**:
   - Formats network packets directly according to database wire protocol specifications (e.g., PostgreSQL Frontend/Backend Protocol 3.0), serializing parameter lengths and byte arrays sent over TCP sockets.
3. **OS File Descriptor I/O**:
   - SQLite reads and writes directly to local disk database files using OS system calls (`pwrite(2)`, `pread(2)`, `fsync(2)`).

---

## 4. Hardware Interaction: CPU, ALU, and MU (Memory Unit)

### Central Processing Unit (CPU)
* **Query Serialization & Buffer Packing**:
  - The query builder assembles SQL strings by appending byte slices into pre-allocated memory buffers, optimizing CPU instruction pipelines with minimal reallocation.
* **Binary Column Unpacking**:
  - Unpacking binary database row columns into Mantra `FieldValue` objects executes direct register load operations (`MOV` into general-purpose registers for integers, `MOVSD` into `XMM` registers for floats).

### Arithmetic Logic Unit (ALU)
* **Parameter Index & Offset Calculation**:
  - SQLite parameter bindings compute 1-based parameter indexes (`?1`, `?2`) and memory buffer offset math in the ALU.
* **Numeric Boundary Checks**:
  - Verifies numeric ranges and string length limits (`VarChar(255)`) using fast ALU integer comparison instructions (`CMP`, `JBE`).

### Memory Unit (MU) / Memory Management Unit (MMU)
* **Direct Memory-Mapped I/O (mmap)**:
  - SQLite uses the OS MMU to memory-map database pages (`mmap(2)`). Reading database records accesses virtual memory pages directly cached in physical DRAM, reducing disk latency to CPU memory bus speeds (~100 nanoseconds).
* **Disk Write Cache Flushes (fsync)**:
  - Committing SQL transactions executes `fsync(2)`, issuing hardware cache flush commands down the PCIe / NVMe bus to ensure persistent NAND flash cell writes.
* **Row Memory Allocation**:
  - Query result sets allocate memory for record rows. Columns are stored in contiguous memory layouts to maximize CPU cache line re-use when iterating over large result sets.
