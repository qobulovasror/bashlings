---
title: "sed, awk va grep mahorat"
description: "sed — stream editor, awk — field-based processor. Substitution, addresses, in-place edit, BEGIN/END, arrays, real production misollar."
---

# 3. sed, awk va grep mahorat

> **🎯 Bu bobda nimani o'rganasiz:**
> - **`sed`** — stream editor: substitution, address ranges, in-place edit, backreferences
> - **`awk`** — field-based processor: `$1`/`$NF`, `BEGIN`/`END`, arrays, counter pattern
> - **Real misollar:** Apache log statistikasi, CSV transform, konfiguratsiya parsing
> - **macOS BSD sed** vs **GNU sed** farqlari (`-i` gotcha)
>
> **⏱ Vaqt:** ~40 daqiqa
> **🧪 Mashqlar:** `bashlings watch 08_text_advanced` (kelajak sprint)

---

## 3.1. Uchta tayanch — kim nima qiladi?

Unix dunyosida matn bilan ishlash uchun **uchta klassik vosita** bor. Har biri o'z roliga ega:

| Vosita | Vazifasi                                       | Misol so'rov                                   |
|--------|------------------------------------------------|------------------------------------------------|
| `grep` | **Topish** — qatorlarni filterlash             | "log'da nechta ERROR bor?"                     |
| `sed`  | **O'zgartirish** — stream'da almashtirish/o'chirish | "config faylida 'localhost' ni 'prod' ga almashtir" |
| `awk`  | **Hisoblash** — fieldlar va arifmetika         | "CSV'ning 3-ustun yig'indisi qancha?"          |

::: tip Esda saqlang
**`grep` — find. `sed` — edit. `awk` — compute.**

Aksariyat real vazifalarda **uchchalasini birga** pipe orqali ishlatamiz.
:::

`grep` Part 1/04 da chuqur ko'rib chiqilgan. Bu bobda **`sed`** va **`awk`** ga e'tibor beramiz — chunki ular DevOps va SRE ish kunining 60-70% ini tashkil qiladi.

---

## 3.2. `sed` asoslari — substitution

`sed` — **S**tream **ED**itor. Standart kirishdan (yoki fayldan) o'qiydi, har qatorga belgilangan **buyruqni qo'llaydi**, natijani stdout'ga chiqaradi.

### Eng asosiy operatsiya — `s/PATTERN/REPLACEMENT/`

```bash
echo "salom dunyo" | sed 's/dunyo/bash/'
# salom bash
```

Format: `s/<qidirilayotgan>/<almashtiriladigan>/[flags]`

### `g` flag — barcha moslashlarni almashtirish

Default'da `sed` har qatorda **faqat birinchi** moslashni almashtiradi:

```bash
echo "ola bola ola" | sed 's/ola/X/'
# X bola ola      ← faqat birinchi

echo "ola bola ola" | sed 's/ola/X/g'
# X bola X        ← barcha
```

### Fayl bilan ishlash

```bash
# Faylni stdin sifatida
sed 's/old/new/g' file.txt

# Yoki argument sifatida (xuddi shu)
sed 's/old/new/g' < file.txt
```

Natija stdout'ga chiqadi — **fayl o'zi o'zgarmaydi** (bu in-place edit emas).

### Boshqa foydali flaglar

| Flag  | Mazmuni                                  |
|-------|------------------------------------------|
| `g`   | global — har qatordagi barcha moslashlar |
| `i`   | case-insensitive (GNU sed)               |
| `2`   | faqat 2-moslashni almashtirish           |
| `p`   | almashtirilgan qatorni qo'shimcha bosib chiqarish (`-n` bilan) |

```bash
# 2-ola ni X qilish, qolganini qoldirish
echo "ola bola ola" | sed 's/ola/X/2'
# ola bola X
```

### Ajratuvchi belgini almashtirish

Patternda `/` bo'lsa, escape qilishdan ko'ra **boshqa belgi** ishlating:

