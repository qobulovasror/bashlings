# bashlings

Rustlings-style interactive **Bash** exercises — a runner, file watcher, and
hint renderer. Fix small broken scripts, run `bashlings run` (or `watch`), and
progress from beginner to advanced.

The interface is bilingual: **Uzbek by default**, English via `--lang en` or
`BASHLINGS_LANG=en`.

> `bashlings` is the CLI for the [Bash UZ](https://github.com/qobulovasror/bashlings)
> learning ecosystem (a VitePress book + 101 exercises). The binary loads
> exercises from a cloned repo, so install the CLI **and** clone the repo.

## Install

```bash
# crates.io
cargo install bashlings

# from source
cargo build --release --manifest-path cli/Cargo.toml

# prebuilt binary (Linux/macOS) — see GitHub Releases
curl -fsSL https://raw.githubusercontent.com/qobulovasror/bashlings/main/scripts/install.sh | sh
```

Then clone the repo and run from inside it (the CLI walks up to find
`exercises/info.toml`):

```bash
git clone https://github.com/qobulovasror/bashlings
cd bashlings
bashlings watch
```

## Commands

| Command                          | Description                                                  |
|----------------------------------|--------------------------------------------------------------|
| `bashlings list`                 | All exercises and their status (`--pending`/`--done`/`--json`) |
| `bashlings run [name]`           | Check an exercise; no name → the next pending one            |
| `bashlings verify`               | Run all in order, stop at the first failure                 |
| `bashlings watch`                | Auto-recheck on save + hotkeys (`h`/`s`/`r`/`l`/`q`)         |
| `bashlings hint <name>`          | Progressive hint, one step per call (`--all`/`--reset`)     |
| `bashlings solution <name>`      | Reveal the solution — **only after tests pass**            |
| `bashlings reset <name>`         | Restore an exercise to its original state (`git checkout`)  |
| `bashlings progress`             | Overall + per-chapter progress (`--json`)                  |
| `bashlings next`                 | Print the next pending exercise name (`--json`)            |
| `bashlings completions <shell>`  | Shell completions (bash/zsh/fish/…)                        |
| `bashlings --lang <uz\|en>`      | Interface language (also `BASHLINGS_LANG`)                 |

## Exercise test format

Each exercise script ends with `# @test:...` directives:

| Directive                          | Checks                                       |
|------------------------------------|----------------------------------------------|
| `# @test:stdout: <text>`           | stdout equals the literal text               |
| `# @test:stdout-cmd: <command>`    | stdout equals the command's stdout           |
| `# @test:stdout-contains: <text>`  | stdout contains the substring                |
| `# @test:stdout-regex: <pattern>`  | stdout matches the regular expression        |
| `# @test:stderr: <text>`           | stderr equals the literal text               |
| `# @test:exit: <code>`             | exit code equals the number                  |
| `# @test:file-exists: <path>`      | the path exists after the run                |

The runner is hardened: each script runs with a **10s timeout** (infinite loops
can't hang the CLI), `stdin` is `/dev/null`, and the working directory is the
workspace root (deterministic).

## Exit codes

| Code | Meaning                                                  |
|------|----------------------------------------------------------|
| 0    | Success / a pending exercise exists (`next`)             |
| 1    | Tests failed / nothing pending (`next`)                  |
| 2    | Unrecoverable error (workspace not found, parse error…)  |

## Colors

ANSI colors are auto-disabled when stdout is not a TTY (e.g. `| grep`).
`NO_COLOR=1` forces them off; `CLICOLOR_FORCE=1` forces them on in pipes.

## License

MIT
