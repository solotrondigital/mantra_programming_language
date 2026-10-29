// ==============================================================================
// mantra_mobile::storage — Cross-Platform Storage & Hardware Security
// ==============================================================================

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Storage tier requested by the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageTier {
    /// Non-sensitive preference data (SharedPreferences / UserDefaults)
    Standard,
    /// Hardware-encrypted data (Android KeyStore / iOS Keychain)
    Secure,
}

/// Abstract storage backend provider.
pub trait StorageBackend: Send + Sync {
    fn get(&self, tier: StorageTier, key: &str) -> Option<String>;
    fn set(&self, tier: StorageTier, key: &str, value: &str) -> Result<(), String>;
    fn remove(&self, tier: StorageTier, key: &str) -> Result<(), String>;
    fn clear(&self, tier: StorageTier) -> Result<(), String>;
    fn keys(&self, tier: StorageTier) -> Vec<String>;
}

/// In-memory thread-safe reference implementation of storage.
#[derive(Clone, Default)]
pub struct MemoryStorageBackend {
    standard_store: Arc<Mutex<HashMap<String, String>>>,
    secure_store: Arc<Mutex<HashMap<String, String>>>,
}

impl MemoryStorageBackend {
    pub fn new() -> Self {
        MemoryStorageBackend {
            standard_store: Arc::new(Mutex::new(HashMap::new())),
            secure_store: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl StorageBackend for MemoryStorageBackend {
    fn get(&self, tier: StorageTier, key: &str) -> Option<String> {
        let store = match tier {
            StorageTier::Standard => self.standard_store.lock().unwrap(),
            StorageTier::Secure => self.secure_store.lock().unwrap(),
        };
        store.get(key).cloned()
    }

    fn set(&self, tier: StorageTier, key: &str, value: &str) -> Result<(), String> {
        let mut store = match tier {
            StorageTier::Standard => self.standard_store.lock().unwrap(),
            StorageTier::Secure => self.secure_store.lock().unwrap(),
        };
        store.insert(key.to_string(), value.to_string());
        Ok(())
    }

    fn remove(&self, tier: StorageTier, key: &str) -> Result<(), String> {
        let mut store = match tier {
            StorageTier::Standard => self.standard_store.lock().unwrap(),
            StorageTier::Secure => self.secure_store.lock().unwrap(),
        };
        store.remove(key);
        Ok(())
    }

    fn clear(&self, tier: StorageTier) -> Result<(), String> {
        let mut store = match tier {
            StorageTier::Standard => self.standard_store.lock().unwrap(),
            StorageTier::Secure => self.secure_store.lock().unwrap(),
        };
        store.clear();
        Ok(())
    }

    fn keys(&self, tier: StorageTier) -> Vec<String> {
        let store = match tier {
            StorageTier::Standard => self.standard_store.lock().unwrap(),
            StorageTier::Secure => self.secure_store.lock().unwrap(),
        };
        store.keys().cloned().collect()
    }
}

/// High-level Storage Manager providing JSON serialization for bridge requests.
pub struct StorageManager {
    backend: Arc<dyn StorageBackend>,
}

impl StorageManager {
    pub fn new(backend: Arc<dyn StorageBackend>) -> Self {
        StorageManager { backend }
    }

    pub fn handle_get(&self, tier: StorageTier, key: &str) -> Result<String, String> {
        match self.backend.get(tier, key) {
            Some(val) => Ok(format!(r#"{{"key":"{}","value":"{}"}}"#, key, val)),
            None => Ok(format!(r#"{{"key":"{}","value":null}}"#, key)),
        }
    }

    pub fn handle_set(&self, tier: StorageTier, key: &str, value: &str) -> Result<String, String> {
        self.backend.set(tier, key, value)?;
        Ok(format!(r#"{{"key":"{}","status":"stored"}}"#, key))
    }

    pub fn handle_remove(&self, tier: StorageTier, key: &str) -> Result<String, String> {
        self.backend.remove(tier, key)?;
        Ok(format!(r#"{{"key":"{}","status":"removed"}}"#, key))
    }
}

