# mantra_mobile — Hybrid Mobile Runtime, Bridge Protocol & Hardware Interaction

## 1. Package Overview & Purpose

`mantra_mobile` is the native mobile runtime and hybrid bridge subsystem for the Mantra programming language. Introduced in Mantra v0.5.0, it allows Mantra application developers to write cross-platform mobile applications for **Android** and **iOS** using a hybrid architecture:
* **UI Layer**: High-performance, portable WebView UI (HTML5, modern CSS, WebAssembly UI components).
* **Business Logic Layer**: Shared Mantra business logic, data models, validation services, and API clients compiled into WebAssembly or native shared libraries (`.so` on Android, `.dylib`/framework on iOS).
* **Native Capability Bridge**: A secure, capability-based asynchronous messaging bridge connecting the WebView execution context to native device hardware, lifecycle events, secure storage, and network stacks.

---

## 2. Modules, Data Structures & Functions Used

### Key Modules:
* [`src/bridge.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mobile/src/bridge.rs): JSON-RPC style asynchronous request/response protocol (`BridgeRequest`, `BridgeResponse`, `BridgeRouter`).
* [`src/security.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mobile/src/security.rs): Granular capability-based access control (`Capability`, `SecurityPolicy`), origin validation, URL allowlists, and Content Security Policy (CSP) enforcement.
* [`src/lifecycle.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mobile/src/lifecycle.rs): Cross-platform mobile lifecycle state machine (`AppLifecycleState`, `LifecycleEvent`, `LifecycleManager`).
* [`src/storage.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mobile/src/storage.rs): Abstract storage tier (`StorageTier::Standard`, `StorageTier::Secure`) backed by Android SharedPreferences/KeyStore and iOS UserDefaults/Keychain.
* [`src/networking.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mobile/src/networking.rs): Managed network proxy enforcing HTTPS, App Transport Security (ATS), and cleartext traffic blocking.
* [`src/permissions.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mobile/src/permissions.rs): Unified permissions manager for Camera, Location, Notifications, Storage, and Microphone.
* [`src/android.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mobile/src/android.rs): Kotlin native bridge template (`MantraBridge.kt`), `MainActivity.kt`, `AndroidManifest.xml`, and Gradle build scripts.
* [`src/ios.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mobile/src/ios.rs): Swift native bridge template (`MantraBridge.swift`), `ViewController.swift`, `Info.plist`, and Xcode project specifications.
* [`src/lib.rs`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/crates/mantra_mobile/src/lib.rs): Public API and `MobileRuntime` coordinator.

---

## 3. Communication with Low-Level Language Binaries

`mantra_mobile` directly interfaces with native mobile platform runtimes and OS binaries:

1. **Android Runtime (ART) / JNI & `@JavascriptInterface`**:
   - The Android WebView attaches an interface object to the JavaScript `window` object via `WebView.addJavascriptInterface()`.
   - Incoming JSON messages are received via JNI (Java Native Interface) boundaries, dispatched onto background threads, evaluated by the Mantra runtime, and returned via `webView.evaluateJavascript()`.
2. **iOS WebKit (`WKScriptMessageHandler` & JavaScriptCore)**:
   - On iOS, communication utilizes Apple's high-performance IPC boundary:
     ```javascript
     window.webkit.messageHandlers.mantraBridge.postMessage(jsonString);
     ```
   - Ingested by `MantraBridge.swift` inside `userContentController(_:didReceive:)` and returned via `WKWebView.evaluateJavaScript()`.
3. **Native Cryptographic Hardware (KeyStore / Secure Enclave)**:
   - `StorageTier::Secure` requests delegate to hardware security modules (HSMs)—the Android KeyStore (TEE / StrongBox) and Apple Secure Enclave (AES-256 GCM hardware encryption).

---

## 4. Hardware Interaction: CPU, ALU, and MU (Memory Unit)

### Central Processing Unit (CPU)
* **Heterogeneous Multi-Core Scheduling (big.LITTLE / DynamIQ)**:
  - Mobile processors (Qualcomm Snapdragon, Apple A-series / M-series, Google Tensor) feature asymmetric CPU cores: High-efficiency LITTLE cores (e.g. Cortex-A55 / Blizzard) and High-performance big cores (e.g. Cortex-X4 / Firestorm).
  - Background bridge message parsing and storage I/O run on secondary background threads scheduled by the OS onto efficiency cores, conserving battery life while keeping the main UI thread at 60/120 Hz.
* **Context Switching between WebKit and Native Threads**:
  - Asynchronous bridge messages invoke lightweight OS IPC context switches, saving and restoring thread registers with sub-millisecond dispatch latency.

### Arithmetic Logic Unit (ALU)
* **Message Boundary & Length Validation**:
  - The ALU enforces `max_payload_bytes` limits (e.g. 512 KiB) using unsigned 64-bit integer comparison instructions (`CMP`, `JBE`), shielding the native runtime from memory exhaustion attacks.
* **Hash Calculation for Permission Registry**:
  - Looking up permission states and action handlers computes SipHash / AHash digests via ALU integer multiplication, bitwise rotations, and XOR operations.

### Memory Unit (MU) / Memory Management Unit (MMU)
* **Isolated Memory Sandboxes**:
  - The WebView JavaScript engine (V8 on Android, Nitro/JavaScriptCore on iOS) runs in a separate, isolated OS process sandbox.
  - Memory pointers inside the WebView cannot directly read or write native app memory; communication must cross the structured string serialization boundary, enforcing hardware-enforced MMU isolation.
* **Low Memory Warnings & Cache Purging**:
  - When the physical DRAM fills up, the mobile OS issues hardware low-memory events (`onLowMemory()` on Android, `didReceiveMemoryWarning` on iOS).
  - The `LifecycleManager` intercepts these events, triggering memory cache evictions and reducing page table pressure to prevent OS Out-Of-Memory (OOM) killer terminations.

