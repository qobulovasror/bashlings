---
title: "Glossary of terms"
description: "Bash, Linux and DevOps terms — with explanations and context."
---

# Glossary of terms

This page is the **single source of truth** for all the technical terms that appear in the course. To avoid translation inconsistency, each term is used the same way.

The terms are in alphabetical order.

---

## A

### `alias`
A shorthand for a command. `alias ll='ls -la'` — if you type `ll`, `ls -la` is executed. It applies only in the current shell session (for a permanent one, add it to `~/.bashrc`).

### Argument
A value passed to a script or function. In Bash: `$1`, `$2`, ... `$@`, `$#`.

### `awk`
A domain-specific programming language that works on text data with "column-row" logic. Example: `awk '{print $2}'` — prints the 2nd column of each line.

---

## B

### `bash`
"Bourne Again SHell" — the default shell on Linux. Our entire course is for this shell.

### `bashlings`
The **CLI runner** of this project (written in Rust). It auto-checks exercises, renders hints, and runs in watch mode.

### `binary`
A **compiled** program (not a text script). Example: `/bin/ls` — a binary.

### Built-in
A command that lives INSIDE the shell ITSELF (`cd`, `echo`, `pwd`, `read`, `[`, `test`). Not an external program — so it runs fast.

---

## C

### Command substitution
`$(cmd)` or backtick `\`cmd\`` — capturing a command's stdout result into a variable. Example: `now=$(date +%F)`.

### Container
In Docker — a running instance of an image. You can create many containers from a single image.

### `cron`
A system daemon that runs recurring tasks on a time schedule. Edited via `crontab -e`.

### Crontab
The configuration file for `cron`. Each line: `minute hour day month dow command`.

### `curl`
An HTTP client — sends a request to a URL and gets the response. The standard combination for scripts is `-fsSL`.

---

## D

### Daemon
A process that runs continuously in the background. Example: `cron`, `sshd`, `dockerd`. On Linux its name usually ends with `d`.

### `docker`
A containerization platform. An image is a template, a container is its running instance.

### `Dockerfile`
The instructions for building a Docker image. The `FROM`, `RUN`, `COPY`, `CMD` directives.

---

## E

### `echo`
Prints arguments to stdout. A Bash built-in. Example: `echo "Salom"`.

### Environment variable
A key-value pair passed to a process. Exported via `export VAR=value`. Example: `$PATH`, `$HOME`, `$USER`.

### Exit code (return code)
The integer (0–255) a command returns when it finishes. `0` = success, `>0` = error. You can get the last exit code via `$?`.

---

## F

### File descriptor (FD)
The number of a file/stream opened for a process. The standard ones:
- `0` = stdin
- `1` = stdout
- `2` = stderr

### `for`
A loop construct. In Bash: `for x in a b c; do echo "$x"; done`.

### Function
A reusable, named block of code. Syntax: `name() { ... }`.

---

## G

### `git`
A distributed version control system. Used to clone our project and to save exercises.

### Glob
A file-system pattern: `*.sh`, `file?.txt`, `[abc].md`. Bash expands it itself (not regex).

### `grep`
A tool for finding a PATTERN in text. `grep -i` (case-insensitive), `-r` (recursive), `-c` (count).

---

## H

### Heredoc
A multi-line string syntax: `cat <<EOF\n...\nEOF`. `<<'EOF'` (with quotes) — NO variable interpolation.

### `home` directory
The user's personal directory (`$HOME` or `~`). Example: `/Users/ali` (macOS) or `/home/ali` (Linux).

---

## I

### Image (Docker)
A file-system template for a container. **Read-only** (immutable).

### Interactive mode
The shell is in conversation with the user — you type commands at the prompt (vs. **script mode** — commands inside a file are executed).

---

## J

### `jq`
A command-line parser for JSON. `jq '.field'` — get a field, `jq -r` — raw (without quotes).

### Job
A process that runs within the shell. `jobs` — list, `bg`/`fg` — control.

---

## K

### Kernel
The core of the operating system — it works directly with the hardware. On Linux it's the `Linux kernel`; on macOS it's `XNU`.

---

## L

### Locale
A language/format setting (`LC_ALL`, `LANG`). Example: `en_US.UTF-8`. It affects the results of `awk` and `sort`.

### Loop
A construct for repeated execution. In Bash: `for`, `while`, `until`.

---

## M

### `man`
A manual page — a full guide about a command. `man bash`, `man curl`.

### Marker (I AM NOT DONE)
In Bashlings, the `# I AM NOT DONE` line in every exercise file. It's removed after you finish the exercise — the CLI uses it as an `is_done` indicator.

