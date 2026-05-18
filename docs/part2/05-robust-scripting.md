---
title: "Robust skriptlar — set -euo pipefail"
description: "Strict mode, getopts, logging, ShellCheck, security va production-grade Bash skriptlar uchun to'liq qo'llanma."
---

# 5. Robust skriptlar — `set -euo pipefail`

> **🎯 Bu bobda nimani o'rganasiz:**
> - **Strict mode** — `set -euo pipefail` va `IFS=$'\n\t'` chuqur tahlil
> - **`getopts`** — professional argument parsing
> - **Logging** — daraja, vaqt, rang, TTY detect bilan
> - **ShellCheck** — bashning rasmiy lintera
> - **Security** — quoting, command injection, secrets
> - **Production template** — copy-paste qilsa bo'ladigan skelet
>
> **⏱ Vaqt:** ~40 daqiqa
> **🧪 Mashqlar:** `bashlings watch 10_robust` (kelajak sprint)

---

## 5.1. Nima farqi bor — "ishlaydi" va "production-ready"?

```bash
# Versiya 1 — "ishlaydi"
cp /tmp/data.txt /var/backups/

# Versiya 2 — "production-ready"
set -euo pipefail
readonly SRC="/tmp/data.txt"
readonly DST="/var/backups/data_$(date +%Y%m%d).txt"
[[ -r "$SRC" ]] || { log_error "$SRC o'qib bo'lmadi"; exit 1; }
cp -p "$SRC" "$DST" || { log_error "Backup muvaffaqiyatsiz"; exit 1; }
log_info "✅ Backup: $DST"
```

Ikkalasi ham bir xil ishni qiladi. Lekin **ikkinchisi**:
- Xato bo'lsa darhol to'xtaydi
- Aniqlanmagan o'zgaruvchini sezadi
- Xato sabablarini log'ga yozadi
- Manba mavjudligini oldindan tekshiradi
- `cp` xato bo'lsa skript davom etmaydi
- Foydalanuvchiga aniq feedback beradi

Bu bobning maqsadi — har bir skriptingizni **versiya 1**'dan **versiya 2**'ga aylantirish.

::: tip Qoida
**Production skript = "ishlaydi" + "to'g'ri xatolik chiqaradi" + "xavfsiz" + "qayta ishga tushirib bo'ladi" + "tekshirib bo'ladi".**
:::

---

## 5.2. Strict mode — `set -euo pipefail`

Bashning eng kuchli (va eng kam ishlatiladigan) qatori:

```bash
#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'
```

Buni har skript boshida yozish — DevOps olamining standartiga aylangan. Keling, har birini batafsil ko'ramiz.

### `set -e` — birinchi xatoda to'xtash

Default Bash xulq-atvori — buyruq xato bersa **davom etadi**:

```bash
ls /yoq-katalog        # xato
echo "Davom etyapman"  # haligacha bajariladi!
```

`set -e` bilan — birinchi xatoda darhol exit:

```bash
set -e
ls /yoq-katalog        # bu yerda to'xtaydi
echo "Bu satrgacha yetmaydi"
```

### Bu yetarlimi? Yo'q — `set -e` ning nyuanslari

`set -e` **ko'p holatda chetlab o'tiladi**:

```bash
set -e

cmd || true             # || dan keyin xato bekor qilinadi
cmd && other_cmd        # ham bekor
if cmd; then            # if shartda — bekor
    ...
fi
cmd | grep foo          # ❌ cmd xatosi yashirinadi (pipefail kerak)
fn() { cmd; }           # funksiya ichida — partial
```

::: warning `set -e` bilan funksiya — diqqat
```bash
set -e
risky() {
    cmd_that_fails
    echo "Bu chiqadi yoki yo'q?"
}
risky || true
```
Bash 4.4'gacha `set -e` funksiya ichida nostandart ishlardi. Hozir ham nyuanslari bor. Funksiyalar ichida **aniq** xato handling tavsiya etiladi.
:::

### `set -u` — aniqlanmagan o'zgaruvchi xato

