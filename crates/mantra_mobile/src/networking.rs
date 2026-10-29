// ==============================================================================
// mantra_mobile::networking — Native Mobile Network Proxy & ATS
// ==============================================================================

use std::collections::HashMap;

/// Native HTTP Request specification.
#[derive(Debug, Clone)]
pub struct NativeHttpRequest {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub timeout_ms: u64,
}

/// Native HTTP Response specification.
#[derive(Debug, Clone)]
pub struct NativeHttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub latency_ms: u64,
}

impl NativeHttpResponse {
    pub fn to_json(&self) -> String {
        let mut out = String::from("{");
        out.push_str(&format!(r#""status":{},"#, self.status));
        out.push_str(&format!(r#""latencyMs":{},"#, self.latency_ms));
        out.push_str(&format!(r#""body":"{}""#, escape_json(&self.body)));
        out.push('}');
        out
    }
}

/// Managed Network Proxy validating security policies before dispatch.
pub struct NetworkProxy {
    pub block_cleartext_traffic: bool,
    pub default_timeout_ms: u64,
}

impl Default for NetworkProxy {
    fn default() -> Self {
        NetworkProxy {
            block_cleartext_traffic: true,
            default_timeout_ms: 15_000,
        }
    }
}

impl NetworkProxy {
    pub fn new(block_cleartext: bool) -> Self {
        NetworkProxy {
            block_cleartext_traffic: block_cleartext,
            default_timeout_ms: 15_000,
        }
    }

    /// Validates request URL against App Transport Security (ATS) rules.
    pub fn validate_url(&self, url: &str) -> Result<(), String> {
        if self.block_cleartext_traffic && url.starts_with("http://") {
            return Err("Cleartext HTTP traffic is blocked by native security policy. Use HTTPS.".to_string());
        }
        if !url.starts_with("https://") && !url.starts_with("http://") {
            return Err("Invalid protocol. Only HTTP and HTTPS are permitted.".to_string());
        }
        Ok(())
    }

    /// Simulates native network request execution for verification.
    pub fn execute_simulated(&self, req: &NativeHttpRequest) -> Result<NativeHttpResponse, String> {
        self.validate_url(&req.url)?;

        // Return a mock successful HTTP 200 response
        let mut resp_headers = HashMap::new();
        resp_headers.insert("content-type".to_string(), "application/json".to_string());

        Ok(NativeHttpResponse {
            status: 200,
            headers: resp_headers,
            body: r#"{"status":"ok","proxy":"mantra_native_networking"}"#.to_string(),
            latency_ms: 45,
        })
    }
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

