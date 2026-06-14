# Bash UZ — Linux va Bash o'rganish ekotizimi

> Rust-book + Rustlings modelida ishlovchi **uzbek tilidagi** to'liq Bash & Linux o'qitish ekotizimi.

---

## Uchta tayanch (Three Pillars)

| Pillar          | Mavzu                              | Texnologiya       |
|-----------------|------------------------------------|-------------------|
| 📘 **Kitob**     | 16 ta bob, ~8 500 qator markdown   | VitePress         |
| 🧪 **Mashqlar**   | 101 ta `# I AM NOT DONE` mashq     | Bash skript fayllar |
| ⚡ **CLI**       | `bashlings` runner + watch         | Rust              |

---

## Tezkor boshlash

### 1. CLI ni o'rnatish

**Bir qatorli install skript** (tayyor binar — Rust shart emas):

```bash
curl -fsSL https://raw.githubusercontent.com/qobulovasror/bashlings/main/scripts/install.sh | sh
```

> Linux/macOS (x86_64 + arm64). Windows uchun [Releases](https://github.com/qobulovasror/bashlings/releases) sahifasidan `.tar.gz` ni qo'lda yuklab oling.

**Homebrew:**

```bash
brew install qobulovasror/bashlings/bashlings
```

**Cargo orqali** (crates.io — Rust kerak):

```bash
cargo install bashlings
```

**Manbadan build qilish:**

```bash
cd cli
cargo build --release
cp target/release/bashlings ~/.local/bin/   # PATH ichida bo'lsin
```

### 2. Mashqlarni ishga tushirish

Repo ildizidan:

```bash
bashlings list           # 101 ta mashqning hammasi va statusi
bashlings watch          # interaktiv rejim — saqlasangiz avto-tekshiruv
bashlings run            # keyingi pending mashqni tekshirish
bashlings run intro1     # aniq bitta mashq
bashlings verify         # hammasini tartibda — birinchi xatoda to'xtaydi
bashlings hint intro1    # progressiv maslahat (har chaqiruvda keyingi bosqich)
bashlings next           # birinchi pending mashq nomini chiqarish (CI uchun)
```

### 3. Kitobni o'qish

Kitob VitePress orqali render bo'ladi:

```bash
npm install
npm run docs:dev         # http://localhost:5173
```

---

## Loyiha tuzilishi

```
bash-doc/
├── docs/                 # 📘 Kitob (VitePress markdown)
│   ├── index.md
│   ├── foreword.md / setup.md / glossary.md
│   ├── part1/            # 5 bob — Linux & Bash asoslari
│   ├── part2/            # 5 bob — Advanced scripting
│   └── part3/            # 6 bob — Real-world ko'nikmalar
├── exercises/            # 🧪 Mashqlar (har biri "# I AM NOT DONE" markerli)
│   ├── info.toml         # markaziy registry
│   ├── 01_intro/         ... 16_cicd/
├── .solutions/           # 🔐 Yechimlar (yashirin — CLI orqali ochiladi)
├── cli/                  # ⚡ bashlings (Rust)
│   ├── src/
│   ├── Cargo.toml
│   └── Formula/          # Homebrew formula
├── STATUS.md             # batafsil status va yo'l xaritasi
└── README.md             # bu fayl
```

---

## Mashqlar tuzilishi

Har mashq quyidagi 3 ta fayldan iborat:

```
exercises/01_intro/
├── intro1.sh             # mashq (# I AM NOT DONE markerli)
├── intro1.hint.md        # 3-bosqichli maslahat
└── README.md             # bobni umumiy tavsifi
```

Yechimlar yashirin `.solutions/` katalogida — to'g'ridan-to'g'ri ko'rilmaydi.
Mashq pass bo'lgandan keyin `bashlings solution <name>` orqali ochiladi
(rustlings-style gating).

### Test format

Har `.sh` fayl oxirida `# @test:...` direktivalar:

| Direktiva                            | Tekshiradi                                  |
|--------------------------------------|---------------------------------------------|
| `# @test:stdout: <matn>`             | stdout aynan berilgan satrga teng           |
| `# @test:stdout-cmd: <buyruq>`       | stdout buyruq natijasiga teng               |
| `# @test:stdout-contains: <matn>`    | stdout berilgan kichik satrni o'z ichiga oladi |
| `# @test:stdout-regex: <pattern>`    | stdout regular expression'ga mos keladi     |
| `# @test:stderr: <matn>`             | stderr aynan berilgan satrga teng           |
| `# @test:exit: <kod>`                | Skript exit code'i teng                     |
| `# @test:file-exists: <yo'l>`        | Berilgan fayl/katalog skriptdan keyin mavjud |

---

## Boblar ro'yxati

### Part 1 — Linux Buyruqlari va Bash Asoslari

| #  | Bob                              | Mashqlar |
|----|----------------------------------|----------|
| 01 | Shell, Terminal va Bash nima?    | 5        |
| 02 | Fayl tizimi va navigatsiya       | 8        |
| 03 | I/O Redirection va Pipelines     | 8        |
| 04 | Matnlar bilan ishlash            | 5        |
| 05 | Birinchi Bash skript             | 6        |

### Part 2 — Advanced Bash Scripting

| #  | Bob                              | Mashqlar |
|----|----------------------------------|----------|
| 01 | Funksiyalar va modullik          | 6        |
| 02 | Massivlar va lug'atlar           | 6        |
| 03 | sed, awk va grep mahorat         | 6        |
| 04 | Signallar va traps               | 5        |
| 05 | Robust skriptlar                 | 5        |

### Part 3 — Real-world ko'nikmalar

| #  | Bob                              | Mashqlar |
|----|----------------------------------|----------|
| 01 | Tarmoq buyruqlari (curl, ping)   | 7        |
| 02 | SSH va remote management         | 7        |
| 03 | JSON va YAML — jq                | 6        |
| 04 | Cron va systemd timer'lar        | 7        |
| 05 | Docker asoslari                  | 7        |
| 06 | CI/CD — GitHub Actions           | 7        |

**Jami:** 16 bob, 101 mashq, ~8 500+ qator markdown.

---

## bashlings CLI buyruqlari

| Buyruq                       | Vazifasi                                              |
|------------------------------|-------------------------------------------------------|
| `bashlings list`             | Hamma mashqlar va statusi (progress bar bilan)        |
| `bashlings list --pending`   | Faqat hali tugatilmagan mashqlar                       |
| `bashlings list --done`      | Faqat tugatilgan mashqlar                              |
| `bashlings list --json`      | Mashqlar ro'yxati JSON ko'rinishida (skript/IDE uchun) |
| `bashlings run [name]`       | Mashqni tekshirish + pass'da marker avto-o'chadi (nomsiz — keyingi pending) |
| `bashlings verify`           | Barcha mashqlarni tartibda tekshirish, birinchi xatoda to'xtash |
| `bashlings watch`            | **Interaktiv rejim** — fayl saqlash + hotkeys (h/s/r/l/q) |
| `bashlings hint <name>`      | Progressiv maslahat — har chaqiruvda keyingi bosqich (yechim YO'Q) |
| `bashlings hint <name> --all`   | Barcha maslahat bosqichlarini birato'la ko'rsatish  |
| `bashlings hint <name> --reset` | Ochilgan maslahat bosqichlarini qayta tiklash       |
| `bashlings solution <name>`  | Yechim — **faqat mashq pass bo'lgandan keyin ochiladi** |
| `bashlings reset <name>`     | Mashqni asl holatga qaytarish (git checkout — asl kod + marker) |
| `bashlings progress`         | Compact statistika — umumiy + qism bo'yicha (`--json` ham) |
| `bashlings next`             | Birinchi pending mashq nomini chiqarish (`--json` ham) |
| `bashlings completions <shell>` | Shell completion skripti (bash/zsh/fish/...)       |
| `bashlings --help`           | Yordam                                                 |
| `bashlings --version`        | Versiyani ko'rsatish                                   |

### Watch rejimida hotkeys

| Tugma     | Harakat                                          |
|-----------|--------------------------------------------------|
| `h`       | Maslahat ko'rsatish                              |
| `s`       | Yechimni ko'rsatish (faqat test pass bo'lsa)      |
| `r` / `Enter` | Joriy mashqni qayta tekshirish                |
| `l` / `p` | Progress overview                                |
| `q` / `Esc` / `Ctrl+C` | Chiqish                              |

### Shell completion o'rnatish

```bash
# bash
bashlings completions bash > ~/.local/share/bash-completion/completions/bashlings
# zsh (fpath ichidagi katalogga)
bashlings completions zsh > ~/.zfunc/_bashlings
```

### Til / Language

Interfeys default **o'zbekcha**. Inglizcha uchun:

```bash
bashlings run intro1 --lang en      # bitta buyruq uchun
export BASHLINGS_LANG=en             # sessiya uchun
```

Hint fayllari ham locale'ga qarab tanlanadi: `<mashq>.hint.en.md` mavjud bo'lsa
inglizchasi, bo'lmasa o'zbekchasiga (`.hint.md`) qaytadi. Barcha 101 hint va 101
mashq tavsifi ikki tilli.

**Kitob** ham ikki tilda: o'zbekcha (default) va inglizcha (`/en/` — saytdagi til
almashtirgich orqali). Barcha 21 bob tarjima qilingan.

