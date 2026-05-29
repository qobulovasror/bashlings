# Hissa qo'shish — Contributing

Salom! Bashlings'ga hissa qo'shganingiz uchun rahmat. Quyida — eng tez-tez uchraydigan ssenariylar va ularning to'g'ri yo'li.

---

## Loyihaning tuzilishi

```
bash-doc/
├── docs/              # Kitob (VitePress markdown)
├── exercises/         # Mashqlar (.sh fayllar + hint + info.toml)
├── .solutions/        # YASHIRIN yechimlar (CI test uchun)
├── cli/               # bashlings Rust CLI
├── scripts/           # Yordamchi skriptlar
└── .github/workflows/ # CI
```

## Avval o'rnatish

```bash
git clone https://github.com/qobulovasror/bashlings
cd bashlings
cargo install --path cli
bashlings list           # 101 mashq
```

To'liq sozlash uchun → [`docs/setup.md`](./docs/setup.md).

---

## Sizning birinchi hissangiz

### 🐛 Bug yoki noaniqlikni xabar qilish

[GitHub Issues](https://github.com/qobulovasror/bashlings/issues) ga yangi issue oching:

- Qaysi buyruq ishlamadi?
- Qaysi mashq xato deb tushiriladi?
- Kitobning qaysi bobi noaniq?

Imkon qadar **reproduktsiya qadamlari** + **kutilgan vs olingan** natijani yozib bering.

### ✏️ Hujjatda kichik tuzatish (typo, izoh)

1. Repo'ni fork qiling
2. `docs/...` yoki `exercises/.../hint.md` ni tahrirlang
3. PR oching

Kichik fixlar uchun issue ochish shart emas.

### 🧪 Yangi mashq qo'shish

Mashqlar **uchta fayldan** iborat:

```
exercises/01_intro/intro6.sh        # mashq (foydalanuvchi tahrirlaydi)
exercises/01_intro/intro6.hint.md   # 3-bosqichli maslahat (yechim YO'Q)
.solutions/01_intro/intro6.sh       # yechim (yashirin)
```

Va `exercises/info.toml` ga yozuv:

```toml
[[exercises]]
name = "intro6"
path = "01_intro/intro6.sh"
mode = "stdout"                   # yoki "stdout-cmd" / "exit"
hint = "01_intro/intro6.hint.md"
chapter = "part1/01-introduction"
```

#### Mashq fayli shabloni

```bash
#!/usr/bin/env bash
#
# MASHQ: <qisqa sarlavha>
# DARAJA: ★★☆☆☆
# MAVZU: <kitob bo'limi>
#
# <2-3 qator vazifa tavsifi>
#
# Kutilgan:
#     <expected output>
#
# Maslahat:
#   - <hint 1>
#   - <hint 2>

# I AM NOT DONE

# TODO: vazifani bajaring
echo "TODO"

# === TEST META ===
# @test:stdout: <kutilgan matn>
# @test:exit: 0
```

#### Test rejimlari

| Rejim         | Direktiva                       | Tekshiradi                          |
|---------------|---------------------------------|-------------------------------------|
| `stdout`      | `# @test:stdout: <matn>`        | Skript stdout aynan berilgan matnga teng |
| `stdout-cmd`  | `# @test:stdout-cmd: <buyruq>`  | Stdout boshqa buyruq natijasiga teng |
| `exit`        | `# @test:exit: <kod>`           | Exit code aynan teng                |

Bir mashqda bir nechta direktiva bo'lishi mumkin.

#### Hint fayli shabloni

```markdown
# 💡 intro6

## 1-bosqich
<konseptual maslahat — yo'nalish>

## 2-bosqich
<aniqroq misol bilan>

## 3-bosqich (ixtiyoriy)
<chuqurroq yo'nalish>
```

⚠️ **Yechimni hint'ga qo'shmang!** Yechim faqat `.solutions/` faylida.

### ✅ Yangi mashqni tekshirish

```bash
# Build CLI
cargo build --release --manifest-path cli/Cargo.toml

# Bitta mashq
./cli/target/release/bashlings run intro6

# Hamma yechimlar
./scripts/test-solutions.sh

# Unit testlar
cargo test --manifest-path cli/Cargo.toml
```

---

## CLI ishlab chiqish

```bash
cd cli
cargo build               # debug
cargo test                # unit testlar
cargo clippy -- -D warnings  # lint
cargo run -- list         # ishga tushirish
```

**Yangi buyruq qo'shish:**

1. `cli/src/commands/<name>.rs` da modul yarating
2. `cli/src/commands/mod.rs` ga `pub mod <name>;`
3. `cli/src/main.rs` da `Commands` enum'iga variant qo'shing
4. `dispatch` da ulang
5. Unit test yozing (`#[cfg(test)] mod tests`)

---

## Stil va sifat talablar

- **Tilim:** Mashq tavsifi, hint, README — **o'zbek tilida**. CLI xato xabarlar ham
- **Atamalar:** [`docs/glossary.md`](./docs/glossary.md) bilan mos
- **Bash:** Skriptlar ShellCheck `--severity=warning` ni o'tishi kerak (pedagogik istisnolar bo'lsa, izoh bilan)
- **Rust:** `cargo clippy -- -D warnings` clean
- **Test:** Yangi mantiq → yangi `#[test]`

CI quyidagilarni har push'da tekshiradi:

- ✅ `cargo build --release` (Ubuntu + macOS)
- ✅ `cargo test`
- ✅ `cargo clippy -- -D warnings`
- ✅ Hamma 101 yechim `bashlings run` orqali o'tadi
- ✅ ShellCheck `.solutions/` da
- ✅ `npm run docs:build` muvaffaqiyatli

---

## Commit message stili

Conventional commits — kichik prefiks bilan:

```
feat: net8 mashqi qo'shildi (TLS sertifikat)
fix: cron4 hint'ida 0=Yakshanba xatosi
docs: foreword.md ga screenshot qo'shildi
refactor: info.rs marker logic'ini pure'ga ajratdi
```

Branch nomi: `feature/...`, `fix/...`, `docs/...`.

---

## Code of Conduct

Bizning maqsadimiz — **o'zbek tilida ochiq va do'stona** Bash o'rganish ekotizimi.

- Yangi boshlovchilarga sabr-toqat bilan munosabatda bo'ling
- Texnik tanqid o'rinli, lekin shaxsiy emas
- Tarjima va atamashunoslik bo'yicha bahslar uchun [`docs/glossary.md`](./docs/glossary.md) standart manba

---

## Yordam kerakmi?

- [GitHub Discussions](https://github.com/qobulovasror/bashlings/discussions) — savol-javob
- [GitHub Issues](https://github.com/qobulovasror/bashlings/issues) — bug, feature so'rovi

Rahmat! 🎉
