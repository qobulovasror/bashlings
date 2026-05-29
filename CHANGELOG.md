# Changelog

Bu loyihaning barcha sezilarli o'zgarishlari shu faylda hujjatlanadi.

Format [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) asosida,
versiyalash [SemVer](https://semver.org/) bo'yicha.

## [Unreleased]

### Qo'shildi

- 🧪 **35 ta yangi Part 3 mashqi** — `11_network`, `12_ssh`, `14_cron`, `15_docker`, `16_cicd` (jami 101 mashq)
- 📘 **3 ta yangi sahifa** — `foreword.md`, `setup.md`, `glossary.md`
- 📘 **Part 1 boblariga top callout** — "What you'll learn" + Vaqt + Mashqlar (Part 2+3 ga sinxron)
- 📘 **Kitobdan mashqqa link** — 16 ta bobda `bashlings watch` callout
- ⚡ **CLI yangi buyruqlar:**
  - `bashlings solution <name>` — yechim, **faqat test pass bo'lgach** ochiladi
  - `bashlings reset <name>` — `# I AM NOT DONE` markerni qaytaradi
  - `bashlings progress` — compact statistika (umumiy + qism bo'yicha)
  - `bashlings list --pending` / `--done` filtrlari
- ⚡ **Watch interaktiv hotkeys** (crossterm raw mode):
  - `h` → hint
  - `s` → solution (🔒 gated)
  - `r` / `Enter` → re-run
  - `l` / `p` → progress
  - `q` / `Esc` / `Ctrl+C` → quit
- 🤖 **GitHub Actions CI** — cargo build/test/clippy, 101 yechim test, ShellCheck, VitePress build (Ubuntu + macOS)
- 🧪 **31 ta Rust unit test** — info modul + test modul
- 📝 **`CONTRIBUTING.md`** — yangi mashq qo'shish va CLI ishlab chiqish ko'rsatmasi
- 📝 **`LICENSE`** (MIT)
- 📝 **`CHANGELOG.md`** (bu fayl)
- 📝 **`scripts/test-solutions.sh`** — barcha 101 yechimni avto-tekshirish
- 📘 VitePress nav "Boshlash" dropdown (Foreword/Setup/Glossary)

### O'zgartirildi

- 🔐 **`solutions/` → `.solutions/`** — yechim fayllari yashirin katalogga ko'chirildi (rustlings-style)
- ⚡ **Test pass'da `# I AM NOT DONE` AVTO-o'chadi** — manual edit'dan friction olib tashlandi
- 📘 **Hint fayllaridan `## ✅ Yechim` bo'limi olib tashlandi** (101 fayl) — yechim faqat `bashlings solution` orqali

### Tuzatildi

- 🐛 **`is_done` marker bug** — `contains()` o'rniga `lines().any()` ishlatiladi. Avval `intro1` kabi description'da marker matni bo'lgan fayllar noto'g'ri "not done" deb belgilanardi
- 🐛 **Clippy warning'lari** — `emoji.to_string()` va format string'lar tozalandi (deny-warnings clean)

---

## [0.1.0] — 2026-05-16

Birinchi public release.

### Qo'shildi

- 📘 Part 1 + Part 2 boblari (10 bob)
- 📘 Part 3 — 6 bob (network, ssh, jq, cron, docker, cicd)
- 🧪 66 ta mashq (intro / navigation / pipes / text / scripting / functions / arrays / sed-awk / traps / robust / jq)
- ⚡ MVP CLI: `list`, `run`, `watch`, `hint`, `next` (5 buyruq)
- 📦 Homebrew formula skeleti (`cli/Formula/bashlings.rb`)

---

[Unreleased]: https://github.com/qobulovasror/bashlings/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/qobulovasror/bashlings/releases/tag/v0.1.0
