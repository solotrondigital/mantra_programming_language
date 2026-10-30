#!/usr/bin/env python3
"""
Mantra Automated Fuzzing & Robustness Test Suite
tests/fuzz/fuzz_runner.py

Generates pseudo-random and mutated Mantra source inputs (unclosed strings,
deep indentation, corrupted tokens, invalid unicode, circular types) and verifies
that the compiler (mantrac) handles every anomaly gracefully without crashing (no SIGSEGV / panic).
"""

import os
import sys
import subprocess
import random
import string
import tempfile

MANTRAC = os.path.abspath(os.path.join(os.path.dirname(__file__), "../../target/debug/mantrac"))
if not os.path.exists(MANTRAC):
    MANTRAC = os.path.abspath(os.path.join(os.path.dirname(__file__), "../../target/release/mantrac"))

def safe_random_char():
    while True:
        cp = random.randint(0x20, 0xFFFF)
        if not (0xD800 <= cp <= 0xDFFF):
            return chr(cp)

MUTATION_PATTERNS = [
    # Broken Indentation & Tabs
    lambda: "\t" * random.randint(1, 10) + "let x = 1\n",
    # Mismatched Parentheses / Braces
    lambda: "def test(" + "("*random.randint(1, 50) + "):\n    pass\n",
    # Deeply Nested Binary Expressions
    lambda: "let x = " + " + ".join(["1"] * random.randint(50, 300)) + "\n",
    # Unclosed String Literals
    lambda: 'let s = "' + "".join(random.choices(string.ascii_letters, k=50)) + "\nlet y = 2\n",
    # Random Keywords Collision
    lambda: " ".join(random.choices(["class", "def", "if", "while", "pub", "abstract", "override", "implements", "extends", "let", "mut"], k=30)) + "\n",
    # Random Binary / Non-ASCII Garbage
    lambda: "".join(safe_random_char() for _ in range(50)) + "\n",
    # Enormous Identifier Lengths
    lambda: "let " + ("a" * 5000) + " = 42\n",
    # Cyclic Inheritance Syntax
    lambda: "class A extends B:\n    pass\nclass B extends A:\n    pass\n",
    # Null Dereference Patterns
    lambda: "let x: String = null\nlet len = x.length()\n",
]

def run_fuzz_iteration(iter_num):
    idx = random.randint(0, len(MUTATION_PATTERNS) - 1)
    gen_fn = MUTATION_PATTERNS[idx]
    payload = gen_fn()

    with tempfile.NamedTemporaryFile(suffix=".mantra", mode="w", encoding="utf-8", errors="replace", delete=False) as f:
        f.write(payload)
        temp_path = f.name

    try:
        # Run mantrac --check (compiler type check & parse)
        proc = subprocess.run(
            [MANTRAC, temp_path, "--check"],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=5
        )

        stderr = proc.stderr.decode("utf-8", errors="replace")
        stdout = proc.stdout.decode("utf-8", errors="replace")

        # Invariant: Compiler must NEVER panic (fatal Rust panic abort) or crash with signal (SIGSEGV = -11)
        if proc.returncode < 0:
            print(f"FAILED (Crashed with signal {-proc.returncode}) on iteration {iter_num} (pattern {idx})!")
            print(f"Payload:\n{payload}\n")
            return False

        if "panicked at" in stderr or "SIGSEGV" in stderr:
            print(f"FAILED (Compiler panic detected) on iteration {iter_num} (pattern {idx})!")
            print(f"Stderr:\n{stderr}")
            return False

        return True
    except subprocess.TimeoutExpired:
        print(f"FAILED (Hang / Timeout detected) on iteration {iter_num} (pattern {idx})!")
        print(f"Payload repr: {repr(payload)}")
        return False
    finally:
        if os.path.exists(temp_path):
            os.remove(temp_path)

def main():
    total_iters = int(sys.argv[1]) if len(sys.argv) > 1 else 250
    print(f">> Running Mantra Compiler Fuzzing Suite ({total_iters} iterations)...")
    
    passed = 0
    for i in range(1, total_iters + 1):
        if not run_fuzz_iteration(i):
            sys.exit(1)
        passed += 1
        if i % 50 == 0:
            print(f"   [{i}/{total_iters}] iterations completed safely...")

    print(f"✓ All {passed}/{total_iters} fuzzing iterations survived! 0 crashes, 0 panics.")

if __name__ == "__main__":
    main()

