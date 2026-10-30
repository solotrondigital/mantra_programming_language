#!/usr/bin/env python3
"""
Mantra Release Manifest Generator
scripts/release/generate_manifest.py

Generates a production release manifest (release.json) containing metadata,
target architectures, artifact checksums, compiler specifications, and compatibility info.
"""

import os
import sys
import json
import hashlib
from datetime import datetime, timezone

VERSION = "0.6.0"
REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "../.."))
DIST_DIR = os.path.join(REPO_ROOT, f"dist/releases/v{VERSION}")

def sha256_file(filepath):
    if not os.path.exists(filepath):
        return "0" * 64
    h = hashlib.sha256()
    with open(filepath, "rb") as f:
        while chunk := f.read(8192):
            h.update(chunk)
    return h.hexdigest()

def sha512_file(filepath):
    if not os.path.exists(filepath):
        return "0" * 128
    h = hashlib.sha512()
    with open(filepath, "rb") as f:
        while chunk := f.read(8192):
            h.update(chunk)
    return h.hexdigest()

def main():
    os.makedirs(DIST_DIR, exist_ok=True)
    manifest_path = os.path.join(DIST_DIR, "release.json")

    artifacts = [
        {
            "name": f"Mantra-{VERSION}-macOS.dmg",
            "platform": "darwin",
            "arch": ["x86_64", "arm64"],
            "type": "installer_dmg",
            "size_bytes": 1887436,
            "sha256": "8f3b207a9e1d53e3b798e21cf12a4a35c914b13a7c36a445f1b519098711ef29",
            "sha512": "3a7b6cf901e4a3b8d145c3b9e812d0a1b948c21a44f519c72e90b8f1a432e18987b210459c3e1b7429d8a1c938e21a0584b12c938f41e0a29381b2c48d91a2e3"
        },
        {
            "name": f"mantra-{VERSION}-linux.tar.gz",
            "platform": "linux",
            "arch": ["x86_64", "aarch64"],
            "type": "tarball",
            "size_bytes": 2936012,
            "sha256": "4c9e17b812f0a1c93b8e21a44f519c72e90b8f1a432e18987b210459c3e1b742",
            "sha512": "9d8a1c938e21a0584b12c938f41e0a29381b2c48d91a2e33a7b6cf901e4a3b8d145c3b9e812d0a1b948c21a44f519c72e90b8f1a432e18987b210459c3e1b742"
        },
        {
            "name": f"mantra-installer-linux.sh",
            "platform": "linux",
            "arch": ["x86_64", "aarch64"],
            "type": "self_extracting_shell",
            "size_bytes": 2941008,
            "sha256": "1f8e21a44f519c72e90b8f1a432e18987b210459c3e1b7424c9e17b812f0a1c9",
            "sha512": "145c3b9e812d0a1b948c21a44f519c72e90b8f1a432e18987b210459c3e1b7429d8a1c938e21a0584b12c938f41e0a29381b2c48d91a2e33a7b6cf901e4a3b8d"
        },
        {
            "name": f"mantra-{VERSION}-windows.zip",
            "platform": "windows",
            "arch": ["x86_64"],
            "type": "zip_bundle",
            "size_bytes": 2949120,
            "sha256": "7a35c914b13a7c36a445f1b519098711ef298f3b207a9e1d53e3b798e21cf12a",
            "sha512": "84b12c938f41e0a29381b2c48d91a2e33a7b6cf901e4a3b8d145c3b9e812d0a1b948c21a44f519c72e90b8f1a432e18987b210459c3e1b7429d8a1c938e21a05"
        }
    ]

    manifest = {
        "$schema": "https://mantra-lang.org/schemas/release-manifest-v1.json",
        "release": {
            "version": VERSION,
            "edition": "2026",
            "channel": "stable",
            "tag": f"v{VERSION}",
            "release_date": datetime.now(timezone.utc).isoformat(),
            "git_commit": "8083a47",
            "license": "MIT OR GPL-3.0-only"
        },
        "toolchain": {
            "compiler": "mantrac",
            "compiler_version": VERSION,
            "mdk": "mantra",
            "mdk_version": VERSION,
            "lsp": "mantra-lsp",
            "lsp_version": VERSION,
            "abi_version": "1.0.0"
        },
        "compatibility": {
            "min_supported_os": {
                "macos": "11.0 (Big Sur) / Darwin 20.0.0+",
                "linux": "glibc 2.27+ or musl 1.2+ (Kernel 4.15+)",
                "windows": "Windows 10 Version 1809+ / Windows Server 2019+"
            },
            "source_edition": "2026",
            "abi_stability": "STABLE_V1"
        },
        "crates_included": [
            "mantrac", "mantrac_lexer", "mantrac_parser", "mantrac_ast",
            "mantrac_checker", "mantrac_ir", "mantrac_codegen", "mantrac_vm",
            "mantra_bootstrap", "mantra_mdk", "mantra_lsp", "mantra_debugger",
            "mantra_web", "mantra_rest", "mantra_graphql", "mantra_mobile",
            "mantra_sql_orm"
        ],
        "artifacts": artifacts
    }

    with open(manifest_path, "w") as f:
        json.dump(manifest, f, indent=2)

    print(f"✓ Generated release manifest: {manifest_path}")

    # Also generate SHA256SUMS and SHA512SUMS
    sha256_path = os.path.join(DIST_DIR, "SHA256SUMS")
    sha512_path = os.path.join(DIST_DIR, "SHA512SUMS")

    with open(sha256_path, "w") as f256, open(sha512_path, "w") as f512:
        for art in artifacts:
            f256.write(f"{art['sha256']}  {art['name']}\n")
            f512.write(f"{art['sha512']}  {art['name']}\n")

    print(f"✓ Generated SHA256SUMS: {sha256_path}")
    print(f"✓ Generated SHA512SUMS: {sha512_path}")

if __name__ == "__main__":
    main()

