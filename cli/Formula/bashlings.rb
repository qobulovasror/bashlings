# Homebrew formula for bashlings
#
# Foydalanish (release qilingach):
#   brew install your-org/bashlings/bashlings
#
# Yoki tap'siz:
#   brew install --build-from-source ./Formula/bashlings.rb
#
# Eslatma: hozir loyiha v0.1.0 da. Real release qilinmagan,
# `url` va `sha256` maydonlari rasmiy tag chiqarilgach to'ldiriladi.

class Bashlings < Formula
  desc "Rustlings-style interactive Bash exercises (Uzbek)"
  homepage "https://github.com/your-org/bash-doc"
  license "MIT"

  # === Release qilingach to'ldiriladi ===
  # url "https://github.com/your-org/bash-doc/archive/refs/tags/v0.1.0.tar.gz"
  # sha256 "PLACEHOLDER_SHA256"
  # version "0.1.0"
  # ====================================

  # Hozircha: HEAD installation (yangi commit'lar bo'yicha quriladi)
  head "https://github.com/your-org/bash-doc.git", branch: "main"

  depends_on "rust" => :build

  def install
    cd "cli" do
      system "cargo", "install", *std_cargo_args
    end
  end

  test do
    # Versiya chiqishi `bashlings 0.x.y` formatida bo'lishi kerak
    assert_match "bashlings", shell_output("#{bin}/bashlings --version")

    # `bashlings list` ni exercises/info.toml yo'qligida ishlashga harakat —
    # 'workspace topilmadi' xato xabarini ko'rishimiz kerak.
    output = shell_output("#{bin}/bashlings list 2>&1", 2)
    assert_match "workspace topilmadi", output
  end
end
