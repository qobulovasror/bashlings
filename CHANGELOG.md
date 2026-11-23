# Changelog

Bu loyihaning barcha sezilarli o'zgarishlari shu faylda hujjatlanadi.

Format [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) asosida,
versiyalash [SemVer](https://semver.org/) bo'yicha.

## [0.1.2] — chiqarilmagan

### Qo'shildi

- 🧪 **Bir martalik sandbox + `# @setup:` direktivalari** (`file`, `mkdir`, `fixture`, `perm`, `symlink`, `env`, `args`, `stdin`, `timeout`, `isolate-home`) va yangi test rejimlari; `script7`/`script8` mashqlari (jami 103)
- 🔢 **`info.toml` → `min_cli_version`** — eski CLI yangi mashqlarda tushunarli "yangilang" xabarini beradi
- ⌨️ **`watch`: pass ekrani** — mashq o'tganda ekran saqlanadi; `n`/Enter — keyingi mashq, `s` — yechim
- 🪟 **Windows binari (eksperimental)** — rasmiy yo'l WSL2

### O'zgartirildi

- 📊 **Progress `.bashlings/state.toml`'da** — mashq faqat testlari o'tganda "tugatildi"; CLI mashq fayliga endi tegmaydi, `# I AM NOT DONE` marker olib tashlandi. Eski progressni tiklash: `bashlings verify`
- 🐧 **Linux binarlari musl (statik)** — eski glibc'li distrolarda ham ishlaydi
- 🍺 **Homebrew formula `qobulovasror/homebrew-tap` ga ko'chirildi** — o'rnatish: `brew install qobulovasror/tap/bashlings`; `cli/Formula/` olib tashlandi, `scripts/update-formula.sh` endi tap'dagi formulani yangilaydi
- 🧹 **22 mashq `/tmp/bashlings-*` o'rniga sandbox'da ishlaydi**

### Tuzatildi

- 🐛 **Linux/WSL: ko'p thread'li sessiyada har `fork` yiqilardi** — `ulimit -u` endi thread'lar bo'yicha hisoblanadi
- 🐛 **Ctrl+C mashq skriptini to'xtatmasdi** (cheksiz tsikl CPU'ni egallab qolardi)
- 🐛 **`intro3` tegilmasdan o'tardi**; `verify` yiqilgan mashqni ikki marta ishga tushirardi
- 🐛 **Windows build** (crossterm `windows` feature)
- 🧪 `test-solutions.sh`: lokal build'ni ishlatadi, Ctrl+C'da fayllarni tiklaydi, tegilmagan mashq yiqilishini tekshiradi
- 📘 Kitob: `${...:?...}` ichidagi apostrof sintaksis xatosi, `/en` canonical, README'da `git clone` qadami

## [0.1.1] — 2026-06-14

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
