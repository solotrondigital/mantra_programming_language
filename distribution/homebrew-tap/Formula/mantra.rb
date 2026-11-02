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
  version "0.8.0"
  license "MIT OR GPL-3.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/solotrondigital/mantra_programming_language/releases/download/v0.8.0/Mantra-0.8.0-macOS.dmg"
      sha256 "b71059378c2b4910e207df2a3bfc319a22ea3829c8596b593343ec45c9279716"
    else
      url "https://github.com/solotrondigital/mantra_programming_language/releases/download/v0.8.0/Mantra-0.8.0-macOS.dmg"
      sha256 "b71059378c2b4910e207df2a3bfc319a22ea3829c8596b593343ec45c9279716"
    end
  end

  on_linux do
    url "https://github.com/solotrondigital/mantra_programming_language/releases/download/v0.8.0/mantra-0.8.0-linux.tar.gz"
    sha256 "1369c72f1f8e227c79b8d4dc2510e5b0d4d085ff736bb60f292ee0f4e00f3311"
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
