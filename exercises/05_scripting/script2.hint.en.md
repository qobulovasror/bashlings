# 💡 script2 — Hints

## Step 1
**Command substitution** `$(...)` — returns the output of a command as text:
```bash
echo "Bugun: $(date +%F)"
# Bugun: 2026-05-16
```

## Step 2
`$PWD` — an environment variable holding the current directory path (just like the output of `pwd`).

`basename` — returns the last part of a path:
```bash
basename "/Users/mac/Desktop"   # Desktop
basename "$PWD"                 # name of the current directory
```

## Step 3
Combine both inside a single `echo`:
```bash
echo "$PWD ($(basename "$PWD"))"
```