```bash
# Klassik usul — escape qilish
echo "/usr/local/bin" | sed 's/\/usr\//\/opt\//'
# /opt/local/bin    ← o'qish qiyin

# Yaxshiroq — `|` yoki `,` ni ajratuvchi qiling
echo "/usr/local/bin" | sed 's|/usr/|/opt/|'
# /opt/local/bin    ← toza
```

---

## 3.3. Address — qaysi qatorga qo'llash

Default'da `sed` har qatorga buyruqni qo'llaydi. **Address** bilan cheklash mumkin.

### Qator raqami bo'yicha

```bash
# Faqat 3-qatorda almashtirish
sed '3 s/foo/bar/' file.txt

# Diapazon: 5'dan 10'gacha
sed '5,10 s/foo/bar/' file.txt

# Oxirgi qator — $
sed '$ s/foo/bar/' file.txt
```

### Regex bo'yicha

```bash
# Faqat "ERROR" so'zi bor qatorlarda
sed '/ERROR/ s/timeout/TIMEOUT/' log.txt

# "begin" va "end" oraliqdagi qatorlar
sed '/begin/,/end/ s/foo/bar/' file.txt
```

### Negation — `!`

```bash
# 5-qatordan tashqari hamma joyda
sed '5! s/foo/bar/' file.txt

# Bo'sh bo'lmagan qatorlarda
sed '/^$/! s/foo/bar/' file.txt
```

---

## 3.4. Boshqa sed buyruqlari

Substitution faqat boshlanishi. `sed` ko'p qirrali — quyida eng kerakliylari.

### `d` — qatorni o'chirish

```bash
# 1-qatorni o'chirish (header skip)
sed '1d' data.csv

# 5-10 qatorlarni o'chirish
sed '5,10d' file.txt

# Bo'sh qatorlarni o'chirish
sed '/^$/d' file.txt

# "DEBUG" bor qatorlarni o'chirish
sed '/DEBUG/d' log.txt
```

### `p` + `-n` — faqat ko'rsatilgan qatorlarni chiqarish

`p` — qatorni bosib chiqarish. `-n` — default chiqishni o'chiradi.

```bash
# Faqat 5-10 qatorlar (Part 1/04 da head|tail orqali edi)
sed -n '5,10p' file.txt

# Faqat "ERROR" bor qatorlar (grep ekvivalenti)
sed -n '/ERROR/p' log.txt

# Birinchi 3 qator
sed -n '1,3p' file.txt
```

### `i` va `a` — insert/append

```bash
# 3-qatordan oldin matn qo'shish
sed '3i\
yangi qator
' file.txt

# 3-qatordan keyin matn qo'shish
sed '3a\
qoshilgan qator
' file.txt
```

::: warning macOS BSD vs GNU sed
GNU sed (Linux) `i\` va `a\` ni inline qo'llab-quvvatlaydi:
```bash
sed '3i\new line' file   # GNU OK, BSD XATO
```

macOS'da har doim **yangi qator** kerak (yuqoridagi misol kabi).
:::

### Multiple buyruq — `-e` yoki `;`

```bash
# Ham almashtirish, ham o'chirish
sed -e 's/foo/bar/g' -e '/DEBUG/d' file.txt

# Yoki semicolon bilan
sed 's/foo/bar/g; /DEBUG/d' file.txt
```

---

## 3.5. In-place edit — `-i` flag

Hozirgacha `sed` natijasi **stdout**'ga ketdi. `-i` bilan faylni **joyida** o'zgartirish mumkin.

::: danger macOS muammosi
**Bu GNU sed va BSD sed o'rtasidagi eng katta farq.**

```bash
# GNU sed (Linux)
sed -i 's/foo/bar/g' file.txt          # ✓ ishlaydi

