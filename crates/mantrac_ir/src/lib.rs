pub mod diagnostics;
pub mod ir;
pub mod jit;
pub mod lower;
pub mod wasm;

pub use diagnostics::PipelineProfiler;
pub use ir::{IrFunction, IrInstruction, IrProgram, IrType, IrValue};
pub use jit::{JitDiagnostics, JitEngine};
pub use lower::AstLowering;
pub use wasm::WasmEmitter;
