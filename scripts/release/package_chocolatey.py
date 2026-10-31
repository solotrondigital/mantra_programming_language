#!/usr/bin/env python3
"""
Mantra Chocolatey Packager
scripts/release/package_chocolatey.py

Packages the Chocolatey .nupkg file for Mantra from distribution/choco-mantra.
Can run on any platform (Linux, macOS, Windows) without needing choco installed.
"""

import os
import sys
import zipfile
import xml.etree.ElementTree as ET
from datetime import datetime, timezone

REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "../.."))

def get_version():
    if len(sys.argv) > 1 and sys.argv[1] and not sys.argv[1].startswith("-"):
        return sys.argv[1].lstrip("vV")
    cargo_path = os.path.join(REPO_ROOT, "Cargo.toml")
    if os.path.exists(cargo_path):
        with open(cargo_path, "r", encoding="utf-8") as f:
            for line in f:
                line = line.strip()
                if line.startswith("version") and "=" in line:
                    return line.split("=", 1)[1].strip().strip('"').strip("'")
    return "0.5.0"

def update_nuspec(version):
    nuspec_path = os.path.join(REPO_ROOT, "distribution/choco-mantra/mantra.nuspec")
    if not os.path.exists(nuspec_path):
        return None

    with open(nuspec_path, "r", encoding="utf-8") as f:
        content = f.read()

    import re
    content = re.sub(r"<version>[^<]+</version>", f"<version>{version}</version>", content)
    content = re.sub(r"tag/v[^<\"']+", f"tag/v{version}", content)

    with open(nuspec_path, "w", encoding="utf-8") as f:
        f.write(content)

    print(f"✓ Updated mantra.nuspec to version {version}")
    return nuspec_path

def update_install_ps1(version):
    ps1_path = os.path.join(REPO_ROOT, "distribution/choco-mantra/tools/chocolateyInstall.ps1")
    if not os.path.exists(ps1_path):
        return None

    with open(ps1_path, "r", encoding="utf-8") as f:
        content = f.read()

    import re, hashlib
    # Find zip file in dist or installer
    zip_paths = [
        os.path.join(REPO_ROOT, f"dist/releases/v{version}/mantra-{version}-windows.zip"),
        os.path.join(REPO_ROOT, f"installer/windows/mantra-{version}-windows.zip"),
    ]
    checksum = ""
    for zp in zip_paths:
        if os.path.exists(zp):
            with open(zp, "rb") as zf:
                checksum = hashlib.sha256(zf.read()).hexdigest()
            break

    content = re.sub(r"\$version\s*=\s*'[^']+'", f"$version     = '{version}'", content)
    content = re.sub(
        r"\$url64\s*=\s*\"[^\"]+\"",
        f'$url64       = "https://github.com/solotrondigital/mantra_programming_language/releases/download/v$version/mantra-$version-windows.zip"',
        content
    )
    if checksum:
        content = re.sub(r"\$checksum64\s*=\s*'[^']+'", f"$checksum64  = '{checksum}'", content)

    with open(ps1_path, "w", encoding="utf-8") as f:
        f.write(content)

    print(f"✓ Updated chocolateyInstall.ps1 to version {version}")
    return ps1_path

def create_nupkg(version):
    dist_dir = os.path.join(REPO_ROOT, f"dist/releases/v{version}")
    os.makedirs(dist_dir, exist_ok=True)
    nupkg_filename = f"mantra.{version}.nupkg"
    nupkg_path = os.path.join(dist_dir, nupkg_filename)

    choco_dir = os.path.join(REPO_ROOT, "distribution/choco-mantra")
    tools_dir = os.path.join(choco_dir, "tools")
    nuspec_path = os.path.join(choco_dir, "mantra.nuspec")

    # Content Types for Open Packaging Conventions (OPC / .nupkg)
    content_types_xml = """<?xml version="1.0" encoding="utf-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml" />
  <Default Extension="ps1" ContentType="text/plain" />
  <Default Extension="nuspec" ContentType="application/octet-stream" />
  <Override PartName="/mantra.nuspec" ContentType="application/octet-stream" />
</Types>"""

    rels_xml = f"""<?xml version="1.0" encoding="utf-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Type="http://schemas.microsoft.com/packaging/2010/07/manifest" Target="/mantra.nuspec" Id="R1" />
</Relationships>"""

    with zipfile.ZipFile(nupkg_path, "w", zipfile.ZIP_DEFLATED) as z:
        z.writestr("[Content_Types].xml", content_types_xml)
        z.writestr("_rels/.rels", rels_xml)
        if os.path.exists(nuspec_path):
            z.write(nuspec_path, "mantra.nuspec")
        if os.path.exists(tools_dir):
            for root, _, files in os.walk(tools_dir):
                for file in files:
                    full_p = os.path.join(root, file)
                    rel_p = os.path.relpath(full_p, choco_dir)
                    z.write(full_p, rel_p)

    print(f"✓ Generated Chocolatey package: {nupkg_path}")
    return nupkg_path

def main():
    version = get_version()
    update_nuspec(version)
    update_install_ps1(version)
    create_nupkg(version)

if __name__ == "__main__":
    main()

