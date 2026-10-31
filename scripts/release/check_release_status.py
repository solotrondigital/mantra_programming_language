#!/usr/bin/env python3
"""
Mantra Release Status & Forced-Update Checker
scripts/release/check_release_status.py

Checks if the release already exists for the given version and determines
whether the release pipeline should proceed (override) or skip.
"""

import os
import sys
import urllib.request
import urllib.error

if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
if hasattr(sys.stderr, "reconfigure"):
    sys.stderr.reconfigure(encoding="utf-8", errors="replace")

REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "../.."))

def get_config():
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
    return version or "0.7.0", forced_update

def release_exists_on_github(repo, version, token=None):
    url = f"https://api.github.com/repos/{repo}/releases/tags/v{version}"
    req = urllib.request.Request(url)
    req.add_header("User-Agent", "Mantra-Release-Checker")
    if token:
        req.add_header("Authorization", f"Bearer {token}")
    try:
        with urllib.request.urlopen(req) as response:
            return response.status == 200
    except urllib.error.HTTPError as e:
        if e.code == 404:
            return False
        return False
    except Exception:
        return False

def main():
    repo = os.environ.get("GITHUB_REPOSITORY", "solotrondigital/mantra_programming_language")
    token = os.environ.get("GITHUB_TOKEN")
    version, forced_update = get_config()
    
    # Allow CLI override if provided
    if len(sys.argv) > 1 and sys.argv[1]:
        version = sys.argv[1].lstrip("vV")
    if len(sys.argv) > 2 and sys.argv[2]:
        forced_update = sys.argv[2].lower() in ("true", "1", "yes")

    print(f"=== Mantra Release Gate Check ===")
    print(f"Repository     : {repo}")
    print(f"Target Version : v{version}")
    print(f"Forced Update  : {forced_update}")
    
    exists = release_exists_on_github(repo, version, token)
    print(f"Release Exists : {exists}")

    # Set outputs for GitHub Actions if GITHUB_OUTPUT is present
    github_output = os.environ.get("GITHUB_OUTPUT")

    if forced_update:
        print(f">> [Action: OVERRIDE] forced-update=true. Overriding all distribution packages for v{version}.")
        should_release = "true"
        action = "override"
    elif exists:
        print(f">> [Action: SKIP] Distribution package v{version} already exists and forced-update=false. Skipping package release.")
        should_release = "false"
        action = "skip"
    else:
        print(f">> [Action: RELEASE] Distribution package v{version} is new. Proceeding with package release.")
        should_release = "true"
        action = "create"

    if github_output and os.path.exists(github_output):
        with open(github_output, "a", encoding="utf-8") as f:
            f.write(f"version={version}\n")
            f.write(f"forced_update={'true' if forced_update else 'false'}\n")
            f.write(f"should_release={should_release}\n")
            f.write(f"action={action}\n")

    print("=================================")
    if should_release == "false" and "--fail-on-skip" in sys.argv:
        sys.exit(2)
    sys.exit(0)

if __name__ == "__main__":
    main()