```bash
set -u
echo "$undefined_var"   # ❌ unbound variable
```

Bu **typoлардан** himoya qiladi:

```bash
set -u
name="Ali"
echo "Salom, $namr"     # typo! → unbound variable xatosi
```

### `set -u` bilan ishlash texnikalari

Ba'zan o'zgaruvchi **ixtiyoriy** bo'lishi mumkin. Default qiymat patternini ishlating:

```bash
set -u

# ❌ Xato beradi:
echo "${OPT_VAR}"

# ✓ "qiymat berilmagan bo'lsa, bo'sh":
echo "${OPT_VAR:-}"

# ✓ Default qiymat:
echo "${OPT_VAR:-default}"

# ✓ Bo'sh emasligini tekshirish:
if [[ -z "${OPT_VAR:-}" ]]; then
    echo "berilmagan"
fi
```

### `set -o pipefail` — pipeline ichidagi xatolarni ushlash

Default'da pipe xatosi yashirinadi — faqat **oxirgi** buyruqning exit code'i qaytariladi:

```bash
false | true            # exit 0 ✓ (false yashirildi)
echo $?                 # 0

set -o pipefail
false | true            # exit 1 (birinchi xato qaytariladi)
echo $?                 # 1
```

Real misol:
```bash
# Bu xavfli — agar curl yiqilsa, jq ham 0 qaytaradi
curl https://api.com/data | jq .

# pipefail bilan curl xatosi sezish mumkin
set -o pipefail
curl https://api.com/data | jq .
```

### `IFS=$'\n\t'` — xavfsiz word-splitting

Default `IFS` — `<space><tab><newline>`. Bu — fayl nomlarida probel bo'lganda xavfli:

```bash
files="my file.txt other.txt"
for f in $files; do echo "$f"; done
# my
# file.txt
# other.txt    ← "my file.txt" buzilgan!
```

`IFS=$'\n\t'` — faqat newline va tab. Probel saqlanadi:

```bash
IFS=$'\n\t'
files=$'my file.txt\nother.txt'
for f in $files; do echo "$f"; done
# my file.txt
# other.txt    ← to'g'ri!
```

::: tip Skript boshida 4 qator
```bash
#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'
```
Bu — "Bash Strict Mode" — har production skript boshlanish formulasi.
:::

---

## 5.3. Xatoliklarni boshqarish — strict mode'dan tashqari

`set -e` kuchli, lekin ba'zan **xato kutilgan** bo'ladi. Masalan, `grep` topmagan bo'lsa exit 1 qaytaradi — bu xato emas, bu **natija**.

### `||` — "xato bo'lsa default qiymat"

```bash
matches=$(grep "ERROR" log.txt) || matches="topilmadi"

if ! command -v jq &>/dev/null; then
    echo "jq o'rnatilmagan" >&2
    exit 1
fi
```

### `if ! command` — "buyruq xato bo'ldi"

```bash
if ! curl -fsS "https://api.com/" > response.json; then
    echo "API yiqildi"
    exit 1
fi
```

### `set +e` / `set -e` blok

Strict mode'ni vaqtincha o'chirib qo'yish:

```bash
set -e
some_critical_step

set +e                   # strict mode'ni o'chirish
optional_step_that_may_fail
maybe_fail
set -e                   # qaytarish
```

### `ERR` trap — markaziy xatolik handler

```bash
#!/usr/bin/env bash
set -euo pipefail

error_handler() {
    local line=$1
    local cmd=$2
    echo "❌ Xato $line qatorida: $cmd" >&2
    exit 1
}

trap 'error_handler $LINENO "$BASH_COMMAND"' ERR

# Bu yerda xato sodir bo'lsa, handler chaqiriladi
```

---

## 5.4. Idempotent skriptlar

**Idempotent** — bir necha marta ishga tushirsangiz ham bir xil natija beradigan skript. Production'ning oltin qoidasi.

### Misol — toza emas (idempotent emas)

```bash
mkdir /var/myapp        # ikkinchi marta xato beradi
adduser myapp           # ikkinchi marta xato beradi
echo "config" > /etc/myapp.conf  # eskini almashtiradi
```

