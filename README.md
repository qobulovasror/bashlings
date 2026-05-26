# Bash UZ — Linux va Bash o'rganish ekotizimi

> Rust-book + Rustlings modelida ishlovchi **uzbek tilidagi** to'liq Bash & Linux o'qitish ekotizimi.

---

## Uchta tayanch (Three Pillars)

| Pillar          | Mavzu                              | Texnologiya       |
|-----------------|------------------------------------|-------------------|
| 📘 **Kitob**     | 10 ta bob, ~5 500 qator markdown   | VitePress         |
| 🧪 **Mashqlar**   | 60 ta `# I AM NOT DONE` mashq      | Bash skript fayllar |
| ⚡ **CLI**       | `bashlings` runner + watch         | Rust              |

---

## Tezkor boshlash

### 1. CLI ni o'rnatish

**Cargo orqali** (Rust kerak):

```bash
cd cli
cargo install --path .
```

**Manbadan build qilish:**

```bash
cd cli
cargo build --release
cp target/release/bashlings ~/.local/bin/   # PATH ichida bo'lsin
```

**Homebrew (yaqin kelajakda):**

```bash
brew install qobulovasror/bashlings/bashlings
```

### 2. Mashqlarni ishga tushirish

Repo ildizidan:

```bash
bashlings list           # 60 ta mashqning hammasi va statusi
bashlings watch          # interaktiv rejim — saqlasangiz avto-tekshiruv
bashlings run intro1     # bitta mashq
bashlings hint intro1    # 3-bosqichli maslahat
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

| Direktiva                       | Tekshiradi                                  |
|---------------------------------|---------------------------------------------|
| `# @test:stdout: <matn>`        | stdout aynan berilgan satrga teng           |
| `# @test:stdout-cmd: <buyruq>`  | stdout buyruq natijasiga teng               |
| `# @test:exit: <kod>`           | Skript exit code'i teng                     |

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

**Jami:** 10 bob, 60 mashq, ~5 500 qator markdown.

---

## bashlings CLI buyruqlari

| Buyruq                       | Vazifasi                                              |
|------------------------------|-------------------------------------------------------|
| `bashlings list`             | Hamma mashqlar va statusi (progress bar bilan)        |
| `bashlings list --pending`   | Faqat hali tugatilmagan mashqlar                       |
| `bashlings list --done`      | Faqat tugatilgan mashqlar                              |
| `bashlings run <name>`       | Bitta mashqni tekshirish + pass'da marker avto-o'chadi |
| `bashlings watch`            | **Interaktiv rejim** — fayl saqlash + hotkeys (h/s/r/l/q) |
| `bashlings hint <name>`      | 3-bosqichli maslahat (yechim YO'Q)                     |
| `bashlings solution <name>`  | Yechim — **faqat mashq pass bo'lgandan keyin ochiladi** |
| `bashlings reset <name>`     | Mashqni boshlang'ich holatga qaytarish (marker'ni qaytaradi) |
| `bashlings progress`         | Compact statistika — umumiy + qism bo'yicha           |
| `bashlings next`             | Birinchi pending mashq nomini chiqarish (CI uchun)     |
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

---

## Talablar

- **Bash 4+** (macOS stock 3.2 ishlamaydi — `brew install bash` qiling)
- **Rust 1.70+** (CLI build qilish uchun)
- **Node.js 18+** (faqat kitobni `npm run docs:dev` qilish uchun)
- `sed`, `awk`, `grep`, `sort`, `uniq`, `wc`, `head`, `tail` (har Unix tizimda mavjud)

---

## Holat va yo'l xaritasi

Loyihaning batafsil holati [`STATUS.md`](./STATUS.md) faylida.

| Pillar | Holat                                       |
|--------|---------------------------------------------|
| 📘 Kitob | 80% (Part 1+2 to'la, Part 3 yo'q hali)     |
| 🧪 Mashqlar | 100% (60/60 Part 1+2)                    |
| ⚡ CLI | 100% (4/4 MVP: list, run, watch, hint + next) |

---

## Inspiratsiya

- [`jlevy/the-art-of-command-line`](https://github.com/jlevy/the-art-of-command-line)
- [`denysdovhan/bash-handbook`](https://github.com/denysdovhan/bash-handbook)
- [`rust-lang/rustlings`](https://github.com/rust-lang/rustlings)
- [The Rust Book](https://doc.rust-lang.org/book/)

---

## Litsenziya

MIT
