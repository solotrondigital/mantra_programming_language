// ==============================================================================
// mantra_bootstrap::version — Compiler Version Metadata
// ==============================================================================
//
// CompilerVersion is embedded into every Mantra-compiled artifact header so
// that stage-0, stage-1 and stage-2 outputs carry an unambiguous identity.
// The version number follows Mantra's own semver policy.

/// Canonical version constants for Mantra v0.3.0 (Self-Hosting Compiler).
pub const MANTRA_VERSION_MAJOR: u32 = 0;
pub const MANTRA_VERSION_MINOR: u32 = 7;
pub const MANTRA_VERSION_PATCH: u32 = 0;
pub const MANTRA_VERSION_STR: &str   = "0.7.0";
pub const MANTRA_EDITION: &str       = "2026";
pub const MANTRA_CODENAME: &str      = "production-stable";

/// IR schema version — must be bumped whenever the IrContract changes in a
/// way that produces different serialised output.
pub const IR_SCHEMA_VERSION: &str    = "1.0";

/// AST schema version — must be bumped whenever the AST S-expression format
/// changes.
pub const AST_SCHEMA_VERSION: &str   = "1.0";

/// Rust toolchain used for stage-0 builds.
pub const RUST_EDITION: &str         = "2021";

#[derive(Debug, Clone, PartialEq)]
pub struct CompilerVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub stage: BootstrapStage,
    pub codename: String,
    pub edition: String,
    pub ir_schema: String,
}

/// The three bootstrap stages defined in the MVP 3 acceptance criteria.
#[derive(Debug, Clone, PartialEq)]
pub enum BootstrapStage {
    /// Stage 0: Rust-based bootstrap compiler (mantrac, built from Rust source).
    RustBootstrap,
    /// Stage 1: Self-hosted Mantra compiler built by the stage-0 binary.
    MantraStage1,
    /// Stage 2: Re-compilation of the stage-1 source by the stage-1 binary.
    /// Output must be bit-identical to stage-1 output for acceptance.
    MantraStage2Verify,
}

impl std::fmt::Display for BootstrapStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BootstrapStage::RustBootstrap      => write!(f, "stage-0 (Rust bootstrap)"),
            BootstrapStage::MantraStage1       => write!(f, "stage-1 (Mantra self-hosted)"),
            BootstrapStage::MantraStage2Verify => write!(f, "stage-2 (reproducibility verify)"),
        }
    }
}

impl CompilerVersion {
    pub fn current(stage: BootstrapStage) -> Self {
        CompilerVersion {
            major:     MANTRA_VERSION_MAJOR,
            minor:     MANTRA_VERSION_MINOR,
            patch:     MANTRA_VERSION_PATCH,
            stage,
            codename:  MANTRA_CODENAME.to_string(),
            edition:   MANTRA_EDITION.to_string(),
            ir_schema: IR_SCHEMA_VERSION.to_string(),
        }
    }

    /// Human-readable one-line version string emitted by `mantrac --version`.
    pub fn to_version_string(&self) -> String {
        format!(
            "mantrac {}.{}.{} ({}) [{}] IR-schema={} edition={}",
            self.major, self.minor, self.patch,
            self.codename,
            self.stage,
            self.ir_schema,
            self.edition,
        )
    }

    /// Compact header inserted into every generated artifact (AST file, token
    /// stream, compiled binary RDATA section).
    pub fn to_artifact_header(&self) -> String {
        format!(
            "; mantrac-self {}.{}.{} stage={} ir_schema={} edition={}",
            self.major, self.minor, self.patch,
            match &self.stage {
                BootstrapStage::RustBootstrap      => 0,
                BootstrapStage::MantraStage1       => 1,
                BootstrapStage::MantraStage2Verify => 2,
            },
            self.ir_schema,
            self.edition,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_string_contains_semver() {
        let v = CompilerVersion::current(BootstrapStage::RustBootstrap);
        let s = v.to_version_string();
        assert!(s.contains("0.7.0"), "Expected 0.3.0 in version string: {}", s);
    }

    #[test]
    fn artifact_header_contains_stage() {
        let v1 = CompilerVersion::current(BootstrapStage::MantraStage1);
        assert!(v1.to_artifact_header().contains("stage=1"));
        let v2 = CompilerVersion::current(BootstrapStage::MantraStage2Verify);
        assert!(v2.to_artifact_header().contains("stage=2"));
    }
}

