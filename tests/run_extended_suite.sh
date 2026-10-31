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

if [ -f "$DIR/target/release/mantrac" ] && [ -f "$DIR/target/release/mantra" ]; then
    MANTRAC="$DIR/target/release/mantrac"
    MANTRA="$DIR/target/release/mantra"
elif [ -f "$DIR/target/debug/mantrac" ] && [ -f "$DIR/target/debug/mantra" ]; then
    MANTRAC="$DIR/target/debug/mantrac"
    MANTRA="$DIR/target/debug/mantra"
else
    echo ">> Compiling workspace binaries..."
    cargo build
    MANTRAC="$DIR/target/debug/mantrac"
    MANTRA="$DIR/target/debug/mantra"
fi

# 1. Run Standard Conformance & Diagnostic Tests
echo ">> [1/5] Executing Core Conformance & Diagnostic Suite..."
bash "$DIR/tests/run_all.sh"

# 2. Run Stress Testing Suite
echo ""
echo ">> [2/5] Executing Stress Testing Suite (VM & Native)..."
TMP_STRESS_BIN="/tmp/mantra_stress_bins"
mkdir -p "$TMP_STRESS_BIN"

for test in tests/stress/*.mantra; do
    tname=$(basename "$test" .mantra)
    printf "  Stress (VM): %-40s ... " "$tname"
    $MANTRAC "$test" --run > /dev/null
    echo -e "\033[32mPASS\033[0m"

    printf "  Stress (Native): %-36s ... " "$tname"
    $MANTRAC "$test" -o "$TMP_STRESS_BIN/$tname" > /dev/null
    "$TMP_STRESS_BIN/$tname" > /dev/null
    echo -e "\033[32mPASS\033[0m"
done

# 3. Run Fuzzing Robustness Suite
echo ""
echo ">> [3/5] Executing Automated Compiler Fuzzing Suite (250 iterations)..."
python3 "$DIR/tests/fuzz/fuzz_runner.py" 250

# 4. Run Micro-benchmarks
echo ""
echo ">> [4/5] Executing Performance Micro-benchmarks..."
$MANTRAC "tests/benchmarks/bench_main.mantra" --run

# 5. Run Security Audit Integrity Check
echo ""
echo ">> [5/5] Executing MDK Security Audit Verification..."
cd /tmp/mantra_mdk_integration_test
$MANTRA audit

echo ""
echo "=================================================================="
echo "   🎉 ALL EXTENDED TESTS, FUZZING & REGRESSIONS PASSED SAFELY!    "
echo "=================================================================="

