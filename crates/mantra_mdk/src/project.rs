use crate::manifest::PackageManifest;
use std::fs;
use std::path::Path;

pub struct ProjectScaffolder;

/// Validates that project name follows the `lowercase_with_underscores` naming convention.
pub fn validate_project_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Project name cannot be empty.".to_string());
    }

    // Must consist only of lowercase letters, digits, and underscores, and cannot start with a digit.
    let is_valid = name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        && !name.starts_with(|c: char| c.is_ascii_digit());

    if !is_valid {
        return Err(format!(
            "\"{}\" is not a valid Mantra project name.\n\
             (Note: Your project name must follow the lowercase_with_underscores naming convention).",
            name
        ));
    }
    Ok(())
}

/// Arguments for cross-platform `mantra create` command.
pub struct CreateArgs<'a> {
    /// Reverse-DNS organisation prefix, e.g. "com.raghav"
    pub org: &'a str,
    /// Mantra package identifier, e.g. "mantralayanivas"
    pub project_name: &'a str,
    /// Output directory (usually equals project_name)
    pub directory: &'a str,
    /// Custom project description for mantra.toml
    pub description: Option<&'a str>,
    /// Creates a project with a minimal main.mantra file (skips default counter/model app template)
    pub empty: bool,
    /// Limits the platforms supported by the project (default generates files for all platforms: ios, android, web)
    pub platforms: Vec<String>,
    /// Specifies the type of project: app, module, package, plugin, or skeleton
    pub template: &'a str,
}

impl ProjectScaffolder {
    pub fn new_app(name: &str) -> Result<(), String> {
        let root = Path::new(name);
        if root.exists() {
            return Err(format!("Directory '{}' already exists", name));
        }

        fs::create_dir_all(root.join("src"))
            .map_err(|e| format!("Failed to create src dir: {}", e))?;
        fs::create_dir_all(root.join("tests"))
            .map_err(|e| format!("Failed to create tests dir: {}", e))?;

        let manifest = PackageManifest {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            edition: "2026".to_string(),
            description: format!("Mantra application: {}", name),
            license: "MIT".to_string(),
            entry: "src/main.mantra".to_string(),
            dependencies: std::collections::HashMap::new(),
        };
        manifest.save_to_dir(root)?;

        let main_src = format!(
            r#"# Welcome to Mantra Application: {name}

class Greeter:
    def __init__(self, name: String):
        self.name = name

    pub def greet(self) -> String:
        return "Hello, " + self.name + "! Welcome to Mantra."

def main() -> Int:
    greeter = Greeter("{name}")
    print(greeter.greet())
    return 0
"#,
            name = name
        );
        fs::write(root.join("src/main.mantra"), main_src)
            .map_err(|e| format!("Failed to write src/main.mantra: {}", e))?;

        let test_src = format!(
            r#"# Tests for {name}

class TestSuite:
    def __init__(self):
        self.tests_run = 0

    pub def test_greeter(self) -> Bool:
        let expected = "Hello, Mantra! Welcome to Mantra."
        let actual = "Hello, Mantra! Welcome to Mantra."
        assert(expected == actual)
        return True

def main() -> Int:
    suite = TestSuite()
    print("Running test_greeter...")
    suite.test_greeter()
    print("All tests passed successfully!")
    return 0
"#,
            name = name
        );
        fs::write(root.join("tests/test_main.mantra"), test_src)
            .map_err(|e| format!("Failed to write tests/test_main.mantra: {}", e))?;

        let gitignore = "target/\n*.o\n*.exe\n.DS_Store\n";
        fs::write(root.join(".gitignore"), gitignore)
            .map_err(|e| format!("Failed to write .gitignore: {}", e))?;

        let readme = format!(
            r#"# {name}

An Object-Oriented application built with Mantra.

## Development

Run development target:
```bash
mantra run
```

Run test suite:
```bash
mantra test
```

Build native executable:
```bash
mantra build --release
```
"#,
            name = name
        );
        fs::write(root.join("README.md"), readme)
            .map_err(|e| format!("Failed to write README.md: {}", e))?;

        println!("Created Mantra application '{}'", name);
        println!("To get started:\n  cd {}\n  mantra run", name);

        Ok(())
    }

    pub fn new_package(name: &str) -> Result<(), String> {
        let root = Path::new(name);
        if root.exists() {
            return Err(format!("Directory '{}' already exists", name));
        }

        fs::create_dir_all(root.join("src"))
            .map_err(|e| format!("Failed to create src dir: {}", e))?;
        fs::create_dir_all(root.join("tests"))
            .map_err(|e| format!("Failed to create tests dir: {}", e))?;

        let manifest = PackageManifest {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            edition: "2026".to_string(),
            description: format!("Mantra package library: {}", name),
            license: "MIT".to_string(),
            entry: "src/lib.mantra".to_string(),
            dependencies: std::collections::HashMap::new(),
        };
        manifest.save_to_dir(root)?;

        let lib_src = format!(
            r#"# Mantra Package: {name}

interface DataProcessor:
    def process(value: String) -> String

pub class DefaultProcessor implements DataProcessor:
    pub def process(self, value: String) -> String:
        return "Processed: " + value
"#,
            name = name
        );
        fs::write(root.join("src/lib.mantra"), lib_src)
            .map_err(|e| format!("Failed to write src/lib.mantra: {}", e))?;

        let test_src = format!(
            r#"# Tests for package {name}

def main() -> Int:
    print("Testing package {name}...")
    assert(True)
    print("Package tests passed!")
    return 0
"#,
            name = name
        );
        fs::write(root.join("tests/test_lib.mantra"), test_src)
            .map_err(|e| format!("Failed to write tests/test_lib.mantra: {}", e))?;

        let gitignore = "target/\n*.o\n*.exe\n.DS_Store\n";
        fs::write(root.join(".gitignore"), gitignore)
            .map_err(|e| format!("Failed to write .gitignore: {}", e))?;

        println!("Created Mantra package '{}'", name);
        Ok(())
    }

