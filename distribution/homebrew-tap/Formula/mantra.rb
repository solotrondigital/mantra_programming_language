# ==============================================================================
# Homebrew Formula for Mantra Programming Language Toolchain
# Tap: solotrondigital/tap (or mantra-lang/tap)
# Formula: mantra.rb
#
# Usage:
#   brew tap solotrondigital/tap
#   brew install mantra
#   mantra --version
# ==============================================================================

class Mantra < Formula
  desc "Mantra programming language toolchain — Compiler, MDK, LSP, and Runtime"
  homepage "https://solotrondigital.github.io/mantra_website"
  version "0.4.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/solotrondigital/mantra_programming_language/releases/download/v0.4.0/mantra-0.4.0-aarch64-apple-darwin.tar.gz"
      sha256 "a95cf61a6aae982fd7880afdb147692465c4ec5042e261d5a074461d2d1d595d"
    else
      url "https://github.com/solotrondigital/mantra_programming_language/releases/download/v0.4.0/mantra-0.4.0-x86_64-apple-darwin.tar.gz"
      sha256 "a95cf61a6aae982fd7880afdb147692465c4ec5042e261d5a074461d2d1d595d"
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      url "https://github.com/solotrondigital/mantra_programming_language/releases/download/v0.4.0/mantra-0.4.0-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "a95cf61a6aae982fd7880afdb147692465c4ec5042e261d5a074461d2d1d595d"
    end
  end

  depends_on "llvm" => :recommended # For native compilation backend (Clang)

  def install
    bin.install Dir["bin/*"]
    lib.install Dir["lib/*"] if File.directory?("lib")
    include.install Dir["include/*"] if File.directory?("include")
    share.install Dir["share/*"] if File.directory?("share")
    prefix.install Dir["templates"] if File.directory?("templates")
  end

  test do
    system bin/"mantra", "--version"
    system bin/"mantrac", "--version"
    system bin/"mantra-lsp", "--version"
  end
end
