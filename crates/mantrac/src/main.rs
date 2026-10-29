use mantrac_checker::TypeChecker;
use mantrac_codegen::{CEmitter, NativeCompiler};
use mantrac_lexer::Lexer;
use mantrac_parser::Parser;
use mantrac_vm::Interpreter;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

fn print_help() {
    println!(
        r#"Mantra Compiler (mantrac) v0.1.0-draft
Usage:
    mantrac <input.mantra> [options]

Options:
    -o <path>       Specify the output executable or file path
    --run           Execute the program directly using the integrated VM
    --check         Parse and type-check the source without generating code
    --emit-c        Emit generated C intermediate source code
    --release       Build with native optimizations enabled (-O2)
    -v, --version   Print mantrac version
    -h, --help      Display this help message
"#
    );
}

fn print_version() {
    println!("mantrac 0.1.0-draft (Mantra native bootstrap compiler, Rust backend)");
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
    let mut check_only = false;
    let mut emit_c = false;
    let mut release = false;

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
            "--check" => {
                check_only = true;
            }
            "--emit-c" => {
                emit_c = true;
            }
            "--release" => {
                release = true;
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
    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer error [{}]: {} at {}:{}", e.code, e.message, e.span.line, e.span.col);
            exit(1);
        }
    };

    // 2. Parsing
    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    // 3. Type Checking & OOP Semantic Analysis
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

    if check_only {
        println!("Checked '{}': 0 errors found.", input_path);
        return;
    }

    // 4. Execution or Compilation
    if run_direct {
        let mut interpreter = Interpreter::new();
        match interpreter.execute_program(&program) {
            Ok(_) => return,
            Err(e) => {
                eprintln!("Runtime error: {} at {}:{}", e.message, e.span.line, e.span.col);
                exit(1);
            }
        }
    }

    // 5. Code generation
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
            println!("Compiled native binary: {}", path.display());
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}

