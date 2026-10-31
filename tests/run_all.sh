#!/bin/bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$DIR"

echo "=========================================="
echo "    MANTRA TOOLCHAIN VERIFICATION SUITE    "
echo "=========================================="
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
    echo "Building toolchain binaries..."
    cargo build
    if [ -f "$DIR/target/debug/mantrac.exe" ]; then
        MANTRAC="$DIR/target/debug/mantrac.exe"
        MANTRA="$DIR/target/debug/mantra.exe"
    else
        MANTRAC="$DIR/target/debug/mantrac"
        MANTRA="$DIR/target/debug/mantra"
    fi
fi

PASSED=0
FAILED=0

run_test() {
    local name="$1"
    local cmd="$2"
    printf "  %-50s ... " "$name"
    if output=$(eval "$cmd" 2>&1); then
        echo -e "\033[32mPASS\033[0m"
        PASSED=$((PASSED + 1))
    else
        echo -e "\033[31mFAIL\033[0m"
        echo "$output" | sed 's/^/    /'
        FAILED=$((FAILED + 1))
    fi
}

echo "[1/4] Running Conformance Tests in VM Mode (--run)..."
for test in tests/conformance/*.mantra; do
    tname=$(basename "$test")
    run_test "VM: $tname" "$MANTRAC $test --run"
done

echo ""
echo "[2/4] Running Conformance Tests in Native Mode (-o)..."
TMP_BIN_DIR="/tmp/mantra_conformance_bins"
mkdir -p "$TMP_BIN_DIR"
for test in tests/conformance/*.mantra; do
    tname=$(basename "$test" .mantra)
    bin_path="$TMP_BIN_DIR/$tname"
    run_test "Native: $tname" "$MANTRAC $test -o $bin_path && ( [ -f \"$bin_path\" ] && \"$bin_path\" || \"${bin_path}.exe\" )"
done

echo ""
echo "[3/4] Running Diagnostic Negative Tests (expecting errors)..."
check_negative() {
    local file="$1"
    local expected_code="$2"
    local tname=$(basename "$file")
    printf "  %-50s ... " "Diagnostics: $tname ($expected_code)"
    if output=$($MANTRAC "$file" --run 2>&1); then
        echo -e "\033[31mFAIL (compiled unexpectedly)\033[0m"
        FAILED=$((FAILED + 1))
    else
        if echo "$output" | grep -q "$expected_code"; then
            echo -e "\033[32mPASS ($expected_code verified)\033[0m"
            PASSED=$((PASSED + 1))
        else
            echo -e "\033[31mFAIL (missing $expected_code)\033[0m"
            echo "$output" | sed 's/^/    /'
            FAILED=$((FAILED + 1))
        fi
    fi
}

check_negative "tests/negative/err_abstract_instantiate.mantra" "E0101"
check_negative "tests/negative/err_private_access.mantra" "E0102"
check_negative "tests/negative/err_missing_interface.mantra" "E0103"
check_negative "tests/negative/err_invalid_override.mantra" "E0104"
check_negative "tests/negative/err_unimplemented_abstract.mantra" "E0105"

echo ""
echo "[4/4] Running MDK CLI Integration Tests..."
TEST_APP_DIR="/tmp/mantra_mdk_integration_test"
rm -rf "$TEST_APP_DIR"

run_test "MDK: mantra new test_app" "cd /tmp && rm -rf mantra_mdk_integration_test && $MANTRA new mantra_mdk_integration_test"
run_test "MDK: mantra check" "cd $TEST_APP_DIR && $MANTRA check"
run_test "MDK: mantra run" "cd $TEST_APP_DIR && echo 'q' | $MANTRA run"
run_test "MDK: mantra test" "cd $TEST_APP_DIR && $MANTRA test"
run_test "MDK: mantra fmt" "cd $TEST_APP_DIR && $MANTRA fmt"
run_test "MDK: mantra doc" "cd $TEST_APP_DIR && $MANTRA doc"
run_test "MDK: mantra add dependency" "cd $TEST_APP_DIR && $MANTRA add json 1.0.0"
run_test "MDK: mantra build --release" "cd $TEST_APP_DIR && $MANTRA build --release && target/release/mantra_mdk_integration_test"
run_test "MDK: mantra publish" "cd $TEST_APP_DIR && $MANTRA publish"

echo ""
echo "=========================================="
echo "Summary: $PASSED passed, $FAILED failed"
echo "=========================================="

if [ "$FAILED" -ne 0 ]; then
    exit 1
fi