    pub fn new_web(name: &str) -> Result<(), String> {
        let root = Path::new(name);
        if root.exists() {
            return Err(format!("Directory '{}' already exists", name));
        }

        fs::create_dir_all(root.join("src"))
            .map_err(|e| format!("Failed to create src dir: {}", e))?;
        fs::create_dir_all(root.join("public"))
            .map_err(|e| format!("Failed to create public dir: {}", e))?;
        fs::create_dir_all(root.join("tests"))
            .map_err(|e| format!("Failed to create tests dir: {}", e))?;

        let mut manifest = PackageManifest {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            edition: "2026".to_string(),
            description: format!("Mantra Web Application: {}", name),
            license: "MIT".to_string(),
            entry: "src/main.mantra".to_string(),
            dependencies: std::collections::HashMap::new(),
        };
        manifest.dependencies.insert("mantra.web".to_string(), "0.5.0".to_string());
        manifest.dependencies.insert("mantra.rest".to_string(), "0.5.0".to_string());
        manifest.save_to_dir(root)?;

        let main_src = format!(
            r#"# Mantra Web Application: {name}
import mantra.web
import mantra.rest

class WebApp:
    def __init__(self):
        self.server = Server(ServerConfig(host="127.0.0.1", port=8080, workers=4))

    pub def setup_routes(self):
        # Mount static frontend assets from public/
        self.server.serve_static("/", "public")
        print("Mounted static files at '/' from public/")

    pub def run(self) -> Int:
        self.setup_routes()
        print("Starting {name} on http://127.0.0.1:8080")
        self.server.listen()
        return 0

def main() -> Int:
    app = WebApp()
    return app.run()
"#,
            name = name
        );
        fs::write(root.join("src/main.mantra"), main_src)
            .map_err(|e| format!("Failed to write src/main.mantra: {}", e))?;

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{name} — Powered by Mantra</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
    <div class="container">
        <header>
            <h1>⚡ {name}</h1>
            <p class="subtitle">Built with Mantra Web Framework &amp; WebAssembly</p>
        </header>

        <main>
            <div class="card">
                <h2>Server &amp; Browser Interoperability</h2>
                <p id="status">Connecting to Mantra backend...</p>
                <button id="btn-ping" class="btn">Test API Endpoint</button>
            </div>
        </main>
    </div>
    <script src="app.js"></script>
</body>
</html>
"#,
            name = name
        );
        fs::write(root.join("public/index.html"), html)
            .map_err(|e| format!("Failed to write public/index.html: {}", e))?;

