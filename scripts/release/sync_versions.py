#!/usr/bin/env python3
"""
Mantra Project Version Synchronizer
scripts/release/sync_versions.py

Reads project version and forced-update flag from Cargo.toml.
Synchronizes all workspace crates, tools, and distribution files to this version.
"""

import os
import re
import sys
import glob

if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
if hasattr(sys.stderr, "reconfigure"):
    sys.stderr.reconfigure(encoding="utf-8", errors="replace")

REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "../.."))

def read_root_config():
    cargo_path = os.path.join(REPO_ROOT, "Cargo.toml")
    version = None
    forced_update = False
    
    with open(cargo_path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if line.startswith("version") and "=" in line and not version:
                version = line.split("=", 1)[1].strip().strip('"').strip("'")
            elif line.startswith("forced-update") and "=" in line:
                val = line.split("=", 1)[1].strip().strip('"').strip("'").lower()
                forced_update = val in ("true", "1", "yes")
                
    if not version:
        version = "0.7.0"
    return version, forced_update

def sync_crate_cargo_tomls(version):
    crate_tomls = glob.glob(os.path.join(REPO_ROOT, "crates", "*", "Cargo.toml"))
    for toml_path in crate_tomls:
        with open(toml_path, "r", encoding="utf-8") as f:
            content = f.read()
        # Replace package version
        new_content = re.sub(r'(^\[package\][\s\S]*?^version\s*=\s*)"[^"]+"', rf'\1"{version}"', content, flags=re.MULTILINE)
        if new_content != content:
            with open(toml_path, "w", encoding="utf-8") as f:
                f.write(new_content)
            print(f"[OK] Updated {os.path.relpath(toml_path, REPO_ROOT)} to {version}")

def sync_package_jsons(version):
    package_jsons = glob.glob(os.path.join(REPO_ROOT, "tooling", "ide", "*", "package.json"))
    for pkg_path in package_jsons:
        with open(pkg_path, "r", encoding="utf-8") as f:
            content = f.read()
        new_content = re.sub(r'("version"\s*:\s*)"[^"]+"', rf'\1"{version}"', content)
        if new_content != content:
            with open(pkg_path, "w", encoding="utf-8") as f:
                f.write(new_content)
            print(f"[OK] Updated {os.path.relpath(pkg_path, REPO_ROOT)} to {version}")

def sync_mantra_tomls(version):
    mantra_tomls = glob.glob(os.path.join(REPO_ROOT, "**", "mantra.toml"), recursive=True)
    for mt_path in mantra_tomls:
        with open(mt_path, "r", encoding="utf-8") as f:
            content = f.read()
        new_content = re.sub(r'(^\[package\][\s\S]*?^version\s*=\s*)"[^"]+"', rf'\1"{version}"', content, flags=re.MULTILINE)
        if new_content != content:
            with open(mt_path, "w", encoding="utf-8") as f:
                f.write(new_content)
            print(f"[OK] Updated {os.path.relpath(mt_path, REPO_ROOT)} to {version}")

def sync_rust_sources(version):
    # mantra_bootstrap version
    boot_ver = os.path.join(REPO_ROOT, "crates/mantra_bootstrap/src/version.rs")
    if os.path.exists(boot_ver):
        parts = version.split(".")
        major = parts[0] if len(parts) > 0 else "0"
        minor = parts[1] if len(parts) > 1 else "0"
        patch = parts[2] if len(parts) > 2 else "0"
        with open(boot_ver, "r", encoding="utf-8") as f:
            content = f.read()
        content = re.sub(r'pub const MANTRA_VERSION_MAJOR:\s*u32\s*=\s*[0-9]+;', f'pub const MANTRA_VERSION_MAJOR: u32 = {major};', content)
        content = re.sub(r'pub const MANTRA_VERSION_MINOR:\s*u32\s*=\s*[0-9]+;', f'pub const MANTRA_VERSION_MINOR: u32 = {minor};', content)
        content = re.sub(r'pub const MANTRA_VERSION_PATCH:\s*u32\s*=\s*[0-9]+;', f'pub const MANTRA_VERSION_PATCH: u32 = {patch};', content)
        content = re.sub(r'pub const MANTRA_VERSION_STR:\s*&str\s*=\s*"[^"]+";', f'pub const MANTRA_VERSION_STR: &str   = "{version}";', content)
        content = re.sub(r'assert!\(s\.contains\("[^"]+"\)', f'assert!(s.contains("{version}")', content)
        with open(boot_ver, "w", encoding="utf-8") as f:
            f.write(content)
        print(f"[OK] Updated {os.path.relpath(boot_ver, REPO_ROOT)} to {version}")

    # mantra_lsp version
    lsp_srv = os.path.join(REPO_ROOT, "crates/mantra_lsp/src/server.rs")
    if os.path.exists(lsp_srv):
        with open(lsp_srv, "r", encoding="utf-8") as f:
            content = f.read()
        content = re.sub(r'("name":"mantra-lsp","version":)"[^"]+"', rf'\1"{version}"', content)
        with open(lsp_srv, "w", encoding="utf-8") as f:
            f.write(content)

    lsp_main = os.path.join(REPO_ROOT, "crates/mantra_lsp/src/main.rs")
    if os.path.exists(lsp_main):
        with open(lsp_main, "r", encoding="utf-8") as f:
            content = f.read()
        content = re.sub(r'mantra-lsp\)\s*v[0-9.]+', f'mantra-lsp) v{version}', content)
        with open(lsp_main, "w", encoding="utf-8") as f:
            f.write(content)

    # mantra_mdk version
    mdk_main = os.path.join(REPO_ROOT, "crates/mantra_mdk/src/main.rs")
    if os.path.exists(mdk_main):
        with open(mdk_main, "r", encoding="utf-8") as f:
            content = f.read()
        content = re.sub(r'CLI — v[0-9.]+', f'CLI — v{version}', content)
        content = re.sub(r'MDK\)\s*v[0-9.]+', f'MDK) v{version}', content)
        with open(mdk_main, "w", encoding="utf-8") as f:
            f.write(content)

def main():
    version, forced_update = read_root_config()
    print(f"=== Mantra Project Synchronizer ===")
    print(f"Target Version : {version}")
    print(f"Forced Update  : {forced_update}")
    print("===================================")
    sync_crate_cargo_tomls(version)
    sync_package_jsons(version)
    sync_mantra_tomls(version)
    sync_rust_sources(version)
    print(f"[OK] All workspace libraries synchronized to v{version}")

if __name__ == "__main__":
    main()
