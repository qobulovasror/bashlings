---
title: "Matnlar bilan ishlash asoslari"
description: "cat, less, head, tail, grep, wc orqali matnli fayllarni o'qish, qidirish va sanash."
---

# 4. Matnlar bilan ishlash asoslari

Unix dunyosida **hamma narsa matn**: konfiguratsiya fayllari, log'lar, kod, hatto tarmoq trafigi ham. Shuning uchun matnni samarali o'qish, qidirish va o'lchash — har bir muhandisning asosiy ko'nikmasi.

Bu bobda eng ko'p ishlatiladigan **olti** buyruqni o'rganamiz: `cat`, `less`, `head`, `tail`, `grep`, `wc`.

## 4.1. `cat` — fayl mazmunini chiqarish

`cat` (concatenate) — fayl mazmunini `stdout`ga chiqaradi.

```bash
cat hello.txt                # mazmun
cat -n hello.txt             # qator raqamlari bilan
cat -A hello.txt             # ko'rinmas belgilarni ko'rsatadi ($ end-of-line)
cat -s file.txt              # ketma-ket bo'sh qatorlarni 1 ga siqadi

# Bir nechta fayllarni birlashtirib chiqarish
cat header.txt body.txt footer.txt

# Faylga birlashtirib yozish
cat part1.txt part2.txt > full.txt
```

::: warning `cat` antipattern
Boshlovchilar ko'p qiladigan xato:

```bash
cat file.txt | grep "foo"     # ❌ keraksiz cat
grep "foo" file.txt           # ✅ to'g'ri
```

Bu "UUOC" — *Useless Use of Cat* deb ataladi.
:::

## 4.2. `less` — interaktiv ko'rish

Katta fayllarni `cat` qilsangiz, ekrandan o'tib ketadi. `less` — sahifama-sahifa o'qish uchun:

```bash
less /var/log/system.log
less -N file.txt        # qator raqamlari bilan
less +F access.log      # follow rejimi (yangi qatorlarni real time)
```

Ichida tezkor buyruqlar:

| Tugma          | Vazifasi                        |
|----------------|---------------------------------|
| `↑` `↓` yoki `j` `k` | Yuqori / Past                |
| `Space` / `b`  | Sahifa pastga / yuqoriga        |
| `g` / `G`      | Boshiga / Oxiriga               |
| `/so'z`        | Qidirish                        |
| `n` / `N`      | Keyingi / Oldingi natija        |
| `q`            | Chiqish                         |
| `h`            | Yordam                          |

::: tip `less` vs `more`
`more` eski buyruq — faqat pastga harakatlanadi. `less` — kuchliroq, ikki tomonga. "**less is more**" degan iborani aynan shu yerdan eshitgan bo'lsangiz kerak 😉
:::

## 4.3. `head` — boshini ko'rish

```bash
head file.txt          # birinchi 10 qator (default)
head -n 5 file.txt     # birinchi 5 qator
head -5 file.txt       # qisqartma shakli
head -c 100 file.txt   # birinchi 100 bayt
head *.log             # bir nechta fayl uchun
```

## 4.4. `tail` — oxirini ko'rish

```bash
tail file.txt              # oxirgi 10 qator
tail -n 20 file.txt        # oxirgi 20 qator
tail -f /var/log/nginx.log # FOLLOW — yangi qatorlar real time
tail -F file.txt           # follow + retry (fayl o'chirilsa ham)
tail -n +5 file.txt        # 5-qatordan oxirigacha
```

::: tip `tail -f` server log monitoringi uchun ideal
Server live-debug qilayotganda eng ko'p ishlatiladigan buyruq:

```bash
tail -f /var/log/app.log | grep ERROR
```
:::

### `head` + `tail` = O'rta qism

```bash
# 50-60 qatorlarni olish
sed -n '50,60p' file.txt
# yoki
head -60 file.txt | tail -11
```

## 4.5. `wc` — qator, so'z, belgi sanash

`wc` (word count):

