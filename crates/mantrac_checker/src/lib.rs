pub mod checker;
pub mod diagnostics;
pub mod scope;
pub mod symbols;

pub use checker::TypeChecker;
pub use diagnostics::{Diagnostic, DiagnosticLevel};
pub use symbols::*;

