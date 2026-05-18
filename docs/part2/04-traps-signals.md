---
title: "Signallar va traps"
description: "Unix signallari, trap, EXIT pseudo-signali, cleanup pattern, graceful shutdown va background jarayonlar."
---

# 4. Signallar va traps

> **🎯 Bu bobda nimani o'rganasiz:**
> - **Unix signallari** — SIGINT, SIGTERM, SIGKILL, SIGHUP va boshqalar
> - **`trap`** orqali signallarni qabul qilish
> - **`EXIT` pseudo-signali** — eng muhim cleanup mexanizmi
> - **Cleanup pattern** — temp fayllar, lock fayllar, ulanishlar
> - **Graceful shutdown** — Ctrl+C bosilsa ham toza yakunlanish
> - **Background jarayonlar** — `&`, `wait`, signal propagation
>
> **⏱ Vaqt:** ~25 daqiqa
> **🧪 Mashqlar:** `bashlings watch 09_traps` (kelajak sprint)

---

## 4.1. Nima uchun bu kerak?

Quyidagi vaziyatlarni tasavvur qiling:

1. Skriptingiz 100MB vaqtinchalik faylga ma'lumot yozayotgan edi. Foydalanuvchi **`Ctrl+C`** bosdi. Fayl qaytib qoldi — diskni iflos qildi.

2. Database backup skripti ishlayotgan edi. Terminal yopildi. Yarim-bajarilgan backup va ochiq ulanish saqlanib qoldi.

3. Server-monitoring skripti `cron`'da yurar edi. Tizim qayta yuklandi. PID fayli qoldi, keyingi run "allaqachon ishlamoqda" deb noto'g'ri xato berdi.

Bularning **hammasi** `trap` va signallar bilan oldini olinadi.

::: tip Asosiy g'oya
**Hech qachon "muvaffaqiyatli tugadi" deb umid qilmang.** Skriptingiz **istalgan vaqtda** to'xtatilishi mumkin — `Ctrl+C`, terminal yopilishi, OOM killer, `kill -9`. **Cleanup'ni kafolatlash kerak.**
:::

---

## 4.2. Unix signallari — qisqacha modeli

