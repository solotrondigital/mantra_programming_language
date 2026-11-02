#!/usr/bin/env python3
"""
Mantra Release Manifest Generator
scripts/release/generate_manifest.py

Generates a production release manifest (release.json) containing metadata,
target architectures, artifact checksums, compiler specifications, and compatibility info.
Extracts the release version dynamically from Cargo.toml.
"""

import os
import sys
import json
import hashlib
import tomllib
import re
from datetime import datetime, timezone

if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
if hasattr(sys.stderr, "reconfigure"):
    sys.stderr.reconfigure(encoding="utf-8", errors="replace")

REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "../.."))

def get_version():
    if len(sys.argv) > 1 and re.fullmatch(r"v?\d+\.\d+\.\d+(?:[-+][0-9A-Za-z.-]+)?", sys.argv[1]):
        v = sys.argv[1]
        return v.lstrip("vV")
    cargo_path = os.path.join(REPO_ROOT, "crates", "mantrac", "Cargo.toml")
    with open(cargo_path, "rb") as f:
        return tomllib.load(f)["package"]["version"]

VERSION = get_version()
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

def get_workspace_crates():
    with open(os.path.join(REPO_ROOT, "Cargo.toml"), "rb") as f:
        workspace = tomllib.load(f)["workspace"]
    crates = []
    for member in workspace["members"]:
        manifest = os.path.join(REPO_ROOT, member, "Cargo.toml")
        with open(manifest, "rb") as f:
            crates.append(tomllib.load(f)["package"]["name"])
    return sorted(crates)

def main():
    os.makedirs(DIST_DIR, exist_ok=True)
    manifest_path = os.path.join(DIST_DIR, "release.json")

    metadata_files = {"release.json", "sbom-spdx.json", "sbom-cyclonedx.json",
                      "SHA256SUMS", "SHA512SUMS", "SHA256SUMS.sig"}
    if not os.path.isdir(DIST_DIR):
        raise SystemExit(f"Release artifact directory does not exist: {DIST_DIR}")
    artifact_paths = [os.path.join(DIST_DIR, name) for name in sorted(os.listdir(DIST_DIR))
                      if name not in metadata_files and os.path.isfile(os.path.join(DIST_DIR, name))]
    if not artifact_paths:
        raise SystemExit("No release artifacts found; refusing to generate a manifest with placeholder hashes")

    artifacts = []
    for path in artifact_paths:
        name = os.path.basename(path)
        artifacts.append({
            "name": name,
            "size_bytes": os.path.getsize(path),
            "sha256": sha256_file(path),
            "sha512": sha512_file(path),
        })

    manifest = {
        "$schema": "https://mantra-lang.org/schemas/release-manifest-v1.json",
        "release": {
            "name": "mantra_programming_languague",
            "version": VERSION,
            "edition": "2026",
            "rust_edition": "2021",
            "channel": "stable",
            "tag": f"v{VERSION}",
            "release_date": datetime.now(timezone.utc).isoformat(),
            "license": "MIT OR GPL-3.0"
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
            "source_edition": "2021",
            "abi_stability": "STABLE_V1"
        },
        "crates_included": get_workspace_crates(),
        "artifacts": artifacts
    }

    with open(manifest_path, "w", encoding="utf-8") as f:
        json.dump(manifest, f, indent=2)

    print(f"[OK] Generated release manifest: {manifest_path} (Version: {VERSION})")

    # Generate SHA256SUMS and SHA512SUMS
    sha256_path = os.path.join(DIST_DIR, "SHA256SUMS")
    sha512_path = os.path.join(DIST_DIR, "SHA512SUMS")

    with open(sha256_path, "w", encoding="utf-8") as f256, open(sha512_path, "w", encoding="utf-8") as f512:
        for art in artifacts:
            f256.write(f"{art['sha256']}  {art['name']}\n")
            f512.write(f"{art['sha512']}  {art['name']}\n")

    print(f"[OK] Generated SHA256SUMS: {sha256_path}")
    print(f"[OK] Generated SHA512SUMS: {sha512_path}")

if __name__ == "__main__":
    main()
