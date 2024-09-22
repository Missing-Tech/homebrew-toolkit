class Toolkit < Formula
  desc "Generates C# .NET service boilerplate"
  homepage "https://github.com/Missing-Tech/toolkit"
  url "https://github.com/Missing-Tech/toolkit/archive/v0.1.3.tar.gz"
  sha256 "0e2bec8ed0453d3204a01d3ba03b0ababe917d0824e28a3759fe0a26c95c30af"
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
