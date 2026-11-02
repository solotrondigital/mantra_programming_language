#!/usr/bin/env python3
"""Create a Homebrew formula whose URLs and checksums match release assets."""

import hashlib
import sys
from pathlib import Path

from version_config import read_config


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as artifact:
        for chunk in iter(lambda: artifact.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def main():
    workspace_version, _ = read_config()
    version = sys.argv[1] if len(sys.argv) > 1 else workspace_version
    if version.lstrip("vV") != workspace_version:
        raise SystemExit(f"Requested Homebrew version {version} does not match Cargo workspace version {workspace_version}")
    version = workspace_version
    artifacts_dir = Path(sys.argv[2]) if len(sys.argv) > 2 else Path("dist/release-assets")
    formula_path = Path(sys.argv[3]) if len(sys.argv) > 3 else Path("dist/mantra.rb")
    macos = artifacts_dir / f"Mantra-{version}-macOS.dmg"
    linux = artifacts_dir / f"mantra-{version}-linux.tar.gz"
    missing = [str(path) for path in (macos, linux) if not path.is_file()]
    if missing:
        raise SystemExit("Missing Homebrew release assets: " + ", ".join(missing))

    mac_sha = sha256(macos)
    linux_sha = sha256(linux)
    formula = f'''class Mantra < Formula
  desc "Mantra programming language toolchain — Compiler, MDK, LSP, and Runtime"
  homepage "https://solotrondigital.github.io/mantra_website"
  version "{version}"
  license "MIT OR GPL-3.0"

  on_macos do
    url "https://github.com/solotrondigital/mantra_programming_language/releases/download/v{version}/{macos.name}"
    sha256 "{mac_sha}"
  end

  on_linux do
    url "https://github.com/solotrondigital/mantra_programming_language/releases/download/v{version}/{linux.name}"
    sha256 "{linux_sha}"
  end

  depends_on "llvm" => :recommended

  def install
    bin.install Dir["bin/*"]
    lib.install Dir["lib/*"] if File.directory?("lib")
    include.install Dir["include/*"] if File.directory?("include")
    share.install Dir["share/*"] if File.directory?("share")
  end

  test do
    system bin/"mantra", "--version"
    system bin/"mantrac", "--help"
  end
end
'''
    formula_path.parent.mkdir(parents=True, exist_ok=True)
    formula_path.write_text(formula, encoding="utf-8")
    print(f"Generated Homebrew formula for v{version}: {formula_path}")


if __name__ == "__main__":
    main()
