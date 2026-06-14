---
title: "Signals and traps"
description: "Unix signals, trap, the EXIT pseudo-signal, cleanup pattern, graceful shutdown, and background processes."
---

# 4. Signals and traps

> **🎯 What you will learn in this chapter:**
> - **Unix signals** — SIGINT, SIGTERM, SIGKILL, SIGHUP, and others
> - Catching signals with **`trap`**
> - **The `EXIT` pseudo-signal** — the most important cleanup mechanism
> - **Cleanup pattern** — temp files, lock files, connections
> - **Graceful shutdown** — clean termination even when Ctrl+C is pressed
> - **Background processes** — `&`, `wait`, signal propagation
>
> **⏱ Time:** ~25 minutes
> **🧪 Exercises:** `bashlings watch` — 5 interactive exercises ready ([`exercises/09_traps/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/09_traps))

---

## 4.1. Why do you need this?

Imagine the following situations:

1. Your script was writing data to a 100MB temporary file. The user pressed **`Ctrl+C`**. The file was left behind — it cluttered the disk.

2. A database backup script was running. The terminal was closed. A half-finished backup and an open connection were left hanging.

3. A server-monitoring script was running under `cron`. The system rebooted. The PID file remained, and the next run incorrectly reported "already running."

**All** of these can be prevented with `trap` and signals.

::: tip Core idea
**Never hope that things "finished successfully."** Your script can be stopped **at any time** — `Ctrl+C`, the terminal closing, the OOM killer, `kill -9`. **You must guarantee cleanup.**
:::

---

## 4.2. Unix signals — a brief model

A **signal** is an **asynchronous message** from one process to another (or from the kernel). The user's `Ctrl+C` is a signal. The `kill` command sends a signal. Controlling how the system runs — done through signals.

### Signals — each has a number and a name

```bash
kill -l
# 1) SIGHUP   2) SIGINT   3) SIGQUIT  ...
# 9) SIGKILL  15) SIGTERM ...
```

### Signals you'll need

| Signal      | Number  | Meaning                                       | Can it be trapped? |
|-------------|---------|-----------------------------------------------|------------------------|
| `SIGINT`    | 2       | **Ctrl+C** — a polite request to stop         | ✅                     |
| `SIGTERM`   | 15      | "Please stop" (default `kill PID`)            | ✅                     |
| `SIGHUP`    | 1       | Terminal closed (hangup)                      | ✅                     |
| `SIGQUIT`   | 3       | **Ctrl+\\** — stop with a core dump           | ✅                     |
| `SIGUSR1`   | 10/30   | User-defined #1                               | ✅                     |
| `SIGUSR2`   | 12/31   | User-defined #2                               | ✅                     |
| `SIGCHLD`   | 17/20   | A child process changed state                 | ✅                     |
| `SIGPIPE`   | 13      | The pipe being written to was closed          | ✅                     |
| `SIGALRM`   | 14      | A timer fired                                 | ✅                     |
| **`SIGKILL`** | **9** | **Kill immediately — you cannot resist it**   | ❌ **NO**            |
| **`SIGSTOP`** | 19/17 | **Pause immediately — you cannot resist it**  | ❌ **NO**            |
| **`EXIT`**  | (0)     | **Bash pseudo-signal** — on any kind of exit  | ✅ (most important)     |

::: warning `SIGKILL` can never be caught
`kill -9 PID` kills the process **immediately**. Cleanup code does not run. That is why, for critical situations, you need "external guardian" processes.
:::

---

## 4.3. `kill` — sending a signal

```bash
# Default — sends SIGTERM (15)
kill 1234

# A specific signal
kill -SIGINT 1234
kill -INT 1234            # you can drop the SIG prefix
kill -2 1234              # by number

# The strongest — cannot be resisted
kill -9 1234
kill -KILL 1234

# By name (process name)
killall my-script.sh
pkill -f "long pattern"

# PID of the current process
echo $$                   # for bash
```

### `kill -0` — check only

```bash
# Is the process alive?
if kill -0 1234 2>/dev/null; then
    echo "Hali ishlayapti"
fi
```

`-0` does not send a signal; it only checks permissions.

---

## 4.4. `trap` basics

`trap` defines what to do when a signal is received.

### Syntax

```bash
trap '<code to run>' SIGNAL [SIGNAL ...]
```

### First example

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

Run it and press `Ctrl+C` — the script warns you and keeps going.

### Viewing traps

```bash
trap -p                   # list of currently set traps
trap -p SIGINT            # only for SIGINT
```

### Removing a trap (restoring the default)

```bash
trap - SIGINT             # SIGINT default behavior (kill)
trap - EXIT INT TERM      # several together
```

### Ignoring a trap entirely

```bash
trap '' SIGINT            # Ctrl+C does nothing at all
```

::: danger Ignoring is dangerous
`trap '' SIGINT` cancels the user's `Ctrl+C`. This is questionable UX. Use it only in **special, documented** situations.
:::

---

## 4.5. `EXIT` — the most important pseudo-signal

`EXIT` is not a real Unix signal; it is bash's **special pseudo-signal**. It runs whenever the script ends **by any means**:

- Normal termination (the last command ran)
- The `exit` command was called
- It stopped with an error (under `set -e`)
- A signal was received (`SIGINT`, `SIGTERM`)

::: tip A win for cleanup
This very property makes `EXIT` **the perfect place for cleanup**. You write it once — and it runs in every case.
:::

### The classic pattern

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

Try it:
- Finish normally → cleanup runs
- Press `Ctrl+C` → cleanup runs
- Add `exit 1` → cleanup runs

---

## 4.6. The best cleanup pattern

Most scripts catch `Ctrl+C` with a separate message and use EXIT for cleanup. This is the cleanest model:

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

# 1) EXIT — cleanup in every case
trap cleanup EXIT

# 2) INT / TERM — graceful exit (the EXIT trap also runs)
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

### What runs when?

| Event                 | INT trap | EXIT trap | Cleanup called? |
|-----------------------|----------|-----------|---------------------|
| Normal termination    | —        | ✅        | **Yes**             |
| `Ctrl+C`              | ✅ → `exit 130` | ✅ | **Yes**         |
| `kill PID`            | TERM → `exit 130` | ✅ | **Yes**          |
| `kill -9 PID`         | ❌ not caught | ❌ | **No** ⚠         |
| `exit 1` inside script| —        | ✅        | **Yes**             |

::: tip Exit code convention
- `130` = `128 + SIGINT(2)` — the result of Ctrl+C
- `143` = `128 + SIGTERM(15)` — the result of kill

This is a Unix convention. It is useful for programmatic checks.
:::

---

## 4.7. Multiple cleanups and the `done` flag

If you need to guard against `cleanup` being called twice (for example, when `Ctrl+C` is followed by `exit 1` as well):

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

This is the idempotent cleanup pattern.

---

## 4.8. `SIGKILL` — you cannot resist it

```bash
trap 'echo "haa"' SIGKILL   # ❌ bash xato beradi
```

Bash will not even allow you to write this. The reason: `SIGKILL` and `SIGSTOP` are kernel-level signals, and a user process cannot block them by any means.

### The real rules

- **`kill PID`** (SIGTERM) — a polite request. The script can clean up and exit.
- **`kill -9 PID`** (SIGKILL) — using force. Cleanup does not run.

::: tip Production behavior
1. **First approach:** a polite `SIGTERM`
2. **Wait a few seconds** (for cleanup)
3. **If nothing happens:** `SIGKILL` (last resort)

`systemctl stop` works exactly on this model.
:::

---

## 4.9. Background processes and `wait`

In scripts that work asynchronously — background processes launched with `&` — special attention is needed.

### `&` and `$!`

```bash
sleep 30 &
echo "Background PID: $!"     # the most recent background PID
```

### `wait` — waiting

```bash
sleep 5 &
pid1=$!

sleep 3 &
pid2=$!

wait $pid1 $pid2
echo "Ikkalasi ham tugadi"
```

### Signal propagation

By default, if the parent script receives `Ctrl+C`, the child processes **are not killed automatically**. You have to direct it explicitly:

```bash
#!/usr/bin/env bash

cleanup() {
    echo "Child'larni o'ldiraman..."
    jobs -p | xargs -r kill 2>/dev/null
}
trap cleanup EXIT INT TERM

# Several workers
worker.sh &
worker.sh &
worker.sh &

wait
```

`jobs -p` — all background PIDs. `xargs -r kill` — kills them.

### `wait -n` (Bash 4.3+)

Wait for the first child to finish:

```bash
worker1 &
worker2 &
worker3 &

# We wait for any one of them to finish
wait -n
echo "Bittasi tugadi"
```

---

## 4.10. The graceful shutdown pattern

For long-running scripts (server, monitor, daemon), you need to accept a stop request **smoothly**.

```bash
#!/usr/bin/env bash
#
# monitor.sh — check disk status every 5 seconds
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

### What does this do?

1. When `Ctrl+C` is pressed — it sets `running=0`
2. The current iteration **finishes** (the `df` output is complete)
3. The next `while` check returns `false`
4. The script terminates cleanly

`exit` is not called — we finish whatever work is in progress and then exit.

---

## 4.11. The `ERR` and `DEBUG` traps for debugging

### `ERR` — on every error

```bash
#!/usr/bin/env bash
set -e

trap 'echo "❌ Xato $LINENO qatorida: $BASH_COMMAND"' ERR

echo "Ish boshlandi"
ls /yoq-katalog            # bu yerda xato bo'ladi
echo "Bu satrgacha yetmaydi"
```

When run:
```text
Ish boshlandi
ls: cannot access '/yoq-katalog': No such file or directory
❌ Xato 6 qatorida: ls /yoq-katalog
```

### `DEBUG` — before every command

```bash
trap 'echo ">> [LINE $LINENO] $BASH_COMMAND"' DEBUG

x=5
echo "salom"
ls
```

Result:
```text
>> [LINE 3] x=5
>> [LINE 4] echo "salom"
salom
>> [LINE 5] ls
...
```

::: tip The difference from `set -x`
`set -x` also prints every command, but the DEBUG trap lets you format the output **however you want**.
:::

---

## 4.12. A real example — a robust backup script

A complete example that combines all the concepts:

```bash
#!/usr/bin/env bash
#
# backup.sh — Ctrl+C, kill, or a normal finish — it handles all of them cleanly
#

set -euo pipefail

readonly SCRIPT_NAME="$(basename "$0")"
readonly LOCKFILE="/tmp/${SCRIPT_NAME%.sh}.lock"
TMPDIR=""

# --- Lock check (single-instance pattern) ---
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

# --- Cleanup function ---
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

# --- Main work ---
TMPDIR=$(mktemp -d)
echo "📁 Vaqtinchalik katalog: $TMPDIR"

echo "📦 Backup boshlandi ($(date '+%T'))"

for i in {1..10}; do
    echo "  Qadam $i / 10"
    sleep 1
done

archive="$HOME/backup_$(date +%Y%m%d_%H%M%S).tar.gz"
echo "💾 Arxivga yozilmoqda: $archive"
# tar -czf "$archive" -C "$TMPDIR" .   # the real command

echo "📊 Hajmi: $(du -sh "$TMPDIR" 2>/dev/null | cut -f1)"
```

This script:

- **Guarantees a single instance** via a **lock file** (checks for stale locks with `kill -0`)
- **`trap cleanup EXIT`** — cleans up on any kind of exit
- **`INT` → exit 130** and **`TERM` → exit 143** — the Unix convention
- **The `__cleaned` flag** — prevents being called twice
- **A temporary directory with `mktemp -d`** — safe and unique
- **Color/emoji** — clear feedback for the user

---

## 4.13. Common mistakes

::: danger Classic traps

1. **Trying to trap `SIGKILL`.**
   Impossible. `kill -9` bypasses cleanup. Use an external guardian (`systemd`, `supervisord`).

2. **Raising a new error inside cleanup.**
   Under `set -e`, an error inside cleanup drops the following lines. `cleanup() { rm -f "$tmpfile" || true; ... }`.

3. **EXIT trap indicators.**
   When `cleanup` is called, `$?` is the exit code of the most recent command. Capture it on the trap's first line with `local rc=$?`.

4. **Background processes left without a trap.**
   When the parent dies, the children become orphans. Protect against this with `trap 'kill $(jobs -p) 2>/dev/null' EXIT`.

5. **Using a variable in a trap — single vs. double quote.**
   ```bash
   trap "echo $tmpfile" EXIT   # ❌ substituted when the trap is SET
   trap 'echo $tmpfile' EXIT   # ✓ when the trap is CALLED
   ```
   This is a subtle difference! Writing it through a `cleanup` function is safer.

6. **Forgetting `exit` inside cleanup.**
   ```bash
   cleanup() { rm -f "$tmp"; }     # OK — in the EXIT trap case
   trap 'cleanup; exit 130' INT    # ✓ exit is also needed, otherwise it continues
   ```

7. **A lock file left in a stale, uncleaned state.**
   If the script crashes, the lock remains. Always check with `kill -0 $pid` and remove it if necessary.
:::

---

## 4.14. Exercises

::: tip 🧪 Bashlings — interactive exercises
This chapter's **5** exercises are available through the `bashlings` CLI with automatic checking:

```bash
bashlings watch              # start from the first pending exercise
bashlings run trap1          # check a single exercise
bashlings hint trap1         # step-by-step hint
```

Source: [`exercises/09_traps/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/09_traps)
:::

And try the following conceptual exercises by hand on your own:

1. **Countdown** — write a script that prints from 10 down to 1, one per second. If `Ctrl+C` is pressed, it should print "sorry, couldn't continue" and return exit 130.

2. **`tmp-safe`** — create a file with `mktemp`, write 5 random numbers into it, and delete it in every case. Cleanup should be done through an EXIT trap.

3. **Single instance** — a script that uses a lock file to forbid a second instance from starting. It should automatically detect a stale lock.

4. **Parallel workers** — a parent script that launches 3 background workers. On `Ctrl+C` it should kill them all (`jobs -p | xargs kill`).

5. **ERR trace** — write a script that uses `set -e` and `trap '... $LINENO $BASH_COMMAND ...' ERR` to print the line number and the command when an error occurs.

---

## 4.15. Summary

| Concept                 | Key point                                          |
|-------------------------|----------------------------------------------------|
| What a signal is        | An asynchronous message between processes           |
| `SIGINT` (2)            | Ctrl+C — the most common                           |
| `SIGTERM` (15)          | Default `kill` — a polite request to stop          |
| **`SIGKILL` (9)**       | **Cannot be trapped** — no cleanup either          |
| `EXIT`                  | Bash pseudo-signal — **the most important cleanup point** |
| `trap code SIGNAL`      | Set up a reaction to a signal                       |
| `trap - SIGNAL`         | Restore the default                                |
| `trap '' SIGNAL`        | Ignore it                                          |
| `trap -p`               | List of current traps                              |
| Exit code 130           | The result of Ctrl+C (128+SIGINT)                  |
| Exit code 143           | The result of kill (128+SIGTERM)                   |
| `kill -0 PID`           | Check whether a process is alive                   |
| `$!`                    | The most recent background PID                     |

### 5 key ideas

1. **Never hope that things "succeeded."** Guarantee cleanup with `trap EXIT`.
2. **The EXIT pseudo-signal** is one of the most powerful features of the bash world. Use it.
3. **`SIGKILL` cannot be caught.** For critical situations you need an external guardian.
4. **Single instance + lock + kill -0** is the standard pattern for production scripts.
5. **Graceful shutdown** — with the `while [[ $running -eq 1 ]]` model, finish each iteration before exiting.

🎉 Now your scripts will terminate cleanly in any situation. In the next and **final chapter**, we will learn to write production-grade scripts using **`set -euo pipefail`**, **ShellCheck**, and **`getopts`**.

> **Next page:** [5. Robust scripting →](./05-robust-scripting)
