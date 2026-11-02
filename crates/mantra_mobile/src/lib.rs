// ==============================================================================
// mantra_mobile — Mobile Runtime and Hybrid Bridge Framework
// ==============================================================================

pub mod android;
pub mod bridge;
pub mod ios;
pub mod lifecycle;
pub mod networking;
pub mod permissions;
pub mod security;
pub mod storage;

pub use android::AndroidBridgeTemplate;
pub use bridge::{BridgeErrorCode, BridgeRequest, BridgeResponse, BridgeRouter};
pub use ios::IosBridgeTemplate;
pub use lifecycle::{AppLifecycleState, LifecycleEvent, LifecycleManager};
pub use networking::{NativeHttpRequest, NativeHttpResponse, NetworkProxy};
pub use permissions::{PermissionStatus, PermissionType, PermissionsManager};
pub use security::{Capability, SecurityPolicy};
pub use storage::{MemoryStorageBackend, StorageBackend, StorageManager, StorageTier};

use std::sync::Arc;

/// Complete Mobile Runtime coordinating security, bridge dispatch, and native subsystems.
pub struct MobileRuntime {
    pub router: BridgeRouter,
    pub lifecycle: LifecycleManager,
    pub storage: StorageManager,
    pub permissions: PermissionsManager,
    pub network: NetworkProxy,
}

impl Default for MobileRuntime {
    fn default() -> Self {
        Self::new(SecurityPolicy::default())
    }
}

impl MobileRuntime {
    pub fn new(policy: SecurityPolicy) -> Self {
        let storage_backend = Arc::new(MemoryStorageBackend::new());
        let storage = StorageManager::new(storage_backend);
        let mut router = BridgeRouter::new(policy);

        // Register default platform handlers
        router.register("device.info", Capability::DeviceInfo, |_req| {
            Ok(format!(
                r#"{{"platform":"hybrid","engine":"mantra_v{}","arch":"wasm32/arm64"}}"#,
                env!("CARGO_PKG_VERSION")
            ))
        });

        router.register("lifecycle.status", Capability::DeviceInfo, |_req| {
            Ok(r#"{"state":"active","interactive":true}"#.to_string())
        });

        let read_storage = storage.clone();
        let secure_storage_granted = router.policy.has_capability(Capability::SecureStorage);
        router.register("storage.get", Capability::StorageRead, move |req| {
            let payload: serde_json::Value = serde_json::from_str(&req.payload)
                .map_err(|err| format!("Invalid storage payload: {err}"))?;
            let key = payload
                .get("key")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| "Storage request requires a string 'key'".to_string())?;
            let tier = storage_tier(&payload, secure_storage_granted)?;
            read_storage.handle_get(tier, key)
        });

        for action in ["storage.set", "storage.remove"] {
            let write_storage = storage.clone();
            let is_remove = action == "storage.remove";
            router.register(action, Capability::StorageWrite, move |req| {
                let payload: serde_json::Value = serde_json::from_str(&req.payload)
                    .map_err(|err| format!("Invalid storage payload: {err}"))?;
                let key = payload
                    .get("key")
                    .and_then(serde_json::Value::as_str)
                    .ok_or_else(|| "Storage request requires a string 'key'".to_string())?;
                let tier = storage_tier(&payload, secure_storage_granted)?;
                if is_remove {
                    write_storage.handle_remove(tier, key)
                } else {
                    let value = payload
                        .get("value")
                        .and_then(serde_json::Value::as_str)
                        .ok_or_else(|| "Storage set requires a string 'value'".to_string())?;
                    write_storage.handle_set(tier, key, value)
                }
            });
        }

        MobileRuntime {
            router,
            lifecycle: LifecycleManager::new(),
            storage,
            permissions: PermissionsManager::new(),
            network: NetworkProxy::default(),
        }
    }

    /// Handles an incoming raw message string from the WebView.
    pub fn handle_message(&self, raw_json: &str) -> String {
        self.router.dispatch_raw(raw_json)
    }
}

