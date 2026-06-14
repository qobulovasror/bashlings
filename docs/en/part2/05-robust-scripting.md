---
title: "Robust scripts — set -euo pipefail"
description: "A complete guide to strict mode, getopts, logging, ShellCheck, security, and production-grade Bash scripts."
---

# 5. Robust scripts — `set -euo pipefail`

> **🎯 What you'll learn in this chapter:**
> - **Strict mode** — a deep dive into `set -euo pipefail` and `IFS=$'\n\t'`
> - **`getopts`** — professional argument parsing
> - **Logging** — with levels, timestamps, color, and TTY detection
> - **ShellCheck** — Bash's official linter
> - **Security** — quoting, command injection, secrets
> - **Production template** — a copy-paste-ready skeleton
>
> **⏱ Time:** ~40 minutes
> **🧪 Exercises:** `bashlings watch` — 5 interactive exercises ready ([`exercises/10_robust/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/10_robust))

---

## 5.1. What's the difference — "it works" vs. "production-ready"?

```bash
# Version 1 — "it works"
cp /tmp/data.txt /var/backups/

# Version 2 — "production-ready"
set -euo pipefail
readonly SRC="/tmp/data.txt"
readonly DST="/var/backups/data_$(date +%Y%m%d).txt"
[[ -r "$SRC" ]] || { log_error "could not read $SRC"; exit 1; }
cp -p "$SRC" "$DST" || { log_error "Backup failed"; exit 1; }
log_info "✅ Backup: $DST"
```

Both do the same thing. But the **second one**:
- Stops immediately on error
- Detects undefined variables
- Logs the reasons for errors
- Checks in advance that the source exists
- Doesn't continue if `cp` fails
- Gives the user clear feedback

The goal of this chapter is to turn every one of your scripts from **version 1** into **version 2**.

::: tip Rule
**Production script = "it works" + "it surfaces errors correctly" + "it's secure" + "it can be re-run" + "it can be tested".**
:::

---

## 5.2. Strict mode — `set -euo pipefail`

Bash's most powerful (and least-used) line:

```bash
#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'
```

Writing this at the top of every script has become a standard in the DevOps world. Let's look at each part in detail.

### `set -e` — stop on the first error

The default Bash behavior — if a command fails, it **keeps going**:

```bash
ls /no-such-dir        # error
echo "Still going"     # still runs!
```

With `set -e` — exit immediately on the first error:

```bash
set -e
ls /no-such-dir        # stops here
echo "This line is never reached"
```

### Is that enough? No — the nuances of `set -e`

`set -e` **is bypassed in many cases**:

```bash
set -e

cmd || true             # error is cancelled after ||
cmd && other_cmd        # also cancelled
if cmd; then            # in an if condition — cancelled
    ...
fi
cmd | grep foo          # ❌ cmd's error is hidden (you need pipefail)
fn() { cmd; }           # inside a function — partial
```

::: warning `set -e` with functions — careful
```bash
set -e
risky() {
    cmd_that_fails
    echo "Does this print or not?"
}
risky || true
```
Before Bash 4.4, `set -e` behaved non-standardly inside functions. Even now there are nuances. Inside functions, **explicit** error handling is recommended.
:::

### `set -u` — error on undefined variables

```bash
set -u
echo "$undefined_var"   # ❌ unbound variable
```

This protects against **typos**:

```bash
set -u
name="Ali"
echo "Hello, $namr"     # typo! → unbound variable error
```

### Techniques for working with `set -u`

Sometimes a variable may be **optional**. Use the default-value pattern:

```bash
set -u

# ❌ Errors:
echo "${OPT_VAR}"

# ✓ "if no value is given, empty":
echo "${OPT_VAR:-}"

# ✓ Default value:
echo "${OPT_VAR:-default}"

# ✓ Checking it's non-empty:
if [[ -z "${OPT_VAR:-}" ]]; then
    echo "not provided"
fi
```

### `set -o pipefail` — catch errors inside a pipeline

By default a pipe's error is hidden — only the exit code of the **last** command is returned:

```bash
false | true            # exit 0 ✓ (false was hidden)
echo $?                 # 0

set -o pipefail
false | true            # exit 1 (the first error is returned)
echo $?                 # 1
```

A real example:
```bash
# This is dangerous — if curl fails, jq still returns 0
curl https://api.com/data | jq .

# With pipefail you can detect curl's error
set -o pipefail
curl https://api.com/data | jq .
```

### `IFS=$'\n\t'` — safe word-splitting

The default `IFS` is `<space><tab><newline>`. This is dangerous when filenames contain spaces:

```bash
files="my file.txt other.txt"
for f in $files; do echo "$f"; done
# my
# file.txt
# other.txt    ← "my file.txt" got broken!
```

`IFS=$'\n\t'` — only newline and tab. Spaces are preserved:

```bash
IFS=$'\n\t'
files=$'my file.txt\nother.txt'
for f in $files; do echo "$f"; done
# my file.txt
# other.txt    ← correct!
```

::: tip 4 lines at the top of a script
```bash
#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'
```
This is the "Bash Strict Mode" — the opening formula for every production script.
:::

---

## 5.3. Handling errors — beyond strict mode

`set -e` is powerful, but sometimes an **error is expected**. For example, when `grep` finds no match it returns exit 1 — that's not an error, it's a **result**.

### `||` — "use a default value on error"

```bash
matches=$(grep "ERROR" log.txt) || matches="not found"

if ! command -v jq &>/dev/null; then
    echo "jq is not installed" >&2
    exit 1
fi
```

### `if ! command` — "the command failed"

```bash
if ! curl -fsS "https://api.com/" > response.json; then
    echo "API went down"
    exit 1
fi
```

### `set +e` / `set -e` block

Temporarily disabling strict mode:

```bash
set -e
some_critical_step

set +e                   # disable strict mode
optional_step_that_may_fail
maybe_fail
set -e                   # re-enable
```

### `ERR` trap — a centralized error handler

```bash
#!/usr/bin/env bash
set -euo pipefail

error_handler() {
    local line=$1
    local cmd=$2
    echo "❌ Error on line $line: $cmd" >&2
    exit 1
}

trap 'error_handler $LINENO "$BASH_COMMAND"' ERR

# If an error occurs here, the handler is called
```

---

## 5.4. Idempotent scripts

**Idempotent** — a script that produces the same result even if you run it several times. The golden rule of production.

### Example — not clean (not idempotent)

```bash
mkdir /var/myapp        # fails the second time
adduser myapp           # fails the second time
echo "config" > /etc/myapp.conf  # overwrites the old one
```

### Idempotent variant

```bash
mkdir -p /var/myapp                          # OK if it already exists
id myapp &>/dev/null || adduser myapp        # only if it doesn't exist
[[ -f /etc/myapp.conf ]] || echo "config" > /etc/myapp.conf
```

### State marker pattern

```bash
readonly MARKER="/var/run/migration_v2.done"

if [[ -f "$MARKER" ]]; then
    echo "Migration already done"
    exit 0
fi

# ... migration work ...

touch "$MARKER"
echo "✅ Done"
```

### Lock file — a single-copy guarantee

(Familiar from Chapter 4 — here it is once more)

```bash
readonly LOCKFILE="/tmp/myapp.lock"

if [[ -f "$LOCKFILE" ]] && kill -0 "$(cat "$LOCKFILE")" 2>/dev/null; then
    echo "Already running"
    exit 0
fi
echo $$ > "$LOCKFILE"
trap 'rm -f "$LOCKFILE"' EXIT
```

---

## 5.5. Logging — production quality

### `printf` vs `echo`

```bash
echo -e "Hello\tworld"   # ❌ -e is not portable (BSD/POSIX differences)
printf 'Hello\tworld\n'  # ✓ works everywhere
```

**Rule:** always use `printf`. Use `echo` only for simple strings.

### Log levels and timestamps

```bash
log_debug() { [[ "${DEBUG:-0}" == "1" ]] && printf '[%s] [DEBUG] %s\n' "$(date +%T)" "$*" >&2; }
log_info()  { printf '[%s] [INFO]  %s\n' "$(date +%T)" "$*"; }
log_warn()  { printf '[%s] [WARN]  %s\n' "$(date +%T)" "$*" >&2; }
log_error() { printf '[%s] [ERROR] %s\n' "$(date +%T)" "$*" >&2; }
```

Usage:
```bash
log_info "Server started"
log_warn "Disk 80% full"
log_error "Database is not responding"
DEBUG=1 ./script.sh     # debug enabled
```

### Color — only if it's a TTY

```bash
if [[ -t 1 ]]; then       # is stdout a TTY?
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

`[[ -t 1 ]]` — is stdout a terminal? When output is written to a pipe or file, no color is added (clean log files).

### `logger` — write to syslog

```bash
logger -t myapp "Server started"
# written to /var/log/syslog (Linux)
# Tag: myapp
```

---

## 5.6. `getopts` — professional argument parsing

Reading `$1`, `$2`, `$3` by hand — fine for simple scripts. When you need **multiple flags** — use `getopts`.

### Basic syntax

```bash
verbose=0
output=""

while getopts ":vo:h" opt; do
    case "$opt" in
        v) verbose=1 ;;
        o) output="$OPTARG" ;;
        h) echo "Help"; exit 0 ;;
        \?) echo "Unknown flag: -$OPTARG" >&2; exit 2 ;;
        :)  echo "Flag -$OPTARG requires an argument" >&2; exit 2 ;;
    esac
done
shift $((OPTIND - 1))    # removes the parsed flags from $@
```

### Reading the format string

`":vo:h"`:
- The leading `:` — silent error mode (we print errors ourselves)
- `v` — flag (no argument)
- `o:` — flag + argument (the `:` after it)
- `h` — flag (no argument)

### A real example

```bash
#!/usr/bin/env bash
set -euo pipefail

usage() {
    cat <<EOF
Usage: $0 [-v] [-o FILE] [-h] INPUT

Options:
  -v        Verbose mode
  -o FILE   Output file (default: stdout)
  -h        Help
EOF
}

verbose=0
output=""

while getopts ":vo:h" opt; do
    case "$opt" in
        v) verbose=1 ;;
        o) output="$OPTARG" ;;
        h) usage; exit 0 ;;
        \?) echo "❌ Unknown flag: -$OPTARG" >&2; usage; exit 2 ;;
        :)  echo "❌ -$OPTARG requires an argument" >&2; exit 2 ;;
    esac
done
shift $((OPTIND - 1))

if [[ $# -lt 1 ]]; then
    echo "❌ INPUT argument required" >&2
    usage
    exit 2
fi

input="$1"
[[ $verbose -eq 1 ]] && echo "Verbose enabled, input=$input"
```

Usage:
```bash
./script.sh -v -o result.txt input.txt
./script.sh -h
./script.sh -x          # → Unknown flag: -x
```

### `getopts` limitations

- It only supports **short flags** (`-v`, `-o`)
- If you need long flags (`--verbose`) — use `getopt` (which differs between BSD/GNU) or hand-roll the parsing

---

## 5.7. Configuration — environment vs CLI

### Environment variables — for tuning

```bash
LOG_LEVEL="${LOG_LEVEL:-info}"
TIMEOUT="${TIMEOUT:-30}"
API_URL="${API_URL:-https://api.example.com}"
```

Usage:
```bash
LOG_LEVEL=debug ./script.sh
TIMEOUT=60 ./script.sh
```

### Configuration file — `source`

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

### `.env` file

```bash
# .env
DATABASE_URL=postgres://localhost/myapp
SECRET_KEY=xyz123
```

```bash
# Load only if .env exists
if [[ -f .env ]]; then
    set -a       # automatically export
    source .env
    set +a
fi
```

::: warning Add `.env` to gitignore
Never commit secrets files to a repository. `.env` must be in `.gitignore`.
:::

---

## 5.8. ShellCheck — Bash's official linter

`shellcheck` statically analyzes Bash scripts for errors and anti-patterns. **Writing a production script — not without `shellcheck`.**

### Installation

```bash
# macOS
brew install shellcheck

# Ubuntu/Debian
sudo apt install shellcheck

# Web option (for trying it out)
# https://www.shellcheck.net/
```

### Usage

```bash
shellcheck script.sh
```

### Example error

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

The fixed version:
```bash
name="$1"
echo "Salom $name"      # already quoted — OK
rm "$name"              # quoted
```

### The most common warnings

| Code   | Meaning                                          |
|--------|--------------------------------------------------|
| SC2086 | Unquoted variable — add quotes                   |
| SC2155 | `local var=$(cmd)` — the exit code is swallowed  |
| SC2046 | Quote `$(cmd)`                                   |
| SC2034 | Unused variable                                  |
| SC2164 | Handle the error from `cd`                       |
| SC2181 | Use `if cmd` instead of `[[ $? ... ]]`           |
| SC2207 | `arr=($(cmd))` — use `mapfile -t`                |

### Selective suppression

Sometimes you write something "wrong" on purpose — for example, intentional word-splitting:

```bash
# shellcheck disable=SC2086
echo $unquoted_intentional
```

A `# shellcheck` directive — it's a **comment**, but shellcheck reads it.

::: tip Make it mandatory in CI
Add a `shellcheck *.sh` step to GitHub Actions / GitLab CI. If there's an error in a pull request, CI fails — this raises quality immediately.
:::

---

## 5.9. Testing — `bats-core`

For automatically testing scripts, **bats** (Bash Automated Testing System) is the most popular.

### Installation

```bash
brew install bats-core
```

### Test example

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

### Usage

```bash
bats tests/
# ✓ yigindi 2 va 3
# ✓ salbiy sonlar ishlaydi
# ✓ argumentsiz xato beradi
# 3 tests, 0 failures
```

### CI integration

```yaml
# .github/workflows/test.yml
- run: bats tests/
- run: shellcheck *.sh
```

---

## 5.10. Security — the most important part of production

### 1. Always quote

```bash
file="$1"

rm $file        # ❌ "my file.txt" → two arguments
rm "$file"      # ✓
```

### 2. Command injection risk

```bash
# ❌ DANGEROUS
read -p "Filename: " name
ls $name
# If the user types "; rm -rf /" — disaster

# ✓ SAFE
read -p "Filename: " name
ls "$name"
```

### 3. Avoid `eval`

```bash
eval "$user_input"      # ❌ NEVER
```

### 4. `mktemp` — safe temporary files

```bash
# ❌ Predictable, dangerous
tmpfile=/tmp/myapp.tmp

# ✓ Random, atomic
tmpfile=$(mktemp)
tmpdir=$(mktemp -d)

trap 'rm -rf "$tmpfile" "$tmpdir"' EXIT
```

### 5. Working with secrets

```bash
# ❌ Never put a password inside the script
DB_PASSWORD="hardcoded123"

# ✓ From the environment
DB_PASSWORD="${DB_PASSWORD:?DB_PASSWORD is not set}"

# ✓ Or a secrets manager (vault, AWS Secrets, ...)
DB_PASSWORD=$(aws secretsmanager get-secret-value --secret-id db --query SecretString --output text)
```

### 6. File permissions

```bash
umask 077                # created files will be 600 (only the owner can read)
tmpfile=$(mktemp)
echo "secret" > "$tmpfile"
ls -l "$tmpfile"         # -rw-------
```

### 7. The security benefit of `set -u`

```bash
set -u
rm -rf "$WORK_DIR"/*    # if WORK_DIR is undefined — ERROR, root is not wiped
```

This is an important mechanism that protects you from disasters like `rm -rf /`.

---

## 5.11. Documentation and help

### Usage pattern

```bash
usage() {
    cat <<EOF
$(basename "$0") — short description

USAGE:
    $(basename "$0") [OPTIONS] ARGUMENTS

OPTIONS:
    -h          Show this help text
    -v          Verbose mode
    -o FILE     Output file

EXAMPLES:
    $(basename "$0") -v input.txt
    $(basename "$0") -o result.txt data.csv

EOF
}
```

### Inline comments

Good:
```bash
# Clean up old (>30 days) backups
find "$BACKUP_DIR" -name "*.tar.gz" -mtime +30 -delete
```

Bad:
```bash
find "$BACKUP_DIR" -name "*.tar.gz" -mtime +30 -delete  # uses find
```

A comment should explain the **WHY**, not the **WHAT** (the code says that itself).

---

## 5.12. A real example — Production-Ready Template

This is a copy-paste-ready **skeleton**. Everything together:

```bash
#!/usr/bin/env bash
#
# my-script.sh — short description (a line or two)
#
# Usage:
#   ./my-script.sh [-v] [-o output] <input>
#

# === Strict mode ===
set -euo pipefail
IFS=$'\n\t'

# === Constants ===
readonly SCRIPT_NAME="$(basename "$0")"
readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly LOG_FILE="${LOG_FILE:-/tmp/${SCRIPT_NAME%.sh}.log}"

# === Colors (only for a TTY) ===
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
    # clean up tmp files, lock files, and other resources
    [[ -n "${TMPDIR_:-}" && -d "$TMPDIR_" ]] && rm -rf "$TMPDIR_"
    if [[ $rc -eq 0 ]]; then
        log_info "Finished (success)"
    elif [[ $rc -eq 130 ]]; then
        log_warn "User interrupted"
    else
        log_error "Exit code: $rc"
    fi
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
trap 'die "Error on line $LINENO: $BASH_COMMAND"' ERR

# === Usage ===
usage() {
    cat <<EOF
$SCRIPT_NAME — short description

USAGE:
    $SCRIPT_NAME [OPTIONS] <input>

OPTIONS:
    -v          Verbose mode
    -o FILE     Output file (default: stdout)
    -h          This help

ENVIRONMENT VARIABLES:
    DEBUG=1     Enable debug logging
    LOG_FILE    Log file path (default: /tmp/$SCRIPT_NAME.log)
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
        \?) die "Unknown flag: -$OPTARG" ;;
        :)  die "Flag -$OPTARG requires an argument" ;;
    esac
done
shift $((OPTIND - 1))

[[ $# -lt 1 ]] && { usage; die "Input required"; }

readonly INPUT="$1"

# === Main work ===
main() {
    log_info "Started: $INPUT"
    [[ $verbose -eq 1 ]] && log_info "Verbose enabled"

    [[ -r "$INPUT" ]] || die "Not found or not readable: $INPUT"

    # Temporary directory
    TMPDIR_=$(mktemp -d)
    log_debug "Tmp: $TMPDIR_"

    # ... the real work goes here ...

    log_info "Done"
}

main "$@"
```

### What does this template do?

| Element                | Benefit                                        |
|------------------------|------------------------------------------------|
| `set -euo pipefail`    | Strict mode — errors aren't hidden             |
| `IFS=$'\n\t'`          | Safe with files containing spaces              |
| `readonly` constants   | Keeps the script's state stable                |
| TTY-aware colors       | Writes correctly to both terminal and file     |
| Logging — 4 levels     | For production analysis                         |
| Cleanup + 4 traps      | Clean termination in every situation           |
| `usage()` + `-h`       | Self-documenting                               |
| `getopts`              | Professional argument parsing                  |
| `main "$@"`            | Main logic in a single function — easy to test |

::: tip Usage
Save this template as `template.sh`. When writing a new script, start by copying it. **80% of the work is already done.**
:::

---

## 5.13. Common mistakes

::: danger Top 10 production pitfalls

1. **`set -e` is missing.**
   The script silently leaves errors behind. Always add it.

2. **`set -u` is missing.**
   Typos slip through easily.

3. **`set -o pipefail` is forgotten.**
   Pipeline errors are invisible.

4. **Unquoted variable.**
   `rm $file` — files with spaces get broken.

5. **`local r=$(cmd)` — the exit code is swallowed.**
   ```bash
   local r          # declare first
   r=$(cmd)         # then assign
   ```

6. **stderr to `/dev/null` is forgotten.**
   `ls /nope 2>/dev/null` — should pass silently.

7. **`ShellCheck` is not used.**
   80% of errors are detected automatically.

8. **The error from `cd` is not handled.**
   ```bash
   cd "$dir" || die "cd error: $dir"
   ```

9. **`eval` is used.**
   Command injection — avoid it.

10. **Hardcoded file paths.**
    `/Users/mac/...` — won't work on another machine. Use `$HOME`, `$(dirname "$0")`.
:::

---

## 5.14. Exercises

::: tip 🧪 Bashlings — interactive exercises
This chapter's **5** exercises come with auto-checking via the `bashlings` CLI:

```bash
bashlings watch              # start from the first pending exercise
bashlings run rob1           # check a single exercise
bashlings hint rob1          # step-by-step hint
```

Source: [`exercises/10_robust/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/10_robust)
:::

Try the following conceptual exercises by hand as well:

1. **Strict-mode lab** — the script below only errors under strict mode. Explain why and fix it:
   ```bash
   #!/usr/bin/env bash
   name="${name}"
   files=$(ls /yoq)
   echo "tugadi"
   ```

2. **ShellCheck cleanup** — check it with `shellcheck --severity=warning` and bring it to an error-free state:
   ```bash
   #!/bin/bash
   for f in $(ls *.log); do
       cp $f /backup/
       echo "Done $f"
   done
   ```

3. **`getopts` parser** — write a script that supports the `-c COUNT`, `-d DELAY`, `-v`, and `-h` flags.

4. **Idempotent installer** — write a script that installs `nginx`. It must be safe to run several times (it must not error if it's already installed).

5. **Production template** — copy the template above and write a simple "Hello, World" script. It must pass ShellCheck.

---

## 5.15. Summary

### Production-ready script checklist

- [ ] Shebang: `#!/usr/bin/env bash`
- [ ] Strict mode: `set -euo pipefail`
- [ ] Safe IFS: `IFS=$'\n\t'`
- [ ] `readonly` constants
- [ ] Logging — at least 3 levels (info/warn/error)
- [ ] Cleanup `trap EXIT`
- [ ] `INT`/`TERM` graceful handling
- [ ] `usage()` + `-h` flag
- [ ] Argument parsing with `getopts`
- [ ] **Every** variable quoted: `"$var"`
- [ ] Tmp files via `mktemp`
- [ ] Passes ShellCheck (`shellcheck` with 0 warnings)
- [ ] Main work in a `main()` function
- [ ] Tests (bats-core)

### The 10 golden rules

| #  | Rule                                             |
|----|--------------------------------------------------|
| 1  | `set -euo pipefail` always                       |
| 2  | Quote every variable: `"$var"`                   |
| 3  | Tmp files using `mktemp`                          |
| 4  | `trap cleanup EXIT`                              |
| 5  | `ShellCheck` — before every commit               |
| 6  | Arguments via `getopts`                          |
| 7  | `printf` (not `echo`)                             |
| 8  | `[[ ]]` (not `[ ]`)                               |
| 9  | `$(...)` (not `` `cmd` ``)                        |
| 10 | No "magic numbers" — `readonly` constants        |

---

## 🎉 Part 2 is complete!

Congratulations! You have mastered the following:

| Chapter | Topic                              | Skill                                 |
|---------|------------------------------------|---------------------------------------|
| 1       | Functions                          | Reusing code                          |
| 2       | Arrays                             | Structured data                       |
| 3       | sed and awk                        | Industrial text processing            |
| 4       | Signals and traps                  | Real-world cleanup                    |
| 5       | Robust scripts                     | Production-grade code                 |

You are now a **professional Bash developer**. DevOps, SRE, system administrator — in any role, you have the tools you need at hand.

### Next steps

- **Practice:** in every real project, before reaching for Python or Go instead of Bash — try to solve it in Bash first
- **Reading:** [Google Shell Style Guide](https://google.github.io/styleguide/shellguide.html), [Bash Pitfalls](https://mywiki.wooledge.org/BashPitfalls)
- **Learn the future of Bash** — `coproc`, `<()` process substitution in more depth, `mapfile` flags
- **Part 3** — integration with `curl`, `ssh`, `jq`, `cron`, `docker`, CI/CD

> **Well done!** 🚀
>
> There's still much more to do — but the foundation is now solid.
