// Mantra Hybrid Mobile JavaScript Bridge Client
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