fn storage_tier(payload: &serde_json::Value, secure_granted: bool) -> Result<StorageTier, String> {
    match payload
        .get("tier")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("standard")
    {
        "standard" => Ok(StorageTier::Standard),
        "secure" if secure_granted => Ok(StorageTier::Secure),
        "secure" => Err("Secure storage capability has not been granted".to_string()),
        other => Err(format!("Unknown storage tier '{other}'")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_request_parse_and_response() {
        let raw = r#"{"id":"req_1","action":"device.info","origin":"file://","timestamp":1000}"#;
        let runtime = MobileRuntime::default();
        let resp_json = runtime.handle_message(raw);

        assert!(resp_json.contains(r#""id":"req_1""#));
        assert!(resp_json.contains(r#""success":true"#));
        assert!(resp_json.contains(r#""platform":"hybrid""#));
    }

    #[test]
    fn test_storage_bridge_dispatch_and_capability_checks() {
        let runtime = MobileRuntime::default();
        let set = runtime.handle_message(
            r#"{"id":"s1","action":"storage.set","origin":"file://","payload":{"key":"theme","value":"dark"}}"#,
        );
        assert!(set.contains("stored"));
        let get = runtime.handle_message(
            r#"{"id":"s2","action":"storage.get","origin":"file://","payload":{"key":"theme"}}"#,
        );
        let parsed: serde_json::Value = serde_json::from_str(&get).unwrap();
        assert_eq!(parsed["data"]["value"], "dark");

        let secure = runtime.handle_message(
            r#"{"id":"s3","action":"storage.set","origin":"file://","payload":{"tier":"secure","key":"token","value":"secret"}}"#,
        );
        let secure: serde_json::Value = serde_json::from_str(&secure).unwrap();
        assert_eq!(secure["success"], false);
    }

    #[test]
    fn test_security_policy_unauthorized_origin() {
        let raw = r#"{"id":"req_2","action":"device.info","origin":"https://malicious-site.com","timestamp":1000}"#;
        let runtime = MobileRuntime::default();
        let resp_json = runtime.handle_message(raw);

        assert!(resp_json.contains(r#""success":false"#));
        assert!(resp_json.contains("ERR_UNAUTHORIZED_ORIGIN"));
    }

    #[test]
    fn test_origin_allowlist_rejects_prefix_spoofing() {
        let policy = SecurityPolicy::default();
        assert!(!policy.is_origin_allowed("https://localhost.attacker.example"));
        assert!(!policy.is_origin_allowed("http://localhost:8080"));
        assert!(policy.is_origin_allowed("https://localhost"));
    }

    #[test]
    fn test_bridge_json_handles_escaped_and_nested_payloads() {
        let request = BridgeRequest::parse(
            r#"{"id":"line\\\"one","action":"device.info","origin":"file://","payload":{"nested":[1,{"x":"}"}]}}"#,
        ).unwrap();
        assert_eq!(request.id, "line\\\"one");
        let payload: serde_json::Value = serde_json::from_str(&request.payload).unwrap();
        assert_eq!(payload["nested"][1]["x"], "}");
        let response = BridgeResponse::error(
            request.id,
            BridgeErrorCode::ExecutionFailed,
            "bad \"value\"",
        );
        let parsed: serde_json::Value = serde_json::from_str(&response.to_json()).unwrap();
        assert_eq!(parsed["errorMessage"], "bad \"value\"");
    }

    #[test]
    fn test_storage_json_escapes_keys_and_values() {
        let backend = Arc::new(MemoryStorageBackend::new());
        let storage = StorageManager::new(backend);
        let key = "key\"\\name";
        let value = "value\n\"safe\"";
        storage
            .handle_set(StorageTier::Standard, key, value)
            .unwrap();
        let parsed: serde_json::Value =
            serde_json::from_str(&storage.handle_get(StorageTier::Standard, key).unwrap()).unwrap();
        assert_eq!(parsed["key"], key);
        assert_eq!(parsed["value"], value);
    }

    #[test]
    fn test_security_policy_capability_denial() {
        let mut policy = SecurityPolicy::default();
        policy.revoke(Capability::DeviceInfo);
        let runtime = MobileRuntime::new(policy);

        let raw = r#"{"id":"req_3","action":"device.info","origin":"file://","timestamp":1000}"#;
        let resp_json = runtime.handle_message(raw);

        assert!(resp_json.contains(r#""success":false"#));
        assert!(resp_json.contains("ERR_PERMISSION_DENIED"));
    }

    #[test]
    fn test_lifecycle_transitions() {
        let mut lm = LifecycleManager::new();
        assert_eq!(lm.current_state(), AppLifecycleState::Created);

        lm.handle_android_event(LifecycleEvent::AndroidOnResume);
        assert_eq!(lm.current_state(), AppLifecycleState::ForegroundActive);

        lm.handle_android_event(LifecycleEvent::AndroidOnPause);
        assert_eq!(lm.current_state(), AppLifecycleState::ForegroundInactive);

        lm.handle_android_event(LifecycleEvent::AndroidOnStop);
        assert_eq!(lm.current_state(), AppLifecycleState::BackgroundRunning);

        lm.handle_android_event(LifecycleEvent::AndroidOnDestroy);
        assert_eq!(lm.current_state(), AppLifecycleState::Destroyed);
    }

    #[test]
    fn test_permissions_manager() {
        let mut pm = PermissionsManager::new();
        assert_eq!(
            pm.check_permission(PermissionType::Camera),
            PermissionStatus::NotDetermined
        );

        let status = pm.request_permission(PermissionType::Camera);
        assert_eq!(status, PermissionStatus::Granted);
        assert_eq!(
            pm.check_permission(PermissionType::Camera),
            PermissionStatus::Granted
        );
    }

    #[test]
    fn test_storage_manager() {
        let backend = Arc::new(MemoryStorageBackend::new());
        let sm = StorageManager::new(backend);

        let set_res = sm.handle_set(StorageTier::Standard, "username", "mantra_dev");
        assert!(set_res.is_ok());

        let get_res = sm.handle_get(StorageTier::Standard, "username");
        assert!(get_res.is_ok());
        assert!(get_res.unwrap().contains("mantra_dev"));
    }

    #[test]
    fn test_network_proxy_ats_cleartext_block() {
        let proxy = NetworkProxy::new(true);
        let res = proxy.validate_url("http://insecure-api.com/data");
        assert!(res.is_err());
        assert!(res
            .unwrap_err()
            .contains("Cleartext HTTP traffic is blocked"));

        let ok_res = proxy.validate_url("https://secure-api.com/data");
        assert!(ok_res.is_ok());
    }

    #[test]
    fn test_android_and_ios_templates_generate() {
        let kt_bridge = AndroidBridgeTemplate::generate_bridge_kotlin("com.example.app");
        assert!(kt_bridge.contains("class MantraBridge"));

        let swift_bridge = IosBridgeTemplate::generate_bridge_swift();
        assert!(swift_bridge.contains("class MantraBridge: NSObject"));

        let swiftui = IosBridgeTemplate::generate_swiftui_content_view("TestApp", "");
        assert!(swiftui.contains("struct ContentView: View"));

        let android_layout = AndroidBridgeTemplate::generate_native_layout_xml("TestApp", "");
        assert!(android_layout.contains("ScrollView"));

        let plist = IosBridgeTemplate::generate_info_plist("com.example.app", "TestApp");
        assert!(plist.contains("NSCameraUsageDescription"));
    }
}
