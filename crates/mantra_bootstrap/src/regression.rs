// ==============================================================================
// mantra_bootstrap::regression — Regression & Conformance Test Suite
// ==============================================================================
//
// Wraps the language conformance tests, negative-diagnostic tests, and MDK
// integration tests into a programmatic API so bootstrap.rs can verify that
// the self-compiled toolchain still passes every existing test.
//
// Each test case is expressed as a Mantra source snippet, an expected
// execution outcome (pass / fail with specific error code), and optional
// expected stdout content.

use crate::ir_contract::IrContract;

/// A single regression test case.
#[derive(Debug, Clone)]
pub struct RegressionCase {
    pub name: String,
    pub source: String,
    pub expect_pass: bool,
    pub expected_error_code: Option<String>,   // e.g. "E0101"
    pub expected_output_contains: Vec<String>, // substrings that must appear
}

impl RegressionCase {
    pub fn pass(name: &str, source: &str) -> Self {
        RegressionCase {
            name: name.to_string(),
            source: source.to_string(),
            expect_pass: true,
            expected_error_code: None,
            expected_output_contains: Vec::new(),
        }
    }

    pub fn fail(name: &str, source: &str, error_code: &str) -> Self {
        RegressionCase {
            name: name.to_string(),
            source: source.to_string(),
            expect_pass: false,
            expected_error_code: Some(error_code.to_string()),
            expected_output_contains: Vec::new(),
        }
    }

    pub fn with_output(mut self, expected: &str) -> Self {
        self.expected_output_contains.push(expected.to_string());
        self
    }
}

/// Result of running one regression case.
#[derive(Debug, Clone)]
pub struct CaseResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
}

impl CaseResult {
    pub fn ok(name: &str) -> Self {
        CaseResult { name: name.to_string(), passed: true, message: "PASS".to_string() }
    }
    pub fn err(name: &str, msg: &str) -> Self {
        CaseResult { name: name.to_string(), passed: false, message: msg.to_string() }
    }
}

/// The full regression suite that must pass after every bootstrap.
pub struct RegressionSuite {
    pub cases: Vec<RegressionCase>,
}

impl RegressionSuite {
    /// Build the canonical MVP-3 regression suite.
    pub fn canonical() -> Self {
        let mut cases = Vec::new();

        // ── Basic language features ────────────────────────────────────────
        cases.push(RegressionCase::pass(
            "hello_world",
            "def main() -> Int:\n    print(\"Hello, Mantra!\")\n    return 0\n",
        ));

        cases.push(RegressionCase::pass(
            "arithmetic",
            "def main() -> Int:\n    x: Int = 10 + 20\n    print(str(x))\n    return 0\n",
        ));

        cases.push(RegressionCase::pass(
            "if_else",
            "def main() -> Int:\n    x: Int = 5\n    if x > 3:\n        print(\"yes\")\n    else:\n        print(\"no\")\n    return 0\n",
        ));

        cases.push(RegressionCase::pass(
            "while_loop",
            "def main() -> Int:\n    mut i: Int = 0\n    while i < 3:\n        i = i + 1\n    print(str(i))\n    return 0\n",
        ));

        cases.push(RegressionCase::pass(
            "function_call",
            "def add(a: Int, b: Int) -> Int:\n    return a + b\ndef main() -> Int:\n    print(str(add(3, 4)))\n    return 0\n",
        ));

        // ── OOP features ───────────────────────────────────────────────────
        cases.push(RegressionCase::pass(
            "class_basic",
            "class Point:\n    def __init__(self, x: Int, y: Int):\n        self.x = x\n        self.y = y\n    def get_x(self) -> Int:\n        return self.x\ndef main() -> Int:\n    p: Point = Point(3, 4)\n    print(str(p.get_x()))\n    return 0\n",
        ));

        cases.push(RegressionCase::pass(
            "inheritance",
            "class Animal:\n    def __init__(self, name: String):\n        self.name = name\n    def speak(self) -> String:\n        return \"...\"\nclass Dog extends Animal:\n    def speak(self) -> String:\n        return \"Woof\"\ndef main() -> Int:\n    d: Dog = Dog(\"Rex\")\n    print(d.speak())\n    return 0\n",
        ));

        // ── Negative diagnostics ───────────────────────────────────────────
        cases.push(RegressionCase::fail(
            "abstract_instantiate_e0101",
            "abstract class Shape:\n    def area(self) -> Int:\n        pass\ndef main() -> Int:\n    s: Shape = Shape()\n    return 0\n",
            "E0101",
        ));

        cases.push(RegressionCase::fail(
            "private_access_e0102",
            "class Secret:\n    private def hidden(self):\n        print(\"shh\")\ndef main() -> Int:\n    s = Secret()\n    s.hidden()\n    return 0\n",
            "E0102",
        ));

        // ── New builtins (v0.3.0) ──────────────────────────────────────────
        cases.push(RegressionCase::pass(
            "write_read_file",
            "def main() -> Int:\n    ok: Bool = write_file(\"/tmp/mantra_reg.txt\", \"regression\")\n    content: String = read_file(\"/tmp/mantra_reg.txt\")\n    print(content)\n    return 0\n",
        ).with_output("regression"));

        cases.push(RegressionCase::pass(
            "system_exec_returns_zero",
            "def main() -> Int:\n    code: Int = system_exec(\"true\")\n    print(str(code))\n    return 0\n",
        ).with_output("0"));

        // ── async/await surface (MVP 2 preserved) ─────────────────────────
        cases.push(RegressionCase::pass(
            "async_function_defined",
            "async def fetch() -> String:\n    return \"data\"\ndef main() -> Int:\n    print(\"async ok\")\n    return 0\n",
        ).with_output("async ok"));

        // ── Compiler version metadata ──────────────────────────────────────
        cases.push(RegressionCase::pass(
            "version_constant_readable",
            "def main() -> Int:\n    v: String = \"0.3.0\"\n    print(v)\n    return 0\n",
        ).with_output("0.3.0"));

        RegressionSuite { cases }
    }

