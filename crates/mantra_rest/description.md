# mantra_rest — REST API Framework, JSON Serialization & Hardware Interaction

## 1. Package Overview & Purpose

`mantra_rest` is the native REST API client and server framework for the Mantra programming language. Exposed to developers via the standard library package `import mantra.rest`, it builds upon `mantra_web` to provide high-level abstractions for designing RESTful microservices, JSON serialization, pagination, and error handling.

### Core Responsibilities:
* **REST Controllers (`RestController`, `CrudController`)**: Standardized mapping of resource entities to HTTP verbs (`GET` for read, `POST` for create, `PUT`/`PATCH` for update, `DELETE` for removal).
* **Native JSON Serialization (`JsonSerializer`, `JsonValue`)**: Zero-dependency JSON parser and serializer converting between native Mantra data structures and RFC 8259 JSON strings.
* **Standardized API Responses (`ApiResponse`, `ApiError`, `Pagination`)**: Consistent JSON envelopes for successful payloads, error codes, validation errors, and cursor/page-based metadata.
* **Route Registry (`RouteRegistry`, `RouteHandler`)**: Decorator-driven route dispatch and handler execution.

---

## 2. Modules, Data Structures & Functions Used

### Key Modules:
* [`src/controller.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_rest/src/controller.rs): Base controllers and CRUD abstractions.
* [`src/json.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_rest/src/json.rs): JSON parser, serializer, and value AST.
* [`src/api_response.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_rest/src/api_response.rs): Structured HTTP response models and pagination envelopes.
* [`src/route.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_rest/src/route.rs): Dynamic route registry and handler invocation.

### Key Data Structures:
* `JsonValue`: `Null`, `Bool(bool)`, `Number(f64)`, `String(String)`, `Array(Vec<JsonValue>)`, `Object(HashMap<String, JsonValue>)`.
* `ApiResponse<T>`: `success: bool`, `data: Option<T>`, `error: Option<ApiError>`, `pagination: Option<Pagination>`.
* `CrudController`: Trait specifying `index`, `show(id)`, `create(body)`, `update(id, body)`, `delete(id)`.

### Primary Functions:
* `JsonSerializer::serialize(val: &JsonValue) -> String`: Converts memory trees to compact or indented JSON byte streams.
* `JsonSerializer::parse(json_str: &str) -> Result<JsonValue, String>`: Recursive descent parser converting JSON text into memory nodes.
* `ApiResponse::ok(data: T) -> Self`: Builds 200 OK responses with enveloped data.
* `ApiResponse::error(status: u16, msg: &str) -> Self`: Formats RFC 7807 problem details payloads.

---

## 3. Communication with Low-Level Language Binaries

1. **Serialization into Raw Byte Buffers**:
   - JSON serialization writes raw UTF-8 byte arrays directly into memory buffers passed to the socket layer (`write(2)` / `send(2)`).
2. **Dynamic C Library Linkage**:
   - High-throughput JSON parsing leverages SIMD instructions (e.g., SIMD-JSON techniques) when compiled down to native machine code via `mantrac_codegen`, utilizing vector registers to scan for structural delimiters (`"`, `:`, `,`, `{`, `}`).

---

## 4. Hardware Interaction: CPU, ALU, and MU (Memory Unit)

### Central Processing Unit (CPU)
* **SIMD Vector Scanning (SSE4.2 / AVX2 / NEON)**:
  - Scanning JSON strings for structural characters (`{`, `}`, `[`, `]`, `"`, `\`) processes 16 bytes (SSE/NEON) or 32 bytes (AVX2) in a single CPU cycle using vector comparison instructions (`PCMPEQB` / `VPCMPEQB`).
* **Branch Prediction in Recursive Descent**:
  - Predicts token transitions when deserializing repetitive JSON arrays, keeping CPU instruction fetch pipelines active.

### Arithmetic Logic Unit (ALU)
* **Numeric Parsing & Conversion**:
  - JSON numbers with exponents (`1.25e-4`) are converted into 64-bit IEEE 754 floats via ALU integer scaling and floating-point multiply-accumulate operations (`FMADD` / `FMA3`).
* **Pagination Boundary Arithmetic**:
  - Calculates pagination offsets:
    $$\text{offset} = (\text{page} - 1) \times \text{limit}$$
    $$\text{total\_pages} = \left\lceil \frac{\text{total\_count}}{\text{limit}} \right\rceil$$
  - Executed using fast ALU integer arithmetic and division instructions.

### Memory Unit (MU) / Memory Management Unit (MMU)
* **String Allocation & Arena Memory**:
  - Deserializing large JSON documents creates hundreds of small heap strings. Memory allocators cluster these allocations in contiguous heap arenas, minimizing memory fragmentation.
* **Cache Line Packing**:
  - The `JsonValue` tagged enum is optimized in memory to occupy 24–32 bytes, enabling two instances to reside within a single 64-byte L1 Data Cache line.
