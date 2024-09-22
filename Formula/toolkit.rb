class Toolkit < Formula
  desc "Generates C# .NET service boilerplate"
  homepage "https://github.com/Missing-Tech/toolkit"
  url "https://github.com/Missing-Tech/toolkit/archive/v0.1.1.tar.gz"
  sha256 "11f008b686e1dc976d68ad26f37c0e2c60615a616888ca207f1964db4d2ef4f1"
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
