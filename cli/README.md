# bashlings

Interaktiv Bash mashqlar runner (rustlings uslubida, uzbek tilida).

## O'rnatish

### Cargo orqali (eng oson)

Rust o'rnatilgan bo'lsa:

```bash
cd cli
cargo install --path .
```

Binary `~/.cargo/bin/bashlings` ga o'rnatildi. PATH'da bo'lsa, darhol ishlatishingiz mumkin.

### Manbadan build

```bash
cd cli
cargo build --release
# Binary: ./target/release/bashlings
```

Stripped + LTO bilan ~1 MB.

### Homebrew (kelajakda)

Release qilingach:
```bash
brew install qobulovasror/bashlings/bashlings
```

Hozir formula `Formula/bashlings.rb` da — release tag chiqarilgach to'ldiriladi.

## Buyruqlar

| Buyruq                  | Vazifasi                                              |
|-------------------------|-------------------------------------------------------|
| `bashlings list`        | Hamma mashqlar va statusi (progress bar bilan)        |
| `bashlings run <name>`  | Bitta mashqni tekshirish + diff                       |
| `bashlings watch`       | **Avto-tekshiruv** — fayl saqlanganda darhol ishlaydi |
| `bashlings hint <name>` | Markdown hint terminalda chiroyli render               |
| `bashlings next`        | Birinchi pending mashq nomi (skriptlar uchun)         |
| `bashlings --help`      | Yordam                                                 |
| `bashlings --version`   | Versiya                                                |

## Arxitektura

```
cli/src/
├── main.rs              # clap CLI definitsiyasi + dispatch
├── info.rs              # info.toml parser
├── test.rs              # @test:... direktivalar + assertion evaluator
└── commands/
    ├── mod.rs
    ├── list.rs          # `bashlings list`
    ├── run.rs           # `bashlings run`
    ├── watch.rs         # `bashlings watch`
    ├── hint.rs          # `bashlings hint`
    └── next.rs          # `bashlings next`
```

CLI ildiz katalogini avtomatik topadi: yuqoriga ko'tarila boradi va `exercises/info.toml` mavjud bo'lgan birinchi katalogni tanlaydi.

## Dependency'lar

| Crate         | Maqsad                                  |
|---------------|------------------------------------------|
| `clap`        | CLI argument parsing                     |
| `serde + toml`| `info.toml` deserializing                |
| `anyhow`      | Xato wrap                                |
| `owo-colors`  | Terminal rang                            |
| `notify`      | Filesystem events (watch rejimi uchun)   |
| `libc` (Unix) | SIGPIPE default'ga qaytarish             |

## Exit kodlar

| Kod | Mazmuni                                             |
|-----|------------------------------------------------------|
| 0   | Muvaffaqiyat / `next` da pending mashq topildi       |
| 1   | Tests fail bo'ldi / `next` da hech narsa pending emas |
| 2   | Anyhow xato (workspace topilmadi, parse xatosi, ...) |

## MVP statusi

| Buyruq           | Status      |
|------------------|-------------|
| `bashlings list` | ✅ Tayyor   |
| `bashlings run`  | ✅ Tayyor   |
| `bashlings watch`| ✅ Tayyor   |
| `bashlings hint` | ✅ Tayyor   |
| `bashlings next` | ✅ Tayyor   |

## Kelajakdagi rejalashtirilgan xususiyatlar

- [ ] `--no-color` flag (owo-colors `if_supports_color` ga migratsiya bilan)
- [ ] Keyboard shortcuts `watch` ichida (`q`, `n`, `h`, `l`)
- [ ] Progress state'ni saqlash (`~/.bashlings/state.json`)
- [ ] `bashlings reset <name>` — mashqni boshlang'ich holatga qaytarish
- [ ] `bashlings init` — bo'sh skelet workspace yaratish

## Litsenziya

MIT