# BSD sed (macOS)
sed -i 's/foo/bar/g' file.txt          # ❌ xato
sed -i '' 's/foo/bar/g' file.txt       # ✓ ishlaydi (bo'sh string)
sed -i.bak 's/foo/bar/g' file.txt      # ✓ ikkalasida ham (backup yaratadi)
```

**Cross-platform xavfsiz yo'l:** har doim `-i.bak` ishlatish. Ortiqcha `file.txt.bak` yaratiladi, lekin ikkala tizimda ham ishlaydi.
:::

```bash
# Cross-platform pattern
sed -i.bak 's/old/new/g' config.txt
rm -f config.txt.bak
```

---

## 3.6. Backreferences — guruh qaytarish

Patternda `\(...\)` (yoki `-E` bilan `(...)`) — **guruh**. Replacement'da `\1`, `\2` orqali qaytariladi.

```bash
# Telefon raqamini formatlash
echo "tel 998901234567" | sed -E 's/([0-9]{3})([0-9]{2})([0-9]{7})/+\1 \2 \3/'
# tel +998 90 1234567
```

```bash
# Faylname dan ekstensiyani ajratish
echo "report.tar.gz" | sed -E 's/(.+)\.(tar\.gz)$/Nom: \1, Tip: \2/'
# Nom: report, Tip: tar.gz
```

::: tip `-E` (extended regex)
`sed -E` — Perl-like regex. `(`, `)`, `+`, `?`, `|` to'g'ridan-to'g'ri ishlatish mumkin (escape kerak emas).

GNU'da `-r` ham xuddi shunday. macOS'da `-E` ishlatish ham GNU'da, ham BSD'da portable.
:::

---

## 3.7. `awk` asoslari — field-based processor

`awk` — bu **mini-til**. U inputni avtomatik **field**larga ajratadi (default'da probelda) va sizga `$1`, `$2`, ... orqali murojaat imkonini beradi.

### Eng oddiy misol

```bash
echo "Ali 25 Toshkent" | awk '{ print $1 }'
# Ali

echo "Ali 25 Toshkent" | awk '{ print $2 }'
# 25

echo "Ali 25 Toshkent" | awk '{ print $1, $3 }'
# Ali Toshkent
```

`$0` — butun qator.

### Fayl bilan

```bash
# /etc/passwd ning birinchi ustuni (foydalanuvchi nomlari)
awk -F':' '{ print $1 }' /etc/passwd
```

`-F':'` — input field separator (default probel).

### Built-in o'zgaruvchilar

| O'zgaruvchi | Mazmuni                                |
|-------------|----------------------------------------|
| `$0`        | Butun qator                            |
| `$1`, `$2`, ... | N-field                             |
| `$NF`       | **Oxirgi field** (NF = Number of Fields)|
| `NF`        | Joriy qatordagi field soni             |
| `NR`        | Joriy qator raqami                     |
| `FS`        | Input field separator                  |
| `OFS`       | Output field separator (default probel)|
| `FILENAME`  | Joriy fayl nomi                        |

```bash
# Qator raqamlari bilan
awk '{ print NR, $0 }' file.txt

# Faqat oxirgi ustun
awk '{ print $NF }' file.txt

# Field soni va birinchi field
awk '{ print NF, $1 }' file.txt
```

---

## 3.8. Pattern + action sintaksisi

`awk` syntax: `pattern { action }`. Ikkalasi ham ixtiyoriy.

```bash
# 1. Faqat action — har qatorga
awk '{ print $1 }' file

# 2. Faqat pattern — moslashgan qatorlarni print qiladi
awk '/ERROR/' log.txt           # grep ekvivalenti

# 3. Pattern + action
awk '/ERROR/ { print $1 }' log.txt
```

### Shartli pattern

```bash
# Birinchi field "ERROR" bo'lganlar
awk '$1 == "ERROR" { print }' log.txt

# Ikkinchi ustun > 100
awk '$2 > 100 { print $1, $2 }' scores.txt

# Yoki: butun qatorda "FAIL" bor
awk '/FAIL/ { print NR, $0 }' log.txt
```

### Logical operators

```bash
# Ham ERROR, ham 500
awk '/ERROR/ && /500/' log.txt

# Yoki ikkalasidan biri
awk '/ERROR/ || /FATAL/' log.txt

