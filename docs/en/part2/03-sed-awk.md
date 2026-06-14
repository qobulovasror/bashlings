---
title: "sed, awk and grep mastery"
description: "sed — stream editor, awk — field-based processor. Substitution, addresses, in-place edit, BEGIN/END, arrays, real production examples."
---

# 3. sed, awk and grep mastery

> **🎯 What you'll learn in this chapter:**
> - **`sed`** — stream editor: substitution, address ranges, in-place edit, backreferences
> - **`awk`** — field-based processor: `$1`/`$NF`, `BEGIN`/`END`, arrays, counter pattern
> - **Real examples:** Apache log statistics, CSV transform, configuration parsing
> - **macOS BSD sed** vs **GNU sed** differences (`-i` gotcha)
>
> **⏱ Time:** ~40 minutes
> **🧪 Exercises:** `bashlings watch` — 6 interactive exercises ready ([`exercises/08_text_advanced/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/08_text_advanced))

---

## 3.1. The three pillars — who does what?

In the Unix world there are **three classic tools** for working with text. Each has its own role:

| Tool | Job                                            | Example request                                |
|--------|------------------------------------------------|------------------------------------------------|
| `grep` | **Find** — filter lines                        | "how many ERRORs are in the log?"              |
| `sed`  | **Edit** — substitute/delete in a stream       | "replace 'localhost' with 'prod' in the config file" |
| `awk`  | **Compute** — fields and arithmetic            | "what's the sum of the CSV's 3rd column?"      |

::: tip Remember
**`grep` — find. `sed` — edit. `awk` — compute.**

In most real tasks we use **all three together** via a pipe.
:::

`grep` was covered in depth in Part 1/04. In this chapter we focus on **`sed`** and **`awk`** — because they make up 60-70% of a DevOps and SRE workday.

---

## 3.2. `sed` basics — substitution

`sed` — **S**tream **ED**itor. It reads from standard input (or a file), **applies a specified command** to each line, and prints the result to stdout.

### The most basic operation — `s/PATTERN/REPLACEMENT/`

```bash
echo "salom dunyo" | sed 's/dunyo/bash/'
# salom bash
```

Format: `s/<searched>/<replacement>/[flags]`

### `g` flag — replace all matches

By default `sed` replaces **only the first** match on each line:

```bash
echo "ola bola ola" | sed 's/ola/X/'
# X bola ola      ← only the first

echo "ola bola ola" | sed 's/ola/X/g'
# X bola X        ← all
```

### Working with a file

```bash
# File as stdin
sed 's/old/new/g' file.txt

# Or as an argument (same thing)
sed 's/old/new/g' < file.txt
```

The result goes to stdout — **the file itself is not changed** (this is not in-place edit).

### Other useful flags

| Flag  | Meaning                                  |
|-------|------------------------------------------|
| `g`   | global — all matches on each line        |
| `i`   | case-insensitive (GNU sed)               |
| `2`   | replace only the 2nd match               |
| `p`   | additionally print the replaced line (with `-n`) |

```bash
# Make the 2nd "ola" into X, leave the rest
echo "ola bola ola" | sed 's/ola/X/2'
# ola bola X
```

### Changing the delimiter

If the pattern contains `/`, instead of escaping it, **use a different character**:

```bash
# Classic way — escaping
echo "/usr/local/bin" | sed 's/\/usr\//\/opt\//'
# /opt/local/bin    ← hard to read

# Better — make `|` or `,` the delimiter
echo "/usr/local/bin" | sed 's|/usr/|/opt/|'
# /opt/local/bin    ← clean
```

---

## 3.3. Address — which line to apply to

By default `sed` applies the command to every line. With an **address** you can restrict it.

### By line number

```bash
# Replace only on line 3
sed '3 s/foo/bar/' file.txt

# Range: from 5 to 10
sed '5,10 s/foo/bar/' file.txt

# Last line — $
sed '$ s/foo/bar/' file.txt
```

### By regex

```bash
# Only on lines containing the word "ERROR"
sed '/ERROR/ s/timeout/TIMEOUT/' log.txt

# Lines between "begin" and "end"
sed '/begin/,/end/ s/foo/bar/' file.txt
```

### Negation — `!`

```bash
# Everywhere except line 5
sed '5! s/foo/bar/' file.txt

# On non-empty lines
sed '/^$/! s/foo/bar/' file.txt
```

---

## 3.4. Other sed commands

Substitution is just the beginning. `sed` is versatile — below are the most essential ones.

### `d` — delete a line

```bash
# Delete line 1 (header skip)
sed '1d' data.csv

# Delete lines 5-10
sed '5,10d' file.txt

# Delete blank lines
sed '/^$/d' file.txt

# Delete lines containing "DEBUG"
sed '/DEBUG/d' log.txt
```

### `p` + `-n` — print only the specified lines

`p` — print the line. `-n` — disable the default output.

```bash
# Only lines 5-10 (in Part 1/04 this was done via head|tail)
sed -n '5,10p' file.txt

# Only lines containing "ERROR" (grep equivalent)
sed -n '/ERROR/p' log.txt

# First 3 lines
sed -n '1,3p' file.txt
```

### `i` and `a` — insert/append

```bash
# Add text before line 3
sed '3i\
yangi qator
' file.txt

# Add text after line 3
sed '3a\
qoshilgan qator
' file.txt
```

::: warning macOS BSD vs GNU sed
GNU sed (Linux) supports `i\` and `a\` inline:
```bash
sed '3i\new line' file   # GNU OK, BSD ERROR
```

On macOS you always need a **newline** (like the example above).
:::

### Multiple commands — `-e` or `;`

```bash
# Both substitute and delete
sed -e 's/foo/bar/g' -e '/DEBUG/d' file.txt

# Or with a semicolon
sed 's/foo/bar/g; /DEBUG/d' file.txt
```

---

## 3.5. In-place edit — `-i` flag

So far the `sed` result went to **stdout**. With `-i` you can modify the file **in place**.

::: danger macOS problem
**This is the biggest difference between GNU sed and BSD sed.**

```bash
# GNU sed (Linux)
sed -i 's/foo/bar/g' file.txt          # ✓ works

# BSD sed (macOS)
sed -i 's/foo/bar/g' file.txt          # ❌ error
sed -i '' 's/foo/bar/g' file.txt       # ✓ works (empty string)
sed -i.bak 's/foo/bar/g' file.txt      # ✓ on both (creates a backup)
```

**Cross-platform safe way:** always use `-i.bak`. An extra `file.txt.bak` is created, but it works on both systems.
:::

```bash
# Cross-platform pattern
sed -i.bak 's/old/new/g' config.txt
rm -f config.txt.bak
```

---

## 3.6. Backreferences — returning a group

In the pattern, `\(...\)` (or `(...)` with `-E`) is a **group**. In the replacement it is referenced via `\1`, `\2`.

```bash
# Format a phone number
echo "tel 998901234567" | sed -E 's/([0-9]{3})([0-9]{2})([0-9]{7})/+\1 \2 \3/'
# tel +998 90 1234567
```

```bash
# Extract the extension from a filename
echo "report.tar.gz" | sed -E 's/(.+)\.(tar\.gz)$/Nom: \1, Tip: \2/'
# Nom: report, Tip: tar.gz
```

::: tip `-E` (extended regex)
`sed -E` — Perl-like regex. `(`, `)`, `+`, `?`, `|` can be used directly (no escaping needed).

In GNU, `-r` is the same. On macOS, using `-E` is portable across both GNU and BSD.
:::

---

## 3.7. `awk` basics — field-based processor

`awk` is a **mini-language**. It automatically splits the input into **fields** (by whitespace by default) and lets you reference them via `$1`, `$2`, ...

### The simplest example

```bash
echo "Ali 25 Toshkent" | awk '{ print $1 }'
# Ali

echo "Ali 25 Toshkent" | awk '{ print $2 }'
# 25

echo "Ali 25 Toshkent" | awk '{ print $1, $3 }'
# Ali Toshkent
```

`$0` — the whole line.

### With a file

```bash
# The first column of /etc/passwd (usernames)
awk -F':' '{ print $1 }' /etc/passwd
```

`-F':'` — input field separator (default whitespace).

### Built-in variables

| Variable | Meaning                                |
|-------------|----------------------------------------|
| `$0`        | The whole line                         |
| `$1`, `$2`, ... | The Nth field                       |
| `$NF`       | **The last field** (NF = Number of Fields)|
| `NF`        | Number of fields on the current line   |
| `NR`        | Current line number                    |
| `FS`        | Input field separator                  |
| `OFS`       | Output field separator (default whitespace)|
| `FILENAME`  | Current file name                      |

```bash
# With line numbers
awk '{ print NR, $0 }' file.txt

# Only the last column
awk '{ print $NF }' file.txt

# Field count and first field
awk '{ print NF, $1 }' file.txt
```

---

## 3.8. Pattern + action syntax

`awk` syntax: `pattern { action }`. Both are optional.

```bash
# 1. Action only — applied to each line
awk '{ print $1 }' file

# 2. Pattern only — prints matching lines
awk '/ERROR/' log.txt           # grep equivalent

# 3. Pattern + action
awk '/ERROR/ { print $1 }' log.txt
```

### Conditional pattern

```bash
# Lines where the first field is "ERROR"
awk '$1 == "ERROR" { print }' log.txt

# Second column > 100
awk '$2 > 100 { print $1, $2 }' scores.txt

# Or: the whole line contains "FAIL"
awk '/FAIL/ { print NR, $0 }' log.txt
```

### Logical operators

```bash
# Both ERROR and 500
awk '/ERROR/ && /500/' log.txt

# Or either of the two
awk '/ERROR/ || /FATAL/' log.txt

# Negation
awk '!/DEBUG/' log.txt
```

---

## 3.9. `BEGIN` and `END` blocks

`BEGIN { }` — runs **once** before any line is read.
`END { }` — runs **once** after all lines have been read.

### Classic counter

```bash
awk '
BEGIN { count = 0 }
/ERROR/ { count++ }
END   { print "Jami xatolar:", count }
' app.log
```

### Table with a header

```bash
awk '
BEGIN { print "ID\tFOYDALANUVCHI" }
{ print NR "\t" $1 }
' users.txt
```

### Sum

```bash
# Sum of the first column
awk '{ sum += $1 } END { print sum }' numbers.txt

# Average value
awk '{ sum += $1; n++ } END { print sum/n }' numbers.txt
```

---

## 3.10. Arrays — counter pattern

In `awk`, arrays are **string-indexed** (just like an associative array).

### Word frequency

```bash
echo "salom dunyo bash salom uz bash bash" | \
  awk '{
    for (i=1; i<=NF; i++) count[$i]++
  } END {
    for (word in count) print count[word], word
  }'
```

Result:
```text
1 dunyo
1 uz
2 salom
3 bash
```

### IP statistics in an Apache access log

```bash
awk '{ count[$1]++ } END {
  for (ip in count) print count[ip], ip
}' access.log | sort -rn | head -10
```

The top 10 IPs that sent the most requests.

### Sum by category

```bash
# users.txt:
#   ali     25  IT
#   vali    30  IT
#   gulnora 28  HR
#   bobur   35  IT

