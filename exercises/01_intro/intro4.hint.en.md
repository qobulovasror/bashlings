# 💡 intro4 — Hints

## Step 1
By default the `date` command prints a long format (e.g. `Sat May 16 14:22:01 2026`). We need **YYYY-MM-DD**.

## Step 2
In the Bash documentation, `date +%F` is the **ISO 8601** format: `2026-05-16`. Try it in the terminal.

## Step 3
To use a command's output inside a string, use `$(...)`:
```bash
today=$(date +%F)
echo "Bugun: $today"
```
Or on a single line:
```bash
echo "Bugun: $(date +%F)"
```
