package com.mantra.07_hybrid_mobile_app

import android.annotation.SuppressLint
import android.os.Bundle
import android.webkit.WebSettings
import android.webkit.WebView
import android.webkit.WebViewClient
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {
    private lateinit var webView: WebView
    private lateinit var bridge: MantraBridge

    @SuppressLint("SetJavaScriptEnabled")
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        webView = WebView(this)
        setContentView(webView)

        val settings: WebSettings = webView.settings
        settings.javaScriptEnabled = true
        settings.domStorageEnabled = true
        settings.allowFileAccess = false
        settings.allowContentAccess = false

        bridge = MantraBridge(this, webView) { rawRequest ->
            // In a full native build, this invokes compiled Mantra shared library logic (.so)
            handleNativeRequest(rawRequest)
        }

        // Attach safe JavaScript interface
        webView.addJavascriptInterface(bridge, "MantraNativeBridge")
        webView.webViewClient = object : WebViewClient() {
            override fun onPageFinished(view: WebView?, url: String?) {
                super.onPageFinished(view, url)
                bridge.dispatchLifecycleEvent("app_ready", "active")
            }
        }

        // Load bundled asset UI
        webView.loadUrl("file:///android_asset/www/index.html")
    }

    override fun onResume() {
        super.onResume()
        bridge.dispatchLifecycleEvent("lifecycle_change", "active")
    }

    override fun onPause() {
        super.onPause()
        bridge.dispatchLifecycleEvent("lifecycle_change", "inactive")
    }

    override fun onDestroy() {
        super.onDestroy()
        bridge.dispatchLifecycleEvent("lifecycle_change", "destroyed")
    }

    private fun handleNativeRequest(requestJson: String): String {
        // Echo response or forward to compiled Mantra runtime
        return """{"success":true,"data":{"platform":"android","app":"07_hybrid_mobile_app","status":"ready"}}"""
    }
}
