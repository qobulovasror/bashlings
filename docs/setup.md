---
title: "O'rnatish va sozlash"
description: "Bash 4+, bashlings CLI va yordamchi vositalarni (jq, rust) o'rnatish — macOS / Linux / Windows uchun."
---

# O'rnatish va sozlash

Bu sahifada **0 dan boshlab** ishchi muhitni tayyorlaymiz. Hammasi
~15 daqiqada.

## Kerakli vositalar

| Vosita     | Talab | Sabab                                       |
|------------|-------|---------------------------------------------|
| **Bash 4+** | majburiy | Mashqlar shu versiyaga taqalgan (assoc array, `mapfile`) |
| **Rust** (cargo) | majburiy | `bashlings` CLI ni build qilish uchun |
| **git**    | majburiy | Repo'ni klon qilish |
| **jq**     | tavsiya etiladi | Part 3 / `13_jq` mashqlari uchun |
| **Node.js 18+** | ixtiyoriy | Kitobni `npm run docs:dev` orqali lokal ko'rish uchun |

---

## 1. Bash 4+ o'rnatish

### macOS

::: warning macOS'da default Bash — **3.2** (2007-yilgi)
Mashqlar **Bash 4+** ga taqalgan. Yangilash kerak.
:::

```bash
# Homebrew bilan
brew install bash

# Tekshirish
bash --version    # 5.x bo'lishi kerak
```

Bash'ni standart qilish (ixtiyoriy):
```bash
# /etc/shells fayliga qo'shing
echo "/opt/homebrew/bin/bash" | sudo tee -a /etc/shells

# Standart shell qilib o'rnating
chsh -s /opt/homebrew/bin/bash
```

### Linux

```bash
# Ubuntu / Debian
sudo apt update && sudo apt install -y bash

# Tekshirish
bash --version    # odatda 5.x
```

### Windows

Bash'ni Windows'da to'g'ridan-to'g'ri ishlatib bo'lmaydi. **WSL2** yoki **Git Bash** kerak:

```powershell
# WSL2 (tavsiya etiladi)
wsl --install -d Ubuntu

# Yoki Git for Windows: https://git-scm.com/download/win
```

---

## 2. Rust toolchain o'rnatish

`bashlings` CLI Rust'da yozilgan — `cargo` bilan build qilamiz.

```bash
# Linux / macOS / WSL — rustup orqali
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Terminalni qayta oching yoki:
source "$HOME/.cargo/env"

# Tekshirish
cargo --version    # cargo 1.70+
```

---

## 3. Bashlings CLI o'rnatish

```bash
# 1. Repo'ni klon qiling
git clone https://github.com/qobulovasror/bashlings
cd bashlings

# 2. CLI ni build + install qiling
cd cli
cargo install --path .

# 3. Tekshirish — repo ildizidan
cd ..
bashlings list
```

`bashlings list` natijasi:
```
  Bashlings  ·  101 ta mashq

  ·  intro1     part1/01-introduction
  ·  intro2     part1/01-introduction
  ...
  Progress: 0 / 101  (0%)

  Keyingi: intro1
```

::: tip Homebrew (yaqin kelajakda)
Homebrew formula tayyorlanmoqda. Hozirgi paytda `cargo install` orqali
o'rnating.
:::

### Asosiy buyruqlar — qisqacha

| Buyruq                       | Vazifasi                                              |
|------------------------------|-------------------------------------------------------|
| `bashlings list`             | 101 mashq + status                                    |
| `bashlings watch`            | **Boshlash uchun shu** — interaktiv rejim             |
| `bashlings run intro1`       | Bitta mashqni tekshirish                              |
| `bashlings hint intro1`      | Maslahat (3-bosqich)                                  |
| `bashlings solution intro1`  | Yechim — faqat pass bo'lgach ochiladi 🔒              |
| `bashlings reset intro1`     | Boshlang'ich holatga qaytarish                        |
| `bashlings progress`         | Compact statistika                                    |

::: info Yechimlar yashirin
`.solutions/` katalogi *hidden* — `ls` da ko'rinmaydi. Mashqni hal qilganingizdan
keyingina `bashlings solution <name>` orqali yechimni ko'rishingiz mumkin.
Bu — rustlings'dagi kabi, o'zingiz fikrlash uchun bosim hosil qiladi.
:::

---

## 4. Yordamchi vositalar

### jq — JSON parser (Part 3 / 13_jq mashqlari uchun)

```bash
# macOS
brew install jq

# Ubuntu / Debian
sudo apt install -y jq

# Tekshirish
jq --version    # jq-1.6 yoki yangiroq
```

### Boshqa standart vositalar

`sed`, `awk`, `grep`, `sort`, `uniq`, `wc`, `head`, `tail`, `cut`, `tr` —
har Unix tizimda mavjud. Alohida o'rnatish shart emas.

::: warning macOS BSD vs GNU
macOS'da `sed -i` va `awk` GNU versiyasidan biroz farq qiladi. Mashqlar
**portativ** sintaksis ishlatadi, lekin og'ir vazifalar uchun GNU
versiyasini olish foydali:
```bash
brew install gnu-sed gawk grep
# va `gsed`, `gawk`, `ggrep` orqali chaqiring
```
:::

---

## 5. Node.js (ixtiyoriy — kitobni lokal ko'rish)

```bash
# macOS
brew install node

# Ubuntu (NodeSource orqali)
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs

# Repo ildizida
npm install
npm run docs:dev    # http://localhost:5173
```

---

## 6. To'liq tekshiruv

Repo ildizidan:

```bash
bashlings --version           # 0.1.x
bashlings list                # 101 mashq
bashlings run intro1          # birinchi mashq — xato berishi normal!
```

Agar `bashlings run intro1` quyidagi natijani bersa — siz tayyor:

```
  Running intro1
  Fayl: exercises/01_intro/intro1.sh

  ✗  stdout      expected:  "Salom, Bash!"
                  actual:    ""
  ✗  exit        expected:  "0"
                  actual:    "127"

  ❌ intro1 — xato (0/2)

  Skript stderr chiqishi:
    .../intro1.sh: line 13: eko: command not found

  💡 Maslahat: bashlings hint intro1
```

Bu — birinchi mashqning **boshlang'ich (broken)** holati. Endi siz fayl'ni
oching va to'g'rilang!

---

## Tez-tez uchraydigan muammolar

::: details "command not found: bashlings"
`cargo install --path .` muvaffaqiyatli o'tdi, lekin `bashlings` topilmaydi?

`~/.cargo/bin` PATH'da emas. `.bashrc` yoki `.zshrc` ga qo'shing:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```
Keyin: `source ~/.bashrc` (yoki yangi terminal oching).
:::

::: details "workspace topilmadi" xatosi
`bashlings` faqat repo ildizidan ishlaydi (yoki uning subkatalogidan).
`exercises/info.toml` faylini topa olmasligi mumkin. Yechim: repo ildiziga
o'ting (`cd bashlings`).
:::

::: details Bash 3.2 muammolari (macOS)
Mashq `declare -A` yoki `mapfile` xatolar bersa — siz hali Bash 3.2 dan
foydalanyapsiz. `which bash` bilan tekshiring; `/bin/bash` o'rniga
`/opt/homebrew/bin/bash` bo'lishi kerak.
:::

::: details `jq: command not found`
Faqat `13_jq` mashqlari uchun kerak. `brew install jq` yoki
`sudo apt install jq` bilan o'rnating.
:::

---

## Keyingi qadam

Sozlash tugadi. Endi birinchi bobga o'tamiz:

[**→ 1-bob: Shell, Terminal va Bash nima?**](/part1/01-introduction)