```bash
wc file.txt
# 42   210  1573  file.txt
# qatorlar so'zlar baytlar nomi

wc -l file.txt          # faqat qatorlar
wc -w file.txt          # faqat so'zlar
wc -c file.txt          # faqat baytlar
wc -m file.txt          # belgilar (multibyte hisobga olib)
```

Real misollar:

```bash
ls | wc -l                       # katalogdagi fayllar soni
cat /etc/passwd | wc -l          # tizim foydalanuvchilari soni
grep -c "ERROR" app.log          # xatoliklar soni (-c)
find . -name "*.js" | wc -l      # loyihadagi .js fayllar soni
```

## 4.6. `grep` — matn qidirish (qudrat!)

`grep` — **Global Regular Expression Print**. Linux dunyosining eng kuchli matn-qidiruv vositasi.

```bash
grep "kalit" file.txt              # qator ichida "kalit" bo'lgan qatorlar
grep -i "error" log.txt            # case-insensitive (kichik/katta farqsiz)
grep -n "TODO" code.py             # qator raqamlari bilan
grep -c "warning" log.txt          # nechta marta uchragani (faqat son)
grep -v "INFO" log.txt             # INFO BO'LMAGAN qatorlar (invert)
grep -r "API_KEY" .                # rekursiv barcha fayllarda
grep -l "import" *.py              # FAYL NOMLARI ro'yxati (kontentsiz)
grep -w "cat" file.txt             # FAQAT to'liq so'z (cathedral'ni topmaydi)
grep -A 3 "ERROR" log.txt          # ERROR'dan keyin 3 qator
grep -B 2 "ERROR" log.txt          # ERROR'dan oldin 2 qator
grep -C 2 "ERROR" log.txt          # ERROR atrofidan 2 qator
```

### Regular Expression bilan

```bash
grep "^Salom" file.txt        # SALOM bilan boshlanadigan qatorlar
grep "tugadi$" file.txt       # tugadi bilan tugaydigan qatorlar
grep "[0-9]\{3\}" log.txt     # 3 ta raqamli pattern
grep -E "(error|fail)" log    # extended regex: error YOKI fail
```