awk '{ sum[$3] += $2 } END {
  for (dept in sum) print dept, sum[dept]
}' users.txt
```

Result:
```text
IT 90
HR 28
```

---

## 3.11. `printf` — formatted output

`print` is simple. `printf` — formatted like in C:

```bash
awk '{ printf "%-10s %5d\n", $1, $2 }' data.txt
```

`%-10s` — left-aligned 10-character text.
`%5d` — a 5-character number (right-aligned).
`\n` — newline (automatic in `print`, must be added in `printf`).

Example:
```bash
echo "ali 100
bobur 99
saida 1000" | awk '{ printf "%-10s %5d\n", $1, $2 }'
# ali          100
# bobur         99
# saida       1000
```

---

## 3.12. Control logic (if, for, while)

Inside an `awk` action block you can write full programs:

```bash
# if/else
awk '{
  if ($2 >= 60) print $1, "passed"
  else          print $1, "failed"
}' scores.txt

# for loop
awk '{
  for (i=1; i<=NF; i++) print i ":", $i
}' file.txt

# while
awk '{
  i = 1
  while (i <= NF) { print $i; i++ }
}' file.txt
```

---

## 3.13. Real production examples

### Example 1 — Apache log statistics

`access.log` in standard format:
```
192.168.1.5 - - [10/Oct/2026:14:22:01] "GET /api/users HTTP/1.1" 200 1234
192.168.1.5 - - [10/Oct/2026:14:22:02] "GET /api/posts HTTP/1.1" 404 89
...
```

**Top 10 IPs:**
```bash
awk '{ print $1 }' access.log | sort | uniq -c | sort -rn | head -10
```

**Statistics by status code:**
```bash
awk '{ print $9 }' access.log | sort | uniq -c | sort -rn
```

**Only 5xx errors:**
```bash
awk '$9 >= 500 { print }' access.log
```

**Largest response (last column — bytes):**
```bash
awk '{ print $NF, $7 }' access.log | sort -rn | head -5
```

### Example 2 — Disk usage analysis

Sort the result of `du -sh ~/*` by size (`sort -h` also exists, but here with awk):

```bash
du -sh ~/* 2>/dev/null | sort -hr | head -10
```

Or filter with awk — those larger than 100MB:

```bash
du -k ~/* 2>/dev/null | awk '$1 > 102400 { print $1/1024 "MB", $2 }' | sort -rn
```

### Example 3 — CSV transform

`users.csv`:
```
ali,25,toshkent
vali,30,samarqand
gulnora,28,buxoro
```

**Only those aged 27+:**
```bash
awk -F',' '$2 >= 27 { print $1, $3 }' users.csv
# vali samarqand
# gulnora buxoro
```

**Output as JSON:**
```bash
awk -F',' '
BEGIN { print "[" }
{ printf "  {\"name\":\"%s\",\"age\":%d,\"city\":\"%s\"}%s\n",
    $1, $2, $3, (NR==total ? "" : ",")
}
END { print "]" }
' total=$(wc -l < users.csv) users.csv
```

### Example 4 — Updating configuration

```bash
# Change the port in nginx.conf
sed -i.bak 's/listen 80;/listen 8080;/' /etc/nginx/nginx.conf

# Change API_KEY in the .env file
sed -i.bak 's/^API_KEY=.*/API_KEY=new-secret-key/' .env

