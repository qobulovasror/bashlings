---
title: "Fayl tizimi va navigatsiya"
description: "ls, cd, pwd, mkdir, rm, cp, mv buyruqlari bilan fayl tizimida professional ishlashni o'rganamiz."
---

# 2. Fayl tizimi bo'ylab navigatsiya

Linux/Unix fayl tizimi — bu **bitta katta daraxt**. Uning ildizi `/` belgisi bilan ifodalanadi. Hamma narsa shu ildiz ostida joylashadi: foydalanuvchi fayllari, dasturlar, qurilmalar, hatto tizim sozlamalari ham.

```text
/
├── bin/      # asosiy ijro etiluvchi fayllar (ls, cat, ...)
├── etc/      # konfiguratsiya fayllari
├── home/     # foydalanuvchi katalogi (Linux)
├── Users/    # foydalanuvchi katalogi (macOS)
├── tmp/      # vaqtinchalik fayllar
├── usr/      # qo'shimcha dastur va kutubxonalar
└── var/      # log, mail va o'zgaruvchan ma'lumotlar
```

## 2.1. `pwd` — qayerdaman?

**`pwd`** (Print Working Directory) — siz hozirda turgan katalogning **to'liq yo'lini** ko'rsatadi.

```bash
$ pwd
/Users/mac/Desktop/projects
```

::: tip Absolute va Relative path
- **Absolute path** — `/` dan boshlanadi: `/Users/mac/Documents`
- **Relative path** — joriy katalogga nisbatan: `./docs` yoki `../images`
:::

## 2.2. `ls` — katalogdagi fayllar

**`ls`** (list) — joriy yoki ko'rsatilgan katalogdagi fayllarni ro'yxat qiladi.

```bash
ls                    # oddiy ro'yxat
ls -l                 # batafsil (long format)
ls -a                 # yashirin fayllar (.) bilan
ls -lh                # human-readable hajm (KB, MB)
ls -la                # eng ko'p ishlatiladigan kombinatsiya
ls -lt                # vaqt bo'yicha tartibda
ls -lS                # hajm bo'yicha tartibda
ls /etc               # boshqa katalog ichidagilarni
ls *.md               # faqat .md fayllar
```

`ls -l` natijasi:

```text
-rw-r--r--  1 mac  staff   2.1K Mar 14 10:22 README.md
drwxr-xr-x  4 mac  staff   128B Mar 12 09:00 src
```

Har bir ustun:

| Ustun         | Ma'nosi                                  |
|---------------|------------------------------------------|
| `-rw-r--r--`  | Ruxsatlar (permissions)                  |
| `1`           | Hard link soni                           |
| `mac`         | Egasi (owner)                            |
| `staff`       | Guruh (group)                            |
| `2.1K`        | Hajmi                                    |
| `Mar 14 10:22`| O'zgartirilgan vaqt                      |
| `README.md`   | Fayl nomi                                |

::: info Permissions belgilari
- `-` — oddiy fayl, `d` — katalog, `l` — symlink
- `r` — read, `w` — write, `x` — execute
- Tartib: **egasi / guruh / boshqalar**
:::

## 2.3. `cd` — katalogni o'zgartirish

**`cd`** (Change Directory):

```bash
cd /etc                # absolute path bo'yicha
cd Documents           # joriy katalog ichidagi Documents'ga
cd ..                  # bir pog'ona yuqoriga
cd ../..               # ikki pog'ona yuqoriga
cd ~                   # home katalogga
cd                     # ham home katalogga (qisqartma)
cd -                   # oldingi katalogga qaytish
cd /                   # ildizga
```

::: tip Qisqartmalar
| Belgi    | Ma'nosi                              |
|----------|--------------------------------------|
| `~`      | Home katalog (`/Users/mac`)          |
| `.`      | Joriy katalog                        |
| `..`     | Yuqori katalog                       |
| `-`      | Oldingi katalog (`cd -` bilan)       |
:::

## 2.4. `mkdir` — yangi katalog yaratish

```bash
mkdir loyiha                       # bitta katalog
mkdir loyiha tests docs            # bir nechta katalog
mkdir -p src/components/buttons    # ichma-ich (parent ham yaratiladi)
mkdir -v new_folder                # verbose: nima qilingani ko'rinadi
```

