# Homebrew formula for bashlings (binary install — rust toolchain shart emas).
#
# O'rnatish (tap qilingach):
#   brew install qobulovasror/bashlings/bashlings
#
# Bu fayl `homebrew-bashlings` tap repo'siga qo'yiladi. Har release'dan keyin
# `url`/`sha256` qiymatlari yangilanishi kerak — buni avtomatlashtirish uchun:
#   scripts/update-formula.sh v0.1.0
#
# `version` cli/Cargo.toml dagi versiyaga mos bo'lishi kerak.

class Bashlings < Formula
  desc "Rustlings-style interactive Bash exercises (Uzbek)"
  homepage "https://github.com/qobulovasror/bashlings"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/qobulovasror/bashlings/releases/download/v0.1.0/bashlings-aarch64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_SHA256_aarch64-apple-darwin"
    end
    on_intel do
      url "https://github.com/qobulovasror/bashlings/releases/download/v0.1.0/bashlings-x86_64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_SHA256_x86_64-apple-darwin"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/qobulovasror/bashlings/releases/download/v0.1.0/bashlings-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "REPLACE_WITH_SHA256_aarch64-unknown-linux-gnu"
    end
    on_intel do
      url "https://github.com/qobulovasror/bashlings/releases/download/v0.1.0/bashlings-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "REPLACE_WITH_SHA256_x86_64-unknown-linux-gnu"
    end
  end

  def install
    bin.install "bashlings"
  end

  test do
    # Versiya chiqishi `bashlings 0.x.y` formatida bo'lishi kerak.
    assert_match "bashlings", shell_output("#{bin}/bashlings --version")

    # Workspace'dan tashqarida 'workspace topilmadi' xatosi kutiladi (exit 2).
    output = shell_output("#{bin}/bashlings list 2>&1", 2)
    assert_match "workspace topilmadi", output
  end
end
