// ==============================================================================
// mantra_mobile::bridge — Core Cross-Platform Bridge Protocol
// ==============================================================================

use crate::security::{Capability, SecurityPolicy};
use std::collections::HashMap;

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
        let trimmed = json_str.trim();
        if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
            return Err("Bridge payload must be a JSON object".to_string());
        }

        let id = extract_json_string(trimmed, "id").unwrap_or_else(|| "anonymous".to_string());
        let action = extract_json_string(trimmed, "action")
            .ok_or_else(|| "Missing required 'action' in bridge request".to_string())?;
        let origin = extract_json_string(trimmed, "origin")
            .unwrap_or_else(|| "file://".to_string());
        let payload = extract_json_raw_field(trimmed, "payload").unwrap_or_else(|| "{}".to_string());
        let timestamp = extract_json_number(trimmed, "timestamp").unwrap_or(0);

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
        let mut out = String::from("{");
        out.push_str(&format!(r#""id":"{}","#, escape_json(&self.id)));
        out.push_str(&format!(r#""success":{},"#, self.success));

        if let Some(ref data) = self.data {
            out.push_str(&format!(r#""data":{},"#, data));
        } else {
            out.push_str(r#""data":null,"#);
        }

        if let Some(ref err) = self.error_code {
            out.push_str(&format!(r#""errorCode":"{}","#, err.as_str()));
            out.push_str(&format!(
                r#""errorMessage":"{}""#,
                escape_json(self.error_message.as_deref().unwrap_or(""))
            ));
        } else {
            out.push_str(r#""errorCode":null,"errorMessage":null"#);
        }
        out.push('}');
        out
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
pub type ActionHandler = fn(&BridgeRequest) -> Result<String, String>;

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
    pub fn register(&mut self, action: &str, required_cap: Capability, handler: ActionHandler) {
        self.handlers.insert(action.to_string(), (required_cap, handler));
    }

    /// Dispatches an incoming raw JSON message string and returns serialized response.
    pub fn dispatch_raw(&self, raw_json: &str) -> String {
        // Enforce maximum payload size check
        if raw_json.len() > self.policy.max_payload_bytes {
            return BridgeResponse::error(
                "unknown".to_string(),
                BridgeErrorCode::PayloadTooLarge,
                &format!("Payload size {} exceeds maximum allowed {}", raw_json.len(), self.policy.max_payload_bytes),
            ).to_json();
        }

        let req = match BridgeRequest::parse(raw_json) {
            Ok(r) => r,
            Err(e) => {
                return BridgeResponse::error(
                    "unknown".to_string(),
                    BridgeErrorCode::InvalidPayload,
                    &e,
                ).to_json();
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
                &format!("Origin '{}' is not permitted by mobile security policy", req.origin),
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
                &format!("Action '{}' requires capability '{}' which has not been granted", req.action, required_cap.as_str()),
            );
        }

        // 4. Execution
        match handler(req) {
            Ok(json_data) => BridgeResponse::success(req.id.clone(), &json_data),
            Err(err_msg) => BridgeResponse::error(
                req.id.clone(),
                BridgeErrorCode::ExecutionFailed,
                &err_msg,
            ),
        }
    }
}

// Minimal zero-dependency JSON extraction helpers
fn extract_json_string(json: &str, key: &str) -> Option<String> {
    let pattern = format!(r#""{}""#, key);
    let key_pos = json.find(&pattern)?;
    let after_key = &json[key_pos + pattern.len()..];
    let colon_pos = after_key.find(':')?;
    let after_colon = after_key[colon_pos + 1..].trim_start();

    if !after_colon.starts_with('"') {
        return None;
    }
    let content = &after_colon[1..];
    let end_quote = content.find('"')?;
    Some(content[..end_quote].to_string())
}

fn extract_json_number(json: &str, key: &str) -> Option<u64> {
    let pattern = format!(r#""{}""#, key);
    let key_pos = json.find(&pattern)?;
    let after_key = &json[key_pos + pattern.len()..];
    let colon_pos = after_key.find(':')?;
    let after_colon = after_key[colon_pos + 1..].trim_start();

    let num_str: String = after_colon.chars().take_while(|c| c.is_ascii_digit()).collect();
    num_str.parse::<u64>().ok()
}

fn extract_json_raw_field(json: &str, key: &str) -> Option<String> {
    let pattern = format!(r#""{}""#, key);
    let key_pos = json.find(&pattern)?;
    let after_key = &json[key_pos + pattern.len()..];
    let colon_pos = after_key.find(':')?;
    let after_colon = after_key[colon_pos + 1..].trim_start();

    if after_colon.starts_with('{') {
        let mut depth = 0;
        let mut end = 0;
        for (i, c) in after_colon.char_indices() {
            if c == '{' {
                depth += 1;
            } else if c == '}' {
                depth -= 1;
                if depth == 0 {
                    end = i + 1;
                    break;
                }
            }
        }
        if end > 0 {
            return Some(after_colon[..end].to_string());
        }
    }
    None
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