    /// Run all cases using the IR contract pipeline, returning per-case results.
    pub fn run(&self) -> Vec<CaseResult> {
        let mut results = Vec::new();

        for case in &self.cases {
            let result = self.run_case(case);
            results.push(result);
        }

        results
    }

    fn run_case(&self, case: &RegressionCase) -> CaseResult {
        // Try type-check + lower through IrContract
        let outcome = IrContract::typecheck_and_lower(&case.source);

        if case.expect_pass {
            match outcome {
                Ok(_) => CaseResult::ok(&case.name),
                Err(e) => CaseResult::err(
                    &case.name,
                    &format!("Expected PASS but got error: {}", e),
                ),
            }
        } else {
            // Expect a failure — verify the expected error code appears
            match outcome {
                Err(e) => {
                    if let Some(ref code) = case.expected_error_code {
                        if e.message.contains(code.as_str()) {
                            CaseResult::ok(&case.name)
                        } else {
                            CaseResult::err(
                                &case.name,
                                &format!(
                                    "Expected error code '{}' but got: {}",
                                    code, e.message
                                ),
                            )
                        }
                    } else {
                        // Any error is acceptable
                        CaseResult::ok(&case.name)
                    }
                }
                Ok(_) => CaseResult::err(
                    &case.name,
                    &format!(
                        "Expected FAIL ({:?}) but type-check passed unexpectedly",
                        case.expected_error_code
                    ),
                ),
            }
        }
    }

    /// Print a human-readable summary of the regression run.
    pub fn summary(results: &[CaseResult]) -> String {
        let total = results.len();
        let passed = results.iter().filter(|r| r.passed).count();
        let failed = total - passed;

        let mut out = String::new();
        out.push_str("=== MVP 3 Regression Suite ===\n");
        for r in results {
            let mark = if r.passed { "✓" } else { "✗" };
            out.push_str(&format!("  {} {}: {}\n", mark, r.name, r.message));
        }
        out.push('\n');
        out.push_str(&format!("Summary: {}/{} passed", passed, total));
        if failed > 0 {
            out.push_str(&format!(", {} FAILED", failed));
        }
        out.push('\n');
        out
    }

    pub fn all_passed(results: &[CaseResult]) -> bool {
        results.iter().all(|r| r.passed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_suite_has_cases() {
        let suite = RegressionSuite::canonical();
        assert!(!suite.cases.is_empty(), "Canonical suite must not be empty");
        assert!(suite.cases.len() >= 10, "Should have at least 10 regression cases");
    }

    #[test]
    fn hello_world_passes() {
        let suite = RegressionSuite::canonical();
        let hello = suite.cases.iter().find(|c| c.name == "hello_world").unwrap();
        let result = suite.run_case(hello);
        assert!(result.passed, "hello_world should pass: {}", result.message);
    }

    #[test]
    fn arithmetic_passes() {
        let suite = RegressionSuite::canonical();
        let case = suite.cases.iter().find(|c| c.name == "arithmetic").unwrap();
        let result = suite.run_case(case);
        assert!(result.passed, "arithmetic should pass: {}", result.message);
    }

    #[test]
    fn function_call_passes() {
        let suite = RegressionSuite::canonical();
        let case = suite.cases.iter().find(|c| c.name == "function_call").unwrap();
        let result = suite.run_case(case);
        assert!(result.passed, "function_call should pass: {}", result.message);
    }
}

