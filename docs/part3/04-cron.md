---
title: "Cron va vazifalarni rejalashtirish"
description: "crontab sintaksisi, at, systemd timers, flock idempotent skriptlar va production monitoring patternlari."
---

# 4. Cron va vazifalarni rejalashtirish

> **🎯 Bu bobda nimani o'rganasiz:**
> - **`cron`** asoslari — `crontab`, 5-maydon syntax
> - **PATH muammosi** — bashning eng ko'p uchraydigan cron tuzog'i
> - `at` — bir martalik kechiktirilgan vazifa
> - **`systemd` timers** — zamonaviy muqobil
> - **`flock`** — bir nusxa kafolati uchun production pattern
> - Real misol — **Nightly backup** notification bilan
>
> **⏱ Vaqt:** ~25 daqiqa
> **🧪 Mashqlar:** `bashlings watch 14_cron` (kelajak sprint)

---

## 4.1. Nima uchun rejalashtirish kerak?

Server'ning kundalik avtomatik ishlari:

| Vazifa                    | Tipik vaqt   | Vosita              |
|---------------------------|--------------|---------------------|
| Database backup           | har kecha    | cron / systemd      |
| Log rotation              | har kun      | logrotate (cron)    |
| SSL sertifikat yangilash  | oyiga 2 marta| certbot (cron)      |
| Cache tozalash            | har 15 daqiqa| cron                |
| Health check & alert      | har 5 daqiqa | cron / monitor      |
| Index rebuild             | dam olishlarda| cron               |

::: tip Asosiy g'oya
Inson ishtirokisiz, ishonchli, doimiy bajariladigan har vazifa — **rejalashtirilgan vazifa**. Mantra: "**Agar oyiga ikkidan ko'p marta qilsang — avtomatlashtir.**"
:::

---

## 4.2. `cron` asoslari

`cron` — Unix tizimlarining klassik scheduler'i (1975-yildan beri).

### Asosiy buyruqlar

```bash
crontab -l          # joriy ro'yxat
crontab -e          # tahrirlash (default editor ochiladi)
crontab -r          # ⚠ HAMMASINI o'chirish — diqqat!
crontab my.cron     # fayldan import
crontab -u ali -l   # boshqa foydalanuvchi cron'i (root sifatida)
```

### Joylashuvlar

```
~/                          ← foydalanuvchi cron, `crontab -e` orqali
/var/spool/cron/<user>      ← cron'lar shu yerda saqlanadi (root only)
/etc/crontab                ← system-wide cron (user maydoni qo'shimcha)
/etc/cron.d/                ← qo'shimcha cron fayllar
/etc/cron.daily/            ← har kun ishga tushiriladigan skriptlar
/etc/cron.hourly/
/etc/cron.weekly/
/etc/cron.monthly/
```

::: warning `crontab -r` — diqqat!
`-r` flagi — **butun cron jadvalingizni** so'rovsiz o'chiradi. **`-i` flagi yo'q!**

Backup qilib qo'ying:
```bash
crontab -l > ~/cron-backup-$(date +%F).txt
```
:::

---

## 4.3. Cron syntax — 5 ta maydon

```
*  *  *  *  *  buyruq
│  │  │  │  │
│  │  │  │  └── hafta kuni      (0-7,  0 va 7 = yakshanba)
│  │  │  └───── oy              (1-12)
│  │  └──────── kun             (1-31)
│  └─────────── soat            (0-23)
└────────────── daqiqa          (0-59)
```

### Maxsus belgilar

| Belgi    | Ma'nosi                       | Misol                          |
|----------|-------------------------------|--------------------------------|
| `*`      | har qiymat                    | `* * * * *` — har daqiqa       |
| `,`      | ro'yxat                       | `0,15,30,45 * * * *` — 15 daqiqa'da bir |
| `-`      | diapazon                      | `9-17` — soat 9 dan 17 gacha   |
| `/`      | step                          | `*/5` — har 5 daqiqada         |
| `*/N`    | step bilan                    | `*/15 * * * *`                 |

