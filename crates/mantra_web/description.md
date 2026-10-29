# mantra_web — Native Web Application Runtime & Hardware Interaction

## 1. Package Overview & Purpose

`mantra_web` is the native web server and HTTP application runtime library for the Mantra programming language. Exposed to user applications via the standard library import `import mantra.web`, it provides the low-level foundation for building performant web applications, APIs, and microservices directly on top of the Mantra native runtime.

### Core Responsibilities:
* **HTTP Server Core (`Server`, `ServerConfig`)**: Manages TCP socket binding, connection acceptance, thread pools, and worker lifecycle.
* **HTTP Routing (`Router`, `Route`, `HttpMethod`)**: URL pattern matching, HTTP verb dispatch (`GET`, `POST`, `PUT`, `DELETE`, `PATCH`), and path parameter extraction.
* **HTTP Protocol Primitives (`Request`, `Response`)**: Headers, cookies, query parameters, multipart form parsing, status codes, and streaming response builders.
* **Middleware Pipeline (`MiddlewareChain`)**: Request interception, structured logging, CORS handling, authentication guards, and exception recovery.
* **Templating Engine (`TemplateEngine`)**: Dynamic HTML page rendering with variable interpolation.

---

## 2. Modules, Data Structures & Functions Used

### Key Modules:
* [`src/server.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_web/src/server.rs): Core HTTP server lifecycle and worker scheduling.
* [`src/router.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_web/src/router.rs): URL router and route prefix nesting.
* [`src/request.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_web/src/request.rs): Inbound HTTP request parsing and parameter extraction.
* [`src/response.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_web/src/response.rs): HTTP response construction and MIME type serialization.
* [`src/middleware.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_web/src/middleware.rs): Pipeline chains, CORS, and logging handlers.
* [`src/template.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_web/src/template.rs): HTML template rendering.

### Primary Functions:
* `Server::new(config: ServerConfig) -> Self`: Initializes server with worker count and socket address.
* `Server::mount(self, prefix: &str, router: Router) -> Self`: Mounts sub-routers at specified URL prefixes.
* `Router::add_route(&mut self, method: HttpMethod, path: &str, handler: ...)`: Registers path endpoints.
* `ResponseBuilder::status(code: StatusCode) -> Self`: Sets HTTP status code and prepares payload buffers.

---

## 3. Communication with Low-Level Language Binaries

`mantra_web` connects high-level web APIs to native OS networking binaries:

1. **POSIX Socket System Calls**:
   - Interacts with operating system networking APIs: `socket(2)`, `bind(2)`, `listen(2)`, `accept4(2)`, `send(2)`, `recv(2)`.
   - Native binaries link to socket libraries (`libc` on Linux/macOS; `Ws2_32.dll` on Windows).
2. **OS Non-Blocking Event Loops (epoll / kqueue / IOCP)**:
   - For high-concurrency scaling, socket file descriptors are registered with OS kernel notification multiplexers:
     - Linux: `epoll_create1(2)`, `epoll_ctl(2)`, `epoll_wait(2)`.
     - macOS / BSD: `kqueue(2)`, `kevent(2)`.
     - Windows: I/O Completion Ports (`CreateIoCompletionPort`).
3. **TCP/IP Stack Handshake**:
   - Transmits and receives binary TCP packets over loopback or Ethernet hardware interfaces, initiating TCP 3-way handshakes (`SYN`, `SYN-ACK`, `ACK`).

---

## 4. Hardware Interaction: CPU, ALU, and MU (Memory Unit)

### Central Processing Unit (CPU)
* **Worker Thread Scheduling Across Multi-Core Hardware**:
  - `ServerConfig.workers` configures OS worker threads. The OS scheduler distributes these threads across physical CPU cores (e.g., Core 0 through Core 7).
* **Hardware Context Switches & Interrupts**:
  - Incoming network packets trigger Network Interface Card (NIC) hardware interrupts, which the CPU handles via driver Interrupt Service Routines (ISRs) before waking waiting server worker threads.

### Arithmetic Logic Unit (ALU)
* **URL Routing & Path Hash Calculation**:
  - Routing tables compute fast string hashes and path segment comparisons in the ALU.
* **HTTP Header Parsing & Length Calculations**:
  - Parsing `Content-Length` headers involves ASCII-to-integer conversion loops using ALU multiplication and additions (`* 10 + digit`).

### Memory Unit (MU) / Memory Management Unit (MMU)
* **Zero-Copy Network Buffering**:
  - Static file responses leverage OS zero-copy system calls (`sendfile(2)`), which transfer data directly from page cache RAM to the NIC buffer without copying bytes into userspace CPU cache.
* **Ring Buffers & Memory Alignment**:
  - Incoming HTTP byte streams are read into contiguous memory buffers. 4 KiB and 64 KiB buffer sizes match hardware MMU page frame boundaries, preventing TLB thrashing during heavy traffic loads.
