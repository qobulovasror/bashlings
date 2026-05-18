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

## Test meta-format

Har `.sh` fayl oxirida quyidagi qatorlardan biri bo'ladi:

| Direktiva                     | Ma'nosi                                             |
|-------------------------------|-----------------------------------------------------|
| `# @test:stdout: <matn>`      | Skript stdout aynan shu satrga teng bo'lishi kerak  |
| `# @test:stdout-cmd: <cmd>`   | Stdout boshqa buyruq natijasiga teng bo'lishi kerak |
| `# @test:exit: <code>`        | Exit code teng bo'lishi kerak                       |
| `# @test:regex: <pattern>`    | Stdout regex'ga mos kelishi kerak                   |
| `# @test:file: <yo'l>`        | Belgilangan fayl yaratilgan bo'lishi kerak          |

## Daraja belgisi

- ★☆☆☆☆ — kirish (2 daqiqa)
- ★★☆☆☆ — boshlovchi
- ★★★☆☆ — o'rta
- ★★★★☆ — kuchli
- ★★★★★ — capstone (mini-loyiha)

## Solutions

`solutions/` katalogida — har mashqning to'g'ri yechimi. **Faqat keying chiqaring!** Avval o'zingiz hal qilib ko'ring, keyin hint'lardan foydalaning, keyin yechimga qarang.