### Ranglar

Chiqish TTY bo'lmaganda (masalan `| grep`) ranglar avtomatik o'chadi.
`NO_COLOR=1` — har doim o'chiradi; `CLICOLOR_FORCE=1` — quvurda ham yoqadi.

---

## Talablar

- **Bash 4+** (macOS stock 3.2 ishlamaydi — `brew install bash` qiling)
- **Rust 1.70+** (CLI build qilish uchun)
- **Node.js 18+** (faqat kitobni `npm run docs:dev` qilish uchun)
- `sed`, `awk`, `grep`, `sort`, `uniq`, `wc`, `head`, `tail` (har Unix tizimda mavjud)

---

## Holat va yo'l xaritasi

Loyihaning batafsil holati [`STATUS.md`](./STATUS.md) faylida.

| Pillar | Holat                                                          |
|--------|----------------------------------------------------------------|
| 📘 Kitob | 95% (Part 1+2+3 to'la, Foreword+Setup+Glossary; Appendix qarz) |
| 🧪 Mashqlar | 100% (101/101 — Part 1: 32, Part 2: 28, Part 3: 41)          |
| ⚡ CLI | 100% (8 buyruq: list/run/watch/hint/solution/reset/progress/next) |

---

## Inspiratsiya

- [`jlevy/the-art-of-command-line`](https://github.com/jlevy/the-art-of-command-line)
- [`denysdovhan/bash-handbook`](https://github.com/denysdovhan/bash-handbook)
- [`rust-lang/rustlings`](https://github.com/rust-lang/rustlings)
- [The Rust Book](https://doc.rust-lang.org/book/)

---

## Litsenziya

MIT
