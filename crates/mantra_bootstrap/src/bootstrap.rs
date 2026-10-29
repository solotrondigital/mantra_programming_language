// ==============================================================================
// mantra_bootstrap::bootstrap — Bootstrap Pipeline Controller
// ==============================================================================
//
// Manages the three-stage bootstrap execution pipeline and records the result
// of each stage for inclusion in the bootstrap_report.txt artifact.

use std::time::{Instant, Duration};
use std::path::{Path, PathBuf};
use crate::version::{CompilerVersion, BootstrapStage};
use crate::reproducibility::ReproducibilityChecker;

/// Result of a single bootstrap stage execution.
#[derive(Debug, Clone)]
pub struct StageResult {
    pub stage: BootstrapStage,
    pub success: bool,
    pub duration: Duration,
    pub output_path: PathBuf,
    pub error: Option<String>,
}

impl StageResult {
    pub fn to_report_line(&self) -> String {
        let status = if self.success { "PASS" } else { "FAIL" };
        format!(
            "  Stage {:?}: {} ({:.2?}) → {}",
            self.stage, status, self.duration,
            self.output_path.display()
        )
    }
}

/// BootstrapPipeline orchestrates stages 0–2 of the self-hosting compiler build.
pub struct BootstrapPipeline {
    pub workspace_root: PathBuf,
    pub mantrac_binary: PathBuf,
    pub frontend_source: PathBuf,
    pub stage1_output: PathBuf,
    pub stage2_output: PathBuf,
}

impl BootstrapPipeline {
    pub fn new(workspace_root: &Path) -> Self {
        let root = workspace_root.to_path_buf();
        BootstrapPipeline {
            mantrac_binary:   root.join("target/release/mantrac"),
            frontend_source:  root.join("compiler_src/frontend.mantra"),
            stage1_output:    std::env::temp_dir().join("mantra_bootstrap_stage1.tokens"),
            stage2_output:    std::env::temp_dir().join("mantra_bootstrap_stage2.tokens"),
            workspace_root:   root,
        }
    }

    /// Run a single stage using the stage-0 Rust bootstrap compiler.
    pub fn run_stage(&self, stage: u32) -> StageResult {
        let t = Instant::now();
        let bootstrap_stage = match stage {
            1 => BootstrapStage::MantraStage1,
            2 => BootstrapStage::MantraStage2Verify,
            _ => BootstrapStage::RustBootstrap,
        };

        if !self.mantrac_binary.exists() {
            return StageResult {
                stage: bootstrap_stage,
                success: false,
                duration: t.elapsed(),
                output_path: self.stage1_output.clone(),
                error: Some(format!("Bootstrap compiler not found: {}", self.mantrac_binary.display())),
            };
        }

        let out_path = if stage == 1 { &self.stage1_output } else { &self.stage2_output };

        let status = std::process::Command::new(&self.mantrac_binary)
            .arg(&self.frontend_source)
            .arg("--interpret")
            .env("MANTRA_SOURCE", &self.frontend_source)
            .env("MANTRA_OUT1", &self.stage1_output)
            .env("MANTRA_OUT2", &self.stage2_output)
            .env("MANTRA_STAGE", stage.to_string())
            .status();

        let success = matches!(status, Ok(s) if s.success());
        let error = if success {
            None
        } else {
            Some(format!("Stage {} exited with error: {:?}", stage, status))
        };

        StageResult {
            stage: bootstrap_stage,
            success,
            duration: t.elapsed(),
            output_path: out_path.clone(),
            error,
        }
    }

    /// Execute all three stages and return a full bootstrap report.
    pub fn run_all(&self) -> BootstrapReport {
        let mut report = BootstrapReport::new(
            CompilerVersion::current(BootstrapStage::RustBootstrap)
        );

        // Stage 1
        let s1 = self.run_stage(1);
        let s1_ok = s1.success;
        report.stages.push(s1);

        // Stage 2 (only if stage 1 passed)
        if s1_ok {
            let s2 = self.run_stage(2);
            let s2_ok = s2.success;
            report.stages.push(s2);

            // Reproducibility check (only if stage 2 passed)
            if s2_ok {
                let checker = ReproducibilityChecker::new();
                report.reproducible = Some(
                    checker.compare_files(&self.stage1_output, &self.stage2_output)
                );
            }
        }

        report.finished_at = std::time::SystemTime::now();
        report
    }
}

/// BootstrapReport aggregates stage results and reproducibility outcome.
pub struct BootstrapReport {
    pub compiler_version: CompilerVersion,
    pub stages: Vec<StageResult>,
    pub reproducible: Option<bool>,
    pub started_at: std::time::SystemTime,
    pub finished_at: std::time::SystemTime,
}

impl BootstrapReport {
    pub fn new(version: CompilerVersion) -> Self {
        let now = std::time::SystemTime::now();
        BootstrapReport {
            compiler_version: version,
            stages: Vec::new(),
            reproducible: None,
            started_at: now,
            finished_at: now,
        }
    }

    pub fn all_passed(&self) -> bool {
        let stages_ok = self.stages.iter().all(|s| s.success);
        let repro_ok  = self.reproducible.unwrap_or(false);
        stages_ok && repro_ok
    }

    pub fn to_report_string(&self) -> String {
        let mut out = String::new();
        out.push_str("Mantra v0.3.0 Bootstrap Verification Report\n");
        out.push_str(&format!("Compiler: {}\n", self.compiler_version.to_version_string()));
        out.push_str(&format!("Stages:   {}\n\n", self.stages.len()));

        for s in &self.stages {
            out.push_str(&s.to_report_line());
            out.push('\n');
            if let Some(ref e) = s.error {
                out.push_str(&format!("    Error: {}\n", e));
            }
        }

        out.push('\n');
        match self.reproducible {
            Some(true)  => out.push_str("Reproducibility: PASS\n"),
            Some(false) => out.push_str("Reproducibility: FAIL\n"),
            None        => out.push_str("Reproducibility: NOT CHECKED\n"),
        }

        out.push('\n');
        let verdict = if self.all_passed() { "PASS" } else { "FAIL" };
        out.push_str(&format!("OVERALL: {}\n", verdict));
        out
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), String> {
        std::fs::write(path, self.to_report_string())
            .map_err(|e| format!("Failed to write bootstrap report: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_report_fails() {
        let v = CompilerVersion::current(BootstrapStage::RustBootstrap);
        let report = BootstrapReport::new(v);
        assert!(!report.all_passed(), "Empty report should not pass");
    }
}