**Signal** — bir jarayondan boshqasiga (yoki kernel'dan) **asinxron xabar**. Foydalanuvchining `Ctrl+C` — bu signal. `kill` buyrug'i — signal yuborish. Tizim ishini boshqarish — signalar orqali.

### Signallar — har biri raqam va nomga ega

```bash
kill -l
# 1) SIGHUP   2) SIGINT   3) SIGQUIT  ...
# 9) SIGKILL  15) SIGTERM ...
```

### Kerak bo'ladigan signallar

| Signal      | Raqam   | Mazmuni                                       | Trap qilsa bo'ladimi? |
|-------------|---------|-----------------------------------------------|------------------------|
| `SIGINT`    | 2       | **Ctrl+C** — odob bilan to'xtatish so'rovi    | ✅                     |
| `SIGTERM`   | 15      | "Iltimos to'xtang" (default `kill PID`)       | ✅                     |
| `SIGHUP`    | 1       | Terminal yopildi (hangup)                     | ✅                     |
| `SIGQUIT`   | 3       | **Ctrl+\\** — core dump bilan to'xtatish      | ✅                     |
| `SIGUSR1`   | 10/30   | Foydalanuvchi belgilangan #1                  | ✅                     |
| `SIGUSR2`   | 12/31   | Foydalanuvchi belgilangan #2                  | ✅                     |
| `SIGCHLD`   | 17/20   | Child process holati o'zgardi                 | ✅                     |
| `SIGPIPE`   | 13      | Yozilayotgan pipe yopildi                     | ✅                     |
| `SIGALRM`   | 14      | Timer ishladi                                 | ✅                     |
| **`SIGKILL`** | **9** | **Darhol o'ldirish — qarshi tura olmaysiz**   | ❌ **YO'Q**            |
| **`SIGSTOP`** | 19/17 | **Darhol pauza — qarshi tura olmaysiz**       | ❌ **YO'Q**            |
| **`EXIT`**  | (0)     | **Bash pseudo-signali** — har xil yakunda     | ✅ (eng muhim)         |

::: warning `SIGKILL` ni hech qachon ushlab bo'lmaydi
`kill -9 PID` — jarayonni **darhol** o'ldiradi. Cleanup kodi ishlamaydi. Shuning uchun kritik holatlar uchun "tashqi qo'riqchi" jarayonlar kerak.
:::

---

## 4.3. `kill` — signal yuborish

```bash
# Default — SIGTERM (15) yuboradi
kill 1234

# Aniq signal
kill -SIGINT 1234
kill -INT 1234            # SIG prefiksini tashlash mumkin
kill -2 1234              # raqam bilan

# Eng kuchlisi — qarshi tura olmaydi
kill -9 1234
kill -KILL 1234

# Nom bo'yicha (process name)
killall my-script.sh
pkill -f "long pattern"

# Joriy jarayon PID si
echo $$                   # bash uchun
```

### `kill -0` — faqat tekshirish

```bash
# Jarayon hayotmi?
if kill -0 1234 2>/dev/null; then
    echo "Hali ishlayapti"
fi
```

`-0` signal yuborilmaydi, faqat ruxsat tekshiriladi.

---

## 4.4. `trap` asoslari

`trap` — signal qabul qilinganda nima qilishni belgilaydi.

### Sintaksis

```bash
trap '<bajariladigan kod>' SIGNAL [SIGNAL ...]
```

### Birinchi misol

```bash
#!/usr/bin/env bash

trap 'echo "⚠ Ctrl+C bosildi, lekin men tirikman!"' SIGINT

echo "5 soniya kutaman..."
for i in {1..5}; do
    echo "  $i"
    sleep 1
done
echo "Yakunlandi"
```

Ishga tushiring va `Ctrl+C` bosing — skript ogohlantirib davom etadi.

### Trap'larni ko'rish

```bash
trap -p                   # joriy o'rnatilgan trap'lar ro'yxati
trap -p SIGINT            # faqat SIGINT uchun
```

### Trap'ni o'chirish (default'ga qaytarish)

```bash
trap - SIGINT             # SIGINT default xulq-atvor (kill)
trap - EXIT INT TERM      # bir nechta birgalikda
```

### Trap'ni butunlay e'tibor bermaslik

```bash
trap '' SIGINT            # Ctrl+C umuman ishlamaydi
```

::: danger E'tiborsiz qoldirish xavfli
`trap '' SIGINT` — foydalanuvchining `Ctrl+C` ni bekor qiladi. Bu shubhali UX. Faqat **maxsus, hujjatlangan** vaziyatlarda ishlating.
:::

---

## 4.5. `EXIT` — eng muhim pseudo-signal

`EXIT` — haqiqiy Unix signali emas, bash ning **maxsus pseudo-signali**. U **har qanday yo'l bilan** skript tugaganda ishlaydi:

- Normal yakun (oxirgi buyruq bajarildi)
- `exit` buyrug'i chaqirildi
- Xato bilan to'xtadi (`set -e` ostida)
- Signal qabul qilindi (`SIGINT`, `SIGTERM`)

::: tip Cleanup uchun zarba
Aynan shu xususiyat `EXIT` ni **cleanup uchun mukammal joy** qiladi. Bir marta yozasiz — har holatda ishlaydi.
:::

### Klassik pattern

```bash
#!/usr/bin/env bash
set -euo pipefail

tmpfile=$(mktemp)

cleanup() {
    rm -f "$tmpfile"
    echo "🧹 Tozalandi"
}

trap cleanup EXIT

# Asosiy ish
echo "ma'lumot" > "$tmpfile"
# ... boshqa kod ...

# `cleanup` har holda chaqiriladi
```

Sinab ko'ring:
- Normal tugating → cleanup chiqaradi
- `Ctrl+C` bosing → cleanup chiqaradi
- `exit 1` qo'shing → cleanup chiqaradi

---

## 4.6. Eng yaxshi cleanup pattern

Ko'pchilik skriptlar `Ctrl+C` ni alohida xabar bilan qarshilab, EXIT ni cleanup uchun ishlatadi. Bu eng toza model:

```bash
#!/usr/bin/env bash
set -euo pipefail

tmpfile=$(mktemp)
lockfile="/tmp/myapp.lock"

cleanup() {
    local rc=$?
    rm -f "$tmpfile"
    rm -f "$lockfile"
    if [[ $rc -ne 0 ]]; then
        echo "❌ Skript $rc kodi bilan tugadi"
    fi
}

# 1) EXIT — har holatda cleanup
trap cleanup EXIT

# 2) INT / TERM — graceful exit (EXIT trap ham chaqiriladi)
trap 'echo "⚠ to`xtatish so`rovi qabul qilindi"; exit 130' INT TERM

# Asosiy ish
echo $$ > "$lockfile"
echo "ma'lumot" > "$tmpfile"

for i in {1..30}; do
    echo "Qadam $i"
    sleep 1
done

echo "✅ Yakunlandi"
```

### Nima ishlaydi?

| Hodisa                | INT trap | EXIT trap | Cleanup chaqiriladi? |
|-----------------------|----------|-----------|---------------------|
| Normal yakun          | —        | ✅        | **Ha**              |
| `Ctrl+C`              | ✅ → `exit 130` | ✅ | **Ha**          |
| `kill PID`            | TERM → `exit 130` | ✅ | **Ha**           |
| `kill -9 PID`         | ❌ ushlamaydi | ❌ | **Yo'q** ⚠       |
| Skript ichida `exit 1`| —        | ✅        | **Ha**              |

::: tip Exit kodlar konventsiyasi
- `130` = `128 + SIGINT(2)` — Ctrl+C natijasi
- `143` = `128 + SIGTERM(15)` — kill natijasi

Bu Unix konvensiyasi. Programatik tekshirishda foydali.
:::

---

## 4.7. Bir nechta cleanup va `done` flag

Agar `cleanup` ikki marta chaqirilishidan saqlanish kerak bo'lsa (masalan, `Ctrl+C` keyin `exit 1` ham chaqirilsa):

```bash
__cleaned=0
cleanup() {
    [[ $__cleaned -eq 1 ]] && return
    __cleaned=1

    rm -f "$tmpfile"
    rm -f "$lockfile"
}
trap cleanup EXIT
```

Bu — idempotent cleanup patterni.

---

## 4.8. `SIGKILL` — qarshi tura olmaslik

```bash
trap 'echo "haa"' SIGKILL   # ❌ bash xato beradi
```

Bash hatto bunday yozishga ruxsat bermaydi. Sababi: `SIGKILL` va `SIGSTOP` — kernel darajasidagi signallar, foydalanuvchi jarayoni hech qanday yo'l bilan to'sib qo'ya olmaydi.

### Real qoidalar

- **`kill PID`** (SIGTERM) — odobli so'rov. Skript cleanup qilib chiqishi mumkin.
- **`kill -9 PID`** (SIGKILL) — kuch ishlatish. Cleanup ishlamaydi.

::: tip Production xulq-atvor
1. **Birinchi yondashuv:** odobli `SIGTERM`
2. **Bir nechta soniya kutish** (cleanup uchun)
3. **Hech narsa bo'lmasa:** `SIGKILL` (oxirgi chora)

`systemctl stop` aniq shu modelda ishlaydi.
:::

---

## 4.9. Background jarayonlar va `wait`

Asinxron ishlovchi skriptlarda — `&` bilan ishga tushirilgan background jarayonlar — alohida e'tibor kerak.

### `&` va `$!`

```bash
sleep 30 &
echo "Background PID: $!"     # eng oxirgi background PID
```

### `wait` — kutish

```bash
sleep 5 &
pid1=$!

sleep 3 &
pid2=$!

wait $pid1 $pid2
echo "Ikkalasi ham tugadi"
```

### Signal propagation

Default'da, agar parent skriptga `Ctrl+C` bosilsa — child jarayonlar **avtomatik o'lmaydi**. Aniq yo'naltirish kerak:

```bash
#!/usr/bin/env bash

cleanup() {
    echo "Child'larni o'ldiraman..."
    jobs -p | xargs -r kill 2>/dev/null
}
trap cleanup EXIT INT TERM

# Bir nechta worker
worker.sh &
worker.sh &
worker.sh &

wait
```

`jobs -p` — barcha background PID'lar. `xargs -r kill` — ularni o'ldirish.

### `wait -n` (Bash 4.3+)

Birinchi tugagan child'ni kutish:

```bash
worker1 &
worker2 &
worker3 &

# Birortasi tugashini kutamiz
wait -n
echo "Bittasi tugadi"
```

---

## 4.10. Graceful shutdown patterni

Uzun ishlovchi skriptlar uchun (server, monitor, daemon) — to'xtash so'rovini **ravon** qabul qilish kerak.

```bash
#!/usr/bin/env bash
#
# monitor.sh — har 5 soniyada disk holatini tekshirish
#

set -euo pipefail

running=1
shutdown() {
    echo "Shutdown so'rovi qabul qilindi, joriy iteratsiyani yakunlayman..."
    running=0
}
trap shutdown INT TERM

while [[ $running -eq 1 ]]; do
    df -h / | tail -1
    sleep 5
done

echo "✅ Toza yakunlanish"
```

### Bu nima qiladi?

1. `Ctrl+C` bosilsa — `running=0` qiladi
2. Joriy iteratsiya **tugaydi** (`df` chiqishi to'liq)
3. Keyingi `while` tekshiruvi `false` qaytaradi
4. Skript toza yakunlanadi

`exit` chaqirilmaydi — har ishni tugatib chiqamiz.

---

## 4.11. Debug uchun `ERR` va `DEBUG` traplari

### `ERR` — har xatoda

```bash
#!/usr/bin/env bash
set -e

trap 'echo "❌ Xato $LINENO qatorida: $BASH_COMMAND"' ERR

echo "Ish boshlandi"
ls /yoq-katalog            # bu yerda xato bo'ladi
echo "Bu satrgacha yetmaydi"
```

Ishga tushganda:
```text
Ish boshlandi
ls: cannot access '/yoq-katalog': No such file or directory
❌ Xato 6 qatorida: ls /yoq-katalog
```

### `DEBUG` — har buyruqdan oldin

```bash
trap 'echo ">> [LINE $LINENO] $BASH_COMMAND"' DEBUG

x=5
echo "salom"
ls
```

Natija:
```text
>> [LINE 3] x=5
>> [LINE 4] echo "salom"
salom
>> [LINE 5] ls
...
```

::: tip `set -x` bilan farqi
`set -x` ham har buyruqni chiqaradi, lekin DEBUG trap **siz xohlagan ko'rinishda** formatlash imkonini beradi.
:::

---

## 4.12. Real misol — robust backup skript

Hamma kontseptsiyalarni birlashtirgan to'liq misol:

```bash
#!/usr/bin/env bash
#
# backup.sh — Ctrl+C ham, kill ham, oddiy yakun ham — barchasini toza boshqaradi
#

set -euo pipefail

readonly SCRIPT_NAME="$(basename "$0")"
readonly LOCKFILE="/tmp/${SCRIPT_NAME%.sh}.lock"
TMPDIR=""

# --- Lock check (bitta nusxa pattern) ---
if [[ -f "$LOCKFILE" ]]; then
    pid=$(cat "$LOCKFILE")
    if kill -0 "$pid" 2>/dev/null; then
        echo "❌ Backup allaqachon ishlamoqda (PID: $pid)" >&2
        exit 1
    else
        echo "⚠ Eski stale lock topildi, tozalanmoqda"
        rm -f "$LOCKFILE"
    fi
fi

echo $$ > "$LOCKFILE"

# --- Cleanup funksiyasi ---
__cleaned=0
cleanup() {
    local rc=$?
    [[ $__cleaned -eq 1 ]] && return
    __cleaned=1

    echo ""
    echo "🧹 Tozalanmoqda..."
    [[ -n "$TMPDIR" && -d "$TMPDIR" ]] && rm -rf "$TMPDIR"
    rm -f "$LOCKFILE"

    if [[ $rc -eq 0 ]]; then
        echo "✅ Muvaffaqiyatli yakunlandi"
    elif [[ $rc -eq 130 ]]; then
        echo "⚠ Foydalanuvchi to'xtatdi (Ctrl+C)"
    else
        echo "❌ Xato bilan tugadi (exit=$rc)"
    fi
}

trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

# --- Asosiy ish ---
TMPDIR=$(mktemp -d)
echo "📁 Vaqtinchalik katalog: $TMPDIR"

echo "📦 Backup boshlandi ($(date '+%T'))"

for i in {1..10}; do
    echo "  Qadam $i / 10"
    sleep 1
done

archive="$HOME/backup_$(date +%Y%m%d_%H%M%S).tar.gz"
echo "💾 Arxivga yozilmoqda: $archive"
# tar -czf "$archive" -C "$TMPDIR" .   # haqiqiy buyruq

echo "📊 Hajmi: $(du -sh "$TMPDIR" 2>/dev/null | cut -f1)"
```

Bu skript:

- **Lock fayl** orqali bitta nusxa kafolatlaydi (`kill -0` bilan stale tekshiradi)
- **`trap cleanup EXIT`** — har qanday yakunda tozalanadi
- **`INT` → exit 130** va **`TERM` → exit 143** — Unix konventsiya
- **`__cleaned` flag** — ikki marta chaqirilishni oldini oladi
- **Vaqtinchalik katalog `mktemp -d`** — xavfsiz va noyob
- **Color/emoji** — foydalanuvchiga aniq feedback

---

## 4.13. Tez-tez uchraydigan xatolar

::: danger Klassik tuzoqlar

1. **`SIGKILL` ni trap qilishga urinish.**
   Imkonsiz. `kill -9` cleanup'ni chetlab o'tadi. Tashqi qo'riqchi (`systemd`, `supervisord`) ishlating.

2. **Cleanup'da yangi xato chiqarish.**
   `set -e` ostida cleanup ichidagi xato keyingi qatorlarni tushiradi. `cleanup() { rm -f "$tmpfile" || true; ... }`.

3. **EXIT trap ko'rsatkichlari.**
   `cleanup` chaqirilganda `$?` — eng oxirgi buyruq exit code. Trap birinchi qatorida `local rc=$?` qilib ushlang.

4. **Background process'lar trap'siz qoldirilgan.**
   Parent o'lganda children orfan bo'lib qoladi. `trap 'kill $(jobs -p) 2>/dev/null' EXIT` bilan saqlang.

5. **Trap'da o'zgaruvchi ishlatish — single vs double quote.**
   ```bash
   trap "echo $tmpfile" EXIT   # ❌ trap O'RNATILGAN paytda almashtiriladi
   trap 'echo $tmpfile' EXIT   # ✓ trap CHAQIRILGAN paytda
   ```
   Bu nozik farq! `cleanup` funksiya orqali yozish xavfsizroq.

6. **`exit` ni cleanup ichida unutish.**
   ```bash
   cleanup() { rm -f "$tmp"; }     # OK — EXIT trap holatida
   trap 'cleanup; exit 130' INT    # ✓ exit ham kerak, aks holda davom etadi
   ```

7. **Lock fayl tozalanmagan stale holatda.**
   Skript crash bo'lsa, lock qoladi. Har doim `kill -0 $pid` bilan tekshiring va kerak bo'lsa olib tashlang.
:::

---

## 4.14. Mashqlar

> 🧪 Kelajakda `bashlings watch 09_traps` paketida.

1. **Countdown** — 10 dan 1 gacha har soniyada chiqaruvchi skript yozing. `Ctrl+C` bosilsa "uzr, davom eta olmadi" deb chiqarsin va exit 130 qaytarsin.

2. **`tmp-safe`** — `mktemp` orqali fayl yaratadi, ichiga 5 ta tasodifiy son yozadi, har holatda o'chirib tashlaydi. Cleanup EXIT trap orqali bo'lsin.

3. **Single instance** — lock fayl orqali ikkinchi nusxa ishga tushishini taqiqlovchi skript. Stale lock'ni avtomatik aniqlasin.

4. **Parallel workers** — 3 ta background worker ishga tushiruvchi parent skript. `Ctrl+C` da hammasini o'ldirib chiqsin (`jobs -p | xargs kill`).

5. **ERR trace** — `set -e` va `trap '... $LINENO $BASH_COMMAND ...' ERR` ishlatib, xato sodir bo'lganda qator raqami va buyruqni chiqaruvchi skript yozing.

---

## 4.15. Xulosa

| Tushuncha               | Asosiy nuqta                                       |
|-------------------------|----------------------------------------------------|
| Signal nima             | Jarayonlar o'rtasidagi asinxron xabar              |
| `SIGINT` (2)            | Ctrl+C — eng ko'p uchraydigan                      |
| `SIGTERM` (15)          | Default `kill` — odob bilan to'xtatish so'rovi     |
| **`SIGKILL` (9)**       | **Trap qilib bo'lmaydi** — cleanup ham yo'q       |
| `EXIT`                  | Bash pseudo-signal — **eng muhim cleanup nuqtasi** |
| `trap kod SIGNAL`       | Signalga reaksiya o'rnatish                        |
| `trap - SIGNAL`         | Default'ga qaytarish                               |
| `trap '' SIGNAL`        | E'tibor bermay qo'yish                             |
| `trap -p`               | Joriy trap'lar ro'yxati                            |
| Exit code 130           | Ctrl+C natijasi (128+SIGINT)                       |
| Exit code 143           | kill natijasi (128+SIGTERM)                        |
| `kill -0 PID`           | Jarayon tirikligini tekshirish                     |
| `$!`                    | Eng oxirgi background PID                          |

### 5 ta asosiy g'oya

1. **Hech qachon "muvaffaqiyatli" deb umid qilmang.** Cleanup'ni `trap EXIT` bilan kafolatlang.
2. **EXIT pseudo-signal** — bash dunyosining eng kuchli xususiyatlaridan biri. Foydalaning.
3. **`SIGKILL` ushlab bo'lmaydi.** Kritik holatlar uchun tashqi qo'riqchi kerak.
4. **Single instance + lock + kill -0** — production skriptlarning standart pattern.
5. **Graceful shutdown** — `while [[ $running -eq 1 ]]` modeli bilan har iteratsiyani tugatib chiqing.

🎉 Endi sizning skriptlaringiz har qanday vaziyatda toza yakunlanadi. Keyingi va **oxirgi bobda** biz **`set -euo pipefail`**, **ShellCheck** va **`getopts`** orqali production-grade skript yozishni o'rganamiz.

> **Keyingi sahifa:** [5. Robust skriptlar →](./05-robust-scripting)
