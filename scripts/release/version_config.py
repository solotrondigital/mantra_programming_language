#!/usr/bin/env python3
"""Read and validate release metadata from the Cargo workspace manifest."""

import argparse
import json
import os
import re
import subprocess
import tomllib
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
CARGO_TOML = REPO_ROOT / "Cargo.toml"


def read_config():
    with CARGO_TOML.open("rb") as cargo_file:
        config = tomllib.load(cargo_file)
    workspace = config.get("workspace", {})
    version = workspace.get("package", {}).get("version")
    if not isinstance(version, str) or not re.fullmatch(r"\d+\.\d+\.\d+(?:[-+][0-9A-Za-z.-]+)?", version):
        raise ValueError("Cargo.toml must define a valid [workspace.package].version")
    forced = workspace.get("metadata", {}).get("mantra", {}).get("forced-update", False)
    if not isinstance(forced, bool):
        raise ValueError("[workspace.metadata.mantra].forced-update must be a boolean")
    return version, forced


def verify_workspace_packages(version):
    result = subprocess.run(
        ["cargo", "metadata", "--no-deps", "--format-version", "1"],
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    packages = json.loads(result.stdout)["packages"]
    mismatches = [(package["name"], package["version"]) for package in packages if package["version"] != version]
    if mismatches:
        details = ", ".join(f"{name}={package_version}" for name, package_version in mismatches)
        raise ValueError(f"Workspace packages do not all match {version}: {details}")
    print(f"Verified {len(packages)} workspace packages use v{version}")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("field", choices=("version", "forced-update", "verify"), default="version", nargs="?")
    args = parser.parse_args()
    version, forced = read_config()
    if args.field == "version":
        print(version)
    elif args.field == "forced-update":
        print(str(forced).lower())
    else:
        verify_workspace_packages(version)


if __name__ == "__main__":
    main()
