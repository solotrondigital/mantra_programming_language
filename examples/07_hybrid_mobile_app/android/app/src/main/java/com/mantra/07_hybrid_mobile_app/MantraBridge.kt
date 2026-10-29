package com.mantra.07_hybrid_mobile_app

import android.content.Context
import android.os.Handler
import android.os.Looper
import android.webkit.JavascriptInterface
import android.webkit.WebView
import org.json.JSONObject

class MantraBridge(
    private val context: Context,
    private val webView: WebView,
    private val onMessageCallback: (String) -> String
) {
    private val mainHandler = Handler(Looper.getMainLooper())

    @JavascriptInterface
    fun postMessage(jsonString: String) {
        // Execute bridge logic on background thread to prevent UI freezing
        Thread {
            val responseJson = try {
                onMessageCallback(jsonString)
            } catch (e: Exception) {
                val errorObj = JSONObject().apply {
                    put("id", "unknown")
                    put("success", false)
                    put("errorCode", "ERR_EXECUTION_FAILED")
                    put("errorMessage", e.message ?: "Unknown native error")
                }
                errorObj.toString()
            }

            // Deliver response back to WebView on the Main UI Thread
            mainHandler.post {
                val js = "if (window.MantraBridge && window.MantraBridge.__onNativeResponse) { window.MantraBridge.__onNativeResponse($responseJson); }"
                webView.evaluateJavascript(js, null)
            }
        }.start()
    }

    fun dispatchLifecycleEvent(eventName: String, state: String) {
        mainHandler.post {
            val eventJson = JSONObject().apply {
                put("event", eventName)
                put("state", state)
            }
            val js = "if (window.MantraBridge && window.MantraBridge.__onNativeEvent) { window.MantraBridge.__onNativeEvent($eventJson); }"
            webView.evaluateJavascript(js, null)
        }
    }
}
