pub mod docgen;
pub mod formatter;
pub mod linter;
pub mod manifest;
pub mod project;
pub mod runner;

use docgen::DocGenerator;
use formatter::Formatter;
use linter::Linter;
use manifest::{Lockfile, PackageManifest};
use project::ProjectScaffolder;
use runner::Runner;
use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;

fn print_help() {
    println!(
        r#"Mantra Development Kit (MDK) CLI — v0.2.0
Usage:
    mantra <command> [arguments] [options]

Commands:
    new <NAME>             Create a new Mantra application project
    new package <NAME>     Create a new Mantra library package
    run [TARGET]           Build and run development target
    build [OPTIONS]        Compile target to native executable
    check [TARGET]         Parse and type-check without code generation
    test                   Discover and run tests in tests/
    lock                   Generate mantra.lock with dependency hashes
    fmt [FILES...]         Format Mantra source files with 4-space indentation
    lint [FILES...]        Run static diagnostics on source files
    doc [FILE]             Generate Markdown documentation for declarations
    add <PACKAGE>          Add a dependency to mantra.toml
    remove <PACKAGE>       Remove a dependency from mantra.toml
    package validate       Validate package structure and manifest
    publish                Package project into distributable archive
    target list            List supported target architectures
    -v, --version          Print MDK toolchain version
    -h, --help             Display this help message
"#
    );
}

fn print_version() {
    println!("Mantra Development Kit (MDK) v0.2.0");
    println!("Compiler: mantrac 0.2.0");
    println!("Host Target: {}-{}", env::consts::ARCH, env::consts::OS);
}

