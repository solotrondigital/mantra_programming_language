// ==============================================================================
// mantra_bootstrap::plugin — Compiler Plugin Registry (Security-Gated)
// ==============================================================================
//
// MVP 3 acceptance criterion:
//   "Add compiler plugins only if their security and stability model is clearly
//    specified."
//
// This module provides a plugin registration mechanism where every plugin must:
//   1. Declare a semver-compatible API version.
//   2. Pass a stability tier check (Stable | Experimental | Unsafe).
//   3. Declare its capability scope (ReadOnly, ReadWrite, or Codegen).
//   4. Not be loaded in production builds when PluginTier::Unsafe unless
//      MANTRA_ALLOW_UNSAFE_PLUGINS=1 is explicitly set.

/// The stability tier of a compiler plugin.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PluginTier {
    /// Fully reviewed, stable API surface. Safe to load without extra flags.
    Stable,
    /// Useful but not yet stabilised. Requires --experimental-plugins.
    Experimental,
    /// May perform arbitrary operations. Requires explicit env opt-in.
    Unsafe,
}

impl std::fmt::Display for PluginTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginTier::Stable       => write!(f, "stable"),
            PluginTier::Experimental => write!(f, "experimental"),
            PluginTier::Unsafe       => write!(f, "unsafe"),
        }
    }
}

/// What a plugin is permitted to do during compilation.
#[derive(Debug, Clone, PartialEq)]
pub enum PluginScope {
    /// May only inspect the AST/IR — no modifications allowed.
    ReadOnly,
    /// May rewrite the AST/IR within documented constraints.
    ReadWrite,
    /// May emit additional code-generation artefacts.
    Codegen,
}

/// Metadata every compiler plugin must provide.
#[derive(Debug, Clone)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub api_version: String,
    pub tier: PluginTier,
    pub scope: PluginScope,
    pub description: String,
}

impl PluginManifest {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Plugin name must not be empty".to_string());
        }
        if self.version.is_empty() || self.api_version.is_empty() {
            return Err("Plugin must declare both version and api_version".to_string());
        }
        if self.api_version != crate::version::MANTRA_VERSION_STR {
            return Err(format!(
                "Plugin '{}' api_version '{}' is not compatible with compiler version '{}'",
                self.name, self.api_version, crate::version::MANTRA_VERSION_STR
            ));
        }
        Ok(())
    }
}

/// The plugin registry that enforces security and stability gates.
#[derive(Default)]
pub struct PluginRegistry {
    plugins: Vec<PluginManifest>,
    allow_experimental: bool,
    allow_unsafe: bool,
}

impl PluginRegistry {
    pub fn new() -> Self {
        PluginRegistry {
            plugins: Vec::new(),
            allow_experimental: false,
            allow_unsafe: false,
        }
    }

    pub fn with_experimental(mut self) -> Self {
        self.allow_experimental = true;
        self
    }

    /// Opt-in to loading Unsafe-tier plugins. Only valid when the environment
    /// variable MANTRA_ALLOW_UNSAFE_PLUGINS=1 is set.
    pub fn with_unsafe_if_env(mut self) -> Self {
        if std::env::var("MANTRA_ALLOW_UNSAFE_PLUGINS").as_deref() == Ok("1") {
            self.allow_unsafe = true;
        }
        self
    }

    /// Register a plugin after passing the security gate.
    pub fn register(&mut self, manifest: PluginManifest) -> Result<(), String> {
        manifest.validate()?;

        match manifest.tier {
            PluginTier::Stable => {}
            PluginTier::Experimental => {
                if !self.allow_experimental {
                    return Err(format!(
                        "Plugin '{}' is marked experimental. Pass --experimental-plugins to allow.",
                        manifest.name
                    ));
                }
            }
            PluginTier::Unsafe => {
                if !self.allow_unsafe {
                    return Err(format!(
                        "Plugin '{}' is marked unsafe. Set MANTRA_ALLOW_UNSAFE_PLUGINS=1 to allow.",
                        manifest.name
                    ));
                }
            }
        }

        self.plugins.push(manifest);
        Ok(())
    }

    pub fn loaded_plugins(&self) -> &[PluginManifest] {
        &self.plugins
    }

    pub fn summary(&self) -> String {
        if self.plugins.is_empty() {
            return "No compiler plugins loaded.".to_string();
        }
        self.plugins
            .iter()
            .map(|p| format!("  {} v{} [{}]", p.name, p.version, p.tier))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stable_plugin(name: &str) -> PluginManifest {
        PluginManifest {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            api_version: crate::version::MANTRA_VERSION_STR.to_string(),
            tier: PluginTier::Stable,
            scope: PluginScope::ReadOnly,
            description: "Test plugin".to_string(),
        }
    }

    #[test]
    fn stable_plugin_registers() {
        let mut reg = PluginRegistry::new();
        assert!(reg.register(stable_plugin("test-lint")).is_ok());
        assert_eq!(reg.loaded_plugins().len(), 1);
    }

    #[test]
    fn experimental_blocked_without_flag() {
        let mut reg = PluginRegistry::new();
        let mut p = stable_plugin("exp-plugin");
        p.tier = PluginTier::Experimental;
        assert!(reg.register(p).is_err());
    }

    #[test]
    fn experimental_allowed_with_flag() {
        let mut reg = PluginRegistry::new().with_experimental();
        let mut p = stable_plugin("exp-plugin");
        p.tier = PluginTier::Experimental;
        assert!(reg.register(p).is_ok());
    }

    #[test]
    fn wrong_api_version_rejected() {
        let mut reg = PluginRegistry::new();
        let mut p = stable_plugin("old-plugin");
        p.api_version = "0.1.0".to_string(); // Not 0.3.0
        assert!(reg.register(p).is_err());
    }
}

