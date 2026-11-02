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
import tomllib

if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
if hasattr(sys.stderr, "reconfigure"):
    sys.stderr.reconfigure(encoding="utf-8", errors="replace")

REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "../.."))

def read_root_config():
    cargo_path = os.path.join(REPO_ROOT, "Cargo.toml")
    with open(cargo_path, "rb") as f:
        config = tomllib.load(f)
    version = config["workspace"]["package"]["version"]
    forced_update = config.get("workspace", {}).get("metadata", {}).get("mantra", {}).get("forced-update", False)
    return version, bool(forced_update)

def sync_crate_cargo_tomls(version):
    crate_tomls = glob.glob(os.path.join(REPO_ROOT, "crates", "*", "Cargo.toml"))
    for toml_path in crate_tomls:
        with open(toml_path, "r", encoding="utf-8") as f:
            content = f.read()
        # Inherit each library's version from the workspace source of truth.
        package_start = re.search(r"^\[package\]\s*$", content, re.MULTILINE)
        if not package_start:
            continue
        package_end = re.search(r"^\[", content[package_start.end():], re.MULTILINE)
        end = package_start.end() + package_end.start() if package_end else len(content)
        package = content[package_start.start():end]
        package = re.sub(r"(?m)^version(?:\.workspace)?\s*=\s*(?:\"[^\"]+\"|true)\s*$", "version.workspace = true", package, count=1)
        new_content = content[:package_start.start()] + package + content[end:]
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
        new_content = re.sub(r'(?m)^("[^"]+"|[A-Za-z0-9_.-]+)\s*=\s*"0\.7\.0"\s*$', rf'\1 = "{version}"', new_content)
        new_content = re.sub(r'(?m)^(mantra_widgets\s*=\s*)"0\.5\.0"\s*$', rf'\1"{version}"', new_content)
        if new_content != content:
            with open(mt_path, "w", encoding="utf-8") as f:
                f.write(new_content)
            print(f"[OK] Updated {os.path.relpath(mt_path, REPO_ROOT)} to {version}")

def sync_central_example_dependencies(version):
    examples_root = os.path.join(os.path.dirname(REPO_ROOT), "mantra_programming_language_examples")
    if not os.path.isdir(examples_root):
        return
    for mt_path in glob.glob(os.path.join(examples_root, "**", "mantra.toml"), recursive=True):
        with open(mt_path, "r", encoding="utf-8") as f:
            content = f.read()
        updated = re.sub(r'(?m)^("[^"]+"|[A-Za-z0-9_.-]+)\s*=\s*"0\.7\.0"\s*$', rf'\1 = "{version}"', content)
        if updated != content:
            with open(mt_path, "w", encoding="utf-8") as f:
                f.write(updated)
            print(f"[OK] Updated example dependencies in {os.path.relpath(mt_path, examples_root)} to {version}")

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
        content = re.sub(r'pub const MANTRA_VERSION_STR:\s*&str\s*=\s*(?:"[^"]+"|env!\("CARGO_PKG_VERSION"\));', 'pub const MANTRA_VERSION_STR: &str = env!("CARGO_PKG_VERSION");', content)
        content = re.sub(r'assert!\(s\.contains\("[^"]+"\)', f'assert!(s.contains("{version}")', content)
        with open(boot_ver, "w", encoding="utf-8") as f:
            f.write(content)
        print(f"[OK] Updated {os.path.relpath(boot_ver, REPO_ROOT)} to {version}")

    # mantra_lsp version
    lsp_srv = os.path.join(REPO_ROOT, "crates/mantra_lsp/src/server.rs")
    if os.path.exists(lsp_srv):
        with open(lsp_srv, "r", encoding="utf-8") as f:
            content = f.read()
        content = re.sub(r'("name":"mantra-lsp","version":)"[^"]+"', r'\1"{version}"', content)
        with open(lsp_srv, "w", encoding="utf-8") as f:
            f.write(content)

    lsp_main = os.path.join(REPO_ROOT, "crates/mantra_lsp/src/main.rs")
    if os.path.exists(lsp_main):
        with open(lsp_main, "r", encoding="utf-8") as f:
            content = f.read()
        content = re.sub(r'mantra-lsp\)\s*v[0-9.]+', f'mantra-lsp) v{version}', content)
        content = re.sub(r'(?m)(mantra-lsp [^\n]*?\"version\":\")([0-9.]+)(\")', rf'\g<1>{version}\3', content)
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

