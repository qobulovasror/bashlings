---
title: "Funksiyalar va modullik"
description: "Bash funksiyalari: argumentlar, scope, return qiymatlari, kutubxonalar va best practice'lar."
---

# 1. Funksiyalar va modullik

> **🎯 Bu bobda nimani o'rganasiz:**
> - Funksiyani e'lon qilish, chaqirish va argumentlar uzatish
> - `return` va `echo` orqali qiymat qaytarish (ikkalasi farq qiladi!)
> - `local` o'zgaruvchilar va scope mantig'i
> - Qayta ishlatiluvchi kutubxonalar yozish (`source` orqali yuklash)
> - Real misol — kichik logging library
>
> **⏱ Vaqt:** ~25 daqiqa o'qish + mashqlar
> **🧪 Mashqlar:** `bashlings watch 06_functions` (kelajak sprint)

---

## 1.1. Nima uchun funksiyalar?

Tasavvur qiling — skriptingiz uch joyda **bir xil** ishni bajaradi: log yozadi, vaqt belgisini qo'yadi, qiziq rang chiqaradi. Har joyda 4 qatordan yozish — DRY (**Don't Repeat Yourself**) prinsipini buzadi.

Funksiya — bu **nomi bor kod bloki**. Bir marta yozasiz, ko'p marta chaqirasiz.

```bash
# YOMON — uch joyda takror
echo "[$(date +%T)] Backup boshlandi"
# ... ishlar ...
echo "[$(date +%T)] Database eksport qilindi"
# ... ishlar ...
echo "[$(date +%T)] Yakunlandi"

# YAXSHI — funksiya orqali
log() {
    echo "[$(date +%T)] $*"
}

log "Backup boshlandi"
log "Database eksport qilindi"
log "Yakunlandi"
```

::: tip Funksiyaning afzalliklari
- **Qisqalik** — kod 3× kichrayadi
- **Yagona haqiqat manbai** — format'ni o'zgartirish 1 joyda
- **Test qilish oson** — har funksiyani alohida sinash mumkin
- **O'qish oson** — `log "..."` o'zi maqsadni aytadi
:::

---

## 1.2. E'lon qilish sintaksisi

Bashda funksiyani **ikki shaklda** e'lon qilish mumkin:

### A. `function` kalit so'zi bilan

```bash
function salomlash() {
    echo "Salom!"
}
```

### B. POSIX shakli (qavslar bilan)

```bash
salomlash() {
    echo "Salom!"
}
```

Ikkalasi ham ishlaydi. **POSIX shakli tavsiya etiladi** — chunki:

- Boshqa POSIX shellarda ham ishlaydi (sh, dash)
- Qisqaroq
- `function` kalit so'zi keraksiz shovqin

::: warning Qavslar bo'sh — argument deyiladi, lekin ishlatilmaydi
`salomlash()` ichidagi `()` — funksiya ekanligini bildiradi. **Argumentlar bu yerda berilmaydi**, ular `$1`, `$2`, ... orqali ichkarida ishlatiladi.
:::

### Chaqirish

Funksiyani **nomi bilan** chaqirasiz, **qavslarsiz**:

```bash
salomlash           # ✅ to'g'ri
salomlash()         # ❌ xato — sintaksis buzilgan
```

---

## 1.3. Birinchi to'liq misol

```bash
#!/usr/bin/env bash

# Funksiya e'loni
salomlash() {
    echo "Salom, Bash dunyosi!"
    echo "Bugun: $(date +%F)"
}

# Asosiy mantiq
echo "=== Skript boshlandi ==="
salomlash
echo "=== Skript yakunlandi ==="
```

Natija:

```text
=== Skript boshlandi ===
Salom, Bash dunyosi!
Bugun: 2026-05-16
=== Skript yakunlandi ===
```

::: tip E'lon tartibi muhim!
Funksiyani **ishlatishdan oldin** e'lon qilish kerak. Bash skriptlarni yuqoridan pastga o'qiydi.