# Negation
awk '!/DEBUG/' log.txt
```

---

## 3.9. `BEGIN` va `END` bloklar

`BEGIN { }` — har qanday qator o'qilishidan oldin **bir marta** ishlaydi.
`END { }` — barcha qatorlar o'qib bo'lingach **bir marta** ishlaydi.

### Klassik counter

```bash
awk '
BEGIN { count = 0 }
/ERROR/ { count++ }
END   { print "Jami xatolar:", count }
' app.log
```

### Header bilan jadval

```bash
awk '
BEGIN { print "ID\tFOYDALANUVCHI" }
{ print NR "\t" $1 }
' users.txt
```

### Yig'indi (sum)

```bash
# Birinchi ustun yig'indisi
awk '{ sum += $1 } END { print sum }' numbers.txt

# O'rtacha qiymat
awk '{ sum += $1; n++ } END { print sum/n }' numbers.txt
```

---

## 3.10. Massivlar — counter pattern

`awk` da massivlar **string-indexed** (xuddi associative array kabi).

### So'zlar chastotasi

```bash
echo "salom dunyo bash salom uz bash bash" | \
  awk '{
    for (i=1; i<=NF; i++) count[$i]++
  } END {
    for (word in count) print count[word], word
  }'
```

Natija:
```text
1 dunyo
1 uz
2 salom
3 bash
```

### Apache access log da IP statistikasi

```bash
awk '{ count[$1]++ } END {
  for (ip in count) print count[ip], ip
}' access.log | sort -rn | head -10
```

Eng ko'p so'rov yuborgan 10 ta IP.

### Kategoriya bo'yicha yig'indi

```bash
# users.txt:
#   ali     25  IT
#   vali    30  IT
#   gulnora 28  HR
#   bobur   35  IT

awk '{ sum[$3] += $2 } END {
  for (dept in sum) print dept, sum[dept]
}' users.txt
```

Natija:
```text
IT 90
HR 28
```

---

## 3.11. `printf` — formatlangan chiqish

`print` oddiy. `printf` — C tilidagi kabi formatlangan:

```bash
awk '{ printf "%-10s %5d\n", $1, $2 }' data.txt
```

`%-10s` — chap tomonga 10 belgili matn.
`%5d` — 5 belgili son (o'ng tomonga).
`\n` — yangi qator (`print` da avtomatik, `printf` da qo'shish kerak).

Misol:
```bash
echo "ali 100
bobur 99
saida 1000" | awk '{ printf "%-10s %5d\n", $1, $2 }'
# ali          100
# bobur         99
# saida       1000
```

---

## 3.12. Boshqaruv mantiq (if, for, while)

`awk` action bloki ichida to'liq dasturlash mumkin:

```bash
# if/else
awk '{
  if ($2 >= 60) print $1, "passed"
  else          print $1, "failed"
}' scores.txt

# for loop
awk '{
  for (i=1; i<=NF; i++) print i ":", $i
}' file.txt

# while
awk '{
  i = 1
  while (i <= NF) { print $i; i++ }
}' file.txt
```

---

## 3.13. Real production misollar

### Misol 1 — Apache log statistikasi

`access.log` standart formatda:
```
192.168.1.5 - - [10/Oct/2026:14:22:01] "GET /api/users HTTP/1.1" 200 1234
192.168.1.5 - - [10/Oct/2026:14:22:02] "GET /api/posts HTTP/1.1" 404 89
...
```

**Top 10 IP:**
```bash
awk '{ print $1 }' access.log | sort | uniq -c | sort -rn | head -10
```

**Status code'lar bo'yicha statistika:**
```bash
awk '{ print $9 }' access.log | sort | uniq -c | sort -rn
```

**Faqat 5xx xatolar:**
```bash
awk '$9 >= 500 { print }' access.log
```

**Eng katta response (oxirgi ustun — byte):**
```bash
awk '{ print $NF, $7 }' access.log | sort -rn | head -5
```

### Misol 2 — Disk hajmi tahlili

`du -sh ~/*` natijasini hajmga ko'ra tartiblash (sort -h ham bor, lekin awk bilan):

```bash
du -sh ~/* 2>/dev/null | sort -hr | head -10
```

Yoki awk bilan filterlash — 100MB'dan kattalar:

```bash
du -k ~/* 2>/dev/null | awk '$1 > 102400 { print $1/1024 "MB", $2 }' | sort -rn
```

### Misol 3 — CSV transform

`users.csv`:
```
ali,25,toshkent
vali,30,samarqand
gulnora,28,buxoro
```

**Faqat yoshi 27+ bo'lganlar:**
```bash
awk -F',' '$2 >= 27 { print $1, $3 }' users.csv
# vali samarqand
# gulnora buxoro
```

**JSON ko'rinishida chiqarish:**
```bash
awk -F',' '
BEGIN { print "[" }
{ printf "  {\"name\":\"%s\",\"age\":%d,\"city\":\"%s\"}%s\n",
    $1, $2, $3, (NR==total ? "" : ",")
}
END { print "]" }
' total=$(wc -l < users.csv) users.csv
```

### Misol 4 — Konfiguratsiya yangilash

```bash
# nginx.conf da port'ni almashtirish
sed -i.bak 's/listen 80;/listen 8080;/' /etc/nginx/nginx.conf

# .env faylida API_KEY ni almashtirish
sed -i.bak 's/^API_KEY=.*/API_KEY=new-secret-key/' .env