::: tip `egrep` va `fgrep`
- `grep -E` = `egrep` — kengaytirilgan regex
- `grep -F` = `fgrep` — qattiq matn (regex'siz, tezroq)
:::

### Real misollar

```bash
# Apache log'da 404 xatoliklar
grep " 404 " /var/log/apache/access.log

# Kodda TODO va FIXME
grep -rn -E "(TODO|FIXME|XXX)" src/

# IP manzillarni topish
grep -E -o "([0-9]{1,3}\.){3}[0-9]{1,3}" access.log

# Email addresslarni topish
grep -E -o "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}" contacts.txt
```

## 4.7. Qo'shimcha foydali buyruqlar

### `sort` — tartiblash

```bash
sort file.txt              # alifbo bo'yicha
sort -r file.txt           # teskari
sort -n numbers.txt        # son bo'yicha
sort -u file.txt           # unique (takrorlarni olib tashlaydi)
sort -k 2 file.txt         # 2-ustun bo'yicha
sort -t',' -k 3 csv.txt    # CSV'da 3-ustun bo'yicha
```

### `uniq` — takrorlarni boshqarish

```bash
sort file.txt | uniq                # takrorlanmaganlar
sort file.txt | uniq -c             # har biri necha marta uchragani
sort file.txt | uniq -d             # FAQAT takrorlanganlar
```

::: warning `uniq` faqat ketma-ket dublikatlarni topadi
Shuning uchun har doim `sort` bilan kombinatsiyada ishlatiladi.
:::

### `cut` — ustunlarni ajratish

```bash
cut -d':' -f1 /etc/passwd            # birinchi ustun (foydalanuvchi nomi)
cut -d',' -f1,3 data.csv             # 1 va 3-ustunlar
cut -c1-10 file.txt                  # har qatorning 1-10 belgilari
```

### `tr` — belgilarni o'zgartirish

```bash
echo "salom" | tr 'a-z' 'A-Z'        # SALOM
echo "a,b,c,d" | tr ',' '\n'         # vergulni newline'ga
cat file.txt | tr -d ' '             # bo'shliqlarni o'chirish
cat file.txt | tr -s ' '             # ketma-ket bo'shliqlarni 1 ga siqish
```

## 4.8. Real-world misollar

### Top 10 IP — eng ko'p so'rov yuborgan

```bash
awk '{print $1}' access.log \
  | sort \
  | uniq -c \
  | sort -rn \
  | head -10
```

### Disk hajmini eng ko'p egallagan kataloglar

```bash
du -h /var | sort -rh | head -20
```

### Log fayldagi xatolik statistikasi

```bash
grep -E "ERROR|WARN|FATAL" app.log \
  | awk '{print $4}' \
  | sort \
  | uniq -c \
  | sort -rn
```

### Loyihada satrlar sonini hisoblash (kodlar)

```bash
find . -name "*.py" -exec wc -l {} + | sort -n
```

### Foydalanuvchilar ro'yxati va shell'lari

```bash
cut -d':' -f1,7 /etc/passwd
```

## 4.9. Tez-tez uchraydigan xatolar

::: danger Eslatma

1. **`cat | grep` — keraksiz.** `grep "x" file` to'g'ri.
2. **`grep` regex'da maxsus belgilar.** `.` `*` `?` belgilarini escape qiling: `grep "\.com"`.
3. **`sort | uniq` tartibi muhim.** `uniq` o'zi sort qilmaydi.
4. **`tail -f` log rotation'ni sezmaydi.** `tail -F` (katta F) ishlating.
5. **Katta fayllarni `cat`'lamang.** `less` ishlating.
:::

## 4.10. Mashqlar

::: tip 🧪 Bashlings — interaktiv mashqlar
Bu bobning **5 ta** mashqi `bashlings` CLI orqali avto-tekshiruv bilan:

```bash
bashlings watch              # birinchi pending mashqdan boshlash
bashlings run text1          # bitta mashqni tekshirish
bashlings hint text1         # bosqichli maslahat
```

Manba: [`exercises/04_text/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/04_text)
:::

Quyidagi qo'shimcha vazifalarni terminalda qo'l bilan bajaring:

1. `/etc/passwd` faylida nechta foydalanuvchi `/bin/bash` ishlatishini toping.
2. Joriy katalogdagi eng katta 5 ta faylni hajmi bo'yicha tartiblang.
3. `dmesg` natijasidan barcha `error` so'zli qatorlarni (case-insensitive) chiqaring va sanang.
4. `~/.bash_history` faylidagi eng ko'p ishlatgan 10 ta buyruqni toping.
5. `ls -la` natijasidan faqat kataloglarni (`d` bilan boshlanadigan) chiqaruvchi pipeline yozing.

## 4.11. Xulosa

| Buyruq    | Vazifasi                                    |
|-----------|---------------------------------------------|
| `cat`     | Faylni to'liq chiqarish, birlashtirish      |
| `less`    | Interaktiv ko'rish (katta fayllar uchun)    |
| `head`    | Birinchi N qator                            |
| `tail`    | Oxirgi N qator, `-f` bilan follow           |
| `grep`    | Matn qidirish, regex bilan                  |
| `wc`      | Qator, so'z, belgi sanash                   |
| `sort`    | Tartiblash                                  |
| `uniq`    | Takrorlarni boshqarish                      |
| `cut`     | Ustun ajratish                              |
| `tr`      | Belgilarni almashtirish/o'chirish           |

Endi siz har qanday matnli ma'lumotni o'qib, qidirib, hisoblay olasiz. Bu — DevOps va sistema administratorining kundalik hayotidagi 80% ishi.

Keyingi bobda biz **birinchi Bash skript**imizni yozamiz!

> **Keyingi sahifa:** [5. Birinchi Bash skript →](./05-basic-scripting)