```bash
salomlash   # ❌ XATO — hali e'lon bo'lmagan
salomlash() { echo "Salom"; }
salomlash   # ✅ to'g'ri
```
:::

---

## 1.4. Argumentlar

Funksiyaga argumentlar **xuddi skriptga argument uzatgandek** uzatiladi — `$1`, `$2`, ... orqali.

```bash
salomlash() {
    echo "Salom, $1!"
    echo "Yoshingiz: $2"
}

salomlash "Ali" 25
# Salom, Ali!
# Yoshingiz: 25
```

### Asosiy maxsus o'zgaruvchilar

| O'zgaruvchi | Mazmuni                                                  |
|-------------|----------------------------------------------------------|
| `$1`, `$2`, …| Pozitsion argumentlar                                   |
| `$#`        | Argumentlar **soni**                                     |
| `$@`        | Barcha argumentlar (alohida — har biri o'z elementi)     |
| `$*`        | Barcha argumentlar (bitta string)                        |
| `$0`        | **Skript** nomi (funksiya nomi EMAS — diqqat!)           |
| `${FUNCNAME[0]}` | Joriy **funksiya** nomi                              |

::: warning `$@` va `$*` farqi
```bash
example() {
    for arg in "$@"; do echo "@ qoldirdi: $arg"; done
    for arg in "$*"; do echo "* qoldirdi: $arg"; done
}
example "salom dunyo" "bash"
```
Natija:
```text
@ qoldirdi: salom dunyo
@ qoldirdi: bash
* qoldirdi: salom dunyo bash   # birga qo'shdi!
```
Deyarli har doim `"$@"` ishlatiladi.
:::

### Misol: ikki son yig'indisi

```bash
yigindi() {
    echo $(($1 + $2))
}

yigindi 5 7        # 12
yigindi 100 200    # 300
```

---

## 1.5. Default qiymatlar va validatsiya

### Default qiymat — `${var:-default}`

```bash
salomlash() {
    local ism="${1:-Anonim}"
    echo "Salom, $ism!"
}

salomlash "Ali"     # Salom, Ali!
salomlash           # Salom, Anonim!
```

### Majburiy argument — `${var:?xato xabar}`

```bash
backup() {
    local src="${1:?manba katalog ko'rsatilmagan}"
    echo "Backup boshlandi: $src"
}

backup           # XATO: manba katalog ko'rsatilmagan
backup ~/docs    # Backup boshlandi: /Users/mac/docs
```

::: tip Parameter expansion to'plami
| Sintaksis             | Mazmuni                                       |
|-----------------------|-----------------------------------------------|
| `${var:-default}`     | Bo'sh bo'lsa default, lekin **o'rnatmaydi**   |
| `${var:=default}`     | Bo'sh bo'lsa default va **o'rnatadi**         |
| `${var:?xabar}`       | Bo'sh bo'lsa xato bilan to'xtatadi            |
| `${var:+almashtir}`   | Bo'sh **emas** bo'lsa almashtirilgan qiymatni qaytaradi |
:::

### Argumentlarni qo'lda validatsiya

```bash
backup() {
    if [[ $# -lt 2 ]]; then
        echo "Foydalanish: backup <manba> <maqsad>" >&2
        return 1
    fi

    local src="$1"
    local dst="$2"

    if [[ ! -d "$src" ]]; then
        echo "❌ Manba topilmadi: $src" >&2
        return 1
    fi

    echo "✅ $src → $dst"
}
```

---

## 1.6. Qaytariladigan qiymatlar — `return` vs `echo`

Bashning **eng chalkash** jihati — funksiyadan qiymat qaytarish.

### `return` — faqat exit kodi (0..255)

`return` boshqa tillardagi `return` emas. U faqat **exit code** qaytaradi: 0 (muvaffaqiyat) yoki 1..255 (xato).

```bash
fayl_bormi() {
    [[ -f "$1" ]] && return 0 || return 1
}

if fayl_bormi "/etc/passwd"; then
    echo "Mavjud"
fi
```

::: danger 256 yoki ko'p — XATO
```bash
qaytar() { return 256; }
qaytar
echo $?   # 0 — wrap around!
```
:::

### `echo` — haqiqiy qiymat qaytarish

Agar real qiymat (son, string) qaytarmoqchi bo'lsangiz — uni `echo` qiling. Chaqiruvchi `$(...)` bilan oladi.

```bash
kvadrat() {
    local n="$1"
    echo $((n * n))
}

natija=$(kvadrat 5)
echo "5 ning kvadrati: $natija"
# 5 ning kvadrati: 25
```

### Ikkalasini birga ishlatish

```bash
parse_yosh() {
    local yosh="$1"
    if [[ ! "$yosh" =~ ^[0-9]+$ ]]; then
        echo "noto'g'ri yosh" >&2
        return 1
    fi
    echo "$yosh"
    return 0
}

if y=$(parse_yosh "$1"); then
    echo "✅ Yosh: $y"
else
    echo "❌ Parsing xato bo'ldi"
fi
```

::: tip Stdout vs stderr
Funksiya **xato xabarini stderr** ga (`>&2`), **qaytariladigan qiymatni stdout**ga yozsin. Aks holda `$(...)` xato xabarini ham yutib qoladi.
:::

---

## 1.7. Scope: `local` vs global

Default holatda Bashda **barcha** o'zgaruvchilar global. Bu — xavfli.

```bash
salomlash() {
    ism="Ali"          # GLOBAL — funksiyadan tashqarida ham ko'rinadi!
    echo "Salom, $ism"
}

ism="Vali"
salomlash
echo "Tashqarida: $ism"   # "Ali" — qayta yozildi!
```

### Yechim: `local`

```bash
salomlash() {
    local ism="Ali"    # FAQAT funksiya ichida
    echo "Salom, $ism"
}

ism="Vali"
salomlash
echo "Tashqarida: $ism"   # "Vali" — saqlanib qoldi
```

::: warning Har doim `local` ishlating!
Funksiya ichidagi har bir yangi o'zgaruvchi uchun `local` qo'shing. Bu — xavfsizlik va izolyatsiya.
:::

### `local` ning nozik tomonlari

```bash
# Bir nechta local qo'shish
fn() {
    local a b c
    local x="bir" y="ikki"
}

# `local` exit code'ni yutib qo'yadi — buggi bo'lishi mumkin
fn() {
    local result=$(maybe_fail)   # ⚠️ maybe_fail xato bo'lsa ham $? = 0
}

# To'g'ri yo'l:
fn() {
    local result
    result=$(maybe_fail)         # endi $? to'g'ri
}
```

---

## 1.8. `readonly` — konstantalar

```bash
readonly MAX_RETRIES=3
readonly LOG_FILE="/var/log/app.log"

MAX_RETRIES=5   # XATO: readonly variable
```

::: tip Loyiha-darajadagi konstantalar
Skript boshida `readonly` o'zgaruvchilarni e'lon qilib qo'ying — bu **kod oqimini** belgilab beradi:

```bash
readonly SCRIPT_NAME="$(basename "$0")"
readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly LOG_FILE="${LOG_FILE:-/tmp/app.log}"
```
:::

---

## 1.9. Funksiyalar kutubxonasi — `source`

Funksiyalarni boshqa faylga ko'chirib, qayta-qayta ishlatish mumkin.

### `lib/log.sh` — qayta ishlatiluvchi modul

```bash
# lib/log.sh

log_info()  { printf '\033[32m[INFO]\033[0m  %s\n' "$*"; }
log_warn()  { printf '\033[33m[WARN]\033[0m  %s\n' "$*" >&2; }
log_error() { printf '\033[31m[ERROR]\033[0m %s\n' "$*" >&2; }

die() {
    log_error "$*"
    exit 1
}
```

### Asosiy skript: `main.sh`

```bash
#!/usr/bin/env bash
set -euo pipefail

# Modulni yuklash — ikki yo'l mavjud:
source "$(dirname "$0")/lib/log.sh"
# yoki qisqa:
. "$(dirname "$0")/lib/log.sh"

log_info "Skript ishga tushdi"
log_warn "Disk hajmi past"
log_error "Database ulanishi yo'q"
die "Kritik xato — to'xtatamiz"
```

::: tip `source` qachon `bash` dan farq qiladi?
- `bash script.sh` → **yangi subshell** ochiladi. Funksiyalar tashqariga chiqmaydi.
- `source script.sh` → **joriy shell**da ishlaydi. Funksiyalar yuklanadi.

Kutubxona uchun har doim **`source`** (yoki `.`) ishlating.
:::

### Idiomatic — modulni faqat bir marta yuklash (guard)

```bash
# lib/log.sh boshida
[[ -n "${__LOG_LOADED:-}" ]] && return 0
readonly __LOG_LOADED=1

log_info() { ... }
# ...
```

C/C++'dagi `#ifndef GUARD` ekvivalenti.

---

## 1.10. Nom konvensiyalari va best practice

| Tavsiya                                | Misol                            |
|----------------------------------------|----------------------------------|
| `snake_case` ishlating                 | `log_info`, `parse_args`         |
| Ichki funksiyalar `_` bilan boshlansin | `_private_helper`                |
| Fe'l + ot                              | `get_user`, `check_disk`         |
| Boolean — `is_*` yoki `has_*`          | `is_root`, `has_internet`        |
| Modul prefiksi                         | `db_connect`, `db_close`         |

### To'liq dokumentlangan funksiya namunasi

```bash
# Foydalanuvchi root ekanligini tekshiradi.
#
# Argumentlar:
#   yo'q
# Qaytaradi:
#   0 — root
#   1 — boshqa foydalanuvchi
# Misol:
#   if is_root; then ...
is_root() {
    [[ $EUID -eq 0 ]]
}
```

---

## 1.11. Real misol — Logging library

To'liq qayta ishlatiluvchi modul:

```bash
# lib/logger.sh
#
# Foydalanish:
#   source lib/logger.sh
#   log_info "Boshlandi"
#   log_warn "Disk past"
#   log_error "Xato"
#   LOG_LEVEL=debug log_debug "Detal"

[[ -n "${__LOGGER_LOADED:-}" ]] && return 0
readonly __LOGGER_LOADED=1

# Konfiguratsiya (foydalanuvchi override qilishi mumkin)
LOG_LEVEL="${LOG_LEVEL:-info}"
LOG_FILE="${LOG_FILE:-}"

readonly __LOG_RED='\033[31m'
readonly __LOG_YELLOW='\033[33m'
readonly __LOG_GREEN='\033[32m'
readonly __LOG_BLUE='\033[34m'
readonly __LOG_RESET='\033[0m'

# Daraja raqamlari
declare -A __LOG_LEVELS=(
    [debug]=0 [info]=1 [warn]=2 [error]=3
)

_log() {
    local level="$1"; shift
    local color="$1"; shift
    local msg="$*"

    # Daraja filtrlash
    local req="${__LOG_LEVELS[$level]:-1}"
    local cur="${__LOG_LEVELS[$LOG_LEVEL]:-1}"
    (( req < cur )) && return 0

    local ts
    ts=$(date '+%Y-%m-%d %H:%M:%S')
    local line
    line=$(printf '[%s] [%-5s] %s' "$ts" "${level^^}" "$msg")

    printf '%b%s%b\n' "$color" "$line" "$__LOG_RESET" >&2
    [[ -n "$LOG_FILE" ]] && printf '%s\n' "$line" >> "$LOG_FILE"
}

log_debug() { _log debug "$__LOG_BLUE"   "$@"; }
log_info()  { _log info  "$__LOG_GREEN"  "$@"; }
log_warn()  { _log warn  "$__LOG_YELLOW" "$@"; }
log_error() { _log error "$__LOG_RED"    "$@"; }

die() {
    log_error "$@"
    exit 1
}
```

Ishlatish:

```bash
#!/usr/bin/env bash
set -euo pipefail
source ./lib/logger.sh

LOG_FILE=/tmp/app.log
log_info "Boshlandi"
log_warn "Disk 80% to'lgan"
log_error "Database javob bermayapti"

[[ -f /etc/critical ]] || die "Kritik fayl yo'q"
```

::: tip Bu lib nima qiladi?
- 4 ta daraja: debug/info/warn/error
- `LOG_LEVEL` orqali filtrlash
- `LOG_FILE` o'rnatilgan bo'lsa — faylga ham yozadi
- Ranglar terminalga, oddiy matn faylga
- Yagona "guard" — bir necha marta source qilinmaydi
- Hammasi `local` da, scope toza
:::

---

## 1.12. Tez-tez uchraydigan xatolar

::: danger Funksiyalarda klassik tuzoqlar

1. **`local` unutilgan.**
   ```bash
   fn() { x=5; }   # x global bo'lib qoladi!
   ```

2. **`local` exit code'ni yutadi.**
   ```bash
   fn() { local r=$(may_fail); echo $?; }   # har doim 0
   # To'g'ri:
   fn() { local r; r=$(may_fail); echo $?; }
   ```

3. **`return` raqamli emas.**
   ```bash
   return "xato"      # XATO — faqat 0..255
   ```

4. **Funksiya nomi va o'zgaruvchi nomi to'qnashishi.**
   ```bash
   log=anything       # endi log nomli funksiyani ham buzasiz potentsial
   ```

5. **Funksiya hali e'lon qilinmagan vaqtda chaqirilgan.**
   ```bash
   fn          # ❌
   fn() { ...}
   ```

6. **`echo` o'rniga `printf` ishlatilmagan.**
   `echo -e` portativ emas. Format kerak bo'lsa `printf` ishlating.

7. **Stderr ga yozish unutilgan.**
   Xato xabari — stdout'da emas, `>&2`'da bo'lishi kerak.
:::

---

## 1.13. Mashqlar

> 🧪 Kelajakda quyidagilar `bashlings watch 06_functions` orqali avto-tekshiriladi. Hozircha qo'l bilan tekshiring.

1. `is_even` funksiyasini yozing — son juftmi yoki toqmi (`return 0` / `return 1`).
2. `repeat_word` funksiyasi — birinchi argumentdagi so'zni ikkinchi argumentdagi son marta chiqarsin.
3. `max3` — uchta sondan eng kattasini stdout'ga `echo` qilsin.
4. `prompt_confirm` — foydalanuvchidan "Davom etamizmi? [y/N]" so'rab, javobiga ko'ra 0 yoki 1 qaytarsin.
5. `lib/math.sh` modulini yarating: `sum`, `mul`, `pow` funksiyalari bilan. Asosiy skriptda `source` qiling va sinab ko'ring.

---

## 1.14. Xulosa

| Tushuncha               | Asosiy nuqta                                       |
|-------------------------|----------------------------------------------------|
| **E'lon qilish**        | `name() { ... }` — POSIX shakl tavsiya             |
| **Argumentlar**         | `$1`, `$@`, `$#` — har doim `"$@"`                 |
| **Default qiymat**      | `${1:-default}`                                    |
| **`return`**            | Faqat 0..255 — exit code                           |
| **`echo`**              | Haqiqiy qiymat qaytarish uchun                     |
| **`local`**             | Funksiya ichidagi har o'zgaruvchi uchun majburiy   |
| **`readonly`**          | Konstantalar uchun                                 |
| **`source` / `.`**      | Kutubxonani **joriy shell**ga yuklash              |
| **Nom konvensiyasi**    | `snake_case`, `is_*`/`has_*`, modul prefiksi       |
| **Stderr**              | Xato xabarlari `>&2` ga                            |

🎉 Birinchi capstone yaqinlashdi — keyingi bobda biz **massivlar** bilan yanada kuchli funksiyalar yozamiz.

> **Keyingi sahifa:** [2. Massivlar va lug'atlar →](./02-arrays)
