// ==============================================================================
// mantra_mobile::ios — iOS Native Bridge & Platform Templates
// ==============================================================================

pub struct IosBridgeTemplate;

impl IosBridgeTemplate {
    /// Generates production-ready Swift MantraBridge implementation.
    pub fn generate_bridge_swift() -> &'static str {
        r#"import Foundation
import WebKit

public class MantraBridge: NSObject, WKScriptMessageHandler {
    private weak var webView: WKWebView?
    private let onMessageCallback: (String) -> String

    public init(webView: WKWebView, onMessageCallback: @escaping (String) -> String) {
        self.webView = webView
        self.onMessageCallback = onMessageCallback
        super.init()
    }

    public func userContentController(_ userContentController: WKUserContentController, didReceive message: WKScriptMessage) {
        guard message.name == "mantraBridge",
              let bodyString = message.body as? String else {
            return
        }

        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            guard let self = self else { return }
            let responseJson = self.onMessageCallback(bodyString)

            DispatchQueue.main.async {
                let js = "if (window.MantraBridge && window.MantraBridge.__onNativeResponse) { window.MantraBridge.__onNativeResponse(\(responseJson)); }"
                self.webView?.evaluateJavaScript(js, completionHandler: nil)
            }
        }
    }

    public func dispatchLifecycleEvent(eventName: String, state: String) {
        DispatchQueue.main.async { [weak self] in
            let eventJson = "{\"event\":\"\(eventName)\",\"state\":\"\(state)\"}"
            let js = "if (window.MantraBridge && window.MantraBridge.__onNativeEvent) { window.MantraBridge.__onNativeEvent(\(eventJson)); }"
            self?.webView?.evaluateJavaScript(js, completionHandler: nil)
        }
    }
}
"#
    }

    /// Generates iOS ViewController.swift configuring WKWebView and security boundaries.
    pub fn generate_view_controller_swift(app_name: &str) -> String {
        format!(
            r#"import UIKit
import WebKit

class ViewController: UIViewController, WKNavigationDelegate {{
    private var webView: WKWebView!
    private var bridge: MantraBridge!

    override func viewDidLoad() {{
        super.viewDidLoad()

        let configuration = WKWebViewConfiguration()
        let contentController = WKUserContentController()

        bridge = MantraBridge(webView: nil) {{ [weak self] rawRequest in
            self?.handleNativeRequest(rawRequest) ?? "{{\"success\":false}}"
        }}

        contentController.add(bridge, name: "mantraBridge")
        configuration.userContentController = contentController

        webView = WKWebView(frame: view.bounds, configuration: configuration)
        webView.autoresizingMask = [.flexibleWidth, .flexibleHeight]
        webView.navigationDelegate = self
        view.addSubview(webView)

        // Point bridge to active webView
        // Load bundled www assets
        if let bundleUrl = Bundle.main.url(forResource: "index", withExtension: "html", subdirectory: "www") {{
            webView.loadFileURL(bundleUrl, allowingReadAccessTo: bundleUrl.deletingLastPathComponent())
        }}
    }}

    func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {{
        bridge.dispatchLifecycleEvent(eventName: "app_ready", state: "active")
    }}

    private func handleNativeRequest(_ requestJson: String) -> String {{
        // Echo response or forward to compiled Mantra runtime
        return """{{"success":true,"data":{{"platform":"ios","app":"{app_name}","status":"ready"}}}}"""
    }}
}}
"#,
            app_name = app_name
        )
    }

    /// Generates iOS Info.plist with permission usage strings and App Transport Security.
    pub fn generate_info_plist(bundle_id: &str, app_name: &str) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleIdentifier</key>
    <string>{bundle_id}</string>
    <key>CFBundleName</key>
    <string>{app_name}</string>
    <key>CFBundleShortVersionString</key>
    <string>0.5.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSRequiresIPhoneOS</key>
    <true/>

    <!-- Hardware and Privacy Permission Usage Descriptions -->
    <key>NSCameraUsageDescription</key>
    <string>{app_name} requires camera access for photo capture and scanning.</string>
    <key>NSLocationWhenInUseUsageDescription</key>
    <string>{app_name} requires location access to provide location-based services.</string>
    <key>NSPhotoLibraryUsageDescription</key>
    <string>{app_name} requires photo library access to select and upload media.</string>

    <!-- App Transport Security (ATS): Enforce HTTPS -->
    <key>NSAppTransportSecurity</key>
    <dict>
        <key>NSAllowsArbitraryLoads</key>
        <false/>
        <key>NSExceptionDomains</key>
        <dict>
            <key>localhost</key>
            <dict>
                <key>NSExceptionAllowsInsecureHTTPLoads</key>
                <true/>
            </dict>
        </dict>
    </dict>

    <key>UILaunchStoryboardName</key>
    <string>LaunchScreen</string>
    <key>UIRequiredDeviceCapabilities</key>
    <array>
        <string>arm64</string>
    </array>
</dict>
</plist>
"#,
            bundle_id = bundle_id,
            app_name = app_name
        )
    }
}

