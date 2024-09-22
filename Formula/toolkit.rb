class Toolkit < Formula
  desc "Generates C# .NET service boilerplate"
  homepage "https://github.com/Missing-Tech/toolkit"
  url "https://github.com/Missing-Tech/toolkit/archive/v0.1.2.tar.gz"
  sha256 "b5fa9e58885f3269b8eea70d475ad48765bbe891c67762cebc2ef04b07a9f815"
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
