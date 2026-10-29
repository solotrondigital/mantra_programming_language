import Foundation
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
