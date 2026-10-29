use crate::manifest::PackageManifest;
use mantrac_checker::TypeChecker;
use mantrac_codegen::{CEmitter, NativeCompiler};
use mantrac_lexer::Lexer;
use mantrac_parser::Parser;
use mantrac_vm::Interpreter;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Runner;

impl Runner {
    pub fn run_target(target_path: Option<&str>) -> Result<(), String> {
        let file_path = Self::resolve_entry_file(target_path)?;
        let source = fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read {}: {}", file_path.display(), e))?;

        let mut lexer = Lexer::new(&source);
        let tokens = lexer.tokenize().map_err(|e| e.message)?;
        let mut parser = Parser::new(tokens);
        let program = parser.parse().map_err(|e| e.message)?;

        let mut checker = TypeChecker::new();
        if let Err(diags) = checker.check_program(&program) {
            for d in diags {
                eprintln!("{}", d.format_with_source(&file_path.to_string_lossy(), &source));
            }
            return Err("Type checking failed".to_string());
        }

        let mut interpreter = Interpreter::new();
        interpreter.execute_program(&program).map_err(|e| e.message)?;

        Ok(())
    }

    pub fn build_target(target_path: Option<&str>, release: bool, custom_output: Option<&str>) -> Result<PathBuf, String> {
        let file_path = Self::resolve_entry_file(target_path)?;
        let source = fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read {}: {}", file_path.display(), e))?;

        let mut lexer = Lexer::new(&source);
        let tokens = lexer.tokenize().map_err(|e| e.message)?;
        let mut parser = Parser::new(tokens);
        let program = parser.parse().map_err(|e| e.message)?;

        let mut checker = TypeChecker::new();
        if let Err(diags) = checker.check_program(&program) {
            for d in diags {
                eprintln!("{}", d.format_with_source(&file_path.to_string_lossy(), &source));
            }
            return Err("Type checking failed".to_string());
        }

        let mut emitter = CEmitter::new(&checker.symbols);
        let c_code = emitter.emit(&program);

        let out_dir = PathBuf::from("target").join(if release { "release" } else { "debug" });
        fs::create_dir_all(&out_dir).map_err(|e| format!("Failed to create target dir: {}", e))?;

        let binary_name = if let Some(out) = custom_output {
            PathBuf::from(out)
        } else {
            let stem = if let Ok(m) = PackageManifest::load_from_dir(Path::new(".")) {
                m.name
            } else {
                file_path.file_stem().unwrap_or_default().to_string_lossy().to_string()
            };
            let bin = if cfg!(windows) { format!("{}.exe", stem) } else { stem };
            out_dir.join(bin)
        };

        NativeCompiler::compile_c_to_binary(&c_code, &binary_name, release)?;
        Ok(binary_name)
    }

    pub fn build_web(target_path: Option<&str>, _release: bool, custom_output: Option<&str>) -> Result<PathBuf, String> {
        let file_path = Self::resolve_entry_file(target_path)?;
        let source = fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read {}: {}", file_path.display(), e))?;

        let mut lexer = Lexer::new(&source);
        let tokens = lexer.tokenize().map_err(|e| e.message)?;
        let mut parser = Parser::new(tokens);
        let program = parser.parse().map_err(|e| e.message)?;

        let mut checker = TypeChecker::new();
        if let Err(diags) = checker.check_program(&program) {
            for d in diags {
                eprintln!("{}", d.format_with_source(&file_path.to_string_lossy(), &source));
            }
            return Err("Type checking failed".to_string());
        }

        println!("Compiling WebAssembly target (wasm32-unknown-unknown)...");

        let mut lowering = mantrac_ir::AstLowering::new();
        let ir_prog = lowering.lower_program(&program);

        let emitter = mantrac_ir::WasmEmitter::new();
        let wasm_bytes = emitter.emit_wasm_binary(&ir_prog);

        let web_dir = PathBuf::from("web");
        if !web_dir.exists() {
            let _ = fs::create_dir_all(&web_dir);
        }

        let out_path = if let Some(out) = custom_output {
            PathBuf::from(out)
        } else if web_dir.exists() {
            web_dir.join("app.wasm")
        } else {
            PathBuf::from("target/web/app.wasm")
        };

        if let Some(parent) = out_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        fs::write(&out_path, &wasm_bytes)
            .map_err(|e| format!("Failed to write WebAssembly output to {}: {}", out_path.display(), e))?;

        let wat_code = emitter.emit_wat(&ir_prog);
        let wat_path = out_path.with_extension("wat");
        let _ = fs::write(&wat_path, wat_code);

        println!("✓ Emitted WebAssembly binary : {} ({} bytes)", out_path.display(), wasm_bytes.len());
        println!("✓ Emitted WebAssembly text   : {}", wat_path.display());
        if web_dir.join("index.html").exists() {
            println!("✓ Web application shell      : web/index.html");
        }
        println!("✓ Web target ready to serve  : python3 -m http.server -d web");

        Ok(out_path)
    }

