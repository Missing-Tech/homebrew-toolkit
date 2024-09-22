class Toolkit < Formula
  desc "Generates C# .NET service boilerplate"
  homepage "https://github.com/Missing-Tech/toolkit"
  url "https://github.com/Missing-Tech/toolkit/archive/v0.1.0.tar.gz"
  sha256 "fd871f5a28bf3e433234093044e7eda7f0364311de3e242520902a167bb71cb8"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--path", ".", "--root", prefix
  end

  test do
    # Basic test to check if the tool is installed and accessible
    assert_match "toolkit", shell_output("#{bin}/toolkit --version")
  end
end
