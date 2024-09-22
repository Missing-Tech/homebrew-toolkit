class Toolkit < Formula
  desc "Generates C# .NET service boilerplate"
  homepage "https://github.com/Missing-Tech/toolkit"
  url "https://github.com/Missing-Tech/toolkit/archive/v0.1.2.tar.gz"
  sha256 "a4e72550fd22687f01e75a4ef90e2f33332d839bd9573a2194d1a93c6b532590"
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