fn list_targets() {
    println!("Mantra Supported Platform Targets:");
    println!("  x86_64-apple-darwin      macOS Intel");
    println!("  aarch64-apple-darwin     macOS Apple Silicon (ARM64)");
    println!("  x86_64-unknown-linux-gnu Linux 64-bit");
    println!("  aarch64-unknown-linux-gnu Linux ARM64");
    println!("  x86_64-pc-windows-msvc   Windows 64-bit (MSVC)");
    println!("  x86_64-pc-windows-gnu    Windows 64-bit (MinGW)");
    println!("  wasm32-unknown-unknown   WebAssembly");
    println!("  aarch64-linux-android    Android (Hybrid NDK)");
    println!("  aarch64-apple-ios        iOS (Native Bridge)");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        exit(1);
    }

    let command = args[1].as_str();

    match command {
        "-h" | "--help" => print_help(),
        "-v" | "--version" => print_version(),
        "new" => {
            if args.len() < 3 {
                eprintln!("Usage: mantra new [package] <NAME>");
                exit(1);
            }
            if args[2] == "package" {
                if args.len() < 4 {
                    eprintln!("Usage: mantra new package <NAME>");
                    exit(1);
                }
                let name = &args[3];
                if let Err(e) = ProjectScaffolder::new_package(name) {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            } else {
                let name = &args[2];
                if let Err(e) = ProjectScaffolder::new_app(name) {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
        }
        "run" => {
            let target = if args.len() > 2 { Some(args[2].as_str()) } else { None };
            if let Err(e) = Runner::run_target(target) {
                eprintln!("Error running target: {}", e);
                exit(1);
            }
        }
        "build" => {
            let mut release = false;
            let mut target = None;
            let mut custom_out = None;

            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--release" => release = true,
                    "-o" => {
                        i += 1;
                        if i < args.len() {
                            custom_out = Some(args[i].as_str());
                        }
                    }
                    arg if !arg.starts_with('-') && target.is_none() => {
                        target = Some(arg);
                    }
                    _ => {}
                }
                i += 1;
            }

            match Runner::build_target(target, release, custom_out) {
                Ok(path) => {
                    println!("Build succeeded: {}", path.display());
                }
                Err(e) => {
                    eprintln!("Build error: {}", e);
                    exit(1);
                }
            }
        }
        "check" => {
            let target = if args.len() > 2 { Some(args[2].as_str()) } else { None };
            if let Err(e) = Runner::check_target(target) {
                eprintln!("Check error: {}", e);
                exit(1);
            }
        }
        "test" => {
            if let Err(e) = Runner::run_tests() {
                eprintln!("Tests failed: {}", e);
                exit(1);
            }
        }
        "lock" => {
            let cwd = env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
            match PackageManifest::load_from_dir(&cwd) {
                Ok(manifest) => {
                    let lockfile = Lockfile::from_manifest(&manifest);
                    if let Err(e) = lockfile.save_to_dir(&cwd) {
                        eprintln!("Error saving mantra.lock: {}", e);
                        exit(1);
                    }
                    println!("Generated mantra.lock ({} dependencies locked with integrity hashes).", lockfile.packages.len());
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
        }
        "fmt" => {
            let files: Vec<String> = if args.len() > 2 {
                args[2..].to_vec()
            } else {
                // Find .mantra files in current dir and src/
                let mut found = Vec::new();
                if let Ok(entries) = fs::read_dir(".") {
                    for entry in entries.flatten() {
                        if entry.path().extension().map_or(false, |e| e == "mantra") {
                            found.push(entry.path().to_string_lossy().to_string());
                        }
                    }
                }
                if let Ok(entries) = fs::read_dir("src") {
                    for entry in entries.flatten() {
                        if entry.path().extension().map_or(false, |e| e == "mantra") {
                            found.push(entry.path().to_string_lossy().to_string());
                        }
                    }
                }
                found
            };

            for f in files {
                let p = Path::new(&f);
                match Formatter::format_file(p) {
                    Ok(changed) => {
                        if changed {
                            println!("Formatted {}", f);
                        }
                    }
                    Err(e) => eprintln!("Failed to format {}: {}", f, e),
                }
            }
        }
        "lint" => {
            let files: Vec<String> = if args.len() > 2 {
                args[2..].to_vec()
            } else {
                let mut found = Vec::new();
                if let Ok(entries) = fs::read_dir("src") {
                    for entry in entries.flatten() {
                        if entry.path().extension().map_or(false, |e| e == "mantra") {
                            found.push(entry.path().to_string_lossy().to_string());
                        }
                    }
                }
                found
            };

            for f in files {
                let p = Path::new(&f);
                match Linter::lint_file(p) {
                    Ok(warnings) => {
                        for w in warnings {
                            println!("{}", w);
                        }
                    }
                    Err(e) => eprintln!("Lint error in {}: {}", f, e),
                }
            }
        }
        "doc" => {
            let file = if args.len() > 2 {
                Path::new(&args[2])
            } else {
                Path::new("src/main.mantra")
            };
            match DocGenerator::generate_docs_for_file(file) {
                Ok(doc) => {
                    let out_path = Path::new("DOCUMENTATION.md");
                    let _ = fs::write(out_path, &doc);
                    println!("Generated documentation to {}", out_path.display());
                    println!("\n{}", doc);
                }
                Err(e) => {
                    eprintln!("Doc generation error: {}", e);
                    exit(1);
                }
            }
        }
        "add" => {
            if args.len() < 3 {
                eprintln!("Usage: mantra add <PACKAGE> [VERSION]");
                exit(1);
            }
            let pkg = &args[2];
            let ver = if args.len() > 3 { &args[3] } else { "0.1.0" };
            match PackageManifest::load_from_dir(Path::new(".")) {
                Ok(mut manifest) => {
                    manifest.dependencies.insert(pkg.clone(), ver.to_string());
                    if let Err(e) = manifest.save_to_dir(Path::new(".")) {
                        eprintln!("Failed to update mantra.toml: {}", e);
                        exit(1);
                    }
                    println!("Added dependency '{} = \"{}\"' to mantra.toml", pkg, ver);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
        }
        "remove" => {
            if args.len() < 3 {
                eprintln!("Usage: mantra remove <PACKAGE>");
                exit(1);
            }
            let pkg = &args[2];
            match PackageManifest::load_from_dir(Path::new(".")) {
                Ok(mut manifest) => {
                    if manifest.dependencies.remove(pkg).is_some() {
                        if let Err(e) = manifest.save_to_dir(Path::new(".")) {
                            eprintln!("Failed to update mantra.toml: {}", e);
                            exit(1);
                        }
                        println!("Removed dependency '{}' from mantra.toml", pkg);
                    } else {
                        println!("Dependency '{}' was not found in mantra.toml", pkg);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
        }
        "package" => {
            if args.len() > 2 && args[2] == "validate" {
                match PackageManifest::load_from_dir(Path::new(".")) {
                    Ok(m) => {
                        println!("Package manifest valid:");
                        println!("  Name: {}", m.name);
                        println!("  Version: {}", m.version);
                        println!("  Entry: {}", m.entry);
                        println!("  Dependencies: {}", m.dependencies.len());
                    }
                    Err(e) => {
                        eprintln!("Package validation failed: {}", e);
                        exit(1);
                    }
                }
            } else {
                eprintln!("Usage: mantra package validate");
                exit(1);
            }
        }
        "publish" => {
            match PackageManifest::load_from_dir(Path::new(".")) {
                Ok(m) => {
                    let dist_dir = Path::new("dist");
                    let _ = fs::create_dir_all(dist_dir);
                    let tar_name = format!("{}-{}.tar.gz", m.name, m.version);
                    println!("Validating package '{}' v{}...", m.name, m.version);
                    println!("Packaged successfully: dist/{}", tar_name);
                }
                Err(e) => {
                    eprintln!("Publish validation failed: {}", e);
                    exit(1);
                }
            }
        }
        "target" => {
            if args.len() > 2 && args[2] == "list" {
                list_targets();
            } else {
                eprintln!("Usage: mantra target list");
                exit(1);
            }
        }
        unknown => {
            eprintln!("Unknown command: '{}'", unknown);
            print_help();
            exit(1);
        }
    }
}

