// ==============================================================================
// mantra_mobile::bridge — Core Cross-Platform Bridge Protocol
// ==============================================================================

use crate::security::{Capability, SecurityPolicy};
use std::collections::HashMap;
use std::sync::Arc;

/// Result status for bridge operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeErrorCode {
    UnauthorizedOrigin,
    PermissionDenied,
    UnknownAction,
    InvalidPayload,
    PayloadTooLarge,
    ExecutionFailed,
}

impl BridgeErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            BridgeErrorCode::UnauthorizedOrigin => "ERR_UNAUTHORIZED_ORIGIN",
            BridgeErrorCode::PermissionDenied => "ERR_PERMISSION_DENIED",
            BridgeErrorCode::UnknownAction => "ERR_UNKNOWN_ACTION",
            BridgeErrorCode::InvalidPayload => "ERR_INVALID_PAYLOAD",
            BridgeErrorCode::PayloadTooLarge => "ERR_PAYLOAD_TOO_LARGE",
            BridgeErrorCode::ExecutionFailed => "ERR_EXECUTION_FAILED",
        }
    }
}

/// Standardized incoming bridge message from WebView.
#[derive(Debug, Clone)]
pub struct BridgeRequest {
    pub id: String,
    pub action: String,
    pub payload: String,
    pub origin: String,
    pub timestamp: u64,
}

impl BridgeRequest {
    /// Parses a raw JSON string into a BridgeRequest.
    pub fn parse(json_str: &str) -> Result<Self, String> {
        let value: serde_json::Value =
            serde_json::from_str(json_str).map_err(|err| format!("Invalid bridge JSON: {err}"))?;
        let object = value
            .as_object()
            .ok_or_else(|| "Bridge payload must be a JSON object".to_string())?;

        let id = object
            .get("id")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("anonymous")
            .to_string();
        let action = object
            .get("action")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| "Missing required string 'action' in bridge request".to_string())?
            .to_string();
        let origin = object
            .get("origin")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("file://")
            .to_string();
        let payload = object
            .get("payload")
            .map(serde_json::Value::to_string)
            .unwrap_or_else(|| "{}".to_string());
        let timestamp = object
            .get("timestamp")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0);

        Ok(BridgeRequest {
            id,
            action,
            payload,
            origin,
            timestamp,
        })
    }
}

/// Standardized outgoing response dispatched back to WebView.
#[derive(Debug, Clone)]
pub struct BridgeResponse {
    pub id: String,
    pub success: bool,
    pub data: Option<String>,
    pub error_code: Option<BridgeErrorCode>,
    pub error_message: Option<String>,
}

impl BridgeResponse {
    pub fn success(id: String, json_data: &str) -> Self {
        BridgeResponse {
            id,
            success: true,
            data: Some(json_data.to_string()),
            error_code: None,
            error_message: None,
        }
    }

    pub fn error(id: String, code: BridgeErrorCode, message: &str) -> Self {
        BridgeResponse {
            id,
            success: false,
            data: None,
            error_code: Some(code),
            error_message: Some(message.to_string()),
        }
    }

    /// Serializes response to standard JSON for injection into WebView.
    pub fn to_json(&self) -> String {
        let data = self
            .data
            .as_deref()
            .and_then(|raw| serde_json::from_str::<serde_json::Value>(raw).ok())
            .unwrap_or(serde_json::Value::Null);
        serde_json::json!({
            "id": self.id,
            "success": self.success,
            "data": data,
            "errorCode": self.error_code.as_ref().map(BridgeErrorCode::as_str),
            "errorMessage": self.error_message,
        })
        .to_string()
    }

    /// Formats JavaScript evaluation snippet to deliver callback to window.MantraBridge.
    pub fn to_js_callback(&self) -> String {
        let json = self.to_json();
        format!(
            "if (window.MantraBridge && window.MantraBridge.__onNativeResponse) {{ window.MantraBridge.__onNativeResponse({}); }}",
            json
        )
    }
}

/// Action handler signature.
pub type ActionHandler = Arc<dyn Fn(&BridgeRequest) -> Result<String, String> + Send + Sync>;

/// Bridge Router mapping actions to capabilities and native handlers.
pub struct BridgeRouter {
    pub policy: SecurityPolicy,
    handlers: HashMap<String, (Capability, ActionHandler)>,
}

impl BridgeRouter {
    pub fn new(policy: SecurityPolicy) -> Self {
        BridgeRouter {
            policy,
            handlers: HashMap::new(),
        }
    }

    /// Registers an action with its required security capability.
    pub fn register<F>(&mut self, action: &str, required_cap: Capability, handler: F)
    where
        F: Fn(&BridgeRequest) -> Result<String, String> + Send + Sync + 'static,
    {
        self.handlers
            .insert(action.to_string(), (required_cap, Arc::new(handler)));
    }

    /// Dispatches an incoming raw JSON message string and returns serialized response.
    pub fn dispatch_raw(&self, raw_json: &str) -> String {
        // Enforce maximum payload size check
        if raw_json.len() > self.policy.max_payload_bytes {
            return BridgeResponse::error(
                "unknown".to_string(),
                BridgeErrorCode::PayloadTooLarge,
                &format!(
                    "Payload size {} exceeds maximum allowed {}",
                    raw_json.len(),
                    self.policy.max_payload_bytes
                ),
            )
            .to_json();
        }

        let req = match BridgeRequest::parse(raw_json) {
            Ok(r) => r,
            Err(e) => {
                return BridgeResponse::error(
                    "unknown".to_string(),
                    BridgeErrorCode::InvalidPayload,
                    &e,
                )
                .to_json();
            }
        };

        let resp = self.dispatch_request(&req);
        resp.to_json()
    }

    /// Dispatches a structured request through security boundaries.
    pub fn dispatch_request(&self, req: &BridgeRequest) -> BridgeResponse {
        // 1. Origin verification
        if !self.policy.is_origin_allowed(&req.origin) {
            return BridgeResponse::error(
                req.id.clone(),
                BridgeErrorCode::UnauthorizedOrigin,
                &format!(
                    "Origin '{}' is not permitted by mobile security policy",
                    req.origin
                ),
            );
        }

        // 2. Action resolution
        let (required_cap, handler) = match self.handlers.get(&req.action) {
            Some(pair) => pair,
            None => {
                return BridgeResponse::error(
                    req.id.clone(),
                    BridgeErrorCode::UnknownAction,
                    &format!("No native handler registered for action '{}'", req.action),
                );
            }
        };

        // 3. Capability authorization
        if !self.policy.has_capability(*required_cap) {
            return BridgeResponse::error(
                req.id.clone(),
                BridgeErrorCode::PermissionDenied,
                &format!(
                    "Action '{}' requires capability '{}' which has not been granted",
                    req.action,
                    required_cap.as_str()
                ),
            );
        }

        // 4. Execution
        match handler(req) {
            Ok(json_data) => BridgeResponse::success(req.id.clone(), &json_data),
            Err(err_msg) => {
                BridgeResponse::error(req.id.clone(), BridgeErrorCode::ExecutionFailed, &err_msg)
            }
        }
    }
}
