#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

if command -v cc >/dev/null 2>&1; then
    cc -std=c11 -Wall -Wextra -Werror "$ROOT/tests/abi/abi_header_smoke.c" -o "$TMP_DIR/abi-c"
    "$TMP_DIR/abi-c"
elif command -v cl >/dev/null 2>&1; then
    cl /nologo /std:c11 /TC /W4 /WX "$ROOT/tests/abi/abi_header_smoke.c" \
        "/Fo$TMP_DIR/abi-c.obj" "/Fe$TMP_DIR/abi-c.exe"
    "$TMP_DIR/abi-c.exe"
else
    echo "No supported C compiler (cc or cl) found; C ABI smoke check cannot run" >&2
    exit 1
fi

if command -v c++ >/dev/null 2>&1; then
    c++ -std=c++11 -Wall -Wextra -Werror "$ROOT/tests/abi/abi_header_smoke.cpp" -o "$TMP_DIR/abi-cxx"
    "$TMP_DIR/abi-cxx"
elif command -v cl >/dev/null 2>&1; then
    cl /nologo /std:c++14 /EHsc /TP /W4 /WX "$ROOT/tests/abi/abi_header_smoke.cpp" \
        "/Fo$TMP_DIR/abi-cxx.obj" "/Fe$TMP_DIR/abi-cxx.exe"
    "$TMP_DIR/abi-cxx.exe"
else
    echo "No supported C++ compiler (c++ or cl) found; C++ ABI smoke check cannot run" >&2
    exit 1
fi

echo "C11 and C++11 ABI header checks passed."