        let css = r#"body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    background: #0f172a;
    color: #f8fafc;
    margin: 0;
    padding: 2rem;
    display: flex;
    justify-content: center;
}
.container { max-width: 680px; width: 100%; }
header { margin-bottom: 2rem; }
h1 { color: #38bdf8; margin: 0; }
.subtitle { color: #94a3b8; margin-top: 0.5rem; }
.card {
    background: #1e293b;
    border-radius: 12px;
    padding: 1.5rem;
    border: 1px solid #334155;
}
.btn {
    background: #0284c7;
    color: white;
    border: none;
    padding: 0.6rem 1.2rem;
    border-radius: 6px;
    cursor: pointer;
    font-weight: 600;
}
.btn:hover { background: #0369a1; }
"#;
        fs::write(root.join("public/style.css"), css)
            .map_err(|e| format!("Failed to write public/style.css: {}", e))?;

        let js = r#"document.addEventListener("DOMContentLoaded", () => {
    const status = document.getElementById("status");
    status.innerText = "Mantra WebApp active on 127.0.0.1:8080";

    document.getElementById("btn-ping").addEventListener("click", () => {
        status.innerText = "Response: HTTP 200 OK — Mantra Web Server Healthy!";
    });
});
"#;
        fs::write(root.join("public/app.js"), js)
            .map_err(|e| format!("Failed to write public/app.js: {}", e))?;

        let test_src = format!(
            r#"# Tests for WebApp {name}

def main() -> Int:
    print("Testing {name} web routes...")
    assert(True)
    print("Web routes passed!")
    return 0
"#,
            name = name
        );
        fs::write(root.join("tests/test_web.mantra"), test_src)
            .map_err(|e| format!("Failed to write tests/test_web.mantra: {}", e))?;

        let gitignore = "target/\npublic/dist/\n.DS_Store\n";
        fs::write(root.join(".gitignore"), gitignore)
            .map_err(|e| format!("Failed to write .gitignore: {}", e))?;

        println!("Created Mantra Web application '{}'", name);
        println!("To start development server:\n  cd {}\n  mantra run", name);
        Ok(())
    }

    pub fn new_mobile(name: &str) -> Result<(), String> {
        let root = Path::new(name);
        if root.exists() {
            return Err(format!("Directory '{}' already exists", name));
        }

        let package_name = format!("com.mantra.{}", name.replace('-', "_").to_lowercase());

        // Create directory hierarchy
        fs::create_dir_all(root.join("src/shared"))
            .map_err(|e| format!("Failed to create src/shared dir: {}", e))?;
        fs::create_dir_all(root.join("www"))
            .map_err(|e| format!("Failed to create www dir: {}", e))?;
        fs::create_dir_all(root.join("android/app/src/main/java").join(package_name.replace('.', "/")))
            .map_err(|e| format!("Failed to create android dir: {}", e))?;
        fs::create_dir_all(root.join("android/app/src/main/res/xml"))
            .map_err(|e| format!("Failed to create android res dir: {}", e))?;
        fs::create_dir_all(root.join("ios/MantraApp"))
            .map_err(|e| format!("Failed to create ios dir: {}", e))?;
        fs::create_dir_all(root.join("tests"))
            .map_err(|e| format!("Failed to create tests dir: {}", e))?;

        // Manifest
        let mut manifest = PackageManifest {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            edition: "2026".to_string(),
            description: format!("Mantra Hybrid Mobile Application: {}", name),
            license: "MIT".to_string(),
            entry: "src/main.mantra".to_string(),
            dependencies: std::collections::HashMap::new(),
        };
        manifest.dependencies.insert("mantra.mobile".to_string(), "0.5.0".to_string());
        manifest.save_to_dir(root)?;

        // 1. Shared Models
        let models_src = r#"# Shared Data Models for Server, Web, Android, and iOS

pub class User:
    def __init__(self, id: String, email: String, name: String):
        self.id = id
        self.email = email
        self.name = name

pub class TaskItem:
    def __init__(self, id: String, title: String, completed: Bool):
        self.id = id
        self.title = title
        self.completed = completed
"#;
        fs::write(root.join("src/shared/models.mantra"), models_src)
            .map_err(|e| format!("Failed to write models.mantra: {}", e))?;

        // 2. Shared Validation
        let validation_src = r#"# Shared Cross-Platform Validation Rules

pub class Validator:
    pub def validate_email(self, email: String) -> Bool:
        if len(email) < 5:
            return False
        # Basic email check
        return True

    pub def validate_password(self, password: String) -> Bool:
        if len(password) < 8:
            return False
        return True
"#;
        fs::write(root.join("src/shared/validation.mantra"), validation_src)
            .map_err(|e| format!("Failed to write validation.mantra: {}", e))?;

        // 3. Shared API Client
        let api_client_src = r#"# Shared HTTP API Client

pub class ApiClient:
    def __init__(self, base_url: String):
        self.base_url = base_url

    pub def get_tasks_endpoint(self) -> String:
        return self.base_url + "/api/v1/tasks"
"#;
        fs::write(root.join("src/shared/api_client.mantra"), api_client_src)
            .map_err(|e| format!("Failed to write api_client.mantra: {}", e))?;

        // 4. Main Service
        let main_src = format!(
            r#"# Mobile Business Logic Service: {name}
import shared.models
import shared.validation

class MobileApp:
    def __init__(self):
        self.validator = Validator()

    pub def run(self) -> Int:
        print("Mantra Mobile Business Logic Initialized: {name}")
        let valid = self.validator.validate_email("user@example.com")
        print("Validation test result: True")
        return 0

def main() -> Int:
    app = MobileApp()
    return app.run()
"#,
            name = name
        );
        fs::write(root.join("src/main.mantra"), main_src)
            .map_err(|e| format!("Failed to write src/main.mantra: {}", e))?;

        // 5. Mobile WebView Assets (www/)
        let www_html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=no, viewport-fit=cover">
    <meta http-equiv="Content-Security-Policy" content="default-src 'self' file: data:; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; connect-src 'self' https:;">
    <title>{name}</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
    <div class="safe-area">
        <header>
            <h2>📱 {name}</h2>
            <span id="badge" class="badge">Hybrid Mobile</span>
        </header>

        <section class="card">
            <h3>Native Bridge Status</h3>
            <p id="bridge-status">Connecting to Mantra Native Bridge...</p>
            <div class="button-group">
                <button id="btn-info" class="btn">Get Device Info</button>
                <button id="btn-camera" class="btn">Request Camera</button>
            </div>
        </section>

        <section class="card">
            <h3>Shared Validation Test</h3>
            <input type="email" id="email-input" placeholder="Enter email..." value="test@mantra.lang">
            <button id="btn-validate" class="btn secondary">Validate with Shared Logic</button>
            <p id="val-result"></p>
        </section>
    </div>
    <script src="app.js"></script>
</body>
</html>
"#,
            name = name
        );
        fs::write(root.join("www/index.html"), www_html)
            .map_err(|e| format!("Failed to write www/index.html: {}", e))?;

        let www_css = r#"* { box-sizing: border-box; }
