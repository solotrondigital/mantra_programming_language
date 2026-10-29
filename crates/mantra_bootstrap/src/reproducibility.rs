// ==============================================================================
// mantra_bootstrap::reproducibility — Output Comparison & Verification
// ==============================================================================
//
// Compares the token streams / AST files produced in stage-1 and stage-2.
// Acceptance criterion:
//   "repeated builds produce equivalent outputs or documented reproducibility
//    exceptions."
//
// When outputs differ, the checker reports the *first* line where they
// diverge so engineers can pinpoint the non-determinism.

use std::path::Path;

/// A single line-level difference between stage-1 and stage-2 outputs.
#[derive(Debug, Clone)]
pub struct DiffEntry {
    pub line_number: usize,
    pub stage1_line: String,
    pub stage2_line: String,
}

/// The outcome of comparing two pipeline outputs.
#[derive(Debug, Clone)]
pub struct ReproducibilityResult {
    pub identical: bool,
    pub stage1_lines: usize,
    pub stage2_lines: usize,
    pub differences: Vec<DiffEntry>,
    pub exception_documented: bool,
    pub exception_reason: Option<String>,
}

impl ReproducibilityResult {
    pub fn summary(&self) -> String {
        if self.identical {
            format!(
                "PASS: Stage-1 and Stage-2 outputs are bit-identical ({} lines).",
                self.stage1_lines
            )
        } else {
            format!(
                "FAIL: {} difference(s) found. Stage-1: {} lines, Stage-2: {} lines.",
                self.differences.len(),
                self.stage1_lines,
                self.stage2_lines,
            )
        }
    }
}

/// ReproducibilityChecker compares stage outputs, stripping documented
/// exception patterns (e.g., embedded timestamps) before comparison.
pub struct ReproducibilityChecker {
    /// Lines matching these prefixes are excluded from the comparison because
    /// they are *documented reproducibility exceptions* (e.g., build timestamps,
    /// host paths embedded by stage-0).
    pub exception_prefixes: Vec<&'static str>,
}

impl ReproducibilityChecker {
    pub fn new() -> Self {
        ReproducibilityChecker {
            // Documented exceptions:
            //   "; mantrac-self … stage=N …" — stage number intentionally differs.
            exception_prefixes: vec!["; mantrac-self"],
        }
    }

    fn is_exception_line(&self, line: &str) -> bool {
        self.exception_prefixes.iter().any(|p| line.starts_with(p))
    }

    fn filter_lines<'a>(&self, text: &'a str) -> Vec<&'a str> {
        text.lines()
            .filter(|l| !self.is_exception_line(l.trim()))
            .collect()
    }

    /// Compare two raw output strings.
    pub fn compare_strings(&self, stage1: &str, stage2: &str) -> ReproducibilityResult {
        let s1_lines = self.filter_lines(stage1);
        let s2_lines = self.filter_lines(stage2);

        let mut diffs = Vec::new();
        let max = s1_lines.len().max(s2_lines.len());

        for i in 0..max {
            let l1 = s1_lines.get(i).copied().unwrap_or("<missing>");
            let l2 = s2_lines.get(i).copied().unwrap_or("<missing>");
            if l1 != l2 {
                diffs.push(DiffEntry {
                    line_number: i + 1,
                    stage1_line: l1.to_string(),
                    stage2_line: l2.to_string(),
                });
                if diffs.len() >= 20 {
                    break; // Cap at 20 diff entries in report
                }
            }
        }

        ReproducibilityResult {
            identical: diffs.is_empty() && s1_lines.len() == s2_lines.len(),
            stage1_lines: s1_lines.len(),
            stage2_lines: s2_lines.len(),
            differences: diffs,
            exception_documented: true,
            exception_reason: Some(
                "Stage header lines ('; mantrac-self … stage=N …') are excluded \
                 from comparison because the stage number legitimately differs."
                .to_string()
            ),
        }
    }

    /// Compare two output files by path.
    pub fn compare_files(&self, stage1_path: &Path, stage2_path: &Path) -> bool {
        let s1 = std::fs::read_to_string(stage1_path).unwrap_or_default();
        let s2 = std::fs::read_to_string(stage2_path).unwrap_or_default();
        let result = self.compare_strings(&s1, &s2);
        if !result.identical {
            eprintln!("{}", result.summary());
            for diff in &result.differences {
                eprintln!(
                    "  Line {}: stage-1='{}' stage-2='{}'",
                    diff.line_number, diff.stage1_line, diff.stage2_line
                );
            }
        }
        result.identical
    }
}

impl Default for ReproducibilityChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_outputs_pass() {
        let checker = ReproducibilityChecker::new();
        let s = "KEYWORD:def\nIDENT:main\nSYM:(\nSYM:)\nEOF";
        let r = checker.compare_strings(s, s);
        assert!(r.identical, "Identical inputs should produce identical result");
    }

    #[test]
    fn stage_header_lines_are_excluded() {
        let checker = ReproducibilityChecker::new();
        let s1 = "; mantrac-self 0.3.0 stage=1\nKEYWORD:def\nEOF";
        let s2 = "; mantrac-self 0.3.0 stage=2\nKEYWORD:def\nEOF";
        let r = checker.compare_strings(s1, s2);
        assert!(r.identical, "Stage header differences should be excluded: {:?}", r.differences);
    }

    #[test]
    fn different_tokens_detected() {
        let checker = ReproducibilityChecker::new();
        let s1 = "KEYWORD:def\nIDENT:foo\nEOF";
        let s2 = "KEYWORD:def\nIDENT:bar\nEOF";
        let r = checker.compare_strings(s1, s2);
        assert!(!r.identical, "Different tokens should be detected");
        assert_eq!(r.differences.len(), 1);
        assert_eq!(r.differences[0].line_number, 2);
    }
}