# Update the package.json version
sed -i.bak -E 's/"version": "[0-9.]+"/"version": "2.0.0"/' package.json
```

### Example 5 — Log file analysis

```bash
# Most frequent errors in the last 100 lines
tail -n 100 app.log \
  | awk '/ERROR/ { print $4 }' \
  | sort | uniq -c | sort -rn | head -5
```

---

## 3.14. `sed` vs `awk` — when to use what?

| Task                            | Choice         | Reason                               |
|---------------------------------|----------------|--------------------------------------|
| Replace a single pattern        | **`sed`**      | Shorter, faster                      |
| Delete / slice out lines        | **`sed`**      | `d`, `p`, address syntax             |
| Working with columns            | **`awk`**      | `$1`, `$2` field-based               |
| Arithmetic operations           | **`awk`**      | `sum += $2`, `count++`               |
| Conditional logic               | **`awk`**      | `if/else`, `for`, `while`            |
| Counter / aggregate             | **`awk`**      | `count[$1]++` paradigm               |
| In-place edit                   | **`sed -i`**   | in awk it's `-i inplace` (GNU only)  |
| Multiline transformation        | **`awk`**      | or Perl, sed is complicated          |

::: tip Rule of thumb
- **`sed`** — simple substitution at the `s/A/B/` level
- **`awk`** — any logic working with multiple fields
- **When both get complicated** — it's time to move to `python` or `perl`
:::

---

## 3.15. Modern alternatives

`sed` and `awk` were created in the 1970s. Modern alternatives:

| Tool      | Job                               | Installation             |
|-------------|-----------------------------------|--------------------------|
| `ripgrep` (`rg`) | a faster version of grep      | `brew install ripgrep`   |
| `sd`        | a simple alternative to sed       | `cargo install sd`       |
| `miller` (`mlr`) | unified tool for CSV/JSON/awk  | `brew install miller`    |
| `jq`        | JSON parser (powerful)            | `brew install jq`        |
| `xsv`       | dedicated to CSV                  | `cargo install xsv`      |

But: **`sed` and `awk` are available on every system.** Learn them first, then add modern tools.

---

## 3.16. Common mistakes

::: danger Classic pitfalls

1. **macOS `sed -i` errors without an empty argument.**
   ```bash
   sed -i 's/old/new/' file       # ❌ macOS
   sed -i '' 's/old/new/' file    # ✓ macOS
   sed -i.bak 's/old/new/' file   # ✓ both GNU and BSD
   ```

2. **`/` is awkward to escape — change the delimiter.**
   ```bash
   sed 's/\/usr\/bin\///'   # ❌ unreadable
   sed 's|/usr/bin/||'      # ✓
   ```

3. **Forgetting the `g` flag.**
   ```bash
   sed 's/foo/bar/'  fayl    # replaces only the 1st match
   sed 's/foo/bar/g' fayl    # all of them
   ```

4. **Using a variable with `$` in `awk` (confusing it with Bash).**
   ```bash
   awk '{ x = 5; print $x }'    # $x = the 5th field
   awk '{ x = 5; print x }'     # the value of x = 5
   ```

5. **Forgetting `-E` (extended regex) — `(...)` doesn't work.**
   ```bash
   sed 's/(foo|bar)/X/g'      # ❌ literal (
   sed -E 's/(foo|bar)/X/g'   # ✓
   ```

6. **Not changing the `awk` separator.**
   ```bash
   awk '{ print $1 }' file.csv          # ❌ treated as whitespace — broken
   awk -F',' '{ print $1 }' file.csv    # ✓
   ```

7. **Confusing `NR` and `NF`.**
   - `NR` = line number
   - `NF` = field count
:::

---

## 3.17. Exercises

::: tip 🧪 Bashlings — interactive exercises
This chapter's **6** exercises with auto-checking via the `bashlings` CLI:

```bash
bashlings watch              # start from the first pending exercise
bashlings run sed1           # check a single exercise
bashlings hint sed1          # step-by-step hint
```

Source: [`exercises/08_text_advanced/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/08_text_advanced)
:::