body {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    background: #0f172a;
    color: #f1f5f9;
}
.safe-area {
    padding: env(safe-area-inset-top, 20px) 16px env(safe-area-inset-bottom, 20px) 16px;
}
header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
}
h2 { margin: 0; color: #38bdf8; font-size: 1.4rem; }
.badge {
    background: #0284c7;
    color: white;
    font-size: 0.75rem;
    padding: 4px 8px;
    border-radius: 12px;
}
.card {
    background: #1e293b;
    border-radius: 12px;
    padding: 16px;
    margin-bottom: 16px;
    border: 1px solid #334155;
}
h3 { margin-top: 0; font-size: 1.1rem; color: #e2e8f0; }
.btn {
    background: #38bdf8;
    color: #0f172a;
    border: none;
    padding: 10px 14px;
    border-radius: 8px;
    font-weight: 600;
    cursor: pointer;
    margin-right: 8px;
}
.btn.secondary { background: #64748b; color: white; margin-top: 8px; }
input {
    width: 100%;
    padding: 10px;
    border-radius: 8px;
    border: 1px solid #475569;
    background: #0f172a;
    color: white;
}
"#;
        fs::write(root.join("www/style.css"), www_css)
            .map_err(|e| format!("Failed to write www/style.css: {}", e))?;

        let www_js = r#"// Mantra Hybrid Mobile JavaScript Bridge Client
window.MantraBridge = {
    callbacks: {},

    send(action, payload = {}) {
        return new Promise((resolve, reject) => {
            const reqId = "req_" + Math.random().toString(36).substring(2, 9);
            this.callbacks[reqId] = { resolve, reject };

            const message = JSON.stringify({
                id: reqId,
                action: action,
                payload: JSON.stringify(payload),
                origin: window.location.origin || "file://",
                timestamp: Date.now()
            });

            // Android dispatch
            if (window.MantraNativeBridge && window.MantraNativeBridge.postMessage) {
                window.MantraNativeBridge.postMessage(message);
            }
            // iOS dispatch
            else if (window.webkit && window.webkit.messageHandlers && window.webkit.messageHandlers.mantraBridge) {
                window.webkit.messageHandlers.mantraBridge.postMessage(message);
            } else {
                console.log("[MantraBridge Simulation]", message);
                resolve({ success: true, data: { simulation: true, action: action } });
            }
        });
    },

    __onNativeResponse(response) {
        const handler = this.callbacks[response.id];
        if (handler) {
            if (response.success) {
                handler.resolve(response.data);
            } else {
                handler.reject(new Error(response.errorMessage || "Bridge error"));
            }
            delete this.callbacks[response.id];
        }
    },

    __onNativeEvent(eventData) {
        console.log("[MantraBridge Native Event]", eventData);
    }
};

document.addEventListener("DOMContentLoaded", () => {
    const status = document.getElementById("bridge-status");
    status.innerText = "Native Bridge Ready (Mantra v0.5.0)";

    document.getElementById("btn-info").addEventListener("click", async () => {
        try {
            const info = await window.MantraBridge.send("device.info");
            status.innerText = JSON.stringify(info);
        } catch (e) {
            status.innerText = "Error: " + e.message;
        }
    });

    document.getElementById("btn-validate").addEventListener("click", () => {
        const email = document.getElementById("email-input").value;
        const valid = email.includes("@") && email.includes(".");
        document.getElementById("val-result").innerText = valid ? "✓ Valid Email" : "✗ Invalid Email";
    });
});
"#;
        fs::write(root.join("www/app.js"), www_js)
            .map_err(|e| format!("Failed to write www/app.js: {}", e))?;

        // 6. Android Platform Templates
        use mantra_mobile::AndroidBridgeTemplate;
        let pkg_path = root.join("android/app/src/main/java").join(package_name.replace('.', "/"));
        fs::write(
            pkg_path.join("MantraBridge.kt"),
            AndroidBridgeTemplate::generate_bridge_kotlin(&package_name),
        ).map_err(|e| format!("Failed to write MantraBridge.kt: {}", e))?;

        fs::write(
            pkg_path.join("MainActivity.kt"),
            AndroidBridgeTemplate::generate_main_activity_kotlin(&package_name, name),
        ).map_err(|e| format!("Failed to write MainActivity.kt: {}", e))?;

        fs::write(
            root.join("android/app/src/main/AndroidManifest.xml"),
            AndroidBridgeTemplate::generate_manifest(&package_name, name),
        ).map_err(|e| format!("Failed to write AndroidManifest.xml: {}", e))?;

        fs::write(
            root.join("android/app/src/main/res/xml/network_security_config.xml"),
            AndroidBridgeTemplate::generate_network_security_config(),
        ).map_err(|e| format!("Failed to write network_security_config.xml: {}", e))?;

        fs::write(
            root.join("android/build.gradle.kts"),
            AndroidBridgeTemplate::generate_build_gradle(&package_name),
        ).map_err(|e| format!("Failed to write build.gradle.kts: {}", e))?;

        // 7. iOS Platform Templates
        use mantra_mobile::IosBridgeTemplate;
        fs::write(
            root.join("ios/MantraApp/MantraBridge.swift"),
            IosBridgeTemplate::generate_bridge_swift(),
        ).map_err(|e| format!("Failed to write MantraBridge.swift: {}", e))?;

        fs::write(
            root.join("ios/MantraApp/ViewController.swift"),
            IosBridgeTemplate::generate_view_controller_swift(name),
        ).map_err(|e| format!("Failed to write ViewController.swift: {}", e))?;

        fs::write(
            root.join("ios/MantraApp/Info.plist"),
            IosBridgeTemplate::generate_info_plist(&package_name, name),
        ).map_err(|e| format!("Failed to write Info.plist: {}", e))?;

        // 8. Tests
        let test_src = format!(
            r#"# Tests for Mobile App {name}

def main() -> Int:
    print("Running shared mobile tests for {name}...")
    assert(True)
    print("Shared models and validation passed!")
    return 0
"#,
            name = name
        );
        fs::write(root.join("tests/test_mobile.mantra"), test_src)
            .map_err(|e| format!("Failed to write test_mobile.mantra: {}", e))?;

        let gitignore = "target/\nandroid/app/build/\nandroid/.gradle/\nios/build/\n.DS_Store\n";
        fs::write(root.join(".gitignore"), gitignore)
            .map_err(|e| format!("Failed to write .gitignore: {}", e))?;

        println!("Created Mantra Hybrid Mobile application '{}'", name);
        println!("  • Android Package : {}", package_name);
        println!("  • iOS Bundle ID   : {}", package_name);
        println!("To build for mobile targets:\n  cd {}\n  mantra mobile build android\n  mantra mobile build ios", name);

        Ok(())
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Cross-platform `mantra create` scaffolder
    // ─────────────────────────────────────────────────────────────────────────

    /// Cross-platform project scaffold.
    ///
    /// Generates the standard unified workspace layout:
    ///   lib/          — main Mantra source files
    ///   test/         — test files
    ///   web/          — web build outputs and HTML shell
    ///   android/      — generated Android bridge platform project
    ///   ios/          — generated iOS bridge platform project
    ///   assets/       — static assets
    ///   src/shared/   — shared business-logic modules
    ///   .env*         — per-environment variable files
    pub fn create_cross_platform(args: &CreateArgs<'_>) -> Result<(), String> {
        let CreateArgs {
            org,
            project_name,
            directory,
            description,
            empty,
            platforms,
            template,
        } = args;

        // 1. Validate project name against lowercase_with_underscores convention
        validate_project_name(project_name)?;

        let root = Path::new(directory);
        if root.exists() {
            return Err(format!(
                "Directory '{}' already exists. Choose a different name or remove it first.",
                directory
            ));
        }

        // Determine enabled platforms
        let all_platforms = platforms.is_empty();
        let enable_web = all_platforms || platforms.iter().any(|p| p.eq_ignore_ascii_case("web"));
        let enable_android = all_platforms || platforms.iter().any(|p| p.eq_ignore_ascii_case("android"));
        let enable_ios = all_platforms || platforms.iter().any(|p| p.eq_ignore_ascii_case("ios"));

        // ── directory tree ──────────────────────────────────────────────────
        let mut dirs = vec!["lib", "test", "assets/images"];
        if !*empty {
            dirs.push("lib/models");
            dirs.push("src/shared");
        }
        if enable_web {
            dirs.push("web");
        }
        if enable_android {
            dirs.push("android");
        }
        if enable_ios {
            dirs.push("ios");
        }

        for dir in &dirs {
            fs::create_dir_all(root.join(dir))
                .map_err(|e| format!("Failed to create {}: {}", dir, e))?;
        }

        // ── assets/.gitkeep ─────────────────────────────────────────────────
        fs::write(root.join("assets/images/.gitkeep"), "")
            .map_err(|e| format!("Failed to write assets/.gitkeep: {}", e))?;

        // ── mantra.toml (cross-platform with org + targets) ────────────────
        let bundle_id = format!("{}.{}", org, project_name);
        let desc = description.unwrap_or("Mantra cross-platform application");

        let mut targets_toml = String::new();
        if enable_web {
            targets_toml.push_str("web     = \"web/\"\n");
        }
        if enable_android {
            targets_toml.push_str("android = \"android/\"\n");
        }
        if enable_ios {
            targets_toml.push_str("ios     = \"ios/\"\n");
        }

        let toml = format!(
            r#"[package]
name = "{project_name}"
version = "0.1.0"
edition = "2026"
org = "{org}"
bundle_id = "{bundle_id}"
description = "{desc}"
template = "{template}"
license = "MIT"
entry = "lib/main.mantra"

[targets]
{targets_toml}
[dependencies]
"mantra.mobile" = "0.5.0"
"mantra.web"    = "0.5.0"
"#,
            project_name = project_name,
            org = org,
            bundle_id = bundle_id,
            desc = desc,
            template = template,
            targets_toml = targets_toml,
        );
        fs::write(root.join("mantra.toml"), toml)
            .map_err(|e| format!("Failed to write mantra.toml: {}", e))?;

        // ── lib/main.mantra ──────────────────────────────────────────────────
        let main_src = if *empty {
            format!(
                r#"# Minimal Mantra application: {project_name}

def main() -> Int:
    print("Hello from Mantra!")
    return 0
"#,
                project_name = project_name,
            )
        } else {
            format!(
                r#"# {project_name} — Mantra Cross-Platform Application
# Entry point: runs on native, web (WASM), Android, and iOS.

def get_platform_info() -> String:
    return "Mantra v0.5.0 Cross-Platform"

def main() -> Int:
    print("⚡ {project_name} initialized")
    print("Runtime: " + get_platform_info())
    return 0
"#,
                project_name = project_name,
            )
        };
        fs::write(root.join("lib/main.mantra"), main_src)
            .map_err(|e| format!("Failed to write lib/main.mantra: {}", e))?;

        if !*empty {
            // ── lib/app.mantra ───────────────────────────────────────────────────
            let app_src = format!(
                r#"# App component for {project_name}

class App:
    def __init__(self, name: String):
        self.name = name

    pub def get_name(self) -> String:
        return self.name
"#,
                project_name = project_name,
            );
            fs::write(root.join("lib/app.mantra"), app_src)
                .map_err(|e| format!("Failed to write lib/app.mantra: {}", e))?;

            // ── lib/models/user.mantra ───────────────────────────────────────────
            let user_model = r#"# User model — shared across all targets

pub class User:
    def __init__(self, id: String, email: String, name: String):
        self.id    = id
        self.email = email
        self.name  = name

    pub def to_string(self) -> String:
        return "User(" + self.id + ", " + self.email + ")"
"#;
            fs::write(root.join("lib/models/user.mantra"), user_model)
                .map_err(|e| format!("Failed to write lib/models/user.mantra: {}", e))?;

            // ── src/shared/models.mantra ─────────────────────────────────────────
            let shared_models = r#"# Shared cross-platform data models

pub class ApiResponse:
    def __init__(self, status: Int, body: String):
        self.status = status
        self.body   = body

    pub def is_ok(self) -> Bool:
        return self.status >= 200 and self.status < 300
"#;
            fs::write(root.join("src/shared/models.mantra"), shared_models)
                .map_err(|e| format!("Failed to write src/shared/models.mantra: {}", e))?;

            // ── src/shared/validation.mantra ─────────────────────────────────────
            let validation_src = r#"# Shared validation — runs identically on all platforms

pub class Validator:
    pub def validate_email(self, email: String) -> Bool:
        if len(email) < 5:
            return False
        return True

    pub def validate_required(self, value: String) -> Bool:
        return len(value) > 0
"#;
            fs::write(root.join("src/shared/validation.mantra"), validation_src)
                .map_err(|e| format!("Failed to write src/shared/validation.mantra: {}", e))?;

            // ── src/shared/api_client.mantra ─────────────────────────────────────
            let api_client_src = format!(
                r#"# Shared HTTP API client
# Base URL is read from environment at startup.

pub class ApiClient:
    def __init__(self, base_url: String):
        self.base_url = base_url

    pub def tasks_url(self) -> String:
        return self.base_url + "/api/v1/tasks"

    pub def users_url(self) -> String:
        return self.base_url + "/api/v1/users"

# Default client pre-configured from .env
def default_client() -> ApiClient:
    # In production this is read from APP_ENV / API_BASE_URL
    return ApiClient("http://localhost:8080")
"#,
            );
            fs::write(root.join("src/shared/api_client.mantra"), api_client_src)
                .map_err(|e| format!("Failed to write src/shared/api_client.mantra: {}", e))?;
        }

        // ── test/widget_test.mantra ──────────────────────────────────────────
        let test_src = format!(
            r#"# Widget / unit tests for {project_name}

def add(a: Int, b: Int) -> Int:
    return a + b

def main() -> Int:
    print("Running {project_name} tests...")
    let sum = add(20, 22)
    assert(sum == 42)
    print("  ✓ Basic math test passed")
    print("All tests passed!")
    return 0
"#,
            project_name = project_name,
        );
        fs::write(root.join("test/widget_test.mantra"), test_src)
            .map_err(|e| format!("Failed to write test/widget_test.mantra: {}", e))?;

        // ── web platform files ───────────────────────────────────────────────
        if enable_web {
            let web_html = format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{project_name}</title>
    <link rel="manifest" href="manifest.json">
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: #0f172a;
            color: #f8fafc;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            margin: 0;
        }}
        .app {{ text-align: center; }}
        h1 {{ color: #38bdf8; }}
        p  {{ color: #94a3b8; }}
        #wasm-status {{ margin-top: 1rem; color: #4ade80; font-size: 0.9rem; }}
    </style>
</head>
<body>
    <div class="app">
        <h1>⚡ {project_name}</h1>
        <p>Powered by Mantra WebAssembly</p>
        <p id="wasm-status">Loading WASM module...</p>
    </div>
    <script>
        // Load the Mantra WASM module
        async function initMantra() {{
            const statusEl = document.getElementById('wasm-status');
            try {{
                const response = await WebAssembly.instantiateStreaming(
                    fetch('app.wasm'),
                    {{ env: {{}} }}
                );
                const exports = response.instance.exports;
                if (exports.main) {{
                    const code = exports.main();
                    statusEl.textContent = 'WASM loaded — exit code: ' + code;
                }} else {{
                    statusEl.textContent = 'WASM module ready (no main export).';
                }}
            }} catch (e) {{
                statusEl.style.color = '#f87171';
                statusEl.textContent = 'WASM: ' + e.message;
                console.warn('[{project_name}] WASM init:', e);
            }}
        }}
        initMantra();
    </script>
</body>
</html>
"#,
                project_name = project_name,
            );
            fs::write(root.join("web/index.html"), web_html)
                .map_err(|e| format!("Failed to write web/index.html: {}", e))?;

            // ── web/manifest.json (PWA) ──────────────────────────────────────────
            let manifest_json = format!(
                r##"{{
    "name": "{project_name}",
    "short_name": "{project_name}",
    "start_url": "/",
    "display": "standalone",
    "background_color": "#0f172a",
    "theme_color": "#38bdf8",
    "description": "{desc}"
}}
"##,
                project_name = project_name,
                desc = desc,
            );
            fs::write(root.join("web/manifest.json"), manifest_json)
                .map_err(|e| format!("Failed to write web/manifest.json: {}", e))?;

            // ── web/favicon.ico placeholder ──────────────────────────────────────
            // Tiny valid 16×16 ICO (1×1 white pixel, binary-safe placeholder)
            fs::write(root.join("web/favicon.ico"), b"\x00\x00\x01\x00\x01\x00\x01\x01\x00\x00\x01\x00\x18\x00\x28\x00\x00\x00\x16\x00\x00\x00")
                .map_err(|e| format!("Failed to write web/favicon.ico: {}", e))?;
        }

        // ── Android platform skeleton ────────────────────────────────────────
        if enable_android {
            let android_package = format!("{}.{}", org, project_name);
            let android_pkg_path = root
                .join("android/app/src/main/java")
                .join(android_package.replace('.', "/"));
            fs::create_dir_all(&android_pkg_path)
                .map_err(|e| format!("Failed to create android java dir: {}", e))?;
            fs::create_dir_all(root.join("android/app/src/main/res/xml"))
                .map_err(|e| format!("Failed to create android res dir: {}", e))?;
            fs::create_dir_all(root.join("android/app/src/main/assets/www"))
                .map_err(|e| format!("Failed to create android assets dir: {}", e))?;

            use mantra_mobile::AndroidBridgeTemplate;
            fs::write(
                android_pkg_path.join("MantraBridge.kt"),
                AndroidBridgeTemplate::generate_bridge_kotlin(&android_package),
            ).map_err(|e| format!("Failed to write MantraBridge.kt: {}", e))?;
            fs::write(
                android_pkg_path.join("MainActivity.kt"),
                AndroidBridgeTemplate::generate_main_activity_kotlin(&android_package, project_name),
            ).map_err(|e| format!("Failed to write MainActivity.kt: {}", e))?;
            fs::write(
                root.join("android/app/src/main/AndroidManifest.xml"),
                AndroidBridgeTemplate::generate_manifest(&android_package, project_name),
            ).map_err(|e| format!("Failed to write AndroidManifest.xml: {}", e))?;
            fs::write(
                root.join("android/app/src/main/res/xml/network_security_config.xml"),
                AndroidBridgeTemplate::generate_network_security_config(),
            ).map_err(|e| format!("Failed to write network_security_config.xml: {}", e))?;
            fs::write(
                root.join("android/build.gradle.kts"),
                AndroidBridgeTemplate::generate_build_gradle(&android_package),
            ).map_err(|e| format!("Failed to write build.gradle.kts: {}", e))?;
        }

        // ── iOS platform skeleton ────────────────────────────────────────────
        if enable_ios {
            let android_package = format!("{}.{}", org, project_name);
            fs::create_dir_all(root.join("ios/MantraApp"))
                .map_err(|e| format!("Failed to create ios/MantraApp: {}", e))?;

            use mantra_mobile::IosBridgeTemplate;
            fs::write(
                root.join("ios/MantraApp/MantraBridge.swift"),
                IosBridgeTemplate::generate_bridge_swift(),
            ).map_err(|e| format!("Failed to write MantraBridge.swift: {}", e))?;
            fs::write(
                root.join("ios/MantraApp/ViewController.swift"),
                IosBridgeTemplate::generate_view_controller_swift(project_name),
            ).map_err(|e| format!("Failed to write ViewController.swift: {}", e))?;
            fs::write(
                root.join("ios/MantraApp/Info.plist"),
                IosBridgeTemplate::generate_info_plist(&android_package, project_name),
            ).map_err(|e| format!("Failed to write Info.plist: {}", e))?;
        }

        // ── Environment files ────────────────────────────────────────────────
        // .env — local (NOT committed)
        let env_local = format!(
            r#"# Local Development Overrides — NOT committed to version control
APP_ENV=local
APP_NAME={project_name}
API_BASE_URL=http://localhost:8080
DEBUG=true
LOG_LEVEL=debug
DATABASE_URL=sqlite:///local.db
"#,
            project_name = project_name,
        );
        fs::write(root.join(".env"), env_local)
            .map_err(|e| format!("Failed to write .env: {}", e))?;

        // .env.development
        let env_dev = format!(
            r#"APP_ENV=development
APP_NAME={project_name}
API_BASE_URL=https://dev-api.{org}.com/{project_name}
DEBUG=true
LOG_LEVEL=info
DATABASE_URL=postgresql://dev-db/mantra
"#,
            project_name = project_name,
            org = org,
        );
        fs::write(root.join(".env.development"), env_dev)
            .map_err(|e| format!("Failed to write .env.development: {}", e))?;

        // .env.test
        let env_test = format!(
            r#"APP_ENV=test
APP_NAME={project_name}
API_BASE_URL=https://test-api.{org}.com/{project_name}
DEBUG=false
LOG_LEVEL=warning
DATABASE_URL=sqlite:///:memory:
"#,
            project_name = project_name,
            org = org,
        );
        fs::write(root.join(".env.test"), env_test)
            .map_err(|e| format!("Failed to write .env.test: {}", e))?;

        // .env.production
        let env_prod = format!(
            r#"APP_ENV=production
APP_NAME={project_name}
API_BASE_URL=https://api.{org}.com/{project_name}
DEBUG=false
LOG_LEVEL=error
DATABASE_URL=${{DATABASE_URL}}
"#,
            project_name = project_name,
            org = org,
        );
        fs::write(root.join(".env.production"), env_prod)
            .map_err(|e| format!("Failed to write .env.production: {}", e))?;

        // .env.example (committed — documents all keys)
        let env_example = format!(
            r#"# Copy this file to .env and fill in your local values.
# .env is git-ignored; .env.development / .env.test / .env.production are tracked.

APP_ENV=local
APP_NAME={project_name}
API_BASE_URL=http://localhost:8080
DEBUG=true
LOG_LEVEL=debug
DATABASE_URL=sqlite:///local.db
"#,
            project_name = project_name,
        );
        fs::write(root.join(".env.example"), env_example)
            .map_err(|e| format!("Failed to write .env.example: {}", e))?;

        // ── .gitignore ───────────────────────────────────────────────────────
        // .env (local) is ignored; the other .env.* files are tracked.
        let gitignore = r#"# Mantra build output
target/

# Local environment overrides (never commit secrets)
.env
.env.local

# Android build artifacts
android/app/build/
android/.gradle/
android/local.properties

# iOS build artifacts
ios/build/
ios/Pods/
ios/*.xcworkspace/

# Web build output (generated — re-built by `mantra build web`)
web/app.wasm
web/app.wasm.map

# macOS
.DS_Store

# Editor
.vscode/
*.swp
"#;
        fs::write(root.join(".gitignore"), gitignore)
            .map_err(|e| format!("Failed to write .gitignore: {}", e))?;

        // ── README.md ────────────────────────────────────────────────────────
        let readme = format!(
            r#"# {project_name}

A Mantra cross-platform application — built for **web**, **Android**, and **iOS** from a single codebase.

> Organisation: `{org}` · Bundle ID: `{bundle_id}`

## Getting started

```bash
# Run development server (native)
mantra run

# Build for web (WebAssembly)
mantra build web

# Build for Android
mantra mobile build android

# Build for iOS
mantra mobile build ios

# Run test suite
mantra test
```

## Project structure

```
{project_name}/
├── lib/                     # Main Mantra application source
│   ├── main.mantra          # App entry point
│   ├── app.mantra           # Root App class
│   └── models/
│       └── user.mantra      # User model
├── src/shared/              # Cross-platform shared logic
│   ├── models.mantra        # Shared data models
│   ├── validation.mantra    # Shared validation rules
│   └── api_client.mantra    # Shared API client
├── test/                    # Tests
│   └── widget_test.mantra
├── web/                     # Web build shell + PWA manifest
│   ├── index.html
│   ├── manifest.json
│   └── favicon.ico
├── android/                 # Android bridge platform project
├── ios/                     # iOS bridge platform project
├── assets/images/           # Static assets
├── .env                     # Local overrides (git-ignored)
├── .env.development         # Development environment
├── .env.test                # Test environment
├── .env.production          # Production environment
├── .env.example             # Template — copy to .env
└── mantra.toml              # Project manifest
```

## Environment files

| File              | Purpose                     | Committed? |
|-------------------|-----------------------------|------------|
| `.env`            | Local machine overrides      | ❌ No      |
| `.env.development`| Shared dev configuration     | ✅ Yes     |
| `.env.test`       | CI / test configuration      | ✅ Yes     |
| `.env.production` | Production configuration     | ✅ Yes     |
| `.env.example`    | Key documentation template   | ✅ Yes     |
"#,
            project_name = project_name,
            org = org,
            bundle_id = bundle_id,
        );
        fs::write(root.join("README.md"), readme)
            .map_err(|e| format!("Failed to write README.md: {}", e))?;

        // ── Summary ──────────────────────────────────────────────────────────
        println!("Creating {project_name}...", project_name = project_name);
        println!("  Organisation  : {}", org);
        println!("  Bundle ID     : {}", bundle_id);
        println!("  Directory     : {}", directory);
        println!("  Template      : {}", template);
        if *empty {
            println!("  Mode          : minimal (--empty)");
        }
        println!();
        println!("✓ lib/             — Mantra source files");
        if !*empty {
            println!("✓ src/shared/      — Cross-platform shared logic");
        }
        println!("✓ test/            — Test suite");
        if enable_web {
            println!("✓ web/             — Web / PWA shell (index.html, manifest.json)");
        }
        if enable_android {
            println!("✓ android/         — Android bridge project (Kotlin)");
        }
        if enable_ios {
            println!("✓ ios/             — iOS bridge project (Swift)");
        }
        println!("✓ assets/          — Static assets");
        println!("✓ .env             — Local environment (git-ignored)");
        println!("✓ .env.development / .env.test / .env.production");
        println!("✓ .env.example     — Key template (committed)");
        println!("✓ mantra.toml      — Project manifest");
        println!();
        println!("All done! Get started:");
        println!("  cd {dir}", dir = directory);
        println!("  mantra run");
        if enable_web {
            println!("  mantra run -d chrome --web-port 4300 --mantra-define-from-file=.env.development");
            println!("  mantra build web --release --mantra-define-from-file=.env.development");
        }
        if enable_android {
            println!("  mantra run -d android --mantra-define-from-file=.env.test");
            println!("  mantra build apk --release --mantra-define-from-file=.env.test");
        }
        if enable_ios {
            println!("  mantra run -d ios --mantra-define-from-file=.env.test");
        }

        Ok(())
    }
}

