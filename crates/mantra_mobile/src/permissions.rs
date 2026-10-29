// ==============================================================================
// mantra_mobile::permissions — Unified Native Permissions Bridge
// ==============================================================================

use std::collections::HashMap;

/// Standard permission types across Android and iOS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PermissionType {
    Camera,
    FineLocation,
    CoarseLocation,
    Notifications,
    Storage,
    Microphone,
}

impl PermissionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            PermissionType::Camera => "camera",
            PermissionType::FineLocation => "location_fine",
            PermissionType::CoarseLocation => "location_coarse",
            PermissionType::Notifications => "notifications",
            PermissionType::Storage => "storage",
            PermissionType::Microphone => "microphone",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "camera" => Some(PermissionType::Camera),
            "location_fine" | "location" => Some(PermissionType::FineLocation),
            "location_coarse" => Some(PermissionType::CoarseLocation),
            "notifications" => Some(PermissionType::Notifications),
            "storage" => Some(PermissionType::Storage),
            "microphone" => Some(PermissionType::Microphone),
            _ => None,
        }
    }

    /// Android manifest permission name
    pub fn android_permission(&self) -> &'static str {
        match self {
            PermissionType::Camera => "android.permission.CAMERA",
            PermissionType::FineLocation => "android.permission.ACCESS_FINE_LOCATION",
            PermissionType::CoarseLocation => "android.permission.ACCESS_COARSE_LOCATION",
            PermissionType::Notifications => "android.permission.POST_NOTIFICATIONS",
            PermissionType::Storage => "android.permission.READ_EXTERNAL_STORAGE",
            PermissionType::Microphone => "android.permission.RECORD_AUDIO",
        }
    }

    /// iOS Info.plist usage description key
    pub fn ios_usage_description_key(&self) -> &'static str {
        match self {
            PermissionType::Camera => "NSCameraUsageDescription",
            PermissionType::FineLocation => "NSLocationWhenInUseUsageDescription",
            PermissionType::CoarseLocation => "NSLocationWhenInUseUsageDescription",
            PermissionType::Notifications => "NSUserNotificationUsageDescription",
            PermissionType::Storage => "NSPhotoLibraryUsageDescription",
            PermissionType::Microphone => "NSMicrophoneUsageDescription",
        }
    }
}

/// Unified permission status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionStatus {
    Granted,
    Denied,
    PermanentlyDenied,
    NotDetermined,
}

impl PermissionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            PermissionStatus::Granted => "granted",
            PermissionStatus::Denied => "denied",
            PermissionStatus::PermanentlyDenied => "permanently_denied",
            PermissionStatus::NotDetermined => "not_determined",
        }
    }
}

/// Permissions Manager managing permission grants and simulation.
pub struct PermissionsManager {
    registry: HashMap<PermissionType, PermissionStatus>,
}

impl Default for PermissionsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PermissionsManager {
    pub fn new() -> Self {
        PermissionsManager {
            registry: HashMap::new(),
        }
    }

    pub fn check_permission(&self, permission: PermissionType) -> PermissionStatus {
        self.registry.get(&permission).copied().unwrap_or(PermissionStatus::NotDetermined)
    }

    pub fn set_permission(&mut self, permission: PermissionType, status: PermissionStatus) {
        self.registry.insert(permission, status);
    }

    /// Simulates requesting a permission.
    pub fn request_permission(&mut self, permission: PermissionType) -> PermissionStatus {
        let current = self.check_permission(permission);
        if current == PermissionStatus::Granted || current == PermissionStatus::PermanentlyDenied {
            return current;
        }
        // In default simulator mode, grant the permission
        self.registry.insert(permission, PermissionStatus::Granted);
        PermissionStatus::Granted
    }

    pub fn to_json(&self, permission: PermissionType, status: PermissionStatus) -> String {
        format!(
            r#"{{"permission":"{}","status":"{}"}}"#,
            permission.as_str(),
            status.as_str()
        )
    }
}