### Real misollar

```cron
# Har daqiqa
* * * * * /opt/heartbeat.sh

# Har 5 daqiqada
*/5 * * * * /opt/healthcheck.sh

# Har soat 0-daqiqasida
0 * * * * /opt/log-rotate.sh

# Har kun 02:00
0 2 * * * /opt/backup.sh

# Har yakshanba kecha 03:30
30 3 * * 0 /opt/weekly-report.sh

# Ish kuni 9-17 oraliqda har soat (1-5 = dushanba-juma)
0 9-17 * * 1-5 /opt/business-check.sh

# Har oyning 1-kunida 00:30
30 0 1 * * /opt/monthly-billing.sh

# Har 3 soatda
0 */3 * * * /opt/cache-refresh.sh

# Har 30 daqiqada, faqat ish kuni
*/30 * * * 1-5 /opt/sync.sh
```

### Maxsus stringlar — `@`

| String     | Ekvivalent     | Mazmuni                  |
|------------|----------------|--------------------------|
| `@reboot`  | (yuklanganda)  | Tizim yuklanganida bir marta |
| `@yearly`  | `0 0 1 1 *`    | Yilda bir marta          |
| `@monthly` | `0 0 1 * *`    | Oyda bir marta           |
| `@weekly`  | `0 0 * * 0`    | Haftada bir (yakshanba)  |
| `@daily`   | `0 0 * * *`    | Kuniga bir               |
| `@hourly`  | `0 * * * *`    | Soatiga bir              |

```cron
@reboot   /opt/startup.sh
@daily    /opt/backup.sh
@hourly   /opt/cleanup.sh
```

