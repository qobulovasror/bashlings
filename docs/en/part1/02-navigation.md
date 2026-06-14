---
title: "The file system and navigation"
description: "Learn to work professionally in the file system with the ls, cd, pwd, mkdir, rm, cp, mv commands."
---

# 2. Navigating the file system

> What you will learn in this chapter:
> - **`pwd`** / **`ls`** / **`cd`** — where am I, what's here, where do I go?
> - **`mkdir`** / **`touch`** — creating directories and files
> - **`cp`** / **`mv`** / **`rm`** — copying, moving, deleting
> - The difference between **absolute and relative** paths (`/etc/hosts` vs `../config`)
> - Hidden files (those starting with `.`) and glob patterns
>
> **⏱ Time:** ~25 minutes
> **🧪 Exercises:** `bashlings watch` — 8 interactive exercises ready ([`exercises/02_navigation/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/02_navigation))

The Linux/Unix file system is **one big tree**. Its root is represented by the `/` symbol. Everything lives under this root: user files, programs, devices, and even system settings.

```text
/
├── bin/      # core executable files (ls, cat, ...)
├── etc/      # configuration files
├── home/     # user directory (Linux)
├── Users/    # user directory (macOS)
├── tmp/      # temporary files
├── usr/      # additional programs and libraries
└── var/      # logs, mail, and variable data
```

## 2.1. `pwd` — where am I?

**`pwd`** (Print Working Directory) — shows the **full path** of the directory you are currently in.

```bash
$ pwd
/Users/mac/Desktop/projects
```

::: tip Absolute and Relative paths
- **Absolute path** — starts from `/`: `/Users/mac/Documents`
- **Relative path** — relative to the current directory: `./docs` or `../images`
:::

## 2.2. `ls` — files in a directory

**`ls`** (list) — lists the files in the current or a specified directory.

```bash
ls                    # simple listing
ls -l                 # detailed (long format)
ls -a                 # including hidden files (.)
ls -lh                # human-readable size (KB, MB)
ls -la                # the most commonly used combination
ls -lt                # ordered by time
ls -lS                # ordered by size
ls /etc               # contents of another directory
ls *.md               # only .md files
```

Output of `ls -l`:

```text
-rw-r--r--  1 mac  staff   2.1K Mar 14 10:22 README.md
drwxr-xr-x  4 mac  staff   128B Mar 12 09:00 src
```

Each column:

| Column        | Meaning                                  |
|---------------|------------------------------------------|
| `-rw-r--r--`  | Permissions                              |
| `1`           | Number of hard links                     |
| `mac`         | Owner                                    |
| `staff`       | Group                                    |
| `2.1K`        | Size                                     |
| `Mar 14 10:22`| Modification time                        |
| `README.md`   | File name                                |

::: info Permission symbols
- `-` — regular file, `d` — directory, `l` — symlink
- `r` — read, `w` — write, `x` — execute
- Order: **owner / group / others**
:::

## 2.3. `cd` — change directory

**`cd`** (Change Directory):

```bash
cd /etc                # by absolute path
cd Documents           # to Documents inside the current directory
cd ..                  # one level up
cd ../..               # two levels up
cd ~                   # to the home directory
cd                     # also to the home directory (shortcut)
cd -                   # back to the previous directory
cd /                   # to the root
```

::: tip Shortcuts
| Symbol   | Meaning                              |
|----------|--------------------------------------|
| `~`      | Home directory (`/Users/mac`)        |
| `.`      | Current directory                    |
| `..`     | Parent directory                     |
| `-`      | Previous directory (with `cd -`)     |
:::

## 2.4. `mkdir` — create a new directory

```bash
mkdir loyiha                       # a single directory
mkdir loyiha tests docs            # multiple directories
mkdir -p src/components/buttons    # nested (parents are created too)
mkdir -v new_folder                # verbose: shows what was done
```

The `-p` (parent) flag is very powerful — if directories in the path don't exist, it creates them all:

```bash
mkdir -p projects/2026/january/reports
# all 4 levels are created
```

::: warning `mkdir` errors
Without `-p`, if a `parent` directory does not exist, it raises an error:

```text
mkdir: a/b/c: No such file or directory
```
:::

## 2.5. `touch` — create an empty file

```bash
touch readme.txt                   # a new empty file
touch file1.txt file2.txt          # multiple files
touch -t 202601011200 old.txt      # create with a set timestamp
```

::: info An extra job of `touch`
If the file already exists, `touch` updates its **last modification time** (not its contents).
:::

## 2.6. `cp` — copy

```bash
cp manba.txt nusxa.txt              # copy of a file
cp file.txt /tmp/                    # to another directory
cp -r src/ backup/                   # recursive (for directories)
cp -v a.txt b.txt                    # verbose
cp -i a.txt b.txt                    # interactive (asks before overwriting)
cp -p file.txt copy.txt              # preserve permissions and timestamp
cp *.md docs/                        # all .md files
```

::: warning `-r` is required
If you want to copy a directory, don't forget the `-r` (recursive) flag:

```bash
cp folder1 folder2          # WRONG
cp -r folder1 folder2       # CORRECT
```
:::

## 2.7. `mv` — move or rename

`mv` does two jobs: **moving files to another location** and **renaming**.

```bash
mv eski.txt yangi.txt                # rename
mv file.txt /tmp/                    # move
mv *.log logs/                       # multiple files
mv -i a.txt b.txt                    # interactive
mv -n a.txt b.txt                    # don't overwrite if it exists
mv -v old.txt new.txt                # verbose
```

::: tip Trick
`mv` doesn't require `-r` for directories — it always works "fully":

```bash
mv src/ archive/   # works correctly
```
:::

## 2.8. `rm` — delete

::: danger The most dangerous command!
A file deleted with `rm` **cannot be recovered**. It doesn't go to the Trash. It's gone — that's it.
:::

```bash
rm fayl.txt              # a single file
rm a.txt b.txt c.txt     # multiple files
rm -i fayl.txt           # asks for confirmation
rm -f fayl.txt           # force, without asking
rm -r katalog/           # recursive (for directories)
rm -rf katalog/          # FORCE + RECURSIVE — be careful!
```

### Dangerous antipatterns

```bash
rm -rf /                 # ❌ deletes the entire system
rm -rf $UNDEFINED_VAR/*  # ❌ if the variable is empty = rm -rf /
rm -rf * .git            # ⚠ watch out for the space
```

::: tip Safe practice
1. Instead of `rm`, use safe-delete utilities like `trash` or `gomi`.
2. Before ever using `rm -rf` with a variable, check it with `echo`:

```bash
echo rm -rf "$PROJECT_DIR"   # first we look
rm -rf "$PROJECT_DIR"        # now we run it
```
:::

## 2.9. `rmdir` — remove an empty directory

`rmdir` only removes **empty directories**:

```bash
rmdir empty_folder       # removes it if empty
rmdir non_empty          # ERROR: "Directory not empty"
```

For a non-empty directory, use `rm -r`.

## 2.10. `tree` — tree view

Displays the directory structure visually (you may need to install it separately):

```bash
brew install tree    # macOS
sudo apt install tree  # Ubuntu/Debian

tree                  # current directory
tree -L 2             # up to 2 levels
tree -a               # including hidden files
tree -d               # only directories
```

Output:

```text
.
├── docs/
│   ├── part1/
│   │   ├── 01-introduction.md
│   │   └── 02-navigation.md
│   └── index.md
└── package.json
```

## 2.11. Wildcards (Globbing)

Bash supports **wildcard** (joker) symbols for specifying multiple files:

| Symbol     | Meaning                                  | Example                          |
|------------|------------------------------------------|----------------------------------|
| `*`        | Any character(s)                         | `*.txt`, `report-*`              |
| `?`        | Exactly one character                    | `file?.txt`                      |
| `[abc]`    | One of a, b, or c                        | `f[123].log`                     |
| `[a-z]`    | A range                                  | `[a-c]*.md`                      |
| `{a,b}`    | Brace expansion                          | `{src,test}/*.js`                |

Examples:

```bash
ls *.md                       # all .md files
rm temp_?.log                 # temp_1.log, temp_2.log, ...
cp report-{2024,2025}.pdf bk/  # works with both
ls /etc/[a-c]*                # those starting with a, b, or c
```

## 2.12. Real example: creating a project skeleton

```bash
# 1. Main directory
mkdir -p ~/projects/my-app
cd ~/projects/my-app

# 2. Structure
mkdir -p src/{components,utils,services} tests docs

# 3. Empty files
touch README.md .gitignore package.json
touch src/index.js src/utils/helpers.js

# 4. Let's check
tree -L 3
```

## 2.13. Exercises

::: tip 🧪 Bashlings — interactive exercises
This chapter's **8** exercises come with auto-checking via the `bashlings` CLI:

```bash
bashlings watch              # start from the first pending exercise
bashlings run nav1           # check a single exercise
bashlings hint nav1          # step-by-step hint
```

Source: [`exercises/02_navigation/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/02_navigation)
:::

Do the following additional tasks by hand in the terminal:

1. In your home directory, create a directory named `bash-mashq` and, inside it, create the subdirectories `notes`, `scripts`, and `archive` with a single command.
2. Inside `scripts/`, create the files `hello.sh`, `backup.sh`, and `cleanup.sh`.
3. Make a copy of `hello.sh` named `hello.bak` in the same directory.
4. Delete the `archive` directory.
5. Move all `.sh` files into the `notes/` directory with a single command.

## 2.14. Summary

| Command   | Purpose                                     |
|-----------|---------------------------------------------|
| `pwd`     | Show the current directory                  |
| `ls`      | View the contents of a directory            |
| `cd`      | Change directory                            |
| `mkdir`   | Create a new directory                      |
| `touch`   | Create a new empty file                     |
| `cp`      | Copy                                        |
| `mv`      | Move or rename                              |
| `rm`      | Delete (with care!)                         |
| `rmdir`   | Remove an empty directory                   |
| `tree`    | Display in a tree view                      |

Now you have the skill to move freely around the file system. In the next chapter we will learn to connect commands through **I/O redirection** and **pipelines**.

> **Next page:** [3. I/O Redirection and Pipelines →](./03-pipes-redirection)
