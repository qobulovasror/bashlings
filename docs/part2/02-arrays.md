---
title: "Massivlar va lug'atlar"
description: "Indexed va associative array'lar, iteratsiya, IFS, parsing va real CSV/TODO misollari."
---

# 2. Massivlar va lug'atlar

> **🎯 Bu bobda nimani o'rganasiz:**
> - **Indexed array** — `arr=(a b c)`, qo'shish, o'chirish, iteratsiya
> - **Associative array** (lug'at) — `declare -A` (Bash 4+)
> - `"${arr[@]}"` vs `"${arr[*]}"` — kritik farq
> - **`IFS`** orqali CSV va string parsing
> - Real misol — kichik TODO list manager
>
> **⏱ Vaqt:** ~30 daqiqa
> **🧪 Mashqlar:** `bashlings watch` — 6 ta interaktiv mashq tayyor ([`exercises/07_arrays/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/07_arrays))

---

## 2.1. Nima uchun massivlar?

Tasavvur qiling — skriptingiz uchta serverga deploy qiladi. Birinchi yondashuv:

```bash
servers="alpha beta gamma"
for s in $servers; do
    deploy "$s"
done
```

Ishlaydi. Lekin agar server nomida **probel** bo'lsa-chi? `"beta server"`? Bash uni 2 ta alohida elementga ajratib yuboradi:

```bash
servers="alpha beta server gamma"
# deploy: alpha, beta, server, gamma — 4 ta!
```

Massiv esa **rasmiy ma'lumot tuzilmasi** — har element o'z hujayrasida saqlanadi:

```bash
servers=("alpha" "beta server" "gamma")
for s in "${servers[@]}"; do
    deploy "$s"
done
# deploy: alpha, "beta server", gamma — 3 ta, to'g'ri ✓
```

::: tip Asosiy g'oya
Bash'da har qachon **bir nechta qiymat**ni saqlash kerak bo'lsa — string emas, massiv ishlating. Probel, maxsus belgilar va word-splitting tuzog'idan saqlaydi.
:::

---

## 2.2. Indexed array — e'lon qilish

Indexed array — eng oddiy turi: elementlar **butun son indeks**lar bilan saqlanadi (0'dan boshlab).

### Sintaksis variantlari

```bash
# Bo'sh massiv
arr=()

# Bir nechta element bilan
fruits=("olma" "anor" "uzum")

# Diapazon (brace expansion)
nums=({1..10})            # 1 2 3 4 5 6 7 8 9 10

# Aralash tipdagi qiymatlar bilan
mixed=("matn" 42 3.14 "yana matn")
```

### `declare -a` bilan aniqlash

```bash
declare -a cities
cities[0]="Toshkent"
cities[1]="Samarqand"
cities[2]="Buxoro"
```

`declare -a` — bash'da indexed array ekanligini **aniq belgilaydi**. Funksiya ichida `local -a` shaklida ishlatiladi.

### Indekslar 0'dan boshlanadi

```bash
fruits=("olma" "anor" "uzum")
#         ↑0      ↑1     ↑2
```

::: warning Buyruq natijasidan ehtiyot bo'ling
```bash
files=($(ls *.txt))   # ⚠ XAVFLI
```

Sababi: agar fayl nomida probel bo'lsa, `ls` chiqishini bash o'zi word-splitting qiladi. **`my file.txt`** → `"my"` va `"file.txt"` — ikki element bo'lib qoladi.

To'g'ri yo'l (Bash 4+):
```bash
mapfile -t files < <(ls *.txt)
```
:::

---

## 2.3. Elementlar bilan ishlash

### Yagona elementga kirish

```bash
fruits=("olma" "anor" "uzum")

echo "${fruits[0]}"      # olma
echo "${fruits[1]}"      # anor
echo "${fruits[-1]}"     # uzum (oxirgi — Bash 4.2+)
```

⚠ **Diqqat:** `$fruits` (`${fruits}`'siz, indekssiz) **birinchi elementni qaytaradi**, hammasini emas. Bu klassik tuzoq.

```bash
echo "$fruits"           # olma  ❌ ko'pchilik buni "hammasi" deb o'ylaydi
echo "${fruits[@]}"      # olma anor uzum  ✓
```

### Massiv uzunligi (element soni)

```bash
echo "${#fruits[@]}"     # 3
```

### Element qo'shish (append)

```bash
fruits+=("shaftoli")     # oxiriga qo'shadi
echo "${fruits[@]}"      # olma anor uzum shaftoli
```

Bir nechta elementni birga qo'shish:

```bash
fruits+=("nok" "olcha")
```

### Element o'zgartirish

```bash
fruits[1]="banan"
echo "${fruits[@]}"      # olma banan uzum shaftoli ...
```

### Element o'chirish

```bash
unset 'fruits[1]'
echo "${fruits[@]}"      # olma uzum shaftoli ...
```

::: warning `unset` "tirik holat" qoldiradi
`unset arr[1]` — elementni o'chiradi, lekin **indekslarni qayta tartiblamaydi**:

```bash
arr=(a b c d)
unset 'arr[1]'
echo "${arr[@]}"         # a c d  (b ketdi)
echo "${arr[2]}"         # c       (indeks 2 saqlanib qoldi)
echo "${arr[1]}"         # (bo'sh — chunki o'chirilgan)
```

Indekslarni jamlash uchun qayta yozish kerak:
```bash
arr=("${arr[@]}")
```
:::

### Butun massivni o'chirish

```bash
unset fruits             # massiv butunlay yo'qoladi
```

---

## 2.4. Iteratsiya

### Eng keng tarqalgan — `for ... in`

```bash
fruits=("olma" "anor" "uzum")

for f in "${fruits[@]}"; do
    echo "Meva: $f"
done
```

::: danger Tirnoqlarni unutmang!
`for f in ${fruits[@]}` — tirnoqsiz — har elementni word-split qiladi.
**Har doim** `"${fruits[@]}"` shaklida yozing.
:::

### Indekslar bilan

Element + uning pozitsiyasi kerak bo'lsa:

```bash
fruits=("olma" "anor" "uzum")

for i in "${!fruits[@]}"; do
    echo "$i: ${fruits[$i]}"
done
# 0: olma
# 1: anor
# 2: uzum
```

`${!arr[@]}` — **indekslar ro'yxati**ni qaytaradi (qiymatlar emas).

### Slicing — qismli kesish

```bash
nums=(10 20 30 40 50 60 70)

# ${arr[@]:start:count}
echo "${nums[@]:2:3}"    # 30 40 50  (2-pozitsiyadan boshlab 3 ta)
echo "${nums[@]:4}"      # 50 60 70  (4-pozitsiyadan oxirigacha)
echo "${nums[@]: -2}"    # 60 70     (oxirgi 2 ta — probel kerak!)
```

::: tip Negativ slicing
`${nums[@]:-2}` — bu **default qiymat operatori**, slicing emas!
Negativ offset uchun **probel** kerak: `${nums[@]: -2}`.
:::

### `while` orqali (kamroq uchraydi)

```bash
i=0
while [[ $i -lt ${#fruits[@]} ]]; do
    echo "${fruits[$i]}"
    ((i++))
done
```

---

## 2.5. `"${arr[@]}"` vs `"${arr[*]}"` — kritik farq

Bu bashning eng ko'p chalkashadigan jihati. Ikkalasi ham "hamma element" deb tushuniladi, **lekin tirnoq ichida butunlay boshqacha ishlaydi**.

```bash
arr=("salom dunyo" "bash" "uz")

# @ — har element alohida
for x in "${arr[@]}"; do echo "[$x]"; done
# [salom dunyo]
# [bash]
# [uz]

# * — bitta string, IFS belgisi bilan ulangan (default IFS = probel)
for x in "${arr[*]}"; do echo "[$x]"; done
# [salom dunyo bash uz]   ← bitta string!
```

| Sintaksis      | Tirnoq ichida natija                      |
|----------------|-------------------------------------------|
| `"${arr[@]}"`  | **Har element — alohida argument**        |
| `"${arr[*]}"`  | **Bitta string, IFS bilan birlashtirilgan**|
| `${arr[@]}`    | Tirnoqsiz — word-splitting xavfi          |

::: tip Qoida
**99% holatlarda** `"${arr[@]}"` ishlatiladi. `"${arr[*]}"` faqat **stringga aylantirish** kerak bo'lganda.
:::

### `IFS` bilan custom separator

`${arr[*]}` IFS belgisini ishlatadi:

```bash
arr=("a" "b" "c")

IFS=, echo "${arr[*]}"
# a,b,c

IFS='|' echo "${arr[*]}"
# a|b|c
```

Aslida bu — ro'yxatni CSV satrga aylantirish texnikasi.

---

## 2.6. Associative arrays — lug'atlar (Bash 4+)

Associative array — **string kalitlar** bilan saqlash. Boshqa tillarda buni "dictionary", "map" yoki "hashtable" deyiladi.

::: danger macOS muammosi
macOS'da default bash — **3.2** (2007-yildan). U **associative array'ni qo'llab-quvvatlamaydi**.

Hal qilish:
```bash
brew install bash
which bash               # /opt/homebrew/bin/bash
```

Skript boshida `#!/usr/bin/env bash` — `env` PATH'dan yangiroq bash ni topadi.

Versiyani tekshirish:
```bash
bash --version           # 5.x bo'lishi kerak
```
:::

### E'lon qilish va to'ldirish

```bash
declare -A user

user[name]="Ali"
user[age]=25
user[city]="Toshkent"
```

Yoki bir martda:

```bash
declare -A user=(
    [name]="Ali"
    [age]=25
    [city]="Toshkent"
)
```

### Qiymatlarga kirish

```bash
echo "${user[name]}"     # Ali
echo "${user[age]}"      # 25
```

Mavjud bo'lmagan kalit — bo'sh string qaytaradi (xato emas):

```bash
echo "${user[phone]}"    # (bo'sh)
```

### Kalitlar va qiymatlar

```bash
echo "${!user[@]}"       # name age city  (kalitlar)
echo "${user[@]}"        # Ali 25 Toshkent (qiymatlar)
echo "${#user[@]}"       # 3              (jami)
```

::: warning Tartib kafolatlanmaydi
Associative array'da kalitlar tartibi **hash table** ichki tuzilmasiga bog'liq. Iteratsiya tartibi ishonchli emas. Tartibli kerak bo'lsa, alohida indexed array'da kalitlarni saqlang.
:::

### Iteratsiya

```bash
for key in "${!user[@]}"; do
    echo "$key = ${user[$key]}"
done
# name = Ali
# age = 25
# city = Toshkent
```

### Element o'chirish

```bash
unset 'user[city]'
```

### Real misol — fayllar to'plami uchun counter

```bash
#!/usr/bin/env bash
declare -A count

# Joriy katalogdagi fayllar ekstensiyasini sanash
for f in *; do
    ext="${f##*.}"
    ((count[$ext]++))
done

for ext in "${!count[@]}"; do
    printf '%-10s %d\n' "$ext" "${count[$ext]}"
done
```

Natija:
```text
md         12
sh          8
txt         3
```

---

## 2.7. `IFS` va string parsing

**IFS** (Internal Field Separator) — bash'ning word-splitting belgisi. Default qiymati: probel + tab + newline.

### `read -ra` — stringni massivga ajratish

```bash
csv="ali,25,toshkent"

IFS=',' read -ra fields <<< "$csv"

echo "${fields[0]}"      # ali
echo "${fields[1]}"      # 25
echo "${fields[2]}"      # toshkent
```

`-r` — backslash'larni o'zgartirmaslik (har doim ishlatish kerak).
`-a fields` — natijani `fields` massiviga yozish.
`<<<` — here-string (bir qatorli stdin).

### Massivni stringga birlashtirish

```bash
words=("salom" "dunyo" "bash")

# Probel bilan
result="${words[*]}"
echo "$result"           # salom dunyo bash

# Custom separator bilan
(IFS=','; result="${words[*]}"; echo "$result")
# salom,dunyo,bash
```

::: tip Subshell bilan `IFS` ehtiyot
`(IFS=','; ...)` ni qavslarga olganimiz `IFS` ni o'zgartirish faqat shu blokda qolishi uchun. Tashqarida IFS o'zgarmaydi.
:::

### Fayldan satrlarni massivga

```bash
# Bash 4+ uchun eng ravon yo'l
mapfile -t lines < /etc/passwd

echo "${#lines[@]}"      # qator soni
echo "${lines[0]}"       # birinchi qator
```

`mapfile -t` — har qatorni alohida element qiladi, `-t` newline'ni olib tashlaydi.

Eski bash uchun:
```bash
lines=()
while IFS= read -r line; do
    lines+=("$line")
done < /etc/passwd
```

---

## 2.8. Real misol — TODO list manager

Massivlarni amalda ko'rsatuvchi to'liq misol:

```bash
#!/usr/bin/env bash
#
# todo.sh — kichik TODO list manager
#
# Foydalanish:
#   todo.sh add "Buyruq"
#   todo.sh list
#   todo.sh done 2
#

set -euo pipefail

readonly TODO_FILE="${TODO_FILE:-$HOME/.todo}"

# Faylni yuklash (massiv sifatida)
load_tasks() {
    if [[ -f "$TODO_FILE" ]]; then
        mapfile -t tasks < "$TODO_FILE"
    else
        tasks=()
    fi
}

# Massivni faylga saqlash
save_tasks() {
    printf '%s\n' "${tasks[@]}" > "$TODO_FILE"
}

cmd_add() {
    local item="$*"
    [[ -z "$item" ]] && { echo "Foydalanish: todo add <matn>" >&2; exit 1; }
    tasks+=("$item")
    save_tasks
    echo "✓ Qo'shildi: $item"
}

cmd_list() {
    if [[ ${#tasks[@]} -eq 0 ]]; then
        echo "(ro'yxat bo'sh)"
        return
    fi
    for i in "${!tasks[@]}"; do
        printf '%2d. %s\n' "$((i + 1))" "${tasks[$i]}"
    done
}

cmd_done() {
    local n="$1"
    local idx=$((n - 1))
    if [[ -z "${tasks[$idx]:-}" ]]; then
        echo "❌ #$n topilmadi" >&2
        exit 1
    fi
    echo "✓ Bajarildi: ${tasks[$idx]}"
    unset 'tasks[idx]'
    tasks=("${tasks[@]}")    # qayta indekslab jamlash
    save_tasks
}

# --- Asosiy ---
load_tasks
case "${1:-list}" in
    add)  shift; cmd_add "$@" ;;
    list) cmd_list ;;
    done) cmd_done "$2" ;;
    *) echo "Foydalanish: $0 {add|list|done}"; exit 1 ;;
esac
```

Sinab ko'ramiz:

```bash
$ todo.sh add "non sotib olish"
✓ Qo'shildi: non sotib olish

$ todo.sh add "kitobni o'qish"
✓ Qo'shildi: kitobni o'qish

$ todo.sh list
 1. non sotib olish
 2. kitobni o'qish

$ todo.sh done 1
✓ Bajarildi: non sotib olish

$ todo.sh list
 1. kitobni o'qish
```

### Bu misolda nima ishlatildi?

| Texnika                             | Qaerda                       |
|-------------------------------------|------------------------------|
| `mapfile -t` (fayl → massiv)         | `load_tasks`                 |
| `printf '%s\n' "${arr[@]}"`         | `save_tasks` (faylga yozish) |
| `tasks+=(...)` qo'shish              | `cmd_add`                    |
| `"${!arr[@]}"` indekslar             | `cmd_list`                   |
| `unset 'arr[i]'` + qayta jamlash    | `cmd_done`                   |
| `${var:-default}` bo'sh qiymat tekshirish | `cmd_list`              |

---

## 2.9. Tez-tez uchraydigan xatolar

::: danger Klassik tuzoqlar

1. **`$arr` — faqat birinchi element.**
   ```bash
   arr=(a b c)
   echo "$arr"      # a   ❌
   echo "${arr[@]}" # a b c  ✓
   ```

2. **Tirnoqsiz iteratsiya.**
   ```bash
   for x in ${arr[@]}; do ...   # ❌ word-splitting
   for x in "${arr[@]}"; do ... # ✓
   ```

3. **`$(ls)` ni massivga to'ldirish.**
   ```bash
   files=($(ls))    # ❌ probelli fayl nomlari sinadi
   mapfile -t files < <(ls)   # ✓
   ```

4. **`unset` indekslarni qaytarib bermaydi.**
   `unset 'arr[1]'` keyin `${#arr[@]}` 2 emas, hali ham `${arr[2]}` mavjud.
   Yechim: `arr=("${arr[@]}")` bilan jamlash.

5. **macOS'da associative array ishlamaydi.**
   Default bash 3.2 — `declare -A` xato beradi. `brew install bash` kerak.

6. **Negativ slicing va default qiymat aralashishi.**
   ```bash
   "${arr[@]:-2}"     # ❌ default qiymat
   "${arr[@]: -2}"    # ✓ oxirgi 2 element (probel!)
   ```

7. **Quote'siz string birlashtirish.**
   ```bash
   IFS=',' echo "${arr[*]}"     # ❌ echo IFS'ni ko'rmaydi
   (IFS=','; echo "${arr[*]}")  # ✓ subshell ichida
   ```
:::

---

## 2.10. Mashqlar

::: tip 🧪 Bashlings — interaktiv mashqlar
Bu bobning **6 ta** mashqi `bashlings` CLI orqali avto-tekshiruv bilan:

```bash
bashlings watch              # birinchi pending mashqdan boshlash
bashlings run arr1           # bitta mashqni tekshirish
bashlings hint arr1          # bosqichli maslahat
```

Manba: [`exercises/07_arrays/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/07_arrays)
:::

Quyidagi kontseptual mashqlarni esa o'zingiz qo'l bilan sinab ko'ring:

1. **`reverse_array`** — funksiya yozing, massivni teskari tartibda chiqarsin.
   Hint: `for ((i=${#arr[@]}-1; i>=0; i--))`

2. **`unique_items`** — massivdan takrorlarsiz elementlarni qaytarsin. Hint: `sort -u` bilan birlashtirish.

3. **`csv_to_dict`** — `"key1=val1,key2=val2,..."` formatli stringni associative array'ga aylantiruvchi funksiya.

4. **`top_n`** — sonli massiv va `n` argument oladi, eng katta `n` ta sonni qaytaradi. `sort -rn | head` patterni.

5. **`group_by_ext`** — joriy katalogdagi fayllarni ekstensiyasi bo'yicha guruhlasin (associative array). Misol natija:
   ```
   md: [README.md doc.md]
   sh: [build.sh test.sh]
   ```

---

## 2.11. Xulosa

| Tushuncha                       | Asosiy nuqta                                       |
|---------------------------------|----------------------------------------------------|
| **Indexed array**               | `arr=(a b c)` yoki `declare -a`                    |
| **Associative array**           | `declare -A` (Bash 4+ majburiy)                    |
| **Hammasi**                     | `"${arr[@]}"` (har element alohida)                |
| **Bitta string**                | `"${arr[*]}"` (IFS bilan birlashtirilgan)          |
| **Element soni**                | `${#arr[@]}`                                       |
| **Kalitlar/indekslar**          | `"${!arr[@]}"`                                     |
| **Qo'shish**                    | `arr+=("yangi")`                                   |
| **Slicing**                     | `"${arr[@]:start:count}"`                          |
| **Fayl → massiv**               | `mapfile -t arr < fayl.txt`                        |
| **Stringni split**              | `IFS=',' read -ra arr <<< "$s"`                    |
| **Macros tirnoq**               | Har doim `"${arr[@]}"` — tirnoqsiz xavfli         |

### Asosiy g'oyalar

1. **Massiv = bir nechta qiymat saqlash.** String — yagona qiymat.
2. **`@` har doim ishlating** — `*` faqat string birlashtirish uchun.
3. **macOS bash 3.2** — associative array ishlamaydi, brew bash kerak.
4. **`mapfile`** — fayldan massivga eng xavfsiz yo'l.
5. **`unset` qoldiq qoldiradi** — `arr=("${arr[@]}")` bilan jamlang.

🎉 Endi sizda Bash'da kuchli ma'lumot strukturalari ishlatish ko'nikmasi bor. Keyingi bobda biz **`sed`** va **`awk`** orqali matnlarni industrial darajada qayta ishlashni o'rganamiz.

> **Keyingi sahifa:** [3. sed, awk va grep mahorat →](./03-sed-awk)
