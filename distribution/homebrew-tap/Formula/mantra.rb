# ==============================================================================
# Homebrew Formula for Mantra Programming Language Toolchain
# Tap: solotrondigital/tap
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
  version "0.5.0"
  license "MIT OR GPL-3.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/solotrondigital/mantra_programming_language/releases/download/v0.5.0/Mantra-0.5.0-macOS.dmg"
      sha256 "82695af8348492832200d6573260c316c1427e1c8de6a98a698355b9c927f3b1"
    else
      url "https://github.com/solotrondigital/mantra_programming_language/releases/download/v0.5.0/Mantra-0.5.0-macOS.dmg"
      sha256 "82695af8348492832200d6573260c316c1427e1c8de6a98a698355b9c927f3b1"
    end
  end

  on_linux do
    url "https://github.com/solotrondigital/mantra_programming_language/releases/download/v0.5.0/mantra-0.5.0-linux.tar.gz"
    sha256 "657b5b5e819acaf52ec1c5a403e0b27e17c51f3c238f206021295826e043d30a"
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
