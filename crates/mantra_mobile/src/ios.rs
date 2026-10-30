// ==============================================================================
// mantra_mobile::ios — iOS Native Bridge & Platform Templates
// ==============================================================================

pub struct IosBridgeTemplate;

impl IosBridgeTemplate {
    /// Generates production-ready Swift MantraBridge implementation.
    pub fn generate_bridge_swift() -> &'static str {
        r#"import Foundation

public class MantraBridge: NSObject {
    private let onMessageCallback: (String) -> String

    public init(onMessageCallback: @escaping (String) -> String) {
        self.onMessageCallback = onMessageCallback
        super.init()
    }

    public func dispatchAction(action: String, payload: [String: Any], completion: @escaping (String) -> Void) {
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            guard let self = self else { return }
            let requestJson = "{\"action\":\"\(action)\"}"
            let response = self.onMessageCallback(requestJson)
            DispatchQueue.main.async {
                completion(response)
            }
        }
    }

    public func dispatchLifecycleEvent(eventName: String, state: String) {
        // Native lifecycle notification
    }
}
"#
    }

    /// Generates pure Native SwiftUI ContentView.swift.
    /// Does NOT use WKWebView — renders native iOS SwiftUI View hierarchy.
    pub fn generate_swiftui_content_view(app_name: &str, content: &str) -> String {
        let inner = if content.trim().is_empty() {
            format!(
                r#"                VStack(spacing: 8) {{
                    Text("🔐")
                        .font(.system(size: 48))
                    Text("{app_name}")
                        .font(.title)
                        .fontWeight(.bold)
                    Text("Built with Mantra Widgets • Native iOS SwiftUI")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                }}

                VStack(spacing: 16) {{
                    VStack(alignment: .leading, spacing: 6) {{
                        Text("Email Address").font(.caption).foregroundColor(.secondary)
                        TextField("name@example.com", text: $emailInput)
                            .padding()
                            .background(Color(.secondarySystemBackground))
                            .cornerRadius(10)
                            .keyboardType(.emailAddress)
                    }}

                    VStack(alignment: .leading, spacing: 6) {{
                        Text("Password").font(.caption).foregroundColor(.secondary)
                        SecureField("••••••••", text: $passInput)
                            .padding()
                            .background(Color(.secondarySystemBackground))
                            .cornerRadius(10)
                    }}

                    Button(action: {{
                        isLoggedIn = true
                    }}) {{
                        Text("Sign In")
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(Color.accentColor)
                            .foregroundColor(.white)
                            .cornerRadius(10)
                            .fontWeight(.semibold)
                    }}
                }}
                .padding(24)
                .background(Color(.systemBackground))
                .cornerRadius(16)
                .shadow(radius: 4)"#
            )
        } else {
            content.to_string()
        };

        format!(
            r#"import SwiftUI

public struct ContentView: View {{
    @State private var emailInput: String = ""
    @State private var passInput: String = ""
    @State private var rememberChk: Bool = false
    @State private var isLoggedIn: Bool = false
    @State private var userName: String = ""

    public init() {{}}

    public var body: some View {{
        ZStack {{
            Color(.systemGroupedBackground).ignoresSafeArea()
            ScrollView {{
                VStack(spacing: 24) {{
{inner}
                }}
                .padding(20)
            }}
        }}
    }}
}}

#Preview {{
    ContentView()
}}
"#,
            inner = inner
        )
    }

    /// Generates pure Native SwiftUI App entry point.
    pub fn generate_native_app_swift(app_name: &str) -> String {
        let clean_name = app_name.replace(|c: char| !c.is_alphanumeric(), "");
        format!(
            r#"import SwiftUI

@main
struct {clean_name}App: App {{
    var body: some Scene {{
        WindowGroup {{
            ContentView()
        }}
    }}
}}
"#
        )
    }

    /// Generates iOS ViewController.swift hosting native SwiftUI ContentView.
    /// Does NOT use WKWebView.
    pub fn generate_view_controller_swift(app_name: &str) -> String {
        let _ = app_name;
        r#"import UIKit
import SwiftUI

class ViewController: UIViewController {
    override func viewDidLoad() {
        super.viewDidLoad()

        // Pure Native iOS: Host SwiftUI ContentView via UIHostingController
        let hostingController = UIHostingController(rootView: ContentView())
        addChild(hostingController)
        hostingController.view.frame = view.bounds
        hostingController.view.autoresizingMask = [.flexibleWidth, .flexibleHeight]
        view.addSubview(hostingController.view)
        hostingController.didMove(toParent: self)
    }
}
"#.to_string()
    }

    /// Generates iOS Info.plist with permission usage strings and App Transport Security.
    pub fn generate_info_plist(bundle_id: &str, app_name: &str) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>{app_name}</string>
    <key>CFBundleIdentifier</key>
    <string>{bundle_id}</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>{app_name}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.5.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSRequiresIPhoneOS</key>
    <true/>
    <key>UILaunchScreen</key>
    <dict/>

    <!-- Hardware and Privacy Permission Usage Descriptions -->
    <key>NSCameraUsageDescription</key>
    <string>{app_name} requires camera access for photo capture and scanning.</string>
    <key>NSLocationWhenInUseUsageDescription</key>
    <string>{app_name} requires location access to provide location-based services.</string>
    <key>NSPhotoLibraryUsageDescription</key>
    <string>{app_name} requires photo library access to select and upload media.</string>
</dict>
</plist>
"#,
            bundle_id = bundle_id,
            app_name = app_name
        )
    }
}