    pub fn check_target(target_path: Option<&str>) -> Result<(), String> {
        let file_path = Self::resolve_entry_file(target_path)?;
        let source = fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read {}: {}", file_path.display(), e))?;

        let mut lexer = Lexer::new(&source);
        let tokens = lexer.tokenize().map_err(|e| e.message)?;
        let mut parser = Parser::new(tokens);
        let program = parser.parse().map_err(|e| e.message)?;

        let mut checker = TypeChecker::new();
        if let Err(diags) = checker.check_program(&program) {
            for d in diags {
                eprintln!("{}", d.format_with_source(&file_path.to_string_lossy(), &source));
            }
            return Err("Type checking errors found".to_string());
        }

        println!("Checked '{}': 0 errors found.", file_path.display());
        Ok(())
    }

    pub fn run_tests() -> Result<(), String> {
        let candidate_dirs = [Path::new("tests"), Path::new("test")];
        let mut test_files = Vec::new();

        for dir in &candidate_dirs {
            if dir.exists() {
                if let Ok(entries) = fs::read_dir(dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.extension().map_or(false, |ext| ext == "mantra") {
                            test_files.push(path);
                        }
                    }
                }
            }
        }

        if test_files.is_empty() {
            println!("No test files found in tests/ or test/");
            return Ok(());
        }

        println!("Running {} test file(s)...", test_files.len());
        let mut passed = 0;
        let mut failed = 0;

        for test_file in test_files {
            print!("  test {} ... ", test_file.display());
            match Self::run_target(Some(test_file.to_str().unwrap())) {
                Ok(_) => {
                    println!("\x1b[32mok\x1b[0m");
                    passed += 1;
                }
                Err(e) => {
                    println!("\x1b[31mFAILED\x1b[0m ({})", e);
                    failed += 1;
                }
            }
        }

        println!("\nTest results: {} passed; {} failed", passed, failed);
        if failed > 0 {
            Err(format!("{} test(s) failed", failed))
        } else {
            Ok(())
        }
    }

    fn resolve_entry_file(target_path: Option<&str>) -> Result<PathBuf, String> {
        if let Some(path_str) = target_path {
            let p = PathBuf::from(path_str);
            if p.exists() {
                return Ok(p);
            }
            return Err(format!("File '{}' does not exist", path_str));
        }

        // Try manifest entry
        if let Ok(manifest) = PackageManifest::load_from_dir(Path::new(".")) {
            let entry_p = PathBuf::from(manifest.entry);
            if entry_p.exists() {
                return Ok(entry_p);
            }
        }

        // Defaults (Flutter-style lib/main.mantra first, then src/main.mantra, etc.)
        if Path::new("lib/main.mantra").exists() {
            return Ok(PathBuf::from("lib/main.mantra"));
        }
        if Path::new("src/main.mantra").exists() {
            return Ok(PathBuf::from("src/main.mantra"));
        }
        if Path::new("src/lib.mantra").exists() {
            return Ok(PathBuf::from("src/lib.mantra"));
        }
        if Path::new("main.mantra").exists() {
            return Ok(PathBuf::from("main.mantra"));
        }

        Err("No entry point found. Provide a file path or ensure lib/main.mantra or src/main.mantra exists.".to_string())
    }
}

