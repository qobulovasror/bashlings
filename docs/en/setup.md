---
title: "Installation and setup"
description: "Install Bash 4+, the bashlings CLI and helper tools (jq, rust) — for macOS / Linux / Windows."
---

# Installation and setup

On this page we'll prepare a working environment **from 0**. All in
~15 minutes.

## Quick tour (terminal recording)

Installing and using `bashlings` — check an exercise, fix it, and see your
progress:

<AsciinemaPlayer src="/casts/bashlings.en.cast" :cols="100" :rows="30" />

---

## Required tools

| Tool       | Requirement | Reason                                      |
|------------|-------|---------------------------------------------|
| **Bash 4+** | mandatory | The exercises depend on this version (assoc array, `mapfile`) |
| **Rust** (cargo) | optional | Only to build the CLI from source (brew / the install script give a prebuilt binary) |
| **git**    | mandatory | To clone the repo |
| **jq**     | recommended | For the Part 3 / `13_jq` exercises |
| **bc**     | recommended | For the `pipe8` exercise (may be missing on minimal Linux / WSL: `sudo apt install bc`) |
| **Node.js 18+** | optional | To view the book locally via `npm run docs:dev` |

---

## 1. Install Bash 4+

### macOS

::: warning The default Bash on macOS is **3.2** (from 2007)
The exercises depend on **Bash 4+**. You need to upgrade.
:::

```bash
# With Homebrew
brew install bash

# Check
bash --version    # should be 5.x
```

Make Bash the default (optional):
```bash
# Add it to the /etc/shells file
echo "/opt/homebrew/bin/bash" | sudo tee -a /etc/shells

# Set it as the default shell
chsh -s /opt/homebrew/bin/bash
```

### Linux

```bash
# Ubuntu / Debian
sudo apt update && sudo apt install -y bash

# Check
bash --version    # usually 5.x
```

### Windows

The supported path is **WSL2**: install Ubuntu and follow every next step in
the WSL terminal, as on Linux.

```powershell
wsl --install -d Ubuntu
```

::: warning Native Windows — experimental
The Releases page has a Windows binary (`bashlings-x86_64-pc-windows-msvc.tar.gz`),
but it is meant for use inside Git Bash and is not fully tested (a timeout
cannot stop the whole process tree, `perm`/`symlink` fixtures don't work).
If you hit problems, use WSL2.
:::

---

## 2. Rust toolchain (optional)

Only needed to build the CLI from source. For a prebuilt binary (Homebrew or
the install script) skip this section.

```bash
# Linux / macOS / WSL — via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reopen the terminal or:
source "$HOME/.cargo/env"

# Check
cargo --version    # cargo 1.70+
```

---

## 3. Install the Bashlings CLI

```bash
# 1. Clone the repo — the exercises live here
git clone https://github.com/qobulovasror/bashlings
cd bashlings

# 2. Install the CLI — pick one:
brew install qobulovasror/tap/bashlings      # macOS / Linux, prebuilt binary
curl -fsSL https://raw.githubusercontent.com/qobulovasror/bashlings/main/scripts/install.sh | sh
cargo install --path cli                      # from source (needs Rust)

# 3. Check — from the repo root
bashlings list
```

Output of `bashlings list`:
```
  Bashlings  ·  103 ta mashq

  ·  intro1     part1/01-introduction
  ·  intro2     part1/01-introduction
  ...
  Progress: 0 / 103  (0%)

  Keyingi: intro1
```

::: tip Homebrew (coming soon)
A Homebrew formula is being prepared. For now, install via `cargo install`.
:::

### Main commands — at a glance

| Command                      | Purpose                                               |
|------------------------------|-------------------------------------------------------|
| `bashlings list`             | 103 exercises + status                                |
| `bashlings watch`            | **Start here** — interactive mode                     |
| `bashlings run intro1`       | Check a single exercise                               |
| `bashlings hint intro1`      | Hint (3 levels)                                       |
| `bashlings solution intro1`  | Solution — unlocked only after you pass 🔒            |
| `bashlings reset intro1`     | Reset to the initial state                            |
| `bashlings progress`         | Compact statistics                                    |

::: info Solutions are hidden
The `.solutions/` directory is *hidden* — it doesn't show up in `ls`. Only after
you have solved an exercise can you view the solution via `bashlings solution <name>`.
This — like in rustlings — creates pressure to think for yourself.
:::

---

## 4. Helper tools

### jq — JSON parser (for the Part 3 / 13_jq exercises)

```bash
# macOS
brew install jq

# Ubuntu / Debian
sudo apt install -y jq

# Check
jq --version    # jq-1.6 or newer
```

### Other standard tools

`sed`, `awk`, `grep`, `sort`, `uniq`, `wc`, `head`, `tail`, `cut`, `tr` —
available on every Unix system. No separate installation needed.

::: warning macOS BSD vs GNU
On macOS, `sed -i` and `awk` differ slightly from the GNU version. The exercises
use **portable** syntax, but for heavy tasks it's useful to get the GNU
version:
```bash
brew install gnu-sed gawk grep
# and call them via `gsed`, `gawk`, `ggrep`
```
:::

---

## 5. Node.js (optional — to view the book locally)

```bash
# macOS
brew install node

# Ubuntu (via NodeSource)
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs

# At the repo root
npm install
npm run docs:dev    # http://localhost:5173
```

---

## 6. Full verification

From the repo root:

```bash
bashlings --version           # 0.1.x
bashlings list                # 103 exercises
bashlings run intro1          # first exercise — it's normal for it to fail!
```

If `bashlings run intro1` gives the following output — you're ready:

```
  Running intro1
  Fayl: exercises/01_intro/intro1.sh

  ✗  stdout      expected:  "Salom, Bash!"
                  actual:    ""
  ✗  exit        expected:  "0"
                  actual:    "127"

  ❌ intro1 — xato (0/2)

  Skript stderr chiqishi:
    .../intro1.sh: line 13: eko: command not found

  💡 Maslahat: bashlings hint intro1
```

This — the **initial (broken)** state of the first exercise. Now open the file
and fix it!

---

## Frequently encountered problems

::: details "command not found: bashlings"
The install succeeded, but `bashlings` can't be found?

The binary's directory is not in your PATH: `~/.local/bin` for the install
script, `~/.cargo/bin` for cargo. Add it to `.bashrc` or `.zshrc`:
```bash
export PATH="$HOME/.local/bin:$HOME/.cargo/bin:$PATH"
```
Then: `source ~/.bashrc` (or open a new terminal).
:::

::: details "workspace not found" error
`bashlings` only works from the repo root (or a subdirectory of it).
It may not be able to find the `exercises/info.toml` file. Solution: go to the
repo root (`cd bashlings`).
:::

::: details Bash 3.2 problems (macOS)
If an exercise gives `declare -A` or `mapfile` errors — you're still
using Bash 3.2. Check with `which bash`; it should be
`/opt/homebrew/bin/bash` instead of `/bin/bash`.
:::

::: details `jq: command not found`
Only needed for the `13_jq` exercises. Install with `brew install jq` or
`sudo apt install jq`.
:::

---

## Next step

Setup is done. Now let's move on to the first chapter:

[**→ Chapter 1: What are the Shell, Terminal and Bash?**](/en/part1/01-introduction)