### Idempotent variant

```bash
mkdir -p /var/myapp                          # mavjud bo'lsa OK
id myapp &>/dev/null || adduser myapp        # faqat yo'q bo'lsa
[[ -f /etc/myapp.conf ]] || echo "config" > /etc/myapp.conf
```

### State marker pattern

```bash
readonly MARKER="/var/run/migration_v2.done"

if [[ -f "$MARKER" ]]; then
    echo "Migration allaqachon bajarilgan"
    exit 0
fi

# ... migration ishi ...

touch "$MARKER"
echo "✅ Tugadi"
```

### Lock fayl — bitta nusxa kafolati

(4-bobdan tanish — bu yerda yana bir bor)

```bash
readonly LOCKFILE="/tmp/myapp.lock"

if [[ -f "$LOCKFILE" ]] && kill -0 "$(cat "$LOCKFILE")" 2>/dev/null; then
    echo "Allaqachon ishlamoqda"
    exit 0
fi
echo $$ > "$LOCKFILE"
trap 'rm -f "$LOCKFILE"' EXIT
```

---

## 5.5. Logging — production sifati

### `printf` vs `echo`

```bash
echo -e "Salom\tdunyo"   # ❌ -e portable emas (BSD/POSIX farqlari)
printf 'Salom\tdunyo\n'  # ✓ har joyda ishlaydi
```

**Qoida:** har doim `printf` ishlating. `echo` faqat oddiy stringlar uchun.

### Log levels va timestamp

```bash
log_debug() { [[ "${DEBUG:-0}" == "1" ]] && printf '[%s] [DEBUG] %s\n' "$(date +%T)" "$*" >&2; }
log_info()  { printf '[%s] [INFO]  %s\n' "$(date +%T)" "$*"; }
log_warn()  { printf '[%s] [WARN]  %s\n' "$(date +%T)" "$*" >&2; }
log_error() { printf '[%s] [ERROR] %s\n' "$(date +%T)" "$*" >&2; }
```

Ishlatish:
```bash
log_info "Server boshlandi"
log_warn "Disk 80% to'lgan"
log_error "Database javob bermayapti"
DEBUG=1 ./script.sh     # debug yoqilgan
```

### Rang — faqat TTY bo'lsa

```bash
if [[ -t 1 ]]; then       # stdout TTY mi?
    RED='\033[31m'
    GREEN='\033[32m'
    YELLOW='\033[33m'
    RESET='\033[0m'
else
    RED='' GREEN='' YELLOW='' RESET=''
fi

log_info()  { printf '%b[%s] [INFO]%b  %s\n' "$GREEN"  "$(date +%T)" "$RESET" "$*"; }
log_warn()  { printf '%b[%s] [WARN]%b  %s\n' "$YELLOW" "$(date +%T)" "$RESET" "$*" >&2; }
log_error() { printf '%b[%s] [ERROR]%b %s\n' "$RED"    "$(date +%T)" "$RESET" "$*" >&2; }
```

`[[ -t 1 ]]` — stdout terminal'mi? Pipe yoki faylga yozilsa rang qo'shilmaydi (toza log fayllar).

### `logger` — syslog'ga yozish

```bash
logger -t myapp "Server boshlandi"
# /var/log/syslog'ga yoziladi (Linux)
# Tag: myapp
```

---

## 5.6. `getopts` — professional argument parsing

`$1`, `$2`, `$3` qo'l bilan o'qish — oddiy skriptlar uchun. **Bir nechta flag** kerak bo'lsa — `getopts` ishlatamiz.

### Asosiy sintaksis

```bash
verbose=0
output=""

while getopts ":vo:h" opt; do
    case "$opt" in
        v) verbose=1 ;;
        o) output="$OPTARG" ;;
        h) echo "Yordam"; exit 0 ;;
        \?) echo "Noma'lum flag: -$OPTARG" >&2; exit 2 ;;
        :)  echo "Flag -$OPTARG argument talab qiladi" >&2; exit 2 ;;
    esac
done
shift $((OPTIND - 1))    # parse qilingan flag'larni $@ dan olib tashlaydi
```

