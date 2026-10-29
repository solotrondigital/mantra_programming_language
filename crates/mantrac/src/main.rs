use mantra_debugger::DebugSession;
use mantra_bootstrap::version::{CompilerVersion, BootstrapStage};
use mantra_bootstrap::regression::RegressionSuite;
use mantra_bootstrap::bootstrap::BootstrapPipeline;
use mantrac_checker::TypeChecker;
use mantrac_codegen::{CEmitter, NativeCompiler};
use mantrac_ir::{AstLowering, JitEngine, PipelineProfiler, WasmEmitter};
use mantrac_lexer::Lexer;
use mantrac_parser::Parser;
use mantrac_vm::Interpreter;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::Instant;

fn print_help() {
    println!(
        r#"Mantra Compiler (mantrac) v0.3.0 (MVP 3 — Self-Hosting Compiler)
Usage:
    mantrac <input.mantra> [options]

Execution Modes:
    --jit               Execute using high-performance JIT engine with Interpreter fallback
    --interpret         Execute using the v0.1.0 tree-walking interpreter
    --run               Run program immediately (JIT with automated fallback)
    --debug             Launch interactive Mantra debugger (mantra-debugger)

Compilation & Target Options:
    -o <path>           Specify the output executable or file path
    --emit-c            Emit generated C intermediate source code
    --emit wasm         Emit WebAssembly binary module (.wasm)
    --emit wat          Emit WebAssembly Text format (.wat)
    --target <triple>   Cross-compile target (e.g. wasm32-wasi, native)
    --release           Build with native optimizations enabled (-O2)
    --check             Parse and type-check the source without generating code

Bootstrap & Verification (MVP 3):
    --bootstrap         Run self-hosting bootstrap pipeline (stages 0→1→2 + verify)
    --regression        Run the canonical language regression suite
    --version-metadata  Print compiler version metadata in artifact-header format

Diagnostics & Profiling:
    --jit-diagnostics   Display detailed JIT compilation and execution metrics
    --profile           Display pipeline stage duration breakdown

Informational:
    -v, --version       Print mantrac version
    -h, --help          Display this help message
"#
    );
}

