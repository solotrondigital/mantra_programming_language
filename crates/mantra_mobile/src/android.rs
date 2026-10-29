// ==============================================================================
// mantra_mobile::android — Android Native Bridge & Platform Templates
// ==============================================================================

pub struct AndroidBridgeTemplate;

impl AndroidBridgeTemplate {
    /// Generates production-ready Kotlin MantraBridge implementation.
    pub fn generate_bridge_kotlin(package_name: &str) -> String {
        format!(
            r#"package {package_name}

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
) {{
    private val mainHandler = Handler(Looper.getMainLooper())

    @JavascriptInterface
    fun postMessage(jsonString: String) {{
        // Execute bridge logic on background thread to prevent UI freezing
        Thread {{
            val responseJson = try {{
                onMessageCallback(jsonString)
            }} catch (e: Exception) {{
                val errorObj = JSONObject().apply {{
                    put("id", "unknown")
                    put("success", false)
                    put("errorCode", "ERR_EXECUTION_FAILED")
                    put("errorMessage", e.message ?: "Unknown native error")
                }}
                errorObj.toString()
            }}

            // Deliver response back to WebView on the Main UI Thread
            mainHandler.post {{
                val js = "if (window.MantraBridge && window.MantraBridge.__onNativeResponse) {{ window.MantraBridge.__onNativeResponse($responseJson); }}"
                webView.evaluateJavascript(js, null)
            }}
        }}.start()
    }}

    fun dispatchLifecycleEvent(eventName: String, state: String) {{
        mainHandler.post {{
            val eventJson = JSONObject().apply {{
                put("event", eventName)
                put("state", state)
            }}
            val js = "if (window.MantraBridge && window.MantraBridge.__onNativeEvent) {{ window.MantraBridge.__onNativeEvent($eventJson); }}"
            webView.evaluateJavascript(js, null)
        }}
    }}
}}
"#,
            package_name = package_name
        )
    }

    /// Generates Android MainActivity.kt configuring WebView and security settings.
    pub fn generate_main_activity_kotlin(package_name: &str, app_name: &str) -> String {
        format!(
            r#"package {package_name}

import android.annotation.SuppressLint
import android.os.Bundle
import android.webkit.WebSettings
import android.webkit.WebView
import android.webkit.WebViewClient
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {{
    private lateinit var webView: WebView
    private lateinit var bridge: MantraBridge

    @SuppressLint("SetJavaScriptEnabled")
    override fun onCreate(savedInstanceState: Bundle?) {{
        super.onCreate(savedInstanceState)

        webView = WebView(this)
        setContentView(webView)

        val settings: WebSettings = webView.settings
        settings.javaScriptEnabled = true
        settings.domStorageEnabled = true
        settings.allowFileAccess = false
        settings.allowContentAccess = false

        bridge = MantraBridge(this, webView) {{ rawRequest ->
            // In a full native build, this invokes compiled Mantra shared library logic (.so)
            handleNativeRequest(rawRequest)
        }}

        // Attach safe JavaScript interface
        webView.addJavascriptInterface(bridge, "MantraNativeBridge")
        webView.webViewClient = object : WebViewClient() {{
            override fun onPageFinished(view: WebView?, url: String?) {{
                super.onPageFinished(view, url)
                bridge.dispatchLifecycleEvent("app_ready", "active")
            }}
        }}

        // Load bundled asset UI
        webView.loadUrl("file:///android_asset/www/index.html")
    }}

    override fun onResume() {{
        super.onResume()
        bridge.dispatchLifecycleEvent("lifecycle_change", "active")
    }}

    override fun onPause() {{
        super.onPause()
        bridge.dispatchLifecycleEvent("lifecycle_change", "inactive")
    }}

    override fun onDestroy() {{
        super.onDestroy()
        bridge.dispatchLifecycleEvent("lifecycle_change", "destroyed")
    }}

    private fun handleNativeRequest(requestJson: String): String {{
        // Echo response or forward to compiled Mantra runtime
        return """{{"success":true,"data":{{"platform":"android","app":"{app_name}","status":"ready"}}}}"""
    }}
}}
"#,
            package_name = package_name,
            app_name = app_name
        )
    }

    /// Generates AndroidManifest.xml.
    pub fn generate_manifest(package_name: &str, app_name: &str) -> String {
        format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    package="{package_name}">

    <!-- Essential Hybrid Mobile Permissions -->
    <uses-permission android:name="android.permission.INTERNET" />
    <uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />

    <application
        android:allowBackup="false"
        android:icon="@mipmap/ic_launcher"
        android:label="{app_name}"
        android:networkSecurityConfig="@xml/network_security_config"
        android:supportsRtl="true"
        android:theme="@style/Theme.AppCompat.Light.NoActionBar">
        
        <activity
            android:name=".MainActivity"
            android:configChanges="orientation|screenSize|keyboardHidden"
            android:exported="true">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
    </application>
</manifest>
"#,
            package_name = package_name,
            app_name = app_name
        )
    }

    /// Generates network_security_config.xml blocking cleartext HTTP traffic.
    pub fn generate_network_security_config() -> &'static str {
        r#"<?xml version="1.0" encoding="utf-8"?>
<network-security-config>
    <!-- Enforce HTTPS / Block cleartext traffic -->
    <base-config cleartextTrafficPermitted="false">
        <trust-anchors>
            <certificates src="system" />
        </trust-anchors>
    </base-config>
    <domain-config cleartextTrafficPermitted="true">
        <domain includeSubdomains="true">localhost</domain>
        <domain includeSubdomains="true">10.0.2.2</domain> <!-- Android Emulator Loopback -->
    </domain-config>
</network-security-config>
"#
    }

    /// Generates root and app build.gradle.kts.
    pub fn generate_build_gradle(package_name: &str) -> String {
        format!(
            r#"plugins {{
    id("com.android.application")
    kotlin("android")
}}

android {{
    namespace = "{package_name}"
    compileSdk = 34

    defaultConfig {{
        applicationId = "{package_name}"
        minSdk = 24
        targetSdk = 34
        versionCode = 1
        versionName = "0.5.0"
    }}

    buildTypes {{
        release {{
            isMinifyEnabled = true
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
        }}
    }}
}}

dependencies {{
    implementation("androidx.core:core-ktx:1.12.0")
    implementation("androidx.appcompat:appcompat:1.6.1")
}}
"#,
            package_name = package_name
        )
    }
}
