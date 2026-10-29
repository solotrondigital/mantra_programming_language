// ==============================================================================
// mantra_mdk::bench — Micro-benchmarking Subsystem
// ==============================================================================

use crate::runner::Runner;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

pub struct BenchmarkResult {
    pub name: String,
    pub iterations: u32,
    pub total_duration_us: u128,
    pub avg_latency_ns: f64,
    pub ops_per_sec: f64,
}

impl BenchmarkResult {
    pub fn to_table_row(&self) -> String {
        format!(
            "{:<24} {:>10} {:>14.2} ns/op {:>14.2} ops/sec",
            self.name, self.iterations, self.avg_latency_ns, self.ops_per_sec
        )
    }
}

pub struct BenchRunner;

impl BenchRunner {
    pub fn run_benchmarks(
        target: Option<&str>,
        iterations: u32,
        warmup: u32,
    ) -> Result<Vec<BenchmarkResult>, String> {
        let bench_files = Self::discover_benchmarks(target)?;
        if bench_files.is_empty() {
            return Err("No benchmark targets found. Create files in benches/ (e.g. benches/bench_main.mantra)".to_string());
        }

        println!("Running Mantra Benchmarks (iterations={}, warmup={})\n", iterations, warmup);
        println!("{:<24} {:>10} {:>20} {:>22}", "BENCHMARK", "ITERATIONS", "TIME/OP", "THROUGHPUT");
        println!("{:-<80}", "");

        let mut results = Vec::new();

        for file in &bench_files {
            let stem = file.file_stem().unwrap_or_default().to_string_lossy().to_string();

            // Warmup iterations
            for _ in 0..warmup {
                let _ = Runner::run_target(Some(&file.to_string_lossy()));
            }

            // Timed execution
            let start = Instant::now();
            let mut successful_iters = 0;
            for _ in 0..iterations {
                if Runner::run_target(Some(&file.to_string_lossy())).is_ok() {
                    successful_iters += 1;
                }
            }
            let elapsed_us = start.elapsed().as_micros();

            if successful_iters == 0 {
                return Err(format!("Benchmark '{}' failed during execution", stem));
            }

            let avg_ns = (elapsed_us as f64 * 1000.0) / successful_iters as f64;
            let ops_sec = if avg_ns > 0.0 {
                1_000_000_000.0 / avg_ns
            } else {
                0.0
            };

            let res = BenchmarkResult {
                name: stem,
                iterations: successful_iters,
                total_duration_us: elapsed_us,
                avg_latency_ns: avg_ns,
                ops_per_sec: ops_sec,
            };

            println!("{}", res.to_table_row());
            results.push(res);
        }

        println!("{:-<80}", "");
        println!("Benchmark suite completed: {} benchmark(s) executed successfully.\n", results.len());

        Ok(results)
    }

    fn discover_benchmarks(target: Option<&str>) -> Result<Vec<PathBuf>, String> {
        if let Some(t) = target {
            let p = PathBuf::from(t);
            if p.exists() {
                return Ok(vec![p]);
            }
            return Err(format!("Specified benchmark target '{}' not found", t));
        }

        let mut discovered = Vec::new();

        // 1. Check benches/ directory
        let bench_dir = Path::new("benches");
        if bench_dir.exists() && bench_dir.is_dir() {
            if let Ok(entries) = fs::read_dir(bench_dir) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    if p.extension().map_or(false, |ext| ext == "mantra") {
                        discovered.push(p);
                    }
                }
            }
        }

        // 2. Fallback to src/main.mantra if no benches/ directory
        if discovered.is_empty() {
            let main_path = PathBuf::from("src/main.mantra");
            if main_path.exists() {
                discovered.push(main_path);
            }
        }

        discovered.sort();
        Ok(discovered)
    }
}