`-p` (parent) flagi juda kuchli — agar yo'lda yo'q kataloglar bo'lsa, hammasini yaratadi:

```bash
mkdir -p projects/2026/january/reports
# barcha 4 daraja yaratiladi
```

::: warning `mkdir` xatolari
Agar `-p` bo'lmasa va `parent` katalog mavjud bo'lmasa — xato beradi:

```text
mkdir: a/b/c: No such file or directory
```
:::

## 2.5. `touch` — bo'sh fayl yaratish

```bash
touch readme.txt                   # yangi bo'sh fayl
touch file1.txt file2.txt          # bir nechta fayl
touch -t 202601011200 old.txt      # vaqtni belgilab yaratish
```

::: info `touch` qo'shimcha vazifasi
Agar fayl mavjud bo'lsa, `touch` uning **oxirgi o'zgartirilgan vaqti**ni yangilaydi (kontentini emas).
:::

## 2.6. `cp` — nusxa olish

```bash
cp manba.txt nusxa.txt              # fayl nusxasi
cp file.txt /tmp/                    # boshqa katalogga
cp -r src/ backup/                   # rekursiv (kataloglar uchun)
cp -v a.txt b.txt                    # verbose
cp -i a.txt b.txt                    # interactive (qayta yozish so'raydi)
cp -p file.txt copy.txt              # ruxsatlar va vaqtni saqlash
cp *.md docs/                        # barcha .md fayllarni
```

::: warning `-r` zarur
Agar siz katalogni nusxa olmoqchi bo'lsangiz, `-r` (recursive) flagini unutmang:

```bash
cp folder1 folder2          # XATO
cp -r folder1 folder2       # TO'G'RI
```
:::

## 2.7. `mv` — ko'chirish yoki nomini o'zgartirish

`mv` ikki vazifani bajaradi: **fayllarni boshqa joyga ko'chirish** va **nomini o'zgartirish**.

```bash
mv eski.txt yangi.txt                # nomini o'zgartirish
mv file.txt /tmp/                    # ko'chirish
mv *.log logs/                       # bir nechta fayl
mv -i a.txt b.txt                    # interactive
mv -n a.txt b.txt                    # mavjud bo'lsa overwrite qilmaslik
mv -v old.txt new.txt                # verbose
```

::: tip Trick
`mv` katalog uchun `-r` talab qilmaydi — u har doim "to'liq" ishlaydi:

```bash
mv src/ archive/   # to'g'ri ishlaydi
```
:::

## 2.8. `rm` — o'chirish

::: danger Eng xavfli buyruq!
`rm` o'chirilgan faylni **qayta tiklab bo'lmaydi**. Trash (savatga) tushmaydi. Yo'q bo'ladi — tamom.
:::

```bash
rm fayl.txt              # bitta fayl
rm a.txt b.txt c.txt     # bir nechta
rm -i fayl.txt           # tasdiqlab so'raydi
rm -f fayl.txt           # force, savol bermasdan
rm -r katalog/           # rekursiv (katalog uchun)
rm -rf katalog/          # FORCE + RECURSIVE — ehtiyot bo'ling!
```

### Xavfli antipatternlar

```bash
rm -rf /                 # ❌ butun tizimni o'chiradi
rm -rf $UNDEFINED_VAR/*  # ❌ agar o'zgaruvchi bo'sh bo'lsa = rm -rf /
rm -rf * .git            # ⚠ probelga e'tibor bering
```

::: tip Xavfsiz amaliyot
1. `rm` o'rniga `trash` yoki `gomi` kabi safe-delete utilitalardan foydalaning.
2. `rm -rf` ni hech qachon o'zgaruvchi bilan ishlatishdan oldin `echo` orqali tekshiring:

```bash
echo rm -rf "$PROJECT_DIR"   # avval ko'ramiz
rm -rf "$PROJECT_DIR"        # endi bajaramiz
```
:::

## 2.9. `rmdir` — bo'sh katalogni o'chirish

`rmdir` faqat **bo'sh kataloglarni** o'chiradi:

```bash
rmdir empty_folder       # bo'sh bo'lsa o'chiradi
rmdir non_empty          # XATO: "Directory not empty"
```

To'la katalog uchun `rm -r` ishlatiladi.