# package.json versiyasini yangilash
sed -i.bak -E 's/"version": "[0-9.]+"/"version": "2.0.0"/' package.json
```

### Misol 5 — Log fayl tahlili

```bash
# So'nggi 100 qatorda eng ko'p uchragan xatolar
tail -n 100 app.log \
  | awk '/ERROR/ { print $4 }' \
  | sort | uniq -c | sort -rn | head -5
```

---

## 3.14. `sed` vs `awk` — qachon nima?

| Vazifa                          | Tanlov         | Sabab                                |
|---------------------------------|----------------|--------------------------------------|
| Bir patternni almashtirish      | **`sed`**      | Qisqaroq, tezroq                     |
| Qator o'chirish / kesib olish    | **`sed`**      | `d`, `p`, address syntax             |
| Ustunlar bilan ishlash          | **`awk`**      | `$1`, `$2` field-based               |
| Arifmetik amallar               | **`awk`**      | `sum += $2`, `count++`               |
| Shartli mantiq                  | **`awk`**      | `if/else`, `for`, `while`            |
| Counter / aggregate             | **`awk`**      | `count[$1]++` paradigma              |
| In-place edit                   | **`sed -i`**   | awk'da `-i inplace` (GNU only)       |
| Multiline transformation        | **`awk`**      | yoki Perl, sed murakkab              |

::: tip Qoida
- **`sed`** — `s/A/B/` darajasidagi oddiy almashtirish
- **`awk`** — bir nechta field bilan ishlovchi har qanday mantiq
- **Ikkalasi murakkab bo'lsa** — `python` yoki `perl` ga o'tish vaqti
:::

---

## 3.15. Zamonaviy alternativalar

`sed` va `awk` — 1970-yillarda yaratilgan. Zamonaviy alternativalar:

| Vosita      | Vazifasi                          | O'rnatish                |
|-------------|-----------------------------------|--------------------------|
| `ripgrep` (`rg`) | grep'ning tezroq versiyasi    | `brew install ripgrep`   |
| `sd`        | sed'ga sodda alternativ           | `cargo install sd`       |
| `miller` (`mlr`) | CSV/JSON/awk uchun unified     | `brew install miller`    |
| `jq`        | JSON parser (kuchli)              | `brew install jq`        |
| `xsv`       | CSV uchun maxsus                  | `cargo install xsv`      |

Ammo: **`sed` va `awk` har tizimda mavjud**. Birinchi navbatda ularni o'rganing, keyin zamonaviy vositalarni qo'shing.

---

## 3.16. Tez-tez uchraydigan xatolar

::: danger Klassik tuzoqlar

1. **macOS `sed -i` bo'sh argumentsiz xato beradi.**
   ```bash
   sed -i 's/old/new/' file       # ❌ macOS
   sed -i '' 's/old/new/' file    # ✓ macOS
   sed -i.bak 's/old/new/' file   # ✓ ham GNU, ham BSD
   ```

2. **`/` qiyin escape — ajratuvchini almashtiring.**
   ```bash
   sed 's/\/usr\/bin\///'   # ❌ o'qib bo'lmaydi
   sed 's|/usr/bin/||'      # ✓
   ```

3. **`g` flag unutilgan.**
   ```bash
   sed 's/foo/bar/'  fayl    # faqat 1-moslashni almashtiradi
   sed 's/foo/bar/g' fayl    # barchasini
   ```

4. **`awk` da o'zgaruvchini `$` bilan ishlatish (Bash bilan adashtirish).**
   ```bash
   awk '{ x = 5; print $x }'    # $x = 5-field
   awk '{ x = 5; print x }'     # x ning qiymati = 5
   ```

5. **`-E` (extended regex) unutilgan — `(...)` ishlamaydi.**
   ```bash
   sed 's/(foo|bar)/X/g'      # ❌ literal (
   sed -E 's/(foo|bar)/X/g'   # ✓
   ```

6. **`awk` separator'i o'zgartirilmagan.**
   ```bash
   awk '{ print $1 }' file.csv          # ❌ probel sifatida — buzilgan
   awk -F',' '{ print $1 }' file.csv    # ✓
   ```

7. **`NR` va `NF` ni adashtirish.**
   - `NR` = qator raqami
   - `NF` = field soni
:::

---

## 3.17. Mashqlar

> 🧪 Kelajakda `bashlings watch 08_text_advanced` paketida.

1. **Email maskala** — `ali@example.com` ni `a***@example.com` qiling. `sed` regex bilan.

2. **Bo'sh qatorlarni tozalash** — fayldan barcha bo'sh va faqat probelli qatorlarni o'chiruvchi `sed` zanjiri yozing.

3. **Top 5 word** — kirgan matn faylidan eng ko'p uchragan 5 ta so'zni va ularning sonini chiqarsin (`awk` + `sort`).

4. **CSV averager** — `awk` skripti yozing: CSV faylning 3-ustun (sonli) o'rtacha qiymatini hisoblasin.

5. **Versiyani yangilash** — `package.json` ichidagi `"version": "X.Y.Z"` ni yangi `"version": "1.2.3"` ga o'zgartiruvchi cross-platform skript (macOS + Linux).

---

## 3.18. Xulosa

| Tushuncha                  | Asosiy nuqta                                  |
|----------------------------|------------------------------------------------|
| `sed 's/A/B/'`             | Birinchi `A` ni `B` ga almashtiradi            |
| `sed 's/A/B/g'`            | **Barcha** moslashlar                          |
| `sed '5,10d' fayl`         | 5-10 qatorlarni o'chiradi                      |
| `sed -n '/pattern/p'`      | Faqat moslangan qatorlar                       |
| `sed -E 's/(...)/\1/'`     | Backreference + extended regex                 |
| `sed -i.bak '...' fayl`    | In-place edit (cross-platform xavfsiz)         |
| `awk '{ print $1 }'`       | Birinchi field                                 |
| `awk -F','`                | Input separator — vergul                       |
| `awk '$2 > 10'`            | Shartli filter                                 |
| `awk 'NR==1'`              | Faqat 1-qator                                  |
| `BEGIN { } / END { }`      | Boshlanish/yakun bloklari                      |
| `count[$1]++`              | Counter pattern (associative array)            |

### 5 ta asosiy g'oya

1. **grep finds, sed edits, awk computes.** Vazifaga qarab to'g'risini tanlang.
2. **macOS BSD sed** — har doim `-i.bak` ishlating (yoki `gnu-sed` o'rnating).
3. **Counter pattern** (`count[$1]++; END { for ... }`) — DevOps'ning eng ko'p ishlatadigan idioma.
4. **`$NF` oxirgi field uchun** — log fayllarda response size, qator oxiri uchun ideal.
5. **Pipeline'da birlashtiring** — `grep | awk | sort | uniq -c | head` real workflow.

🎉 Endi sizda industrial darajadagi matn qayta ishlash ko'nikmasi bor. Keyingi bobda biz **signallar va traps** orqali robust skript yozishni o'rganamiz — `Ctrl+C` bosilgani-yu, cleanup, graceful shutdown.

> **Keyingi sahifa:** [4. Signallar va traps →](./04-traps-signals)