### Multiplexing (SSH)
Multiple SSH sessions over a single TCP connection (`ControlMaster`). For fast reconnection.

---

## P

### `PATH`
An environment variable — it tells where commands should be searched for. A list of directories separated by `:`. Example: `/usr/local/bin:/usr/bin:/bin`.

### Pipe (`|`)
Connecting one command's stdout to another's stdin. `ls | wc -l` — the number of files.

### Pipeline
Two or more commands connected via a pipe. Example: `cat log | grep error | sort | uniq -c`.

### PID
Process ID — the unique number of each running process. `echo $$` — the PID of the current shell.

### Process
A running instance of a program. Each process has a PID, a parent (PPID), an env and FDs.

---

## Q

### Quoting
The way to wrap strings in Bash:
- `"..."` double quotes — variables expand, `$(cmd)` works
- `'...'` single quotes — nothing changes, literal
- Unquoted — sensitive to whitespace and glob expansion

---

## R

### Redirect
Directing a stream to a file:
- `>` — stdout, the file is OVERWRITTEN
- `>>` — stdout, APPENDED to the end of the file
- `<` — stdin from a file
- `2>&1` — stderr also to the stdout location

### `rsync`
A synchronization tool that copies only the CHANGED parts. Classic flags: `-avz --delete`.

---

## S

### `scp`
Secure Copy — copying files over SSH. Syntax: `scp src user@host:dst`.

### `sed`
Stream EDitor — replacing and editing in text. Example: `sed 's/old/new/g'`.

### Shebang
The first line of a script file — it specifies which interpreter it runs with. Example: `#!/usr/bin/env bash`.

### Shell
The interpreter between the user and the kernel. Examples: `bash`, `zsh`, `fish`, `sh`.

### `ShellCheck`
A static linter for Bash scripts. It finds possible errors and anti-patterns.

### Signal
An asynchronous message sent to a process. `SIGINT` (Ctrl+C), `SIGTERM` (polite termination), `SIGKILL` (forced kill).

### Script
A text file whose commands are executed one after another by the shell. In our project — `.sh` files.

### `ssh`
Secure SHell — an encrypted connection to a remote server. `ssh user@host`.

### Stage (Docker multi-stage)
`FROM ... AS name` — a separate build stage inside a Dockerfile. Only the last stage remains in the final image.

### stderr (FD 2)
The standard error stream — error messages are usually directed here.

### stdin (FD 0)
The standard input stream — the data a command takes from the user or from a pipe.

### stdout (FD 1)
The standard output stream — the normal result of a command.

### Subshell
A separate shell process running inside the main shell. Commands inside parentheses `(...)` are executed in a subshell.

---

## T

### `tar`
An archiving tool. Classic flags: `tar -czvf archive.tar.gz dir/` (create, gzip, verbose, file).

### `tee`
Directing a pipe's output BOTH to a file AND to stdout. `echo X | tee log.txt`.

### Terminal
A graphical program (window) for typing commands and seeing the result. Example: Terminal.app, iTerm2, GNOME Terminal. The shell runs INSIDE the terminal.

### `trap`
A Bash signal handler — it specifies what to do when a script receives a signal. `trap cleanup EXIT`.

---

## V

### Variable
A variable. In Bash it's set with `name=value` (NO whitespace) and read via `$name`.

### VitePress
A static site generator from Markdown (based on Vue.js). This course is rendered with this tool.

---

## W

### `watch` (bashlings)
Auto-check mode — when a file is saved, the exercise is re-checked. `bashlings watch`.

### `wget`
An HTTP client (an alternative to curl) — geared toward downloading files. `wget -c` — resume a partial download.

---

## Y

### YAML
"YAML Ain't Markup Language" — a data serialization format. It depends on indentation. Docker compose and GitHub Actions use YAML.

---

## Z

### `zsh`
A shell almost compatible with Bash — the default on macOS. This course is aimed at **bash**, but many exercises work in zsh too.

---

## Translation notes

| English        | Uzbek (recommended)           | Reason                          |
|----------------|-------------------------------|--------------------------------|
| array          | massiv                        | familiar from mathematics      |
| associative array | lug'at / assotsiativ massiv | both are accepted              |
| flag           | flag                          | no shorter variant             |
| shell          | shell                         | the translation "qobiq" is ambiguous |
| pipe           | quvur / pipe                  | in a tech context `pipe` is kept |
| exit code      | exit code / chiqish kodi      | both are possible              |
| redirect       | yo'naltirish / redirect       | the `>` semantics is lost in translation |
| script         | skript                        | widely used                    |
| process        | jarayon / process             | both are accepted              |

If you think a term is missing — [open an Issue](https://github.com/qobulovasror/bashlings/issues), and we'll add it.
