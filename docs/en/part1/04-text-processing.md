---
title: "Text Processing Basics"
description: "Read, search, and count text files using cat, less, head, tail, grep, and wc."
---

# 4. Text Processing Basics

> What you'll learn in this chapter:
> - **`cat`** / **`less`** — view the contents of a file
> - **`head`** / **`tail`** — view the beginning or the end
> - **`grep`** — find a PATTERN inside text (`-i`, `-r`, `-c`, `-v`)
> - **`wc`** — count lines/words/characters
> - Patterns for chaining these together with `|`
>
> **⏱ Time:** ~20 minutes
> **🧪 Exercises:** `bashlings watch` — 5 interactive exercises ready ([`exercises/04_text/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/04_text))

In the Unix world, **everything is text**: configuration files, logs, code, even network traffic. That is why reading, searching, and measuring text efficiently is a core skill for every engineer.

In this chapter we'll learn the **six** most commonly used commands: `cat`, `less`, `head`, `tail`, `grep`, `wc`.

## 4.1. `cat` — print the contents of a file

`cat` (concatenate) prints the contents of a file to `stdout`.

```bash
cat hello.txt                # contents
cat -n hello.txt             # with line numbers
cat -A hello.txt             # show invisible characters ($ end-of-line)
cat -s file.txt              # squeeze consecutive blank lines into 1

# Concatenate and print several files
cat header.txt body.txt footer.txt

# Concatenate and write to a file
cat part1.txt part2.txt > full.txt
```

::: warning The `cat` antipattern
A mistake beginners often make:

```bash
cat file.txt | grep "foo"     # ❌ unnecessary cat
grep "foo" file.txt           # ✅ correct
```

This is called "UUOC" — *Useless Use of Cat*.
:::

## 4.2. `less` — interactive viewing

If you `cat` a large file, it scrolls off the screen. Use `less` to read it page by page:

```bash
less /var/log/system.log
less -N file.txt        # with line numbers
less +F access.log      # follow mode (new lines in real time)
```

Quick commands inside it:

| Key            | What it does                    |
|----------------|---------------------------------|
| `↑` `↓` or `j` `k` | Up / Down                    |
| `Space` / `b`  | Page down / up                  |
| `g` / `G`      | To the start / end              |
| `/word`        | Search                          |
| `n` / `N`      | Next / Previous result          |
| `q`            | Quit                            |
| `h`            | Help                            |

::: tip `less` vs `more`
`more` is an older command — it can only move down. `less` is more powerful, moving both ways. You've probably heard the phrase "**less is more**" — this is exactly where it comes from 😉
:::

## 4.3. `head` — view the beginning

```bash
head file.txt          # first 10 lines (default)
head -n 5 file.txt     # first 5 lines
head -5 file.txt       # shorthand form
head -c 100 file.txt   # first 100 bytes
head *.log             # for several files
```

## 4.4. `tail` — view the end

```bash
tail file.txt              # last 10 lines
tail -n 20 file.txt        # last 20 lines
tail -f /var/log/nginx.log # FOLLOW — new lines in real time
tail -F file.txt           # follow + retry (even if the file is deleted)
tail -n +5 file.txt        # from line 5 to the end
```

::: tip `tail -f` is ideal for monitoring server logs
The most frequently used command when live-debugging a server:

```bash
tail -f /var/log/app.log | grep ERROR
```
:::

### `head` + `tail` = the middle part

```bash
# Get lines 50-60
sed -n '50,60p' file.txt
# or
head -60 file.txt | tail -11
```

## 4.5. `wc` — count lines, words, characters

`wc` (word count):

```bash
wc file.txt
# 42   210  1573  file.txt
# lines words bytes name

wc -l file.txt          # lines only
wc -w file.txt          # words only
wc -c file.txt          # bytes only
wc -m file.txt          # characters (accounting for multibyte)
```

Real examples:

```bash
ls | wc -l                       # number of files in a directory
cat /etc/passwd | wc -l          # number of system users
grep -c "ERROR" app.log          # number of errors (-c)
find . -name "*.js" | wc -l      # number of .js files in the project
```

## 4.6. `grep` — searching text (power!)

`grep` — **Global Regular Expression Print**. The most powerful text-search tool in the Linux world.

```bash
grep "kalit" file.txt              # lines that contain "kalit"
grep -i "error" log.txt            # case-insensitive (lower/upper doesn't matter)
grep -n "TODO" code.py             # with line numbers
grep -c "warning" log.txt          # how many times it occurred (count only)
grep -v "INFO" log.txt             # lines that do NOT contain INFO (invert)
grep -r "API_KEY" .                # recursively in all files
grep -l "import" *.py              # list of FILE NAMES (no content)
grep -w "cat" file.txt             # ONLY the whole word (won't match cathedral)
grep -A 3 "ERROR" log.txt          # 3 lines after ERROR
grep -B 2 "ERROR" log.txt          # 2 lines before ERROR
grep -C 2 "ERROR" log.txt          # 2 lines around ERROR
```

### With Regular Expressions

```bash
grep "^Salom" file.txt        # lines starting with Salom
grep "tugadi$" file.txt       # lines ending with tugadi
grep "[0-9]\{3\}" log.txt     # a 3-digit pattern
grep -E "(error|fail)" log    # extended regex: error OR fail
```

::: tip `egrep` and `fgrep`
- `grep -E` = `egrep` — extended regex
- `grep -F` = `fgrep` — fixed text (no regex, faster)
:::

### Real examples

```bash
# 404 errors in an Apache log
grep " 404 " /var/log/apache/access.log

# TODO and FIXME in code
grep -rn -E "(TODO|FIXME|XXX)" src/

# Find IP addresses
grep -E -o "([0-9]{1,3}\.){3}[0-9]{1,3}" access.log

# Find email addresses
grep -E -o "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}" contacts.txt
```

## 4.7. Additional useful commands

### `sort` — sorting

```bash
sort file.txt              # alphabetically
sort -r file.txt           # reverse
sort -n numbers.txt        # numerically
sort -u file.txt           # unique (removes duplicates)
sort -k 2 file.txt         # by the 2nd column
sort -t',' -k 3 csv.txt    # by the 3rd column in a CSV
```

### `uniq` — handling duplicates

```bash
sort file.txt | uniq                # non-duplicated lines
sort file.txt | uniq -c             # how many times each occurred
sort file.txt | uniq -d             # ONLY the duplicated lines
```

::: warning `uniq` only finds consecutive duplicates
That's why it's always used in combination with `sort`.
:::

### `cut` — extracting columns

```bash
cut -d':' -f1 /etc/passwd            # first column (user name)
cut -d',' -f1,3 data.csv             # columns 1 and 3
cut -c1-10 file.txt                  # characters 1-10 of each line
```

### `tr` — transforming characters

```bash
echo "salom" | tr 'a-z' 'A-Z'        # SALOM
echo "a,b,c,d" | tr ',' '\n'         # comma to newline
cat file.txt | tr -d ' '             # delete spaces
cat file.txt | tr -s ' '             # squeeze consecutive spaces into 1
```

## 4.8. Real-world examples

### Top 10 IPs — those that sent the most requests

```bash
awk '{print $1}' access.log \
  | sort \
  | uniq -c \
  | sort -rn \
  | head -10
```

### Directories taking up the most disk space

```bash
du -h /var | sort -rh | head -20
```

### Error statistics in a log file

```bash
grep -E "ERROR|WARN|FATAL" app.log \
  | awk '{print $4}' \
  | sort \
  | uniq -c \
  | sort -rn
```

### Counting lines of code in a project

```bash
find . -name "*.py" -exec wc -l {} + | sort -n
```

### List of users and their shells

```bash
cut -d':' -f1,7 /etc/passwd
```

## 4.9. Common mistakes

::: danger Note

1. **`cat | grep` — unnecessary.** `grep "x" file` is correct.
2. **Special characters in `grep` regex.** Escape `.` `*` `?`: `grep "\.com"`.
3. **The order of `sort | uniq` matters.** `uniq` does not sort by itself.
4. **`tail -f` doesn't detect log rotation.** Use `tail -F` (capital F).
5. **Don't `cat` large files.** Use `less`.
:::

## 4.10. Exercises

::: tip 🧪 Bashlings — interactive exercises
The **5** exercises for this chapter come with auto-checking via the `bashlings` CLI:

```bash
bashlings watch              # start from the first pending exercise
bashlings run text1          # check a single exercise
bashlings hint text1         # step-by-step hint
```

Source: [`exercises/04_text/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/04_text)
:::

Do the following additional tasks by hand in the terminal:

1. Find out how many users in `/etc/passwd` use `/bin/bash`.
2. Sort the 5 largest files in the current directory by size.
3. From the output of `dmesg`, print and count all lines containing the word `error` (case-insensitive).
4. Find the 10 most-used commands in your `~/.bash_history` file.
5. Write a pipeline that, from the output of `ls -la`, prints only the directories (those starting with `d`).

## 4.11. Summary

| Command   | What it does                                |
|-----------|---------------------------------------------|
| `cat`     | Print a file in full, concatenate           |
| `less`    | Interactive viewing (for large files)       |
| `head`    | First N lines                               |
| `tail`    | Last N lines, follow with `-f`              |
| `grep`    | Search text, with regex                     |
| `wc`      | Count lines, words, characters              |
| `sort`    | Sorting                                     |
| `uniq`    | Handling duplicates                         |
| `cut`     | Extracting columns                          |
| `tr`      | Replacing/deleting characters               |

Now you can read, search, and count any text data. This is 80% of the daily work of a DevOps engineer and system administrator.

In the next chapter we'll write our **first Bash script**!

> **Next page:** [5. First Bash script →](./05-basic-scripting)
