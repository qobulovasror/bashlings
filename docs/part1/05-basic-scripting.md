---
title: "Birinchi Bash skript"
description: "Shebang, o'zgaruvchilar, if/else shartlari va looplar — Bash skriptingning asoslari."
---

# 5. Birinchi Bash skript

> Bu bobda nima o'rganasiz:
> - **Shebang** (`#!/usr/bin/env bash`) va executable huquq
> - **O'zgaruvchilar** (`name=value`, qo'shtirnoq qoidalari)
> - **Argumentlar** (`$1`, `$@`, `$#`)
> - **`if` / `else`** — shartli mantiq
> - **`for`** loop
> - Foydalanuvchidan input olish (`read`)
>
> **⏱ Vaqt:** ~40 daqiqa
> **🧪 Mashqlar:** `bashlings watch` — 6 ta interaktiv mashq tayyor ([`exercises/05_scripting/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/05_scripting))

Endi siz terminalda erkin harakatlanasiz, buyruqlarni bog'lashni bilasiz. Vaqti keldi — buyruqlarni **fayl ichida saqlab**, bir nechta marta, hatto avtomatik bajarish uchun **skript** yozaylik.

## 5.1. Birinchi skript: Hello World

Eng oddiysidan boshlaymiz:

```bash
# hello.sh
#!/usr/bin/env bash

echo "Salom, dunyo!"
echo "Bugun: $(date)"
echo "Foydalanuvchi: $USER"
```

Skriptni bajarish uchun ikki qadam:

```bash
# 1. Ijro etish huquqini berish
chmod +x hello.sh

# 2. Ishga tushirish
./hello.sh
```

Natija:

```text
Salom, dunyo!
Bugun: Sat May 16 14:22:01 +05 2026
Foydalanuvchi: mac
```

::: tip Boshqa usul
`chmod +x` qilmasdan ham ishga tushirish mumkin:

```bash
bash hello.sh
```
:::

## 5.2. Shebang `#!` nima?

Skriptning birinchi qatori:

```bash
#!/usr/bin/env bash
```

Bu — **shebang** (yoki **hashbang**). U operatsion tizimga: *"bu skriptni qaysi interpretator yordamida bajarish kerakligini"* aytadi.

Mashhur variantlar:

| Shebang                  | Maqsadi                              |
|--------------------------|--------------------------------------|
| `#!/bin/bash`            | Aniq bash yo'li                      |
| `#!/usr/bin/env bash`    | **Tavsiya:** PATH'dan topadi         |
| `#!/bin/sh`              | POSIX shell                          |
| `#!/usr/bin/env python3` | Python3 skripti                      |

::: tip Nima uchun `env bash`?
`/bin/bash` har tizimda bo'lmaydi (masalan, macOS'da bash 3.x, foydalanuvchining brew bash'i `/usr/local/bin/bash` da bo'lishi mumkin). `env` orqali tizim bash'ning **birinchi mavjud** versiyasini ishga tushiradi.
:::

## 5.3. O'zgaruvchilar (Variables)

### Yaratish va o'qish

```bash
name="Ali"
age=25
greeting="Salom, $name!"

echo "$name"            # Ali
echo "$age"             # 25
echo "$greeting"        # Salom, Ali!
echo "${name}ning yoshi: ${age}"  # Aliing yoshi: 25
```

::: danger Probelga ehtiyot bo'ling!
```bash
name = "Ali"     # ❌ XATO — bash uni buyruq deb tushunadi
name="Ali"       # ✅ to'g'ri
```
:::

### Qo'shtirnoq turlari — juda muhim!

```bash
name="Ali"

echo "Salom, $name"      # ✅ Qo'sh tirnoq: $o'zgaruvchi interpolatsiya qilinadi
echo 'Salom, $name'      # ❌ Yagona tirnoq: $name shu holicha
echo "Bugun $(date)"     # ✅ Buyruq natijasi qo'shiladi
```

| Tirnoq turi | Interpolatsiya         | Maxsus belgilar      |
|-------------|------------------------|----------------------|
| `"..."`     | Bor (`$var`, `\``)     | `\n`, `\t` ishlaydi  |
| `'...'`     | Yo'q (literal)         | Hech narsa           |
| Tirnoqsiz   | Bor, lekin probel xavfli | xavfli             |

::: warning Har doim qo'shtirnoq qo'ying!
```bash
file="my file.txt"
rm $file       # ❌ rm "my" "file.txt" — ikki fayl!
rm "$file"     # ✅ to'g'ri
```
:::

### Command substitution

Buyruq natijasini o'zgaruvchiga yozish:

```bash
today=$(date +%Y-%m-%d)
file_count=$(ls | wc -l)
home_size=$(du -sh ~ | cut -f1)

echo "$today da $file_count ta fayl bor"
echo "Home katalogim hajmi: $home_size"
```

::: info `$(...)` vs backtick
Eski sintaksis `` `command` `` ham ishlaydi, lekin `$(...)` **tavsiya etiladi** — uni ichma-ich joylashtirish oson.
:::

### Maxsus o'zgaruvchilar

| O'zgaruvchi | Mazmuni                                |
|-------------|----------------------------------------|
| `$0`        | Skript nomi                            |
| `$1`, `$2`  | Birinchi, ikkinchi argument            |
| `$#`        | Argumentlar soni                       |
| `$@`        | Barcha argumentlar (alohida)           |
| `$*`        | Barcha argumentlar (bitta string)      |
| `$?`        | Oxirgi buyruq exit kodi (0 = muvaffaq) |
| `$$`        | Joriy skript PID                       |
| `$HOME`     | Foydalanuvchi katalogi                 |
| `$PATH`     | Bajariluvchi fayllar yo'llari          |
| `$USER`     | Foydalanuvchi nomi                     |
| `$PWD`      | Joriy katalog                          |

## 5.4. Argumentlar qabul qilish

```bash
#!/usr/bin/env bash
# greet.sh

echo "Skript nomi: $0"
echo "Birinchi argument: $1"
echo "Ikkinchi argument: $2"
echo "Jami argumentlar: $#"
echo "Barchasi: $@"
```

Ishga tushirish:

```bash
./greet.sh Ali Vali
```

Natija:

```text
Skript nomi: ./greet.sh
Birinchi argument: Ali
Ikkinchi argument: Vali
Jami argumentlar: 2
Barchasi: Ali Vali
```

## 5.5. Foydalanuvchidan input olish

```bash
#!/usr/bin/env bash

read -p "Ismingizni kiriting: " name
echo "Salom, $name!"

# Parolni yashirib kiritish
read -sp "Parol: " password
echo
echo "Parol uzunligi: ${#password}"
```

## 5.6. Shartli operatorlar: `if / else`

### Asosiy sintaksis

```bash
if [[ shart ]]; then
    # bajariladigan kod
elif [[ boshqa_shart ]]; then
    # ...
else
    # default
fi
```

### Misol — kirgan sonni tekshirish

```bash
#!/usr/bin/env bash

read -p "Son kiriting: " num

if [[ $num -gt 0 ]]; then
    echo "Musbat son"
elif [[ $num -lt 0 ]]; then
    echo "Manfiy son"
else
    echo "Nol"
fi
```

### Sonli taqqoslash operatorlari

| Operator | Ma'nosi             |
|----------|---------------------|
| `-eq`    | teng (`==`)         |
| `-ne`    | teng emas (`!=`)    |
| `-gt`    | katta (`>`)         |
| `-lt`    | kichik (`<`)        |
| `-ge`    | katta yoki teng     |
| `-le`    | kichik yoki teng    |

### String taqqoslash

| Operator     | Ma'nosi                        |
|--------------|--------------------------------|
| `=` yoki `==`| teng                           |
| `!=`         | teng emas                      |
| `-z "$s"`    | string bo'shmi                 |
| `-n "$s"`    | string bo'sh emasmi            |
| `<` `>`      | alifbo bo'yicha (lex)          |

### Fayl tekshirish operatorlari

| Operator   | Ma'nosi                          |
|------------|----------------------------------|
| `-e fayl`  | mavjudmi                         |
| `-f fayl`  | oddiy fayl                       |
| `-d fayl`  | katalog                          |
| `-r fayl`  | o'qish ruxsati bormi             |
| `-w fayl`  | yozish ruxsati bormi             |
| `-x fayl`  | ijro etish ruxsati bormi         |
| `-s fayl`  | hajmi 0 dan katta                |

### Real misol — fayl tekshiruvi

```bash
#!/usr/bin/env bash

file="$1"

if [[ -z "$file" ]]; then
    echo "Foydalanish: $0 <fayl_nomi>"
    exit 1
fi

if [[ ! -e "$file" ]]; then
    echo "❌ Fayl topilmadi: $file"
    exit 1
fi

if [[ -d "$file" ]]; then
    echo "📁 Bu katalog"
elif [[ -f "$file" ]]; then
    echo "📄 Bu oddiy fayl"
    echo "   Hajmi: $(du -h "$file" | cut -f1)"
fi
```

### `[[ ]]` vs `[ ]` vs `(( ))`

```bash
[[ "$name" == "Ali" ]]         # bash kengaytmasi — TAVSIYA
[ "$name" = "Ali" ]            # eski POSIX uslub
(( $age > 18 ))                # arifmetik uchun
```

::: tip Har doim `[[ ]]` ishlating
- Probelga toqat qiladi
- Regex `=~` ni qo'llaydi
- `&&`, `||` ichida ishlatish oson
:::

## 5.7. Looplar (sikllar)

### `for` loop

```bash
# Sonlar bo'yicha
for i in 1 2 3 4 5; do
    echo "Raqam: $i"
done

# Diapazon (brace expansion)
for i in {1..10}; do
    echo "$i"
done

# 0..20 ikkilik qadam bilan
for i in {0..20..2}; do
    echo "$i"
done

# C-style
for ((i=0; i<5; i++)); do
    echo "$i"
done

# Fayllar bo'yicha
for file in *.txt; do
    echo "Topildi: $file"
done

# Buyruq natijasi bo'yicha
for user in $(cut -d':' -f1 /etc/passwd); do
    echo "User: $user"
done
```

### `while` loop

```bash
# Hisoblagich
count=1
while [[ $count -le 5 ]]; do
    echo "Qadam: $count"
    ((count++))
done

# Fayldan qator-qator o'qish
while IFS= read -r line; do
    echo "Qator: $line"
done < /etc/passwd

# Cheksiz loop
while true; do
    echo "Davom etmoqda..."
    sleep 1
done
```

### `until` loop

```bash
# Shart YOLG'ON bo'lguncha ishlaydi
n=1
until [[ $n -gt 5 ]]; do
    echo "n=$n"
    ((n++))
done
```

### `break` va `continue`

```bash
for i in {1..10}; do
    if [[ $i -eq 5 ]]; then
        continue       # bu iteratsiyani o'tkazib yuborish
    fi
    if [[ $i -eq 8 ]]; then
        break          # loopni tark etish
    fi
    echo "$i"
done
# 1 2 3 4 6 7
```

## 5.8. `case` — ko'p tarmoqli shart

```bash
#!/usr/bin/env bash

read -p "Tugmani bosing (a/b/c): " key

case "$key" in
    a|A)
        echo "Birinchi tanlov"
        ;;
    b|B)
        echo "Ikkinchi tanlov"
        ;;
    c|C)
        echo "Uchinchi tanlov"
        ;;
    *)
        echo "Noma'lum tugma"
        ;;
esac
```

## 5.9. To'liq amaliy misol: Backup skripti

```bash
#!/usr/bin/env bash
#
# backup.sh — oddiy backup skripti
# Foydalanish: ./backup.sh <manba_katalog> <maqsad_katalog>

src="$1"
dst="$2"

# 1. Argumentlarni tekshirish
if [[ $# -ne 2 ]]; then
    echo "Foydalanish: $0 <manba> <maqsad>"
    exit 1
fi

# 2. Manba katalog bormi?
if [[ ! -d "$src" ]]; then
    echo "❌ Manba katalog topilmadi: $src"
    exit 1
fi

# 3. Maqsad katalog yo'q bo'lsa yaratamiz
if [[ ! -d "$dst" ]]; then
    mkdir -p "$dst"
    echo "📁 Yaratildi: $dst"
fi

# 4. Arxiv nomi
timestamp=$(date +%Y%m%d_%H%M%S)
archive="$dst/backup_$timestamp.tar.gz"

# 5. Arxivlash
echo "📦 Arxivlanmoqda..."
if tar -czf "$archive" -C "$(dirname "$src")" "$(basename "$src")"; then
    size=$(du -h "$archive" | cut -f1)
    echo "✅ Tayyor: $archive ($size)"
else
    echo "❌ Arxivlashda xatolik"
    exit 1
fi

# 6. Eski backuplarni tozalash (7 kundan oldingi)
echo "🧹 Eski backuplar tozalanmoqda..."
find "$dst" -name "backup_*.tar.gz" -mtime +7 -delete

echo "🎉 Backup yakunlandi"
```

Ishga tushirish:

```bash
chmod +x backup.sh
./backup.sh ~/Documents ~/Backups
```

## 5.10. Exit kodlari

Har bir buyruq tugagandan keyin **exit code** qoldiradi:

- `0` — muvaffaqiyat
- `1-255` — turli xatoliklar

```bash
ls /home
echo $?         # 0

ls /yoq-katalog
echo $?         # 2

# Skriptdan exit code qaytarish
exit 0          # muvaffaqiyat
exit 1          # umumiy xato
```

## 5.11. Tez-tez uchraydigan xatolar

::: danger Boshlovchilarning klassik xatolari

1. **`name = "Ali"`** — probelsiz: `name="Ali"`
2. **Tirnoqsiz o'zgaruvchi:** `rm $file` → `rm "$file"`
3. **`if [ $x == "y" ]`** — `[[ ... ]]` ishlating, ikkita `=` mumkin bo'lsa-da `==` mos.
4. **Shebang unutilgan.** Skript boshida `#!/usr/bin/env bash` bo'lsin.
5. **`chmod +x` qilmagan.** Yoki `bash script.sh` orqali ishga tushiring.
6. **`exit 0` yo'q.** Yaxshi skript har doim aniq exit code qaytaradi.
:::

## 5.12. Mashqlar

::: tip 🧪 Bashlings — interaktiv mashqlar
Bu bobning **6 ta** mashqi `bashlings` CLI orqali avto-tekshiruv bilan:

```bash
bashlings watch              # birinchi pending mashqdan boshlash
bashlings run script1        # bitta mashqni tekshirish
bashlings hint script1       # bosqichli maslahat
```

Manba: [`exercises/05_scripting/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/05_scripting)
:::

Quyidagi qo'shimcha vazifalarni terminalda qo'l bilan bajaring:

1. `args.sh` skripti yozing — barcha argumentlarni teskari tartibda chiqarsin.
2. `agedif.sh` — foydalanuvchidan yoshini so'rab, "Bola/Yoshlar/Katta" deb tasnif qilsin.
3. `count.sh` — joriy katalogdagi fayl va katalog sonini alohida-alohida chiqarsin.
4. `loop.sh` — 1 dan 10 gacha sonlarning kvadratini chiqarsin.
5. `safe-rm.sh` — fayl mavjudligini tekshirib, o'chirishdan oldin tasdiq so'raydigan skript.

## 5.13. Xulosa

Siz bu bobda:

- Shebang va birinchi skriptingizni yozdingiz
- O'zgaruvchilar va qo'shtirnoq farqlarini bildingiz
- Argumentlar va inputlar bilan ishlashni o'rgandingiz
- `if/elif/else`, `case`, `for`, `while`, `until` konstruksiyalarini ko'rib chiqdingiz
- Birinchi real loyiha — **backup.sh** — yozdingiz

🎉 **1-qism yakunlandi!** Siz endi Linux & Bash asoslarini chuqur o'zlashtirdingiz.

Keyingi 2-qismda biz **funksiyalar, massivlar, `sed`/`awk`, signallar va robust skriptlar** mavzulariga o'tamiz.

> **Keyingi sahifa:** [2-qism: Funksiyalar va modullik →](../part2/01-functions)
