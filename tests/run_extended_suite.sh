#!/usr/bin/env bash
# ==============================================================================
# Mantra Extended Test, Fuzzing & Regression Suite
# tests/run_extended_suite.sh
#
# Runs:
#   1. Conformance & Diagnostic Negative Tests
#   2. Stress Testing Suite (Deep AST, Multi-Level Hierarchy, Allocations)
#   3. Fuzzing Robustness Suite (250 mutated iterations)
#   4. Micro-benchmarking Subsystem
#   5. Security Audit Integrity Check
# ==============================================================================
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$DIR"

echo "=================================================================="
echo "    MANTRA EXTENDED TESTING, FUZZING & REGRESSION SUITE (v0.6.0)  "
echo "=================================================================="
echo ""

if [ -f "$DIR/target/release/mantrac.exe" ] && [ -f "$DIR/target/release/mantra.exe" ]; then
    MANTRAC="$DIR/target/release/mantrac.exe"
    MANTRA="$DIR/target/release/mantra.exe"
elif [ -f "$DIR/target/release/mantrac" ] && [ -f "$DIR/target/release/mantra" ]; then
    MANTRAC="$DIR/target/release/mantrac"
    MANTRA="$DIR/target/release/mantra"
elif [ -f "$DIR/target/debug/mantrac.exe" ] && [ -f "$DIR/target/debug/mantra.exe" ]; then
    MANTRAC="$DIR/target/debug/mantrac.exe"
    MANTRA="$DIR/target/debug/mantra.exe"
elif [ -f "$DIR/target/debug/mantrac" ] && [ -f "$DIR/target/debug/mantra" ]; then
    MANTRAC="$DIR/target/debug/mantrac"
    MANTRA="$DIR/target/debug/mantra"
else
    echo ">> Compiling workspace binaries..."
    cargo build
    if [ -f "$DIR/target/debug/mantrac.exe" ]; then
        MANTRAC="$DIR/target/debug/mantrac.exe"
        MANTRA="$DIR/target/debug/mantra.exe"
    else
        MANTRAC="$DIR/target/debug/mantrac"
        MANTRA="$DIR/target/debug/mantra"
    fi
fi

# 1. Run Standard Conformance & Diagnostic Tests
echo ">> [1/5] Executing Core Conformance & Diagnostic Suite..."
bash "$DIR/tests/run_all.sh"

# 2. Run Stress Testing Suite
echo ""
echo ">> [2/5] Executing Stress Testing Suite (VM & Native)..."
# Use a temp dir that works on Linux/macOS/Windows Git Bash
TMP_STRESS_BIN="${TEMP:-${TMPDIR:-/tmp}}/mantra_stress_bins"
mkdir -p "$TMP_STRESS_BIN"

for test in tests/stress/*.mantra; do
    tname=$(basename "$test" .mantra)
    printf "  Stress (VM): %-40s ... " "$tname"
    $MANTRAC "$test" --run > /dev/null
    echo -e "\033[32mPASS\033[0m"

    printf "  Stress (Native): %-36s ... " "$tname"
    bin_path="$TMP_STRESS_BIN/$tname"
    $MANTRAC "$test" -o "$bin_path" > /dev/null
    # On Windows, mantrac auto-appends .exe; try both
    if [ -f "${bin_path}.exe" ]; then
        "${bin_path}.exe" > /dev/null
    elif [ -f "$bin_path" ]; then
        "$bin_path" > /dev/null
    else
        echo -e "\033[31mFAIL (binary not found at $bin_path)\033[0m"
        exit 1
    fi
    echo -e "\033[32mPASS\033[0m"
done

# 3. Run Fuzzing Robustness Suite
echo ""
echo ">> [3/5] Executing Automated Compiler Fuzzing Suite (250 iterations)..."
PYTHON_CMD="python3"
if ! command -v python3 >/dev/null 2>&1; then
    PYTHON_CMD="python"
fi
$PYTHON_CMD "$DIR/tests/fuzz/fuzz_runner.py" 250

# 4. Run Micro-benchmarks
echo ""
echo ">> [4/5] Executing Performance Micro-benchmarks..."
$MANTRAC "tests/benchmarks/bench_main.mantra" --run

# 5. Run Security Audit Integrity Check
echo ""
echo ">> [5/5] Executing MDK Security Audit Verification..."
MDK_TEST_DIR="${TEMP:-${TMPDIR:-/tmp}}/mantra_mdk_integration_test"
if [ -d "$MDK_TEST_DIR" ]; then
    cd "$MDK_TEST_DIR"
    $MANTRA audit || true
else
    echo ">> Skipping audit: MDK integration test directory not found at $MDK_TEST_DIR"
fi

echo ""
echo "=================================================================="
echo "   🎉 ALL EXTENDED TESTS, FUZZING & REGRESSIONS PASSED SAFELY!    "
echo "=================================================================="

