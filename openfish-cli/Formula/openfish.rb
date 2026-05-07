class Openfish < Formula
  desc "CLI for Openfish — browse markets, trade, and manage positions"
  homepage "https://github.com/billcheung10/openfish-cli"
  version "0.1.12"
  license "MIT"

  on_macos do
    on_intel do
      url "https://github.com/billcheung10/openfish-cli/releases/download/v#{version}/openfish-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "3a5afa3184090ae83fe2d01ec5293164ea061965a08ee9b7c8929226889560d5"
    end

    on_arm do
      url "https://github.com/billcheung10/openfish-cli/releases/download/v#{version}/openfish-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "935a7ab23a8ba2dbb6a31062d54812d582d6e25a02a2d4c56fa95045f0b9c1a4"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/billcheung10/openfish-cli/releases/download/v#{version}/openfish-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "7df892efeb36e4b2b75f62a0c58469de9ea58da95037daa51b90b8709d283264"
    end

    on_arm do
      url "https://github.com/billcheung10/openfish-cli/releases/download/v#{version}/openfish-v#{version}-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "c39b7db9d364ecf9ec4ed44917e7239e5b082fddabb9e84533ce7b61d300c40b"
    end
  end

  def install
    bin.install "openfish"
  end

  test do
    assert_match "openfish", shell_output("#{bin}/openfish --version")
  end
end
