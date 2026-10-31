//! Platform bridge script generator for Android, iOS, and Web event dispatching.

/// Generates the cross-platform bridge client script embedded in the UI layer.
pub struct PlatformBridge;

impl PlatformBridge {
    pub fn client_script() -> &'static str {
        r#"<script>
(function() {
    window.MantraUI = {
        emit: function(eventType, targetId, value) {
            var payload = JSON.stringify({
                event_type: eventType,
                target_id: targetId,
                value: value || null,
                timestamp_ms: Date.now()
            });

            // 1. Android Native WebView Interface
            if (window.MantraAndroidBridge && typeof window.MantraAndroidBridge.postMessage === 'function') {
                window.MantraAndroidBridge.postMessage(payload);
            }
            // 2. iOS WebKit Script Message Handler
            else if (window.webkit && window.webkit.messageHandlers && window.webkit.messageHandlers.mantraBridge) {
                window.webkit.messageHandlers.mantraBridge.postMessage(payload);
            }
            // 3. Web / WASM Event Target
            else {
                window.dispatchEvent(new CustomEvent('mantra-event', { detail: payload }));
            }
        }
    };
})();
</script>"#
    }
}

