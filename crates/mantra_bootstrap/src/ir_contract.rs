// ==============================================================================
// mantra_bootstrap::ir_contract — Stable Typed IR Interface Contract
// ==============================================================================
//
// Defines the *stable, versioned* interface that every Mantra backend must
// satisfy. This contract acts as the single source of truth that connects:
//
//   AST (mantrac_ast)
//     ↓  IrContract::lower()
//   IrProgram (mantrac_ir)
//     ↓  IrContract::execute_jit()      — JIT engine
//     ↓  IrContract::execute_interpret() — tree-walking interpreter fallback
//     ↓  IrContract::emit_wasm()        — WebAssembly emitter
//     ↓  IrContract::emit_c()           — C99 backend
//
// Adding a new IR instruction requires bumping IR_SCHEMA_VERSION in version.rs
// and updating this contract so all backends can signal whether they support
// the new instruction or must fall back.

use mantrac_ast::Program;
use mantrac_ir::{AstLowering, IrProgram, JitEngine, WasmEmitter};
use mantrac_vm::Interpreter;
use mantrac_codegen::{CEmitter, NativeCompiler};
use mantrac_checker::TypeChecker;
use std::path::PathBuf;

/// The result type returned by all contract operations.
pub type ContractResult<T> = Result<T, ContractError>;

#[derive(Debug, Clone)]
pub struct ContractError {
    pub phase: ContractPhase,
    pub message: String,
    pub fallback_available: bool,
}

impl std::fmt::Display for ContractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.phase, self.message)
    }
}

/// The compilation/execution phase where a contract error occurred.
#[derive(Debug, Clone, PartialEq)]
pub enum ContractPhase {
    TypeCheck,
    IrLower,
    JitCompile,
    JitExecute,
    Interpret,
    WasmEmit,
    CCodegen,
    NativeLink,
}

impl std::fmt::Display for ContractPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::TypeCheck  => "type-check",
            Self::IrLower    => "ir-lower",
            Self::JitCompile => "jit-compile",
            Self::JitExecute => "jit-execute",
            Self::Interpret  => "interpret",
            Self::WasmEmit   => "wasm-emit",
            Self::CCodegen   => "c-codegen",
            Self::NativeLink => "native-link",
        };
        write!(f, "{}", s)
    }
}

/// BackendCapabilities declares what a particular backend supports, so that
/// unsupported constructs fail with clear diagnostics rather than silently
/// producing incorrect output (v0.2.0 acceptance criterion 7, now stable).
#[derive(Debug, Clone)]
pub struct BackendCapabilities {
    pub name: &'static str,
    pub supports_jit: bool,
    pub supports_wasm: bool,
    pub supports_native_c: bool,
    pub supports_async: bool,
    pub supports_dynamic_dispatch: bool,
    pub supports_generics: bool,
    pub ir_schema: &'static str,
}

impl BackendCapabilities {
    pub fn jit_engine() -> Self {
        BackendCapabilities {
            name: "JIT Engine (mantrac_ir)",
            supports_jit: true,
            supports_wasm: false,
            supports_native_c: false,
            supports_async: true,
            supports_dynamic_dispatch: false, // falls back to interpreter
            supports_generics: true,
            ir_schema: crate::version::IR_SCHEMA_VERSION,
        }
    }

    pub fn interpreter() -> Self {
        BackendCapabilities {
            name: "Tree-Walking Interpreter (mantrac_vm)",
            supports_jit: false,
            supports_wasm: false,
            supports_native_c: false,
            supports_async: true,
            supports_dynamic_dispatch: true,
            supports_generics: true,
            ir_schema: crate::version::IR_SCHEMA_VERSION,
        }
    }

    pub fn wasm_emitter() -> Self {
        BackendCapabilities {
            name: "WebAssembly Emitter (mantrac_ir::WasmEmitter)",
            supports_jit: false,
            supports_wasm: true,
            supports_native_c: false,
            supports_async: false,   // WASI async is future work
            supports_dynamic_dispatch: false,
            supports_generics: false, // monomorphised before lowering
            ir_schema: crate::version::IR_SCHEMA_VERSION,
        }
    }

    pub fn c99_backend() -> Self {
        BackendCapabilities {
            name: "C99 Codegen (mantrac_codegen)",
            supports_jit: false,
            supports_wasm: false,
            supports_native_c: true,
            supports_async: false,
            supports_dynamic_dispatch: true,
            supports_generics: true,
            ir_schema: crate::version::IR_SCHEMA_VERSION,
        }
    }
}

