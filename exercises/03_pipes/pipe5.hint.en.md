# 💡 pipe5 — Hints

## Step 1
`grep` — searches for a pattern within lines:
```bash
grep "ERROR" log.txt   # prints the ERROR lines
```

## Step 2
The `-c` flag — prints the number of matches as a **clean number** (not the lines).

```bash
grep -c "ERROR" log.txt
# 2
```

## Step 3
This is the fastest way. A pipe is also possible, but `wc -l` adds leading whitespace on macOS:
```bash
grep "ERROR" log.txt | wc -l   # "       2" — with spaces
```