### Format string'ni o'qish

`":vo:h"`:
- Boshidagi `:` — silent error mode (biz xatolarni o'zimiz chiqaramiz)
- `v` — flag (argumentsiz)
- `o:` — flag + argument (`:` keyin)
- `h` — flag (argumentsiz)

### Real misol

```bash
#!/usr/bin/env bash
set -euo pipefail

usage() {
    cat <<EOF
Foydalanish: $0 [-v] [-o FAYL] [-h] INPUT

Variantlar:
  -v        Verbose rejim
  -o FAYL   Natija fayli (default: stdout)
  -h        Yordam
EOF
}

verbose=0
output=""

while getopts ":vo:h" opt; do
    case "$opt" in
        v) verbose=1 ;;
        o) output="$OPTARG" ;;
        h) usage; exit 0 ;;
        \?) echo "❌ Noma'lum flag: -$OPTARG" >&2; usage; exit 2 ;;
        :)  echo "❌ -$OPTARG argument talab qiladi" >&2; exit 2 ;;
    esac
done
shift $((OPTIND - 1))

if [[ $# -lt 1 ]]; then
    echo "❌ INPUT argument zarur" >&2
    usage
    exit 2
fi

input="$1"
[[ $verbose -eq 1 ]] && echo "Verbose yoqildi, input=$input"
```

Foydalanish:
```bash
./script.sh -v -o result.txt input.txt
./script.sh -h
./script.sh -x          # → Noma'lum flag: -x
```

### `getopts` cheklovlari

- Faqat **qisqa flag** (`-v`, `-o`) qo'llab-quvvatlaydi
- Long flag (`--verbose`) kerak bo'lsa — `getopt` (BSD/GNU farqli) yoki qo'l parsing

---

## 5.7. Konfiguratsiya — environment vs CLI

### Environment variables — sozlash uchun

```bash
LOG_LEVEL="${LOG_LEVEL:-info}"
TIMEOUT="${TIMEOUT:-30}"
API_URL="${API_URL:-https://api.example.com}"
```

Foydalanish:
```bash
LOG_LEVEL=debug ./script.sh
TIMEOUT=60 ./script.sh
```

### Konfiguratsiya fayli — `source`

```bash
# ~/.myapp/config
LOG_LEVEL=debug
API_URL=https://prod.example.com
TIMEOUT=60
```

```bash
# script.sh
[[ -f ~/.myapp/config ]] && source ~/.myapp/config

LOG_LEVEL="${LOG_LEVEL:-info}"
```

### `.env` fayl

```bash
# .env
DATABASE_URL=postgres://localhost/myapp
SECRET_KEY=xyz123
```

```bash
# Load only if .env exists
if [[ -f .env ]]; then
    set -a       # avtomatik export
    source .env
    set +a
fi
```

::: warning `.env` ni gitignore ga qo'shing
Secrets fayllarni hech qachon repository'ga commit qilmang. `.gitignore`'da `.env` bo'lishi shart.
:::

---

## 5.8. ShellCheck — bashning rasmiy linteri

`shellcheck` — Bash skriptlardagi xatoliklarni va anti-pattern'larni statik tahlil qiladi. **Production skript yozasizmi — `shellcheck`siz emas.**

### O'rnatish

```bash
# macOS
brew install shellcheck

# Ubuntu/Debian
sudo apt install shellcheck

# Web variant (sinov uchun)
# https://www.shellcheck.net/
```

### Ishlatish

```bash
shellcheck script.sh
```

### Misol xato

```bash
# bad.sh
name=$1
echo "Salom $name"
rm $name
```

```text
$ shellcheck bad.sh

In bad.sh line 2:
name=$1
     ^-- SC2086: Double quote to prevent globbing and word splitting.

In bad.sh line 3:
echo "Salom $name"
^-- SC2086: ...

In bad.sh line 4:
rm $name
   ^-- SC2086: Double quote to prevent globbing and word splitting.
```

Tuzatilgan versiya:
```bash
name="$1"
echo "Salom $name"      # ichida bor — OK
rm "$name"              # quote qilingan
```

### Eng ko'p uchraydigan warninglar

| Kod    | Mazmuni                                          |
|--------|--------------------------------------------------|
| SC2086 | Tirnoqsiz o'zgaruvchi — quote qiling             |
| SC2155 | `local var=$(cmd)` — exit code yutiladi          |
| SC2046 | `$(cmd)` ni quote qiling                         |
| SC2034 | Ishlatilmagan o'zgaruvchi                        |
| SC2164 | `cd` ning xatosini handle qiling                 |
| SC2181 | `[[ $? ... ]]` o'rniga `if cmd` ishlating         |
| SC2207 | `arr=($(cmd))` — `mapfile -t` ishlating          |

### Selektiv suppression

Ba'zan ataylab "noto'g'ri" yozasiz — masalan, intentional word-splitting:

```bash
# shellcheck disable=SC2086
echo $unquoted_intentional
```

`# shellcheck` directive — bu **kommentar**, lekin shellcheck buni o'qiydi.

::: tip CI'da majburiy qiling
GitHub Actions / GitLab CI da `shellcheck *.sh` qadamini qo'shing. Pull request'da xato bo'lsa CI yiqiladi — bu sifatni darhol oshiradi.
:::

---

## 5.9. Testing — `bats-core`

Skriptlarni avtomatik test qilish uchun **bats** (Bash Automated Testing System) — eng mashhuri.

### O'rnatish

```bash
brew install bats-core
```

### Test misoli

```bash
# tests/test_math.bats

@test "yigindi 2 va 3" {
    result=$(./math.sh 2 3)
    [ "$result" = "5" ]
}

@test "salbiy sonlar ishlaydi" {
    result=$(./math.sh -2 5)
    [ "$result" = "3" ]
}

@test "argumentsiz xato beradi" {
    run ./math.sh
    [ "$status" -ne 0 ]
}
```

### Ishlatish

```bash
bats tests/
# ✓ yigindi 2 va 3
# ✓ salbiy sonlar ishlaydi
# ✓ argumentsiz xato beradi
# 3 tests, 0 failures
```

### CI integratsiya

```yaml
# .github/workflows/test.yml
- run: bats tests/
- run: shellcheck *.sh
```

---

## 5.10. Xavfsizlik — production'ning eng muhim qismi

### 1. Har doim quote qiling

```bash
file="$1"

rm $file        # ❌ "my file.txt" → ikki argument
rm "$file"      # ✓
```

### 2. Command injection xavfi

```bash
# ❌ XAVFLI
read -p "Fayl nomi: " name
ls $name
# Agar foydalanuvchi "; rm -rf /" yozsa — disaster

# ✓ XAVFSIZ
read -p "Fayl nomi: " name
ls "$name"
```

### 3. `eval` dan qoching

```bash
eval "$user_input"      # ❌ HECH QACHON
```

### 4. `mktemp` — xavfsiz vaqtinchalik fayllar

```bash
# ❌ Predictable, xavfli
tmpfile=/tmp/myapp.tmp

# ✓ Tasodifiy, atomic
tmpfile=$(mktemp)
tmpdir=$(mktemp -d)

trap 'rm -rf "$tmpfile" "$tmpdir"' EXIT
```

### 5. Secrets bilan ishlash

```bash
# ❌ Hech qachon parol skript ichida
DB_PASSWORD="hardcoded123"

# ✓ Environment'dan
DB_PASSWORD="${DB_PASSWORD:?DB_PASSWORD o'rnatilmagan}"

# ✓ Yoki secrets manager (vault, AWS Secrets, ...)
DB_PASSWORD=$(aws secretsmanager get-secret-value --secret-id db --query SecretString --output text)
```

### 6. Fayl ruxsatlari

```bash
umask 077                # yaratilgan fayllar 600 bo'ladi (faqat ega o'qiydi)
tmpfile=$(mktemp)
echo "secret" > "$tmpfile"
ls -l "$tmpfile"         # -rw-------
```

### 7. `set -u` xavfsizlik foydasi

```bash
set -u
rm -rf "$WORK_DIR"/*    # WORK_DIR aniqlanmagan bo'lsa — XATO, root o'chmaydi
```

Bu — `rm -rf /` kabi disasterlardan saqlovchi muhim mexanizm.

---

## 5.11. Dokumentatsiya va help

### Usage pattern

```bash
usage() {
    cat <<EOF
$(basename "$0") — qisqa tavsif

FOYDALANISH:
    $(basename "$0") [VARIANTLAR] ARGUMENTLAR

VARIANTLAR:
    -h          Bu yordam matnini ko'rsatish
    -v          Verbose rejim
    -o FAYL     Natija fayli

MISOLLAR:
    $(basename "$0") -v input.txt
    $(basename "$0") -o result.txt data.csv

EOF
}
```

### Inline kommentlar

Yaxshi:
```bash
# Eski (>30 kun) backuplarni tozalash
find "$BACKUP_DIR" -name "*.tar.gz" -mtime +30 -delete
```

Yomon:
```bash
find "$BACKUP_DIR" -name "*.tar.gz" -mtime +30 -delete  # find ishlatadi
```

Kommentar **NIMA UCHUN**ni tushuntirsin, **NIMA**ni emas (kod o'zi aytadi).

---

## 5.12. Real misol — Production-Ready Template

Bu — copy-paste qilsa bo'ladigan **skelet**. Hammasi birga:

```bash
#!/usr/bin/env bash
#
# my-script.sh — qisqa tavsif (bir-ikki qator)
#
# Foydalanish:
#   ./my-script.sh [-v] [-o output] <input>
#

# === Strict mode ===
set -euo pipefail
IFS=$'\n\t'

# === Konstantalar ===
readonly SCRIPT_NAME="$(basename "$0")"
readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly LOG_FILE="${LOG_FILE:-/tmp/${SCRIPT_NAME%.sh}.log}"

# === Ranglar (faqat TTY uchun) ===
if [[ -t 1 ]]; then
    readonly RED='\033[31m' GREEN='\033[32m' YELLOW='\033[33m' DIM='\033[2m' RESET='\033[0m'
else
    readonly RED='' GREEN='' YELLOW='' DIM='' RESET=''
fi

# === Logging ===
_log() {
    local level="$1" color="$2"; shift 2
    local line
    line=$(printf '[%s] [%-5s] %s' "$(date '+%Y-%m-%d %H:%M:%S')" "$level" "$*")
    printf '%b%s%b\n' "$color" "$line" "$RESET" >&2
    printf '%s\n' "$line" >> "$LOG_FILE"
}
log_debug() { [[ "${DEBUG:-0}" == "1" ]] && _log DEBUG "$DIM"    "$@"; }
log_info()  { _log INFO  "$GREEN"  "$@"; }
log_warn()  { _log WARN  "$YELLOW" "$@"; }
log_error() { _log ERROR "$RED"    "$@"; }
die()       { log_error "$@"; exit 1; }

# === Cleanup ===
__cleaned=0
cleanup() {
    local rc=$?
    [[ $__cleaned -eq 1 ]] && return
    __cleaned=1
    # tmp fayllar, lock fayllar va boshqa resurslarni tozalash
    [[ -n "${TMPDIR_:-}" && -d "$TMPDIR_" ]] && rm -rf "$TMPDIR_"
    if [[ $rc -eq 0 ]]; then
        log_info "Yakunlandi (success)"
    elif [[ $rc -eq 130 ]]; then
        log_warn "Foydalanuvchi to'xtatdi"
    else
        log_error "Exit code: $rc"
    fi
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
trap 'die "Xato $LINENO qatorida: $BASH_COMMAND"' ERR

# === Usage ===
usage() {
    cat <<EOF
$SCRIPT_NAME — qisqa tavsif

FOYDALANISH:
    $SCRIPT_NAME [VARIANTLAR] <input>

VARIANTLAR:
    -v          Verbose rejim
    -o FAYL     Natija fayli (default: stdout)
    -h          Bu yordam

MUHIT O'ZGARUVCHILARI:
    DEBUG=1     Debug logni yoqish
    LOG_FILE    Log fayl yo'li (default: /tmp/$SCRIPT_NAME.log)
EOF
}

# === Argument parsing ===
verbose=0
output=""

while getopts ":vo:h" opt; do
    case "$opt" in
        v) verbose=1 ;;
        o) output="$OPTARG" ;;
        h) usage; exit 0 ;;
        \?) die "Noma'lum flag: -$OPTARG" ;;
        :)  die "Flag -$OPTARG argument talab qiladi" ;;
    esac
done
shift $((OPTIND - 1))

[[ $# -lt 1 ]] && { usage; die "Input zarur"; }

readonly INPUT="$1"

# === Asosiy ish ===
main() {
    log_info "Boshlandi: $INPUT"
    [[ $verbose -eq 1 ]] && log_info "Verbose yoqildi"

    [[ -r "$INPUT" ]] || die "Topilmadi yoki o'qib bo'lmaydi: $INPUT"

    # Vaqtinchalik katalog
    TMPDIR_=$(mktemp -d)
    log_debug "Tmp: $TMPDIR_"

    # ... haqiqiy ish bu yerda ...

    log_info "Tugadi"
}

main "$@"
```

### Bu shablon nima qiladi?

| Element                | Foyda                                          |
|------------------------|------------------------------------------------|
| `set -euo pipefail`    | Strict mode — xato yashirinmaydi               |
| `IFS=$'\n\t'`          | Probelli fayllar bilan xavfsiz                 |
| `readonly` konstantalar| Skript holatini barqaror saqlash               |
| TTY-aware ranglar      | Terminal va fayl ikkalasiga to'g'ri yozish     |
| Logging — 4 daraja     | Production tahlil uchun                        |
| Cleanup + 4 trap       | Har vaziyatda toza yakunlanish                 |
| `usage()` + `-h`       | O'z-o'zini hujjatlaydigan                      |
| `getopts`              | Professional argument parsing                  |
| `main "$@"`            | Asosiy mantiq bitta funksiyada — test qilish oson|

::: tip Foydalanish
Bu shablonni `template.sh` deb saqlang. Yangi skript yozayotganda nusxa olib boshlang. **80% vazifa allaqachon bajarilgan.**
:::

---

## 5.13. Tez-tez uchraydigan xatolar

::: danger Top 10 production tuzoq

1. **`set -e` mavjud emas.**
   Skript silently xato qoldiradi. Har doim qo'shing.

2. **`set -u` mavjud emas.**
   Typo'lar oson o'tib ketadi.

3. **`set -o pipefail` unutilgan.**
   Pipeline xatolari ko'rinmaydi.

4. **Tirnoqsiz o'zgaruvchi.**
   `rm $file` — probelli fayllar buziladi.

5. **`local r=$(cmd)` — exit code yutilgan.**
   ```bash
   local r          # avval e'lon
   r=$(cmd)         # keyin to'ldirish
   ```

6. **`/dev/null` ga stderr unutilgan.**
   `ls /yoq 2>/dev/null` — sukut bilan o'tkazish kerak.

7. **`ShellCheck` ishlatilmagan.**
   80% xato avtomatik aniqlanadi.

8. **`cd` xatosi handle qilinmagan.**
   ```bash
   cd "$dir" || die "cd xatosi: $dir"
   ```

9. **`eval` ishlatilgan.**
   Command injection — qoching.

10. **Hardcoded fayl yo'llari.**
    `/Users/mac/...` — boshqa kompyuterda ishlamaydi. `$HOME`, `$(dirname "$0")` ishlating.
:::

---

## 5.14. Mashqlar

> 🧪 Kelajakda `bashlings watch 10_robust` paketida.

1. **Strict-mode lab** — quyidagi skript faqat strict mode'da xato beradi. Sababini tushuntiring va tuzating:
   ```bash
   #!/usr/bin/env bash
   name="${name}"
   files=$(ls /yoq)
   echo "tugadi"
   ```

2. **ShellCheck cleanup** — `shellcheck --severity=warning` bilan tekshirib, xatosiz holatga keltirsin:
   ```bash
   #!/bin/bash
   for f in $(ls *.log); do
       cp $f /backup/
       echo "Done $f"
   done
   ```

3. **`getopts` parser** — `-c COUNT`, `-d DELAY`, `-v`, `-h` flaglarini qo'llab-quvvatlovchi skript yozing.

4. **Idempotent installer** — `nginx` o'rnatuvchi skript yozing. Bir necha marta ishga tushirish xavfsiz bo'lsin (allaqachon o'rnatilgan bo'lsa xato bermasin).

5. **Production template** — yuqoridagi shablonni nusxalab, oddiy "Hello, World" skripti yozing. ShellCheck'dan o'tsin.

---

## 5.15. Xulosa

### Production-ready skript checklist

- [ ] Shebang: `#!/usr/bin/env bash`
- [ ] Strict mode: `set -euo pipefail`
- [ ] Xavfsiz IFS: `IFS=$'\n\t'`
- [ ] `readonly` konstantalar
- [ ] Logging — kamida 3 daraja (info/warn/error)
- [ ] Cleanup `trap EXIT`
- [ ] `INT`/`TERM` graceful handling
- [ ] `usage()` + `-h` flag
- [ ] `getopts` bilan argument parsing
- [ ] **Hamma** o'zgaruvchi quote qilingan: `"$var"`
- [ ] `mktemp` orqali tmp fayllar
- [ ] ShellCheck'dan o'tadi (`shellcheck` 0 warning)
- [ ] Asosiy ish `main()` funksiyasida
- [ ] Test (bats-core)

### 10 ta oltin qoida

| #  | Qoida                                            |
|----|--------------------------------------------------|
| 1  | `set -euo pipefail` har doim                     |
| 2  | Har o'zgaruvchini quote qiling: `"$var"`         |
| 3  | `mktemp` ishlatib tmp fayllar                    |
| 4  | `trap cleanup EXIT`                              |
| 5  | `ShellCheck` — har commit oldidan                |
| 6  | `getopts` orqali argumentlar                     |
| 7  | `printf` (`echo` emas)                           |
| 8  | `[[ ]]` (`[ ]` emas)                             |
| 9  | `$(...)` (`` `cmd` `` emas)                      |
| 10 | "Magic numbers" yo'q — `readonly` konstantalar   |

---

## 🎉 2-qism tugadi!

Tabriklaymiz! Siz quyidagilarni o'zlashtirdingiz:

| Bob | Mavzu                              | Ko'nikma                              |
|-----|------------------------------------|---------------------------------------|
| 1   | Funksiyalar                        | Kodni qayta ishlatish                 |
| 2   | Massivlar                          | Strukturali ma'lumot                   |
| 3   | sed va awk                         | Industrial matn qayta ishlash         |
| 4   | Signallar va traps                 | Hayotiy cleanup                       |
| 5   | Robust skriptlar                   | Production-grade kod                   |

Endi siz **professional Bash dasturchisiz**. DevOps, SRE, sistema administrator — har qanday rolda kerakli vositalar qo'lingizda.

### Keyingi qadamlar

- **Mashq qilish:** har real loyihada Bash o'rniga Python yoki Go o'ylashdan oldin — Bashda hal qila olishingizni sinab ko'ring
- **Kitobxonlik:** [Google Shell Style Guide](https://google.github.io/styleguide/shellguide.html), [Bash Pitfalls](https://mywiki.wooledge.org/BashPitfalls)
- **Bash kelajakni o'rganing** — `coproc`, `<()` process substitution chuqurroq, `mapfile` flaglari
- **3-qism (Part 3)** — `curl`, `ssh`, `jq`, `cron`, `docker`, CI/CD bilan integratsiya

> **Yaxshi yo'l!** 🚀
>
> Yana ko'p ishlash bor — lekin asos endi mustahkam.