::: tip Crontab generator
Murakkab sintaksis uchun — [crontab.guru](https://crontab.guru) — interaktiv online tahlil. Ishonchli vosita.
:::

---

## 4.4. Cron environment va PATH muammosi

**Eng ko'p uchraydigan cron xato:** "Skript terminalda ishlaydi, cron'da yo'q".

Sababi — cron muhitida **PATH minimal**:

```cron
# Sodda ko'rinadi, lekin xato:
* * * * * mybackup.sh
# cron: `mybackup` topilmadi (PATH'da yo'q)
```

### Cron PATH default

Aksariyat tizimda:
```
PATH=/usr/bin:/bin
```

Yo'q: `/usr/local/bin`, `~/bin`, `~/.cargo/bin`, `~/.local/bin` va h.k.

### Yechim — 3 yo'l

**1. To'liq yo'l ishlatish (eng aniq):**
```cron
* * * * * /usr/local/bin/mybackup.sh
```

**2. PATH'ni cron faylida o'rnatish:**
```cron
PATH=/usr/local/bin:/usr/bin:/bin
SHELL=/bin/bash

* * * * * mybackup.sh
```

**3. Skript boshida PATH yuklash:**
```bash
#!/usr/bin/env bash
source ~/.bashrc   # yoki aniq:
export PATH="/usr/local/bin:/usr/bin:/bin"

# ... ish ...
```

::: warning Login shell vs Cron shell
Cron — **interactive bo'lmagan** shell. `~/.bashrc`'da `[[ $- == *i* ]] && return` bo'lsa, sozlamalar yuklanmaydi.

Test qilish:
```bash
* * * * * env > /tmp/cron-env.txt
```
Bir daqiqa kuting, keyin `cat /tmp/cron-env.txt` — cron muhitini ko'rasiz.
:::

---

## 4.5. Logging va debugging

Default'da cron output **mail orqali** foydalanuvchiga yuboriladi. Agar mail sozlanmagan bo'lsa — **yo'qoladi**.

### Yaxshi yo'l — har doim faylga yo'naltirish

```cron
# Stdout va stderr ikkalasini bir faylga
0 2 * * * /opt/backup.sh >> /var/log/backup.log 2>&1

# Stdout va stderr alohida
0 2 * * * /opt/backup.sh > /var/log/backup.out 2>> /var/log/backup.err

# Hammasini yo'q qilish (umuman istamasangiz)
0 2 * * * /opt/silent.sh > /dev/null 2>&1
```

::: tip `MAILTO` o'chirish
Mail tashvishidan qutilish uchun crontab boshida:
```cron
MAILTO=""

0 2 * * * /opt/backup.sh >> /var/log/backup.log 2>&1
```

Yoki aniq emailga:
```cron
MAILTO="admin@example.com"
```
:::

### Tizim cron loglari

```bash
# Ubuntu / Debian
tail -f /var/log/syslog | grep CRON

# RHEL / CentOS
tail -f /var/log/cron

# macOS
log show --predicate 'process == "cron"' --last 1h
```

### Skript debug rejimi

```bash
#!/usr/bin/env bash
set -euo pipefail

# Cron'da ishlasa, log'ga yozish
exec >> /var/log/myscript.log 2>&1
echo "===== $(date) =====" 

# ... asosiy ish ...
```

`exec >> file 2>&1` — skriptning **butun keyingi chiqishini** faylga yo'naltiradi.

---

## 4.6. `at` — bir martalik kechiktirilgan vazifa

`cron` — takroriy. `at` — **bitta marta** kelajakdagi vaqtda bajarish.

```bash
# Bugun 18:00 da
echo "/opt/notify.sh" | at 18:00

# Ertaga
echo "/opt/task.sh" | at 09:00 tomorrow

# 1 soatdan keyin
echo "/opt/reminder.sh" | at now + 1 hour

# Aniq sana
echo "/opt/event.sh" | at 14:30 2026-06-01

# Interaktiv (ko'p qator)
at 22:00
> notify-send "Vaqt boldi"
> /opt/cleanup.sh
> Ctrl+D
```

### Boshqaruv

```bash
atq           # kutilayotgan vazifalar
# 5    Sat May 17 18:00:00 2026 a ali

atrm 5        # bekor qilish

at -c 5       # ichidagi buyruqlarni ko'rish
```

::: tip Cron vs at — qachon qaysi
- **Cron** — takroriy: backup, monitoring, cleanup
- **`at`** — bir martalik: "ertaga ertalab serverni qayta yuklash"
:::

---

## 4.7. `systemd` timers — zamonaviy muqobil

Modern Linux distributivlarida `systemd` mavjud — `cron`'dan ko'p afzalliklarga ega.

### Tuzilish — `.service` + `.timer`

**`/etc/systemd/system/backup.service`:**
```ini
[Unit]
Description=Nightly backup

[Service]
Type=oneshot
ExecStart=/opt/backup.sh
StandardOutput=journal
StandardError=journal
```

**`/etc/systemd/system/backup.timer`:**
```ini
[Unit]
Description=Nightly backup timer
Requires=backup.service

[Timer]
OnCalendar=daily        # = soat 00:00
OnCalendar=*-*-* 02:00:00  # aniq 02:00
Persistent=true         # missed run'ni yetib olish
RandomizedDelaySec=30m  # 0-30 daqiqa tasodifiy kechiktirish

[Install]
WantedBy=timers.target
```

### Yoqish va boshqarish

```bash
# Reload (yangi fayl qo'shgandan keyin)
sudo systemctl daemon-reload

# Yoqish va boshlash
sudo systemctl enable --now backup.timer

# Status
systemctl status backup.timer
systemctl list-timers --all

# Loglar
journalctl -u backup.service       # service log
journalctl -u backup.service --since "1 hour ago"

# Manual ishga tushirish (test)
sudo systemctl start backup.service
```

### `OnCalendar=` syntaxisi

```ini
OnCalendar=hourly                       # har soat
OnCalendar=daily                        # har kun 00:00
OnCalendar=weekly                       # har yakshanba
OnCalendar=monthly                      # har oyning 1-kuni
OnCalendar=Mon..Fri 09:00               # ish kuni 9:00
OnCalendar=*-*-* 02:00:00               # har kun 02:00
OnCalendar=*:0/15                       # har 15 daqiqada
OnCalendar=Sat,Sun 04:00                # dam olishlarda
```

::: tip `systemd-analyze calendar`
```bash
systemd-analyze calendar "Mon..Fri 09:00"
# Original form: Mon..Fri 09:00
# Normalized form: Mon..Fri *-*-* 09:00:00
# Next elapse: Mon 2026-05-19 09:00:00 +05
```
Buyurtma'ngiz to'g'riligini avval tekshiring.
:::

---

## 4.8. `cron` vs `systemd` timers — qiyosiy

| Xususiyat                       | `cron`           | `systemd` timer       |
|---------------------------------|------------------|------------------------|
| Yosh                            | 1975             | 2010                   |
| Universal Unix                  | ✅               | ❌ (faqat systemd-li Linux) |
| macOS / *BSD                    | ✅               | ❌                     |
| Logging integratsiya            | qo'lda           | ✅ journalctl          |
| Missed run yetib olish          | ❌               | ✅ `Persistent=true`   |
| Sandboxing (security)           | ❌               | ✅ to'liq              |
| Resource limits (`MemoryMax`)   | ❌               | ✅                     |
| Dependency'lar (boshqa service) | ❌               | ✅ `Requires=`         |
| Sodda holatlar                  | ✅               | overkill               |
| Kompleks workflow               | qiyin            | ✅ tabiiy              |

::: tip Qoida
- **Bitta server, sodda vazifa** → `cron`
- **Production xizmatlar** → `systemd` timers
- **macOS** → `cron` (yoki `launchd`)
:::

::: warning macOS — `launchd`
macOS'da `cron` rasmiy **deprecated** (lekin hali ishlaydi). Tavsiya etilgan vosita — `launchd` (`.plist` fayllar `~/Library/LaunchAgents/`).

Sodda holatlar uchun cron ishlatish mumkin. Lekin macOS'da `cron` ba'zan **Full Disk Access** ruxsatisiz ishlamaydi — System Preferences orqali berishingiz kerak bo'lishi mumkin.
:::

---

## 4.9. `flock` — bir nusxa kafolati

Eng klassik cron muammo: skript 1 daqiqada bajarilishi kerak, lekin 2 daqiqa ishlamoqda. Keyingi tick'da yana ishga tushadi — **ikki nusxa parallel**. Disk yoki database ustida poyga.

**Yechim:** `flock` bilan lock fayl.

```cron
* * * * * flock -n /tmp/myjob.lock /opt/myjob.sh
```

`flock -n` — lock olib bo'lmasa, **darhol chiqadi** (xato bilan). Ikkinchi instance ishga tushmaydi.

### Skript ichida

```bash
#!/usr/bin/env bash
exec 200>/var/run/myjob.lock
flock -n 200 || { echo "Allaqachon ishlamoqda"; exit 1; }

# ... asosiy ish ...
```

### `flock` flag'lar

| Flag         | Mazmuni                                  |
|--------------|------------------------------------------|
| `-n`         | Lock olib bo'lmasa darhol chiqish        |
| `-w <sec>`   | N soniya kutib, keyin xato qaytarish     |
| `-x`         | Exclusive lock (default)                 |
| `-s`         | Shared lock                              |
| `-u`         | Unlock (qo'lda)                          |

```cron
# 30s kutib, keyin xato bersa, log'ga yozib chiqarish
* * * * * flock -w 30 /tmp/myjob.lock /opt/myjob.sh >> /var/log/myjob.log 2>&1
```

---

## 4.10. Real misol — Production-grade nightly backup

```bash
#!/usr/bin/env bash
#
# nightly-backup.sh — production backup with notifications
#
# crontab entry:
#   0 2 * * * flock -n /tmp/backup.lock /opt/bin/nightly-backup.sh
#

set -euo pipefail
IFS=$'\n\t'

# === Konfiguratsiya ===
readonly SCRIPT_NAME="$(basename "$0")"
readonly LOG_FILE="/var/log/backup.log"
readonly SLACK_WEBHOOK="${SLACK_WEBHOOK:-}"
readonly HEALTHCHECK_URL="${HEALTHCHECK_URL:-}"

readonly BACKUP_DIR="/var/backups"
readonly SOURCE_DIRS=("/var/www" "/etc")
readonly KEEP_DAYS=14

# === Logging ===
exec >> "$LOG_FILE" 2>&1
echo ""
echo "===== $(date '+%Y-%m-%d %H:%M:%S') ====="

log() { printf '[%s] %s\n' "$(date '+%H:%M:%S')" "$*"; }

# === Notification ===
notify_slack() {
    [[ -z "$SLACK_WEBHOOK" ]] && return 0
    local msg="$1"
    curl -fsS -X POST -H 'Content-Type: application/json' \
        -d "{\"text\":\"$msg\"}" \
        "$SLACK_WEBHOOK" > /dev/null
}

# === Cleanup va xato handling ===
cleanup() {
    local rc=$?
    if [[ $rc -ne 0 ]]; then
        log "❌ Backup xato bilan tugadi (exit=$rc)"
        notify_slack ":x: Backup failed on $(hostname) (exit=$rc, see $LOG_FILE)"
    fi
}
trap cleanup EXIT

# === Asosiy ish ===
log "📦 Backup boshlandi"
mkdir -p "$BACKUP_DIR"

archive="$BACKUP_DIR/backup_$(date +%Y%m%d_%H%M%S).tar.gz"
log "📁 Arxiv: $archive"

if ! tar -czf "$archive" "${SOURCE_DIRS[@]}" 2>&1; then
    log "❌ tar yiqildi"
    exit 1
fi

size=$(du -sh "$archive" | cut -f1)
log "✅ Arxiv tayyor: $size"

# === Eski backup'larni tozalash ===
log "🧹 ${KEEP_DAYS} kundan eski'larni o'chirish..."
find "$BACKUP_DIR" -name "backup_*.tar.gz" -mtime "+$KEEP_DAYS" -delete

# === Healthcheck ping ===
if [[ -n "$HEALTHCHECK_URL" ]]; then
    curl -fsS --max-time 10 "$HEALTHCHECK_URL" > /dev/null \
        && log "💚 Healthcheck ping yuborildi"
fi

# === Success notification ===
notify_slack ":white_check_mark: Backup OK on $(hostname) — size: $size"
log "🎉 Yakunlandi"
```

### Crontab entry

```cron
SHELL=/bin/bash
PATH=/usr/local/bin:/usr/bin:/bin
MAILTO=""
SLACK_WEBHOOK=https://hooks.slack.com/services/...
HEALTHCHECK_URL=https://hc-ping.com/uuid-here

# Har kun 02:00 — lock bilan, single instance
0 2 * * * flock -n /tmp/backup.lock /opt/bin/nightly-backup.sh
```

### Bu skript nima qiladi?

| Xususiyat                       | Qaerda                                |
|---------------------------------|----------------------------------------|
| Lock bilan single instance      | `crontab`'da `flock -n`                |
| Markaziy log fayli              | `exec >> "$LOG_FILE" 2>&1`             |
| Slack notification (success+fail)| `notify_slack`                        |
| Healthcheck.io ping              | `curl HEALTHCHECK_URL`                |
| Eski backup'larni avto-tozalash | `find ... -mtime +N -delete`           |
| Trap orqali xato handle          | `trap cleanup EXIT`                    |
| Production-grade error reporting | exit code → Slack alert                |

---

## 4.11. Tez-tez uchraydigan xatolar

::: danger Klassik tuzoqlar

1. **PATH muammosi.**
   Skript terminalda ishlaydi, cron'da yo'q. Yechim: to'liq yo'l yoki `PATH=...` crontab boshida.

2. **Output yo'qoladi.**
   Default'da cron output mailga ketadi. Yechim: `>> file 2>&1` har doim.

3. **`MAILTO` sozlanmagan, har daqiqa mail spam.**
   `MAILTO=""` crontab boshida.

4. **Cron'da `~` ishlamaydi.**
   `~` interactive shell uchun. Cron'da `$HOME` yoki to'liq yo'l ishlating.

5. **`crontab -r` bilan adashish.**
   `-r` = remove (hammasini). `-e` = edit. Tasodifan o'chirsangiz — yo'qotdingiz.

6. **Time zone — UTC vs local.**
   Server odatda UTC'da. Yangi server'da `timedatectl status` bilan tekshiring.

7. **`%` belgisi cron'da maxsus.**
   Cron qatorida `%` — yangi qator. Aniq belgi kerak bo'lsa, escape qiling: `\%`. Yaxshiroq — `date +"%F"` ni alohida skriptga o'tkazing.

8. **Single instance kafolati yo'q.**
   `flock` ishlatmagan bo'lsangiz — uzun ish 2 marta parallel ishlashi mumkin.

9. **macOS'da cron Full Disk Access.**
   System Preferences → Security & Privacy → Full Disk Access — `cron` qo'shing.

10. **Cron faylida xato yangilanish tushunmaslik.**
    `crontab -e` orqali save qilganda — darhol amal qiladi. Lekin SIGTERM yuborilmaydi: ishlovchi vazifa eski sintaksisni ishlatadi.
:::

---

## 4.12. Mashqlar

> 🧪 Kelajakda `bashlings watch 14_cron` paketida.

1. **Cron syntax parser** — quyidagi cron qatorlarini "har soat 30-daqiqasi" yoki "ish kuni 9-17 har 30 daqiqa" deb tushuntirish bo'yicha quiz:
   - `30 * * * *`
   - `*/30 9-17 * * 1-5`
   - `0 0 1 * *`

2. **Logging wrap** — quyidagi cron qatorni log + MAILTO bilan to'g'ri ko'rinishga keltiring:
   ```cron
   * * * * * mybackup.sh
   ```

3. **`flock` lock** — `myjob.sh` skripti uchun cron qatorida `flock -n` qo'shing va lock fayl `/tmp/myjob.lock` bo'lsin.

4. **`at` reminder** — 30 daqiqadan keyin "vaqti" deb chiqaruvchi `at` job yarating va `atq` orqali tekshiring.

5. **Systemd timer** — har 15 daqiqada ping qiluvchi `myping.service` va `myping.timer` faylini yozing.

---

## 4.13. Xulosa

| Tushuncha                   | Asosiy nuqta                                       |
|-----------------------------|----------------------------------------------------|
| `crontab -e`                | Tahrirlash                                         |
| `crontab -l`                | Joriy ro'yxat                                      |
| `crontab -r`                | ⚠ HAMMASINI o'chirish                              |
| **5 maydon**                | `daqiqa soat kun oy hafta-kuni`                    |
| `*/5`                       | Har 5 (step)                                       |
| `@daily`, `@hourly`         | Maxsus stringlar                                   |
| `0 2 * * *`                 | Har kun 02:00                                      |
| `>> log 2>&1`               | Output har doim faylga                             |
| `MAILTO=""`                 | Mail spam'ni o'chirish                             |
| `PATH=...`                  | Cron'da o'rnatish majburiy                         |
| **`flock -n /tmp/x.lock`**  | Single instance kafolati                           |
| `at 18:00`                  | Bir martalik kechiktirish                          |
| `OnCalendar=daily`          | `systemd` timer ekvivalenti                        |
| `journalctl -u <service>`   | `systemd` log'lar                                  |

### 5 ta asosiy g'oya

1. **PATH cron'da minimal** — to'liq yo'l yoki crontab boshida `PATH=...`.
2. **`>> file 2>&1` har doim** — aks holda output yo'qoladi.
3. **`flock -n`** — har production cron qatorida.
4. **`crontab.guru`** — murakkab syntax'ni online tekshiring.
5. **`systemd timer`** — Linux'da production uchun afzal (logging + sandboxing + missed-run catch-up).

🎉 Endi siz **vazifalarni avtomatlashtira olasiz**. Keyingi bobda — **Docker** orqali konteynerlar bilan ishlash.

> **Keyingi sahifa:** [5. Docker bilan integratsiya →](./05-docker)
