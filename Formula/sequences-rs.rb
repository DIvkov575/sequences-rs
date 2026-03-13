class SequencesRs < Formula
  desc "CLI for training number sequence recognition (Optiver/Flow Traders prep)"
  homepage "https://github.com/DIvkov575/sequences-rs"
  url "https://github.com/DIvkov575/sequences-rs/archive/refs/tags/0.2.0.tar.gz"
  sha256 "99ed0a7fb97cd75ea1271c4d80ba12e93173a425785f77c7cfd644dbc81db263"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: ".")
    bin.install_symlink bin/"sequences-rs" => "sequences"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/sequences-rs --version")
  end
end