fn print_version() {
    let v = CompilerVersion::current(BootstrapStage::RustBootstrap);
    println!("{}", v.to_version_string());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        exit(1);
    }

    let mut input_file: Option<String> = None;
    let mut output_file: Option<String> = None;
    let mut run_direct = false;
    let mut use_jit = false;
    let mut force_interpret = false;
    let mut debug_mode = false;
    let mut check_only = false;
    let mut emit_c = false;
    let mut emit_wasm = false;
    let mut emit_wat = false;
    let mut target_wasm = false;
    let mut release = false;
    let mut show_jit_diag = false;
    let mut show_profile = false;
    let mut run_bootstrap = false;
    let mut run_regression = false;
    let mut show_version_meta = false;

    let mut profiler = PipelineProfiler::default();

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                return;
            }
            "-v" | "--version" => {
                print_version();
                return;
            }
            "--run" => {
                run_direct = true;
            }
            "--jit" => {
                run_direct = true;
                use_jit = true;
            }
            "--interpret" => {
                run_direct = true;
                force_interpret = true;
            }
            "--debug" => {
                debug_mode = true;
            }
            "--check" => {
                check_only = true;
            }
            "--emit-c" => {
                emit_c = true;
            }
            "--emit" => {
                i += 1;
                if i < args.len() {
                    match args[i].as_str() {
                        "wasm" => emit_wasm = true,
                        "wat" => emit_wat = true,
                        "c" => emit_c = true,
                        other => {
                            eprintln!("Error: Unknown emit target '{}'. Supported: wasm, wat, c", other);
                            exit(1);
                        }
                    }
                }
            }
            "--target" => {
                i += 1;
                if i < args.len() {
                    if args[i] == "wasm32-wasi" || args[i] == "wasm32" {
                        target_wasm = true;
                        emit_wasm = true;
                    }
                }
            }
            "--release" => {
                release = true;
            }
            "--jit-diagnostics" => {
                show_jit_diag = true;
            }
            "--profile" => {
                show_profile = true;
            }
            "--bootstrap" => {
                run_bootstrap = true;
            }
            "--regression" => {
                run_regression = true;
            }
            "--version-metadata" => {
                show_version_meta = true;
            }
            "--mantra-define-from-file" | "--dart-define-from-file" => {
                i += 1;
                if i < args.len() {
                    load_env_defines(&args[i]);
                }
            }
            opt if opt.starts_with("--mantra-define-from-file=") => {
                let file = opt.trim_start_matches("--mantra-define-from-file=");
                load_env_defines(file);
            }
            opt if opt.starts_with("--dart-define-from-file=") => {
                let file = opt.trim_start_matches("--dart-define-from-file=");
                load_env_defines(file);
            }
            "-o" => {
                i += 1;
                if i < args.len() {
                    output_file = Some(args[i].clone());
                } else {
                    eprintln!("Error: -o requires an output path");
                    exit(1);
                }
            }
            _ => {
                if input_file.is_none() && !arg.starts_with('-') {
                    input_file = Some(arg.clone());
                } else {
                    eprintln!("Unknown option: {}", arg);
                    exit(1);
                }
            }
        }
        i += 1;
    }

    // ── Bootstrap pipeline (MVP 3) ──────────────────────────────────────────
    if show_version_meta {
        let v = CompilerVersion::current(BootstrapStage::RustBootstrap);
        println!("{}", v.to_artifact_header());
        return;
    }

    if run_regression {
        println!("Running Mantra v0.3.0 Regression Suite...\n");
        let suite = RegressionSuite::canonical();
        let results = suite.run();
        print!("{}", RegressionSuite::summary(&results));
        if !RegressionSuite::all_passed(&results) {
            exit(1);
        }
        return;
    }

    if run_bootstrap {
        println!("Running Mantra v0.3.0 Bootstrap Pipeline...\n");
        let workspace = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let pipeline = BootstrapPipeline::new(&workspace);
        let report = pipeline.run_all();
        print!("{}", report.to_report_string());
        let report_path = workspace.join("bootstrap_report.txt");
        let _ = report.save_to_file(&report_path);
        println!("Report saved to: {}", report_path.display());
        if !report.all_passed() {
            exit(1);
        }
        return;
    }

    let input_path = match input_file {
        Some(path) => path,
        None => {
            eprintln!("Error: No input .mantra file provided");
            print_help();
            exit(1);
        }
    };

    let source = match fs::read_to_string(&input_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading '{}': {}", input_path, e);
            exit(1);
        }
    };

    // 1. Lexing
    let t_lex = Instant::now();
    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer error [{}]: {} at {}:{}", e.code, e.message, e.span.line, e.span.col);
            exit(1);
        }
    };
    profiler.lexing = t_lex.elapsed();

    // 2. Parsing
    let t_parse = Instant::now();
    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };
    profiler.parsing = t_parse.elapsed();

    // 3. Type Checking & Semantic Analysis
    let t_type = Instant::now();
    let mut checker = TypeChecker::new();
    if let Err(diagnostics) = checker.check_program(&program) {
        let mut has_error = false;
        for diag in diagnostics {
            eprintln!("{}", diag.format_with_source(&input_path, &source));
            if diag.level == mantrac_checker::DiagnosticLevel::Error {
                has_error = true;
            }
        }
        if has_error {
            exit(1);
        }
    }
    profiler.typechecking = t_type.elapsed();

    if check_only {
        println!("Checked '{}': 0 errors found.", input_path);
        if show_profile {
            profiler.print_report();
        }
        return;
    }

    // 4. Debug Mode (mantra-debugger)
    if debug_mode {
        let mut session = DebugSession::new(input_path.clone(), &source);
        if let Err(e) = session.start_repl(&program) {
            eprintln!("Debugger error: {}", e);
            exit(1);
        }
        return;
    }

    // 5. WebAssembly Target Output
    if emit_wasm || emit_wat || target_wasm {
        let t_ir = Instant::now();
        let mut lowering = AstLowering::new();
        let ir_prog = lowering.lower_program(&program);
        profiler.ir_lowering = t_ir.elapsed();

        let emitter = WasmEmitter::new();
        if emit_wat {
            let wat_code = emitter.emit_wat(&ir_prog);
            if let Some(ref out) = output_file {
                if let Err(e) = fs::write(out, &wat_code) {
                    eprintln!("Failed to write WAT output to '{}': {}", out, e);
                    exit(1);
                }
                println!("Emitted WebAssembly Text (.wat) to '{}'", out);
            } else {
                println!("{}", wat_code);
            }
            return;
        }

        let wasm_bytes = emitter.emit_wasm_binary(&ir_prog);
        let out_path = if let Some(ref o) = output_file {
            PathBuf::from(o)
        } else {
            let p = Path::new(&input_path);
            let stem = p.file_stem().unwrap_or_default().to_string_lossy();
            PathBuf::from(format!("{}.wasm", stem))
        };

        if let Err(e) = fs::write(&out_path, &wasm_bytes) {
            eprintln!("Failed to write WebAssembly output to '{}': {}", out_path.display(), e);
            exit(1);
        }
        println!("Compiled WebAssembly module: {} ({} bytes)", out_path.display(), wasm_bytes.len());
        if show_profile {
            profiler.print_report();
        }
        return;
    }

    // 6. Direct Execution (JIT or Interpreter)
    if run_direct {
        let t_exec = Instant::now();
        if force_interpret && !use_jit {
            let mut interpreter = Interpreter::new();
            if let Err(e) = interpreter.execute_program(&program) {
                eprintln!("Runtime error: {} at {}:{}", e.message, e.span.line, e.span.col);
                exit(1);
            }
        } else {
            // Default to JIT with automated fallback
            let t_ir = Instant::now();
            let mut lowering = AstLowering::new();
            let ir_prog = lowering.lower_program(&program);
            profiler.ir_lowering = t_ir.elapsed();

            let mut jit = JitEngine::new();
            match jit.execute(&ir_prog, &program) {
                Ok(code) => {
                    profiler.execution_or_codegen = t_exec.elapsed();
                    if show_jit_diag {
                        let diag = jit.diagnostics();
                        println!("\n--- JIT Diagnostics ---");
                        println!("Functions compiled    : {}", diag.functions_compiled);
                        println!("Instructions executed : {}", diag.instructions_executed);
                        println!("Compile latency       : {} µs", diag.compile_duration_us);
                        println!("Execution time        : {} µs", diag.execution_duration_us);
                        println!("Fallback triggered    : {}", diag.used_fallback);
                        if let Some(ref reason) = diag.fallback_reason {
                            println!("Fallback reason       : {}", reason);
                        }
                        println!("-----------------------");
                    }
                    if show_profile {
                        profiler.print_report();
                    }
                    if code != 0 {
                        exit(code as i32);
                    }
                    return;
                }
                Err(e) => {
                    eprintln!("Execution error: {}", e);
                    exit(1);
                }
            }
        }
        profiler.execution_or_codegen = t_exec.elapsed();
        if show_profile {
            profiler.print_report();
        }
        return;
    }

    // 7. C Code Generation & Native Compilation
    let t_codegen = Instant::now();
    let mut emitter = CEmitter::new(&checker.symbols);
    let c_code = emitter.emit(&program);

    if emit_c {
        if let Some(ref out) = output_file {
            if let Err(e) = fs::write(out, &c_code) {
                eprintln!("Failed to write C output to '{}': {}", out, e);
                exit(1);
            }
            println!("Emitted C code to '{}'", out);
        } else {
            println!("{}", c_code);
        }
        return;
    }

    // Determine output executable name
    let out_path = if let Some(ref o) = output_file {
        PathBuf::from(o)
    } else {
        let p = Path::new(&input_path);
        let stem = p.file_stem().unwrap_or_default().to_string_lossy();
        if cfg!(windows) {
            PathBuf::from(format!("{}.exe", stem))
        } else {
            PathBuf::from(stem.to_string())
        }
    };

    match NativeCompiler::compile_c_to_binary(&c_code, &out_path, release) {
        Ok(path) => {
            profiler.execution_or_codegen = t_codegen.elapsed();
            println!("Compiled native binary: {}", path.display());
            if show_profile {
                profiler.print_report();
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}

fn load_env_defines(file_path_str: &str) {
    let path = Path::new(file_path_str);
    if !path.exists() {
        eprintln!("Warning: Environment file '{}' does not exist.", file_path_str);
        return;
    }

    if let Ok(content) = fs::read_to_string(path) {
        let mut count = 0;
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            if let Some((raw_key, raw_val)) = trimmed.split_once('=') {
                let key = raw_key.trim();
                let mut val = raw_val.trim();
                if (val.starts_with('"') && val.ends_with('"')) || (val.starts_with('\'') && val.ends_with('\'')) {
                    if val.len() >= 2 {
                        val = &val[1..val.len() - 1];
                    }
                }
                std::env::set_var(key, val);
                count += 1;
            }
        }
        println!("Loaded defines from '{}' ({} variables)", file_path_str, count);
    }
}

