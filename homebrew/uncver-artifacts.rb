class UncverArtifacts < Formula
  desc "CLI tool for managing uncver artifacts with Podman integration"
  homepage "https://github.com/sirdavis99/uncver-artifacts"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/sirdavis99/uncver-artifacts/releases/download/v0.1.0/uncver-artifacts-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_ARM64"
    end
    on_intel do
      url "https://github.com/sirdavis99/uncver-artifacts/releases/download/v0.1.0/uncver-artifacts-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_X86_64"
    end
  end

  def install
    bin.install "uncver-artifacts"
  end

  def caveats
    <<~EOS
      uncver-artifacts has been installed!
      
      To get started:
        uncver-artifacts install    # Install Podman dependencies
        uncver-artifacts list       # List available artifacts
        uncver-artifacts run        # Run default artifacts
      
      For more information:
        uncver-artifacts --help
    EOS
  end

  test do
    system "#{bin}/uncver-artifacts", "--version"
  end
end