def sync_distribution_files(version):
    """Keep package manager metadata and installer documentation on the workspace version."""
    nuspec = os.path.join(REPO_ROOT, "distribution", "choco-mantra", "mantra.nuspec")
    if os.path.exists(nuspec):
        content = open(nuspec, encoding="utf-8").read()
        content = re.sub(r"(<version>)[^<]+(</version>)", rf"\g<1>{version}\2", content, count=1)
        content = re.sub(r"releases/tag/v[0-9.]+", f"releases/tag/v{version}", content)
        open(nuspec, "w", encoding="utf-8").write(content)

    choco = os.path.join(REPO_ROOT, "distribution", "choco-mantra", "tools", "chocolateyInstall.ps1")
    if os.path.exists(choco):
        content = open(choco, encoding="utf-8").read()
        content = re.sub(r"(?m)^(\$version\s*=\s*)'[^']+'", rf"\g<1>'{version}'", content, count=1)
        content = re.sub(r"(?m)^(\$checksum64\s*=\s*)'[^']+'", r"\1'REPLACE_WITH_WINDOWS_X86_64_SHA256'", content, count=1)
        open(choco, "w", encoding="utf-8").write(content)

    formula = os.path.join(REPO_ROOT, "distribution", "homebrew-tap", "Formula", "mantra.rb")
    if os.path.exists(formula):
        content = open(formula, encoding="utf-8").read()
        content = re.sub(r'(?m)^(\s*version\s+)"[^"]+"', rf'\g<1>"{version}"', content, count=1)
        content = re.sub(r"/download/v[0-9.]+/", f"/download/v{version}/", content)
        content = re.sub(r"Mantra-[0-9.]+-macOS", f"Mantra-{version}-macOS", content)
        content = re.sub(r"mantra-[0-9.]+-linux", f"mantra-{version}-linux", content)
        sha_names = iter(("AARCH64_DARWIN", "X86_64_DARWIN", "LINUX_X86_64"))
        content = re.sub(
            r'(?m)^(\s*sha256\s+)"[^"]+"',
            lambda match: f'{match.group(1)}"REPLACE_WITH_{next(sha_names)}_SHA256"',
            content,
        )
        open(formula, "w", encoding="utf-8").write(content)

    verification = os.path.join(REPO_ROOT, "distribution", "choco-mantra", "tools", "VERIFICATION.txt")
    if os.path.exists(verification):
        content = open(verification, encoding="utf-8").read()
        content = re.sub(r"(?m)^VERSION: .*?$", f"VERSION: {version}", content)
        content = re.sub(r"/download/v[0-9.]+/mantra-[0-9.]+-x86_64-pc-windows-msvc", f"/download/v{version}/mantra-{version}-x86_64-pc-windows-msvc", content)
        content = re.sub(r"build_distribution\.sh [0-9.]+", f"build_distribution.sh {version}", content)
        open(verification, "w", encoding="utf-8").write(content)

    readmes = [os.path.join(REPO_ROOT, "installer", "README.md")]
    for readme in readmes:
        if os.path.exists(readme):
            content = open(readme, encoding="utf-8").read()
            content = re.sub(r"v[0-9]+\.[0-9]+\.[0-9]+", f"v{version}", content)
            content = re.sub(r"Mantra-[0-9]+\.[0-9]+\.[0-9]+-macOS", f"Mantra-{version}-macOS", content)
            content = re.sub(r"mantra-[0-9]+\.[0-9]+\.[0-9]+-linux", f"mantra-{version}-linux", content)
            content = re.sub(r"mantra-[0-9]+\.[0-9]+\.[0-9]+-windows", f"mantra-{version}-windows", content)
            open(readme, "w", encoding="utf-8").write(content)

    root_readme = os.path.join(REPO_ROOT, "README.md")
    if os.path.exists(root_readme):
        content = open(root_readme, encoding="utf-8").read()
        content = re.sub(r"(badge/version-)[0-9.]+(-cyan\.svg)", rf"\g<1>{version}\2", content)
        content = re.sub(r'(?m)^(mantra\.[A-Za-z0-9_.-]+\s*=\s*)"[0-9.]+"$', lambda m: f'{m.group(1)}"{version}"', content)
        open(root_readme, "w", encoding="utf-8").write(content)

    # Keep the website's active download links on this release; versioned API docs
    # remain historical snapshots and are not rewritten.
    website_download = os.path.join(os.path.dirname(REPO_ROOT), "mantra_website", "mantra_web", "download.html")
    if os.path.exists(website_download):
        content = open(website_download, encoding="utf-8").read()
        content = re.sub(r"0\.7\.0", version, content)
        open(website_download, "w", encoding="utf-8").write(content)

def main():
    version, forced_update = read_root_config()
    print(f"=== Mantra Project Synchronizer ===")
    print(f"Target Version : {version}")
    print(f"Forced Update  : {forced_update}")
    print("===================================")
    sync_crate_cargo_tomls(version)
    sync_package_jsons(version)
    sync_mantra_tomls(version)
    sync_central_example_dependencies(version)
    sync_rust_sources(version)
    sync_distribution_files(version)
    print(f"[OK] All workspace libraries synchronized to v{version}")

if __name__ == "__main__":
    main()
