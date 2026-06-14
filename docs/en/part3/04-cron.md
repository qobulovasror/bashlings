---
title: "Cron and task scheduling"
description: "crontab syntax, at, systemd timers, flock idempotent scripts, and production monitoring patterns."
---

# 4. Cron and task scheduling

> **🎯 What you'll learn in this chapter:**
> - **`cron`** basics — `crontab`, the 5-field syntax
> - **The PATH problem** — bash's most common cron trap
> - `at` — a one-time delayed task
> - **`systemd` timers** — the modern alternative
> - **`flock`** — the production pattern for a single-instance guarantee
> - A real example — **Nightly backup** with notifications
>
> **⏱ Time:** ~25 minutes
> **🧪 Exercises:** `bashlings watch` — 7 interactive exercises ready ([`exercises/14_cron/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/14_cron))

---

## 4.1. Why do you need scheduling?

A server's daily automated jobs:

| Task                      | Typical time   | Tool                |
|---------------------------|----------------|---------------------|
| Database backup           | every night    | cron / systemd      |
| Log rotation              | every day      | logrotate (cron)    |
| SSL certificate renewal   | twice a month  | certbot (cron)      |
| Cache cleanup             | every 15 minutes| cron               |
| Health check & alert      | every 5 minutes| cron / monitor      |
| Index rebuild             | on weekends    | cron                |

::: tip Core idea
Any task that runs reliably and continuously without human involvement is a **scheduled task**. The mantra: "**If you do it more than twice a month — automate it.**"
:::

---

## 4.2. `cron` basics

`cron` — the classic scheduler of Unix systems (since 1975).

### Core commands

```bash
crontab -l          # current list
crontab -e          # edit (opens the default editor)
crontab -r          # ⚠ delete EVERYTHING — careful!
crontab my.cron     # import from a file
crontab -u ali -l   # another user's cron (as root)
```

### Locations

```
~/                          ← user cron, via `crontab -e`
/var/spool/cron/<user>      ← crons are stored here (root only)
/etc/crontab                ← system-wide cron (with an extra user field)
/etc/cron.d/                ← additional cron files
/etc/cron.daily/            ← scripts run every day
/etc/cron.hourly/
/etc/cron.weekly/
/etc/cron.monthly/
```

::: warning `crontab -r` — careful!
The `-r` flag deletes **your entire cron table** without asking. **There is no `-i` flag!**

Make a backup:
```bash
crontab -l > ~/cron-backup-$(date +%F).txt
```
:::

---

## 4.3. Cron syntax — the 5 fields

```
*  *  *  *  *  command
│  │  │  │  │
│  │  │  │  └── day of week     (0-7,  0 and 7 = Sunday)
│  │  │  └───── month           (1-12)
│  │  └──────── day             (1-31)
│  └─────────── hour            (0-23)
└────────────── minute          (0-59)
```

### Special characters

| Character| Meaning                       | Example                        |
|----------|-------------------------------|--------------------------------|
| `*`      | every value                   | `* * * * *` — every minute     |
| `,`      | list                          | `0,15,30,45 * * * *` — once every 15 minutes |
| `-`      | range                         | `9-17` — hours 9 through 17    |
| `/`      | step                          | `*/5` — every 5 minutes        |
| `*/N`    | with a step                   | `*/15 * * * *`                 |

### Real examples

```cron
# Every minute
* * * * * /opt/heartbeat.sh

# Every 5 minutes
*/5 * * * * /opt/healthcheck.sh

# At minute 0 of every hour
0 * * * * /opt/log-rotate.sh

# Every day at 02:00
0 2 * * * /opt/backup.sh

# Every Sunday night at 03:30
30 3 * * 0 /opt/weekly-report.sh

# Every hour during business hours 9-17 (1-5 = Monday-Friday)
0 9-17 * * 1-5 /opt/business-check.sh

# On the 1st of every month at 00:30
30 0 1 * * /opt/monthly-billing.sh

# Every 3 hours
0 */3 * * * /opt/cache-refresh.sh

# Every 30 minutes, on weekdays only
*/30 * * * 1-5 /opt/sync.sh
```

### Special strings — `@`

| String     | Equivalent     | Meaning                  |
|------------|----------------|--------------------------|
| `@reboot`  | (on boot)      | Once when the system boots |
| `@yearly`  | `0 0 1 1 *`    | Once a year              |
| `@monthly` | `0 0 1 * *`    | Once a month             |
| `@weekly`  | `0 0 * * 0`    | Once a week (Sunday)     |
| `@daily`   | `0 0 * * *`    | Once a day               |
| `@hourly`  | `0 * * * *`    | Once an hour             |

```cron
@reboot   /opt/startup.sh
@daily    /opt/backup.sh
@hourly   /opt/cleanup.sh
```

::: tip Crontab generator
For complex syntax — [crontab.guru](https://crontab.guru) — interactive online analysis. A reliable tool.
:::

---

## 4.4. Cron environment and the PATH problem

**The most common cron error:** "The script works in the terminal but not in cron."

The reason — in the cron environment the **PATH is minimal**:

```cron
# Looks simple, but wrong:
* * * * * mybackup.sh
# cron: `mybackup` not found (not in PATH)
```

### Cron PATH default

On most systems:
```
PATH=/usr/bin:/bin
```

Missing: `/usr/local/bin`, `~/bin`, `~/.cargo/bin`, `~/.local/bin`, etc.

### Solution — 3 ways

**1. Use the full path (the most precise):**
```cron
* * * * * /usr/local/bin/mybackup.sh
```

**2. Set PATH in the cron file:**
```cron
PATH=/usr/local/bin:/usr/bin:/bin
SHELL=/bin/bash

* * * * * mybackup.sh
```

**3. Load PATH at the start of the script:**
```bash
#!/usr/bin/env bash
source ~/.bashrc   # or explicitly:
export PATH="/usr/local/bin:/usr/bin:/bin"

# ... work ...
```

::: warning Login shell vs Cron shell
Cron is a **non-interactive** shell. If `~/.bashrc` has `[[ $- == *i* ]] && return`, the settings won't be loaded.

To test:
```bash
* * * * * env > /tmp/cron-env.txt
```
Wait one minute, then run `cat /tmp/cron-env.txt` — you'll see the cron environment.
:::

---

## 4.5. Logging and debugging

By default, cron output is sent to the user **via mail**. If mail isn't configured — it's **lost**.

### The good way — always redirect to a file

```cron
# Both stdout and stderr to one file
0 2 * * * /opt/backup.sh >> /var/log/backup.log 2>&1

# stdout and stderr separately
0 2 * * * /opt/backup.sh > /var/log/backup.out 2>> /var/log/backup.err

# Discard everything (if you don't want it at all)
0 2 * * * /opt/silent.sh > /dev/null 2>&1
```

::: tip Disabling `MAILTO`
To get rid of the mail nuisance, at the top of the crontab:
```cron
MAILTO=""

0 2 * * * /opt/backup.sh >> /var/log/backup.log 2>&1
```

Or to a specific email:
```cron
MAILTO="admin@example.com"
```
:::

### System cron logs

```bash
# Ubuntu / Debian
tail -f /var/log/syslog | grep CRON

# RHEL / CentOS
tail -f /var/log/cron

# macOS
log show --predicate 'process == "cron"' --last 1h
```

### Script debug mode

```bash
#!/usr/bin/env bash
set -euo pipefail

# When running in cron, write to the log
exec >> /var/log/myscript.log 2>&1
echo "===== $(date) =====" 

# ... main work ...
```

`exec >> file 2>&1` — redirects **all subsequent output** of the script to the file.

---

## 4.6. `at` — a one-time delayed task

`cron` is recurring. `at` runs **once** at a future time.

```bash
# Today at 18:00
echo "/opt/notify.sh" | at 18:00

# Tomorrow
echo "/opt/task.sh" | at 09:00 tomorrow

# In 1 hour
echo "/opt/reminder.sh" | at now + 1 hour

# A specific date
echo "/opt/event.sh" | at 14:30 2026-06-01

# Interactive (multiple lines)
at 22:00
> notify-send "Time's up"
> /opt/cleanup.sh
> Ctrl+D
```

### Management

```bash
atq           # pending tasks
# 5    Sat May 17 18:00:00 2026 a ali

atrm 5        # cancel

at -c 5       # view the commands inside it
```

::: tip Cron vs at — when to use which
- **Cron** — recurring: backup, monitoring, cleanup
- **`at`** — one-time: "reboot the server tomorrow morning"
:::

---

## 4.7. `systemd` timers — the modern alternative

Modern Linux distributions have `systemd` — with many advantages over `cron`.

### Structure — `.service` + `.timer`

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
OnCalendar=daily        # = 00:00
OnCalendar=*-*-* 02:00:00  # exactly 02:00
Persistent=true         # catch up missed runs
RandomizedDelaySec=30m  # random delay of 0-30 minutes

[Install]
WantedBy=timers.target
```

### Enabling and managing

```bash
# Reload (after adding a new file)
sudo systemctl daemon-reload

# Enable and start
sudo systemctl enable --now backup.timer

# Status
systemctl status backup.timer
systemctl list-timers --all

# Logs
journalctl -u backup.service       # service log
journalctl -u backup.service --since "1 hour ago"

# Manual run (test)
sudo systemctl start backup.service
```

### `OnCalendar=` syntax

```ini
OnCalendar=hourly                       # every hour
OnCalendar=daily                        # every day at 00:00
OnCalendar=weekly                       # every Sunday
OnCalendar=monthly                      # on the 1st of every month
OnCalendar=Mon..Fri 09:00               # weekdays at 9:00
OnCalendar=*-*-* 02:00:00               # every day at 02:00
OnCalendar=*:0/15                       # every 15 minutes
OnCalendar=Sat,Sun 04:00                # on weekends
```

::: tip `systemd-analyze calendar`
```bash
systemd-analyze calendar "Mon..Fri 09:00"
# Original form: Mon..Fri 09:00
# Normalized form: Mon..Fri *-*-* 09:00:00
# Next elapse: Mon 2026-05-19 09:00:00 +05
```
Check that your expression is correct first.
:::

---

## 4.8. `cron` vs `systemd` timers — a comparison

| Feature                         | `cron`           | `systemd` timer       |
|---------------------------------|------------------|------------------------|
| Age                             | 1975             | 2010                   |
| Universal Unix                  | ✅               | ❌ (only Linux with systemd) |
| macOS / *BSD                    | ✅               | ❌                     |
| Logging integration             | manual           | ✅ journalctl          |
| Catching up missed runs          | ❌               | ✅ `Persistent=true`   |
| Sandboxing (security)           | ❌               | ✅ full                |
| Resource limits (`MemoryMax`)   | ❌               | ✅                     |
| Dependencies (on other services)| ❌               | ✅ `Requires=`         |
| Simple cases                    | ✅               | overkill               |
| Complex workflows               | hard             | ✅ natural             |

::: tip Rule
- **A single server, a simple task** → `cron`
- **Production services** → `systemd` timers
- **macOS** → `cron` (or `launchd`)
:::

::: warning macOS — `launchd`
On macOS, `cron` is officially **deprecated** (but still works). The recommended tool is `launchd` (`.plist` files in `~/Library/LaunchAgents/`).

For simple cases, you can use cron. But on macOS `cron` sometimes won't work without **Full Disk Access** permission — you may need to grant it via System Preferences.
:::

---

## 4.9. `flock` — a single-instance guarantee

The most classic cron problem: a script is supposed to run in 1 minute, but it's taking 2 minutes. On the next tick it starts again — **two instances in parallel**. A race over the disk or database.

**Solution:** a lock file with `flock`.

```cron
* * * * * flock -n /tmp/myjob.lock /opt/myjob.sh
```

`flock -n` — if it can't acquire the lock, it **exits immediately** (with an error). The second instance won't start.

### Inside the script

```bash
#!/usr/bin/env bash
exec 200>/var/run/myjob.lock
flock -n 200 || { echo "Already running"; exit 1; }

# ... main work ...
```

### `flock` flags

| Flag         | Meaning                                  |
|--------------|------------------------------------------|
| `-n`         | Exit immediately if the lock can't be acquired |
| `-w <sec>`   | Wait N seconds, then return an error     |
| `-x`         | Exclusive lock (default)                 |
| `-s`         | Shared lock                              |
| `-u`         | Unlock (manually)                        |

```cron
# Wait 30s, then if it errors, write it to the log
* * * * * flock -w 30 /tmp/myjob.lock /opt/myjob.sh >> /var/log/myjob.log 2>&1
```

---

## 4.10. A real example — a production-grade nightly backup

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

# === Configuration ===
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

# === Cleanup and error handling ===
cleanup() {
    local rc=$?
    if [[ $rc -ne 0 ]]; then
        log "❌ Backup ended with an error (exit=$rc)"
        notify_slack ":x: Backup failed on $(hostname) (exit=$rc, see $LOG_FILE)"
    fi
}
trap cleanup EXIT

# === Main work ===
log "📦 Backup started"
mkdir -p "$BACKUP_DIR"

archive="$BACKUP_DIR/backup_$(date +%Y%m%d_%H%M%S).tar.gz"
log "📁 Archive: $archive"

if ! tar -czf "$archive" "${SOURCE_DIRS[@]}" 2>&1; then
    log "❌ tar failed"
    exit 1
fi

size=$(du -sh "$archive" | cut -f1)
log "✅ Archive ready: $size"

# === Cleaning up old backups ===
log "🧹 Deleting backups older than ${KEEP_DAYS} days..."
find "$BACKUP_DIR" -name "backup_*.tar.gz" -mtime "+$KEEP_DAYS" -delete

# === Healthcheck ping ===
if [[ -n "$HEALTHCHECK_URL" ]]; then
    curl -fsS --max-time 10 "$HEALTHCHECK_URL" > /dev/null \
        && log "💚 Healthcheck ping sent"
fi

# === Success notification ===
notify_slack ":white_check_mark: Backup OK on $(hostname) — size: $size"
log "🎉 Finished"
```

### Crontab entry

```cron
SHELL=/bin/bash
PATH=/usr/local/bin:/usr/bin:/bin
MAILTO=""
SLACK_WEBHOOK=https://hooks.slack.com/services/...
HEALTHCHECK_URL=https://hc-ping.com/uuid-here

# Every day at 02:00 — with a lock, single instance
0 2 * * * flock -n /tmp/backup.lock /opt/bin/nightly-backup.sh
```

### What does this script do?

| Feature                         | Where                                  |
|---------------------------------|----------------------------------------|
| Single instance with a lock     | `flock -n` in `crontab`                |
| A central log file              | `exec >> "$LOG_FILE" 2>&1`             |
| Slack notification (success+fail)| `notify_slack`                        |
| Healthcheck.io ping              | `curl HEALTHCHECK_URL`                |
| Auto-cleanup of old backups     | `find ... -mtime +N -delete`           |
| Error handling via trap          | `trap cleanup EXIT`                    |
| Production-grade error reporting | exit code → Slack alert                |

---

## 4.11. Common mistakes

::: danger Classic traps

1. **The PATH problem.**
   The script works in the terminal but not in cron. Solution: the full path, or `PATH=...` at the top of the crontab.

2. **Output is lost.**
   By default, cron output goes to mail. Solution: `>> file 2>&1` always.

3. **`MAILTO` not configured, mail spam every minute.**
   `MAILTO=""` at the top of the crontab.

4. **`~` doesn't work in cron.**
   `~` is for an interactive shell. In cron, use `$HOME` or the full path.

5. **Confusing `crontab -r`.**
   `-r` = remove (everything). `-e` = edit. If you delete by accident — you've lost it.

6. **Time zone — UTC vs local.**
   The server is usually in UTC. On a new server, check with `timedatectl status`.

7. **The `%` character is special in cron.**
   In a cron line, `%` means a newline. If you need a literal character, escape it: `\%`. Better — move `date +"%F"` into a separate script.

8. **No single-instance guarantee.**
   If you don't use `flock` — a long-running job can run twice in parallel.

9. **macOS cron Full Disk Access.**
   System Preferences → Security & Privacy → Full Disk Access — add `cron`.

10. **Not understanding how a faulty cron file update behaves.**
    When you save via `crontab -e` — it takes effect immediately. But no SIGTERM is sent: a running task keeps using the old syntax.
:::

---

## 4.12. Exercises

::: tip 🧪 Bashlings — interactive exercises
This chapter's **7** exercises come with auto-checking via the `bashlings` CLI. None
require a cron daemon — they work with syntax and parsing:

```bash
bashlings watch              # start from the first pending exercise
bashlings run cron1          # check a single exercise
bashlings hint cron1         # step-by-step hint
```

Source: [`exercises/14_cron/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/14_cron)
:::

Also try the following additional conceptual exercises:

1. **Cron syntax parser** — a quiz on explaining the following cron lines as "minute 30 of every hour" or "every 30 minutes during business hours 9-17":
   - `30 * * * *`
   - `*/30 9-17 * * 1-5`
   - `0 0 1 * *`

2. **Logging wrap** — convert the following cron line into the correct form with logging + MAILTO:
   ```cron
   * * * * * mybackup.sh
   ```

3. **`flock` lock** — add `flock -n` to the cron line for the `myjob.sh` script, with the lock file being `/tmp/myjob.lock`.

4. **`at` reminder** — create an `at` job that prints "time's up" after 30 minutes and check it with `atq`.

5. **Systemd timer** — write a `myping.service` and `myping.timer` file that pings every 15 minutes.

---

## 4.13. Summary

| Concept                     | Key point                                          |
|-----------------------------|----------------------------------------------------|
| `crontab -e`                | Edit                                               |
| `crontab -l`                | Current list                                       |
| `crontab -r`                | ⚠ Delete EVERYTHING                                |
| **5 fields**                | `minute hour day month day-of-week`               |
| `*/5`                       | Every 5 (step)                                     |
| `@daily`, `@hourly`         | Special strings                                    |
| `0 2 * * *`                 | Every day at 02:00                                 |
| `>> log 2>&1`               | Output always to a file                            |
| `MAILTO=""`                 | Turn off mail spam                                 |
| `PATH=...`                  | Mandatory to set in cron                           |
| **`flock -n /tmp/x.lock`**  | Single-instance guarantee                          |
| `at 18:00`                  | A one-time delay                                   |
| `OnCalendar=daily`          | The `systemd` timer equivalent                     |
| `journalctl -u <service>`   | `systemd` logs                                     |

### 5 key ideas

1. **PATH is minimal in cron** — use the full path or `PATH=...` at the top of the crontab.
2. **`>> file 2>&1` always** — otherwise output is lost.
3. **`flock -n`** — on every production cron line.
4. **`crontab.guru`** — check complex syntax online.
5. **`systemd timer`** — preferred for production on Linux (logging + sandboxing + missed-run catch-up).

🎉 Now you **can automate tasks**. In the next chapter — working with containers via **Docker**.

> **Next page:** [5. Integration with Docker →](./05-docker)
