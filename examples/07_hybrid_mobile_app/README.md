# Example 07: Mantra Hybrid Mobile Application

This example demonstrates a cross-platform mobile application built with the **Mantra Mobile Application Platform (v0.5.0)**. It shares core data models, validation logic, and API clients across Web, Android, and iOS using a secure WebView hybrid bridge.

---

## Architecture Overview

```
┌────────────────────────────────────────────────────────┐
│                   WebView UI Layer                     │
│   HTML5 / CSS3 / JavaScript Mobile Client (www/)       │
└───────────────────────────┬────────────────────────────┘
                            │ window.MantraBridge.send()
                            ▼
┌────────────────────────────────────────────────────────┐
│         Mantra Native Bridge (Android & iOS)           │
│   - Action Whitelisting & Origin Authorization         │
│   - Capability Enforcement (Camera, Storage, Network)  │
│   - JSON-RPC Asynchronous Message Dispatch            │
└─────────────┬────────────────────────────┬─────────────┘
              │                            │
              ▼                            ▼
┌───────────────────────────┐┌───────────────────────────┐
│     Android Subsystem     ││       iOS Subsystem       │
│ • Kotlin MantraBridge     ││ • Swift MantraBridge      │
│ • Android KeyStore        ││ • Apple Secure Enclave    │
│ • App Lifecycle (Activity)││ • App Lifecycle (Scene)   │
│ • OkHttp Native Proxy     ││ • URLSession Native Proxy │
└───────────────────────────┘└───────────────────────────┘
```

---

## Directory Structure

```
07_hybrid_mobile_app/
├── mantra.toml                   # Project manifest & mobile configurations
├── src/
│   ├── main.mantra               # Mobile business logic service
│   └── shared/
│       ├── models.mantra         # Shared entity models (User, TaskItem)
│       ├── validation.mantra     # Shared validation logic (Email, Password)
│       └── api_client.mantra     # Shared HTTP API client
├── www/
│   ├── index.html                # Mobile user interface
│   ├── style.css                 # Native mobile styling
│   └── app.js                    # JavaScript bridge client
├── android/
│   ├── build.gradle.kts          # Gradle build script
│   └── app/
│       └── src/main/
│           ├── AndroidManifest.xml
│           ├── java/com/mantra/.../
│           │   ├── MainActivity.kt
│           │   └── MantraBridge.kt
│           └── res/xml/network_security_config.xml
├── ios/
│   └── MantraApp/
│       ├── Info.plist            # Permissions and ATS settings
│       ├── MantraBridge.swift    # Swift WKScriptMessageHandler bridge
│       └── ViewController.swift  # WKWebView controller
└── tests/
    └── test_mobile.mantra        # Shared test suite
```

---

## Development & Testing

### 1. Run Shared Logic Tests
```bash
mantra test
```

### 2. Verify Mobile Bridge Conformance
```bash
mantra mobile test
```

### 3. Build for Android
Synchronizes web assets into Android project and validates security policies:
```bash
mantra mobile build android
```

**Debug Deployment:**
```bash
cd android && ./gradlew assembleDebug
adb install app/build/outputs/apk/debug/app-debug.apk
```

### 4. Build for iOS
Synchronizes web assets into iOS project and configures App Transport Security:
```bash
mantra mobile build ios
```

**Debug Deployment:**
```bash
open ios/*.xcodeproj
# Or build via command line:
xcodebuild -project ios/*.xcodeproj -scheme App -destination 'generic/platform=iOS'
```

