import UIKit
import WebKit

class ViewController: UIViewController, WKNavigationDelegate {
    private var webView: WKWebView!
    private var bridge: MantraBridge!

    override func viewDidLoad() {
        super.viewDidLoad()

        let configuration = WKWebViewConfiguration()
        let contentController = WKUserContentController()

        bridge = MantraBridge(webView: nil) { [weak self] rawRequest in
            self?.handleNativeRequest(rawRequest) ?? "{\"success\":false}"
        }

        contentController.add(bridge, name: "mantraBridge")
        configuration.userContentController = contentController

        webView = WKWebView(frame: view.bounds, configuration: configuration)
        webView.autoresizingMask = [.flexibleWidth, .flexibleHeight]
        webView.navigationDelegate = self
        view.addSubview(webView)

        // Point bridge to active webView
        // Load bundled www assets
        if let bundleUrl = Bundle.main.url(forResource: "index", withExtension: "html", subdirectory: "www") {
            webView.loadFileURL(bundleUrl, allowingReadAccessTo: bundleUrl.deletingLastPathComponent())
        }
    }

    func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
        bridge.dispatchLifecycleEvent(eventName: "app_ready", state: "active")
    }

    private func handleNativeRequest(_ requestJson: String) -> String {
        // Echo response or forward to compiled Mantra runtime
        return """{"success":true,"data":{"platform":"ios","app":"07_hybrid_mobile_app","status":"ready"}}"""
    }
}
