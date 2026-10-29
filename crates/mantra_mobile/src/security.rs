// ==============================================================================
// mantra_mobile::security — Explicit Security Boundaries & Capabilities
// ==============================================================================

use std::collections::HashSet;

/// Granular native capabilities granted to the web environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    /// Read access to non-sensitive key-value local storage
    StorageRead,
    /// Write access to non-sensitive key-value local storage
    StorageWrite,
    /// Access to hardware-backed secure storage (Keystore/Keychain)
    SecureStorage,
    /// Outbound network requests via native proxy
    NetworkFetch,
    /// Camera hardware access and photo capture
    Camera,
    /// Coarse and fine location tracking
    Location,
    /// Push and local notification scheduling
    Notifications,
    /// Device vibration and haptic feedback
    Haptics,
    /// Access to native device identity and OS version
    DeviceInfo,
}

impl Capability {
    pub fn as_str(&self) -> &'static str {
        match self {
            Capability::StorageRead => "storage:read",
            Capability::StorageWrite => "storage:write",
            Capability::SecureStorage => "storage:secure",
            Capability::NetworkFetch => "network:fetch",
            Capability::Camera => "hardware:camera",
            Capability::Location => "hardware:location",
            Capability::Notifications => "service:notifications",
            Capability::Haptics => "hardware:haptics",
            Capability::DeviceInfo => "device:info",
        }
    }
}

/// Security Policy governing WebView execution.
#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    /// Permitted origins for bridge messages (e.g., "file://", "https://localhost")
    pub allowed_origins: HashSet<String>,
    /// Capabilities granted to the web application
    pub granted_capabilities: HashSet<Capability>,
    /// Permitted domain patterns for native network proxy
    pub network_allowlist: Vec<String>,
    /// Maximum payload size in bytes (prevents buffer overflow / DoS attacks)
    pub max_payload_bytes: usize,
    /// Whether to enforce Content Security Policy headers
    pub enforce_csp: bool,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        let mut origins = HashSet::new();
        origins.insert("file://".to_string());
        origins.insert("https://localhost".to_string());
        origins.insert("http://localhost".to_string());

        let mut capabilities = HashSet::new();
        capabilities.insert(Capability::StorageRead);
        capabilities.insert(Capability::StorageWrite);
        capabilities.insert(Capability::NetworkFetch);
        capabilities.insert(Capability::DeviceInfo);

        SecurityPolicy {
            allowed_origins: origins,
            granted_capabilities: capabilities,
            network_allowlist: vec!["https://*".to_string()],
            max_payload_bytes: 1024 * 1024, // 1 MiB default limit
            enforce_csp: true,
        }
    }
}

impl SecurityPolicy {
    /// Creates a strict production security policy.
    pub fn production(allowed_origin: &str) -> Self {
        let mut origins = HashSet::new();
        origins.insert(allowed_origin.to_string());

        let mut capabilities = HashSet::new();
        capabilities.insert(Capability::StorageRead);
        capabilities.insert(Capability::StorageWrite);
        capabilities.insert(Capability::NetworkFetch);

        SecurityPolicy {
            allowed_origins: origins,
            granted_capabilities: capabilities,
            network_allowlist: Vec::new(),
            max_payload_bytes: 512 * 1024, // 512 KiB
            enforce_csp: true,
        }
    }

    /// Validates if an origin is permitted to invoke native capabilities.
    pub fn is_origin_allowed(&self, origin: &str) -> bool {
        if self.allowed_origins.contains("*") {
            return true;
        }
        for allowed in &self.allowed_origins {
            if origin.starts_with(allowed) {
                return true;
            }
        }
        false
    }

    /// Checks if a capability is explicitly granted.
    pub fn has_capability(&self, capability: Capability) -> bool {
        self.granted_capabilities.contains(&capability)
    }

    /// Grants a new capability.
    pub fn grant(&mut self, capability: Capability) {
        self.granted_capabilities.insert(capability);
    }

    /// Revokes a capability.
    pub fn revoke(&mut self, capability: Capability) {
        self.granted_capabilities.remove(&capability);
    }

    /// Validates if a target URL is permitted by the network allowlist.
    pub fn is_url_allowed(&self, url: &str) -> bool {
        if self.network_allowlist.is_empty() {
            return false;
        }
        for pattern in &self.network_allowlist {
            if pattern == "*" || pattern == "https://*" && url.starts_with("https://") {
                return true;
            }
            if url.starts_with(pattern) {
                return true;
            }
        }
        false
    }

    /// Generates standard Content-Security-Policy (CSP) headers for the WebView.
    pub fn generate_csp_meta_tag(&self) -> String {
        r#"<meta http-equiv="Content-Security-Policy" content="default-src 'self' file: data:; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; connect-src 'self' https:; img-src 'self' data: https:;">"#.to_string()
    }
}