/// IrContract — the stable interface binding AST → IR → backends.
/// All backends consume an `IrProgram` produced by `IrContract::lower`.
pub struct IrContract;

impl IrContract {
    /// Lower a type-checked AST to the shared typed IR.
    pub fn lower(program: &Program) -> ContractResult<IrProgram> {
        let mut lowering = AstLowering::new();
        Ok(lowering.lower_program(program))
    }

    /// Execute IR via JIT engine. Falls back to interpreter on unsupported
    /// constructs; the fallback is documented and always produces correct output.
    pub fn execute_jit(ir: &IrProgram, ast: &Program) -> ContractResult<i64> {
        let mut jit = JitEngine::new();
        jit.execute(ir, ast).map_err(|e| ContractError {
            phase: ContractPhase::JitExecute,
            message: e,
            fallback_available: true,
        })
    }

    /// Execute via tree-walking interpreter (always supports full language).
    pub fn execute_interpret(program: &Program) -> ContractResult<()> {
        let mut interp = Interpreter::new();
        interp.execute_program(program).map(|_| ()).map_err(|e| ContractError {
            phase: ContractPhase::Interpret,
            message: format!("{:?}", e),
            fallback_available: false,
        })
    }

    /// Emit a WebAssembly text module (WAT).
    pub fn emit_wat(ir: &IrProgram) -> ContractResult<String> {
        let emitter = WasmEmitter::new();
        Ok(emitter.emit_wat(ir))
    }

    /// Emit a WebAssembly binary module (.wasm bytes).
    pub fn emit_wasm_binary(ir: &IrProgram) -> ContractResult<Vec<u8>> {
        let emitter = WasmEmitter::new();
        Ok(emitter.emit_wasm_binary(ir))
    }

    /// Emit C99 source and compile to a native binary.
    pub fn emit_native(
        program: &Program,
        checker: &mantrac_checker::TypeChecker,
        output: &std::path::Path,
        release: bool,
    ) -> ContractResult<PathBuf> {
        let mut emitter = CEmitter::new(&checker.symbols);
        let c_code = emitter.emit(program);
        NativeCompiler::compile_c_to_binary(&c_code, output, release)
            .map_err(|e| ContractError {
                phase: ContractPhase::NativeLink,
                message: e,
                fallback_available: false,
            })
    }

    /// Run the full type-check + IR-lower pipeline in one call, returning
    /// a typed-and-lowered IR program. Used by bootstrap verification.
    pub fn typecheck_and_lower(source: &str) -> ContractResult<(Program, IrProgram)> {
        let mut lexer = mantrac_lexer::Lexer::new(source);
        let tokens = lexer.tokenize().map_err(|e| ContractError {
            phase: ContractPhase::IrLower,
            message: format!("Lex error: {}", e.message),
            fallback_available: false,
        })?;

        let mut parser = mantrac_parser::Parser::new(tokens);
        let program = parser.parse().map_err(|e| ContractError {
            phase: ContractPhase::IrLower,
            message: format!("Parse error: {}", e.message),
            fallback_available: false,
        })?;

        let mut checker = TypeChecker::new();
        checker.check_program(&program).map_err(|diags| ContractError {
            phase: ContractPhase::TypeCheck,
            message: diags
                .iter()
                .map(|d| format!("[{}] {}", d.code, d.message))
                .collect::<Vec<_>>()
                .join("; "),
            fallback_available: false,
        })?;

        let ir = Self::lower(&program)?;
        Ok((program, ir))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backend_capabilities_names_are_set() {
        assert!(!BackendCapabilities::jit_engine().name.is_empty());
        assert!(!BackendCapabilities::interpreter().name.is_empty());
        assert!(!BackendCapabilities::wasm_emitter().name.is_empty());
        assert!(!BackendCapabilities::c99_backend().name.is_empty());
    }

    #[test]
    fn wasm_backend_does_not_claim_dynamic_dispatch() {
        let wasm = BackendCapabilities::wasm_emitter();
        assert!(!wasm.supports_dynamic_dispatch,
            "WASM emitter must not claim dynamic dispatch support");
    }

    #[test]
    fn jit_does_not_claim_native_c() {
        let jit = BackendCapabilities::jit_engine();
        assert!(!jit.supports_native_c,
            "JIT engine must not claim C99 backend support");
    }

    #[test]
    fn ir_contract_hello_world() {
        let src = "def main() -> Int:\n    print(\"hello\")\n    return 0\n";
        let result = IrContract::typecheck_and_lower(src);
        assert!(result.is_ok(), "hello world should lower without error: {:?}", result.err());
    }
}

