# Bashlings — Interaktiv Bash mashqlari

Bu — VitePress kitob (`docs/`) ga **parallel ishlovchi mashqlar to'plami**. Har mashq kitobning aniq bobiga bog'langan.

## Boshlash

```bash
# 1. CLI ni quring va o'rnating
cd cli
cargo install --path .

# 2. Repo ildizidan ishlating
cd ..
bashlings list
bashlings run intro1
bashlings watch          # eng qulay rejim — saqlasangiz avto-tekshiradi
bashlings hint intro1    # maslahat
```

## Tuzilish

```
exercises/
├── info.toml             # markaziy registry — barcha mashqlar ro'yxati
├── README.md             # bu fayl
├── 01_intro/             # part1/01-introduction
│   ├── README.md
│   ├── intro1.sh         # mashq (# I AM NOT DONE markerli)
│   ├── intro1.hint.md    # bosqichli maslahatlar
│   └── ...
└── (kelajakda) 02_navigation/, 03_pipes/, ...
```

## Mashq ishlatish jarayoni

1. Faylni oching: `exercises/01_intro/intro1.sh`
2. Yuqorida `# I AM NOT DONE` qatorini ko'rasiz — bu sizning to-do markeringiz
3. Kodda **bilib qo'yilgan xato** yoki **TODO** bor
4. Tuzating va `# I AM NOT DONE` qatorini o'chiring
5. `bashlings run intro1` (yoki `watch` rejimida o'z-o'zidan tekshiradi)

## Sandbox

Har mashq **bir martalik sandbox** ichida ishlaydi
(`.bashlings/sandbox/<nom>-<pid>-<n>/work/`), repo ichida emas. Ya'ni:

- mashq fayl yaratsa, o'chirsa, `chmod` qilsa — repo'ga tegmaydi
- `ls`, `find`, `wc -l` har doim bir xil, ma'lum daraxtni ko'radi
- run tugagach katalog o'chiriladi; ko'rish uchun `bashlings --keep-sandbox run <nom>`

Shuning uchun mashq ichida `work=/tmp/... && cd "$work"` kabi boilerplate
**kerak emas** — quyidagi `# @setup:` direktivalaridan foydalaning.

## Setup meta-format

Skript ishga tushishidan oldin sandbox ichida yaratiladi:

| Direktiva                              | Ma'nosi                                                  |
|----------------------------------------|-----------------------------------------------------------|
| `# @setup:file: <yo'l>`                | Fayl yaratadi; tarkibi keyingi `# \|` qatorlarida         |
| `# @setup:mkdir: <yo'l>`               | Katalog yaratadi (`mkdir -p` kabi)                        |
| `# @setup:fixture: <katalog>`          | Mashq yonidagi `<katalog>/` ni rekursiv nusxalaydi        |
| `# @setup:perm: <yo'l> <0644>`         | Huquqlarni o'rnatadi (sakkizlik)                          |
| `# @setup:symlink: <link> :: <nishon>` | Symlink yaratadi                                          |
| `# @setup:env: KALIT=qiymat`           | Skript uchun environment o'zgaruvchisi                    |
| `# @setup:args: <argumentlar>`         | `$1`, `$#`, `"$@"` — shell qoidasi bo'yicha bo'linadi     |
| `# @setup:stdin:`                      | Skript stdin'i; tarkibi keyingi `# \|` qatorlarida        |
| `# @setup:timeout: <soniya>`           | Vaqt chegarasi (default 10s, maksimum 300)                |
| `# @setup:isolate-home`                | `HOME` ni sandbox ichiga yo'naltiradi (qiymatsiz)          |

Yo'llar sandbox ichida bo'lishi shart — absolut yo'l va `..` rad etiladi.

```bash
# === SETUP (qo'l urmang) ===
# @setup:mkdir: data/sub
# @setup:file: data/a.txt
# |olma
# |anor
# @setup:perm: data/a.txt 0600
```

## Test meta-format

Har `.sh` fayl oxirida quyidagi qatorlardan biri bo'ladi:

| Direktiva                        | Ma'nosi                                             |
|----------------------------------|-----------------------------------------------------|
| `# @test:stdout: <matn>`         | Skript stdout aynan shu satrga teng bo'lishi kerak  |
| `# @test:stdout-cmd: <cmd>`      | Stdout boshqa buyruq natijasiga teng bo'lishi kerak |
| `# @test:stdout-contains: <sub>` | Stdout ichida shu satr bo'lishi kerak               |
| `# @test:stdout-regex: <pattern>`| Stdout regex'ga mos kelishi kerak                   |
| `# @test:stderr: <matn>`         | Stderr aynan shu satrga teng bo'lishi kerak         |
| `# @test:exit: <code>`           | Exit code teng bo'lishi kerak                       |
| `# @test:file-exists: <yo'l>`    | Belgilangan fayl yaratilgan bo'lishi kerak          |
| `# @test:file-missing: <yo'l>`   | Bu yo'lda hech narsa qolmasligi kerak               |
| `# @test:file-content: <yo'l> :: <matn>` | Fayl tarkibi aynan shunday bo'lsin          |
| `# @test:file-contains: <yo'l> :: <sub>` | Fayl ichida shu satr bo'lsin                |
| `# @test:perm: <yo'l> <0755>`    | Fayl huquqlari (sakkizlik)                          |
| `# @test:tree: a.txt, sub/b.txt` | Ish katalogida **aynan** shu fayllar qolsin         |

`file-*` va `perm` yo'llari sandbox ish katalogiga nisbatan hisoblanadi.
`tree` faqat fayllarni sanaydi — bo'sh katalog ko'rinmaydi.
`file-missing` singan symlink'ni ham "mavjud" deb biladi.

## Daraja belgisi

- ★☆☆☆☆ — kirish (2 daqiqa)
- ★★☆☆☆ — boshlovchi
- ★★★☆☆ — o'rta
- ★★★★☆ — kuchli
- ★★★★★ — capstone (mini-loyiha)

## Solutions

`solutions/` katalogida — har mashqning to'g'ri yechimi. **Faqat keying chiqaring!** Avval o'zingiz hal qilib ko'ring, keyin hint'lardan foydalaning, keyin yechimga qarang.
