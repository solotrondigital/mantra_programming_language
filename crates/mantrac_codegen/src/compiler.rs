use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct NativeCompiler;

impl NativeCompiler {
    pub fn compile_c_to_binary(c_code: &str, output_path: &Path, optimize: bool) -> Result<PathBuf, String> {
        let temp_dir = std::env::temp_dir();
        let temp_c_file = temp_dir.join(format!("mantrac_build_{}.c", std::process::id()));

        fs::write(&temp_c_file, c_code)
            .map_err(|e| format!("Failed to write temporary C file: {}", e))?;

        // Determine compiler binary: try clang, gcc, cc
        let compilers = if cfg!(windows) {
            vec!["clang", "gcc", "cl", "cc"]
        } else {
            vec!["clang", "gcc", "cc"]
        };

        let mut compiler_found = None;
        for c in compilers {
            if Command::new(c).arg("--version").output().is_ok() {
                compiler_found = Some(c);
                break;
            }
        }

        let compiler = compiler_found.ok_or_else(|| {
            "No native C compiler (clang, gcc, cc) was found in PATH to produce native executables.".to_string()
        })?;

        let mut cmd = Command::new(compiler);
        cmd.arg(&temp_c_file);
        cmd.arg("-o").arg(output_path);

        if optimize {
            cmd.arg("-O2");
        }
        cmd.arg("-std=c99");

        if !cfg!(windows) {
            cmd.arg("-lm");
        }

        let output = cmd.output().map_err(|e| format!("Failed to execute compiler {}: {}", compiler, e))?;

        // Clean up temp C file
        let _ = fs::remove_file(&temp_c_file);

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Native compilation failed:\n{}", stderr));
        }

        Ok(output_path.to_path_buf())
    }
}

