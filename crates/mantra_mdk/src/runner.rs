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
        let (program, source) = Self::load_and_merge_project(&file_path)?;

        let mut checker = TypeChecker::new();
        if let Err(diags) = checker.check_program(&program) {
            for d in diags {
                eprintln!("{}", d.format_with_source(&file_path.to_string_lossy(), &source));
            }
            return Err("Type checking failed".to_string());
        }

        let mut interpreter = Interpreter::new();
        interpreter.execute_program(&program).map_err(|e| e.message)?;

        // If the project is configured as a backend service (REST/GraphQL) or web frontend,
        // start the real HTTP development server and listen for incoming requests.
        if let Some((host, port, service_kind)) = crate::http_server::detect_service_and_port(target_path) {
            crate::http_server::serve_http(&host, port, &service_kind)?;
        }

        Ok(())
    }

    pub fn build_target(target_path: Option<&str>, release: bool, custom_output: Option<&str>) -> Result<PathBuf, String> {
        let file_path = Self::resolve_entry_file(target_path)?;
        let (program, source) = Self::load_and_merge_project(&file_path)?;

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

    pub fn build_web(
        target_path: Option<&str>,
        release: bool,
        custom_output: Option<&str>,
        defines_file: Option<&str>,
    ) -> Result<PathBuf, String> {
        if let Some(df) = defines_file {
            let vars = crate::env_loader::EnvLoader::load_file(Path::new(df))?;
            crate::env_loader::EnvLoader::print_summary(df, &vars);
        }

        let file_path = Self::resolve_entry_file(target_path)?;
        let (program, source) = Self::load_and_merge_project(&file_path)?;

        let mut checker = TypeChecker::new();
        if let Err(diags) = checker.check_program(&program) {
            for d in diags {
                eprintln!("{}", d.format_with_source(&file_path.to_string_lossy(), &source));
            }
            return Err("Type checking failed".to_string());
        }

        println!("Compiling WebAssembly target (wasm32-unknown-unknown)...");
        if release {
            println!("Build Profile   : release (-O2 equivalent)");
        }

        let mut lowering = mantrac_ir::AstLowering::new();
        let ir_prog = lowering.lower_program(&program);

        let emitter = mantrac_ir::WasmEmitter::new();
        let wasm_bytes = emitter.emit_wasm_binary(&ir_prog);

        let web_dir = if Path::new("platforms/web").exists() {
            PathBuf::from("platforms/web")
        } else {
            PathBuf::from("web")
        };
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

    /// Development web server supporting `-d chrome` or `-d web` on `--web-port`
    pub fn run_web_dev(web_port: u16, defines_file: Option<&str>) -> Result<(), String> {
        if let Some(df) = defines_file {
            let vars = crate::env_loader::EnvLoader::load_file(Path::new(df))?;
            crate::env_loader::EnvLoader::print_summary(df, &vars);
        }

        println!("==================================================");
        println!("   Mantra Web Platform — Development Server       ");
        println!("==================================================");
        println!("Target Platform : Chrome / Web (WASM)");
        println!("Local Server    : http://127.0.0.1:{}", web_port);
        println!("Asset Directory : ./web");

        // Compile WASM bundle first
        Self::build_web(None, false, None, None)?;

        println!("\n>> Starting local HTTP development server on 127.0.0.1:{}...", web_port);
        println!(">> Open http://127.0.0.1:{} in Chrome to view the application.", web_port);
        println!(">> Ready for development. Serving static assets and WASM.");

        println!("✓ Development server active on http://127.0.0.1:{}", web_port);
        crate::http_server::serve_http("127.0.0.1", web_port, "web")?;
        Ok(())
    }

    pub fn check_target(target_path: Option<&str>) -> Result<(), String> {
        let file_path = Self::resolve_entry_file(target_path)?;
        let (program, source) = Self::load_and_merge_project(&file_path)?;

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

    fn load_and_merge_project(entry_file: &Path) -> Result<(mantrac_ast::Program, String), String> {
        let entry_source = fs::read_to_string(entry_file)
            .map_err(|e| format!("Failed to read {}: {}", entry_file.display(), e))?;

        let parent_dir = entry_file.parent().unwrap_or_else(|| Path::new("."));
        let root_dir = if parent_dir.ends_with("src") || parent_dir.ends_with("lib") {
            parent_dir.parent().unwrap_or_else(|| Path::new("."))
        } else {
            Path::new(".")
        };

        let mut files_to_read = Vec::new();

        // Recursively find all .mantra files in the project
        fn visit_dir(dir: &Path, files: &mut Vec<PathBuf>) {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if name.starts_with('.') || name == "target" || name == "dist" || name == "build" {
                        continue;
                    }
                    if path.is_dir() {
                        visit_dir(&path, files);
                    } else if path.extension().and_then(|e| e.to_str()) == Some("mantra") {
                        files.push(path);
                    }
                }
            }
        }

        let search_dirs = [root_dir.join("src"), root_dir.join("lib")];
        for d in &search_dirs {
            if d.exists() && d.is_dir() {
                visit_dir(d, &mut files_to_read);
            }
        }

        // Sort to ensure deterministic merge order
        files_to_read.sort();

        // Deduplicate and ensure entry_file is processed
        let canonical_entry = entry_file.canonicalize().unwrap_or_else(|_| entry_file.to_path_buf());

        let mut merged_declarations = Vec::new();
        let mut seen_canonical = std::collections::HashSet::new();

        // Process other files first so their declarations are available before entry point
        for file in &files_to_read {
            let canon = file.canonicalize().unwrap_or_else(|_| file.to_path_buf());
            if canon == canonical_entry {
                continue;
            }
            if seen_canonical.insert(canon) {
                if let Ok(src) = fs::read_to_string(file) {
                    let mut lexer = Lexer::new(&src);
                    match lexer.tokenize() {
                        Ok(tokens) => {
                            let mut parser = Parser::new(tokens);
                            match parser.parse() {
                                Ok(prog) => merged_declarations.extend(prog.declarations),
                                Err(e) => return Err(format!("Parse error in {}: {}", file.display(), e.message)),
                            }
                        }
                        Err(e) => return Err(format!("Lexer error in {}: {}", file.display(), e.message)),
                    }
                }
            }
        }

        // Now parse the entry file
        let mut entry_lexer = Lexer::new(&entry_source);
        let entry_tokens = entry_lexer.tokenize().map_err(|e| e.message)?;
        let mut entry_parser = Parser::new(entry_tokens);
        let entry_prog = entry_parser.parse().map_err(|e| e.message)?;

        merged_declarations.extend(entry_prog.declarations);

        Ok((mantrac_ast::Program { declarations: merged_declarations }, entry_source))
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

        // Defaults (lib/main.mantra first, then src/main.mantra, etc.)
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

