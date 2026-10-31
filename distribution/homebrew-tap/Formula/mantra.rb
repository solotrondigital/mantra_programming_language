# ==============================================================================
# Homebrew Formula for Mantra Programming Language Toolchain
# Tap: solotrontech/tap
# Formula: mantra.rb
#
# Usage:
#   brew tap solotrontech/tap
#   brew install solotrontech/tap/mantra
#   mantra --version
# ==============================================================================

class Mantra < Formula
  desc "Mantra programming language toolchain — Compiler, MDK, LSP, and Runtime"
  homepage "https://solotrondigital.github.io/mantra_website"
  version "0.7.0"
  license "MIT OR GPL-3.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/solotrondigital/mantra_programming_language/releases/download/v0.7.0/Mantra-0.7.0-macOS.dmg"
      sha256 "MACOS_ARM64_SHA256_PLACEHOLDER"
    else
      url "https://github.com/solotrondigital/mantra_programming_language/releases/download/v0.7.0/Mantra-0.7.0-macOS.dmg"
      sha256 "MACOS_X86_64_SHA256_PLACEHOLDER"
    end
  end

  on_linux do
    url "https://github.com/solotrondigital/mantra_programming_language/releases/download/v0.7.0/mantra-0.7.0-linux.tar.gz"
    sha256 "LINUX_SHA256_PLACEHOLDER"
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
