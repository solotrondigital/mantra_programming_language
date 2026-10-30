use super::fs_service::FilesystemService;
use super::validator::ValidatedOptions;
use mantra_mobile::{AndroidBridgeTemplate, IosBridgeTemplate};
use std::path::Path;

pub struct FrontendTemplateGenerator;

impl FrontendTemplateGenerator {
    pub fn generate(root: &Path, opts: &ValidatedOptions) -> Result<(), String> {
        let name = &opts.project_name;
        let package_name = format!("{}.{}", opts.org, name);

        // 1. Directory Tree
        FilesystemService::create_dir_all(&root.join("src/app"))?;
        FilesystemService::create_dir_all(&root.join("src/components"))?;
        FilesystemService::create_dir_all(&root.join("src/screens"))?;
        FilesystemService::create_dir_all(&root.join("src/navigation"))?;
        FilesystemService::create_dir_all(&root.join("src/services"))?;
        FilesystemService::create_dir_all(&root.join("src/state"))?;
        FilesystemService::create_dir_all(&root.join("src/styles"))?;
        FilesystemService::create_dir_all(&root.join("assets/images"))?;
        FilesystemService::create_dir_all(&root.join("tests"))?;

        FilesystemService::write_file(&root.join("assets/images/.gitkeep"), "")?;

        // 2. src/main.mantra (Entry Point)
        let main_src = format!(
            r#"# {name} — Hybrid Frontend Application Entry Point

class AppBootstrap:
    def __init__(self, name: String):
        self.app_name = name
        self.initialized = False

    def start(self) -> Int:
        print("⚡ Initializing Mantra Hybrid App: " + self.app_name)
        self.initialized = True
        print("✓ Shared navigation, state, and UI tree mounted.")
        return 0

def main() -> Int:
    let app = AppBootstrap("{name}")
    return app.start()
"#
        );
        FilesystemService::write_file(&root.join("src/main.mantra"), &main_src)?;

        // 3. src/app/app.mantra
        let app_src = format!(
            r#"# {name} Application Root Component

class Application:
    def __init__(self, title: String):
        self.title = title
        self.version = "0.1.0"

    def get_title(self) -> String:
        return self.title

    def render(self) -> String:
        return "<App title='" + self.title + "'>"
"#
        );
        FilesystemService::write_file(&root.join("src/app/app.mantra"), &app_src)?;

        // 4. src/components/button.mantra
        let button_src = r#"# Reusable UI Button Component

class Button:
    def __init__(self, label: String):
        self.label = label
        self.is_enabled = True

    def render(self) -> String:
        return "<button enabled='" + str(self.is_enabled) + "'>" + self.label + "</button>"
"#;
        FilesystemService::write_file(&root.join("src/components/button.mantra"), button_src)?;

        // 5. src/components/header.mantra
        let header_src = r#"# Application Header Bar Component

class Header:
    def __init__(self, title: String):
        self.title = title
        self.show_back = False

    def render(self) -> String:
        return "<header><title>" + self.title + "</title></header>"
"#;
        FilesystemService::write_file(&root.join("src/components/header.mantra"), header_src)?;

        // 6. src/screens/home_screen.mantra
        let screen_src = r#"# Main Home Screen View

class HomeScreen:
    def __init__(self):
        self.title = "Home"

    def render_content(self) -> String:
        return "Welcome to Mantra Hybrid Application"
"#;
        FilesystemService::write_file(&root.join("src/screens/home_screen.mantra"), screen_src)?;

        // 7. src/navigation/router.mantra
        let router_src = r#"# Screen Routing and Navigation Controller

class NavigationRouter:
    def __init__(self):
        self.current_route = "/"

    def navigate_to(self, route: String) -> String:
        self.current_route = route
        return self.current_route
"#;
        FilesystemService::write_file(&root.join("src/navigation/router.mantra"), router_src)?;

        // 8. src/services/api_service.mantra
        let service_src = r#"# Cross-Platform Network Service

class ApiService:
    def __init__(self, base_url: String):
        self.base_url = base_url

    def get_endpoint(self, path: String) -> String:
        return self.base_url + path
"#;
        FilesystemService::write_file(&root.join("src/services/api_service.mantra"), service_src)?;

        // 9. src/state/app_state.mantra
        let state_src = r#"# Global State Management Store

class AppState:
    def __init__(self):
        self.counter = 0
        self.dark_mode = False

    def increment(self) -> Int:
        self.counter = self.counter + 1
        return self.counter
"#;
        FilesystemService::write_file(&root.join("src/state/app_state.mantra"), state_src)?;

        // 10. src/styles/theme.mantra
        let theme_src = r##"# Global Theme and Styling Constants

class Theme:
    def __init__(self):
        self.primary_color = "#6200EE"
        self.background_color = "#FFFFFF"
        self.text_color = "#121212"
"##;
        FilesystemService::write_file(&root.join("src/styles/theme.mantra"), theme_src)?;

        // 11. tests/app_test.mantra
        let test_src = format!(
            r#"# {name} Frontend Test Suite

def test_initialization() -> Int:
    let initialized: Bool = True
    assert(initialized == True)
    return 0

def main() -> Int:
    print("Running frontend unit tests for {name}...")
    test_initialization()
    print("✓ All tests passed.")
    return 0
"#
        );
        FilesystemService::write_file(&root.join("tests/app_test.mantra"), &test_src)?;

        // 12. platforms/ directory
        for platform in &opts.platforms {
            match platform.as_str() {
                "web" => {
                    let web_dir = root.join("platforms/web");
                    FilesystemService::create_dir_all(&web_dir)?;
                    let index_html = format!(
                        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{name}</title>
    <link rel="manifest" href="manifest.json">
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            margin: 0;
            padding: 2rem;
            display: flex;
            flex-direction: column;
            align-items: center;
            background: #f7f9fa;
        }}
        .card {{
            background: white;
            padding: 2rem;
            border-radius: 12px;
            box-shadow: 0 4px 16px rgba(0,0,0,0.08);
            max-width: 600px;
            width: 100%;
        }}
    </style>
</head>
<body>
    <div class="card">
        <h1>{name}</h1>
        <p>Mantra Cross-Platform Hybrid Application</p>
        <div id="mantra-root">Running WebAssembly runtime...</div>
    </div>
</body>
</html>
"#
                    );
                    FilesystemService::write_file(&web_dir.join("index.html"), &index_html)?;

                    let manifest_json = format!(
                        r##"{{
  "name": "{name}",
  "short_name": "{name}",
  "start_url": "./index.html",
  "display": "standalone",
  "background_color": "#ffffff",
  "theme_color": "#6200ee"
}}
"##
                    );
                    FilesystemService::write_file(&web_dir.join("manifest.json"), &manifest_json)?;
                }
                "android" => {
                    let android_pkg_path = root
                        .join("platforms/android/app/src/main/java")
                        .join(package_name.replace('.', "/"));
                    let android_res_xml = root.join("platforms/android/app/src/main/res/xml");
                    let android_res_layout = root.join("platforms/android/app/src/main/res/layout");

                    FilesystemService::create_dir_all(&android_pkg_path)?;
                    FilesystemService::create_dir_all(&android_res_xml)?;
                    FilesystemService::create_dir_all(&android_res_layout)?;

                    FilesystemService::write_file(
                        &android_pkg_path.join("MantraBridge.kt"),
                        &AndroidBridgeTemplate::generate_bridge_kotlin(&package_name),
                    )?;
                    FilesystemService::write_file(
                        &android_pkg_path.join("MainActivity.kt"),
                        &AndroidBridgeTemplate::generate_main_activity_kotlin(&package_name, name),
                    )?;
                    FilesystemService::write_file(
                        &android_res_layout.join("activity_main.xml"),
                        &AndroidBridgeTemplate::generate_native_layout_xml(name, ""),
                    )?;
                    FilesystemService::write_file(
                        &root.join("platforms/android/app/src/main/AndroidManifest.xml"),
                        &AndroidBridgeTemplate::generate_manifest(&package_name, name),
                    )?;
                    FilesystemService::write_file(
                        &android_res_xml.join("network_security_config.xml"),
                        AndroidBridgeTemplate::generate_network_security_config(),
                    )?;
                    FilesystemService::write_file(
                        &root.join("platforms/android/build.gradle.kts"),
                        &AndroidBridgeTemplate::generate_build_gradle(&package_name),
                    )?;
                }
                "ios" => {
                    let ios_app_dir = root.join("platforms/ios/MantraApp");
                    FilesystemService::create_dir_all(&ios_app_dir)?;

                    FilesystemService::write_file(
                        &ios_app_dir.join("MantraBridge.swift"),
                        IosBridgeTemplate::generate_bridge_swift(),
                    )?;
                    FilesystemService::write_file(
                        &ios_app_dir.join("ContentView.swift"),
                        &IosBridgeTemplate::generate_swiftui_content_view(name, ""),
                    )?;
                    FilesystemService::write_file(
                        &ios_app_dir.join("App.swift"),
                        &IosBridgeTemplate::generate_native_app_swift(name),
                    )?;
                    FilesystemService::write_file(
                        &ios_app_dir.join("ViewController.swift"),
                        &IosBridgeTemplate::generate_view_controller_swift(name),
                    )?;
                    FilesystemService::write_file(
                        &ios_app_dir.join("Info.plist"),
                        &IosBridgeTemplate::generate_info_plist(&package_name, name),
                    )?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
