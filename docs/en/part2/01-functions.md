---
title: "Functions and modularity"
description: "Bash functions: arguments, scope, return values, libraries, and best practices."
---

# 1. Functions and modularity

> **🎯 What you'll learn in this chapter:**
> - Declaring and calling a function, and passing arguments
> - Returning a value via `return` and `echo` (the two are different!)
> - `local` variables and scope logic
> - Writing reusable libraries (loading via `source`)
> - A real example — a small logging library
>
> **⏱ Time:** ~25 minutes reading + exercises
> **🧪 Exercises:** `bashlings watch` — 6 interactive exercises ready ([`exercises/06_functions/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/06_functions))

---

## 1.1. Why functions?

Imagine your script does the **same** work in three places: it writes a log, stamps the time, prints a colored line. Writing 4 lines in each place violates the DRY (**Don't Repeat Yourself**) principle.

A function is a **named block of code**. You write it once and call it many times.

```bash
# BAD — repeated in three places
echo "[$(date +%T)] Backup boshlandi"
# ... work ...
echo "[$(date +%T)] Database eksport qilindi"
# ... work ...
echo "[$(date +%T)] Yakunlandi"

# GOOD — via a function
log() {
    echo "[$(date +%T)] $*"
}

log "Backup boshlandi"
log "Database eksport qilindi"
log "Yakunlandi"
```

::: tip Advantages of functions
- **Brevity** — the code shrinks 3×
- **Single source of truth** — change the format in one place
- **Easy to test** — each function can be tested separately
- **Easy to read** — `log "..."` states the intent by itself
:::

---

## 1.2. Declaration syntax

In Bash, a function can be declared in **two forms**:

### A. With the `function` keyword

```bash
function salomlash() {
    echo "Salom!"
}
```

### B. POSIX form (with parentheses)

```bash
salomlash() {
    echo "Salom!"
}
```

Both work. **The POSIX form is recommended** — because:

- It also works in other POSIX shells (sh, dash)
- It's shorter
- The `function` keyword is unnecessary noise

::: warning The parentheses are empty — they're called arguments, but aren't used
The `()` inside `salomlash()` indicates that it's a function. **Arguments are not given here**; they're used inside via `$1`, `$2`, ...
:::

### Calling

You call a function **by its name**, **without parentheses**:

```bash
salomlash           # ✅ correct
salomlash()         # ❌ error — syntax is broken
```

---

## 1.3. The first complete example

```bash
#!/usr/bin/env bash

# Function declaration
salomlash() {
    echo "Salom, Bash dunyosi!"
    echo "Bugun: $(date +%F)"
}

# Main logic
echo "=== Skript boshlandi ==="
salomlash
echo "=== Skript yakunlandi ==="
```

Result:

```text
=== Skript boshlandi ===
Salom, Bash dunyosi!
Bugun: 2026-05-16
=== Skript yakunlandi ===
```

::: tip Declaration order matters!
You must declare a function **before using it**. Bash reads scripts from top to bottom.

```bash
salomlash   # ❌ ERROR — not yet declared
salomlash() { echo "Salom"; }
salomlash   # ✅ correct
```
:::

---

## 1.4. Arguments

Arguments are passed to a function **just like passing arguments to a script** — via `$1`, `$2`, ...

```bash
salomlash() {
    echo "Salom, $1!"
    echo "Yoshingiz: $2"
}

salomlash "Ali" 25
# Salom, Ali!
# Yoshingiz: 25
```

### The main special variables

| Variable | Meaning                                                  |
|-------------|----------------------------------------------------------|
| `$1`, `$2`, …| Positional arguments                                   |
| `$#`        | The **number** of arguments                             |
| `$@`        | All arguments (separately — each one its own element)    |
| `$*`        | All arguments (a single string)                          |
| `$0`        | The **script** name (NOT the function name — beware!)    |
| `${FUNCNAME[0]}` | The current **function** name                       |

::: warning The difference between `$@` and `$*`
```bash
example() {
    for arg in "$@"; do echo "@ qoldirdi: $arg"; done
    for arg in "$*"; do echo "* qoldirdi: $arg"; done
}
example "salom dunyo" "bash"
```
Result:
```text
@ qoldirdi: salom dunyo
@ qoldirdi: bash
* qoldirdi: salom dunyo bash   # joined together!
```
Almost always use `"$@"`.
:::

### Example: the sum of two numbers

```bash
yigindi() {
    echo $(($1 + $2))
}

yigindi 5 7        # 12
yigindi 100 200    # 300
```

---

## 1.5. Default values and validation

### Default value — `${var:-default}`

```bash
salomlash() {
    local ism="${1:-Anonim}"
    echo "Salom, $ism!"
}

salomlash "Ali"     # Salom, Ali!
salomlash           # Salom, Anonim!
```

### Required argument — `${var:?error message}`

```bash
backup() {
    local src="${1:?manba katalog ko'rsatilmagan}"
    echo "Backup boshlandi: $src"
}

backup           # ERROR: source directory not specified
backup ~/docs    # Backup boshlandi: /Users/mac/docs
```

::: tip The parameter expansion set
| Syntax                | Meaning                                       |
|-----------------------|-----------------------------------------------|
| `${var:-default}`     | If empty, use default, but **doesn't set it** |
| `${var:=default}`     | If empty, use default and **sets it**         |
| `${var:?message}`     | If empty, stops with an error                 |
| `${var:+replace}`     | If **not** empty, returns the replacement value |
:::

### Validating arguments manually

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

## 1.6. Return values — `return` vs `echo`

The **most confusing** aspect of Bash — returning a value from a function.

### `return` — only an exit code (0..255)

`return` is not the `return` of other languages. It only returns an **exit code**: 0 (success) or 1..255 (error).

```bash
fayl_bormi() {
    [[ -f "$1" ]] && return 0 || return 1
}

if fayl_bormi "/etc/passwd"; then
    echo "Mavjud"
fi
```

::: danger 256 or more — ERROR
```bash
qaytar() { return 256; }
qaytar
echo $?   # 0 — wrap around!
```
:::

### `echo` — returning a real value

If you want to return a real value (a number, a string), `echo` it. The caller gets it with `$(...)`.

```bash
kvadrat() {
    local n="$1"
    echo $((n * n))
}

natija=$(kvadrat 5)
echo "5 ning kvadrati: $natija"
# 5 ning kvadrati: 25
```

### Using both together

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
A function should write its **error message to stderr** (`>&2`), and the **value it returns to stdout**. Otherwise `$(...)` will also swallow the error message.
:::

---

## 1.7. Scope: `local` vs global

By default, in Bash **all** variables are global. This is dangerous.

```bash
salomlash() {
    ism="Ali"          # GLOBAL — visible outside the function too!
    echo "Salom, $ism"
}

ism="Vali"
salomlash
echo "Tashqarida: $ism"   # "Ali" — overwritten!
```

### The fix: `local`

```bash
salomlash() {
    local ism="Ali"    # ONLY inside the function
    echo "Salom, $ism"
}

ism="Vali"
salomlash
echo "Tashqarida: $ism"   # "Vali" — preserved
```

::: warning Always use `local`!
Add `local` for every new variable inside a function. This is safety and isolation.
:::

### The subtle sides of `local`

```bash
# Adding several locals
fn() {
    local a b c
    local x="bir" y="ikki"
}

# `local` swallows the exit code — this can be buggy
fn() {
    local result=$(maybe_fail)   # ⚠️ even if maybe_fail fails, $? = 0
}

# The correct way:
fn() {
    local result
    result=$(maybe_fail)         # now $? is correct
}
```

---

## 1.8. `readonly` — constants

```bash
readonly MAX_RETRIES=3
readonly LOG_FILE="/var/log/app.log"

MAX_RETRIES=5   # ERROR: readonly variable
```

::: tip Project-level constants
Declare `readonly` variables at the start of your script — this defines the **flow of the code**:

```bash
readonly SCRIPT_NAME="$(basename "$0")"
readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly LOG_FILE="${LOG_FILE:-/tmp/app.log}"
```
:::

---

## 1.9. A function library — `source`

You can move functions into another file and reuse them over and over.

### `lib/log.sh` — a reusable module

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

### The main script: `main.sh`

```bash
#!/usr/bin/env bash
set -euo pipefail

# Loading the module — there are two ways:
source "$(dirname "$0")/lib/log.sh"
# or shorter:
. "$(dirname "$0")/lib/log.sh"

log_info "Skript ishga tushdi"
log_warn "Disk hajmi past"
log_error "Database ulanishi yo'q"
die "Kritik xato — to'xtatamiz"
```

::: tip When does `source` differ from `bash`?
- `bash script.sh` → a **new subshell** opens. The functions don't escape to the outside.
- `source script.sh` → runs in the **current shell**. The functions get loaded.

For a library, always use **`source`** (or `.`).
:::

### Idiomatic — load a module only once (guard)

```bash
# at the start of lib/log.sh
[[ -n "${__LOG_LOADED:-}" ]] && return 0
readonly __LOG_LOADED=1

log_info() { ... }
# ...
```

The equivalent of `#ifndef GUARD` in C/C++.

---

## 1.10. Naming conventions and best practices

| Recommendation                         | Example                          |
|----------------------------------------|----------------------------------|
| Use `snake_case`                       | `log_info`, `parse_args`         |
| Internal functions should start with `_` | `_private_helper`              |
| Verb + noun                            | `get_user`, `check_disk`         |
| Boolean — `is_*` or `has_*`            | `is_root`, `has_internet`        |
| Module prefix                          | `db_connect`, `db_close`         |

### A sample of a fully documented function

```bash
# Checks whether the user is root.
#
# Arguments:
#   none
# Returns:
#   0 — root
#   1 — another user
# Example:
#   if is_root; then ...
is_root() {
    [[ $EUID -eq 0 ]]
}
```

---

## 1.11. A real example — a logging library

A fully reusable module:

```bash
# lib/logger.sh
#
# Usage:
#   source lib/logger.sh
#   log_info "Boshlandi"
#   log_warn "Disk past"
#   log_error "Xato"
#   LOG_LEVEL=debug log_debug "Detal"

[[ -n "${__LOGGER_LOADED:-}" ]] && return 0
readonly __LOGGER_LOADED=1

# Configuration (the user can override it)
LOG_LEVEL="${LOG_LEVEL:-info}"
LOG_FILE="${LOG_FILE:-}"

readonly __LOG_RED='\033[31m'
readonly __LOG_YELLOW='\033[33m'
readonly __LOG_GREEN='\033[32m'
readonly __LOG_BLUE='\033[34m'
readonly __LOG_RESET='\033[0m'

# Level numbers
declare -A __LOG_LEVELS=(
    [debug]=0 [info]=1 [warn]=2 [error]=3
)

_log() {
    local level="$1"; shift
    local color="$1"; shift
    local msg="$*"

    # Level filtering
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

Using it:

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

::: tip What does this lib do?
- 4 levels: debug/info/warn/error
- Filtering via `LOG_LEVEL`
- If `LOG_FILE` is set — it also writes to a file
- Colors to the terminal, plain text to the file
- A single "guard" — it won't be sourced multiple times
- Everything is in `local`, the scope is clean
:::

---

## 1.12. Common mistakes

::: danger Classic traps in functions

1. **Forgetting `local`.**
   ```bash
   fn() { x=5; }   # x stays global!
   ```

2. **`local` swallows the exit code.**
   ```bash
   fn() { local r=$(may_fail); echo $?; }   # always 0
   # Correct:
   fn() { local r; r=$(may_fail); echo $?; }
   ```

3. **`return` is not a number.**
   ```bash
   return "xato"      # ERROR — only 0..255
   ```

4. **A function name and a variable name colliding.**
   ```bash
   log=anything       # now you potentially break the function named log too
   ```

5. **Calling a function while it's not yet declared.**
   ```bash
   fn          # ❌
   fn() { ...}
   ```

6. **Using `echo` instead of `printf`.**
   `echo -e` is not portable. If you need formatting, use `printf`.

7. **Forgetting to write to stderr.**
   An error message should be in `>&2`, not in stdout.
:::

---

## 1.13. Exercises

::: tip 🧪 Bashlings — interactive exercises
This chapter's **6** exercises come with auto-checking via the `bashlings` CLI:

```bash
bashlings watch              # start from the first pending exercise
bashlings run func1          # check a single exercise
bashlings hint func1         # step-by-step hint
```

Source: [`exercises/06_functions/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/06_functions)
:::

And try the following conceptual exercises by hand yourself:

1. Write an `is_even` function — whether a number is even or odd (`return 0` / `return 1`).
2. A `repeat_word` function — print the word in the first argument as many times as the number in the second argument.
3. `max3` — `echo` the largest of three numbers to stdout.
4. `prompt_confirm` — ask the user "Davom etamizmi? [y/N]" and return 0 or 1 depending on the answer.
5. Create a `lib/math.sh` module with the functions `sum`, `mul`, `pow`. `source` it in your main script and try it out.

---

## 1.14. Summary

| Concept                 | Key point                                          |
|-------------------------|----------------------------------------------------|
| **Declaration**         | `name() { ... }` — POSIX form recommended          |
| **Arguments**           | `$1`, `$@`, `$#` — always `"$@"`                   |
| **Default value**       | `${1:-default}`                                    |
| **`return`**            | Only 0..255 — an exit code                          |
| **`echo`**              | For returning a real value                          |
| **`local`**             | Mandatory for every variable inside a function     |
| **`readonly`**          | For constants                                       |
| **`source` / `.`**      | Load a library into the **current shell**          |
| **Naming convention**   | `snake_case`, `is_*`/`has_*`, module prefix        |
| **Stderr**              | Error messages to `>&2`                             |

🎉 The first capstone is approaching — in the next chapter we'll write even more powerful functions with **arrays**.

> **Next page:** [2. Arrays and dictionaries →](./02-arrays)
