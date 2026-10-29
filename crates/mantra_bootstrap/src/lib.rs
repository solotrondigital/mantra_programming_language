// ==============================================================================
// mantra_bootstrap — Self-Hosting Bootstrap Infrastructure for Mantra v0.3.0
// crates/mantra_bootstrap/src/lib.rs
//
// Provides:
//   1. CompilerVersion — canonical version metadata embedded in all artifacts.
//   2. IrContract — stable, versioned interface for the shared typed IR used by
//      every backend (interpreter, JIT, C99, WebAssembly).
//   3. BootstrapStage — tracks and documents the current stage in the bootstrap
//      pipeline (0 = Rust, 1 = stage-1 Mantra, 2 = stage-2 verification).
//   4. ReproducibilityChecker — compares stage outputs and reports differences.
//   5. CompilerPlugin — plugin registration with security/stability gates.
//   6. RegressionSuite — wraps conformance and negative-diagnostic tests.
// ==============================================================================

pub mod version;
pub mod ir_contract;
pub mod bootstrap;
pub mod reproducibility;
pub mod plugin;
pub mod regression;