## 2.10. `tree` — daraxt ko'rinishi

Katalog tuzilishini ko'rinishli ko'rsatadi (alohida o'rnatish kerak bo'lishi mumkin):

```bash
brew install tree    # macOS
sudo apt install tree  # Ubuntu/Debian

tree                  # joriy katalog
tree -L 2             # 2 darajagacha
tree -a               # yashirin fayllar ham
tree -d               # faqat kataloglar
```

Natija:

```text
.
├── docs/
│   ├── part1/
│   │   ├── 01-introduction.md
│   │   └── 02-navigation.md
│   └── index.md
└── package.json
```

## 2.11. Wildcards (Globbing)

Bash bir nechta faylni belgilash uchun **wildcard** (joker) belgilarni qo'llab-quvvatlaydi:

| Belgi      | Ma'nosi                                  | Misol                            |
|------------|------------------------------------------|----------------------------------|
| `*`        | Ixtiyoriy belgi(lar)                     | `*.txt`, `report-*`              |
| `?`        | Aniq bitta belgi                         | `file?.txt`                      |
| `[abc]`    | a, b yoki c'dan biri                     | `f[123].log`                     |
| `[a-z]`    | Diapazon                                 | `[a-c]*.md`                      |
| `{a,b}`    | Brace expansion                          | `{src,test}/*.js`                |

Misollar:

```bash
ls *.md                       # barcha .md fayllar
rm temp_?.log                 # temp_1.log, temp_2.log, ...
cp report-{2024,2025}.pdf bk/  # ikkalasi bilan ishlaydi
ls /etc/[a-c]*                # a, b yoki c bilan boshlanadigan
```

## 2.12. Real misol: loyiha skeletini yaratish

```bash
# 1. Asosiy katalog
mkdir -p ~/projects/my-app
cd ~/projects/my-app

# 2. Struktura
mkdir -p src/{components,utils,services} tests docs

# 3. Bo'sh fayllar
touch README.md .gitignore package.json
touch src/index.js src/utils/helpers.js

# 4. Tekshirib ko'ramiz
tree -L 3
```

## 2.13. Mashqlar

::: tip 🧪 Bashlings — interaktiv mashqlar
Bu bobning **8 ta** mashqi `bashlings` CLI orqali avto-tekshiruv bilan:

```bash
bashlings watch              # birinchi pending mashqdan boshlash
bashlings run nav1           # bitta mashqni tekshirish
bashlings hint nav1          # bosqichli maslahat
```

Manba: [`exercises/02_navigation/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/02_navigation)
:::

Quyidagi qo'shimcha vazifalarni terminalda qo'l bilan bajaring:

1. Home katalogingizda `bash-mashq` nomli katalog yarating va ichida `notes`, `scripts`, `archive` ichki kataloglarini bir buyruq bilan yarating.
2. `scripts/` ichida `hello.sh`, `backup.sh`, `cleanup.sh` fayllarini yarating.
3. `hello.sh`ning nusxasini `hello.bak` qilib bir xil katalogga oling.
4. `archive` katalogini o'chirib tashlang.
5. Barcha `.sh` fayllarni bir buyruq bilan `notes/` katalogiga ko'chiring.

## 2.14. Xulosa

| Buyruq    | Vazifasi                                    |
|-----------|---------------------------------------------|
| `pwd`     | Joriy katalogni ko'rsatish                  |
| `ls`      | Katalog tarkibini ko'rish                   |
| `cd`      | Katalogni o'zgartirish                      |
| `mkdir`   | Yangi katalog yaratish                      |
| `touch`   | Yangi bo'sh fayl yaratish                   |
| `cp`      | Nusxa olish                                 |
| `mv`      | Ko'chirish yoki nomini o'zgartirish         |
| `rm`      | O'chirish (ehtiyotkorlik bilan!)            |
| `rmdir`   | Bo'sh katalogni o'chirish                   |
| `tree`    | Daraxt ko'rinishida ko'rsatish              |

Endi sizda fayl tizimida erkin harakatlanish ko'nikmasi bor. Keyingi bobda **I/O redirection** va **pipelines** orqali buyruqlarni bog'lashni o'rganamiz.

> **Keyingi sahifa:** [3. I/O Redirection va Pipelines →](./03-pipes-redirection)
