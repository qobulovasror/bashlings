---
title: "Your First Bash Script"
description: "Shebang, variables, if/else conditions and loops — the fundamentals of Bash scripting."
---

# 5. Your First Bash Script

> What you will learn in this chapter:
> - **Shebang** (`#!/usr/bin/env bash`) and the executable permission
> - **Variables** (`name=value`, quoting rules)
> - **Arguments** (`$1`, `$@`, `$#`)
> - **`if` / `else`** — conditional logic
> - **`for`** loop
> - Getting input from the user (`read`)
>
> **⏱ Time:** ~40 minutes
> **🧪 Exercises:** `bashlings watch` — 6 interactive exercises ready ([`exercises/05_scripting/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/05_scripting))

By now you move around the terminal freely and you know how to chain commands together. The time has come — let's **save our commands inside a file** and write a **script** so we can run them multiple times, or even automatically.

## 5.1. Your first script: Hello World

Let's start with the simplest possible example:

```bash
# hello.sh
#!/usr/bin/env bash

echo "Salom, dunyo!"
echo "Bugun: $(date)"
echo "Foydalanuvchi: $USER"
```

There are two steps to run the script:

```bash
# 1. Grant the execute permission
chmod +x hello.sh

# 2. Run it
./hello.sh
```

Result:

```text
Salom, dunyo!
Bugun: Sat May 16 14:22:01 +05 2026
Foydalanuvchi: mac
```

::: tip Another way
You can also run it without `chmod +x`:

```bash
bash hello.sh
```
:::

## 5.2. What is the shebang `#!`?

The first line of the script:

```bash
#!/usr/bin/env bash
```

This is the **shebang** (or **hashbang**). It tells the operating system *"which interpreter should be used to run this script."*

Popular options:

| Shebang                  | Purpose                              |
|--------------------------|--------------------------------------|
| `#!/bin/bash`            | Explicit path to bash                |
| `#!/usr/bin/env bash`    | **Recommended:** finds it from PATH  |
| `#!/bin/sh`              | POSIX shell                          |
| `#!/usr/bin/env python3` | Python3 script                       |

::: tip Why `env bash`?
`/bin/bash` is not present on every system (for example, macOS ships bash 3.x, and a user's brew bash may live at `/usr/local/bin/bash`). Using `env`, the system runs the **first available** version of bash.
:::

## 5.3. Variables

### Creating and reading

```bash
name="Ali"
age=25
greeting="Salom, $name!"

echo "$name"            # Ali
echo "$age"             # 25
echo "$greeting"        # Salom, Ali!
echo "${name}ning yoshi: ${age}"  # Aliing yoshi: 25
```

::: danger Watch out for spaces!
```bash
name = "Ali"     # ❌ WRONG — bash treats it as a command
name="Ali"       # ✅ correct
```
:::

### Types of quotes — very important!

```bash
name="Ali"

echo "Salom, $name"      # ✅ Double quotes: $variable is interpolated
echo 'Salom, $name'      # ❌ Single quotes: $name stays as-is
echo "Bugun $(date)"     # ✅ Command output is inserted
```

| Quote type   | Interpolation          | Special characters    |
|--------------|------------------------|-----------------------|
| `"..."`      | Yes (`$var`, `\``)     | `\n`, `\t` work       |
| `'...'`      | No (literal)           | Nothing               |
| No quotes    | Yes, but spaces are dangerous | dangerous      |

::: warning Always use double quotes!
```bash
file="my file.txt"
rm $file       # ❌ rm "my" "file.txt" — two files!
rm "$file"     # ✅ correct
```
:::

### Command substitution

Storing the output of a command in a variable:

```bash
today=$(date +%Y-%m-%d)
file_count=$(ls | wc -l)
home_size=$(du -sh ~ | cut -f1)

echo "$today da $file_count ta fayl bor"
echo "Home katalogim hajmi: $home_size"
```

::: info `$(...)` vs backtick
The old syntax `` `command` `` also works, but `$(...)` is **recommended** — it is easy to nest one inside another.
:::

### Special variables

| Variable    | Meaning                                |
|-------------|----------------------------------------|
| `$0`        | Script name                            |
| `$1`, `$2`  | First, second argument                 |
| `$#`        | Number of arguments                    |
| `$@`        | All arguments (separately)             |
| `$*`        | All arguments (a single string)        |
| `$?`        | Exit code of the last command (0 = success) |
| `$$`        | PID of the current script              |
| `$HOME`     | User's home directory                  |
| `$PATH`     | Paths to executable files              |
| `$USER`     | Username                               |
| `$PWD`      | Current directory                      |

## 5.4. Accepting arguments

```bash
#!/usr/bin/env bash
# greet.sh

echo "Skript nomi: $0"
echo "Birinchi argument: $1"
echo "Ikkinchi argument: $2"
echo "Jami argumentlar: $#"
echo "Barchasi: $@"
```

Running it:

```bash
./greet.sh Ali Vali
```

Result:

```text
Skript nomi: ./greet.sh
Birinchi argument: Ali
Ikkinchi argument: Vali
Jami argumentlar: 2
Barchasi: Ali Vali
```

## 5.5. Getting input from the user

```bash
#!/usr/bin/env bash

read -p "Ismingizni kiriting: " name
echo "Salom, $name!"

# Reading a password while hiding it
read -sp "Parol: " password
echo
echo "Parol uzunligi: ${#password}"
```

## 5.6. Conditional operators: `if / else`

### Basic syntax

```bash
if [[ shart ]]; then
    # bajariladigan kod
elif [[ boshqa_shart ]]; then
    # ...
else
    # default
fi
```

### Example — checking the number that was entered

```bash
#!/usr/bin/env bash

read -p "Son kiriting: " num

if [[ $num -gt 0 ]]; then
    echo "Musbat son"
elif [[ $num -lt 0 ]]; then
    echo "Manfiy son"
else
    echo "Nol"
fi
```

### Numeric comparison operators

| Operator | Meaning             |
|----------|---------------------|
| `-eq`    | equal (`==`)        |
| `-ne`    | not equal (`!=`)    |
| `-gt`    | greater than (`>`)  |
| `-lt`    | less than (`<`)     |
| `-ge`    | greater or equal    |
| `-le`    | less or equal       |

### String comparison

| Operator      | Meaning                        |
|---------------|--------------------------------|
| `=` or `==`   | equal                          |
| `!=`          | not equal                      |
| `-z "$s"`     | is the string empty            |
| `-n "$s"`     | is the string non-empty        |
| `<` `>`       | alphabetical order (lex)       |

### File test operators

| Operator   | Meaning                          |
|------------|----------------------------------|
| `-e fayl`  | exists                           |
| `-f fayl`  | regular file                     |
| `-d fayl`  | directory                        |
| `-r fayl`  | has read permission              |
| `-w fayl`  | has write permission             |
| `-x fayl`  | has execute permission           |
| `-s fayl`  | size greater than 0              |

### A real example — checking a file

```bash
#!/usr/bin/env bash

file="$1"

if [[ -z "$file" ]]; then
    echo "Foydalanish: $0 <fayl_nomi>"
    exit 1
fi

if [[ ! -e "$file" ]]; then
    echo "❌ Fayl topilmadi: $file"
    exit 1
fi

if [[ -d "$file" ]]; then
    echo "📁 Bu katalog"
elif [[ -f "$file" ]]; then
    echo "📄 Bu oddiy fayl"
    echo "   Hajmi: $(du -h "$file" | cut -f1)"
fi
```

### `[[ ]]` vs `[ ]` vs `(( ))`

```bash
[[ "$name" == "Ali" ]]         # bash extension — RECOMMENDED
[ "$name" = "Ali" ]            # old POSIX style
(( $age > 18 ))                # for arithmetic
```

::: tip Always use `[[ ]]`
- Tolerates spaces
- Supports the `=~` regex
- Easy to use inside `&&`, `||`
:::

## 5.7. Loops

### `for` loop

```bash
# Over numbers
for i in 1 2 3 4 5; do
    echo "Raqam: $i"
done

# A range (brace expansion)
for i in {1..10}; do
    echo "$i"
done

# 0..20 with a step of two
for i in {0..20..2}; do
    echo "$i"
done

# C-style
for ((i=0; i<5; i++)); do
    echo "$i"
done

# Over files
for file in *.txt; do
    echo "Topildi: $file"
done

# Over command output
for user in $(cut -d':' -f1 /etc/passwd); do
    echo "User: $user"
done
```

### `while` loop

```bash
# A counter
count=1
while [[ $count -le 5 ]]; do
    echo "Qadam: $count"
    ((count++))
done

# Reading a file line by line
while IFS= read -r line; do
    echo "Qator: $line"
done < /etc/passwd

# An infinite loop
while true; do
    echo "Davom etmoqda..."
    sleep 1
done
```

### `until` loop

```bash
# Runs UNTIL the condition becomes FALSE
n=1
until [[ $n -gt 5 ]]; do
    echo "n=$n"
    ((n++))
done
```

### `break` and `continue`

```bash
for i in {1..10}; do
    if [[ $i -eq 5 ]]; then
        continue       # skip this iteration
    fi
    if [[ $i -eq 8 ]]; then
        break          # leave the loop
    fi
    echo "$i"
done
# 1 2 3 4 6 7
```

## 5.8. `case` — a multi-branch condition

```bash
#!/usr/bin/env bash

read -p "Tugmani bosing (a/b/c): " key

case "$key" in
    a|A)
        echo "Birinchi tanlov"
        ;;
    b|B)
        echo "Ikkinchi tanlov"
        ;;
    c|C)
        echo "Uchinchi tanlov"
        ;;
    *)
        echo "Noma'lum tugma"
        ;;
esac
```

## 5.9. A complete practical example: a Backup script

```bash
#!/usr/bin/env bash
#
# backup.sh — a simple backup script
# Usage: ./backup.sh <source_directory> <target_directory>

src="$1"
dst="$2"

# 1. Check the arguments
if [[ $# -ne 2 ]]; then
    echo "Foydalanish: $0 <manba> <maqsad>"
    exit 1
fi

# 2. Does the source directory exist?
if [[ ! -d "$src" ]]; then
    echo "❌ Manba katalog topilmadi: $src"
    exit 1
fi

# 3. If the target directory does not exist, create it
if [[ ! -d "$dst" ]]; then
    mkdir -p "$dst"
    echo "📁 Yaratildi: $dst"
fi

# 4. Archive name
timestamp=$(date +%Y%m%d_%H%M%S)
archive="$dst/backup_$timestamp.tar.gz"

# 5. Archiving
echo "📦 Arxivlanmoqda..."
if tar -czf "$archive" -C "$(dirname "$src")" "$(basename "$src")"; then
    size=$(du -h "$archive" | cut -f1)
    echo "✅ Tayyor: $archive ($size)"
else
    echo "❌ Arxivlashda xatolik"
    exit 1
fi

# 6. Clean up old backups (older than 7 days)
echo "🧹 Eski backuplar tozalanmoqda..."
find "$dst" -name "backup_*.tar.gz" -mtime +7 -delete

echo "🎉 Backup yakunlandi"
```

Running it:

```bash
chmod +x backup.sh
./backup.sh ~/Documents ~/Backups
```

## 5.10. Exit codes

Every command leaves behind an **exit code** when it finishes:

- `0` — success
- `1-255` — various errors

```bash
ls /home
echo $?         # 0

ls /yoq-katalog
echo $?         # 2

# Returning an exit code from a script
exit 0          # success
exit 1          # general error
```

## 5.11. Common mistakes

::: danger Classic beginner mistakes

1. **`name = "Ali"`** — no spaces: `name="Ali"`
2. **Unquoted variable:** `rm $file` → `rm "$file"`
3. **`if [ $x == "y" ]`** — use `[[ ... ]]`; even though a single `=` is possible, `==` is the right choice.
4. **Forgetting the shebang.** Put `#!/usr/bin/env bash` at the start of the script.
5. **Not running `chmod +x`.** Or run it via `bash script.sh`.
6. **No `exit 0`.** A good script always returns an explicit exit code.
:::

## 5.12. Exercises

::: tip 🧪 Bashlings — interactive exercises
This chapter's **6** exercises with auto-checking via the `bashlings` CLI:

```bash
bashlings watch              # start from the first pending exercise
bashlings run script1        # check a single exercise
bashlings hint script1       # step-by-step hint
```

Source: [`exercises/05_scripting/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/05_scripting)
:::

Do the following additional tasks by hand in the terminal:

1. Write an `args.sh` script — it should print all arguments in reverse order.
2. `agedif.sh` — ask the user for their age and classify it as "Child/Youth/Adult".
3. `count.sh` — print the number of files and the number of directories in the current directory separately.
4. `loop.sh` — print the squares of the numbers from 1 to 10.
5. `safe-rm.sh` — a script that checks whether a file exists and asks for confirmation before deleting it.

## 5.13. Summary

In this chapter you:

- Wrote the shebang and your first script
- Learned the differences between variables and quoting
- Learned how to work with arguments and input
- Reviewed the `if/elif/else`, `case`, `for`, `while`, `until` constructs
- Wrote your first real project — **backup.sh**

🎉 **Part 1 is complete!** You have now thoroughly mastered the fundamentals of Linux & Bash.

In the next Part 2 we move on to topics like **functions, arrays, `sed`/`awk`, signals, and robust scripts**.

> **Next page:** [Part 2: Functions and modularity →](../part2/01-functions)