And try the following conceptual exercises by hand yourself:

1. **Email mask** — turn `ali@example.com` into `a***@example.com`. With `sed` regex.

2. **Clean up blank lines** — write a `sed` chain that deletes all blank and whitespace-only lines from a file.

3. **Top 5 words** — from an input text file, print the 5 most frequent words and their counts (`awk` + `sort`).

4. **CSV averager** — write an `awk` script: compute the average value of the (numeric) 3rd column of a CSV file.

5. **Update the version** — a cross-platform script (macOS + Linux) that changes `"version": "X.Y.Z"` inside `package.json` to a new `"version": "1.2.3"`.

---

## 3.18. Summary

| Concept                    | Key point                                      |
|----------------------------|------------------------------------------------|
| `sed 's/A/B/'`             | Replaces the first `A` with `B`                |
| `sed 's/A/B/g'`            | **All** matches                                |
| `sed '5,10d' fayl`         | Deletes lines 5-10                             |
| `sed -n '/pattern/p'`      | Only matching lines                            |
| `sed -E 's/(...)/\1/'`     | Backreference + extended regex                 |
| `sed -i.bak '...' fayl`    | In-place edit (cross-platform safe)            |
| `awk '{ print $1 }'`       | The first field                                |
| `awk -F','`                | Input separator — comma                        |
| `awk '$2 > 10'`            | Conditional filter                             |
| `awk 'NR==1'`              | Only line 1                                    |
| `BEGIN { } / END { }`      | Start/end blocks                               |
| `count[$1]++`              | Counter pattern (associative array)            |

### 5 key ideas

1. **grep finds, sed edits, awk computes.** Choose the right one based on the task.
2. **macOS BSD sed** — always use `-i.bak` (or install `gnu-sed`).
3. **Counter pattern** (`count[$1]++; END { for ... }`) — the most used idiom in DevOps.
4. **`$NF` for the last field** — ideal in log files for response size, end-of-line.
5. **Combine in a pipeline** — `grep | awk | sort | uniq -c | head` is a real workflow.

🎉 Now you have industrial-grade text-processing skills. In the next chapter we'll learn to write robust scripts via **signals and traps** — `Ctrl+C` being pressed, cleanup, graceful shutdown.

> **Next page:** [4. Signals and traps →](./04-traps-signals)
